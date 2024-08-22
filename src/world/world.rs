

use core::slice::SlicePattern;

use cgmath::num_traits::zero;
use winit::{event::{ElementState, KeyEvent, WindowEvent}, event_loop::EventLoopWindowTarget, keyboard::{KeyCode, PhysicalKey}};

use crate::graphics::vertex_input::{Vertex, INDICES, VERTICES};

use super::{camera::*, controller::Controller};



pub struct Transform {
    position: cgmath::Point3<f32>,
    scale: cgmath::Vector3<f32>,
    rotation: cgmath::Quaternion<f32>,
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
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
    transform: Transform
}

// World
pub struct World {
    controllers: Vec<Controller>,
    camera: PerspectiveCamera,
    cube: Cube
}

impl World {
    pub fn new() -> Self {

        let controllers = vec![];

        let camera = CameraBuilder::new()
            .with_position((0.0, 0.0, 5.0).into())
            .with_target((0.0, 0.0, 0.0).into())
            .with_type(CameraType::Perspective)
            .build();

        let cube = Cube {
            vertices: VERTICES.to_vec(),
            indices: INDICES.to_vec(),
            transform: Transform::default()
        };

        Self {
            controllers,
            camera,
            cube
        }
    }

    pub fn update(&mut self) {

    }

    pub fn handle_window_input(&mut self, event: &WindowEvent, control_flow: &EventLoopWindowTarget<()>) {
        self.controllers.iter().for_each(|controller| {
            controller.call(event, control_flow);
        }); 
    }
}
