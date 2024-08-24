

use std::rc::Rc;

use cgmath::{EuclideanSpace, SquareMatrix};
use winit::event::*;

use crate::graphics::vertex_input::{Vertex, CUBE_INDICES, CUBE_VERTICES};

use super::camera::*;

#[derive(Debug)]
pub struct Transform {
    pub position: cgmath::Point3<f32>,
    pub scale: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: (0.0, 0.0, 0.0).into(),
            scale: (1.0, 1.0, 1.0).into(),
            rotation: cgmath::Quaternion { v: (0.0, 0.0, 0.0).into(), s: 0.0 },
        }
    }
}

pub struct Cube {
    transform: Transform
}

impl Cube {
    pub fn new(transform: Transform) -> Self {
        Self {
            transform
        }
    }   

    pub fn model_matrix(&self) -> cgmath::Matrix4<f32> {
        let model = cgmath::Matrix4::<f32>::identity();

        let translation = cgmath::Matrix4::from_translation(self.transform.position.to_vec());
        let translated_model = model * translation;

        let scale = cgmath::Matrix4::from_nonuniform_scale(self.transform.scale.x, self.transform.scale.y, self.transform.scale.z);
        let scaled_model = translated_model * scale;

        scaled_model
    }
}


// World
pub struct World {
    pub camera: PerspectiveCamera,
    last_mouse_pos: Option<(f32, f32)>,
}

impl World {
    pub fn new() -> Self {

        let camera = CameraBuilder::new()
            .with_position((0.0, 0.0, 5.0).into())
            .with_target((0.0, 0.0, 0.0).into())
            .with_type(CameraType::Perspective)
            .with_radius(10.0)
            .build();

        let last_mouse_pos = None;

        Self {
            camera,
            last_mouse_pos
        }
    }

    pub fn update(&mut self) {

    }

    pub fn handle_window_input(&mut self, event: &Event<()>) {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::MouseInput { button, state, .. } => {
                    if *button == MouseButton::Left {
                        if *state == ElementState::Pressed {
                            self.last_mouse_pos = Some((0.0, 0.0));
                        } else {
                            self.last_mouse_pos = None;
                        }
                    }
                }
                _ => {}
            },
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseMotion { delta } => {
                    if let Some((_, _)) = self.last_mouse_pos {
                        self.camera.process_mouse_movement(delta.0 as f32, -delta.1 as f32);
                    }
                },
                DeviceEvent::MouseWheel { delta } => {
                    self.camera.process_mouse_scroll(delta);
                }
                _ => {}
            },
            _ => {}
        }
    }
}
