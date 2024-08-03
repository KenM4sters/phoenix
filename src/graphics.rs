use wgpu::InstanceDescriptor;
use winit::{
    dpi, event::*, event_loop::EventLoop, keyboard::{KeyCode, PhysicalKey}, window::{Window, WindowBuilder}
};


struct Surface<'a> {
    surface: wgpu::Surface<'a>,
    config: Option<wgpu::SurfaceConfiguration>
}

impl<'a, 'b> Surface<'a> {
    async fn new(instance: &'a wgpu::Instance, window: &'a Window) -> Surface<'a> {
        Self {
            surface: instance.create_surface(window).unwrap(),
            config: None
        }
    }

    fn configure(&mut self, physical_device: &'b PhysicalDevice, size : winit::dpi::PhysicalSize<u32>) {
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

impl<'a> LogicalDevice {
    async fn new(physical_device: &'a PhysicalDevice) -> LogicalDevice {
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

pub struct Graphics<'a> {
    physical_device: PhysicalDevice,
    logical_device: LogicalDevice,
    window: &'a Window,
    size: winit::dpi::PhysicalSize<u32>
}

impl<'a> Graphics<'a> {
    pub async fn new(window: &'a Window) -> Graphics<'a> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            #[cfg(not(target_arch="wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch="wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        let mut surface = Surface::new(&instance, window).await;

        let physical_device = PhysicalDevice::new(&instance, &surface).await;

        let logical_device = LogicalDevice::new(&physical_device).await;
        
        surface.configure(&physical_device, size);

        Self {
            physical_device,
            logical_device,
            window,
            size
        }
    }

    pub fn resize() {

    }

    pub fn update() {

    }

    pub fn render() -> Result<(), ()> {
        Ok(())
    }

}


