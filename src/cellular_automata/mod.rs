use macroquad::prelude::*;
use macroquad::rand::*;
mod cell;
mod grid;
use cell::*;
use grid::*;

pub fn automata() -> Conf {
    Conf {
        window_title: "AUTOMATA".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        fullscreen: false,
        ..Default::default()
    }
}

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
    // let mut rng = ::rand::thread_rng();

    let mut grid: Vec<Vec<Cell>> = Vec::new();

    for row in 0..ROW_COUNT {
        grid.push(Vec::new());
        for column in 0..COLUMN_COUNT {
            let random_state = if rand::gen_range(0, 2) < 1 { 0 } else { 1 };
            grid[row as usize].push(Cell::new(column, row, random_state));
        }
    }

    loop {
        clear_background(WHITE);

        for row in 0..ROW_COUNT as usize {
            for column in 0..COLUMN_COUNT as usize {
                let cell: Cell = grid[row as usize][column as usize];
                if cell.current_state == 1 {
                    draw_rectangle(
                        column as f32 * CELL_SIZE as f32,
                        row as f32 * CELL_SIZE as f32,
                        CELL_SIZE as f32,
                        CELL_SIZE as f32,
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

        for row in 0..ROW_COUNT as usize {
            for column in 0..COLUMN_COUNT as usize {
                cell_swap(&mut grid[row as usize][column as usize]);
            }
        }

        draw_text("fps: 30", 2.0, 10.0, 16.0, PINK);

        next_frame().await
    }
}
