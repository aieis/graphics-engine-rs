use std::time::Instant;

use ash::vk;
use stb_truetype::{FontAtlas, CHARS_LEN};
use winit::event::{ElementState, MouseButton};
use winit::keyboard::KeyCode;

use crate::drawable::{drawable_card::DrawableCard, drawable_text::DrawableText};
use crate::geometry::vec3::Vec3;
use crate::primitives::image::PixelFormat;
use crate::mesh::prism;
use crate::utils::colours;
use crate::rhi::{allocator::Allocator, uniform::StaticUniform, uniform::VariableUniform};
use crate::scene::camera::{Camera, CameraParams, CameraAction};
use crate::shader::{ShaderText, ShaderCard};
use crate::utils::{
    image::{ImageLayout_ShaderReadOnlyOptimal, ImageLayout_TransferDstOptimal, ImageLayout_Undefined},
    keyboard_mouse::{KeyboardMouseState, KeyMod}
};

use crate::vk_base::VkBase;
use crate::vk_bundles::TextureBundle;

macro_rules! FONT_ATLAS_PATH_MAC { () => { "../../../assets/fonts/Atlas_Iosevka_Regular_12x8_25x55_atlas.ff" }; }
macro_rules! FONT_ATLAS_DESC_PATH_MAC { () => { "../../../assets/fonts/Atlas_Iosevka_Regular_12x8_25x55_atlas_desc.bin" }; }

const FONT_ATLAS_DATA: &[u8] = include_bytes!(FONT_ATLAS_PATH_MAC!());
const FONT_ATLAS_DESC_DATA: &[u8] = include_bytes!(FONT_ATLAS_DESC_PATH_MAC!());

const CAMERA_LOCATION    : Vec3 = Vec3::new(0.0, 5.0, 1.0);
const CAMERA_DIRECTION_X : f32  = -std::f32::consts::PI / 2.0;
const CAMERA_DIRECTION_Y : f32  = -std::f32::consts::PI / 8.0 * 3.0;
const CAMERA_FOV         : f32  = std::f32::consts::PI / 3.0;

const CAMERA_MOVEMENT_SPEED: f32     = 5.0;
const CAMERA_ROTATION_SPEED_FAC: f32 = 0.5;
const CAMERA_MOUSE_DRAG_SPEED: f32   = 0.2;

const CARD_SIZE: Vec3 = Vec3::new(0.48, 0.02, 0.62);

struct SharedFontData {
    atlas: FontAtlas,
    atlas_texture: TextureBundle,
    glyph_buffer: VariableUniform,
}

pub struct ShelemScene
{
    cards : Vec<DrawableCard>,
	frame_timer: [DrawableText; 1],

    global_descriptor_set: Vec<vk::DescriptorSet>,

    camera_buffer: StaticUniform<CameraParams>,
    camera: Camera,

    font_data: SharedFontData,

    window_size: (u32, u32),
    cursor_delta: (f64, f64),
    cursor_position: (f64, f64),
    cursor_moved: bool,
    fixed_camera: bool,


    selected_card: Option<usize>,

    speed: f32,
	previous_time: Instant,
}

impl ShelemScene
{
    pub fn new(base: &VkBase, allocator: &mut Allocator) -> Self {

        let font_atlas = FontAtlas::parse_atlas_from_memory(FONT_ATLAS_DESC_DATA, FONT_ATLAS_DATA).expect("Failed to load atlas.");
        let font_atlas_texture = crate::utils::image::create_texture_image(&base.device, font_atlas.atlas.w, font_atlas.atlas.h, (font_atlas.atlas.w * font_atlas.atlas.h * 4) as u64, PixelFormat::RGBA);
        let frame_timer = [DrawableText::new(base, Vec3::new(-1.0, -0.95, 0.0), font_atlas.desc.info.clone(), allocator, "0000 ", 64)];

        let font_data = SharedFontData {
            atlas: font_atlas,
            atlas_texture: font_atlas_texture,
            glyph_buffer: VariableUniform::new(allocator, CHARS_LEN as u64 * std::mem::size_of::<u32>() as u64 * 2)
        };

        DrawableText::init_font_atlas(&base.device, &font_data.atlas_texture, &font_data.glyph_buffer.uniform, &frame_timer);

        let window_size = (512, 512);
        let camera = Self::make_camera(window_size.0 as f32, window_size.1 as f32, CAMERA_FOV);
        let camera_buffer = StaticUniform::<CameraParams>::new(allocator);

        let global_descriptor_set = VkBase::create_descriptor_sets(&base.device, base.descriptor_pool, base.global_descriptor_set_layout, base.max_in_flight);
        for descriptor_set in global_descriptor_set.iter() {
            VkBase::update_descriptor_set_buffers(&base.device, *descriptor_set, &[&camera_buffer.uniform], 0);
        }

        const DC : f32 = 0.1 * CARD_SIZE.x;
        const D  : f32 = CARD_SIZE.x + DC;
        const N  : f32 = 3.0;
        const W  : f32 = N * CARD_SIZE.x + (N-1.0) * DC;
        const S  : f32 = - W / 2.0 + CARD_SIZE.x / 2.0;

        let mut cards = Vec::new();
        for i in 0..N as usize {
            cards.push(DrawableCard::new(allocator, prism::make_prism(Vec3::new(S + D * i as f32, 0.0, -2.0), CARD_SIZE, colours::METAL_GREY)));
            cards.push(DrawableCard::new(allocator, prism::make_prism(Vec3::new(S + D * i as f32, 0.0,  2.0), CARD_SIZE, colours::METAL_GREY)));
        }


        Self {
            cards,

            frame_timer,
            global_descriptor_set,

            font_data,

            camera,
            camera_buffer,

            window_size,
            speed: CAMERA_MOVEMENT_SPEED,
            cursor_delta: (0.0, 0.0),
            cursor_position: (0.0, 0.0),
            cursor_moved: false,
            fixed_camera: true,
            selected_card: None,
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

    pub fn handle_cursor_moved(&mut self, position: (f64, f64)) {
        self.cursor_position = position;
        self.cursor_moved = true;
    }

    pub fn handle_mouse_button_event(&mut self, state: ElementState, button: MouseButton) {
        if button == MouseButton::Left && state == ElementState::Pressed {
            self.cursor_delta = (0.0, 0.0)
        }
    }

    pub fn handle_mouse_motion(&mut self, delta: (f64, f64), keyboard_state: &KeyboardMouseState) {

        if keyboard_state[MouseButton::Left] {

            self.cursor_delta = (self.cursor_delta.0 + delta.0, self.cursor_delta.1 + delta.1);

            const MIN_DISP: f64 = 2.0;
            if self.cursor_delta.0.abs() >= MIN_DISP ||  self.cursor_delta.1.abs() >= MIN_DISP {
                if self.cursor_delta.0.abs() >= MIN_DISP {
                    let dx = CAMERA_MOUSE_DRAG_SPEED * self.cursor_delta.0 as f32 / (self.window_size.0 as f32  / 2.0 ) * std::f32::consts::PI;
                    self.camera.update(CameraAction::RotateX, dx);
                }

                if self.cursor_delta.1.abs() >= MIN_DISP {
                    let dy = CAMERA_MOUSE_DRAG_SPEED * self.cursor_delta.1 as f32 / (self.window_size.1 as f32  / 2.0 ) * std::f32::consts::PI;
                    self.camera.update(CameraAction::RotateY, -dy);
                }

                self.cursor_delta = (0.0, 0.0);
            }
        }
    }


    pub fn handle_key(&mut self, key: KeyCode, state: ElementState, _repeat: bool, keyboard_state: &KeyboardMouseState) {

        if state != ElementState::Pressed {
            return;
        }

        match key {

            KeyCode::ArrowLeft => {
                self.select_previous_card();
            }

            KeyCode::ArrowRight => {
                self.select_next_card();
            }


            _ => {

            }
        }

        // Decide if to skp all the camera options or not
        if self.fixed_camera {

            if keyboard_state.is_mod_req_met(KeyMod::None) && key == KeyCode::KeyO {
                self.fixed_camera = false;
            }

            return;
        }

        if keyboard_state.is_mod_req_met(KeyMod::None) {
            match key {
                KeyCode::KeyO => {

                    if self.fixed_camera {
                        self.fixed_camera = false;
                    } else {
                        self.fixed_camera = true;
                        self.reset_camera();
                    }

                }

                KeyCode::KeyT => {
                    self.reset_camera();
                }

                KeyCode::BracketLeft => {
                    self.camera.update(CameraAction::SnapPosX, -0.1);
                }

                KeyCode::BracketRight => {
                    self.camera.update(CameraAction::SnapPosX, 0.1);
                }
                _ => {}
            }
        } else if keyboard_state.is_mod_req_met(KeyMod::Shift) {
            match key {
                KeyCode::BracketLeft => {
                    self.camera.update(CameraAction::SnapPosY, -0.1);
                }

                KeyCode::BracketRight => {
                    self.camera.update(CameraAction::SnapPosY, 0.1);
                },

                _ => {}
            }
        } else if keyboard_state.is_mod_req_met(KeyMod::Ctrl) {
            match key {
                KeyCode::BracketLeft => {
                    self.camera.update(CameraAction::SnapDirX, 0.0);
                }

                KeyCode::BracketRight => {
                    self.camera.update(CameraAction::SnapDirY, 0.0);
                }

                _ => {}
            }
        }
    }

    fn handle_down_keys(&mut self, keyboard_state: &KeyboardMouseState, delta_time: f32) {

        // camera

        if self.fixed_camera {
            return;
        }


        if keyboard_state.is_mod_req_met(KeyMod::None) {
            if keyboard_state[KeyCode::KeyA] {
                self.camera.update(CameraAction::Left, delta_time * self.speed);
            }

            if keyboard_state[KeyCode::KeyD] {
                self.camera.update(CameraAction::Right, delta_time * self.speed);
            }

            if keyboard_state[KeyCode::KeyW] {
                self.camera.update(CameraAction::Forward, delta_time * self.speed);
            }

            if keyboard_state[KeyCode::KeyS] {
                self.camera.update(CameraAction::Backward, delta_time * self.speed);
            }

            if keyboard_state[KeyCode::KeyE] {
                self.camera.update(CameraAction::Up, delta_time * self.speed);
            }

            if keyboard_state[KeyCode::KeyQ] {
                self.camera.update(CameraAction::Down, delta_time * self.speed);
            }
        } else if keyboard_state.is_mod_req_met(KeyMod::Ctrl) {
            if keyboard_state[KeyCode::KeyA] {
                self.camera.update(CameraAction::RotateX, -delta_time * self.speed * CAMERA_ROTATION_SPEED_FAC);
            }

            if keyboard_state[KeyCode::KeyD] {
                self.camera.update(CameraAction::RotateX, delta_time * self.speed * CAMERA_ROTATION_SPEED_FAC);
            }

            if keyboard_state[KeyCode::KeyW] {
                self.camera.update(CameraAction::RotateY, -delta_time * self.speed * CAMERA_ROTATION_SPEED_FAC);
            }

            if keyboard_state[KeyCode::KeyS] {
                self.camera.update(CameraAction::RotateY, delta_time * self.speed * CAMERA_ROTATION_SPEED_FAC);
            }
        }

    }

    pub fn handle_resize_event(&mut self, window_size: (u32, u32)) {
        self.window_size = window_size;
        self.camera.on_view_proj_changes(self.window_size.0 as f32, self.window_size.1 as f32, CAMERA_FOV);
    }

    fn reset_camera(&mut self) {
        self.camera = Self::make_camera(self.window_size.0 as f32, self.window_size.1 as f32, CAMERA_FOV);
    }

    fn make_camera(width: f32, height: f32, fov: f32) -> Camera {
        return Camera::new(CAMERA_LOCATION, CAMERA_DIRECTION_X, CAMERA_DIRECTION_Y, width, height, fov);
    }

    pub fn update(&mut self, base: &VkBase, cb: vk::CommandBuffer, keyboard_state: &KeyboardMouseState, delta_time: f32) {
        let aspect_ratio = self.window_size.0 as f32 / self.window_size.1 as f32;
        self.handle_down_keys(keyboard_state, delta_time);
        self.camera_buffer.update(&base.device, cb, &self.camera.params);

        if self.cursor_moved {

            if let Some(idx) = self.find_item_under_cursor() {
                if let Some(prev_idx) = self.selected_card {
                    if idx != prev_idx {
                        self.unselect_card();
                        self.select_card(idx);
                    }
                } else {
                    self.select_card(idx);
                }
            } else {
                self.unselect_card();
            }

            self.cursor_moved = false;
        }

		let frame_time_ms = self.previous_time.elapsed().as_millis();
		let frame_time = format!("{:>12} ", frame_time_ms);
		self.frame_timer[0].set_text(&frame_time);
		self.frame_timer[0].kern_text(&self.font_data.atlas);
		self.previous_time = Instant::now();

        DrawableCard::update(&base.device, cb, &mut self.cards);
		DrawableText::update(&base.device, cb, &mut self.frame_timer, 1024.0, aspect_ratio);
    }

    pub fn draw(&mut self, base: &mut VkBase, cb: vk::CommandBuffer, current_image: usize) {
        let pso = &base.graphics_pipelines[ShaderCard::ID];

        unsafe {
            base.device.logical.cmd_bind_pipeline(cb, vk::PipelineBindPoint::GRAPHICS, pso.graphics);
        }

        unsafe {
            base.device.logical.cmd_bind_descriptor_sets(cb, vk::PipelineBindPoint::GRAPHICS, pso.layout, 0, &[self.global_descriptor_set[current_image]], &[]);
        }

        DrawableCard::draw(&base.device, cb, &base.graphics_pipelines[ShaderCard::ID], &self.cards);
        DrawableText::draw(&base.device, cb, &base.graphics_pipelines[ShaderText::ID], current_image, &self.frame_timer);
    }


    fn select_previous_card(&mut self) {

        if let Some(idx) = self.selected_card {
            self.unselect_card();
            if idx > 0 {
                self.select_card(idx - 1);
            }
        } else {
            self.select_card(self.cards.len() - 1);
        }

    }

    fn select_next_card(&mut self) {
        if let Some(idx) = self.selected_card {
            self.unselect_card();
            if idx < self.cards.len() - 1 {
                self.select_card(idx + 1);
            }
        } else {
            self.select_card(0);
        }

    }

    fn select_card(&mut self, idx: usize) {
        self.selected_card = Some(idx);
        self.cards[idx].mesh.set_colour(colours::VIOLET);
    }

    fn unselect_card(&mut self) {
        if let Some(idx) = self.selected_card {
            self.cards[idx].mesh.set_colour(colours::METAL_GREY);
            self.selected_card = None;
        }
    }

    pub fn find_item_under_cursor(&self) -> Option<usize> {
        None
    }

    pub fn release(&mut self, base: &VkBase) {

        DrawableText::release(&base.device, &mut self.frame_timer);
        DrawableCard::release(&base.device, &mut self.cards);

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
