use super::config::*;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug, Clone)]
pub struct Cell {
    position: Point,
    neighbours: Vec<Point>,
    current_state: i32,
    future_state: i32,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "position {}", self.position)?;
        writeln!(f, "neighbours: [")?;
        for i in 0..8 {
            writeln!(f, "{}: {}", i, self.neighbours[i])?;
        }
        writeln!(f, "]")?;
        writeln!(f, "current_state: {}", self.current_state)?;
        writeln!(f, "future_state: {}", self.future_state)
    }
}

impl Cell {
    pub fn new(x: i32, y: i32, current_state: i32) -> Self {
        Self {
            position: Point { x, y },
            // neighbours: get_taxicab_neighbours(x, y, 1),
            neighbours: get_moore_neighbours(x, y),
            current_state,
            future_state: 0,
        }
    }

    pub fn tick(&mut self, live_neighbour_count: i32) {
        if self.current_state == 1 {
            if live_neighbour_count < 2 + LIVE_NEIGHBOUR_FACTOR {
                self.future_state = 0;
            } else if live_neighbour_count == 2 + LIVE_NEIGHBOUR_FACTOR
                || live_neighbour_count == 3 + LIVE_NEIGHBOUR_FACTOR
            {
                self.future_state = 1;
            } else {
                self.future_state = 0;
            }
        } else {
            if live_neighbour_count == 3 {
                self.future_state = 1;
            } else {
                self.future_state = 0;
            }
        }
    }

    pub fn swap(&mut self) {
        self.current_state = self.future_state;
    }

    pub fn get_live_neighbour_count(&self, grid: &Vec<Vec<Cell>>) -> i32 {
        let mut neighbour_count: i32 = 0;
        for i in 0..self.neighbours.len() {
            if self.neighbours[i].x >= 0
                && self.neighbours[i].x < grid[0].len() as i32
                && self.neighbours[i].y >= 0
                && self.neighbours[i].y < grid.len() as i32
            {
                let current_state = grid[self.neighbours[i].y as usize]
                    [self.neighbours[i].x as usize]
                    .current_state;
                neighbour_count += current_state;
            }
        }

        neighbour_count
    }

    pub fn get_current_state(&self) -> i32 {
        self.current_state
    }
}

pub fn get_taxicab_neighbours(x: i32, y: i32, r: i32) -> Vec<Point> {
    if r <= 0 {
        panic!("wrong argument r");
    }
    let mut neighbours: Vec<Point> = Vec::new();

    // Taxicab distance |y1 – y2| + |x1 – x2|
    let start_x = x - r;
    let end_x = x + r;
    let start_y = y - r;
    let end_y = y + r;
    for x_n in start_x..=end_x {
        for y_n in start_y..=end_y {
            if (y - y_n).abs() + (x - x_n).abs() <= r && !(y == y_n && x == x_n) {
                neighbours.push(Point { x: x_n, y: y_n });
            }
        }
    }

    neighbours
}

pub fn get_von_neumann_neighbours(x: i32, y: i32) -> Vec<Point> {
    get_taxicab_neighbours(x, y, 1)
}

pub fn get_moore_neighbours(x: i32, y: i32) -> Vec<Point> {
    let mut neighbours: Vec<Point> = Vec::new();
    for i in 0..MOORE_NEIGHBOURHOOD.len() {
        neighbours.push(Point {
            x: MOORE_NEIGHBOURHOOD[i].x + x,
            y: MOORE_NEIGHBOURHOOD[i].y + y,
        });
    }

    neighbours
}
