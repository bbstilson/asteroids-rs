use macroquad::prelude::*;

use crate::{asteroid::Asteroid, consts::*, utils::wrap_around};

pub struct Player {
    pub position: Vec2,
    pub rotation: f32,
    pub velocity: Vec2,
    pub thrusting: bool,
}

impl Player {
    pub fn draw(&self) {
        // screen draws from top left, so we need to flip 180 degrees
        // 0 deg == 180 deg
        // 225 deg == 45 deg
        // 315 deg == 135 deg

        let a = Vec2::new(
            self.position.x + self.rotation.sin() * PLAYER_SIZE,
            self.position.y - self.rotation.cos() * PLAYER_SIZE,
        );

        let b = Vec2::new(
            self.position.x + (self.rotation + ANGLE_135).cos() * PLAYER_SIZE,
            self.position.y + (self.rotation + ANGLE_135).sin() * PLAYER_SIZE,
        );

        let c = Vec2::new(
            self.position.x - (self.rotation + ANGLE_225).cos() * PLAYER_SIZE,
            self.position.y - (self.rotation + ANGLE_225).sin() * PLAYER_SIZE,
        );

        draw_triangle_lines(a, b, c, 2.0, BLACK);

        // debug circle
        draw_circle_lines(
            self.position.x,
            self.position.y,
            PLAYER_SIZE * 0.8,
            1.0,
            BLACK,
        );

        if self.thrusting {
            let d = Vec2::new(
                self.position.x + (self.rotation + ANGLE_180).sin() * (PLAYER_SIZE + FLAME_SIZE),
                self.position.y - (self.rotation + ANGLE_180).cos() * (PLAYER_SIZE + FLAME_SIZE),
            );
            draw_triangle(b, c, d, Color::new(0.698, 0.1328, 0.1328, 1.0));
        }
    }

    pub fn update(&mut self) {
        self.position += self.velocity;
        wrap_around(&mut self.position);
    }

    pub fn collides(&self, asteroid: &Asteroid) -> bool {
        false
    }
}
