use std::io::{BufReader, Cursor};

use crate::graphics::{context::{BindGroupEntry, BindGroupLayoutEntry, Context}, renderer::TransformUniform, vertex_input::Vertex};

use super::world::{Mesh, Transform};

use cgmath::{EuclideanSpace, SquareMatrix};
use gltf::Gltf;

use std::rc::Rc;



pub struct Model {
    transform: Transform,
    meshes: Vec<Mesh>,
}

impl Model {
    pub fn model_matrix(&self) -> cgmath::Matrix4<f32> {
        let model = cgmath::Matrix4::<f32>::identity();

        let translation = cgmath::Matrix4::from_translation(self.transform.position.to_vec());
        let translated_model = model * translation;

        let scale = cgmath::Matrix4::from_nonuniform_scale(self.transform.scale.x, self.transform.scale.y, self.transform.scale.z);
        let scaled_model = translated_model * scale;

        scaled_model
    }
}

pub struct ModelBuilder<'a> {  
    ctx: &'a Context,
    file_path: &'a str,
    transform: Option<Transform>,
}

impl<'a> ModelBuilder<'a> {
    pub fn new(ctx: &Context, file_path: &str) -> Self {
        Self {
            ctx,
            file_path,
            transform: None
        }
    }

    pub fn with_position(mut self, position: cgmath::Point3<f32>) -> Self {
        self.transform
            .get_or_insert_with(Transform::default)
            .position = position;
        self
    }

    pub fn with_scale(mut self, scale: cgmath::Vector3<f32>) -> Self {
        self.transform
            .get_or_insert_with(Transform::default)
            .scale = scale;
        self
    }
    pub fn with_rotation(mut self, rotation: cgmath::Quaternion<f32>) -> Self {
        self.transform
            .get_or_insert_with(Transform::default)
            .rotation = rotation;
        self
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = Some(transform);
        self
    } 

    pub fn build(&self) -> Model {
        let gltf_string = std::fs::read_to_string(self.file_path).expect("Failed to read gltf string");
        let gltf_cursor = Cursor::new(gltf_string);
        let gltf_reader = BufReader::new(gltf_cursor);
        let gltf = Gltf::from_reader(gltf_reader).expect("Failed to read gltf!");

        let mut buffer_data = Vec::new();
        gltf.buffers().into_iter().for_each(|buffer| {
            match buffer.source() {
                gltf::buffer::Source::Bin => {

                },
                gltf::buffer::Source::Uri(uri) => {
                    let bin = std::fs::read(uri).expect("Failed to read uri");
                    buffer_data.push(bin);
                }
            }
        });


        let mut meshes = Vec::new();

        for scene in gltf.scenes() {
            for node in scene.nodes() {
                let mut vertices = Vec::new();
                let mut indices = Vec::new();

                let mesh = node.mesh().expect("Failed to find mesh in node!");

                let primitives = mesh.primitives();

                for primitive in primitives {
                    let reader = primitive.reader(|buffer| Some(&buffer_data[buffer.index()]));

                    if let Some(vertex_attribute) = reader.read_positions() {
                        vertex_attribute.for_each(|position| {
                            vertices.push(Vertex {
                                position: position,
                                normal: Default::default(),
                                uv: Default::default(),
                            })
                        })
                    }

                    if let Some(vertex_attribute) = reader.read_normals() {
                        let mut normal_index = 0;
                        vertex_attribute.for_each(|normal| {
                            vertices.push(Vertex {
                                position: vertices[normal_index].position,
                                normal: normal,
                                uv: vertices[normal_index].uv,
                            });

                            normal_index += 1;
                        })
                    }

                    if let Some(vertex_attribute) = reader.read_tex_coords(0).map(|v| v.into_f32()) {
                        let mut uv_index = 0;
                        vertex_attribute.for_each(|uv| {
                            vertices.push(Vertex {
                                position: vertices[uv_index].position,
                                normal: vertices[uv_index].normal,
                                uv: uv,
                            });

                            uv_index += 1;
                        })
                    }
    
                    if let Some(indices_raw) = reader.read_indices() {
                        indices.append(&mut indices_raw.into_u32().collect::<Vec<u32>>());
                    }
                }

                let vertex_buffer = self.ctx.create_buffer("mesh_vertex_buffer", bytemuck::cast_slice(&vertices), wgpu::BufferUsages::VERTEX);

                let index_buffer = self.ctx.create_buffer("mesh_vertex_buffer", bytemuck::cast_slice(&indices), wgpu::BufferUsages::INDEX);
        
                meshes.push(Mesh {
                    vertex_buffer,
                    index_buffer,
                    num_elements: indices.len() as u32
                });
            }
        }

        let model = cgmath::Matrix4::<f32>::identity();

        let translation = cgmath::Matrix4::from_translation(self.transform.unwrap().position.to_vec());
        let translated_model = model * translation;

        let scale = cgmath::Matrix4::from_nonuniform_scale(self.transform.unwrap().scale.x, self.transform.unwrap().scale.y, self.transform.unwrap().scale.z);
        let scaled_model = translated_model * scale;

        let cube_uniform = TransformUniform { 
            transform: scaled_model.into()
        }; 

        let cube_transform_buffer = self.ctx.create_buffer("model_transform_buffer", bytemuck::cast_slice(&[cube_uniform]), wgpu::BufferUsages::UNIFORM  | wgpu::BufferUsages::COPY_DST);

        let cube_transform_bind_group_layout = self.ctx.create_bind_group_layout(
            "cube_transform_bind_group_layout", 
            vec![
                BindGroupLayoutEntry {
                    binding: 0, 
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer { 
                        ty: wgpu::BufferBindingType::Uniform, 
                        has_dynamic_offset: false, 
                        min_binding_size: None
                    }
                },
            ]
        );

        let cube_transform_bind_group = self.ctx.create_bind_group(
            "cube_transform_bind_group",
            &cube_transform_bind_group_layout.gpu_bind_group_layout, 
            vec![
                BindGroupEntry {
                    binding: 0,
                    resource: cube_transform_buffer.gpu_buffer.as_entire_binding()
                },
            ]
        );        

        let cube_shader = self.ctx.create_shader("cube_shader", "./src/assets/shaders/player.wgsl");

        let camera_layout = self.ctx.get_bind_group_layout("camera_bind_group_layout");

        let cube_pipeline_layout = self.ctx.device.logical_device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("cube_pipeline_layout"),
            bind_group_layouts: &[
                &camera_layout.gpu_bind_group_layout,
                &cube_transform_bind_group_layout.gpu_bind_group_layout,
            ],
            push_constant_ranges: &[]
        }); 

        let cube_pipeline = self.ctx.create_render_pipeline(
            "cube_pipeline",
            cube_pipeline_layout,
            &cube_shader.shader,
            &[Vertex::buffer_layout()],
            Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba32Float,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL
            }),
            None,
            wgpu::PrimitiveTopology::TriangleList,
            wgpu::PolygonMode::Fill
        );

        Model {
            transform: self.transform.unwrap_or(Transform::default()),
            meshes
        }
    }
}
