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

use scene_extensions::{demo_scene::DemoScene, simple_scene::SimpleScene, text_scene::TextScene, global_descriptor::GLOBAL_DESCRIPTOR_SET_BINDING};
use utils::keyboard::KeyboardState;
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

struct App {
    base: VkBase,

    target_scene: TargetScene,
    demo_scene: DemoScene,
    simple_scene: SimpleScene,
    text_scene: TextScene,

    close: bool,

    current_time: Instant,
    delta_time: f32,

    keyboard_state: KeyboardState,

    allocator: Allocator,
    shader_poll_time: Instant
}


const SHADER_POLL_INTERVAL: Duration = Duration::from_millis(500);

impl App {
    fn new(window: Window) -> Self {

        ShaderRegistry::describe_registed_shaders();

        let base = VkBase::new(window, 3, "./assets/shaders", GLOBAL_DESCRIPTOR_SET_BINDING.clone());

        let mut allocator = Allocator::new(&base, AllocatorSizeInfo {
            staging: 10*1024,
            device_vertex: 10*1024,
            device_index: 10*1024,
            uniform_buffer: 10*1024,
        });

        let current_time = Instant::now();
        let delta_time   = 16.0e-3;

        let keyboard_state = KeyboardState::new();

        let demo_scene = DemoScene::new(&base);
        let simple_scene = SimpleScene::new(&base, &mut allocator);
        let text_scene = TextScene::new(&base, &mut allocator);

        Self {
            base,

            target_scene: STARTING_SCENE,
            demo_scene,
            simple_scene,

            allocator,

            current_time,
            delta_time,

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

        match self.target_scene {
            TargetScene::Demo => {
                self.demo_scene.update(&self.base, cb, self.delta_time);
            }

            TargetScene::Simple => {
                let w = self.base.window.inner_size();
                self.simple_scene.update(&self.base, cb, w.width as f32 / w.height as f32, &self.keyboard_state, self.delta_time);
            }

            TargetScene::Text => {
                let w = self.base.window.inner_size();
                self.text_scene.update(&self.base, cb, w.width as f32 / w.height as f32, w.width as f32);
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
                self.simple_scene.draw(&mut self.base, cb, current_frame);
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

                self.keyboard_state[a] = event.state == ElementState::Pressed;

                match a {
                    KeyCode::Escape => {
                        self.close = true;
                    }

                    KeyCode::F1 => {
                        self.target_scene = TargetScene::Empty;
                        self.simple_scene.activated = false;
                    }

                    KeyCode::F2 => {
                        self.target_scene = TargetScene::Demo;
                        self.simple_scene.activated = false;
                    }

                    KeyCode::F3 => {
                        self.target_scene = TargetScene::Simple;
                    }

                    KeyCode::F4 => {
                        self.target_scene = TargetScene::Text;
                        self.simple_scene.activated = false;
                    }

                    _ => { }
                }
            }


            _ => { }

        }
        if event.state == ElementState::Pressed {
            if let Some(text) = event.text {
                match self.target_scene {
                    TargetScene::Simple => {
                        //
                    },

                    TargetScene::Text => {
                        self.text_scene.handle_text_input(&text);
                    }
                    _ => {
                        // noting to do
                    },
                }
            }
        }
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
