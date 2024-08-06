use engine::engine::Engine;
use graphics::graphics::Graphics;
use scene::scene::Scene;

mod engine;
mod graphics;
mod scene;

fn main() {
    pollster::block_on(Engine::new());
}
