use ash::vk;
use stb_truetype::FontAtlasInfo;

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
    pub font_atlas: TextureBundle,

    pub position: Vec3,
    pub capacity: usize,
    pub text: Vec<u32>,

	pub descriptor_sets: Vec<vk::DescriptorSet>,
    pub staging: BufferBundle,
    pub uniform: BufferBundle,

    pub dirty: bool
}

impl DrawableText {

    pub fn new(base: &VkBase, position: Vec3, atlas_info: FontAtlasInfo, font_atlas: TextureBundle, allocator: &mut Allocator, text: &str, capacity: usize) -> Self {

        let size = std::mem::size_of::<TextData>() + capacity * 4;

        let staging = allocator.alloc(BufferType::Staging, size as u64).unwrap();
        let uniform = allocator.alloc(BufferType::Uniform, size as u64).unwrap();

        let mut bytes = text.as_bytes();
        if bytes.len() > capacity {
            bytes = &bytes[0..capacity];
        }

        let text_map = bytes.iter().map(|c| { *c as u32 } ).collect::<Vec<_>>();

		let pso = &base.graphics_pipelines[ShaderText::ID];
		let layout = pso.ubo.as_ref().expect("Expected ubo to be defined.").layouts[0];
		let descriptor_sets = VkBase::create_descriptor_sets(&base.device, base.descriptor_pool, layout, base.max_in_flight);

        DrawableText {
            font_atlas,
            position,
            capacity,
            text: text_map,
            staging,
            uniform,
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
				char_packing: Vec3::new(entity.atlas_info.chars_per_col as f32, entity.atlas_info.chars_per_row as f32, 0.0)
            };

            unsafe {
                let staging_data_ptr = device.logical.map_memory(entity.staging.memory, entity.staging.offset, size as u64, vk::MemoryMapFlags::empty()).unwrap() as *mut u8;

                let data_ptr = staging_data_ptr as *mut TextData;
                data_ptr.copy_from_nonoverlapping(&text_data as _, size_gen);

                let data_ptr = staging_data_ptr.offset(size_gen as isize) as *mut u32;

				let size = std::mem::size_of_val(&entity.text[..]);
                data_ptr.copy_from_nonoverlapping(entity.text.as_ptr(), size);

                device.logical.unmap_memory(entity.staging.memory);

                let copy_region = [
                    vk::BufferCopy::default()
                        .src_offset(entity.staging.offset)
                        .dst_offset(entity.uniform.offset)
                        .size(entity.staging.size)
                ];

                device.logical.cmd_copy_buffer(cb, entity.staging.buffer, entity.uniform.buffer, &copy_region);
            }


            entity.dirty = false;
        }

        return recorded;
    }


    pub fn draw(device: &DeviceBundle, cb: vk::CommandBuffer, pso: &GraphicsPipelineBundle, current_swap_image: usize, entities: &[Self])  {

        unsafe {
            device.logical.cmd_bind_pipeline(cb, vk::PipelineBindPoint::GRAPHICS, pso.graphics);
        }

        unsafe {

            for i in 0..entities.len() {
				let set = &entities[i].descriptor_sets[current_swap_image..current_swap_image+1];

                VkBase::update_descriptor_set_textures(&device, set[0], &[&entities[i].font_atlas], 0);

                let size_gen = std::mem::size_of::<TextData>();

                let buffer = BufferBundle {
                    buffer: entities[i].uniform.buffer,
                    memory: entities[i].uniform.memory,
                    offset: entities[i].uniform.offset,
                    size: size_gen as u64
                };

                VkBase::update_descriptor_set_buffers(&device, set[0], &[&buffer], 2);

                let buffer = BufferBundle {
                    buffer: entities[i].uniform.buffer,
                    memory: entities[i].uniform.memory,
                    offset: entities[i].uniform.offset + size_gen as u64,
                    size: entities[i].capacity as u64 * 4
                };

                VkBase::update_descriptor_set_buffers(&device, set[0], &[&buffer], 1);


                device.logical.cmd_bind_descriptor_sets(
                    cb, vk::PipelineBindPoint::GRAPHICS, pso.layout, 0,
                    &set, &[]);

                device.logical.cmd_draw(cb, entities[i].text.len() as u32 * 6, 1, 0, 0);
            }
        }
    }


    pub fn release(device: &DeviceBundle, textures: &mut [Self])
    {
        for texture in textures.iter() {
            unsafe {
				device.logical.destroy_buffer(texture.font_atlas.staging.buffer, None);
                device.logical.free_memory(texture.font_atlas.staging.memory, None);
                device.logical.destroy_image(texture.font_atlas.resource.image, None);
                device.logical.free_memory(texture.font_atlas.resource.memory, None);
                device.logical.destroy_image_view(texture.font_atlas.image_view, None);
                device.logical.destroy_sampler(texture.font_atlas.sampler, None);
            }
        }
    }
}
