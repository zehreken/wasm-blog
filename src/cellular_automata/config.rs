use super::cell::Point;

pub const WINDOW_WIDTH: i32 = 640;
pub const WINDOW_HEIGHT: i32 = 512;
pub const CELL_SIZE: i32 = 4;
pub const COLUMN_COUNT: i32 = WINDOW_WIDTH / CELL_SIZE;
pub const ROW_COUNT: i32 = WINDOW_HEIGHT / CELL_SIZE;

pub const VON_NEUMANN_NEIGHBORHOOD: [Point; 4] = [
    Point { x: -1, y: 0 },
    Point { x: 0, y: -1 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
];

pub const MOORE_NEIGHBORHOOD: [Point; 8] = [
    Point { x: -1, y: -1 },
    Point { x: -1, y: 0 },
    Point { x: -1, y: 1 },
    Point { x: 0, y: 1 },
    Point { x: 1, y: 1 },
    Point { x: 1, y: 0 },
    Point { x: 1, y: -1 },
    Point { x: 0, y: -1 },
];
