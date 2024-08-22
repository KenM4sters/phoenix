use wgpu::{Device, TextureFormat}; // Import wgpu types directly
use egui::Context;
use egui_wgpu::renderer::Renderer;

pub struct Gui {
    ctx: Context,
    renderer: Renderer,
}

impl Gui {
    pub fn new(device: &Device, target_format: TextureFormat) -> Self {
        let ctx = Context::default();
        let renderer = Renderer::new(device, target_format, None, 1);

        Self {
            ctx,
            renderer,
        }
    }
}
