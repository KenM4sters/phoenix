use std::rc::Rc;

use winit::event::*;

use crate::graphics::vertex_input::Vertex;

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

pub struct Mesh {
    vertices: Rc<Vec<Vertex>>,
    indices: Rc<Vec<u32>>,
    num_elements: u32,
}




// World
pub struct World {
}

impl World {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn update(&mut self) {

    }

    pub fn handle_window_input(&mut self, event: &Event<()>) {
    }
}
