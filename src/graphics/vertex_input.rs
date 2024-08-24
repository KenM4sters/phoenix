use bytemuck::{Pod, Zeroable};



// lib.rs
pub static CUBE_VERTICES: &[Vertex] = &[
    // Front face
    Vertex{position: [-1.0, -1.0,  1.0], normal: [0.0,  0.0,  1.0], uv: [0.0, 0.0]},// Bottom-left
    Vertex{position: [1.0, -1.0,  1.0], normal: [0.0,  0.0,  1.0], uv: [1.0, 0.0]},// Bottom-right
    Vertex{position: [1.0,  1.0,  1.0], normal: [0.0,  0.0,  1.0], uv: [1.0, 1.0]},// Top-right
    Vertex{position: [-1.0,  1.0,  1.0], normal: [0.0,  0.0,  1.0], uv: [0.0, 1.0]},
    // Back
    Vertex{position: [1.0, -1.0, -1.0], normal: [0.0,  0.0, -1.0], uv: [9.0, 9.0]},// Bottom-right
    Vertex{position: [-1.0, -1.0, -1.0], normal: [0.0,  0.0, -1.0], uv: [9.0, 9.0]},// Bottom-left
    Vertex{position: [-1.0,  1.0, -1.0], normal: [0.0,  0.0, -1.0], uv: [9.0, 9.0]},
    Vertex{position: [1.0,  1.0, -1.0], normal: [0.0,  0.0, -1.0], uv: [9.0, 9.0]},// Top-right
    // Left
    Vertex{position: [-1.0,  1.0,  1.0], normal: [-1.0,  0.0,  0.0], uv: [9.0, 9.0]},// Top-right
    Vertex{position: [-1.0,  1.0, -1.0], normal: [-1.0,  0.0,  0.0], uv: [9.0, 9.0]},// Top-left
    Vertex{position: [-1.0, -1.0, -1.0], normal: [-1.0,  0.0,  0.0], uv: [9.0, 9.0]},// Bottom-left
    Vertex{position: [-1.0, -1.0,  1.0], normal: [-1.0,  0.0,  0.0], uv: [9.0, 9.0]},
    // Right
    Vertex{position: [1.0,  1.0, -1.0], normal: [1.0,  0.0,  0.0], uv: [9.0, 9.0]},// Top-right
    Vertex{position: [1.0,  1.0,  1.0], normal: [1.0,  0.0,  0.0], uv: [9.0, 9.0]},// Top-left
    Vertex{position: [1.0, -1.0,  1.0], normal: [1.0,  0.0,  0.0], uv: [9.0, 9.0]},
    Vertex{position: [1.0, -1.0, -1.0], normal: [1.0,  0.0,  0.0], uv: [9.0, 9.0]},// Bottom-right
    // Top
    Vertex{position: [-1.0,  1.0,  1.0], normal: [0.0,  1.0,  0.0], uv: [9.0, 9.0]},// Top-left
    Vertex{position: [1.0,  1.0,  1.0], normal: [0.0,  1.0,  0.0], uv: [9.0, 9.0]},// Top-right
    Vertex{position: [1.0,  1.0, -1.0], normal: [0.0,  1.0,  0.0], uv: [9.0, 9.0]},// Bottom-right
    Vertex{position: [-1.0,  1.0, -1.0], normal: [0.0,  1.0,  0.0], uv: [9.0, 9.0]},
    // Bottom
    Vertex{position: [-1.0, -1.0, -1.0], normal: [0.0, -1.0,  0.0], uv: [9.0, 9.0]},// Bottom-left
    Vertex{position: [1.0, -1.0, -1.0], normal: [0.0, -1.0,  0.0], uv: [9.0, 9.0]},// Bottom-right
    Vertex{position: [1.0, -1.0,  1.0], normal: [0.0, -1.0,  0.0], uv: [9.0, 9.0]},// Top-right
    Vertex{position: [-1.0, -1.0,  1.0], normal: [0.0, -1.0,  0.0], uv: [9.0, 9.0]},// Top-left
            
];


pub const CUBE_INDICES: &[u16] = &[
    // Front face
    0, 1, 2,
    2, 3, 0,
    
    // Back face
    4, 5, 6,
    6, 7, 4,
    
    // Left face
    8, 9, 10,
    10, 11, 8,

    // Right face
    12, 13, 14,
    14, 15, 12,
    
    // Top face
    16, 17, 18,
    18, 19, 16,
    
    // Bottom face
    20, 21, 22,
    22, 23, 20,
];


pub const SQUARE_VERTICES: &[Vertex] = &[
    Vertex{position: [-1.0, -1.0,  1.0], normal: [0.0,  0.0,  1.0], uv: [0.0, 1.0]},// Bottom-left
    Vertex{position: [1.0, -1.0,  1.0], normal: [0.0,  0.0,  1.0], uv: [1.0, 1.0]},// Bottom-right
    Vertex{position: [1.0,  1.0,  1.0], normal: [0.0,  0.0,  1.0], uv: [1.0, 0.0]},// Top-right
    Vertex{position: [-1.0,  1.0,  1.0], normal: [0.0,  0.0,  1.0], uv: [0.0, 0.0]},
];

pub const SQUARE_INDICES: &[u16] = &[
    0, 1, 2,
    2, 3, 0 
];



#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2]
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

