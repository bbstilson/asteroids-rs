use macroquad::{
    audio::{self, PlaySoundParams},
    prelude::*,
    time,
};

use asteroid::Asteroid;
use consts::*;
use laser::Laser;
use player::Player;
use utils::screen_center;

pub mod asteroid;
pub mod consts;
pub mod laser;
pub mod player;
pub mod utils;

pub enum GameState {
    MainMenu,
    Play,
    Won,
    GameOver,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Asteroids".to_owned(),
        window_width: 1920,
        window_height: 1080,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");

    let laser_fire = audio::load_sound("laser.wav").await.unwrap();
    let explosion = audio::load_sound("explosion.wav").await.unwrap();
    let engine = audio::load_sound("engine.wav").await.unwrap();

    audio::set_sound_volume(laser_fire, 1.0);
    audio::set_sound_volume(explosion, 1.0);

    // let mut level: u32 = 1;
    let mut state = GameState::Play;

    // let mut player_lives: u32 = 3;
    let mut player = Player {
        position: screen_center(),
        rotation: 0.0_f32.to_radians(),
        velocity: Vec2::ZERO,
        thrusting: false,
    };

    let mut lasers: Vec<Laser> = vec![];
    let mut asteroids: Vec<Asteroid> = vec![];
    for _ in 0..1 {
        asteroids.push(Asteroid::spawn())
    }

    loop {
        clear_background(LIGHTGRAY);

        match state {
            GameState::MainMenu => {}
            GameState::Play => {
                if is_key_down(KeyCode::Right) {
                    player.rotation += PLAYER_ROTATION_SPEED;
                } else if is_key_down(KeyCode::Left) {
                    player.rotation -= PLAYER_ROTATION_SPEED;
                }

                if is_key_down(KeyCode::Up) {
                    audio::play_sound(
                        engine,
                        PlaySoundParams {
                            looped: true,
                            volume: 1.0,
                        },
                    );
                    player.velocity += Vec2::new(
                        player.rotation.sin() * SHIP_THRUST,
                        -player.rotation.cos() * SHIP_THRUST,
                    );
                    player.thrusting = true;
                } else {
                    audio::stop_sound(engine);
                    player.thrusting = false;
                }

                if is_key_pressed(KeyCode::Space) {
                    audio::play_sound_once(laser_fire);
                    lasers.push(Laser {
                        position: player.position, // TODO fire from ship nose
                        velocity: Vec2::new(
                            player.rotation.sin() * LASER_SPEED,
                            -player.rotation.cos() * LASER_SPEED,
                        ),
                        collided: false,
                    });
                }

                // run updates
                for laser in lasers.iter_mut() {
                    laser.update();
                }

                for asteroid in asteroids.iter_mut() {
                    asteroid.update();
                }

                player.update();

                // check collisions between lasers and asteroids
                let mut asteroids_to_spawn = vec![];
                for laser in lasers.iter_mut() {
                    for asteroid in asteroids.iter_mut() {
                        if laser.collides(asteroid) {
                            audio::play_sound_once(explosion);
                            asteroid.explode();
                            asteroids_to_spawn
                                .push(Asteroid::spawn_at(asteroid.position, asteroid.lives));
                            laser.collided = true;
                        }
                    }
                }

                for asteroid in asteroids_to_spawn {
                    asteroids.push(asteroid);
                }

                // check collisions between asteroids and player
                for asteroid in asteroids.iter_mut() {
                    if player.collides(asteroid) {
                        state = GameState::GameOver;
                    }
                }

                // draw

                // remove any fully exploded asteroids
                asteroids.retain(|asteroid| asteroid.lives > 0);
                if asteroids.is_empty() {
                    state = GameState::Won;
                }

                for asteroid in asteroids.iter_mut() {
                    asteroid.draw();
                }

                // remove any lasers no longer on screen
                lasers.retain(|laser| laser.on_screen() && !laser.collided);
                for laser in lasers.iter_mut() {
                    laser.draw();
                }

                player.draw();
            }
            GameState::Won => {
                player.draw();
                for laser in lasers.iter_mut() {
                    laser.draw()
                }

                let text = "You Win!. Press [enter] to play again.";
                let font_size = 30.0;

                let text_size = measure_text(text, None, font_size as _, 1.0);
                draw_text(
                    text,
                    screen_width() / 2. - text_size.width / 2.,
                    screen_height() / 2. - text_size.height / 2.,
                    font_size,
                    DARKGRAY,
                );

                if is_key_down(KeyCode::Enter) {
                    // let mut level: u32 = 1;
                    state = GameState::Play;

                    // player_lives: u32 = 3;
                    player = Player {
                        position: screen_center(),
                        rotation: 0.0_f32.to_radians(),
                        velocity: Vec2::ZERO,
                        thrusting: false,
                    };

                    lasers = vec![];
                    asteroids = vec![];
                    for _ in 0..1 {
                        asteroids.push(Asteroid::spawn())
                    }
                }
            }
            GameState::GameOver => {
                player.draw();
                for asteroid in asteroids.iter_mut() {
                    asteroid.draw()
                }
                for laser in lasers.iter_mut() {
                    laser.draw()
                }

                let text = "Game Over. Press [enter] to play again.";
                let font_size = 30.0;
                let text_size = measure_text(text, None, font_size as _, 1.0);
                draw_text(
                    text,
                    screen_width() / 2. - text_size.width / 2.,
                    screen_height() / 2. - text_size.height / 2.,
                    font_size,
                    DARKGRAY,
                );
                if is_key_down(KeyCode::Enter) {
                    // let mut level: u32 = 1;
                    state = GameState::Play;

                    // player_lives: u32 = 3;
                    player = Player {
                        position: screen_center(),
                        rotation: 0.0_f32.to_radians(),
                        velocity: Vec2::ZERO,
                        thrusting: false,
                    };

                    lasers = vec![];
                    asteroids = vec![];
                    for _ in 0..1 {
                        asteroids.push(Asteroid::spawn())
                    }
                }
            }
        }
        next_frame().await
    }
}
