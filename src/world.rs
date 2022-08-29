use macroquad::prelude::*;

use crate::{asteroid::Asteroid, laser::Laser, player::Player};

pub struct World {
    pub level: u32,
    pub score: u32,
    pub player: Player,
    pub lasers: Vec<Laser>,
    pub asteroids: Vec<Asteroid>,
}

impl World {
    pub fn new() -> World {
        let level = 1;
        let mut asteroids = vec![];
        World::spawn_asteroids(level, &mut asteroids);

        World {
            asteroids,
            level,
            score: 0,
            player: Player::new(),
            lasers: vec![],
        }
    }

    fn spawn_asteroids(n: u32, asteroids: &mut Vec<Asteroid>) {
        let asteroid_speed = n as f32;

        for _ in 0..n * 2 {
            let mut solved = false;
            while !solved {
                let a = Asteroid::spawn(asteroid_speed);
                if !asteroids.iter().any(|b| a.collides(b)) {
                    asteroids.push(a);
                    solved = true;
                }
            }
        }
    }

    pub fn next_level(self) -> World {
        let level = self.level + 1;
        let mut asteroids = vec![];
        World::spawn_asteroids(level, &mut asteroids);

        World {
            level,
            asteroids,
            score: self.score,
            player: Player::new(),
            lasers: vec![],
        }
    }

    pub fn increment_score(&mut self) {
        self.score += self.level * 10;
    }
}

impl Default for World {
    fn default() -> World {
        World::new()
    }
}
