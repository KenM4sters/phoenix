use pollster::block_on;
use program::program::Program;
use event_loop::EventLoop;
use window::WindowBuilder;
use winit::*;

mod program;
mod graphics;
mod world;



fn main() {
   let world_loop = EventLoop::new().expect("Failed to start event loop");
    
   let window = WindowBuilder::new()
       .build(&world_loop)
       .expect("Failed to start window");

   let mut program = block_on(Program::new(&window));

   block_on(program.run(world_loop));
}
