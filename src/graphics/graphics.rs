use egui::epaint::text::layout;
use egui_wgpu::renderer::ScreenDescriptor;
use winit::{dpi::PhysicalSize, window::Window};


use crate::world::{camera::PerspectiveCamera, world::World};

use super::{context::{BindGroupEntry, BindGroupLayoutEntry, Context}, gui::{example_gui, Gui}, renderer::Renderer, vertex_input::{Vertex, SQUARE_INDICES, SQUARE_VERTICES}};


pub struct Graphics {
    pub ctx: Context,
    size: winit::dpi::PhysicalSize<u32>,
    renderer: Renderer,
    gui: Gui,
}

impl Graphics {
    pub async fn new(world: &World, window: &Window) -> Self 
    {
        let size = window.inner_size();

        let ctx = Context::new(size.clone(), &window).await; 

        let renderer = Renderer::new(&world, &ctx);

        let device = &ctx.device;
        let surface_format = &ctx.surface_config.format;

        let gui = Gui::new(&device.logical_device, surface_format.clone(), None, &window);

        let world_color_texture = ctx.create_texture(
            "world_color_texture", 
            wgpu::Extent3d { width: 1600, height: 1200, depth_or_array_layers: 1,}, 
            1, 
            1, 
            wgpu::TextureDimension::D2, 
            wgpu::TextureFormat::Rgba32Float, 
            wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING
        );

        let world_color_texture_view = ctx.create_texture_view(&world_color_texture.gpu_texture, "world_color_texture_view");

        let world_depth_texture = ctx.create_texture(
            "world_depth_texture", 
            wgpu::Extent3d { width: 1600, height: 1200, depth_or_array_layers: 1,}, 
            1, 
            1, 
            wgpu::TextureDimension::D2, 
            wgpu::TextureFormat::Depth32Float, 
            wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING
        );
            
        let world_depth_texture_view = ctx.create_texture_view(&world_depth_texture.gpu_texture, "world_depth_texture_view");

        ctx.create_buffer("square_vertex_buffer", bytemuck::cast_slice(&SQUARE_VERTICES), wgpu::BufferUsages::VERTEX);

        ctx.create_buffer("square_index_buffer", bytemuck::cast_slice(&SQUARE_INDICES), wgpu::BufferUsages::INDEX);


        let default_sampler = ctx.create_sampler(
            "default_sampler",
            wgpu::AddressMode::Repeat,
            wgpu::AddressMode::Repeat,
            wgpu::AddressMode::Repeat,
            wgpu::FilterMode::Linear,
            wgpu::FilterMode::Linear,
            wgpu::FilterMode::Nearest
        );

        let world_texture_bind_group_layout = ctx.create_bind_group_layout(
            "world_texture_bind_group_layout", 
            vec![
                BindGroupLayoutEntry {
                    binding: 0, 
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture { 
                        sample_type: wgpu::TextureSampleType::Float { filterable: false }, 
                        view_dimension: wgpu::TextureViewDimension::D2, 
                        multisampled: false
                    }
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture { 
                        sample_type: wgpu::TextureSampleType::Float { filterable: false }, 
                        view_dimension: wgpu::TextureViewDimension::D2, 
                        multisampled: false
                    },
                }
            ]
        );

        let world_texture_bind_group = ctx.create_bind_group(
            "world_texture_bind_group",
            &world_texture_bind_group_layout.gpu_bind_group_layout, 
            vec![
                BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&world_color_texture_view.gpu_texture_view)
                },
                BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&default_sampler.gpu_sampler)
                },
            ]
        );


        let screen_quad_shader = ctx.create_shader("screen_quad_shader", "./src/assets/shaders/screen_quad.wgsl");

        let screen_quad_pipeline_layout = device.logical_device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("screen_quad_pipeline_layout"),
            bind_group_layouts: &[
                &world_texture_bind_group_layout.gpu_bind_group_layout,
            ],
            push_constant_ranges: &[]
        });

        let screen_quad_pipeline = ctx.create_render_pipeline(
            "screen_quad_pipeline",
            screen_quad_pipeline_layout,
            &screen_quad_shader.shader,
            &[Vertex::buffer_layout()],
            Some(wgpu::ColorTargetState {
                format: *surface_format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL
            }),
            None,
            wgpu::PrimitiveTopology::TriangleList,
            wgpu::PolygonMode::Fill
        );

        Self {
            ctx,
            size,
            renderer,
            gui,
        }
    }

    pub fn resize(&mut self, new_size: &PhysicalSize<u32>) {
        self.ctx.resize(new_size);
    }

    pub fn render(&mut self, camera: &PerspectiveCamera, window: &Window) {

        let mut encoder = self.ctx.create_encoder("command_encoder");


        let world_color_texture_view = self.ctx.get_texture_view("world_color_texture_view");
        let world_depth_texture_view = self.ctx.get_texture_view("world_depth_texture_view");
        let screen_quad_pipeline = self.ctx.get_render_pipeline("screen_quad_pipeline");
        let screen_quad_vertex_buffer = self.ctx.get_buffer("screen_quad_vertex_buffer");
        let screen_quad_index_buffer = self.ctx.get_buffer("screen_quad_index_buffer");
        let world_texture_bind_group = self.ctx.get_bind_group("world_texture_bind_group");

        self.renderer.render(&mut encoder, &world_color_texture_view.gpu_texture_view, &world_depth_texture_view.gpu_texture_view);

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

            render_pass.set_pipeline(&screen_quad_pipeline.gpu_render_pipeline);

            render_pass.set_bind_group(0, &world_texture_bind_group.gpu_bind_group, &[]);

            render_pass.set_vertex_buffer(0, screen_quad_vertex_buffer.gpu_buffer.slice(..));

            render_pass.set_index_buffer(screen_quad_index_buffer.gpu_buffer.slice(..), wgpu::IndexFormat::Uint16);

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


