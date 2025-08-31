use crate::{
    a_star::{CellType, config::CELL_SIZE},
    shared::Point,
};
use macroquad::prelude::*;

pub struct Cell {
    coord: Point,
    cell_type: CellType,
    estimated: i32,
}

impl Cell {
    pub fn new(x: i32, y: i32, cell_type: CellType, goal_coord: Point) -> Self {
        let estimated = (x - goal_coord.x).abs() + (y - goal_coord.y).abs();
        Self {
            coord: Point::new(x, y),
            cell_type,
            estimated, // also called h
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
        draw_multiline_text(
            &format!("{},{}\n{}", self.coord.x, self.coord.y, self.estimated),
            x + 5.0,
            y + 15.0,
            16.0,
            None,
            BLACK,
        );
    }
}
