use winit::{dpi::PhysicalSize, window::Window};

use crate::game::game::Game;

use super::{device::Device, pipeline::PipelineManager, renderer::Renderer, vertex_input::{INDICES, VERTICES}};


pub struct Graphics<'a> {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'a>,
    surface_config: wgpu::SurfaceConfiguration,
    adapter: wgpu::Adapter,
    device: Device,
    size: winit::dpi::PhysicalSize<u32>,
    renderer: Renderer,
    pipeline_manager: PipelineManager,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
}

impl<'a> Graphics<'a> {
    pub async fn new(window: &'a Window) -> Self 
    {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            #[cfg(not(target_arch="wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch="wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });
        
        let surface = instance.create_surface(window).unwrap();
        

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false
        }).await.unwrap();

        let device = Device::new(&adapter).await; 

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device.logical_device, &surface_config);

        let renderer = Renderer::new(size);

        let pipeline_manager = PipelineManager::new(&device.logical_device, &surface_format);

        let vertex_buffer = device.create_buffer( bytemuck::cast_slice(VERTICES), wgpu::BufferUsages::VERTEX);

        let index_buffer = device.create_buffer( bytemuck::cast_slice(INDICES), wgpu::BufferUsages::INDEX);

        Self {
            instance,
            surface,
            surface_config,
            adapter,
            device,
            size,
            renderer,
            pipeline_manager,
            vertex_buffer,
            index_buffer
        }
    }

    pub fn resize(&mut self, new_size: &PhysicalSize<u32>) {
        self.surface_config.width = new_size.width;
        self.surface_config.height = new_size.height;
        
        self.surface.configure(&self.device.logical_device, &self.surface_config);
    }

    pub fn update(&self) {
        
    }

    pub fn render(&self, game: &Game) -> Result<(), ()> {
        let target = self.surface
            .get_current_texture()
            .expect("Target is not ok!");
        
        let view = target.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.logical_device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder")
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
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

            render_pass.set_pipeline(&self.pipeline_manager.player_pipeline);

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(0..6, 0, 0..1);
        }

        game.for_each_sprite(|sprite| {
        });

        self.device.queue.submit(std::iter::once(encoder.finish()));
        
        target.present();
    
        Ok(())
    }    
}


