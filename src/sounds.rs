use macroquad::{
    audio::{self, Sound},
    prelude::*,
};

pub struct Sounds {
    pub laser: Sound,
    pub explosion: Sound,
    pub engine: Sound,
}

impl Sounds {
    pub async fn init() -> Result<Sounds, macroquad::file::FileError> {
        set_pc_assets_folder("assets");
        let laser = audio::load_sound("laser.wav").await.unwrap();
        let explosion = audio::load_sound("explosion.wav").await.unwrap();
        let engine = audio::load_sound("engine.wav").await.unwrap();

        Ok(Sounds {
            laser,
            explosion,
            engine,
        })
    }
}
