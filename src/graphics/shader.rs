use std::fs;


pub struct ShaderModule {
    context_handle: wgpu::ShaderModule
}

impl ShaderModule {
    pub fn new(logical_device: &wgpu::Device, code_path: &str) -> Self {

        let code = fs::read_to_string(code_path).expect("Failed to read shader file!");
        
        let shader_module = logical_device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(code.into())
        });

        Self {
            context_handle: shader_module
        }
    }
}
