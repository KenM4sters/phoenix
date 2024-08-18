use wgpu::util::DeviceExt;



#[derive(Debug)]
pub struct Buffer {
    data: u8,
    usage: wgpu::BufferUsages,
    context_handle: wgpu::Buffer
}

impl Buffer {
    pub fn new(logical_device: &wgpu::Device, data: u8, usage: wgpu::BufferUsages) -> Self {
        let buffer = logical_device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: &[data],
            usage
        });

        Self {
            data,
            usage,
            context_handle: buffer
        }
    }

    pub fn update(&mut self, logical_device: &wgpu::Device, data: u8, usage: wgpu::BufferUsages) {
        self.data = data;
        self.usage = usage;

        let buffer = logical_device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: &[data],
            usage
        });
    }
}