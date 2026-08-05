mod vk_base;
mod vk_bundles;
mod shader;
mod mesh;
mod shader_utils;
mod devices;
mod drawable;
mod primitives;
mod utils;
mod scene_extensions;
mod geometry;
mod rhi;
mod scene;

use std::time::{Duration, Instant};

use devices::record_player::RecordPlayer;
use drawable::{drawable_mesh::DrawableMesh, drawable_tex::DrawableTexture, drawable2d::Drawable2d};
use geometry::vec3::Vec3;
use mesh::{ Rect, cube};
use primitives::texture2d::{PixelFormat, Texture2d};
use scene::camera::{Camera, CameraParams, CameraAction};
use scene_extensions::simple_scene::SimpleScene;
use utils::{image::{begin_single_time_command, end_single_time_command}, keyboard::KeyboardState};
use vk_bundles::*;
use rhi::allocator::{Allocator, AllocatorSizeInfo, BufferType};

use ash::vk;

use vk_base::VkBase;

use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};

const CAMERA_LOCATION: Vec3 = Vec3::new(0.0, 0.0, 10.0);
const CAMERA_DIRECTION: Vec3 = Vec3::new(0.0, 0.0, -1.0);

struct App {
    base: VkBase,
    rect_bundles: Vec<Drawable2d>,
    mesh_bundles: Vec<DrawableMesh>,
    textures: Vec<DrawableTexture>,
    scenes: Vec<SimpleScene>,
    video_device: RecordPlayer,


    camera_staging: BufferBundle,
    camera_uniform: BufferBundle,
    camera: Camera,

    global_descriptor_set: Vec<vk::DescriptorSet>,
    close: bool,


    current_time: Instant,
    delta_time: f32,
    speed:      f32,

    keyboard_state: KeyboardState,

    allocator: Allocator,
    shader_poll_time: Instant
}

use shader::*;


const SHADER_POLL_INTERVAL: Duration = Duration::from_millis(500);

impl App {
    fn new(window: Window) -> Self {

        ShaderRegistry::describe_registed_shaders();

        let global_descriptor_set_binding = DescSetBinding {
            binding: 0,
            descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
            descriptor_count: 1,
            stage_flags: vk::ShaderStageFlags::VERTEX,
        };


        let video_device = RecordPlayer::from_buffer(include_bytes!("../assets/recordings/record1.rdbin")).unwrap();
        let base = VkBase::new(window, 3, "./assets/shaders", global_descriptor_set_binding);
        let mut allocator = Allocator::new(&base, AllocatorSizeInfo {
            staging: 10*1024,
            device_vertex: 10*1024,
            device_index: 10*1024,
            uniform_buffer: 10*1024,
        });


        let rect_bundles = vec![
            Drawable2d::new(&base.device, Rect::new(-0.9, -0.9, 0.5, 0.5, [1.0, 0.0, 0.0])),
            Drawable2d::new(&base.device, Rect::new(0.0, 0.0, 0.5, 0.5, [0.0, 0.0, 1.0])),
            Drawable2d::new(&base.device, Rect::new(-0.25, -0.25, 0.5, 0.5, [0.0, 1.0, 1.0]))
        ];

        let mesh_bundles = vec![
            DrawableMesh::new(&base.device, cube::make_cube(0.0, 0.0, 0.25, 0.5, [1.0, 0.2, 1.0]))
        ];

        let scenes = vec![
            SimpleScene::new(&base, &mut allocator)
        ];

        let data = unsafe { video_device.current_frame[0..video_device.size() / 2].align_to::<u8>().1.to_vec() };
        let texture = Texture2d::new(data, video_device.width(), video_device.height(), PixelFormat::Z16);

        //TODO: Cleanup descriptor pool

        let cb = begin_single_time_command(&base.device, base.spare_command.pool);

        let textures = vec![
            DrawableTexture::new(&base.device, cb, Rect::new(-1.0, -1.0, 2.0, 2.0, [1.0, 1.0, 1.0]), texture)
        ];

        end_single_time_command(&base.device, base.spare_command.pool, base.device.present_queue, cb);

        let camera_staging = allocator.alloc(BufferType::Staging, std::mem::size_of::<CameraParams>() as u64).unwrap();
        let camera_uniform = allocator.alloc(BufferType::Uniform, std::mem::size_of::<CameraParams>() as u64).unwrap();
        let camera = Self::make_camera();

        let global_descriptor_set = VkBase::create_descriptor_sets(&base.device, base.descriptor_pool, base.global_descriptor_set_layout, base.max_in_flight);

        // TODO: FIX THE DESC SET BINDING JANK
        for descriptor_set in global_descriptor_set.iter() {
            VkBase::update_descriptor_set_buffers(&base.device, *descriptor_set, &[&camera_uniform], 0);
        }

        let current_time = Instant::now();
        let delta_time   = 16.0e-3;
        let speed        = 0.05;

        let keyboard_state = KeyboardState::new();


        Self {
            video_device,
            base,
            rect_bundles,
            mesh_bundles,
            textures,
            scenes,
            camera_staging,
            camera_uniform,
            camera,
            allocator,

            global_descriptor_set,

            current_time,
            delta_time,
            speed,

            keyboard_state,

            shader_poll_time: Instant::now() + SHADER_POLL_INTERVAL,
            close: false,
        }
    }

    fn update(&mut self) {

        let ct = Instant::now();
        let delta_time_dur = ct - self.current_time;
        self.delta_time = delta_time_dur.as_secs_f32();

        self.handle_down_keys();

        for mesh_bundle in self.rect_bundles.iter_mut() {
            mesh_bundle.mesh.transform(0.001, [0.0, 0.0]);
        }

        self.base.cleanup_in_flight_buffers();

        if self.base.sync_objects.spare_fences.len() == 0 {
            return;
        }

        let cb = match self.base.spare_command.buffers.pop() {
            Some(cb) => cb,
            None => { return; }
        };

        let cb_begin_info =  vk::CommandBufferBeginInfo::default()
            .flags(vk::CommandBufferUsageFlags::SIMULTANEOUS_USE);

        unsafe {
            let _ = self.base.device.logical.reset_command_buffer(cb, vk::CommandBufferResetFlags::empty());
            self.base.device.logical.begin_command_buffer(cb, &cb_begin_info).unwrap();
        }

        Drawable2d::update(&self.base.device, &cb, &mut self.rect_bundles);
        DrawableMesh::update(&self.base.device, &cb, &mut self.mesh_bundles);

        let w = self.base.window.inner_size();
        SimpleScene::update(&mut self.scenes, &self.base, &cb, w.width as f32 / w.height as f32);


        if let Some(new_frame) = self.video_device.poll() {
            self.textures[0].texture_data.copy_to_data(&new_frame);
        }

        DrawableTexture::update(&self.base.device, cb, &mut self.textures);

        unsafe {
            let data_ptr = self.base.device.logical.map_memory(self.camera_staging.memory, self.camera_staging.offset, self.camera_staging.size, vk::MemoryMapFlags::empty()).unwrap() as *mut CameraParams;
            data_ptr.copy_from_nonoverlapping(&self.camera.params  as *const CameraParams, self.camera_staging.size as usize);
            self.base.device.logical.unmap_memory(self.camera_staging.memory);

            let copy_region = [
                vk::BufferCopy::default()
                    .src_offset(self.camera_staging.offset)
                    .dst_offset(self.camera_uniform.offset)
                    .size(self.camera_staging.size)
            ];

            self.base.device.logical.cmd_copy_buffer(cb, self.camera_staging.buffer, self.camera_uniform.buffer, &copy_region);
        }


        unsafe { self.base.device.logical.end_command_buffer(cb).unwrap(); }

        let cbs = [cb];
        let submit_info = vk::SubmitInfo::default()
            .command_buffers(&cbs);

        let fences = [self.base.sync_objects.spare_fences.pop().unwrap()];
        unsafe {
            self.base.device.logical.reset_fences(&fences).expect("Failed to reset fences.");
            self.base.device.logical.queue_submit(self.base.device.present_queue, &[submit_info], fences[0]).expect("Failure submitting to the queue.");
        }

        self.base.in_flight_buffers.push((cb, fences[0]));

        if self.shader_poll_time < ct {
            self.base.check_and_recompile_shaders();
            self.shader_poll_time = ct + SHADER_POLL_INTERVAL;
        }
    }

    fn render(&mut self)
    {

        let (cb, image_index) = match self.base.begin_renderpass_command_buffer() {
            Some((cb, image_index)) => (cb, image_index),
            None => { return; }
        };

        // DrawableTexture::draw(&self.base.device, cb, &self.base.graphics_pipelines[ShaderTexture::ID], self.base.current_frame, &self.textures);
        // Drawable2d::draw(&self.base.device, &cb, &self.base.graphics_pipelines[ShaderRect::ID], &self.rect_bundles);
        // DrawableMesh::draw(&self.base.device, &cb, &self.base.graphics_pipelines[ShaderMesh::ID], &self.mesh_bundles);
        let current_image = self.base.current_frame;
        SimpleScene::draw(&self.scenes, &mut self.base, &cb, current_image, self.global_descriptor_set[current_image]);
        self.base.render(&cb, image_index);
    }

    fn handle_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                self.close = true;
            }

            WindowEvent::RedrawRequested => {
                self.update();
                self.render();
            }

            WindowEvent::Resized(_) => {
                self.base.window.request_redraw();
            }

            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                self.handle_key(event);
            }
            _ => {
            }
        }
    }

    fn handle_down_keys(&mut self) {
        if self.keyboard_state[KeyCode::KeyA] {
            self.camera.update(CameraAction::Left, self.delta_time * self.speed);
        }

        if self.keyboard_state[KeyCode::KeyD] {
            self.camera.update(CameraAction::Right, self.delta_time * self.speed);
        }

        if self.keyboard_state[KeyCode::KeyW] {
            self.camera.update(CameraAction::Forward, self.delta_time * self.speed);
        }

        if self.keyboard_state[KeyCode::KeyS] {
            self.camera.update(CameraAction::Backward, self.delta_time * self.speed);
        }

        if self.keyboard_state[KeyCode::KeyE] {
            self.camera.update(CameraAction::Up, self.delta_time * self.speed);
        }

        if self.keyboard_state[KeyCode::KeyQ] {
            self.camera.update(CameraAction::Down, self.delta_time * self.speed);
        }
    }

    fn handle_key(&mut self, event: KeyEvent) {
        match event.physical_key {
            PhysicalKey::Code(a) => {
                match a {
                    KeyCode::Escape => {
                        self.close = true;
                    }

                    KeyCode::KeyA | KeyCode::KeyD | KeyCode::KeyW | KeyCode::KeyS | KeyCode::KeyE | KeyCode::KeyQ => {
                        self.keyboard_state[a] = event.state == ElementState::Pressed;
                    }


                    KeyCode::KeyT => {
                        self.reset_camera();
                    }

                    k => {
                        SimpleScene::handle_key(&mut self.scenes, k, event.state, event.repeat);
                    }
                }
            }
            _ => {}
        }
    }

    fn reset_camera(&mut self) {
        self.camera = Self::make_camera();
    }

    fn make_camera() -> Camera{
        return Camera::new(CAMERA_LOCATION, CAMERA_DIRECTION);
    }
}

impl Drop for App {
    fn drop(&mut self) {
        unsafe {

            let _ = self.base.device.logical.device_wait_idle();


            Drawable2d::release(&self.base.device, &mut self.rect_bundles);
            self.rect_bundles.clear();

            DrawableMesh::release(&self.base.device, &mut self.mesh_bundles);
            self.mesh_bundles.clear();

            DrawableTexture::release(&self.base.device, &mut self.textures);
            self.textures.clear();

            SimpleScene::release(&mut self.scenes, &self.base);
            self.scenes.clear();

            self.allocator.release(&self.base.device);
        }
    }
}


fn main() {
    // SimpleLogger::new().init().unwrap();

    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new()
        .with_title("Vulkan Video")
        .build(&event_loop)
        .unwrap();

    let mut app = App::new(window);

    let mut closing = false;

    let _ = event_loop.run(move |event, elwt| {
        elwt.set_control_flow(winit::event_loop::ControlFlow::Poll);
        match event {
            Event::WindowEvent { event, window_id } if window_id == app.base.window.id() => {
                app.handle_event(event);
            }

            Event::AboutToWait => {
                app.base.window.request_redraw();
            }

            _ => (),
        }

        if app.close && !closing {
            closing = true;
            elwt.exit();
        }
    });

    println!("Process Completed.");
}
