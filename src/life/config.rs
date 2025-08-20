use super::cell::Point;

pub const CELL_SIZE: i32 = 4;
pub const LIVE_NEIGHBOUR_FACTOR: i32 = 0;

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
