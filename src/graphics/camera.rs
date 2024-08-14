use cgmath::{Point3, Vector3};

pub struct OrthographicCamera {
    position: cgmath::Point3<f32>,
    target: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32,
    view_matrix: cgmath::Matrix4<f32>,
    projection_matrix: cgmath::Matrix4<f32>
}

impl OrthographicCamera {
    pub fn new(position: cgmath::Point3<f32>, target: Point3<f32>, up: Vector3<f32>, left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32,) -> Self {

        let view_matrix = cgmath::Matrix4::look_at_rh(position, target, up);
        let projection_matrix = cgmath::ortho(left, right, bottom, top, near, far);

        Self {
            position,
            target,
            up,
            left,
            right,
            bottom,
            top,
            near,
            far,
            view_matrix,
            projection_matrix
        }
    }

    pub fn resize(&mut self, left: f32, right: f32, bottom: f32, top: f32) {
        self.left = left;
        self.right = right;
        self.bottom = bottom;
        self.top = top;

        self.projection_matrix = cgmath::ortho(self.left, self.right, self.bottom, self.top, self.near, self.far);
    }
}