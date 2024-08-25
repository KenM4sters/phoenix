use world::World;

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
    pub fn new(world: &World, ctx: &Context, target_texture_format: &wgpu::TextureFormat) -> Self {

        let device = &ctx.device;

        let cube = &world.cube;

        Self {
        }
    }

    pub fn render<'a>(&'a self, encoder: &mut wgpu::CommandEncoder, color_view: &wgpu::TextureView, depth_view: &wgpu::TextureView) {
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("world_render_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &color_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    }
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            
            // First Order Bind Groups
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
    
            render_pass.set_pipeline(&self.cube_pipeline);
    
            render_pass.set_bind_group(1, &self.cube_transform_bind_group, &[]);
    
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
    
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    
            render_pass.draw_indexed(0..CUBE_INDICES.len() as u32, 0, 0..1);
        }   
        
    }
}
