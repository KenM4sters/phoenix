
use winit::{
    event::{WindowEvent, *}, event_loop::{EventLoop, EventLoopWindowTarget}, keyboard::{KeyCode, PhysicalKey}, window::Window 
};

use crate::graphics::graphics::Graphics;

pub struct Program<'a> {
    graphics: Graphics<'a>,
    window: &'a Window,   
}

impl<'a> Program<'a> {
    pub async fn new(window: &'a Window) -> Self {
    
        let graphics = Graphics::new(&window).await;
    
        env_logger::init();

        Self {
            graphics,
            window
        }
    }

    pub async fn run(&mut self, game_loop: EventLoop<()>) {
        let _ = game_loop.run(move |event, control_flow| {
            match event {
                Event::WindowEvent { 
                    window_id, event 
                } if window_id == self.window.id() => {
                    self.handle_window_input(&event, &control_flow)
                }
                _ => {}
            }
            let _ = self.graphics.render();
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
            WindowEvent::Resized(mut physical_size) => {
                physical_size.width /= self.window.scale_factor() as u32;
                physical_size.height /= self.window.scale_factor() as u32;
        
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
            } => {

            }
            _ => {}
        }
    }
}