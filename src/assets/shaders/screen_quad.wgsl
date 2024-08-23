// Vertex shader

struct VertexInput {
    @location(0) a_position: vec3<f32>,
    @location(1) a_normal: vec3<f32>,
    @location(2) a_uv: vec2<f32>
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) uv: vec2<f32>
}


@group(0) @binding(0)
var u_world_texture: texture_2d<f32>;

@group(0) @binding(1)
var s_sampler: sampler;

@vertex
fn vs_main(vertices: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(vertices.a_position, 1.0);
    out.uv = vertices.a_uv;
    return out;
} 

// Fragment shader

@fragment
fn fs_main(vertex_output: VertexOutput) -> @location(0) vec4<f32> {
    var tex_color: vec4<f32> = textureSample(u_world_texture, s_sampler, vertex_output.uv);
    var final_color: vec4<f32> = tex_color;
    return final_color;
}
