

use winit::{event::WindowEvent, window::Window};

pub struct State<'a> 
{
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: &'a Window
}

impl<'a> State<'a> {
    async fn new(window: &'a Window) -> State<'a> {

    }

    fn window(&self) -> &'a Window {
        self.window
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {

    }

    fn input(&mut self, event: &WindowEvent) -> bool {

    }

    fn update(&mut self) {

    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {

    }

    
}



