use ash::vk;

use crate::geometry::vec3::Vec3;
use crate::rhi::allocator::{Allocator, BufferType};
use crate::vk_base::VkBase;
use crate::vk_bundles::BufferBundle;
use crate::{DeviceBundle, GraphicsPipelineBundle, TextureBundle};


#[repr(C)]
struct TextData {
    scale: f32,
    char_dims: Vec3,
    position: Vec3,
    colour: Vec3
}


pub struct DrawableText {
    pub texture: TextureBundle,

    pub position: Vec3,
    pub capacity: usize,
    pub text: Vec<u32>,

    pub staging: BufferBundle,
    pub uniform: BufferBundle,

    pub dirty: bool
}

impl DrawableText {

    pub fn new(position: Vec3, texture: TextureBundle, allocator: &mut Allocator, text: &str, capacity: usize) -> Self {

        let size = std::mem::size_of::<TextData>() + capacity * 4;

        let staging = allocator.alloc(BufferType::Staging, size as u64).unwrap();
        let uniform = allocator.alloc(BufferType::Uniform, size as u64).unwrap();

        let mut bytes = text.as_bytes();
        if bytes.len() > capacity {
            bytes = &bytes[0..capacity];
        }

        let text_map = bytes.iter().map(|c| { *c as u32 } ).collect::<Vec<_>>();

        DrawableText {
            texture,
            position,
            capacity,
            text: text_map,
            staging,
            uniform,
            dirty: true
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


    pub fn update(device: &DeviceBundle, cb: vk::CommandBuffer, entities: &mut Vec<Self>) -> bool {
        let mut recorded = false;

        for entity in entities.iter_mut() {
            if !entity.dirty {
                continue;
            }

            recorded = true;

            let size = std::mem::size_of::<TextData>() + entity.capacity * 4 + entity.capacity * 2 * 4;

            let size_gen = std::mem::size_of::<TextData>();

            let text_data = TextData {
                scale: 1.0,
                char_dims: Vec3::new(15.0, 33.0, 0.0),
                position: entity.position,
                colour: Vec3::new(1.0, 1.0, 1.0),
            };
            unsafe {
                let staging_data_ptr = device.logical.map_memory(entity.staging.memory, entity.staging.offset, size as u64, vk::MemoryMapFlags::empty()).unwrap() as *mut u8;

                let data_ptr = staging_data_ptr as *mut TextData;
                data_ptr.copy_from_nonoverlapping(&text_data as _, size_gen);

                let data_ptr = staging_data_ptr.offset(size_gen as isize) as *mut u32;
                data_ptr.copy_from_nonoverlapping(entity.text.as_ptr(), entity.text.len());

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

        //TODO: FIX THIS PARADIGM
        match &pso.ubo {
            Some(ubo) => {
                unsafe {
                    let set = &ubo.sets[current_swap_image..current_swap_image+1];

                    for i in 0..entities.len() {

                        VkBase::update_descriptor_set_textures(&device, set[0], &[&entities[i].texture], 0);

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

                        device.logical.cmd_draw(cb, entities[i].text.len() as u32 * 4, 1, 0, 0);
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
        for texture in textures.iter() {
            unsafe {
                device.logical.destroy_image(texture.texture.resource.image, None);
                device.logical.free_memory(texture.texture.resource.memory, None);
                device.logical.destroy_image_view(texture.texture.image_view, None);
                device.logical.destroy_sampler(texture.texture.sampler, None);
            }
        }
    }
}
