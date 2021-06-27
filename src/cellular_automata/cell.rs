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
    pub on_count: i32,
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
        writeln!(f, "future_state: {}", self.future_state)?;
        writeln!(f, "on_count: {}", self.on_count)
    }
}

impl Cell {
    pub fn new(x: i32, y: i32, current_state: i32) -> Self {
        Self {
            position: Point { x, y },
            neighbours: calculate_neighbors(x, y),
            current_state,
            future_state: 0,
            on_count: 0,
        }
    }
}

pub fn calculate_neighbors(x: i32, y: i32) -> [Point; 8] {
    let neighbors: [Point; 8] = [
        Point {
            x: MOORE_NEIGHBORHOOD[0].x + x,
            y: MOORE_NEIGHBORHOOD[0].y + y,
        },
        Point {
            x: MOORE_NEIGHBORHOOD[1].x + x,
            y: MOORE_NEIGHBORHOOD[1].y + y,
        },
        Point {
            x: MOORE_NEIGHBORHOOD[2].x + x,
            y: MOORE_NEIGHBORHOOD[2].y + y,
        },
        Point {
            x: MOORE_NEIGHBORHOOD[3].x + x,
            y: MOORE_NEIGHBORHOOD[3].y + y,
        },
        Point {
            x: MOORE_NEIGHBORHOOD[4].x + x,
            y: MOORE_NEIGHBORHOOD[4].y + y,
        },
        Point {
            x: MOORE_NEIGHBORHOOD[5].x + x,
            y: MOORE_NEIGHBORHOOD[5].y + y,
        },
        Point {
            x: MOORE_NEIGHBORHOOD[6].x + x,
            y: MOORE_NEIGHBORHOOD[6].y + y,
        },
        Point {
            x: MOORE_NEIGHBORHOOD[7].x + x,
            y: MOORE_NEIGHBORHOOD[7].y + y,
        },
    ];

    neighbors
}
