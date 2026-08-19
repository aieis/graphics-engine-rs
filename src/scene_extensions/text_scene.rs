use std::time::Instant;

use ash::vk;
use stb_truetype::{FontAtlas, CHARS_LEN};
use winit::event::ElementState;
use winit::keyboard::KeyCode;

use crate::drawable::drawable_text::DrawableText;
use crate::geometry::vec3::Vec3;
use crate::primitives::image::PixelFormat;
use crate::rhi::allocator::{Allocator, BufferType};
use crate::utils;
use crate::utils::image::{ImageLayout_ShaderReadOnlyOptimal, ImageLayout_TransferDstOptimal, ImageLayout_Undefined};
use crate::vk_bundles::{BufferBundle, TextureBundle};
use crate::vk_base::VkBase;
use crate::shader::ShaderText;


macro_rules! FONT_ATLAS_PATH_MAC { () => { "../../assets/fonts/Atlas_Iosevka_Regular_12x8_25x55_atlas.ff" }; }
macro_rules! FONT_ATLAS_DESC_PATH_MAC { () => { "../../assets/fonts/Atlas_Iosevka_Regular_12x8_25x55_atlas_desc.bin" }; }

const FONT_ATLAS_DATA: &[u8] = include_bytes!(FONT_ATLAS_PATH_MAC!());
const FONT_ATLAS_DESC_DATA: &[u8] = include_bytes!(FONT_ATLAS_DESC_PATH_MAC!());

struct SharedFontData {
    atlas: FontAtlas,
    atlas_texture: TextureBundle,

    staging: BufferBundle,
    glyph_uniform: BufferBundle,
}

pub struct TextScene
{
	frame_timer: DrawableText,
    font_data: SharedFontData,
    initialized: bool
}

impl TextScene
{
    pub fn new(base: &VkBase, allocator: &mut Allocator) -> Self {
        let font_atlas = FontAtlas::parse_atlas_from_memory(FONT_ATLAS_DESC_DATA, FONT_ATLAS_DATA).expect("Failed to load atlas.");
        let font_atlas_texture = utils::image::create_texture_image(&base.device, font_atlas.atlas.w, font_atlas.atlas.h, (font_atlas.atlas.w * font_atlas.atlas.h * 4) as u64, PixelFormat::RGBA);
        let mut frame_timer = DrawableText::new(base, Vec3::new(-0.8, -0.8, 0.0), font_atlas.desc.info.clone(), allocator, " Hello, World", 64);

        let font_data = SharedFontData {
            atlas: font_atlas,
            atlas_texture: font_atlas_texture,
            staging: allocator.alloc(BufferType::Staging, CHARS_LEN as u64 * std::mem::size_of::<u32>() as u64 * 2).unwrap(),
            glyph_uniform: allocator.alloc(BufferType::Uniform, CHARS_LEN as u64 * std::mem::size_of::<u32>() as u64 * 2).unwrap(),
        };

        DrawableText::init_font_atlas(&base.device, &font_data.atlas_texture, &font_data.glyph_uniform, &std::slice::from_ref(&frame_timer));
		frame_timer.kern_text(&font_data.atlas);

        Self {
            frame_timer,
            font_data,
            initialized: false,
        }
    }

    pub fn handle_key(&mut self, key: KeyCode, state: ElementState, _repeat: bool) {

        if state != ElementState::Pressed {
            return;
        }

        match key {
            _ => {

            }
        }
    }

    pub fn update(&mut self, base: &VkBase, cb: vk::CommandBuffer, _aspect_ratio: f32) {

        if !self.initialized {

            unsafe {

                let size = self.font_data.staging.size;
                let data: [(u32, u32); CHARS_LEN] = self.font_data.atlas.desc.glyph_info.each_ref().map(|g| { (g.w as u32, g.h as u32) });
                let data_ptr = base.device.logical.map_memory(self.font_data.staging.memory, 0, size, vk::MemoryMapFlags::empty()).unwrap() as *mut u8;
                let data_ptr = data_ptr.offset(self.font_data.staging.offset as isize) as *mut (u32, u32);
                data_ptr.copy_from_nonoverlapping(data.as_ptr(), size as usize);
                base.device.logical.unmap_memory(self.font_data.staging.memory);

                let size = self.font_data.atlas_texture.staging.size;
                let data_ptr = base.device.logical.map_memory(self.font_data.atlas_texture.staging.memory, self.font_data.atlas_texture.staging.offset, size, vk::MemoryMapFlags::empty()).unwrap() as *mut u8;
                data_ptr.copy_from_nonoverlapping(self.font_data.atlas.atlas.data.as_ptr(), size as usize);
                base.device.logical.unmap_memory(self.font_data.atlas_texture.staging.memory);

				utils::image::transition_image_layout::<ImageLayout_Undefined, ImageLayout_ShaderReadOnlyOptimal>(&base.device, cb, &self.font_data.atlas_texture);
                utils::image::transition_image_layout::<ImageLayout_ShaderReadOnlyOptimal, ImageLayout_TransferDstOptimal>(&base.device, cb, &self.font_data.atlas_texture);
                utils::image::copy_buffer_to_image(&base.device, cb, &self.font_data.atlas_texture, &self.font_data.atlas_texture.staging, self.font_data.atlas.atlas.w, self.font_data.atlas.atlas.h);
                utils::image::transition_image_layout::<ImageLayout_TransferDstOptimal, ImageLayout_ShaderReadOnlyOptimal>(&base.device, cb, &self.font_data.atlas_texture);

				let copy_region = [
					vk::BufferCopy::default()
						.src_offset(self.font_data.staging.offset)
						.dst_offset(self.font_data.glyph_uniform.offset)
						.size(self.font_data.staging.size)
				];

				base.device.logical.cmd_copy_buffer(cb, self.font_data.staging.buffer, self.font_data.glyph_uniform.buffer, &copy_region);
            }

            self.initialized = true;
        }



		// let frame_time_ms = self.previous_time.elapsed().as_millis();
		// let frame_time = format!("{:>12} ", frame_time_ms);
		// self.frame_timer.set_text(&frame_time);
		// self.frame_timer.kern_text(&self.font_data.atlas);
		// self.previous_time = Instant::now();

		DrawableText::update(&base.device, cb, std::slice::from_mut(&mut self.frame_timer));
    }

    pub fn draw(&mut self, base: &mut VkBase, cb: vk::CommandBuffer, current_image: usize) {
		DrawableText::draw(&base.device, cb, &base.graphics_pipelines[ShaderText::ID], current_image, std::slice::from_ref(&self.frame_timer));
    }

    pub fn release(&mut self, base: &VkBase) {
        DrawableText::release(&base.device, std::slice::from_mut(&mut self.frame_timer));

        unsafe {
			base.device.logical.destroy_buffer(self.font_data.atlas_texture.staging.buffer, None);
            base.device.logical.free_memory(self.font_data.atlas_texture.staging.memory, None);
            base.device.logical.destroy_image(self.font_data.atlas_texture.resource.image, None);
            base.device.logical.free_memory(self.font_data.atlas_texture.resource.memory, None);
            base.device.logical.destroy_image_view(self.font_data.atlas_texture.image_view, None);
            base.device.logical.destroy_sampler(self.font_data.atlas_texture.sampler, None);
        }
    }

}
