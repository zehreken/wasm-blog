use macroquad::{
    prelude::Vec2,
    window::{screen_height, screen_width},
};

pub fn get_random_vector() -> Vec2 {
    Vec2 {
        x: (macroquad::rand::rand() % screen_width() as u32) as f32,
        y: (macroquad::rand::rand() % screen_height() as u32) as f32,
    }
}
