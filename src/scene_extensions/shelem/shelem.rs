use std::time::Instant;

use ash::vk;
use stb_truetype::{FontAtlas, CHARS_LEN};
use winit::event::{ElementState, MouseButton};
use winit::keyboard::KeyCode;

use crate::drawable::{drawable_card::DrawableCard, drawable_text::DrawableText};
use crate::geometry::vec3::Vec3;
use crate::primitives::image::PixelFormat;
use crate::rhi::{allocator::Allocator, uniform::StaticUniform, uniform::VariableUniform};
use crate::scene::camera::{Camera, CameraParams};
use crate::shader::{ShaderText, ShaderCard};
use crate::utils::{
    image::{ImageLayout_ShaderReadOnlyOptimal, ImageLayout_TransferDstOptimal, ImageLayout_Undefined},
    keyboard_mouse::KeyboardMouseState
};

use crate::vk_base::VkBase;
use crate::vk_bundles::TextureBundle;

macro_rules! FONT_ATLAS_PATH_MAC { () => { "../../../assets/fonts/Atlas_Iosevka_Regular_12x8_25x55_atlas.ff" }; }
macro_rules! FONT_ATLAS_DESC_PATH_MAC { () => { "../../../assets/fonts/Atlas_Iosevka_Regular_12x8_25x55_atlas_desc.bin" }; }

const FONT_ATLAS_DATA: &[u8] = include_bytes!(FONT_ATLAS_PATH_MAC!());
const FONT_ATLAS_DESC_DATA: &[u8] = include_bytes!(FONT_ATLAS_DESC_PATH_MAC!());

const CAMERA_LOCATION    : Vec3 = Vec3::new(0.0, 0.0, 10.0);
const CAMERA_DIRECTION_X : f32  =  std::f32::consts::PI / 2.0;
const CAMERA_DIRECTION_Y : f32  = -std::f32::consts::PI / 4.0;

const CAMERA_MOVEMENT_SPEED: f32     = 5.0;
const CAMERA_ROTATION_SPEED_FAC: f32 = 0.5;
const CAMERA_MOUSE_DRAG_SPEED: f32   = 0.2;

struct SharedFontData {
    atlas: FontAtlas,
    atlas_texture: TextureBundle,
    glyph_buffer: VariableUniform,
}

pub struct ShelemScene
{
    cards : Vec<DrawableCard>,
    time  : Instant,
	frame_timer: [DrawableText; 1],

    global_descriptor_set: Vec<vk::DescriptorSet>,

    camera_buffer: StaticUniform<CameraParams>,
    camera: Camera,

    font_data: SharedFontData,

    window_size: (u32, u32),

	previous_time: Instant,
}

impl ShelemScene
{
    pub fn new(base: &VkBase, allocator: &mut Allocator) -> Self {

        let time = Instant::now();

        let font_atlas = FontAtlas::parse_atlas_from_memory(FONT_ATLAS_DESC_DATA, FONT_ATLAS_DATA).expect("Failed to load atlas.");
        let font_atlas_texture = crate::utils::image::create_texture_image(&base.device, font_atlas.atlas.w, font_atlas.atlas.h, (font_atlas.atlas.w * font_atlas.atlas.h * 4) as u64, PixelFormat::RGBA);
        let frame_timer = [DrawableText::new(base, Vec3::new(-1.0, -0.95, 0.0), font_atlas.desc.info.clone(), allocator, "0000 ", 64)];

        let font_data = SharedFontData {
            atlas: font_atlas,
            atlas_texture: font_atlas_texture,
            glyph_buffer: VariableUniform::new(allocator, CHARS_LEN as u64 * std::mem::size_of::<u32>() as u64 * 2)
        };

        DrawableText::init_font_atlas(&base.device, &font_data.atlas_texture, &font_data.glyph_buffer.uniform, &frame_timer);

        let camera = Self::make_camera();
        let camera_buffer = StaticUniform::<CameraParams>::new(allocator);

        let global_descriptor_set = VkBase::create_descriptor_sets(&base.device, base.descriptor_pool, base.global_descriptor_set_layout, base.max_in_flight);
        for descriptor_set in global_descriptor_set.iter() {
            VkBase::update_descriptor_set_buffers(&base.device, *descriptor_set, &[&camera_buffer.uniform], 0);
        }

        Self {
            cards: vec![],

            time,
            frame_timer,
            global_descriptor_set,

            font_data,

            camera,
            camera_buffer,

            window_size: (0, 0),
            previous_time: Instant::now(),
        }
    }

    pub fn initialize_scene(&mut self, base: &VkBase, cb: vk::CommandBuffer) {
        let data: [(u32, u32); CHARS_LEN] = self.font_data.atlas.desc.glyph_info.each_ref().map(|g| { (g.w as u32, g.h as u32) });
        self.font_data.glyph_buffer.update(&base.device, cb, &data);

        let size = self.font_data.atlas_texture.staging.size;

        unsafe {
            let data_ptr = base.device.logical.map_memory(self.font_data.atlas_texture.staging.memory, self.font_data.atlas_texture.staging.offset, size, vk::MemoryMapFlags::empty()).unwrap() as *mut u8;
            data_ptr.copy_from_nonoverlapping(self.font_data.atlas.atlas.data.as_ptr(), size as usize);
            base.device.logical.unmap_memory(self.font_data.atlas_texture.staging.memory);
        }

		crate::utils::image::transition_image_layout::<ImageLayout_Undefined, ImageLayout_ShaderReadOnlyOptimal>(&base.device, cb, &self.font_data.atlas_texture);
        crate::utils::image::transition_image_layout::<ImageLayout_ShaderReadOnlyOptimal, ImageLayout_TransferDstOptimal>(&base.device, cb, &self.font_data.atlas_texture);
        crate::utils::image::copy_buffer_to_image(&base.device, cb, &self.font_data.atlas_texture, &self.font_data.atlas_texture.staging, self.font_data.atlas.atlas.w, self.font_data.atlas.atlas.h);
        crate::utils::image::transition_image_layout::<ImageLayout_TransferDstOptimal, ImageLayout_ShaderReadOnlyOptimal>(&base.device, cb, &self.font_data.atlas_texture);
    }

    pub fn handle_mouse_button_event(&mut self, _state: ElementState, _button: MouseButton) {

    }

    pub fn handle_mouse_motion(&mut self, _delta: (f64, f64), keyboard_state: &KeyboardMouseState) {

        if keyboard_state[MouseButton::Left] {

        }
    }


    pub fn handle_key(&mut self, _key: KeyCode, _state: ElementState, _repeat: bool, _keyboard_state: &KeyboardMouseState) {

    }

    fn handle_down_keys(&mut self, _keyboard_state: &KeyboardMouseState, _delta_time: f32) {

    }

    fn make_camera() -> Camera {
        return Camera::new(CAMERA_LOCATION, CAMERA_DIRECTION_X, CAMERA_DIRECTION_Y);
    }

    pub fn update(&mut self, base: &VkBase, cb: vk::CommandBuffer, window_size: (u32, u32), keyboard_state: &KeyboardMouseState, delta_time: f32) {

        self.window_size = window_size;
        let aspect_ratio = window_size.0 as f32 / window_size.1 as f32;
        self.handle_down_keys(keyboard_state, delta_time);
        self.camera_buffer.update(&base.device, cb, &self.camera.params);

		let frame_time_ms = self.previous_time.elapsed().as_millis();
		let frame_time = format!("{:>12} ", frame_time_ms);
		self.frame_timer[0].set_text(&frame_time);
		self.frame_timer[0].kern_text(&self.font_data.atlas);
		self.previous_time = Instant::now();

        DrawableCard::update(&base.device, cb, &mut self.cards);
		DrawableText::update(&base.device, cb, &mut self.frame_timer, 1024.0, aspect_ratio);
    }

    pub fn draw(&mut self, base: &mut VkBase, cb: vk::CommandBuffer, current_image: usize) {
        DrawableCard::draw(&base.device, cb, &base.graphics_pipelines[ShaderCard::ID], &self.cards);
        DrawableText::draw(&base.device, cb, &base.graphics_pipelines[ShaderText::ID], current_image, &self.frame_timer);
    }

    pub fn release(&mut self, base: &VkBase) {

        DrawableText::release(&base.device, &mut self.frame_timer);

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
