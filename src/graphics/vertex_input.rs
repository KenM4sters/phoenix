
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    uv: [f32; 3]
}

impl Vertex {
    fn new(position: Option<[f32; 3]>, normal: Option<[f32; 3]>, uv: Option<[f32; 3]>) -> Vertex {
        Self {
            position: position.unwrap_or([0.0, 0.0, 0.0]),
            normal: normal.unwrap_or([0.0, 0.0, 0.0]), 
            uv: uv.unwrap_or([0.0, 0.0, 0.0])
        }
    }

    fn buffer_layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress, 
            step_mode: wgpu::VertexStepMode::Vertex, 
            attributes: &[ 
                wgpu::VertexAttribute {
                    offset: 0, 
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                }
            ]
        }
    }
}

