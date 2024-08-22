
use camera::{CameraBuilder, CameraType};
use cgmath::SquareMatrix;

use crate::world::*;

use super::{device::Device, shader::ShaderModule, vertex_input::{Vertex, INDICES, VERTICES}};


pub struct Renderer {
    player_pipeline: wgpu::RenderPipeline,
    camera_bind_group: wgpu::BindGroup,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_projection: [[f32; 4]; 4]
} 

impl Renderer {
    pub fn new(device: &Device, target_texture_format: &wgpu::TextureFormat) -> Self {
        
        let vertex_buffer = device.create_buffer(bytemuck::cast_slice(&VERTICES), wgpu::BufferUsages::VERTEX);

        let index_buffer = device.create_buffer(bytemuck::cast_slice(&INDICES), wgpu::BufferUsages::INDEX);

        let camera = CameraBuilder::new()
            .with_position((0.0, 0.0, 5.0).into())
            .with_target((0.0, 0.0, 0.0).into())
            .with_type(CameraType::Perspective)
            .build();

        let camera_uniform = CameraUniform { 
            view_projection: camera.view_projection_matrix().into() 
        };
        
        let camera_buffer = device.create_buffer(bytemuck::cast_slice(&[camera_uniform]), wgpu::BufferUsages::UNIFORM  | wgpu::BufferUsages::COPY_DST);

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

        let player_uniform = PlayerUniform { 
            model: cgmath::Matrix4::identity().into() 
        };
        


        let player_shader = ShaderModule::new(&device.logical_device, "./src/assets/shaders/player.wgsl");

        let player_pipeline_layout = device.logical_device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("player_pipeline_layout"),
            bind_group_layouts: &[
                &camera_bind_group_layout
            ],
            push_constant_ranges: &[]
        }); 

        let player_pipeline = device.logical_device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("player_pipeline"),
            layout: Some(&player_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &player_shader.context_handle,
                entry_point: "vs_main",
                buffers: &[Vertex::buffer_layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &player_shader.context_handle,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: *target_texture_format,
                    blend: Some(wgpu::BlendState::REPLACE),
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
            player_uniform,
            player_pipeline,
            camera_bind_group,
            vertex_buffer,
            index_buffer
        }
    }

    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        // First Order Bind Groups
        render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

        render_pass.set_pipeline(&self.player_pipeline);

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        render_pass.draw_indexed(0..INDICES.len() as u32, 0, 0..1);
    }
}
