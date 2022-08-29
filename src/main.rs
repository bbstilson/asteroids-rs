use macroquad::{
    audio::{self, PlaySoundParams},
    prelude::*,
};

use asteroid::Asteroid;
use consts::*;
use laser::Laser;
use sounds::Sounds;
use utils::screen_center;
use world::World;

pub mod asteroid;
pub mod consts;
pub mod laser;
pub mod player;
pub mod sounds;
pub mod utils;
pub mod world;

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
    let sounds = Sounds::init().await.unwrap();

    let mut state = GameState::MainMenu;
    let mut world = World::new();

    let speed_of_light_pos = Vec2::new(SPEED_OF_LIGHT, SPEED_OF_LIGHT);
    let speed_of_light_neg = Vec2::new(-SPEED_OF_LIGHT, -SPEED_OF_LIGHT);

    loop {
        clear_background(LIGHTGRAY);

        match state {
            GameState::MainMenu => {
                let center = screen_center();

                let text_size = measure_text(TITLE, None, FONT_SIZE as _, 1.0);
                draw_text(
                    TITLE,
                    center.x - text_size.width - 50.0,
                    center.y - text_size.height * 5.0,
                    TITLE_FONT_SIZE,
                    BLACK,
                );

                let mut y_offset: f32 = 0.0;
                for text in &INSTRUCTIONS {
                    let text_size = measure_text(text, None, FONT_SIZE as _, 1.0);
                    y_offset += text_size.height + TEXT_PADDING;
                    draw_text(
                        text,
                        center.x - text_size.width / 2.0,
                        center.y + y_offset,
                        FONT_SIZE,
                        DARKGRAY,
                    );
                }

                if is_key_down(KeyCode::Enter) {
                    state = GameState::Play;
                    world = World::new();
                }
            }
            GameState::Play => {
                let text = format!("Score: {}", world.score);
                let text = text.as_str();
                let text_size = measure_text(text, None, FONT_SIZE as _, 1.0);
                draw_text(
                    text,
                    0.0 + FONT_SIZE,
                    text_size.height + FONT_SIZE,
                    FONT_SIZE,
                    DARKGRAY,
                );

                if is_key_down(KeyCode::Right) {
                    world.player.rotation += PLAYER_ROTATION_SPEED;
                } else if is_key_down(KeyCode::Left) {
                    world.player.rotation -= PLAYER_ROTATION_SPEED;
                }

                if is_key_down(KeyCode::Up) {
                    audio::play_sound(
                        sounds.engine,
                        PlaySoundParams {
                            looped: true,
                            volume: 1.0,
                        },
                    );
                    world.player.velocity += Vec2::new(
                        world.player.rotation.sin() * SHIP_THRUST,
                        -world.player.rotation.cos() * SHIP_THRUST,
                    );
                    // clamp player velocity to the speed of light
                    world.player.velocity = world
                        .player
                        .velocity
                        .clamp(speed_of_light_neg, speed_of_light_pos);

                    world.player.thrusting = true;
                } else {
                    audio::stop_sound(sounds.engine);
                    world.player.thrusting = false;
                }

                if is_key_pressed(KeyCode::Space) {
                    audio::play_sound_once(sounds.laser);
                    world.lasers.push(Laser {
                        position: world.player.position, // TODO fire from ship nose
                        velocity: Vec2::new(
                            world.player.rotation.sin() * LASER_SPEED,
                            -world.player.rotation.cos() * LASER_SPEED,
                        ),
                        collided: false,
                    });
                }

                // run updates
                for laser in world.lasers.iter_mut() {
                    laser.update();
                }

                for asteroid in world.asteroids.iter_mut() {
                    asteroid.update();
                }

                world.player.update();

                // check collisions between lasers and asteroids
                let mut asteroids_to_spawn = vec![];
                let mut times_to_increment_score = 0;
                for laser in world.lasers.iter_mut() {
                    for asteroid in world.asteroids.iter_mut() {
                        if laser.collides(asteroid) {
                            audio::play_sound_once(sounds.explosion);
                            times_to_increment_score += 1;
                            asteroid.explode();
                            asteroids_to_spawn.push(Asteroid::spawn_at(
                                asteroid.position
                                    + Vec2::new(asteroid.radius() * 2.0, asteroid.radius() * 2.0),
                                asteroid.lives,
                            ));
                            laser.collided = true;
                        }
                    }
                }

                for asteroid_a in world.asteroids.clone().iter_mut() {
                    for asteroid_b in world.asteroids.iter_mut() {
                        if asteroid_a.position != asteroid_b.position
                            && asteroid_a.collides(asteroid_b)
                        {
                            std::mem::swap(&mut asteroid_a.velocity, &mut asteroid_b.velocity);
                        }
                    }
                }

                for _ in 0..times_to_increment_score {
                    world.increment_score();
                }

                for asteroid in asteroids_to_spawn {
                    world.asteroids.push(asteroid);
                }

                // check collisions between asteroids and player
                for asteroid in world.asteroids.iter_mut() {
                    if world.player.collides(asteroid) {
                        state = GameState::GameOver;
                    }
                }

                // draw

                // remove any fully exploded asteroids
                world.asteroids.retain(|asteroid| asteroid.lives > 0);
                if world.asteroids.is_empty() {
                    state = GameState::Won;
                }

                for asteroid in world.asteroids.iter_mut() {
                    asteroid.draw();
                }

                // remove any lasers no longer on screen
                world
                    .lasers
                    .retain(|laser| laser.on_screen() && !laser.collided);
                for laser in world.lasers.iter_mut() {
                    laser.draw();
                }

                world.player.draw();
            }
            GameState::Won => {
                let text = format!("Level {} complete!", world.level,);
                let text = text.as_str();
                let text_size = measure_text(text, None, FONT_SIZE as _, 1.0);
                draw_text(
                    text,
                    screen_width() / 2. - text_size.width / 2.,
                    screen_height() / 2. - text_size.height / 2.,
                    FONT_SIZE,
                    DARKGRAY,
                );

                if is_key_down(KeyCode::Enter) {
                    state = GameState::Play;
                    world = world.next_level();
                }
            }
            GameState::GameOver => {
                world.player.draw();
                for asteroid in world.asteroids.iter_mut() {
                    asteroid.draw()
                }
                for laser in world.lasers.iter_mut() {
                    laser.draw()
                }

                let center = screen_center();
                let mut y_offset: f32 = 0.0;
                for text in &DEATH_INSTRUCTIONS {
                    let text_size = measure_text(text, None, FONT_SIZE as _, 1.0);
                    y_offset += text_size.height + TEXT_PADDING;
                    draw_text(
                        text,
                        center.x - text_size.width / 2.0,
                        center.y - 100.0 + y_offset,
                        FONT_SIZE,
                        DARKGRAY,
                    );
                }

                if is_key_down(KeyCode::Enter) {
                    state = GameState::Play;
                    world = World::new();
                }
            }
        }
        next_frame().await
    }
}
