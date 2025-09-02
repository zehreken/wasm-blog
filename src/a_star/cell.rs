use crate::{
    a_star::{CellType, config::CELL_SIZE},
    shared::Point,
};
use macroquad::prelude::*;

pub struct Cell {
    coord: Point,
    cell_type: CellType,
    e: i32,
    c: i32,
    t: i32,
}

impl Cell {
    pub fn new(x: i32, y: i32, cell_type: CellType) -> Self {
        Self {
            coord: Point::new(x, y),
            cell_type,
            e: 0,
            c: 0,
            t: 0,
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
            &format!(
                "{},{}\n{}|{}|{}",
                self.coord.x, self.coord.y, self.e, self.c, self.t
            ),
            x + 3.0,
            y + 13.0,
            16.0,
            None,
            BLACK,
        );
    }

    pub fn set(&mut self, e: i32, c: i32, t: i32) {
        self.e = e;
        self.c = c;
        self.t = t;
    }
}
