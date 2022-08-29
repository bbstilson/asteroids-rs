use macroquad::prelude::*;

use crate::{
    asteroid::Asteroid,
    consts::*,
    utils::{screen_center, wrap_around},
};

type Vertices = (Vec2, Vec2, Vec2);

#[derive(Clone)]
pub struct Player {
    pub position: Vec2,
    pub rotation: f32,
    pub velocity: Vec2,
    pub thrusting: bool,
    pub lives: u32,
}

impl Player {
    pub fn new() -> Player {
        let position = screen_center();
        let rotation = 0.0_f32.to_radians();

        Player {
            position,
            rotation,
            velocity: Vec2::ZERO,
            thrusting: false,
            lives: 3,
        }
    }

    fn get_vertices(position: &Vec2, rotation: f32) -> Vertices {
        // screen draws from top left, so we need to flip 180 degrees
        // 0 deg == 180 deg
        // 225 deg == 45 deg
        // 315 deg == 135 deg
        (
            Vec2::new(
                position.x + rotation.sin() * SHIP_SIZE,
                position.y - rotation.cos() * SHIP_SIZE,
            ),
            Vec2::new(
                position.x + (rotation + ANGLE_135).cos() * SHIP_SIZE,
                position.y + (rotation + ANGLE_135).sin() * SHIP_SIZE,
            ),
            Vec2::new(
                position.x - (rotation + ANGLE_225).cos() * SHIP_SIZE,
                position.y - (rotation + ANGLE_225).sin() * SHIP_SIZE,
            ),
        )
    }

    pub fn draw(&self) {
        let (a, b, c) = Player::get_vertices(&self.position, self.rotation);

        draw_triangle_lines(a, b, c, 2.0, BLACK);

        // debug circle
        // draw_circle_lines(self.position.x, self.position.y, SHIP_SIZE, 1.0, RED);

        if self.thrusting {
            let d = Vec2::new(
                self.position.x + (self.rotation + ANGLE_180).sin() * (SHIP_SIZE + FLAME_SIZE),
                self.position.y - (self.rotation + ANGLE_180).cos() * (SHIP_SIZE + FLAME_SIZE),
            );
            draw_triangle(b, c, d, Color::new(0.698, 0.1328, 0.1328, 1.0));
        }
    }

    pub fn update(&mut self) {
        self.position += self.velocity;
        wrap_around(&mut self.position);
    }

    pub fn collides(&self, asteroid: &Asteroid) -> bool {
        self.position.distance(asteroid.position) - (SHIP_SIZE * 0.8) <= asteroid.radius()
    }
}

impl Default for Player {
    fn default() -> Player {
        Player::new()
    }
}
