use macroquad::prelude::*;

pub fn screen_center() -> Vec2 {
    Vec2::new(screen_width() / 2.0, screen_height() / 2.0)
}

pub fn wrap_around(position: &mut Vec2) {
    if position.x > screen_width() {
        position.x = 0.0;
    } else if position.x < 0.0 {
        position.x = screen_width();
    } else if position.y > screen_height() {
        position.y = 0.0;
    } else if position.y < 0.0 {
        position.y = screen_height();
    }
}
