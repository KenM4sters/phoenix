use wgpu::InstanceDescriptor;
use winit::{
    dpi, event::*, event_loop::EventLoop, keyboard::{KeyCode, PhysicalKey}, window::{Window, WindowBuilder}
};


struct Surface {
    surface: wgpu::Surface,
    config: Option<wgpu::SurfaceConfiguration>,
}

impl<'a> Surface {
    fn new(instance: &wgpu::Instance, window: &Window) -> Surface {
        let surface = unsafe { instance.create_surface(window) }.unwrap();

        Self {
            surface,
            config: None,
        }
    }

    fn configure(&mut self, physical_device: &PhysicalDevice, size: winit::dpi::PhysicalSize<u32>) {
        let surface_caps = self.surface.get_capabilities(&physical_device.adapter);

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

        self.surface.configure(&physical_device.device, self.config.as_ref().unwrap());
    }
}

struct PhysicalDevice {
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl PhysicalDevice {
    async fn new(instance: &wgpu::Instance, surface: &wgpu::Surface) -> PhysicalDevice {
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            compatible_surface: Some(surface),
            force_fallback_adapter: false,
        }).await.unwrap();

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            #[cfg(not(target_arch = "wasm32"))]
            required_limits: wgpu::Limits::default(),
            #[cfg(target_arch = "wasm32")]
            required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
            label: None,
        }, None).await.unwrap();

        Self {
            adapter,
            device,
            queue,
        }
    }
}

struct Graphics {
    surface: Surface,
    physical_device: PhysicalDevice,
    size: winit::dpi::PhysicalSize<u32>,
    window: Window,
}

impl<'a> Graphics {
    async fn new(window: Window) -> Graphics<'a> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            flags: wgpu::InstanceFlags::VALIDATION | wgpu::InstanceFlags::DEBUG,
            dx12_shader_compiler: wgpu::Dx12Compiler::default(),
            gles_minor_version: wgpu::Gles3MinorVersion::default(),
        });

        let surface = Surface::new(&instance, &window);

        let physical_device = PhysicalDevice::new(&instance, &surface.surface).await;

        let mut surface = surface;
        surface.configure(&physical_device, size);

        Self {
            surface,
            physical_device,
            size,
            window,
        }
    }

    fn window(&self) -> &Window {
        &self.window
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.surface.configure(&self.physical_device, new_size);
    }

    fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {}

    fn render(&mut self) {}
}

