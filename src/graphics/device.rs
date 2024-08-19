use wgpu::util::DeviceExt;



pub struct Device {
    pub logical_device: wgpu::Device,
    pub queue: wgpu::Queue
}

impl Device {
    pub async fn new(adapter: &wgpu::Adapter) -> Self {
        let (logical_device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
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

    pub fn create_buffer(&self, data: &[u8], usage: wgpu::BufferUsages) -> wgpu::Buffer {
        self.logical_device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: data,
            usage
        })
    }
}
