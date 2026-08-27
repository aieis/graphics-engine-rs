use std::time::Instant;

use ash::vk;
use stb_truetype::{FontAtlas, CHARS_LEN};
use winit::event::ElementState;
use winit::keyboard::KeyCode;

use crate::ShaderRect;
use crate::drawable::{drawable2d::Drawable2d, drawable_text::DrawableText};
use crate::geometry::vec3::Vec3;
use crate::mesh::prism;
use crate::primitives::image::PixelFormat;
use crate::utils;
use crate::utils::image::{ImageLayout_ShaderReadOnlyOptimal, ImageLayout_TransferDstOptimal, ImageLayout_Undefined};
use crate::utils::keyboard::{KeyboardState, KeyMod};
use crate::vk_bundles::TextureBundle;
use crate::{drawable::drawable_mesh::DrawableMesh, vk_base::VkBase};
use crate::shader::{ShaderSpecialMesh, ShaderText};
use crate::rhi::{allocator::Allocator, uniform::StaticUniform, uniform::VariableUniform};
use crate::scene::camera::{Camera, CameraParams, CameraAction};

macro_rules! FONT_ATLAS_PATH_MAC { () => { "../../assets/fonts/Atlas_Iosevka_Regular_12x8_25x55_atlas.ff" }; }
macro_rules! FONT_ATLAS_DESC_PATH_MAC { () => { "../../assets/fonts/Atlas_Iosevka_Regular_12x8_25x55_atlas_desc.bin" }; }

const FONT_ATLAS_DATA: &[u8] = include_bytes!(FONT_ATLAS_PATH_MAC!());
const FONT_ATLAS_DESC_DATA: &[u8] = include_bytes!(FONT_ATLAS_DESC_PATH_MAC!());


const CAMERA_LOCATION  : Vec3 = Vec3::new(0.0, 0.0, 10.0);
const CAMERA_DIRECTION : Vec3 = Vec3::new(0.0, 0.0, -1.0);

#[repr(C)]
struct SpecialMeshShaderParams {
    time: f32,
    aspect: f32,
    global_camera: f32
}

struct SharedFontData {
    atlas: FontAtlas,
    atlas_texture: TextureBundle,
    glyph_buffer: VariableUniform,
}

pub struct SimpleScene
{
    pub time           : Instant,
    pub static_meshes  : Vec<DrawableMesh>,
    pub dynamic_meshes : Vec<DrawableMesh>,

    rect_bundles: Vec<Drawable2d>,
	frame_timer: [DrawableText; 1],

	descriptor_sets: Vec<vk::DescriptorSet>,
    global_descriptor_set: Vec<vk::DescriptorSet>,

    special_mesh_params_buffer: StaticUniform<SpecialMeshShaderParams>,

    camera_buffer: StaticUniform<CameraParams>,
    camera: Camera,

    font_data: SharedFontData,

    use_global_camera: bool,
    going_down: bool,
    translation_amount: f32,
    speed: f32,

	previous_time: Instant,

    initialized: bool,

    pub activated: bool,
}

impl SimpleScene
{
    pub fn new(base: &VkBase, allocator: &mut Allocator) -> SimpleScene {

        let floor = prism::make_prism(Vec3::new(0.0, -5.0, 0.0), Vec3::new(20.0, 1.0, 20.0), Vec3::of(0.2));

        let prism_b = prism::make_debug_prism(Vec3::new(0.0, 0.0, -5.0), Vec3::new(2.0, 3.0, 8.0));

        let mut cube_c = prism::make_prism(Vec3::new(5.0, 0.0, 0.0), Vec3::of(0.5), Vec3::new(0.0, 0.0, 1.0));
        cube_c.rotate_x(45_f32.to_radians());
        cube_c.rotate_z(45_f32.to_radians());
        cube_c.recompute_normals();

        let cube_d = prism::make_prism(Vec3::new(-5.0, 0.0, 0.0), Vec3::of(0.5), Vec3::new(10.0, 0.0, 0.0));
        let cube_e = prism::make_debug_prism(Vec3::new(0.0, 0.0, 30.0), Vec3::of(5.0));

        let static_meshes = vec![
            DrawableMesh::new(&base.device, floor),
        ];

        let dynamic_meshes = vec![
            DrawableMesh::new(&base.device, prism_b),
            DrawableMesh::new(&base.device, cube_c),
            DrawableMesh::new(&base.device, cube_d),
            DrawableMesh::new(&base.device, cube_e),
        ];

        let special_mesh_params_buffer = StaticUniform::<SpecialMeshShaderParams>::new(allocator);

		let pso = &base.graphics_pipelines[ShaderSpecialMesh::ID];
		let layout = pso.ubo.as_ref().expect("Expected ubo to be defined.").layouts[0];
		let descriptor_sets = VkBase::create_descriptor_sets(&base.device, base.descriptor_pool, layout, base.max_in_flight);
        for descriptor_set in descriptor_sets.iter() {
            VkBase::update_descriptor_set_buffers(&base.device, *descriptor_set, &[&special_mesh_params_buffer.uniform], 0);
        }

        let time = Instant::now();

        let font_atlas = FontAtlas::parse_atlas_from_memory(FONT_ATLAS_DESC_DATA, FONT_ATLAS_DATA).expect("Failed to load atlas.");
        let font_atlas_texture = utils::image::create_texture_image(&base.device, font_atlas.atlas.w, font_atlas.atlas.h, (font_atlas.atlas.w * font_atlas.atlas.h * 4) as u64, PixelFormat::RGBA);
        let frame_timer = [DrawableText::new(base, Vec3::new(-1.0, -0.95, 0.0), font_atlas.desc.info.clone(), allocator, "0000 ", 64)];

        let font_data = SharedFontData {
            atlas: font_atlas,
            atlas_texture: font_atlas_texture,
            glyph_buffer: VariableUniform::new(allocator, CHARS_LEN as u64 * std::mem::size_of::<u32>() as u64 * 2)
        };

        DrawableText::init_font_atlas(&base.device, &font_data.atlas_texture, &font_data.glyph_buffer.uniform, &frame_timer);

        let rect_bundles = vec![
            // Drawable2d::new(&base.device, RectMesh::new(-0.25, -0.25, 0.5, 0.5, [0.0, 0.01, 0.01]))
        ];

        let camera = Self::make_camera();
        let camera_buffer = StaticUniform::<CameraParams>::new(allocator);

        let global_descriptor_set = VkBase::create_descriptor_sets(&base.device, base.descriptor_pool, base.global_descriptor_set_layout, base.max_in_flight);

        Self {
            time,
            static_meshes,
            dynamic_meshes,

            frame_timer,
            descriptor_sets,
            global_descriptor_set,

            font_data,
            rect_bundles,

            special_mesh_params_buffer,
            camera,
            camera_buffer,

            use_global_camera: false,
            going_down: false,
            translation_amount: 0.0,
            speed: 1.0,
            previous_time: Instant::now(),
            initialized: false,
            activated: false,
        }
    }

    pub fn handle_key(&mut self, key: KeyCode, state: ElementState, _repeat: bool, keyboard_state: &KeyboardState) {

        if state != ElementState::Pressed {
            return;
        }

        const ACTION_MAP: [((KeyMod, KeyCode), fn(&mut SimpleScene)); 2] = [
            ((KeyMod::None  , KeyCode::KeyO), |s: &mut SimpleScene| { s.use_global_camera = !s.use_global_camera; }),
            ((KeyMod::None  , KeyCode::KeyT), |s: &mut SimpleScene| { s.reset_camera(); }),
        ];

        for a in ACTION_MAP {
            let ((modifiers, key), action) = a;
            if keyboard_state.is_key_down(modifiers, key) {
                action(self);
            }
        }

        // match key {

        //     KeyCode::KeyO => {
        //         self.use_global_camera = !self.use_global_camera;
        //     }

        //     KeyCode::KeyT => {
        //         self.reset_camera();
        //     }

        //     KeyCode::BracketLeft => {
        //         if keyboard_state[KeyCode::ShiftLeft] || keyboard_state[KeyCode::ShiftRight] {
        //             self.camera.update(CameraAction::SnapPosY, -0.1);
        //         } else {
        //             self.camera.update(CameraAction::SnapPosX, -0.1);
        //         }
        //     }

        //     KeyCode::BracketRight => {
        //         if keyboard_state[KeyCode::ShiftLeft] || keyboard_state[KeyCode::ShiftRight] {
        //             self.camera.update(CameraAction::SnapPosY, 0.1);
        //         } else {
        //             self.camera.update(CameraAction::SnapPosX, 0.1);
        //         }
        //     }

        //     _ => {

        //     }
        // }
    }

    fn handle_down_keys(&mut self, keyboard_state: &KeyboardState, delta_time: f32) {
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

        if keyboard_state[KeyCode::KeyQ] {
            self.camera.update(CameraAction::Down, delta_time * self.speed);
        }
    }


    fn reset_camera(&mut self) {
        self.camera = Self::make_camera();
    }

    fn make_camera() -> Camera{
        return Camera::new(CAMERA_LOCATION, CAMERA_DIRECTION);
    }

    pub fn update(&mut self, base: &VkBase, cb: vk::CommandBuffer, aspect_ratio: f32, keyboard_state: &KeyboardState, delta_time: f32) {

        self.handle_down_keys(keyboard_state, delta_time);
        self.camera_buffer.update(&base.device, cb, &self.camera.params);

        Drawable2d::update(&base.device, cb, &mut self.rect_bundles);

        let mut v = 1e-2;

        const TRANSLATION_MAX: f32 = 1.0;
        if self.translation_amount >= TRANSLATION_MAX {
            self.going_down = !self.going_down;
            self.translation_amount = 0.0;
        } else {
            self.translation_amount += v;
            v *= (TRANSLATION_MAX - self.translation_amount) / TRANSLATION_MAX;
        }

        let d = Vec3::Y * 0.5;

        let v = if self.going_down { d * -v } else { d * v };

        for mesh in self.dynamic_meshes.iter_mut() {
            mesh.mesh.translate(v);
            mesh.mesh.rotate_y(1e-2);
            mesh.mesh.recompute_normals();
        }

        DrawableMesh::update(&base.device, cb, &mut self.dynamic_meshes);
        DrawableMesh::update(&base.device, cb, &mut self.static_meshes);

        let params = SpecialMeshShaderParams {
            time: self.time.elapsed().as_secs_f32(),
            aspect: aspect_ratio,
            global_camera: if self.use_global_camera { 1.0 } else { -1.0 },
        };

        self.special_mesh_params_buffer.update(&base.device, cb, &params);

        if !self.initialized {

            let data: [(u32, u32); CHARS_LEN] = self.font_data.atlas.desc.glyph_info.each_ref().map(|g| { (g.w as u32, g.h as u32) });
            self.font_data.glyph_buffer.update(&base.device, cb, &data);

            unsafe {
                let size = self.font_data.atlas_texture.staging.size;
                let data_ptr = base.device.logical.map_memory(self.font_data.atlas_texture.staging.memory, self.font_data.atlas_texture.staging.offset, size, vk::MemoryMapFlags::empty()).unwrap() as *mut u8;
                data_ptr.copy_from_nonoverlapping(self.font_data.atlas.atlas.data.as_ptr(), size as usize);
                base.device.logical.unmap_memory(self.font_data.atlas_texture.staging.memory);

				utils::image::transition_image_layout::<ImageLayout_Undefined, ImageLayout_ShaderReadOnlyOptimal>(&base.device, cb, &self.font_data.atlas_texture);
                utils::image::transition_image_layout::<ImageLayout_ShaderReadOnlyOptimal, ImageLayout_TransferDstOptimal>(&base.device, cb, &self.font_data.atlas_texture);
                utils::image::copy_buffer_to_image(&base.device, cb, &self.font_data.atlas_texture, &self.font_data.atlas_texture.staging, self.font_data.atlas.atlas.w, self.font_data.atlas.atlas.h);
                utils::image::transition_image_layout::<ImageLayout_TransferDstOptimal, ImageLayout_ShaderReadOnlyOptimal>(&base.device, cb, &self.font_data.atlas_texture);
            }

            self.initialized = true;
        }

        if !self.activated {

            for descriptor_set in self.global_descriptor_set.iter() {
                VkBase::update_descriptor_set_buffers(&base.device, *descriptor_set, &[&self.camera_buffer.uniform], 0);
            }

            self.activated = true;
        }


		let frame_time_ms = self.previous_time.elapsed().as_millis();
		let frame_time = format!("{:>12} ", frame_time_ms);
		self.frame_timer[0].set_text(&frame_time);
		self.frame_timer[0].kern_text(&self.font_data.atlas);
		self.previous_time = Instant::now();

		DrawableText::update(&base.device, cb, &mut self.frame_timer, 1024.0, aspect_ratio);
    }

    pub fn draw(&mut self, base: &mut VkBase, cb: vk::CommandBuffer, current_image: usize) {
        if !self.activated {
            return;
        }

        Drawable2d::draw(&base.device, cb, &base.graphics_pipelines[ShaderRect::ID], &self.rect_bundles);

        let pso = &base.graphics_pipelines[ShaderSpecialMesh::ID];

        unsafe {
            base.device.logical.cmd_bind_pipeline(cb, vk::PipelineBindPoint::GRAPHICS, pso.graphics);
        }

        unsafe {
            base.device.logical.cmd_bind_descriptor_sets(cb, vk::PipelineBindPoint::GRAPHICS, pso.layout, 0, &[self.global_descriptor_set[current_image]], &[]);
        }

		let set = &self.descriptor_sets[current_image..current_image+1];

        unsafe {
            base.device.logical.cmd_bind_descriptor_sets(cb, vk::PipelineBindPoint::GRAPHICS, pso.layout, 1, set, &[]);
        }

        DrawableMesh::draw(&base.device, cb, pso, &self.static_meshes);
        DrawableMesh::draw(&base.device, cb, pso, &self.dynamic_meshes);
        DrawableText::draw(&base.device, cb, &base.graphics_pipelines[ShaderText::ID], current_image, &self.frame_timer);
    }

    pub fn release(&mut self, base: &VkBase) {
        Drawable2d::release(&base.device, &mut self.rect_bundles);
        self.rect_bundles.clear();

        DrawableMesh::release(&base.device, &mut self.dynamic_meshes);
        self.dynamic_meshes.clear();
        DrawableMesh::release(&base.device, &mut self.static_meshes);
        self.static_meshes.clear();
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
