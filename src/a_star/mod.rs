use crate::{
    a_star::{cell::Cell, config::CELL_SIZE},
    app::App,
    shared::{Point, get_taxicab_neighbours},
};
use macroquad::{input, prelude::*, rand::rand};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
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
    start: Point,
    end: Point,
    point_to_move_cost: HashMap<Point, MoveCost>,
    path: HashSet<Point>,
}

struct MoveCost {
    pub estimated: i32,
    pub real: i32,
}

impl MoveCost {
    pub fn total(&self) -> i32 {
        self.real + self.estimated
    }
}

impl AStar {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            row_count: 0,
            column_count: 0,
            grid: Vec::new(),
            start: Point::new(0, 0),
            end: Point::new(0, 0),
            point_to_move_cost: HashMap::new(),
            path: HashSet::new(),
        }
    }

    fn find_path(&mut self) {
        self.point_to_move_cost.clear();
        self.path.clear();

        let mut open_set = BinaryHeap::new();
        let start = self.start;
        let end = self.end;
        let start_to_diff = (start.x - end.x).abs() + (start.y - end.y).abs();
        open_set.push((Reverse(start_to_diff), start));
        self.point_to_move_cost.insert(
            start,
            MoveCost {
                estimated: start_to_diff,
                real: 0,
            },
        );

        let mut closed_set = HashSet::new();
        self.path.insert(start);

        while let Some((_total, current)) = open_set.pop() {
            if current.x == end.x && current.y == end.y {
                break;
            }

            if closed_set.contains(&current) {
                continue;
            }

            closed_set.insert(current);

            let current_cost = self.point_to_move_cost[&current].real;

            let neighbours = get_taxicab_neighbours(current.x, current.y, 1);
            for neighbour in neighbours {
                if neighbour.x < 0
                    || neighbour.y < 0
                    || neighbour.x >= self.column_count as i32
                    || neighbour.y >= self.row_count as i32
                {
                    continue;
                }
                if self.grid[neighbour.y as usize][neighbour.x as usize].cell_type
                    == CellType::Blocked
                {
                    continue;
                }

                self.path.insert(current);
                let estimated = (neighbour.x - end.x).abs() + (neighbour.y - end.y).abs();
                let move_cost = MoveCost {
                    estimated,
                    real: current_cost + 1,
                };
                if !self.point_to_move_cost.contains_key(&neighbour) {
                    open_set.push((Reverse(move_cost.total()), neighbour));
                    self.point_to_move_cost.insert(neighbour, move_cost);
                }
            }
        }
    }
}

impl App for AStar {
    fn update(&mut self) {
        if input::is_key_pressed(KeyCode::F) {
            self.find_path();
        }
    }

    fn draw(&self) {
        for row in 0..self.row_count {
            for column in 0..self.column_count {
                self.grid[row][column].draw();

                let x = column as f32 * CELL_SIZE;
                let y = row as f32 * CELL_SIZE;

                let point = Point::new(column as i32, row as i32);
                if self.point_to_move_cost.contains_key(&point) {
                    let move_cost = &self.point_to_move_cost[&point];
                    draw_multiline_text(
                        &format!(
                            "{},{}\n{}|{}|{}",
                            column,
                            row,
                            move_cost.estimated,
                            move_cost.real,
                            move_cost.total()
                        ),
                        x + 3.0,
                        y + 13.0,
                        14.0,
                        None,
                        BLACK,
                    );
                }
            }
        }
        for point in &self.path {
            let x = point.x as f32 * CELL_SIZE + CELL_SIZE / 2.0;
            let y = point.y as f32 * CELL_SIZE + CELL_SIZE / 2.0;
            draw_circle(x, y, 5.0, BLUE);
        }
    }

    fn resize(&mut self, width: f32, height: f32) {
        let row_count = height as i32 / CELL_SIZE as i32;
        let column_count = width as i32 / CELL_SIZE as i32;
        let mut grid: Vec<Vec<Cell>> = Vec::new();
        let start = Point::new(0, 0);
        let end = Point::new(column_count - 1, 0);
        for row in 0..row_count {
            grid.push(Vec::new());
            for column in 0..column_count {
                let cell_type = if row == start.y && column == start.x {
                    CellType::Start
                } else if row == end.y && column == end.x {
                    CellType::End
                } else {
                    let rnd = rand() % 5;
                    if rnd == 0 {
                        CellType::Blocked
                    } else {
                        CellType::Open
                    }
                };
                grid[row as usize].push(Cell::new(column, row, cell_type));
            }
        }
        self.row_count = row_count as usize;
        self.column_count = column_count as usize;
        self.grid = grid;
        self.start = start;
        self.end = end;
    }
}

#[derive(PartialEq)]
pub enum CellType {
    Open,
    Blocked,
    Start,
    End,
}
