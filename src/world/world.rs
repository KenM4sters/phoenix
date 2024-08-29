use std::rc::Rc;

use winit::event::*;

use crate::graphics::context::{Buffer, Context};

use super::model::{Model, ModelBuilder};

#[derive(Debug, Clone, Copy)]
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
    pub vertex_buffer: Rc<Buffer>,
    pub index_buffer: Rc<Buffer>,
    pub num_elements: u32,
}




// World
pub struct World {
    pub models: Vec<Model>
}

impl World {
    pub fn new(ctx: &mut Context) -> Self {
        let mut models = vec![];

        let model = ModelBuilder::new(ctx, "")
            .build();

        models.push(model);

        Self {
            models
        }
    }

    pub fn update(&mut self) {
        todo!()
    }   

    pub fn handle_window_input(&mut self, event: &Event<()>) {
        todo!()
    }
}
