use wgpu::util::DeviceExt;
use winit::{dpi::PhysicalSize, window::Window};

pub struct Device {
    pub logical_device: wgpu::Device,
    pub queue: wgpu::Queue
}

impl Device {
    pub async fn new(adapter: &wgpu::Adapter) -> Self {
        let (logical_device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            features: wgpu::Features::default(),
            limits: wgpu::Limits::default(),
            label: None
        }, 
        None).await.unwrap();

        Self {
            logical_device,
            queue
        }
    }
}


pub struct Context {
    pub device: Device,
    pub surface: wgpu::Surface,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub render_pipelines: std::collections::HashMap<String, wgpu::RenderPipeline>,
    pub bind_groups: std::collections::HashMap<String, wgpu::BindGroup>,
    pub vertex_buffers: std::collections::HashMap<String, wgpu::Buffer>,
}


impl Context {
    pub async fn new(size: PhysicalSize<u32>, window: &Window) -> Self {
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

        let device = Device::new(&adapter).await;

        surface.configure(&device.logical_device, &surface_config);

        Self {
            device,
            surface,
            surface_config
        }
    }

    pub fn resize(&mut self, new_size: &PhysicalSize<u32>) {
        self.surface_config.width = new_size.width;
        self.surface_config.height = new_size.height;
        
        self.surface.configure(&self.device.logical_device, &self.surface_config);
    }

    pub fn create_encoder(&mut self, label: &str) -> wgpu::CommandEncoder {
        self.device.logical_device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder")
        })
    }

    pub fn create_buffer(&self, data: &[u8], usage: wgpu::BufferUsages) -> wgpu::Buffer {
        self.device.logical_device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: data,
            usage
        })
    }
}