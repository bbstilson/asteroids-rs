use macroquad::prelude::*;

use crate::{
    consts::{ASTEROID_SIZE, ASTEROID_SPAWN_DISTANCE},
    utils::{screen_center, wrap_around},
};

#[derive(Debug, Clone)]
pub struct Asteroid {
    pub position: Vec2,
    pub rotation: f32,
    pub velocity: Vec2,
    pub lives: u32,
}

impl Asteroid {
    pub fn spawn(level: f32) -> Asteroid {
        let screen_center = screen_center();
        let player_avoidance = Rect::new(
            screen_center.x - ASTEROID_SPAWN_DISTANCE,
            screen_center.y - ASTEROID_SPAWN_DISTANCE,
            screen_center.x + ASTEROID_SPAWN_DISTANCE,
            screen_center.y + ASTEROID_SPAWN_DISTANCE,
        );

        let mut position = Vec2::ZERO;
        let mut solved = false;
        while !solved {
            let rand_x = rand::gen_range(0.0, screen_width());
            let rand_y = rand::gen_range(0.0, screen_height());
            let rand_point = Vec2::new(rand_x, rand_y);
            if !player_avoidance.contains(rand_point) {
                solved = true;
                position = rand_point;
            }
        }

        Asteroid {
            position,
            rotation: rand::gen_range(0.1_f32, 0.3_f32).to_radians(),
            velocity: Vec2::new(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0)).normalize()
                * rand::gen_range(1.0, level),
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

    pub fn collides(&self, other: &Asteroid) -> bool {
        self.position.distance(other.position) <= self.radius() + other.radius()
    }
}
