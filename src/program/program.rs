
use winit::{
    event::{WindowEvent, *}, event_loop::{EventLoop, EventLoopWindowTarget}, keyboard::{KeyCode, PhysicalKey}, window::Window 
};

use crate::{graphics::graphics::Graphics, world::{camera::{CameraBuilder, CameraType, PerspectiveCamera}, world::World}};


pub struct Program<'a> {
    world: World,
    camera: PerspectiveCamera,
    graphics: Graphics,
    window: &'a Window, 
    last_mouse_pos: Option<(f32, f32)>,
}

impl<'a> Program<'a> {
    pub async fn new(window: &'a Window) -> Self {
        
        let mut graphics = Graphics::new(&window).await;

        let camera = CameraBuilder::new(&mut graphics.ctx)
            .with_position((0.0, 0.0, 5.0).into())
            .with_target((0.0, 0.0, 0.0).into())
            .with_type(CameraType::Perspective)
            .with_radius(10.0)
            .build();

        let world = World::new(&mut graphics.ctx);


        let last_mouse_pos = None;
    
        env_logger::init();

        Self {
            world,
            camera,
            graphics,
            window,
            last_mouse_pos
        }
    }

    pub async fn run(&mut self, world_loop: EventLoop<()>) {
        let _ = world_loop.run(move |event, control_flow| {
            self.handle_window_input(&event, &control_flow);

            self.camera.update_uniforms(&self.graphics.ctx);

            self.graphics.render(&self.world, &self.camera, &self.window);
        });
    }

    fn handle_window_input(&mut self, event: &Event<()>, control_flow: &EventLoopWindowTarget<()>) {
        match event {
            Event::WindowEvent { window_id, event } => {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                state: ElementState::Pressed,
                                physical_key: PhysicalKey::Code(KeyCode::Escape),
                                ..
                            },
                        ..
                    } => {
                        control_flow.exit()
                    },
                    WindowEvent::Resized(physical_size) => {        
                        self.graphics.resize(&physical_size);
                    },
                    WindowEvent::KeyboardInput {
                        event: 
                            KeyEvent {
                                state: ElementState::Pressed,
                                physical_key: PhysicalKey::Code(KeyCode::KeyW),
                                ..
                            },
                        ..
                    } => {},
                    WindowEvent::MouseInput { button, state, .. } => {
                        if *button == MouseButton::Left {
                            if *state == ElementState::Pressed {
                                self.last_mouse_pos = Some((0.0, 0.0));
                            } else {
                                self.last_mouse_pos = None;
                            }
                        }
                    },
                    _ => {},
                }
            },
            Event::DeviceEvent { device_id, event } => {
                match event {
                    DeviceEvent::MouseMotion { delta } => {
                        if let Some((_, _)) = self.last_mouse_pos {
                            self.camera.process_mouse_movement(delta.0 as f32, -delta.1 as f32);
                        }
                    },
                    DeviceEvent::MouseWheel { delta } => {
                        self.camera.process_mouse_scroll(delta);
                    }
                    _ => {}
                }
            },
            _ => {}
        }
    }
}



