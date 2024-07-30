use std::default;

use wgpu::InstanceDescriptor;
use winit::{
    dpi, event::*, event_loop::EventLoop, keyboard::{KeyCode, PhysicalKey}, window::{Window, WindowBuilder}
};


struct Surface<'a> {
    surface: wgpu::Surface<'a>,
    config: Option<wgpu::SurfaceConfiguration>
}


impl<'a> Surface<'a> {
    fn new(instance: &'a wgpu::Instance, window: &'a Window) -> Surface<'a> {
        let surface = instance.create_surface(window).unwrap();

        Self {
            surface,
            config: None
        }
    }

    fn configure(&'a self, physical_device: &'a PhysicalDevice<'a>, size: winit::dpi::PhysicalSize<u32>) {
        let surface_caps = self.surface.get_capabilities(&physical_device.adapter);

        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        
        // self.config = Some(wgpu::SurfaceConfiguration {
        //     usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        //     format: surface_format,
        //     width: size.width,
        //     height: size.height,
        //     present_mode: surface_caps.present_modes[0],
        //     alpha_mode: surface_caps.alpha_modes[0],
        //     view_formats: vec![],
        //     desired_maximum_frame_latency: 2
        // });
    }
}

struct PhysicalDevice<'a> {
    adapter: wgpu::Adapter,
    instance: &'a wgpu::Instance,
    surface: &'a Surface<'a>,
}


impl<'a> PhysicalDevice<'a> {
    async fn new(instance: &'a wgpu::Instance, surface: &'a Surface<'a>) -> PhysicalDevice<'a> {
        let adapter= instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            compatible_surface: Some(&surface.surface),
            force_fallback_adapter: false
        }).await.unwrap();

        Self {
            adapter,
            instance,
            surface
        }
    }
}


struct LogicalDevice<'a> {
    logical_device: wgpu::Device,
    command_queue: wgpu::Queue,
    physical_device: &'a PhysicalDevice<'a>,
}

impl<'a> LogicalDevice<'a> {
    async fn new(physical_device: &'a PhysicalDevice<'a>) -> LogicalDevice<'a> {
        let (logical_device, command_queue) = physical_device.adapter.request_device(&wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            #[cfg(not(target_arch="wasm32"))]
            required_limits: wgpu::Limits::default(),
            #[cfg(target_arch="wasm32")]
            required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
            memory_hints: wgpu::MemoryHints::Performance,
            label: None
        },
        None
        ).await.unwrap();

        Self {
            logical_device,
            command_queue,
            physical_device
        }
    }
}


struct Graphics<'a> 
{
    surface: Surface<'a>,
    physical_device: PhysicalDevice<'a>,
    logical_device: LogicalDevice<'a>, 
    size: winit::dpi::PhysicalSize<u32>,
    window: &'a Window
}


impl<'a> Graphics<'a> {
    async fn new(window: &'a Window) -> Graphics<'a> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(InstanceDescriptor{
            #[cfg(not(target_arch="wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch="wasm32")]
            backends: wgpu::Backends::GL,
            flags: wgpu::InstanceFlags::VALIDATION | wgpu::InstanceFlags::DEBUG,
            dx12_shader_compiler: wgpu::Dx12Compiler::default(),
            gles_minor_version: wgpu::Gles3MinorVersion::default()
        });

        let surface = Surface::new(&instance, &window);

        let physical_device = PhysicalDevice::new(&instance, &surface).await;

        let logical_device = LogicalDevice::new(&physical_device).await;

        surface.configure(&physical_device, size);

        Self {
            surface,
            physical_device,
            logical_device,
            size,
            window
        }

    }

    fn window(&self) -> &'a Window {
        self.window
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {

    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {

    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        Ok(())
    }


}



