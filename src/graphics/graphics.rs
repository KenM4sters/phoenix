use egui_wgpu::renderer::ScreenDescriptor;
use winit::{dpi::PhysicalSize, window::Window};


use crate::world::world::World;

use super::{device::Device, gui::{example_gui, Gui}, renderer::Renderer};


pub struct Graphics {
    surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,
    device: Device,
    size: winit::dpi::PhysicalSize<u32>,
    renderer: Renderer,
    gui: Gui,
}

impl Graphics {
    pub async fn new(world: &World, window: &Window) -> Self 
    {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            #[cfg(not(target_arch="wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch="wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });
        
        let surface = unsafe { instance.create_surface(window).unwrap() };
        

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
        };

        surface.configure(&device.logical_device, &surface_config);

        let renderer = Renderer::new(&world, &device, &surface_format);

        let gui = Gui::new(&device.logical_device, surface_format.clone(), None, &window);

        Self {
            surface,
            surface_config,
            device,
            size,
            renderer,
            gui
        }
    }

    pub fn resize(&mut self, new_size: &PhysicalSize<u32>) {
        self.surface_config.width = new_size.width;
        self.surface_config.height = new_size.height;
        
        self.surface.configure(&self.device.logical_device, &self.surface_config);
    }

    pub fn update(&mut self, world: &World) {
        self.renderer.update(&world, &self.device);
    }

    pub fn render(&mut self, window: &Window) {
        let target = self.surface
            .get_current_texture()
            .expect("Target is not ok!");
        
        let view = target.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let depth_texture = self.device.logical_device.create_texture(&wgpu::TextureDescriptor {
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
        
        let depth_texture_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
        

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
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_texture_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            self.renderer.render(&mut render_pass);
        }
    
        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [self.surface_config.width, self.surface_config.height],
            pixels_per_point: window.scale_factor() as f32,
        };

        self.gui.draw(
            &self.device,
            &mut encoder,
            &window,
            &view,
            screen_descriptor,
            |ui| example_gui(ui),
        );

        self.device.queue.submit(std::iter::once(encoder.finish()));
        
        target.present();
    
    }    
}


