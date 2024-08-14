use std::collections::HashMap;

use super::shader::{ShaderProgram};

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct PipelineMapKey {
    vertex_path: String,
    fragment_path: String
}

pub struct PipelineManager {
    pipeline_map: HashMap<PipelineMapKey, wgpu::RenderPipeline>
}

impl PipelineManager {
    pub fn new() -> Self {
        let pipeline_map = HashMap::new();

        Self {
            pipeline_map
        }
    }

    pub fn build_pipeline() {
        
    }

    /// # TODO
    /// 
    pub fn get_valid_pipeline(&mut self, key: PipelineMapKey) -> Option<&wgpu::RenderPipeline> {
        let pipeline = self.pipeline_map.get(&key);

        match pipeline {
            Some(pipeline) => {
                Some(pipeline)
            }
            _ => None
        }
    }
}
