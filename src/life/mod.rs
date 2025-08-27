use macroquad::prelude::*;
mod cell;
mod config;
use crate::app::App;
use cell::*;
use config::*;

pub fn get_title() -> String {
    return "Life".to_owned();
}

pub struct Life {
    row_count: i32,
    column_count: i32,
    grid: Vec<Vec<Cell>>,
}

impl Life {
    pub fn new(width: f32, height: f32) -> Self {
        let row_count = height as i32 / CELL_SIZE;
        let column_count = width as i32 / CELL_SIZE;
        let mut grid: Vec<Vec<Cell>> = Vec::new();
        for row in 0..row_count {
            grid.push(Vec::new());
            for column in 0..column_count {
                let random_state = if rand::gen_range(0, 2) < 1 { 0 } else { 1 };
                grid[row as usize].push(Cell::new(column, row, random_state));
            }
        }

        Self {
            row_count,
            column_count,
            grid,
        }
    }
}

impl App for Life {
    fn update(&mut self) {
        for row in 0..self.row_count as usize {
            for column in 0..self.column_count as usize {
                let cell: &Cell = &self.grid[row as usize][column as usize];
                if cell.get_current_state() == 1 {
                    draw_rectangle(
                        column as f32 * CELL_SIZE as f32,
                        row as f32 * CELL_SIZE as f32,
                        CELL_SIZE as f32,
                        CELL_SIZE as f32,
                        BLACK,
                    );
                }

                let live_neighbour_count: i32 = cell.get_live_neighbour_count(&self.grid);

                // Borrow cell mutably
                let cell = &mut self.grid[row as usize][column as usize];
                cell.tick(live_neighbour_count);
            }
        }

        for row in 0..self.row_count as usize {
            for column in 0..self.column_count as usize {
                self.grid[row as usize][column as usize].swap();
            }
        }
    }

    fn draw(&self) {}

    fn resize(&mut self, width: f32, height: f32) {
        self.row_count = height as i32 / CELL_SIZE;
        self.column_count = width as i32 / CELL_SIZE;
        let mut grid: Vec<Vec<Cell>> = Vec::new();
        for row in 0..self.row_count {
            grid.push(Vec::new());
            for column in 0..self.column_count {
                let random_state = if rand::gen_range(0, 2) < 1 { 0 } else { 1 };
                grid[row as usize].push(Cell::new(column, row, random_state));
            }
        }

        self.grid = grid;
    }
}
