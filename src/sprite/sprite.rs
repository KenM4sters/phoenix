
pub enum Weapon {
    Narrow,
    Spread,
}

pub struct Renderable<'a> {
    vertex_buffer: &'a wgpu::Buffer,
    index_buffer: &'a wgpu::Buffer,
    vertex_layout: &'a wgpu::VertexBufferLayout<'a>,
    bind_groups: Vec<wgpu::BindGroup>,
    pipeline: &'a wgpu::RenderPipeline
}

pub struct PlayerSprite<'a> {
    health_points: u32,
    movement_speed: u32,
    weapon: Weapon,
    lives: u32,
    renderable: Renderable<'a>
}