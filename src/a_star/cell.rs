use crate::{
    a_star::{CellType, config::CELL_SIZE},
    shared::Point,
};
use macroquad::prelude::*;

pub struct Cell {
    pub coord: Point,
    pub cell_type: CellType,
}

impl Cell {
    pub fn new(x: i32, y: i32, cell_type: CellType) -> Self {
        Self {
            coord: Point::new(x, y),
            cell_type,
        }
    }

    pub fn draw(&self) {
        let color = match self.cell_type {
            CellType::Open => WHITE,
            CellType::Blocked => GRAY,
            CellType::Start => GREEN,
            CellType::End => RED,
        };
        let (x, y) = (
            self.coord.x as f32 * CELL_SIZE,
            self.coord.y as f32 * CELL_SIZE,
        );
        draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, color);
        draw_rectangle_lines(x, y, CELL_SIZE, CELL_SIZE, 2.0, BLACK);
    }
}
