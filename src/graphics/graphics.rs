use std::{ops::Deref, rc::Rc};

use winit::{dpi::PhysicalSize, window::Window};



struct Surface<'a> {
    window: Rc<Window>,
    surface: wgpu::Surface<'a>,
    config: Option<wgpu::SurfaceConfiguration>
}

impl<'a> Surface<'a> {
    async fn new(instance: &wgpu::Instance, window: Rc<Window>) -> Self
    {
        let surface = unsafe {
            instance.create_surface(window.as_ref()).unwrap()
        };

        Self {
            window,
            surface: surface,
            config: None
        }
    }

    fn configure(&mut self, logical_device: &LogicalDevice, physical_device: &PhysicalDevice, size : &winit::dpi::PhysicalSize<u32>) 
    {
        let surface_caps = self.surface.get_capabilities(&physical_device.physical_device);

        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        self.config = Some(wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        });

        self.surface.configure(&logical_device.logical_device, self.config.as_ref().unwrap());
    }
}

struct PhysicalDevice {
    physical_device: wgpu::Adapter
}

impl<'a> PhysicalDevice {
    async fn new(instance: &'a wgpu::Instance, surface: &'a Surface<'a>) -> PhysicalDevice {
        Self {
            physical_device: instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface.surface),
                force_fallback_adapter: false
            }).await.unwrap()
        }
    }
}


struct LogicalDevice {
    logical_device: wgpu::Device,
    queue: wgpu::Queue
}

impl LogicalDevice {
    async fn new(physical_device:  &PhysicalDevice) -> LogicalDevice {
        let (logical_device, queue) = physical_device.physical_device.request_device(&wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            #[cfg(not(target_arch="wasm32"))]
            required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
            #[cfg(target_arch="wasm32")]
            requred_limits: wgpu::Limits::default(),
            memory_hints: wgpu::MemoryHints::Performance,
            label: None
        }, 
        None).await.unwrap();

        Self {
            logical_device,
            queue
        }
    }
}

pub struct Graphics<'a> 
{
    instance: wgpu::Instance,
    surface: Surface<'a>,
    physical_device: PhysicalDevice,
    logical_device: LogicalDevice,
    size: winit::dpi::PhysicalSize<u32>,
}

impl<'a> Graphics<'a> {
    pub async fn new(window: Rc<Window>) -> Self 
    {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            #[cfg(not(target_arch="wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch="wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        let mut surface = Surface::new(&instance, Rc::clone(&window)).await;

        let physical_device = PhysicalDevice::new(&instance, &surface).await;

        let logical_device = LogicalDevice::new(&physical_device).await;
        
        surface.configure(&logical_device, &physical_device, &size);

        Self {
            instance,
            surface,
            physical_device,
            logical_device,
            size,
        }
    }

    pub fn resize(&mut self, new_size: &PhysicalSize<u32>) {
        self.surface.configure(&self.logical_device, &self.physical_device, new_size);
    }

    pub fn update() {

    }

    pub fn render(&self) -> Result<(), ()> {
        let target = self.surface.surface
            .get_current_texture()
            .expect("Target is not ok!");
    
        let view = target.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.logical_device.logical_device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }
    
        // submit will accept anything that implements IntoIter
        self.logical_device.queue.submit(std::iter::once(encoder.finish()));
        
        target.present();
    
        Ok(())
    }    
}


