use super::game::Game;



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
        movement_speed: f32
    }
}
