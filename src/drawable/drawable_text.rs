 use ash::vk;
use stb_truetype::{FontAtlasInfo, FontAtlas};

use crate::geometry::vec3::Vec3;
use crate::rhi::allocator::{Allocator, BufferType};
use crate::shader::ShaderText;
use crate::vk_base::VkBase;
use crate::vk_bundles::BufferBundle;
use crate::{DeviceBundle, GraphicsPipelineBundle, TextureBundle};


#[repr(C)]
struct TextData {
    char_dims: Vec3,
    position: Vec3,
    colour: Vec3,
	char_packing: Vec3,
}


pub struct DrawableText {
	pub atlas_info: FontAtlasInfo,

    pub position: Vec3,
    pub capacity: usize,
    pub text: Vec<u32>,
    pub kerning_info: Vec<(i32, i32)>,

	pub descriptor_sets: Vec<vk::DescriptorSet>,
    pub staging_info: BufferBundle,
    pub staging_content: BufferBundle,
    pub staging_kerning: BufferBundle,
    pub uniform_info: BufferBundle,
    pub uniform_content: BufferBundle,
    pub uniform_kerning: BufferBundle,

    pub dirty: bool
}

impl DrawableText {

    pub fn new(base: &VkBase, position: Vec3, atlas_info: FontAtlasInfo, allocator: &mut Allocator, text: &str, capacity: usize) -> Self {

        let size = std::mem::size_of::<TextData>() + capacity * 4 + capacity * 2 * 4;

        let staging_info    = allocator.alloc(BufferType::Staging, std::mem::size_of::<TextData>() as u64).unwrap();
        let staging_content = allocator.alloc(BufferType::Staging, capacity as u64 * 4).unwrap();
        let staging_kerning = allocator.alloc(BufferType::Staging, capacity as u64 * 8).unwrap();
        let uniform_info    = allocator.alloc(BufferType::Uniform, std::mem::size_of::<TextData>() as u64).unwrap();
        let uniform_content = allocator.alloc(BufferType::Uniform, capacity as u64 * 4).unwrap();
        let uniform_kerning = allocator.alloc(BufferType::Uniform, capacity as u64 * 8).unwrap();

        let mut bytes = text.as_bytes();
        if bytes.len() > capacity {
            bytes = &bytes[0..capacity];
        }

        let text_map = bytes.iter().map(|c| { *c as u32 } ).collect::<Vec<_>>();
        let kerning_info = vec![(0, 0); text_map.len()];

		let pso = &base.graphics_pipelines[ShaderText::ID];
		let layout = pso.ubo.as_ref().expect("Expected ubo to be defined.").layouts[0];
		let descriptor_sets = VkBase::create_descriptor_sets(&base.device, base.descriptor_pool, layout, base.max_in_flight);

        DrawableText {
            position,
            capacity,
            text: text_map,
            kerning_info,
            staging_info,
            staging_content,
            staging_kerning,
            uniform_info,
            uniform_content,
            uniform_kerning,
            dirty: true,
            descriptor_sets,
            atlas_info,
        }
    }


    pub fn set_text(&mut self, text: &str) {

        let mut bytes = text.as_bytes();
        if bytes.len() > self.capacity {
            bytes = &bytes[0..self.capacity];
        }
        self.text = bytes.iter().map(|c| { *c as u32 } ).collect::<Vec<_>>();

        self.dirty = true;
    }

    pub fn kern_text(&mut self, font: &FontAtlas) {
        // TODO: this map can be removed by using text as Vec<u8> to begin with
        let text = self.text.iter().map(|c| { *c as u8 } ).collect::<Vec<_>>();
        self.kerning_info.resize(self.text.len(), (0, 0));
        font.pack_kerning_data(&text[..], &mut self.kerning_info);
        self.dirty = true;
    }

    pub fn update(device: &DeviceBundle, cb: vk::CommandBuffer, entities: &mut [Self]) -> bool {
        let mut recorded = false;

        for entity in entities.iter_mut() {
            if !entity.dirty {
                continue;
            }

            recorded = true;

            let size = std::mem::size_of::<TextData>() + entity.capacity * 4;

            let size_gen = std::mem::size_of::<TextData>();

            let text_data = TextData {
                char_dims: Vec3::new(entity.atlas_info.char_width as f32, entity.atlas_info.char_height as f32, 0.0),
                position: entity.position,
                colour: Vec3::new(1.0, 1.0, 1.0),
				char_packing: Vec3::new(entity.atlas_info.chars_per_row as f32, entity.atlas_info.chars_per_col as f32, 0.0)
            };

            unsafe {

				let data_ptr = device.logical.map_memory(entity.staging_info.memory, 0, size as u64, vk::MemoryMapFlags::empty()).unwrap() as *mut u8;
                let data_ptr = data_ptr.offset(entity.staging_info.offset as isize) as * mut TextData;
                data_ptr.copy_from_nonoverlapping(&text_data as _, size_gen);
                device.logical.unmap_memory(entity.staging_info.memory);


				let data_ptr = device.logical.map_memory(entity.staging_content.memory, 0, size as u64, vk::MemoryMapFlags::empty()).unwrap() as *mut u8;
                let data_ptr = data_ptr.offset(entity.staging_content.offset as isize) as * mut u32;
				let size = std::mem::size_of_val(&entity.text[..]);
                data_ptr.copy_from_nonoverlapping(entity.text.as_ptr(), size);
                device.logical.unmap_memory(entity.staging_content.memory);

				let data_ptr = device.logical.map_memory(entity.staging_kerning.memory, 0, size as u64, vk::MemoryMapFlags::empty()).unwrap() as *mut u8;
                let data_ptr = data_ptr.offset(entity.staging_kerning.offset as isize) as * mut (i32, i32);
				let size = std::mem::size_of_val(&entity.kerning_info[..]);
                data_ptr.copy_from_nonoverlapping(entity.kerning_info.as_ptr(), size);
                device.logical.unmap_memory(entity.staging_kerning.memory);

                let copy_region = [
                    vk::BufferCopy::default()
                        .src_offset(entity.staging_info.offset)
                        .dst_offset(entity.uniform_info.offset)
                        .size(entity.uniform_info.size)
                ];

                device.logical.cmd_copy_buffer(cb, entity.staging_info.buffer, entity.uniform_info.buffer, &copy_region);

                let copy_region = [
                    vk::BufferCopy::default()
                        .src_offset(entity.staging_content.offset)
                        .dst_offset(entity.uniform_content.offset)
                        .size(entity.uniform_content.size)
                ];

                device.logical.cmd_copy_buffer(cb, entity.staging_content.buffer, entity.uniform_content.buffer, &copy_region);

                let copy_region = [
                    vk::BufferCopy::default()
                        .src_offset(entity.staging_kerning.offset)
                        .dst_offset(entity.uniform_kerning.offset)
                        .size(entity.uniform_kerning.size)
                ];

                device.logical.cmd_copy_buffer(cb, entity.staging_kerning.buffer, entity.uniform_kerning.buffer, &copy_region);
            }

            entity.dirty = false;
        }

        return recorded;
    }

    pub fn init_font_atlas(device: &DeviceBundle, font_atlas: &TextureBundle, glyph_info_buffer: &BufferBundle, entities: &[Self])  {
        for i in 0..entities.len() {
            for set in entities[i].descriptor_sets.iter() {
                VkBase::update_descriptor_set_textures(&device, *set, &[&font_atlas], 0);
                VkBase::update_descriptor_set_buffers(&device, *set, &[&entities[i].uniform_content], 1);
                VkBase::update_descriptor_set_buffers(&device, *set, &[&entities[i].uniform_kerning], 2);
                VkBase::update_descriptor_set_buffers(&device, *set, &[&entities[i].uniform_info], 3);
                VkBase::update_descriptor_set_buffers(&device, *set, &[glyph_info_buffer], 4);
            }
        }
    }


    pub fn draw(device: &DeviceBundle, cb: vk::CommandBuffer, pso: &GraphicsPipelineBundle, current_swap_image: usize, entities: &[Self])  {

        unsafe {
            device.logical.cmd_bind_pipeline(cb, vk::PipelineBindPoint::GRAPHICS, pso.graphics);
        }

        unsafe {

            for i in 0..entities.len() {
				let set = &entities[i].descriptor_sets[current_swap_image..current_swap_image+1];

                device.logical.cmd_bind_descriptor_sets(
                    cb, vk::PipelineBindPoint::GRAPHICS, pso.layout, 0,
                    &set, &[]);

                device.logical.cmd_draw(cb, entities[i].text.len() as u32 * 6, 1, 0, 0);
            }
        }
    }


    pub fn release(_device: &DeviceBundle, _entities: &mut [Self]) {
    }
}
