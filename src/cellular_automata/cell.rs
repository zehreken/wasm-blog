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

#[derive(Debug, Copy, Clone)]
pub struct Cell {
    pub position: Point,
    pub neighbours: [Point; 8],
    pub current_state: i32,
    pub future_state: i32,
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
            neighbours: calculate_neighbours(x, y),
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
}

pub fn calculate_neighbours(x: i32, y: i32) -> [Point; 8] {
    let neighbours: [Point; 8] = [
        Point {
            x: MOORE_NEIGHBOURHOOD[0].x + x,
            y: MOORE_NEIGHBOURHOOD[0].y + y,
        },
        Point {
            x: MOORE_NEIGHBOURHOOD[1].x + x,
            y: MOORE_NEIGHBOURHOOD[1].y + y,
        },
        Point {
            x: MOORE_NEIGHBOURHOOD[2].x + x,
            y: MOORE_NEIGHBOURHOOD[2].y + y,
        },
        Point {
            x: MOORE_NEIGHBOURHOOD[3].x + x,
            y: MOORE_NEIGHBOURHOOD[3].y + y,
        },
        Point {
            x: MOORE_NEIGHBOURHOOD[4].x + x,
            y: MOORE_NEIGHBOURHOOD[4].y + y,
        },
        Point {
            x: MOORE_NEIGHBOURHOOD[5].x + x,
            y: MOORE_NEIGHBOURHOOD[5].y + y,
        },
        Point {
            x: MOORE_NEIGHBOURHOOD[6].x + x,
            y: MOORE_NEIGHBOURHOOD[6].y + y,
        },
        Point {
            x: MOORE_NEIGHBOURHOOD[7].x + x,
            y: MOORE_NEIGHBOURHOOD[7].y + y,
        },
    ];

    neighbours
}
