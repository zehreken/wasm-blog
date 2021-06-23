use macroquad::prelude::*;
mod cell;
mod grid;
use ::rand::Rng;
use cell::*;
use grid::*;

fn get_live_neighbour_count(cell: Cell, grid: &Vec<Vec<Cell>>) -> i32 {
    let mut neighbour_count: i32 = 0;
    for i in 0..8 {
        if cell.neighbours[i].x >= 0
            && cell.neighbours[i].x < COLUMN_COUNT
            && cell.neighbours[i].y >= 0
            && cell.neighbours[i].y < ROW_COUNT
        {
            let current_state =
                grid[cell.neighbours[i].y as usize][cell.neighbours[i].x as usize].current_state;
            neighbour_count += current_state;
        }
    }

    neighbour_count
}

fn cell_tick(cell: &mut Cell, live_neighbour_count: i32) {
    if cell.current_state == 1 {
        if live_neighbour_count < 2 {
            cell.future_state = 0;
            cell.on_count = 0;
        } else if live_neighbour_count == 2 || live_neighbour_count == 3 {
            cell.future_state = 1;
            cell.on_count += 1;
        } else {
            cell.future_state = 0;
            cell.on_count = 0;
        }
    } else {
        if live_neighbour_count == 3 {
            cell.future_state = 1;
        } else {
            cell.future_state = 0;
        }
    }
}

fn cell_swap(cell: &mut Cell) {
    cell.current_state = cell.future_state;
}

pub async fn run() {
    let mut rng = ::rand::thread_rng();

    const CELL_SIZE: f32 = 10.0;
    let row_count = screen_height() / CELL_SIZE;
    let column_count = screen_width() / CELL_SIZE;

    let mut grid: Vec<Vec<Cell>> = Vec::new();

    for row in 0..ROW_COUNT {
        grid.push(Vec::new());
        for column in 0..COLUMN_COUNT {
            grid[row as usize].push(create_cell());
            grid[row as usize][column as usize].position = Point { x: column, y: row };
            grid[row as usize][column as usize].neighbours = calculate_neighbours(column, row);
            grid[row as usize][column as usize].current_state =
                if rng.gen_range(0..2) < 1 { 0 } else { 1 };
        }
    }

    loop {
        clear_background(WHITE);

        for row in 0..row_count as usize {
            for column in 0..column_count as usize {
                let cell: Cell = grid[row as usize][column as usize];
                if cell.current_state == 1 {
                    draw_rectangle(
                        column as f32 * CELL_SIZE,
                        row as f32 * CELL_SIZE,
                        CELL_SIZE,
                        CELL_SIZE,
                        BLACK,
                    );
                }

                let live_neighbour_count: i32 =
                    get_live_neighbour_count(grid[row as usize][column as usize], &grid);
                cell_tick(
                    &mut grid[row as usize][column as usize],
                    live_neighbour_count,
                );
            }
        }

        for row in 0..row_count as usize {
            for column in 0..column_count as usize {
                cell_swap(&mut grid[row as usize][column as usize]);
            }
        }

        draw_text("fps: 30", 2.0, 10.0, 16.0, PINK);

        next_frame().await
    }
}
