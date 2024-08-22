use std::collections::{hash_map, HashMap};

use wgpu::util::DeviceExt;




pub struct Buffer {
    pub buffer: wgpu::Buffer,
    pub is_dirty: bool
}

impl Buffer {
    pub fn new(device: &wgpu::Device, data: &[u8], usage: wgpu::BufferUsages) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: data,
            usage
        });

        Self {
            buffer,
            is_dirty: false
        }
    }
}
pub struct BindGroup {

}

pub struct Pipeline {

}


pub struct GraphicsContext {
    map: std::collections::HashMap<String, Vec<GraphicsPart>>
}

impl GraphicsContext {
    pub fn new() -> Self {
        let map = HashMap::new();

        Self {
            map
        }
    }

    pub fn add_part(&mut self, part: GraphicsPart, name: String) -> Result<(), ()> {
        let mut parts = self.map.get_mut(&name);

        match parts {
            Some(parts) => {
                parts.push(part);
                Ok(())
            },
            None => Err(())
        }
    }
}


pub enum GraphicsPart {
    Buffer(Buffer),
    BindGroup(BindGroup),
    Pipeline(Pipeline)
}