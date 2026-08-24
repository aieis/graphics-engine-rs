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
mod components;

use std::time::{Duration, Instant};

use crate::geometry::vec3::Vec3;
use scene::camera::{Camera, CameraParams, CameraAction};
use scene_extensions::{demo_scene::DemoScene, simple_scene::SimpleScene, text_scene::TextScene};
use utils::{keyboard::KeyboardState};
use vk_bundles::*;
use rhi::allocator::{Allocator, AllocatorSizeInfo, BufferType};
use shader::*;

use ash::vk;

use vk_base::VkBase;

use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};


enum TargetScene {
    Demo,
    Simple,
    Text,
    Empty
}

const STARTING_SCENE: TargetScene = TargetScene::Text;

const CAMERA_LOCATION: Vec3 = Vec3::new(0.0, 0.0, 10.0);
const CAMERA_DIRECTION: Vec3 = Vec3::new(0.0, 0.0, -1.0);

struct App {
    base: VkBase,

    target_scene: TargetScene,
    demo_scene: DemoScene,
    simple_scene: SimpleScene,
    text_scene: TextScene,

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


        let base = VkBase::new(window, 3, "./assets/shaders", global_descriptor_set_binding);

        let mut allocator = Allocator::new(&base, AllocatorSizeInfo {
            staging: 10*1024,
            device_vertex: 10*1024,
            device_index: 10*1024,
            uniform_buffer: 10*1024,
        });

        let current_time = Instant::now();
        let delta_time   = 16.0e-3;
        let speed        = 1.0;

        let keyboard_state = KeyboardState::new();


        let camera_staging = allocator.alloc(BufferType::Staging, std::mem::size_of::<CameraParams>() as u64).unwrap();
        let camera_uniform = allocator.alloc(BufferType::Uniform, std::mem::size_of::<CameraParams>() as u64).unwrap();
        let camera = Self::make_camera();

        let demo_scene = DemoScene::new(&base);
        let simple_scene = SimpleScene::new(&base, &mut allocator);
        let text_scene = TextScene::new(&base, &mut allocator);

        let global_descriptor_set = VkBase::create_descriptor_sets(&base.device, base.descriptor_pool, base.global_descriptor_set_layout, base.max_in_flight);

        // TODO: FIX THE DESC SET BINDING JANK
        for descriptor_set in global_descriptor_set.iter() {
            VkBase::update_descriptor_set_buffers(&base.device, *descriptor_set, &[&camera_uniform], 0);
        }

        Self {
            base,

            target_scene: STARTING_SCENE,
            demo_scene,
            simple_scene,

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
            text_scene,
        }
    }

    fn update(&mut self) {

        let ct = Instant::now();
        let delta_time_dur = ct - self.current_time;
        self.delta_time = delta_time_dur.as_secs_f32();
		self.current_time = ct;

        self.handle_down_keys();

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

        unsafe {
            let data_ptr = self.base.device.logical.map_memory(self.camera_staging.memory, 0, self.camera_staging.size, vk::MemoryMapFlags::empty()).unwrap() as *mut u8;
            let data_ptr = data_ptr.offset(self.camera_staging.offset as isize) as *mut CameraParams;
            data_ptr.copy_from_nonoverlapping(&self.camera.params  as *const CameraParams, 1);
            self.base.device.logical.unmap_memory(self.camera_staging.memory);

            let copy_region = [
                vk::BufferCopy::default()
                    .src_offset(self.camera_staging.offset)
                    .dst_offset(self.camera_uniform.offset)
                    .size(self.camera_staging.size)
            ];

            self.base.device.logical.cmd_copy_buffer(cb, self.camera_staging.buffer, self.camera_uniform.buffer, &copy_region);
        }


        match self.target_scene {
            TargetScene::Demo => {
                self.demo_scene.update(&self.base, cb, self.delta_time);
            }

            TargetScene::Simple => {
                let w = self.base.window.inner_size();
                self.simple_scene.update(&self.base, cb, w.width as f32 / w.height as f32);
            }

            TargetScene::Text => {
                let w = self.base.window.inner_size();
                self.text_scene.update(&self.base, cb, w.width as f32 / w.height as f32);
            }

            TargetScene::Empty => {
                // Do nothing
            },
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

        match self.target_scene {
            TargetScene::Demo => {
                self.demo_scene.draw(&self.base, cb);
            }

            TargetScene::Simple => {
                let current_frame = self.base.current_frame;
                self.simple_scene.draw(&mut self.base, cb, current_frame, self.global_descriptor_set[current_frame]);
            }

            TargetScene::Text => {
                let current_frame = self.base.current_frame;
                self.text_scene.draw(&mut self.base, cb, current_frame);
            }

            TargetScene::Empty => {
                // Do nothing
            },
        }

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
                match self.target_scene {
                    TargetScene::Simple => {
                        self.simple_scene.handle_key(a, event.state, event.repeat);
                    },

                    TargetScene::Text => {
                        self.text_scene.handle_key(a, event.state, event.repeat);
                    }

                    _ => {
                        // noting to do
                    },
                };

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

                    KeyCode::F1 => {
                        self.target_scene = TargetScene::Empty;
                    }

                    KeyCode::F2 => {
                        self.target_scene = TargetScene::Demo;
                    }

                    KeyCode::F3 => {
                        self.target_scene = TargetScene::Simple;
                    }

                    KeyCode::F4 => {
                        self.target_scene = TargetScene::Text;
                    }


                    _ => { }
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
            self.demo_scene.release(&self.base);
            self.simple_scene.release(&self.base);
            self.text_scene.release(&self.base);
            self.allocator.release(&self.base.device);
        }
    }
}


fn main() {
    // SimpleLogger::new().init().unwrap();

    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new()
        .with_title("The Rust Graphics Engine")
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
