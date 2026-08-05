use ash::vk;

use crate::mesh::Rect;
use crate::utils::image::{copy_buffer_to_image, transition_image_layout, ImageLayout_ShaderReadOnlyOptimal, ImageLayout_TransferDstOptimal, ImageLayout_Undefined};
use crate::vk_base::VkBase;
use crate::vk_bundles::BufferBundle;
use crate::{utils, DeviceBundle, GraphicsPipelineBundle, TextureBundle};
use crate::primitives::texture2d::Texture2d;


pub struct DrawableTexture {
    pub rect: Rect,
    pub texture_data: Texture2d,
    pub texture: TextureBundle,
    pub vbo: BufferBundle,
    pub ind: BufferBundle,
    pub coords: BufferBundle,
}

impl DrawableTexture {

    pub fn new( device: &DeviceBundle, command_buffer: vk::CommandBuffer, rect: Rect, texture_data: Texture2d ) -> Self {

        let texture = utils::image::create_texture_image(device, texture_data.width, texture_data.height, texture_data.size, texture_data.format);

        let required_memory_flags = vk::MemoryPropertyFlags::HOST_VISIBLE;
        let usage = vk::BufferUsageFlags::VERTEX_BUFFER;
        let vbo = utils::buffer::create_buffer(device, rect.size_vrt() as u64, usage, required_memory_flags).expect("Failed to create vertex buffer.");

        //TODO: FIX THIS SILLY GOOSE
        let coord_mesh = Rect::new(0.0, 0.0, 1.0, 1.0, [1.0, 1.0, 1.0]);

        let required_memory_flags = vk::MemoryPropertyFlags::HOST_VISIBLE;
        let usage = vk::BufferUsageFlags::VERTEX_BUFFER;
        let coords = utils::buffer::create_buffer(device, coord_mesh.size_vrt() as u64, usage, required_memory_flags).expect("Failed to create vertex buffer.");

        let required_memory_flags = vk::MemoryPropertyFlags::HOST_VISIBLE;
        let usage = vk::BufferUsageFlags::INDEX_BUFFER;
        let ind = utils::buffer::create_buffer(device, coord_mesh.size_vrt() as u64, usage, required_memory_flags).expect("Failed to create vertex buffer.");

        transition_image_layout::<ImageLayout_Undefined, ImageLayout_ShaderReadOnlyOptimal>(device, command_buffer, &texture);
        DrawableTexture { rect, texture_data, texture, vbo, coords, ind }
    }

    pub fn dirty(&self) -> bool {
        return self.rect.dirty_colour || self.rect.dirty_indices || self.rect.dirty_vertices || self.texture_data.dirty;
    }

    pub fn update(device: &DeviceBundle, command_buffer: vk::CommandBuffer, entities: &mut Vec<Self>) -> bool {
        let mut recorded = false;

        for entity in entities.iter_mut() {
            if !entity.dirty() {
                continue;
            }

            recorded = true;

            let size_vrt = entity.rect.size_vrt() as u64;
            let size_ind = entity.rect.size_ind() as u64;
            let texture_size = entity.texture_data.size;

            unsafe {
                if entity.rect.dirty_vertices {
                    let data_ptr = device.logical.map_memory(entity.vbo.memory, 0, size_vrt, vk::MemoryMapFlags::empty()).unwrap() as *mut [f32; 2];
                    data_ptr.copy_from_nonoverlapping(entity.rect.vertices.as_ptr(), entity.rect.vertices.len());
                    device.logical.unmap_memory(entity.vbo.memory);

                    let coord_mesh = Rect::new(0.0, 0.0, 1.0, 1.0, [1.0, 1.0, 1.0]);
                    let data_ptr = device.logical.map_memory(entity.coords.memory, 0, coord_mesh.size_vrt() as u64, vk::MemoryMapFlags::empty()).unwrap() as *mut [f32; 2];
                    data_ptr.copy_from_nonoverlapping(coord_mesh.vertices.as_ptr(), entity.rect.vertices.len());
                    device.logical.unmap_memory(entity.coords.memory);
                }

                if entity.rect.dirty_indices {
                    let data_ptr = device.logical.map_memory(entity.ind.memory, 0, size_ind, vk::MemoryMapFlags::empty()).unwrap() as *mut u16;
                    data_ptr.copy_from_nonoverlapping(entity.rect.indices.as_ptr(), entity.rect.indices.len());
                    device.logical.unmap_memory(entity.ind.memory);
                }

                if entity.texture_data.dirty {
                    let data_ptr = device.logical.map_memory(entity.texture.staging.memory, 0, texture_size, vk::MemoryMapFlags::empty()).unwrap() as *mut u8;
                    data_ptr.copy_from_nonoverlapping(entity.texture_data.data.as_ptr(), texture_size as usize);
                    device.logical.unmap_memory(entity.texture.staging.memory);

                    transition_image_layout::<ImageLayout_ShaderReadOnlyOptimal, ImageLayout_TransferDstOptimal>(device, command_buffer, &entity.texture);
                    copy_buffer_to_image(device, command_buffer, &entity.texture, entity.texture.staging.buffer, entity.texture_data.width, entity.texture_data.height);
                    transition_image_layout::<ImageLayout_TransferDstOptimal, ImageLayout_ShaderReadOnlyOptimal>(device, command_buffer, &entity.texture);
                }
            }

            entity.rect.dirty_colour = false;
            entity.rect.dirty_vertices = false;
            entity.rect.dirty_indices = false;
        }

        return recorded;
    }

    pub fn draw(device: &DeviceBundle, command_buffer: vk::CommandBuffer, pso: &GraphicsPipelineBundle, current_swap_image: usize, entities: &[Self])  {

        unsafe {
            device.logical.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, pso.graphics);
        }

        //TODO: FIX THIS PARADIGM
        match &pso.ubo {
            Some(ubo) => {
                unsafe {
                    let set = &ubo.sets[current_swap_image..current_swap_image+1];

                    for i in 0..entities.len() {

                        VkBase::update_descriptor_set_textures(&device, set[0], &[&entities[i].texture], 0);

                        device.logical.cmd_bind_vertex_buffers(command_buffer, 0, &[entities[i].vbo.buffer, entities[i].coords.buffer], &[0, 0]);
                        device.logical.cmd_bind_index_buffer(command_buffer, entities[i].ind.buffer, 0, vk::IndexType::UINT16);

                        device.logical.cmd_bind_descriptor_sets(
                            command_buffer, vk::PipelineBindPoint::GRAPHICS, pso.layout, 0,
                            &set, &[]);

                        device.logical.cmd_draw_indexed(command_buffer, entities[i].rect.indices.len() as u32, 1, 0, 0, 0);
                    }
                }
            }


            None => {
                panic!("ERROR: NO DESCRIPTOR SETS WHEN ONE IS EXPECTED");
            }
        };

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
