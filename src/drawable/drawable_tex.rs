use ash::vk;

use crate::mesh::RECT_INDICES;
use crate::shader::ShaderTexture;
use crate::utils::image::{copy_buffer_to_image, transition_image_layout, ImageLayout_ShaderReadOnlyOptimal, ImageLayout_TransferDstOptimal, ImageLayout_Undefined};
use crate::vk_base::VkBase;
use crate::vk_bundles::BufferBundle;
use crate::{utils, DeviceBundle, GraphicsPipelineBundle, TextureBundle};
use crate::primitives::{texture2d::Texture2d, rect::Rect};


pub struct DrawableTexture {
    pub screen_span: Rect,
    pub texture_span: Rect,
    pub texture_data: Texture2d,
    pub texture: TextureBundle,
    pub vbo: BufferBundle,
    pub ind: BufferBundle,
    pub coords: BufferBundle,
	pub descriptor_sets: Vec<vk::DescriptorSet>,
    pub screen_span_updated: bool,
    pub texture_span_updated: bool,
    pub initialized: bool
}

const RECT_SIZE_IND: u64 = std::mem::size_of_val(&RECT_INDICES) as u64;
const RECT_SIZE_VRT: u64 = Rect::size_of_vertices() as u64;


impl DrawableTexture {

    pub fn new( base: &VkBase, cb: vk::CommandBuffer, screen_span: Rect, texture_span: Rect, texture_data: Texture2d ) -> Self {

		let device = &base.device;

        let texture = utils::image::create_texture_image(device, texture_data.width, texture_data.height, texture_data.size, texture_data.format);

        let required_memory_flags = vk::MemoryPropertyFlags::HOST_VISIBLE;
        let usage = vk::BufferUsageFlags::VERTEX_BUFFER;
        let vbo = utils::buffer::create_buffer(device, RECT_SIZE_VRT, usage, required_memory_flags).expect("Failed to create vertex buffer.");

        let required_memory_flags = vk::MemoryPropertyFlags::HOST_VISIBLE;
        let usage = vk::BufferUsageFlags::VERTEX_BUFFER;
        let coords = utils::buffer::create_buffer(device, RECT_SIZE_VRT, usage, required_memory_flags).expect("Failed to create vertex buffer.");

        let required_memory_flags = vk::MemoryPropertyFlags::HOST_VISIBLE;
        let usage = vk::BufferUsageFlags::INDEX_BUFFER;
        let ind = utils::buffer::create_buffer(device, RECT_SIZE_IND, usage, required_memory_flags).expect("Failed to create vertex buffer.");

		let pso = &base.graphics_pipelines[ShaderTexture::ID];
		let layout = pso.ubo.as_ref().expect("Expected ubo to be defined.").layouts[0];
		let descriptor_sets = VkBase::create_descriptor_sets(device, base.descriptor_pool, layout, base.max_in_flight);

        transition_image_layout::<ImageLayout_Undefined, ImageLayout_ShaderReadOnlyOptimal>(device, cb, &texture);
        DrawableTexture { screen_span, texture_span, texture_data, texture, vbo, ind, coords, screen_span_updated: true, texture_span_updated: true, initialized: false, descriptor_sets }
    }

    pub fn dirty(&self) -> bool {
        return self.screen_span_updated
            || self.texture_span_updated
            || self.initialized
            || self.texture_data.dirty;
    }

    pub fn update(device: &DeviceBundle, cb: vk::CommandBuffer, entities: &mut [Self]) -> bool {
        let mut recorded = false;

        for entity in entities.iter_mut() {
            if !entity.dirty() {
                continue;
            }

            recorded = true;

            let texture_size = entity.texture_data.size;

            unsafe {
                if !entity.initialized {
                    let data_ptr = device.logical.map_memory(entity.ind.memory, 0, RECT_SIZE_IND, vk::MemoryMapFlags::empty()).unwrap() as *mut u16;
                    data_ptr.copy_from_nonoverlapping(RECT_INDICES.as_ptr(), RECT_INDICES.len());
                    device.logical.unmap_memory(entity.ind.memory);
                    entity.initialized = true;
                }

                if entity.screen_span_updated {
                    let data_ptr = device.logical.map_memory(entity.vbo.memory, 0, RECT_SIZE_VRT, vk::MemoryMapFlags::empty()).unwrap() as *mut [f32; 2];
                    data_ptr.copy_from_nonoverlapping(entity.screen_span.vertices.as_ptr(), entity.screen_span.vertices.len());
                    device.logical.unmap_memory(entity.vbo.memory);
                    entity.screen_span_updated = false;
                }

                if entity.texture_span_updated {
                    let data_ptr = device.logical.map_memory(entity.coords.memory, 0, RECT_SIZE_VRT as u64, vk::MemoryMapFlags::empty()).unwrap() as *mut [f32; 2];
                    data_ptr.copy_from_nonoverlapping(entity.texture_span.vertices.as_ptr(), entity.screen_span.vertices.len());
                    device.logical.unmap_memory(entity.coords.memory);
                    entity.texture_span_updated = false;
                }

                if entity.texture_data.dirty {
                    let data_ptr = device.logical.map_memory(entity.texture.staging.memory, 0, texture_size, vk::MemoryMapFlags::empty()).unwrap() as *mut u8;
                    data_ptr.copy_from_nonoverlapping(entity.texture_data.data.as_ptr(), texture_size as usize);
                    device.logical.unmap_memory(entity.texture.staging.memory);

                    transition_image_layout::<ImageLayout_ShaderReadOnlyOptimal, ImageLayout_TransferDstOptimal>(device, cb, &entity.texture);
                    copy_buffer_to_image(device, cb, &entity.texture, entity.texture.staging.buffer, entity.texture_data.width, entity.texture_data.height);
                    transition_image_layout::<ImageLayout_TransferDstOptimal, ImageLayout_ShaderReadOnlyOptimal>(device, cb, &entity.texture);

                    // TODO: Concurrency assumption
                    entity.texture_data.dirty = false;
                }
            }

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

                VkBase::update_descriptor_set_textures(&device, set[0], &[&entities[i].texture], 0);

                device.logical.cmd_bind_vertex_buffers(cb, 0, &[entities[i].vbo.buffer, entities[i].coords.buffer], &[0, 0]);
                device.logical.cmd_bind_index_buffer(cb, entities[i].ind.buffer, 0, vk::IndexType::UINT16);

                device.logical.cmd_bind_descriptor_sets(
                    cb, vk::PipelineBindPoint::GRAPHICS, pso.layout, 0,
                    &set, &[]);

                device.logical.cmd_draw_indexed(cb, RECT_INDICES.len() as u32, 1, 0, 0, 0);
            }
        }

    }

    pub fn release(device: &DeviceBundle, textures: &mut [Self])
    {
        unsafe
        {
            for texture in textures.iter() {
                device.logical.destroy_buffer(texture.vbo.buffer, None);
                device.logical.free_memory(texture.vbo.memory, None);
                device.logical.destroy_buffer(texture.coords.buffer, None);
                device.logical.free_memory(texture.coords.memory, None);
                device.logical.destroy_buffer(texture.ind.buffer, None);
                device.logical.free_memory(texture.ind.memory, None);
                device.logical.destroy_buffer(texture.texture.staging.buffer, None);
                device.logical.free_memory(texture.texture.staging.memory, None);
                device.logical.destroy_image(texture.texture.resource.image, None);
                device.logical.free_memory(texture.texture.resource.memory, None);
                device.logical.destroy_image_view(texture.texture.image_view, None);
                device.logical.destroy_sampler(texture.texture.sampler, None);
            }
        }
    }
}
