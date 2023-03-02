use macroquad::{
    prelude::Color,
    shapes::{self, draw_circle},
};

#[derive(Clone, Copy)]
pub struct Cell {
    size: f32,
}

impl Cell {
    pub fn new(size: f32) -> Self {
        Self { size }
    }

    pub fn update(&self, delta_time: f32) {}

    pub fn draw(&self) {
        draw_circle(100.0, 100.0, self.size, Color::new(255.0, 0.0, 0.0, 255.0))
    }
}
