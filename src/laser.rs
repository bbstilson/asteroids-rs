use macroquad::prelude::*;

use crate::{asteroid::Asteroid, consts::LASER_SIZE};

pub struct Laser {
    pub position: Vec2,
    pub velocity: Vec2,
    pub collided: bool,
}

impl Laser {
    pub fn draw(&self) {
        let v2 = self.position - self.velocity;
        draw_line(
            self.position.x,
            self.position.y,
            v2.x,
            v2.y,
            LASER_SIZE,
            BLACK,
        );
    }

    pub fn on_screen(&self) -> bool {
        let x = self.position.x;
        let y = self.position.y;
        let width = screen_width();
        let height = screen_height();
        x < width && x > 0.0 && y < height && y > 0.0
    }

    pub fn update(&mut self) {
        self.position += self.velocity
    }

    pub fn collides(&self, asteroid: &Asteroid) -> bool {
        self.position.distance(asteroid.position) <= asteroid.radius()
    }
}

#[cfg(test)]
mod tests {
    use macroquad::prelude::Vec2;

    use crate::asteroid::Asteroid;

    use super::Laser;

    #[test]
    fn test_collides() {
        let laser = Laser {
            position: Vec2::ZERO,
            velocity: Vec2::ZERO,
            collided: false,
        };
        let asteroid = Asteroid {
            position: Vec2::ZERO,
            rotation: 0.0,
            velocity: Vec2::ZERO,
            lives: 1,
        };

        assert!(laser.collides(&asteroid))
    }
}
