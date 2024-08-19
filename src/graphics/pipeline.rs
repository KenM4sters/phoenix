
use super::{shader::ShaderModule, vertex_input::Vertex};

pub struct PipelineManager {
    pub player_pipeline: wgpu::RenderPipeline,
    pub enemy_one_pipeline: wgpu::RenderPipeline
}

impl PipelineManager {
    pub fn new(logical_device: &wgpu::Device, target_texture_format: &wgpu::TextureFormat) -> Self {
        
        // Player
        
        let player_shader = ShaderModule::new(logical_device, "./src/assets/shaders/player.wgsl");

        let player_pipeline_layout = logical_device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("player_pipeline_layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[]
        }); 

        let player_pipeline = logical_device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("player_pipeline"),
            layout: Some(&player_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &player_shader.context_handle,
                entry_point: "vs_main",
                buffers: &[Vertex::buffer_layout()],
                compilation_options: wgpu::PipelineCompilationOptions::default()
            },
            fragment: Some(wgpu::FragmentState {
                module: &player_shader.context_handle,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: *target_texture_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default()
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
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false
            },
            multiview: None,
            cache: None
        });


        // Enemy One

        let enemy_one_shader = ShaderModule::new(logical_device, "./src/assets/shaders/player.wgsl");

        let enemy_one_pipeline_layout = logical_device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("enemy_one_pipeline_layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[]
        }); 

        let enemy_one_pipeline = logical_device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("enemy_one_pipeline"),
            layout: Some(&enemy_one_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &enemy_one_shader.context_handle,
                entry_point: "vs_main",
                buffers: &[Vertex::buffer_layout()],
                compilation_options: wgpu::PipelineCompilationOptions::default()
            },
            fragment: Some(wgpu::FragmentState {
                module: &enemy_one_shader.context_handle,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: *target_texture_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default()
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
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false
            },
            multiview: None,
            cache: None
        });

        Self {
            player_pipeline,
            enemy_one_pipeline
        }
    }
}