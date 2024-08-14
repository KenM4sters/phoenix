use super::{camera::OrthographicCamera, pipeline::PipelineManager};

pub struct Renderer {
    pipeline_manager: PipelineManager,
    size: winit::dpi::PhysicalSize<u32>
}

impl Renderer {
    pub fn new(size: winit::dpi::PhysicalSize<u32>) -> Self {

        let pipeline_manager = PipelineManager::new();

        Self {
            pipeline_manager,
            size
        }
    }

    pub fn draw(&mut self, camera: &OrthographicCamera) {
        
    }
}