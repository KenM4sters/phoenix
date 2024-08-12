

pub struct Renderer {
    size: winit::dpi::PhysicalSize<u32>
}

impl Renderer {
    pub fn new(size: winit::dpi::PhysicalSize<u32>) -> Self {
        Self {
            size
        }
    }

    pub fn draw_sprite(&mut self) {

    }
}