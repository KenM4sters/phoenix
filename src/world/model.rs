use std::io::{BufReader, Cursor};

use crate::graphics::{context::Context, vertex_input::Vertex};

use super::world::Transform;

use gltf::Gltf;




pub struct Material {
    name: String,
    
}

pub struct Mesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    material: Material
}


pub struct Model {
    transform: Transform
}

impl Model {
    pub fn new(file_path: &str, ctx: &Context) {
        let gltf_string = std::fs::read_to_string(file_path).expect("Failed to read gltf string");
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

        for scene in gltf.scenes() {
            for node in scene.nodes() {
                let mesh = node.mesh().expect("Failed to find mesh in node!");
                let primitives = mesh.primitives();

                let mut vertices = Vec::new();
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
                }
            }
        }


    }
}

pub struct ModelBuilder {
    file_path: Option<String>,
    transform: Option<Transform>    
}

impl ModelBuilder {
    pub fn new() -> Self {
        Self {
            file_path: None,
            transform: None
        }
    }

    pub fn with_file_path(mut self, file_path: String) -> Self {
        self.file_path = Some(file_path);
        self
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
        Model {
            transform: self.transform.unwrap()
        }
    }
}
