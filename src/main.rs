
pub mod graphics;
pub mod engine;

use winit::{
    dpi::{PhysicalSize, Pixel}, event::*, event_loop::EventLoop, keyboard::{KeyCode, PhysicalKey}, window::WindowBuilder
};

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new().expect("Failed to start event loop");
    let window = WindowBuilder::new().build(&event_loop).expect("Failed to start window");

    let mut graphics = graphics::Graphics::new(&window).await;

    let _ = event_loop.run(|event, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
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
                physical_size.width /= window.scale_factor() as u32;
                physical_size.height /= window.scale_factor() as u32;

                graphics.resize(&physical_size);
            }
            _ => {}
        },
        _ => {
            let _ = graphics.render();
        }
    }).expect("Failed to run event loop!");
}



fn main() {
    pollster::block_on(run());
}
