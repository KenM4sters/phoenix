
use pollster::block_on;
use program::program::Program;


mod program;
mod graphics;
mod game;
mod scene;


fn main() {
   block_on(Program::new());
}
