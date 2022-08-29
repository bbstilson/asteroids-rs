pub const PLAYER_SIZE: f32 = 25.0;
pub const PLAYER_ROTATION_SPEED: f32 = 0.08;
pub const SHIP_THRUST: f32 = 0.1;
pub const FLAME_SIZE: f32 = 10.0;

// 135 degrees in radians
pub const ANGLE_135: f32 = 2.356194;
// 180 degrees in radians
pub const ANGLE_180: f32 = std::f32::consts::PI;
// 225 degrees in radians
pub const ANGLE_225: f32 = 3.926991;

pub const LASER_SPEED: f32 = 10.0;
pub const LASER_SIZE: f32 = 2.0;

pub const ASTEROID_SIZE: f32 = 20.0;
pub const ASTEROID_SPAWN_DISTANCE: f32 = PLAYER_SIZE * 10.0;

// nothing can accelerate past this velocity
pub const SPEED_OF_LIGHT: f32 = 7.0;

pub const TITLE: &str = "Asteroids";
pub const INSTRUCTIONS: [&str; 5] = [
    "Press [enter] to play",
    "[left] to rotate left",
    "[right] to rotate right",
    "[up] to thrust",
    "[space] to fire laser",
];
pub const DEATH_INSTRUCTIONS: [&str; 3] = [
    "You Died :C",
    "Insert more quarters to continue",
    "Or press [enter]",
];

pub const TITLE_FONT_SIZE: f32 = 120.0;
pub const FONT_SIZE: f32 = 40.0;
pub const TEXT_PADDING: f32 = 20.0;
