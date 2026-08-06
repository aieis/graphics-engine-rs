use ash::vk;

use crate::{drawable::drawable_tex::DrawableTexture, vk_bundles::{DeviceBundle, GraphicsPipelineBundle}};

pub struct SlidingTexture {
    pub texture: DrawableTexture,
    pub sliding_period: f32
}


impl SlidingTexture {

    pub fn new(texture: DrawableTexture, sliding_period: f32) -> Self {
        Self {
            texture,
            sliding_period,
        }
    }

    pub fn update(device: &DeviceBundle, cb: vk::CommandBuffer, delta_time: f32, entities: &mut [Self]) {


        for entity in entities {
            let dy = delta_time / entity.sliding_period;

            if dy + entity.texture.screen_span.y + entity.texture.screen_span.h > 1.0 {
                entity.texture.screen_span.y = 0.0;
                entity.texture.texture_span.y = 0.0;
            } else {
                entity.texture.screen_span.y += dy;
                entity.texture.texture_span.y += 2.0 * dy;
            }

            entity.texture.screen_span_updated = true;
            entity.texture.texture_span_updated = true;

            DrawableTexture::update(device, cb, std::slice::from_mut(&mut entity.texture));
        }

    }

    pub fn draw(device: &DeviceBundle, cb: vk::CommandBuffer, pso: &GraphicsPipelineBundle, current_swap_image: usize, entities: &[Self])  {

        for entity in entities {
            DrawableTexture::draw(device, cb, pso, current_swap_image, std::slice::from_ref(&entity.texture));
        }
    }

    pub fn release(device: &DeviceBundle, entities: &mut [Self]) {
        for entity in entities {
            DrawableTexture::release(device, std::slice::from_mut(&mut entity.texture));            
        }
    }
}
