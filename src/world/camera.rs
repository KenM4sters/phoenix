use cgmath::{AbsDiffEq, Point3, SquareMatrix, Vector3};


pub enum CameraType {
    Perspective = 0,
    Orthographic = 1,
}

pub struct PerspectiveCamera {
    position: cgmath::Point3<f32>,
    target: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>, 
    view_matrix: cgmath::Matrix4<f32>,
    projection_matrix: cgmath::Matrix4<f32>
}

impl PerspectiveCamera {
    pub fn view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        self.projection_matrix * self.view_matrix
    }
}

pub struct OrthographicCamera {

}


pub struct CameraBuilder {
    camera_type: Option<CameraType>,
    position: Option<cgmath::Point3<f32>>,
    target: Option<cgmath::Point3<f32>>,
    up: Option<cgmath::Vector3<f32>>,
    fovy: f32,
    aspect: f32, 
    near: f32, 
    far: f32, 
    view_matrix: cgmath::Matrix4<f32>,
    projection_matrix: cgmath::Matrix4<f32>,
}


impl CameraBuilder {
    pub fn new() -> Self {
        Self {
            camera_type: Some(CameraType::Perspective),
            position: Some((0.0, 0.0, 5.0).into()),
            target: Some((0.0, 0.0, 0.0).into()),
            up: Some((0.0, 1.0, 0.0).into()), 
            fovy: 45.0,
            aspect: 1.0,
            near: 0.1,
            far: 100.0,
            view_matrix: cgmath::Matrix4::identity(),
            projection_matrix: cgmath::Matrix4::identity(),   
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

    pub fn with_near(mut self, near: f32) -> Self {
        self.near = near;
        self
    }

    pub fn with_far(mut self, far: f32) -> Self {
        self.far = far;
        self
    }

    pub fn build(&self) -> PerspectiveCamera {
        match self.camera_type {
            _ => {
                let view_matrix = cgmath::Matrix4::look_at_rh(self.position.unwrap(), self.target.unwrap(), self.up.unwrap());
        
                let projection_matrix = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.near, self.far);
        
                PerspectiveCamera {
                    position: self.position.unwrap(),
                    target: self.target.unwrap(),
                    up: self.up.unwrap(),
                    view_matrix,
                    projection_matrix
                }
            }
        }
    }
}



