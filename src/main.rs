use pollster::block_on;
use program::program::Program;
use winit::{
   event::WindowEvent, window::Window,
   dpi::{PhysicalSize, Pixel}, event::*, event_loop::EventLoop, keyboard::{KeyCode, PhysicalKey}, window::WindowBuilder, 
};

mod program;
mod graphics;
mod game;



fn main() {
   let game_loop = EventLoop::new().expect("Failed to start event loop");
    
   let window = WindowBuilder::new()
       .build(&game_loop)
       .expect("Failed to start window");

   let mut program = block_on(Program::new(&window));

   block_on(program.run(game_loop));
}
