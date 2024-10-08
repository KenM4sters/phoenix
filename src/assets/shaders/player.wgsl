// Vertex shader

struct VertexInput {
    @location(0) a_position: vec3<f32>,
    @location(1) a_normal: vec3<f32>,
    @location(2) a_uv: vec2<f32>
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) vertex_color: vec3<f32>
}

struct CameraUniform {
    view_projection: mat4x4<f32>
}

struct CubeTransformUniform {
    model: mat4x4<f32>
}

@group(0) @binding(0)
var<uniform> u_camera: CameraUniform;

@group(1) @binding(0)
var<uniform> u_model: CubeTransformUniform;



@vertex
fn vs_main(vertices: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = u_camera.view_projection * vec4<f32>(vertices.a_position, 1.0);
    out.vertex_color = vertices.a_normal;
    return out;
} 

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.vertex_color, 1.0);
}
