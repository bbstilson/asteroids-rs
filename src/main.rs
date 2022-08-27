use macroquad::prelude::*;

const PLAYER_SIZE: f32 = 25.0;
const PLAYER_ROTATION_SPEED: f32 = 0.08;
const SHIP_THRUST: f32 = 0.1;
const FLAME_SIZE: f32 = 10.0;

const LASER_SPEED: f32 = 10.0;
const LASER_SIZE: f32 = 2.0;

const ASTEROID_SIZE: f32 = 30.0;
const ASTEROID_SPAWN_DISTANCE: f32 = PLAYER_SIZE * 10.0;

// 135 degrees in radians
const ANGLE_135: f32 = 2.356194;
// 180 degrees in radians
const ANGLE_180: f32 = std::f32::consts::PI;
// 225 degrees in radians
const ANGLE_225: f32 = 3.926991;

struct Player {
    position: Vec2,
    rotation: f32,
    velocity: Vec2,
    thrusting: bool,
}

struct Laser {
    position: Vec2,
    velocity: Vec2,
}

impl Laser {
    pub fn collides(&self, asteroid: &Asteroid) -> bool {
        false
    }
}

struct Asteroid {
    position: Vec2,
    rotation: f32,
    velocity: Vec2,
    lives: u32,
}

fn screen_center() -> Vec2 {
    Vec2::new(screen_width() / 2.0, screen_height() / 2.0)
}

impl Asteroid {
    pub fn new() -> Asteroid {
        let screen_center = screen_center();
        let random_angle = rand::gen_range(0.0, std::f32::consts::PI * 2.0);

        println!("{random_angle:?}");

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

    pub fn draw(&self) {
        draw_circle_lines(self.position.x, self.position.y, ASTEROID_SIZE, 2.0, BROWN);
    }

    pub fn update(&mut self) {
        self.position.x %= screen_width();
        self.position.y %= screen_height();
        self.position += self.velocity;
        println!("{:?}", self.position);
    }
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
        // draw_circle_lines(self.position.x, self.position.y, PLAYER_SIZE, 2.0, BLACK);

        if self.thrusting {
            let d = Vec2::new(
                self.position.x + (self.rotation + ANGLE_180).sin() * (PLAYER_SIZE + FLAME_SIZE),
                self.position.y - (self.rotation + ANGLE_180).cos() * (PLAYER_SIZE + FLAME_SIZE),
            );
            draw_triangle(b, c, d, Color::new(0.698, 0.1328, 0.1328, 1.0));
        }
    }

    pub fn update(&mut self) {
        self.position += self.velocity
    }

    pub fn collides(&self, asteroid: &Asteroid) -> bool {
        false
    }
}

pub enum GameState {
    MainMenu,
    Play,
    Died,
    Won,
    GameOver,
}

#[macroquad::main("asteroids")]
async fn main() {
    let mut level: u32 = 1;
    let mut state = GameState::Play;

    let mut player_lives: u32 = 3;
    let mut player = Player {
        position: screen_center(),
        rotation: 0.0_f32.to_radians(),
        velocity: Vec2::ZERO,
        thrusting: false,
    };

    let mut lasers: Vec<Laser> = vec![];
    let mut asteroids: Vec<Asteroid> = vec![];
    for _ in 0..10 {
        asteroids.push(Asteroid::new())
    }

    loop {
        match state {
            GameState::MainMenu => {}
            GameState::Play => {
                if is_key_down(KeyCode::Right) {
                    player.rotation += PLAYER_ROTATION_SPEED;
                } else if is_key_down(KeyCode::Left) {
                    player.rotation -= PLAYER_ROTATION_SPEED;
                }

                if is_key_down(KeyCode::Up) {
                    player.velocity += Vec2::new(
                        player.rotation.sin() * SHIP_THRUST,
                        -player.rotation.cos() * SHIP_THRUST,
                    );
                    player.thrusting = true;
                } else {
                    player.thrusting = false;
                }

                if is_key_pressed(KeyCode::Space) {
                    lasers.push(Laser {
                        position: player.position, // TODO fire from ship nose
                        velocity: Vec2::new(
                            player.rotation.sin() * LASER_SPEED,
                            -player.rotation.cos() * LASER_SPEED,
                        ),
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
                for laser in lasers.iter_mut() {
                    for asteroid in asteroids.iter_mut() {
                        if laser.collides(asteroid) {
                            // split asteroid
                            // schedule laser for deletion
                        }
                    }
                }

                // check collisions between asteroids and player
                for asteroid in asteroids.iter_mut() {
                    if player.collides(asteroid) {
                        state = GameState::Died;
                    }
                }

                // begin draw

                clear_background(LIGHTGRAY);

                // remove any lasers no longer on screen
                lasers.retain(|laser| laser.on_screen());
                for laser in lasers.iter_mut() {
                    laser.draw();
                }

                // wrap any asteroids that went off screen
                for asteroid in asteroids.iter_mut() {
                    asteroid.draw();
                }

                // wrap player if they went off screen
                player.draw();

                next_frame().await
            }
            GameState::Died => {}
            GameState::Won => {}
            GameState::GameOver => {}
        }
    }
}
