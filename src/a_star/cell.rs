use macroquad::{color::BLACK, math::Vec2, shapes::draw_rectangle_lines};

use crate::a_star::config::CELL_SIZE;

pub struct Cell {
    position: Vec2,
}

impl Cell {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
        }
    }

    pub fn draw(&self) {
        draw_rectangle_lines(
            self.position.x,
            self.position.y,
            CELL_SIZE,
            CELL_SIZE,
            2.0,
            BLACK,
        );
    }
}
