

use winit::{
    event::WindowEvent, window::Window,
    dpi::{PhysicalSize, Pixel}, event::*, event_loop::EventLoop, keyboard::{KeyCode, PhysicalKey}, window::WindowBuilder, 
};

use crate::graphics::graphics::Graphics;
use crate::scene::scene::Scene;


pub struct Engine<'a> 
{
    game_loop: winit::event_loop::EventLoop<()>,
    window: Window,
    input: Input<'a>,
    graphics: Graphics<'a>,
}

impl<'a> Engine<'a> {
    pub async fn new() -> Self 
    {
        let input = Input::new();

        let game_loop = EventLoop::new().expect("Failed to start event loop");

        let window = WindowBuilder::new()
            .build(&game_loop)
            .expect("Failed to start window");

        let graphics = Graphics::new(&window).await;

        env_logger::init();

        Self {
            game_loop,
            window,
            graphics,
            input
        }
    }

    pub async fn run(mut self) {
        let _ = self.game_loop.run(|event, control_flow| match event {
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

trait EngineLayer {
    fn resize(&self, physical_size: winit::dpi::PhysicalSize<u32>);

    fn handle_window_input(&self, event: Option<&WindowEvent>);
}

struct Input<'a> {
    listeners: Vec<&'a dyn EngineLayer>
}
impl<'a> EngineLayer for Scene<'a> {

    fn resize(&self, physical_size: winit::dpi::PhysicalSize<u32>) {
        
    }

    fn handle_window_input(&self, event: Option<&WindowEvent>) {
        match event {
            Some(event) => println!("Event is available from scene"),
            None => println!("Event is not available from scene")
        }
    }
}


impl<'a> Input<'a> {
    fn new() -> Self {
        Self {
            listeners: vec![]
        }
    }

    fn add_listener(&mut self, listener: &'a dyn EngineLayer) {
        self.listeners.push(listener);
    }

    fn call_listeners(&self, window_event: &WindowEvent) {
        self.listeners
            .iter()
            .for_each(move |listener| {
                listener.handle_window_input(Some(window_event))
            });
    }
}
