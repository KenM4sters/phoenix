

#[derive(Debug)]
pub enum Weapon {
    Narrow,
    Spread,
}

#[derive(Debug)]
pub struct Renderable<'a> {
    vertex_buffer: &'a wgpu::Buffer,
    index_buffer: &'a wgpu::Buffer,
    vertex_layout: &'a wgpu::VertexBufferLayout<'a>,
    bind_groups: Vec<wgpu::BindGroup>,
    pipeline: &'a wgpu::RenderPipeline
}

#[derive(Debug)]
pub struct PlayerSprite<'a> {
    health_points: u32,
    movement_speed: u32,
    weapon: Weapon,
    lives: u32,
    renderable: Renderable<'a>
}