use macroquad::prelude::Vec2;

pub fn get_random_vector() -> Vec2 {
    Vec2 {
        x: (macroquad::rand::rand() % 500) as f32,
        y: (macroquad::rand::rand() % 500) as f32,
    }
}
