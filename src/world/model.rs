use std::io::{BufReader, Cursor};

use crate::graphics::vertex_input::Vertex;

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
    file_path: &'a str,
    transform: Option<Transform>,
}

impl<'a> ModelBuilder<'a> {
    pub fn new(file_path: &str) -> Self {
        Self {
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

                let vertices = Rc::new(vertices);

                let indices = Rc::new(indices);
        
                meshes.push(Mesh {
                    vertices,
                    indices,
                    num_elements: indices.len() as u32
                });
            }
        }

        Model {
            transform: self.transform.unwrap_or(Transform::default()),
            meshes
        }
    }
}
