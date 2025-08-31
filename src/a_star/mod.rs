use std::collections::HashMap;

use crate::{
    a_star::{cell::Cell, config::CELL_SIZE},
    app::App,
    shared::Point,
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
                let cell_type = if row == 0 && column == 0 {
                    CellType::Start
                } else if row == 5 && column == 5 {
                    CellType::End
                } else {
                    CellType::Open
                };
                grid[row as usize].push(Cell::new(column, row, cell_type, Point::new(5, 5)));
            }
        }
        Self { grid }
    }

    fn find(&mut self) {
        struct Result {
            estimated: i32,
            cost: i32,
            total: i32,
        }
        let mut cell_to_result: HashMap<&Cell, Result> = HashMap::new();
        let mut frontier: Vec<&Cell> = Vec::new();
        frontier.push(&self.grid[0][0]);

        while !frontier.is_empty() {}
    }
}

impl App for AStar {
    fn update(&mut self) {
        self.find();
    }

    fn draw(&self) {
        for row in 0..self.grid.len() {
            for column in 0..self.grid[0].len() {
                self.grid[row][column].draw();
            }
        }
    }

    fn resize(&mut self, width: f32, height: f32) {
        let row_count = height as i32 / CELL_SIZE as i32;
        let column_count = width as i32 / CELL_SIZE as i32;
        let mut grid: Vec<Vec<Cell>> = Vec::new();
        for row in 0..row_count {
            grid.push(Vec::new());
            for column in 0..column_count {
                let cell_type = if row == 0 && column == 0 {
                    CellType::Start
                } else if row == 5 && column == 5 {
                    CellType::End
                } else {
                    CellType::Open
                };
                grid[row as usize].push(Cell::new(column, row, cell_type, Point::new(5, 5)));
            }
        }
        self.grid = grid;
    }
}

pub enum CellType {
    Open,
    Blocked,
    Start,
    End,
}
