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
            // neighbours: get_moore_neighbours(x, y),
            neighbours: get_von_neumann_neighbours(x, y),
            current_state,
            future_state: 0,
        }
    }

    pub fn tick(&mut self, live_neighbour_count: i32) {
        if self.current_state == 1 {
            if live_neighbour_count < 2 {
                self.future_state = 0;
            } else if live_neighbour_count == 2 || live_neighbour_count == 3 {
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
                && self.neighbours[i].x < COLUMN_COUNT
                && self.neighbours[i].y >= 0
                && self.neighbours[i].y < ROW_COUNT
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

pub fn get_von_neumann_neighbours(x: i32, y: i32) -> Vec<Point> {
    let mut neighbours: Vec<Point> = Vec::new();
    neighbours.push(Point {
        x: VON_NEUMANN_NEIGHBOURHOOD[0].x + x,
        y: VON_NEUMANN_NEIGHBOURHOOD[0].y + y,
    });
    neighbours.push(Point {
        x: VON_NEUMANN_NEIGHBOURHOOD[1].x + x,
        y: VON_NEUMANN_NEIGHBOURHOOD[1].y + y,
    });
    neighbours.push(Point {
        x: VON_NEUMANN_NEIGHBOURHOOD[2].x + x,
        y: VON_NEUMANN_NEIGHBOURHOOD[2].y + y,
    });
    neighbours.push(Point {
        x: VON_NEUMANN_NEIGHBOURHOOD[3].x + x,
        y: VON_NEUMANN_NEIGHBOURHOOD[3].y + y,
    });

    neighbours
}

pub fn get_moore_neighbours(x: i32, y: i32) -> Vec<Point> {
    let mut neighbours: Vec<Point> = Vec::new();
    neighbours.push(Point {
        x: MOORE_NEIGHBOURHOOD[0].x + x,
        y: MOORE_NEIGHBOURHOOD[0].y + y,
    });
    neighbours.push(Point {
        x: MOORE_NEIGHBOURHOOD[1].x + x,
        y: MOORE_NEIGHBOURHOOD[1].y + y,
    });
    neighbours.push(Point {
        x: MOORE_NEIGHBOURHOOD[2].x + x,
        y: MOORE_NEIGHBOURHOOD[2].y + y,
    });
    neighbours.push(Point {
        x: MOORE_NEIGHBOURHOOD[3].x + x,
        y: MOORE_NEIGHBOURHOOD[3].y + y,
    });
    neighbours.push(Point {
        x: MOORE_NEIGHBOURHOOD[4].x + x,
        y: MOORE_NEIGHBOURHOOD[4].y + y,
    });
    neighbours.push(Point {
        x: MOORE_NEIGHBOURHOOD[5].x + x,
        y: MOORE_NEIGHBOURHOOD[5].y + y,
    });
    neighbours.push(Point {
        x: MOORE_NEIGHBOURHOOD[6].x + x,
        y: MOORE_NEIGHBOURHOOD[6].y + y,
    });
    neighbours.push(Point {
        x: MOORE_NEIGHBOURHOOD[7].x + x,
        y: MOORE_NEIGHBOURHOOD[7].y + y,
    });

    neighbours
}
