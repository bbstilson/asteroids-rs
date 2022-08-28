use macroquad::prelude::*;

use crate::{
    consts::{ASTEROID_SIZE, ASTEROID_SPAWN_DISTANCE},
    utils::{screen_center, wrap_around},
};

pub struct Asteroid {
    pub position: Vec2,
    pub rotation: f32,
    pub velocity: Vec2,
    pub lives: u32,
}

impl Asteroid {
    pub fn spawn() -> Asteroid {
        let screen_center = screen_center();
        let random_angle = rand::gen_range(0.0, std::f32::consts::PI * 2.0);

        Asteroid {
            position: screen_center
                + Vec2::new(
                    random_angle.cos() * ASTEROID_SPAWN_DISTANCE,
                    random_angle.sin() * ASTEROID_SPAWN_DISTANCE,
                ),
            rotation: rand::gen_range(0.1_f32, 0.3_f32).to_radians(),
            velocity: Vec2::new(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0)).normalize(),
            lives: 3,
        }
    }

    pub fn radius(&self) -> f32 {
        self.lives as f32 * ASTEROID_SIZE
    }

    pub fn spawn_at(position: Vec2, lives: u32) -> Asteroid {
        Asteroid {
            position,
            lives,
            rotation: rand::gen_range(0.1_f32, 0.3_f32).to_radians(),
            velocity: Vec2::new(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0)).normalize(),
        }
    }

    pub fn draw(&self) {
        draw_circle_lines(self.position.x, self.position.y, self.radius(), 2.0, BROWN);
    }

    pub fn update(&mut self) {
        self.position += self.velocity;
        wrap_around(&mut self.position);
    }

    pub fn explode(&mut self) {
        self.lives -= 1;
    }
}
