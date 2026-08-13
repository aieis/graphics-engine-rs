use std::time::Instant;

use ash::vk;
use stb_truetype::{FontAtlas, parse_font_atlas_info, CHARS_LEN};
use winit::event::ElementState;
use winit::keyboard::KeyCode;

use crate::drawable::drawable_text::DrawableText;
use crate::geometry::vec3::Vec3;
use crate::mesh::prism;
use crate::primitives::image::PixelFormat;
use crate::rhi::allocator::{Allocator, BufferType};
use crate::utils;
use crate::utils::image::{ImageLayout_ShaderReadOnlyOptimal, ImageLayout_TransferDstOptimal, ImageLayout_Undefined};
use crate::vk_bundles::{BufferBundle, TextureBundle};
use crate::{drawable::drawable_mesh::DrawableMesh, vk_base::VkBase};
use crate::shader::{ShaderSpecialMesh, ShaderText};


macro_rules! FONT_ATLAS_PATH_MAC { () => { "../../assets/fonts/Atlas_Iosevka_Regular_12x8_25x55_atlas.ff" }; }
macro_rules! FONT_ATLAS_DESC_PATH_MAC { () => { "../../assets/fonts/Atlas_Iosevka_Regular_12x8_25x55_atlas_desc.bin" }; }


const FONT_ATLAS_PATH: &str  = FONT_ATLAS_PATH_MAC!();
const FONT_ATLAS_DATA: &[u8] = include_bytes!(FONT_ATLAS_PATH_MAC!());
const FONT_ATLAS_DESC_DATA: &[u8] = include_bytes!(FONT_ATLAS_DESC_PATH_MAC!());

#[repr(C)]
struct SpecialMeshShaderParams {
    time: f32,
    aspect: f32,
    global_camera: f32
}

struct SharedFontData {
    atlas: FontAtlas,
    atlas_texture: TextureBundle,

    staging: BufferBundle,
    glyph_uniform: BufferBundle,
}

pub struct SimpleScene
{
    pub time            : Instant,

    pub static_meshes  : Vec<DrawableMesh>,
    pub dynamic_meshes : Vec<DrawableMesh>,

	frame_timer: [DrawableText; 1],

	descriptor_sets: Vec<vk::DescriptorSet>,
    staging: BufferBundle,
    uniform: BufferBundle,

    font_data: SharedFontData,

    use_global_camera: bool,
    going_down: bool,
    translation_amount: f32,

	previous_time: Instant,

    initialized: bool
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

        let staging = allocator.alloc(BufferType::Staging, std::mem::size_of::<SpecialMeshShaderParams>() as u64).unwrap();
        let uniform = allocator.alloc(BufferType::Uniform, std::mem::size_of::<SpecialMeshShaderParams>() as u64).unwrap();

		let pso = &base.graphics_pipelines[ShaderSpecialMesh::ID];
		let layout = pso.ubo.as_ref().expect("Expected ubo to be defined.").layouts[0];
		let descriptor_sets = VkBase::create_descriptor_sets(&base.device, base.descriptor_pool, layout, base.max_in_flight);
        for descriptor_set in descriptor_sets.iter() {
            VkBase::update_descriptor_set_buffers(&base.device, *descriptor_set, &[&uniform], 0);
        }

        let time = Instant::now();

        let font_atlas = FontAtlas::parse_atlas_from_memory(FONT_ATLAS_DESC_DATA, FONT_ATLAS_DATA).expect("Failed to load atlas.");
        let font_atlas_texture = utils::image::create_texture_image(&base.device, font_atlas.atlas.w, font_atlas.atlas.h, (font_atlas.atlas.w * font_atlas.atlas.h * 4) as u64, PixelFormat::RGBA);
        let frame_timer = [DrawableText::new(base, Vec3::new(-0.8, -0.8, 0.0), font_atlas.desc.info.clone(), allocator, "0000 ", 64)];

        let font_data = SharedFontData {
            atlas: font_atlas,
            atlas_texture: font_atlas_texture,
            staging: allocator.alloc(BufferType::Staging, CHARS_LEN as u64 * std::mem::size_of::<u32>() as u64 * 2).unwrap(),
            glyph_uniform: allocator.alloc(BufferType::Uniform, CHARS_LEN as u64 * std::mem::size_of::<u32>() as u64 * 2).unwrap(),
        };

        DrawableText::init_font_atlas(&base.device, &font_data.atlas_texture, &font_data.glyph_uniform, &frame_timer);

        Self {
            time,
            static_meshes,
            dynamic_meshes,
            staging,
            uniform,
            use_global_camera: false,
            going_down: false,
            translation_amount: 0.0,
            frame_timer,
            initialized: false,
            descriptor_sets,
            previous_time: Instant::now(),
            font_data,
        }
    }

    pub fn handle_key(scenes: &mut [SimpleScene], key: KeyCode, state: ElementState, _repeat: bool) {

        if state != ElementState::Pressed {
            return;
        }

        match key {

            KeyCode::KeyO => {
                for scene in scenes.iter_mut() {
                    scene.use_global_camera = !scene.use_global_camera;
                }
            }

            _ => {

            }
        }
    }

    pub fn update(scenes: &mut [SimpleScene], base: &VkBase, cb: &vk::CommandBuffer, aspect_ratio: f32) {
        for scene in scenes.iter_mut() {

            let mut v = 1e-2;

            const TRANSLATION_MAX: f32 = 1.0;
            if scene.translation_amount >= TRANSLATION_MAX {
                scene.going_down = !scene.going_down;
                scene.translation_amount = 0.0;
            } else {
                scene.translation_amount += v;
                v *= (TRANSLATION_MAX - scene.translation_amount) / TRANSLATION_MAX;
            }

            let d = Vec3::Y * 0.5;

            let v = if scene.going_down { d * -v } else { d * v };

            for mesh in scene.dynamic_meshes.iter_mut() {
                mesh.mesh.translate(v);
                mesh.mesh.rotate_y(1e-2);
                mesh.mesh.recompute_normals();
            }

            DrawableMesh::update(&base.device, &cb, &mut scene.dynamic_meshes);
            DrawableMesh::update(&base.device, &cb, &mut scene.static_meshes);

            let params = SpecialMeshShaderParams {
                time: scene.time.elapsed().as_secs_f32(),
                aspect: aspect_ratio,
                global_camera: if scene.use_global_camera { 1.0 } else { -1.0 },
            };

            unsafe {
                let data_ptr = base.device.logical.map_memory(scene.staging.memory, scene.staging.offset, scene.staging.size, vk::MemoryMapFlags::empty()).unwrap() as *mut SpecialMeshShaderParams;
                data_ptr.copy_from_nonoverlapping(&params as *const SpecialMeshShaderParams, scene.staging.size as usize);
                base.device.logical.unmap_memory(scene.staging.memory);

                let copy_region = [
                    vk::BufferCopy::default()
                        .src_offset(scene.staging.offset)
                        .dst_offset(scene.uniform.offset)
                        .size(scene.staging.size)
                ];

                base.device.logical.cmd_copy_buffer(*cb, scene.staging.buffer, scene.uniform.buffer, &copy_region);
            }


            if !scene.initialized {

                unsafe {

                    let size = scene.font_data.staging.size;
                    let data = scene.font_data.atlas.desc.glyph_info.each_ref().map(|g| { (g.w, g.h) });
                    let data_ptr = base.device.logical.map_memory(scene.font_data.staging.memory, 0, size, vk::MemoryMapFlags::empty()).unwrap() as *mut u32;
                    data_ptr.copy_from_nonoverlapping(data.as_ptr() as _, size as usize);
                    base.device.logical.unmap_memory(scene.font_data.staging.memory);

                    let size = scene.font_data.atlas_texture.staging.size;
                    let data_ptr = base.device.logical.map_memory(scene.font_data.atlas_texture.staging.memory, 0, size, vk::MemoryMapFlags::empty()).unwrap() as *mut u8;
                    data_ptr.copy_from_nonoverlapping(scene.font_data.atlas.atlas.data.as_ptr(), size as usize);
                    base.device.logical.unmap_memory(scene.font_data.atlas_texture.staging.memory);

					utils::image::transition_image_layout::<ImageLayout_Undefined, ImageLayout_ShaderReadOnlyOptimal>(&base.device, *cb, &scene.font_data.atlas_texture);
                    utils::image::transition_image_layout::<ImageLayout_ShaderReadOnlyOptimal, ImageLayout_TransferDstOptimal>(&base.device, *cb, &scene.font_data.atlas_texture);
                    utils::image::copy_buffer_to_image(&base.device, *cb, &scene.font_data.atlas_texture, &scene.font_data.atlas_texture.staging, scene.font_data.atlas.atlas.w, scene.font_data.atlas.atlas.h);
                    utils::image::transition_image_layout::<ImageLayout_TransferDstOptimal, ImageLayout_ShaderReadOnlyOptimal>(&base.device, *cb, &scene.font_data.atlas_texture);
                }

                scene.initialized = true;
            }


			let frame_time_ms = scene.previous_time.elapsed().as_millis();
			let frame_time = format!("{:>3} ", frame_time_ms);
			scene.frame_timer[0].set_text(&frame_time);
			scene.previous_time = Instant::now();

			DrawableText::update(&base.device, *cb, &mut scene.frame_timer);


        }

    }

    pub fn draw(scenes: &[SimpleScene], base: &mut VkBase, cb: &vk::CommandBuffer, current_image: usize, global_descriptor_set: vk::DescriptorSet) {

        let pso = &base.graphics_pipelines[ShaderSpecialMesh::ID];

        unsafe {
            base.device.logical.cmd_bind_pipeline(*cb, vk::PipelineBindPoint::GRAPHICS, pso.graphics);
        }

        unsafe {
            base.device.logical.cmd_bind_descriptor_sets(*cb, vk::PipelineBindPoint::GRAPHICS, pso.layout, 0, &[global_descriptor_set], &[]);
        }

        for scene in scenes {
			let set = &scene.descriptor_sets[current_image..current_image+1];

            unsafe {
                base.device.logical.cmd_bind_descriptor_sets(*cb, vk::PipelineBindPoint::GRAPHICS, pso.layout, 1, set, &[]);
            }

            DrawableMesh::draw(&base.device, cb, pso, &scene.static_meshes);
            DrawableMesh::draw(&base.device, cb, pso, &scene.dynamic_meshes);
			DrawableText::draw(&base.device, *cb, &base.graphics_pipelines[ShaderText::ID], current_image, &scene.frame_timer);
        }
    }

    pub fn release(scenes: &mut [Self], base: &VkBase) {
        for scene in scenes.iter_mut() {
            DrawableMesh::release(&base.device, &mut scene.dynamic_meshes);
            scene.dynamic_meshes.clear();
            DrawableMesh::release(&base.device, &mut scene.static_meshes);
            scene.static_meshes.clear();
            DrawableText::release(&base.device, &mut scene.frame_timer);

            unsafe {
			    base.device.logical.destroy_buffer(scene.font_data.atlas_texture.staging.buffer, None);
                base.device.logical.free_memory(scene.font_data.atlas_texture.staging.memory, None);
                base.device.logical.destroy_image(scene.font_data.atlas_texture.resource.image, None);
                base.device.logical.free_memory(scene.font_data.atlas_texture.resource.memory, None);
                base.device.logical.destroy_image_view(scene.font_data.atlas_texture.image_view, None);
                base.device.logical.destroy_sampler(scene.font_data.atlas_texture.sampler, None);
            }
        }
    }

}
