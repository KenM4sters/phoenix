

#[derive(Debug)]
pub struct SceneMesh<'a> {
    vertices: Vec<f32>,
    indices: Vec<u32>,
    vertex_buffer_layout: wgpu::VertexBufferLayout<'a>,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
}

#[derive(Debug)]
pub struct Scene<'a> {
    meshes: Vec<SceneMesh<'a>>
}