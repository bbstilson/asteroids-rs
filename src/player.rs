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
                position.x + rotation.sin() * PLAYER_SIZE,
                position.y - rotation.cos() * PLAYER_SIZE,
            ),
            Vec2::new(
                position.x + (rotation + ANGLE_135).cos() * PLAYER_SIZE,
                position.y + (rotation + ANGLE_135).sin() * PLAYER_SIZE,
            ),
            Vec2::new(
                position.x - (rotation + ANGLE_225).cos() * PLAYER_SIZE,
                position.y - (rotation + ANGLE_225).sin() * PLAYER_SIZE,
            ),
        )
    }

    pub fn draw(&self) {
        let (a, b, c) = Player::get_vertices(&self.position, self.rotation);

        draw_triangle_lines(a, b, c, 2.0, BLACK);

        // debug circle
        // draw_circle_lines(self.position.x, self.position.y, PLAYER_SIZE, 1.0, RED);

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

    // Shamefully stolen from https://2dengine.com/?p=intersections#Circle_vs_triangle
    pub fn collides(&self, asteroid: &Asteroid) -> bool {
        if self.collide_check(asteroid) {
            let q = Player::point_on_triangle(
                asteroid.position,
                Player::get_vertices(&self.position, self.rotation),
            );
            Player::point_in_circle(q, asteroid.position, asteroid.radius())
        } else {
            false
        }
    }

    fn point_on_triangle(p: Vec2, (a, b, c): Vertices) -> Vec2 {
        let (abx, aby) = (b.x - a.x, b.y - a.y);
        let (acx, acy) = (c.x - a.x, c.y - a.y);
        let (apx, apy) = (p.x - a.x, p.y - a.y);

        // vertex region outside a
        let d1 = Vec2::new(abx, aby).dot(Vec2::new(apx, apy));
        let d2 = Vec2::new(acx, acy).dot(Vec2::new(apx, apy));

        if d1 <= 0.0 && d2 <= 0.0 {
            return a;
        }

        // vertex region outside b
        let (bpx, bpy) = (p.x - b.x, p.y - b.y);
        let d3 = Vec2::new(abx, aby).dot(Vec2::new(bpx, bpy));
        let d4 = Vec2::new(acx, acy).dot(Vec2::new(bpx, bpy));
        if d3 >= 0.0 && d4 <= d3 {
            return b;
        }

        // edge region ab
        if d1 >= 0.0 && d3 <= 0.0 && d1 * d4 - d3 * d2 <= 0.0 {
            let v = d1 / (d1 - d3);
            return Vec2::new(a.x + abx * v, a.y + aby * v);
        }

        // vertex region outside c
        let (cpx, cpy) = (p.x - c.x, p.y - c.y);
        let d5 = Vec2::new(abx, aby).dot(Vec2::new(cpx, cpy));
        let d6 = Vec2::new(acx, acy).dot(Vec2::new(cpx, cpy));
        if d6 >= 0.0 && d5 <= d6 {
            return c;
        }
        // edge region ac
        if d2 >= 0.0 && d6 <= 0.0 && d5 * d2 - d1 * d6 <= 0.0 {
            let w = d2 / (d2 - d6);
            return Vec2::new(a.x + acx * w, a.y + acy * w);
        }
        // edge region bc
        if d3 * d6 - d5 * d4 <= 0.0 {
            let d43 = d4 - d3;
            let d56 = d5 - d6;
            if d43 >= 0.0 && d56 >= 0.0 {
                let w = d43 / (d43 + d56);
                return Vec2::new(b.x + (c.x - b.x) * w, b.y + (c.y - b.y) * w);
            }
        }

        // inside face region
        p
    }

    fn point_in_circle(p: Vec2, c: Vec2, r: f32) -> bool {
        let (dx, dy) = (p.x - c.x, p.y - c.y);
        dx * dx + dy * dy <= r * r
    }
    fn collide_check(&self, asteroid: &Asteroid) -> bool {
        // faster check to see if the asteroid is inside a sphere near the ship
        self.position.distance(asteroid.position) <= PLAYER_SIZE
    }
}

impl Default for Player {
    fn default() -> Player {
        Player::new()
    }
}
