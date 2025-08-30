use crate::{
    a_star::{cell::Cell, config::CELL_SIZE},
    app::App,
};

mod cell;
mod config;

pub fn get_title() -> String {
    return "AStar".to_owned();
}

pub struct AStar {
    grid: Vec<Vec<Cell>>,
}

impl AStar {
    pub fn new(width: f32, height: f32) -> Self {
        let row_count = height as i32 / CELL_SIZE as i32;
        let column_count = width as i32 / CELL_SIZE as i32;
        let mut grid: Vec<Vec<Cell>> = Vec::new();
        for row in 0..row_count {
            grid.push(Vec::new());
            for column in 0..column_count {
                grid[row as usize].push(Cell::new(column as f32, row as f32));
            }
        }
        Self { grid }
    }
}

impl App for AStar {
    fn update(&mut self) {}

    fn draw(&self) {
        for row in 0..self.grid.len() {
            for column in 0..self.grid[0].len() {
                self.grid[row][column].draw();
            }
        }
    }

    fn resize(&mut self, width: f32, height: f32) {}
}
