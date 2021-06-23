use std::fmt;

const MOORE_DIRECTIONS: [Point; 8] = [
    Point { x: -1, y: -1 },
    Point { x: -1, y: 0 },
    Point { x: -1, y: 1 },
    Point { x: 0, y: 1 },
    Point { x: 1, y: 1 },
    Point { x: 1, y: 0 },
    Point { x: 1, y: -1 },
    Point { x: 0, y: -1 },
];
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

pub fn create_cell() -> Cell {
    Cell {
        position: Point { x: 0, y: 0 },
        neighbours: calculate_neighbours(0, 0),
        current_state: 0,
        future_state: 0,
        on_count: 0,
    }
}

pub fn calculate_neighbours(x: i32, y: i32) -> [Point; 8] {
    let neighbours: [Point; 8] = [
        Point {
            x: MOORE_DIRECTIONS[0].x + x,
            y: MOORE_DIRECTIONS[0].y + y,
        },
        Point {
            x: MOORE_DIRECTIONS[1].x + x,
            y: MOORE_DIRECTIONS[1].y + y,
        },
        Point {
            x: MOORE_DIRECTIONS[2].x + x,
            y: MOORE_DIRECTIONS[2].y + y,
        },
        Point {
            x: MOORE_DIRECTIONS[3].x + x,
            y: MOORE_DIRECTIONS[3].y + y,
        },
        Point {
            x: MOORE_DIRECTIONS[4].x + x,
            y: MOORE_DIRECTIONS[4].y + y,
        },
        Point {
            x: MOORE_DIRECTIONS[5].x + x,
            y: MOORE_DIRECTIONS[5].y + y,
        },
        Point {
            x: MOORE_DIRECTIONS[6].x + x,
            y: MOORE_DIRECTIONS[6].y + y,
        },
        Point {
            x: MOORE_DIRECTIONS[7].x + x,
            y: MOORE_DIRECTIONS[7].y + y,
        },
    ];

    neighbours
}
