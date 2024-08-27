use std::{fs, rc::Rc, sync::Arc};
use wgpu::{util::DeviceExt};
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

pub trait State {
    fn needs_update(&self) -> bool;
}

pub struct Buffer {
    pub gpu_buffer: wgpu::Buffer,
    needs_update: bool,
}

impl Buffer {
    pub fn new(device: &wgpu::Device, label: &str, data: &[u8], usage: wgpu::BufferUsages) -> Self {
        let gpu_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(label),
            contents: data,
            usage
        });

        Self {
            gpu_buffer,
            needs_update: false,
        }
    }
}

impl State for Buffer {
    fn needs_update(&self) -> bool {
        self.needs_update
    }
}



pub struct Texture {
    pub gpu_texture: wgpu::Texture
}

impl Texture {
    pub fn new(
        device: &wgpu::Device, 
        label: &str, 
        size: wgpu::Extent3d, 
        mip_level_count: u32, 
        sample_count: u32, 
        dimension: wgpu::TextureDimension, 
        format: wgpu::TextureFormat, 
        usage: wgpu::TextureUsages
    ) -> Self {
        let gpu_texture = device.create_texture(&wgpu::TextureDescriptor {
            size,
            mip_level_count, 
            sample_count,
            dimension,
            format,
            usage,
            label: Some(label),
            view_formats: &[],
        });

        Self {
            gpu_texture
        }
    }
}

pub struct TextureView {
    gpu_texture_view: wgpu::TextureView
}

impl TextureView {
    pub fn new(texture: &wgpu::Texture, label: &str) -> Self {
        let gpu_texture_view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some(label),
            ..Default::default()
        });

        Self {
            gpu_texture_view
        }
    }
}

pub struct Sampler {
    gpu_sampler: wgpu::Sampler
}

impl Sampler {
    pub fn new(
        device: &wgpu::Device, 
        label: &str,
        address_mode_u: wgpu::AddressMode, 
        address_mode_v: wgpu::AddressMode, 
        address_mode_w: wgpu::AddressMode, 
        min_filter: wgpu::FilterMode, 
        mag_filter: wgpu::FilterMode, 
        mipmap_filter: wgpu::FilterMode
    ) -> Self {
        let gpu_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u,
            address_mode_v,
            address_mode_w,
            mag_filter,
            min_filter,
            mipmap_filter,
            label: Some(label),
            ..Default::default()
        });

        Self {
            gpu_sampler
        }
    }
}


pub struct BindGroupLayoutEntry {
    binding: u32, 
    visibility: wgpu::ShaderStages, 
    ty: wgpu::BufferBindingType,
}

pub struct BindGroupLayout {
    gpu_bind_group_layout: wgpu::BindGroupLayout,
}

impl BindGroupLayout {
    pub fn new(
        device: &wgpu::Device, 
        label: &str, 
        entries: Vec<BindGroupLayoutEntry>
    ) -> Self {
        let mut gpu_entries = vec![];

        entries.iter().for_each(|entry| {
            gpu_entries.push(wgpu::BindGroupLayoutEntry {
                binding: entry.binding,
                visibility: entry.visibility,
                ty: wgpu::BindingType::Buffer { 
                    ty: entry.ty, 
                    has_dynamic_offset: false, 
                    min_binding_size: None
                },
                count: None
            })
        });

        let gpu_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &gpu_entries,
            label: Some("cube_bind_group_layout")
        });

        Self {
            gpu_bind_group_layout
        }
    }
}

pub struct BindGroupEntry<'a> {
    binding: u32,
    resource: wgpu::BindingResource<'a>,
}

pub struct BindGroup {
    gpu_bind_group: wgpu::BindGroup,
}

impl BindGroup {
    pub fn new(device: &wgpu::Device, label: &str, layout: &wgpu::BindGroupLayout, entries: Vec<BindGroupEntry>) -> Self {
        let mut gpu_entries = vec![];

        entries.iter().for_each(|entry| {
            gpu_entries.push(wgpu::BindGroupEntry {
                binding: entry.binding,
                resource: entry.resource,
            })
        });

        let gpu_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &gpu_entries,
            label: Some(label)
        });

        Self {
            gpu_bind_group
        }
    }
}

pub struct Shader {
    pub shader: wgpu::ShaderModule
}

impl Shader {
    pub fn new(logical_device: &wgpu::Device, label: &str, code_path: &str) -> Self {
        let code = fs::read_to_string(code_path)
            .expect("Failed to read shader file!");
        
        let shader = logical_device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(label),
            source: wgpu::ShaderSource::Wgsl(code.into())
        });

        Self {
            shader
        }
    }
}

pub struct RenderPipeline {
    gpu_render_pipeline: wgpu::RenderPipeline,
}

impl RenderPipeline {
    pub fn new(
        device: &wgpu::Device, 
        label: &str,
        layout: wgpu::PipelineLayout, 
        shader: &wgpu::ShaderModule,
        buffers: &[wgpu::VertexBufferLayout],
        color_target_state: Option<wgpu::ColorTargetState>,
        depth_target_state: Option<wgpu::DepthStencilState>,
        topology: wgpu::PrimitiveTopology,
        polygon_mode: wgpu::PolygonMode,
    ) -> Self {
        let gpu_render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(label),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: "vs_main",
                buffers,
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: "fs_main",
                targets: &[color_target_state],
            }),
            primitive: wgpu::PrimitiveState {
                topology,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode,
                unclipped_depth: false,
                conservative: false
            },
            depth_stencil: depth_target_state,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false
            },
            multiview: None,
        });

        Self {
            gpu_render_pipeline
        }
    }
}


pub enum Renderables {
    Buffer(Buffer),
    Texture(Texture),
    TextureView(TextureView),
    Sampler(Sampler),
    BindGroupLayout(BindGroupLayout),
    BindGroup(BindGroup),
    Shader(Shader),
    RenderPipeline(RenderPipeline),
}


pub struct Context {
    pub device: Device, 
    pub surface: wgpu::Surface,
    pub surface_config: wgpu::SurfaceConfiguration,  
    pub buffers: std::collections::HashMap<String, Rc<Buffer>>,
    pub textures: std::collections::HashMap<String, Rc<Texture>>,
    pub texture_views: std::collections::HashMap<String, Rc<TextureView>>,
    pub samplers: std::collections::HashMap<String, Rc<Sampler>>,
    pub bind_group_layouts: std::collections::HashMap<String, Rc<BindGroupLayout>>,
    pub bind_groups: std::collections::HashMap<String, Rc<BindGroup>>,
    pub shaders: std::collections::HashMap<String, Rc<Shader>>,
    pub render_pipelines: std::collections::HashMap<String, Rc<RenderPipeline>>,
}



impl Context {
    ///
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

        let buffers = std::collections::HashMap::new();
        let textures = std::collections::HashMap::new();
        let texture_views = std::collections::HashMap::new();
        let samplers = std::collections::HashMap::new();
        let bind_group_layouts = std::collections::HashMap::new();
        let bind_groups = std::collections::HashMap::new();
        let shaders = std::collections::HashMap::new();
        let render_pipelines = std::collections::HashMap::new();

        Self {
            device,
            surface,
            surface_config,
            buffers,
            textures,
            texture_views,
            samplers,
            bind_group_layouts,
            bind_groups,
            shaders,
            render_pipelines,
        }
    }

    ///
    pub fn resize(&mut self, new_size: &PhysicalSize<u32>) {
        self.surface_config.width = new_size.width;
        self.surface_config.height = new_size.height;
        
        self.surface.configure(&self.device.logical_device, &self.surface_config);
    }

    ///
    pub fn create_encoder(&mut self, label: &str) -> wgpu::CommandEncoder {
        self.device.logical_device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some(label),
        })
    }

    ///
    pub fn create_buffer(
        &self, 
        label: &str, 
        data: &[u8], 
        usage: wgpu::BufferUsages
    ) {
        let buffer = Rc::new(Buffer::new(&self.device.logical_device, label, data, usage));
        self.buffers.insert(label.to_string(), buffer);
    }

    ///
    pub fn create_texture(
        &self, 
        label: &str,         
        size: wgpu::Extent3d, 
        mip_level_count: u32, 
        sample_count: u32, 
        dimension: wgpu::TextureDimension, 
        format: wgpu::TextureFormat, 
        usage: wgpu::TextureUsages
    ) {
        let texture = Rc::new(Texture::new(&self.device.logical_device, label, size, mip_level_count, sample_count, dimension, format, usage));
        self.textures.insert(label.to_string(), texture);
    }

    ///
    pub fn create_shader(
        &self, 
        label: &str, 
        code_path: &str
    ) {
        let shader = Rc::new(Shader::new(&self.device.logical_device, label, code_path));
        self.shaders.insert(label.to_string(), shader);
    }

    ///
    pub fn create_sampler(
        &self,
        label: &str,
        address_mode_u: wgpu::AddressMode, 
        address_mode_v: wgpu::AddressMode, 
        address_mode_w: wgpu::AddressMode, 
        min_filter: wgpu::FilterMode, 
        mag_filter: wgpu::FilterMode, 
        mipmap_filter: wgpu::FilterMode
    ) {
        let sampler = Rc::new(Sampler::new(&self.device.logical_device, label, address_mode_u, address_mode_v, address_mode_w, min_filter, mag_filter, mipmap_filter));
        self.samplers.insert(label.to_string(), sampler);
    }

    ///
    pub fn create_texture_view(
        &self,
        texture: &wgpu::Texture,
        label: &str
    ) {
        let texture_view = Rc::new(TextureView::new(&texture, label));
        self.texture_views.insert(label.to_string(), texture_view);
    }

    pub fn get_buffer(&self, label: &str) -> &wgpu::Buffer {
        &self.buffers
            .get(label)
            .expect(&format!("Failed to get {} from ctx.buffers!", label.to_string()))
            .as_ref()
            .gpu_buffer
    }

    pub fn get_shader(&self, label: &str) -> &wgpu::ShaderModule {
        &self.shaders
            .get(label)
            .expect(&format!("Failed to get {} from ctx.shaders!", label.to_string()))
            .as_ref()
            .shader
    }

    pub fn get_texture(&self, label: &str) -> &wgpu::Texture {
        &self.textures
            .get(label)
            .expect(&format!("Failed to get {} from ctx.textures!", label.to_string()))
            .as_ref()
            .gpu_texture
    }

    pub fn get_texture_view(&self, label: &str) -> &wgpu::TextureView {
        &self.texture_views
            .get(label)
            .expect(&format!("Failed to get {} from ctx.texture_views!", label.to_string()))
            .as_ref()
            .gpu_texture_view
    }

    pub fn get_sampler(&self, label: &str) -> &wgpu::Sampler {
        &self.samplers
            .get(label)
            .expect(&format!("Failed to get {} from ctx.samplers!", label.to_string()))
            .as_ref()
            .gpu_sampler
    }

    pub fn get_bind_group_layout(&self, label: &str) -> &wgpu::BindGroupLayout {
        &self.bind_group_layouts
            .get(label)
            .expect(&format!("Failed to get {} from ctx.bind_group_layouts!", label.to_string()))
            .as_ref()
            .gpu_bind_group_layout
    }

    pub fn get_bind_group(&self, label: &str) -> &wgpu::BindGroup {
        &self.bind_groups
            .get(label)
            .expect(&format!("Failed to get {} from ctx.bind_groups!", label.to_string()))
            .as_ref()
            .gpu_bind_group
    }

    pub fn get_render_pipeline(&self, label: &str) -> &wgpu::RenderPipeline {
        &self.render_pipelines
            .get(label)
            .expect(&format!("Failed to get {} from ctx.render_pipelines!", label.to_string()))
            .as_ref()
            .gpu_render_pipeline
    }
}