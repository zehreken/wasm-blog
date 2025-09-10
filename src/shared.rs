use macroquad::color::Color;
use std::fmt;

pub const MAIN_COLOR: Color = Color::from_hex(0xFF0066);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

pub const MOORE_NEIGHBOURHOOD: [Point; 8] = [
    Point { x: -1, y: -1 },
    Point { x: -1, y: 0 },
    Point { x: -1, y: 1 },
    Point { x: 0, y: 1 },
    Point { x: 1, y: 1 },
    Point { x: 1, y: 0 },
    Point { x: 1, y: -1 },
    Point { x: 0, y: -1 },
];

// Also known as Manhattan distance
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
