use crate::world::*;

use super::context::Context;


pub struct Renderer {
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TransformUniform {
    pub transform: [[f32; 4]; 4]
} 

impl Renderer {
    pub fn new(ctx: &Context) -> Self {
        Self {
        }
    }

    pub fn render<'a>(&'a self, encoder: &mut wgpu::CommandEncoder, color_view: &wgpu::TextureView, depth_view: &wgpu::TextureView) {
    }
}
