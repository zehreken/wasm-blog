use std::collections::HashMap;

use crate::{
    a_star::{cell::Cell, config::CELL_SIZE},
    app::App,
    shared::{Point, get_id},
};

mod cell;
mod config;

pub fn get_title() -> String {
    return "AStar".to_owned();
}

pub struct AStar {
    row_count: usize,
    column_count: usize,
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
                grid[row as usize].push(Cell::new(column, row, cell_type));
            }
        }
        Self {
            row_count: row_count as usize,
            column_count: column_count as usize,
            grid,
        }
    }

    fn find(&mut self) {
        struct Result {
            estimated: i32,
            cost: i32,
            total: i32,
        }
        let mut id_to_result: HashMap<u64, Result> = HashMap::new();
        for row in 0..self.row_count {
            for column in 0..self.column_count {
                let point = Point::new(column as i32, row as i32);
                id_to_result.insert(
                    get_id(point),
                    Result {
                        estimated: (point.x - 5).abs() + (point.y - 5).abs(),
                        cost: 0,
                        total: 0,
                    },
                );
            }
        }
        let mut frontier: Vec<&Cell> = Vec::new();
        frontier.push(&self.grid[0][0]);

        // while !frontier.is_empty() {
        for row in 0..self.row_count {
            for column in 0..self.column_count {
                let point = Point::new(column as i32, row as i32);
                let r = &id_to_result[&get_id(point)];
                self.grid[row][column].set(r.estimated, r.cost, r.total);
            }
        }
        // }
    }
}

impl App for AStar {
    fn update(&mut self) {
        self.find();
    }

    fn draw(&self) {
        for row in 0..self.row_count {
            for column in 0..self.column_count {
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
                grid[row as usize].push(Cell::new(column, row, cell_type));
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
