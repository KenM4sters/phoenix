use wgpu::util::DeviceExt;



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

    pub fn create_buffer(&self, data: &[u8], usage: wgpu::BufferUsages) -> wgpu::Buffer {
        self.logical_device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: data,
            usage
        })
    }
}
