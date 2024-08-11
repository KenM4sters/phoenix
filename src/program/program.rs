
use winit::{
    event::WindowEvent, window::Window,
    dpi::{PhysicalSize, Pixel}, event::*, event_loop::EventLoop, keyboard::{KeyCode, PhysicalKey}, window::WindowBuilder, 
};

use crate::{game::game::Input, graphics::graphics::Graphics};

pub struct Program<'a> {
    input: Input<'a>,
    graphics: Graphics<'a>,
    window: Window,   
    game_loop: EventLoop<()>
}

impl<'a> Program<'a> {
    pub async fn new() -> Self {
        let input = Input::new();

        let game_loop = EventLoop::new().expect("Failed to start event loop");
    
        let window = WindowBuilder::new()
            .build(&game_loop)
            .expect("Failed to start window");
    
        let graphics = Graphics::new(&window).await;
    
        env_logger::init();

        Self {
            input, 
            graphics,
            window,
            game_loop
        }
    }

    pub async fn run(mut self) {
        let _ = self.game_loop.run(move |event, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == self.window.id() => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            state: ElementState::Pressed,
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            ..
                        },
                    ..
                } => control_flow.exit(),
                WindowEvent::Resized(mut physical_size) => {
                    physical_size.width /= self.window.scale_factor() as u32;
                    physical_size.height /= self.window.scale_factor() as u32;
    
                    self.graphics.resize(&physical_size);
                }
                _ => {}
            },
            _ => {
                let _ = self.graphics.render();
            }
        }).expect("Failed to run event loop!");
    }
}