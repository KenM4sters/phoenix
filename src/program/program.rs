
use winit::{
    event::{WindowEvent, *}, event_loop::{EventLoop, EventLoopWindowTarget}, keyboard::{KeyCode, PhysicalKey}, window::Window 
};

use crate::{world::world::World, graphics::graphics::Graphics};


pub struct Program<'a> {
    world: World,
    graphics: Graphics,
    window: &'a Window,   
}

impl<'a> Program<'a> {
    pub async fn new(window: &'a Window) -> Self {
        
        let world = World::new();
    
        let graphics = Graphics::new(&world, &window).await;
    
        env_logger::init();

        Self {
            world,
            graphics,
            window
        }
    }

    pub async fn run(&mut self, world_loop: EventLoop<()>) {
        let _ = world_loop.run(move |event, control_flow| {
            self.world.handle_window_input(&event);
            match event {
                Event::WindowEvent { 
                    window_id, event 
                } if window_id == self.window.id() => {
                    self.handle_window_input(&event, &control_flow);
                }
                _ => {}
            }

            self.world.update();

            self.graphics.update(&self.world);

            self.graphics.render(&self.window);
        });
    }

    fn handle_window_input(&mut self, event: &WindowEvent, control_flow: &EventLoopWindowTarget<()>) {
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
            } => {}
            _ => {}
        }
    }
}