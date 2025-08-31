use super::config::*;
use crate::shared::{Point, get_moore_neighbours};
use std::fmt;

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
