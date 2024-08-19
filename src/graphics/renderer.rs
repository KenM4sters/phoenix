use crate::game::camera::OrthographicCamera;


pub struct Renderer {
    size: winit::dpi::PhysicalSize<u32>
}

impl Renderer {
    pub fn new(size: winit::dpi::PhysicalSize<u32>) -> Self {
        Self {
            size
        }
    }

    pub fn draw(&mut self, camera: &OrthographicCamera) {
        
    }
}