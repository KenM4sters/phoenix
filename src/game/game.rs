use std::u32;

use wgpu::naga::back::Level;
use winit::{event::{ElementState, KeyEvent, WindowEvent}, event_loop::{EventLoop, EventLoopWindowTarget}, keyboard::{Key, KeyCode, PhysicalKey}};

use super::{controller::Controller, sprite::{GameSprite, Weapon}};


pub struct Game<'a> {
    sprites: Vec<GameSprite<'a>>,
    controllers: Vec<Controller>,
    levels: Vec<Level>,
}

impl<'a> Game<'a> {
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
            renderable: None 
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

        let mut levels = vec![];

        Self {
            sprites,
            controllers,
            levels
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
