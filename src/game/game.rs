use winit::{event::{ElementState, KeyEvent, WindowEvent}, event_loop::EventLoopWindowTarget, keyboard::{KeyCode, PhysicalKey}};

use super::controller::Controller;

use cgmath::{Point3, Vector3};


// Camera

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

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
        // let projection_matrix = cgmath::perspective(cgmath::Deg(45.0), 1.0, near, far);
        
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

    pub fn view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        OPENGL_TO_WGPU_MATRIX * self.projection_matrix * self.view_matrix
    }

    pub fn resize(&mut self, left: f32, right: f32, bottom: f32, top: f32) {
        self.left = left;
        self.right = right;
        self.bottom = bottom;
        self.top = top;

        self.projection_matrix = cgmath::ortho(self.left, self.right, self.bottom, self.top, self.near, self.far);
    }
}






// Sprites

#[derive(Debug)]
pub enum Weapon {
    Narrow {
        power: u32,
        color: cgmath::Point3<f32>,
        speed: u32
    },
    Spread {
        power: u32,
        color: cgmath::Point3<f32>,
        speed: u32
    },
}


#[derive(Debug)]
pub enum GameSprite {
    Player {
        position: cgmath::Point3<f32>,
        rotation: cgmath::Quaternion<f32>,
        size: cgmath::Point3<f32>,
        health_points: u32,
        movement_speed: f32,
        weapon: Weapon,
        lives: u32
    },
    Enemy {
        position: cgmath::Point3<f32>,
        rotation: cgmath::Quaternion<f32>,
        size: cgmath::Point3<f32>,
        health_points: u32,
        movement_speed: f32,
        weapon: Weapon,
        lives: u32,
        ai_component: Option<()>
    },
    Bullet {
        position: cgmath::Point3<f32>,
        rotation: cgmath::Quaternion<f32>,
        size: cgmath::Point3<f32>,
        movement_speed: f32,
    }
}






// Game

pub struct Game {
    sprites: Vec<GameSprite>,
    controllers: Vec<Controller>,
}

impl Game {
    pub fn new() -> Self {
        let mut sprites = vec![];

        let mut player: GameSprite = GameSprite::Player {
            position: cgmath::Point3 { x: 0.0, y: 0.0, z: 0.0 },
            rotation: cgmath::Quaternion { v: cgmath::Vector3 { x: 0.0, y: 0.0, z: 0.0 }, s: 0.0 },
            size: cgmath::Point3 { x: 1.0, y: 1.0, z: 1.0 }, 
            health_points: 10, 
            movement_speed: 0.1, 
            weapon: Weapon::Spread {
                power: 1,
                color: cgmath::Point3 { x: 1.0, y: 1.0, z: 1.0},
                speed: 1,
            }, 
            lives: 5,  
        };

        sprites.push(player);

        let player_controller = Controller::new(move |event, control_flow| {
            match event {
                WindowEvent::KeyboardInput { 
                    event: KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::KeyW),
                        ..
                    },
                    ..
                } => {
                },
                _ => {}
            }
        });

        let mut controllers = vec![];

        controllers.push(player_controller);

        Self {
            sprites,
            controllers,
        }
    }

    pub fn for_each_sprite(&self, sprite_callback: fn(sprite: &GameSprite)) {
        self.sprites.iter().for_each(|sprite| {
            sprite_callback(&sprite);
        })
    }

    pub fn handle_window_input(&mut self, event: &WindowEvent, control_flow: &EventLoopWindowTarget<()>) {
        self.controllers.iter_mut().for_each(move |controller| {
            controller.call(event, control_flow);
        })
    }   

    pub fn update(&mut self) {

    }
}
