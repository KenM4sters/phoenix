use std::{io::{BufReader, Cursor}, rc::Rc};

use crate::graphics::{context::{BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, Context, RenderPipeline}, renderer::TransformUniform, vertex_input::{Vertex, CUBE_INDICES, CUBE_VERTICES}};

use super::world::{Mesh, Transform};

use cgmath::{EuclideanSpace, SquareMatrix};
use gltf::Gltf;



pub struct Model {
    pub transform: Transform,
    pub meshes: Vec<Mesh>,
    pub bind_group_layout: Rc<BindGroupLayout>,
    pub bind_group: Rc<BindGroup>,
    pub pipeline: Rc<RenderPipeline>
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
    ctx: &'a mut Context,
    file_path: &'a str,
    transform: Transform,
}

impl<'a> ModelBuilder<'a> {
    pub fn new(ctx: &'a mut Context, file_path: &'a str) -> Self {
        Self {
            ctx,
            file_path,
            transform: Transform::default()
        }
    }

    pub fn with_position(mut self, position: cgmath::Point3<f32>) -> Self {
        self.transform.position = position;
        self
    }

    pub fn with_scale(mut self, scale: cgmath::Vector3<f32>) -> Self {
        self.transform.scale = scale;
        self
    }
    pub fn with_rotation(mut self, rotation: cgmath::Quaternion<f32>) -> Self {
        self.transform.rotation = rotation;
        self
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    } 

    pub fn build(&mut self) -> Model {
        // let gltf_string = std::fs::read_to_string(self.file_path).expect("Failed to read gltf string");
        // let gltf_cursor = Cursor::new(gltf_string);
        // let gltf_reader = BufReader::new(gltf_cursor);
        // let gltf = Gltf::from_reader(gltf_reader).expect("Failed to read gltf!");

        // let mut buffer_data = Vec::new();
        // gltf.buffers().into_iter().for_each(|buffer| {
        //     match buffer.source() {
        //         gltf::buffer::Source::Bin => {

        //         },
        //         gltf::buffer::Source::Uri(uri) => {
        //             let bin = std::fs::read(uri).expect("Failed to read uri");
        //             buffer_data.push(bin);
        //         }
        //     }
        // });


        let mut meshes = Vec::new();

        // for scene in gltf.scenes() {
        //     for node in scene.nodes() {
        //         let mut vertices = Vec::new();
        //         let mut indices = Vec::new();

        //         let mesh = node.mesh().expect("Failed to find mesh in node!");

        //         let primitives = mesh.primitives();

        //         for primitive in primitives {
        //             let reader = primitive.reader(|buffer| Some(&buffer_data[buffer.index()]));

        //             if let Some(vertex_attribute) = reader.read_positions() {
        //                 vertex_attribute.for_each(|position| {
        //                     vertices.push(Vertex {
        //                         position: position,
        //                         normal: Default::default(),
        //                         uv: Default::default(),
        //                     })
        //                 })
        //             }

        //             if let Some(vertex_attribute) = reader.read_normals() {
        //                 let mut normal_index = 0;
        //                 vertex_attribute.for_each(|normal| {
        //                     vertices.push(Vertex {
        //                         position: vertices[normal_index].position,
        //                         normal: normal,
        //                         uv: vertices[normal_index].uv,
        //                     });

        //                     normal_index += 1;
        //                 })
        //             }

        //             if let Some(vertex_attribute) = reader.read_tex_coords(0).map(|v| v.into_f32()) {
        //                 let mut uv_index = 0;
        //                 vertex_attribute.for_each(|uv| {
        //                     vertices.push(Vertex {
        //                         position: vertices[uv_index].position,
        //                         normal: vertices[uv_index].normal,
        //                         uv: uv,
        //                     });

        //                     uv_index += 1;
        //                 })
        //             }
    
        //             if let Some(indices_raw) = reader.read_indices() {
        //                 indices.append(&mut indices_raw.into_u32().collect::<Vec<u32>>());
        //             }
        //         }

        //     }
        // }

        let vertex_buffer = self.ctx.create_buffer("mesh_vertex_buffer", bytemuck::cast_slice(&CUBE_VERTICES), wgpu::BufferUsages::VERTEX);

        let index_buffer = self.ctx.create_buffer("mesh_vertex_buffer", bytemuck::cast_slice(&CUBE_INDICES), wgpu::BufferUsages::INDEX);

        meshes.push(Mesh {
            vertex_buffer,
            index_buffer,
            num_elements: CUBE_INDICES.len() as u32
        });

        let model = cgmath::Matrix4::<f32>::identity();

        let translation = cgmath::Matrix4::from_translation(self.transform.position.to_vec());
        let translated_model = model * translation;

        let scale = cgmath::Matrix4::from_nonuniform_scale(self.transform.scale.x, self.transform.scale.y, self.transform.scale.z);
        let scaled_model = translated_model * scale;

        let cube_uniform = TransformUniform { 
            transform: scaled_model.into()
        }; 

        let cube_transform_buffer = self.ctx.create_buffer("model_transform_buffer", bytemuck::cast_slice(&[cube_uniform]), wgpu::BufferUsages::UNIFORM  | wgpu::BufferUsages::COPY_DST);

        let bind_group_layout = self.ctx.create_bind_group_layout(
            "cube_transform_bind_group_layout", 
            vec![
                BindGroupLayoutEntry {
                    binding: 0, 
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer { 
                        ty: wgpu::BufferBindingType::Uniform, 
                        has_dynamic_offset: false, 
                        min_binding_size: None
                    }
                },
            ]
        );

        let bind_group = self.ctx.create_bind_group(
            "cube_transform_bind_group",
            &bind_group_layout.gpu_bind_group_layout, 
            vec![
                BindGroupEntry {
                    binding: 0,
                    resource: cube_transform_buffer.gpu_buffer.as_entire_binding()
                },
            ]
        );        

        let cube_shader = self.ctx.create_shader("cube_shader", "./src/assets/shaders/player.wgsl");

        let camera_layout = self.ctx.get_bind_group_layout("camera_bind_group_layout");

        let pipeline_layout = self.ctx.device.logical_device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("cube_pipeline_layout"),
            bind_group_layouts: &[
                &camera_layout.gpu_bind_group_layout,
                &bind_group_layout.gpu_bind_group_layout,
            ],
            push_constant_ranges: &[]
        }); 

        let pipeline = self.ctx.create_render_pipeline(
            "cube_pipeline",
            pipeline_layout,
            &cube_shader.shader,
            &[Vertex::buffer_layout()],
            Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL
            }),
            Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_compare: wgpu::CompareFunction::Less,
                depth_write_enabled: true,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            wgpu::PrimitiveTopology::TriangleList,
            wgpu::PolygonMode::Fill
        );

        Model {
            transform: self.transform,
            meshes,
            bind_group_layout,
            bind_group,
            pipeline
        }
    }
}
