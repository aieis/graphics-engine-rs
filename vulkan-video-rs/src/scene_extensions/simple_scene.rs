use std::time::Instant;

use ash::vk;
use winit::event::ElementState;
use winit::keyboard::KeyCode;

use crate::geometry::vec3::Vec3;
use crate::mesh::prism;
use crate::rhi::allocator::{Allocator, BufferType};
use crate::vk_bundles::BufferBundle;
use crate::{drawable::drawable_mesh::DrawableMesh, vk_base::VkBase};
use crate::shader::ShaderSpecialMesh;

#[repr(C)]
struct SpecialMeshShaderParams {
    time: f32,
    aspect: f32,
    global_camera: f32
}

pub struct SimpleScene
{
    pub time            : Instant,

    pub static_meshes  : Vec<DrawableMesh>,
    pub dynamic_meshes : Vec<DrawableMesh>,

    staging: BufferBundle,
    uniform: BufferBundle,

    use_global_camera: bool,
    going_down: bool,
    translation_amount: f32,
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

        if let Some(ubo) = base.graphics_pipelines[ShaderSpecialMesh::ID].ubo.as_ref() {
            for descriptor_set in ubo.sets.iter() {
                VkBase::update_descriptor_set_buffers(&base.device, *descriptor_set, &[&uniform], 0);
            }
        }

        let time = Instant::now();


        Self {
            time,
            static_meshes,
            dynamic_meshes,
            staging,
            uniform,
            use_global_camera: false,
            going_down: false,
            translation_amount: 0.0,
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

        let empty_desc_sets = [];

        let sets = match &base.graphics_pipelines[ShaderSpecialMesh::ID].ubo {
            Some(ubo) => &ubo.sets[current_image..current_image+1],
            None => &empty_desc_sets,
        };

        for scene in scenes {
            //TODO: Should update the descriptor sets here, but ehh

            unsafe {
                base.device.logical.cmd_bind_descriptor_sets(*cb, vk::PipelineBindPoint::GRAPHICS, pso.layout, 1, sets, &[]);
            }

            DrawableMesh::draw(&base.device, cb, pso, &scene.static_meshes);
            DrawableMesh::draw(&base.device, cb, pso, &scene.dynamic_meshes);
        }
    }

    pub fn release(scenes: &mut [Self], base: &VkBase) {
        for scene in scenes.iter_mut() {
            DrawableMesh::release(&base.device, &mut scene.dynamic_meshes);
            scene.dynamic_meshes.clear();
            DrawableMesh::release(&base.device, &mut scene.static_meshes);
            scene.static_meshes.clear();
        }
    }

}
