use egui_wgpu::renderer::ScreenDescriptor;
use winit::{dpi::PhysicalSize, window::Window};


use crate::world::{camera::PerspectiveCamera, world::World};

use super::{context::Context, gui::{example_gui, Gui}, renderer::Renderer, shader::ShaderModule, vertex_input::{Vertex, SQUARE_INDICES, SQUARE_VERTICES}};


pub struct Graphics {
    pub ctx: Context,
    size: winit::dpi::PhysicalSize<u32>,
    renderer: Renderer,
    gui: Gui,
    world_color_texture: wgpu::Texture,
    world_depth_texture: wgpu::Texture,
    world_color_texture_view: wgpu::TextureView,
    world_depth_texture_view: wgpu::TextureView,
    square_index_buffer: wgpu::Buffer,
    square_vertex_buffer: wgpu::Buffer,
    world_texture_bind_group: wgpu::BindGroup,
    screen_quad_pipeline: wgpu::RenderPipeline,
}

impl Graphics {
    pub async fn new(world: &World, window: &Window) -> Self 
    {
        let size = window.inner_size();

        let ctx = Context::new(size.clone(), &window).await; 

        let device = &ctx.device;
        let surface_format = &ctx.surface_config.format;

        let gui = Gui::new(&device.logical_device, surface_format.clone(), None, &window);

        let world_color_texture = device.logical_device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: 1600,
                height: 1200,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba32Float, // Match this to the format in your pipeline
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let renderer = Renderer::new(&world, &ctx, &world_color_texture.format());

        let world_color_texture_view = world_color_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let world_depth_texture = device.logical_device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: 1600,
                height: 1200,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float, // Match this to the format in your pipeline
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
    
        let world_depth_texture_view = world_depth_texture.create_view(&wgpu::TextureViewDescriptor::default());


        let square_vertex_buffer = ctx.create_buffer(bytemuck::cast_slice(&SQUARE_VERTICES), wgpu::BufferUsages::VERTEX);

        let square_index_buffer = ctx.create_buffer(bytemuck::cast_slice(&SQUARE_INDICES), wgpu::BufferUsages::INDEX);


        let world_texture_sampler = device.logical_device.create_sampler(&wgpu::SamplerDescriptor::default());

        let world_texture_bind_group_layout = device.logical_device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture { 
                        sample_type: wgpu::TextureSampleType::Float { filterable: false }, 
                        view_dimension: wgpu::TextureViewDimension::D2, 
                        multisampled: false
                    },
                    count: None
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                    count: None
                },
            ],
            label: Some("world_texture_bind_group_layout")
        });

        let world_texture_bind_group = device.logical_device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &world_texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&world_color_texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&world_texture_sampler),
                    }
                ],
                label: Some("world_texture_bind_group"),
        });

        let screen_quad_shader = ShaderModule::new(&device.logical_device, "./src/assets/shaders/screen_quad.wgsl");

        let screen_quad_pipeline_layout = device.logical_device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("screen_quad_pipeline_layout"),
            bind_group_layouts: &[
                &world_texture_bind_group_layout,
            ],
            push_constant_ranges: &[]
        });

        let screen_quad_pipeline = device.logical_device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("screen_quad_pipeline"),
            layout: Some(&screen_quad_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &screen_quad_shader.context_handle,
                entry_point: "vs_main",
                buffers: &[Vertex::buffer_layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &screen_quad_shader.context_handle,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: *surface_format,
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
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false
            },
            multiview: None,
        });


        Self {
            ctx,
            size,
            renderer,
            gui,
            world_color_texture,
            world_depth_texture,
            world_color_texture_view,
            world_depth_texture_view,
            square_vertex_buffer,
            square_index_buffer,
            world_texture_bind_group,
            screen_quad_pipeline,
        }
    }

    pub fn resize(&mut self, new_size: &PhysicalSize<u32>) {
        self.ctx.resize(new_size);
    }

    pub fn render(&mut self, camera: &PerspectiveCamera, window: &Window) {

        let mut encoder = self.ctx.create_encoder("command_encoder");

        self.renderer.render(&mut encoder, &self.world_color_texture_view, &self.world_depth_texture_view);

        let surface_texture = self.ctx.surface
            .get_current_texture()
            .expect("");

        let surface_texture_view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("screen_quad_render_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &surface_texture_view,
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
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.screen_quad_pipeline);

            render_pass.set_bind_group(0, &self.world_texture_bind_group, &[]);

            render_pass.set_vertex_buffer(0, self.square_vertex_buffer.slice(..));

            render_pass.set_index_buffer(self.square_index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(0..SQUARE_INDICES.len() as u32, 0, 0..1);
        }
    
        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [self.ctx.surface_config.width, self.ctx.surface_config.height],
            pixels_per_point: window.scale_factor() as f32,
        };

        self.gui.draw(
            &self.ctx.device,
            &mut encoder,
            &window,
            &surface_texture_view,
            screen_descriptor,
            |ui| example_gui(ui),
        );

        self.ctx.device.queue.submit(std::iter::once(encoder.finish()));

        surface_texture.present();
    }    
}


