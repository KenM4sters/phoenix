use std::rc::Rc;

use cgmath::{Angle, EuclideanSpace};
use winit::event::MouseScrollDelta;

use crate::graphics::{context::{BindGroup, BindGroupEntry, BindGroupLayoutEntry, Context}, renderer::TransformUniform};

pub enum CameraType {
    Perspective = 0,
}

pub struct PerspectiveCamera {
    position: cgmath::Point3<f32>,
    target: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,
    fovy: f32,
    aspect: f32, 
    near: f32, 
    far: f32, 
    view_matrix: cgmath::Matrix4<f32>,
    projection_matrix: cgmath::Matrix4<f32>,
    radius: f32,
    yaw: cgmath::Rad<f32>,
    pitch: cgmath::Rad<f32>,
    pub bind_group: Rc<BindGroup>
}

impl PerspectiveCamera {
    pub fn view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        self.projection_matrix * self.view_matrix
    }

    pub fn update_transform_matrices(&mut self) {
        self.view_matrix = cgmath::Matrix4::look_at_rh(self.position, self.target, self.up);
        
        self.projection_matrix = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.near, self.far);
    }

    pub fn translate(&mut self, translation: cgmath::Vector3<f32>) {
        self.position += translation;

        self.update_transform_matrices();
    }

    pub fn update_position(&mut self) {
        let x = self.radius * self.yaw.cos() * self.pitch.cos();
        let y = self.radius * self.pitch.sin();
        let z = self.radius * self.yaw.sin() * self.pitch.cos();

        self.position = cgmath::Point3::new(x, y, z) + self.target.to_vec();

        self.update_transform_matrices();
    }

    pub fn view_matrix(&self) -> cgmath::Matrix4<f32> {
        cgmath::Matrix4::look_at_rh(self.position, self.target, self.up)
    }

    pub fn process_mouse_movement(&mut self, delta_x: f32, delta_y: f32) {
        let sensitivity = 0.005;
        self.yaw += cgmath::Rad(delta_x * sensitivity);
        self.pitch += cgmath::Rad(-delta_y * sensitivity);

        // Clamp pitch
        let max_pitch = cgmath::Rad(1.57); // close to 90 degrees
        if self.pitch > max_pitch {
            self.pitch = max_pitch;
        }
        if self.pitch < -max_pitch {
            self.pitch = -max_pitch;
        }

        self.update_position();
    }

    pub fn process_mouse_scroll(&mut self, delta: &MouseScrollDelta) {
        match delta {
            MouseScrollDelta::LineDelta(x, y) => {
                self.radius += y * 0.01;
            },
            MouseScrollDelta::PixelDelta(position) => {
                self.radius += position.y as f32 * 0.01;
            }
        }

        self.update_position();
    }

    pub fn update_uniforms(&mut self, ctx: &Context) {
        let transform_uniform = TransformUniform {
            transform: self.view_projection_matrix().into()
        };

        let buffer = ctx.get_buffer("camera_transform_uniform_buffer");

        ctx.device.queue.write_buffer(&buffer.gpu_buffer, 0, bytemuck::cast_slice(&transform_uniform.transform));
    }
}

pub struct CameraBuilder<'a> {
    ctx: &'a mut Context,
    camera_type: Option<CameraType>,
    position: Option<cgmath::Point3<f32>>,
    target: Option<cgmath::Point3<f32>>,
    up: Option<cgmath::Vector3<f32>>,
    fovy: f32,
    aspect: f32, 
    near: f32, 
    far: f32,
    radius: f32,
    yaw: cgmath::Rad<f32>,
    pitch: cgmath::Rad<f32>,
}


impl<'a> CameraBuilder<'a> {
    pub fn new(ctx: &'a mut Context) -> Self {
        Self {
            ctx,
            camera_type: Some(CameraType::Perspective),
            position: Some((0.0, 0.0, 5.0).into()),
            target: Some((0.0, 0.0, 0.0).into()),
            up: Some((0.0, 1.0, 0.0).into()), 
            fovy: 45.0,
            aspect: 1.0,
            near: 0.1,
            far: 100.0,
            radius: 5.0,
            yaw: cgmath::Rad(0.0),
            pitch: cgmath::Rad(45.0),
        }
    }

    pub fn with_type(mut self, camera_type: CameraType) -> Self {
        self.camera_type = Some(camera_type);
        self
    }

    pub fn with_position(mut self, position: cgmath::Point3<f32>) -> Self {
        self.position = Some(position);
        self
    }

    pub fn with_up(mut self, up: cgmath::Vector3<f32>) -> Self {
        self.up = Some(up);
        self
    }

    pub fn with_target(mut self, target: cgmath::Point3<f32>) -> Self {
        self.target = Some(target);
        self
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    pub fn with_near(mut self, near: f32) -> Self {
        self.near = near;
        self
    }

    pub fn with_far(mut self, far: f32) -> Self {
        self.far = far;
        self
    }

    pub fn build(&mut self) -> PerspectiveCamera {
        match self.camera_type {
            _ => {
                let view_matrix = cgmath::Matrix4::look_at_rh(self.position.unwrap(), self.target.unwrap(), self.up.unwrap());
        
                let projection_matrix = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.near, self.far);

                let device = &self.ctx.device;

                let transform_uniform = TransformUniform { 
                    transform: (projection_matrix * view_matrix).into(), 
                };
                
                let transform_buffer = self.ctx.create_buffer("camera_transform_uniform_buffer", bytemuck::cast_slice(&transform_uniform.transform), wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST);

                let bind_group_layout = self.ctx.create_bind_group_layout(
                    "camera_bind_group_layout", 
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
                    "camera_transform_bind_group",
                    &bind_group_layout.gpu_bind_group_layout, 
                    vec![
                        BindGroupEntry {
                            binding: 0,
                            resource: transform_buffer.gpu_buffer.as_entire_binding()
                        },
                    ]
                );   
        
                PerspectiveCamera {
                    position: self.position.unwrap(),
                    target: self.target.unwrap(),
                    up: self.up.unwrap(),
                    fovy: self.fovy,
                    aspect: self.aspect,
                    near: self.near,
                    far: self.far,
                    view_matrix,
                    projection_matrix,
                    radius: self.radius,
                    yaw: self.yaw,
                    pitch: self.pitch,
                    bind_group
                }
            }
        }
    }
}



