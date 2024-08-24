
use world::World;

use crate::world::*;

use super::{context::Context, shader::ShaderModule, vertex_input::{Vertex, CUBE_INDICES, CUBE_VERTICES}};


pub struct Renderer {
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    cube_pipeline: wgpu::RenderPipeline,
    cube_transform_buffer: wgpu::Buffer,
    cube_transform_bind_group: wgpu::BindGroup,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TransformUniform {
    transform: [[f32; 4]; 4]
} 

impl Renderer {
    pub fn new(world: &World, ctx: &Context, target_texture_format: &wgpu::TextureFormat) -> Self {

        let device = &ctx.device;

        let camera = &world.camera;

        let cube = &world.cube;

        let vertex_buffer = ctx.create_buffer(bytemuck::cast_slice(&CUBE_VERTICES), wgpu::BufferUsages::VERTEX);

        let index_buffer = ctx.create_buffer(bytemuck::cast_slice(&CUBE_INDICES), wgpu::BufferUsages::INDEX);


        let camera_uniform = TransformUniform { 
            transform: camera.view_projection_matrix().into() 
        };
        
        let camera_buffer = ctx.create_buffer(bytemuck::cast_slice(&[camera_uniform]), wgpu::BufferUsages::UNIFORM  | wgpu::BufferUsages::COPY_DST);

        let camera_bind_group_layout = device.logical_device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer { 
                        ty: wgpu::BufferBindingType::Uniform, 
                        has_dynamic_offset: false, 
                        min_binding_size: None
                    },
                    count: None
                }
            ],
            label: Some("camera_bind_group_layout")
        });

        let camera_bind_group = device.logical_device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding()
                }
            ],
            label: Some("camera_bind_group")
        });

        // Player

        let cube_uniform = TransformUniform { 
            transform: cube.model_matrix().into()
        }; 

        let cube_transform_buffer = ctx.create_buffer(bytemuck::cast_slice(&[cube_uniform]), wgpu::BufferUsages::UNIFORM  | wgpu::BufferUsages::COPY_DST);

        let cube_transform_bind_group_layout = device.logical_device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer { 
                        ty: wgpu::BufferBindingType::Uniform, 
                        has_dynamic_offset: false, 
                        min_binding_size: None
                    },
                    count: None
                }
            ],
            label: Some("cube_bind_group_layout")
        });

        let cube_transform_bind_group = device.logical_device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &cube_transform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: cube_transform_buffer.as_entire_binding()
                }
            ],
            label: Some("cube_bind_group")
        });
        

        let cube_shader = ShaderModule::new(&device.logical_device, "./src/assets/shaders/player.wgsl");

        let cube_pipeline_layout = device.logical_device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("cube_pipeline_layout"),
            bind_group_layouts: &[
                &camera_bind_group_layout,
                &cube_transform_bind_group_layout,
            ],
            push_constant_ranges: &[]
        }); 

        let cube_pipeline = device.logical_device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("cube_pipeline"),
            layout: Some(&cube_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &cube_shader.context_handle,
                entry_point: "vs_main",
                buffers: &[Vertex::buffer_layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &cube_shader.context_handle,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: *target_texture_format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float, // Example format
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less, // Typical depth function
                stencil: wgpu::StencilState::default(), // Default stencil settings
                bias: wgpu::DepthBiasState::default(), // Default depth bias
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false
            },
            multiview: None,
        });

        Self {
            camera_buffer,
            camera_bind_group,
            cube_pipeline,
            cube_transform_buffer,
            cube_transform_bind_group,
            vertex_buffer,
            index_buffer
        }
    }

    pub fn update(&self, world: &World, ctx: &Context) {
        let camera_uniform = TransformUniform {
            transform: world.camera.view_projection_matrix().into()
        };

        ctx.device.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&camera_uniform.transform));
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
