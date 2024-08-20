use bytemuck::{Pod, Zeroable};



// lib.rs
pub const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.5, -0.5, 0.0], normal: [0.0, 0.0, 1.0], uv: [0.0, 0.0] },
    Vertex { position: [-0.5, 0.5, 0.0], normal: [0.0, 0.0, 1.0], uv: [0.0, 1.0] },
    Vertex { position: [0.5, 0.5, 0.0], normal: [0.0, 0.0, 1.0], uv: [1.0, 1.0] },
    Vertex { position: [0.5, -0.5, 0.0], normal: [0.0, 0.0, 1.0], uv: [1.0, 0.0] },
];

pub const INDICES: &[u16] = &[
    0, 1, 2,
    0, 2, 3
];


#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    uv: [f32; 2]
}


impl Vertex {
    pub fn buffer_layout() -> wgpu::VertexBufferLayout<'static> {
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
                    offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                }
            ]
        }
    }
}

