use macroquad::prelude::*;
mod cell;
mod config;
use cell::*;
use config::*;

pub fn automata() -> Conf {
    Conf {
        window_title: "AUTOMATA".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        fullscreen: false,
        ..Default::default()
    }
}

pub async fn run() {
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
                if cell.get_current_state() == 1 {
                    draw_rectangle(
                        column as f32 * CELL_SIZE as f32,
                        row as f32 * CELL_SIZE as f32,
                        CELL_SIZE as f32,
                        CELL_SIZE as f32,
                        BLACK,
                    );
                }

                let live_neighbour_count: i32 = cell.get_live_neighbour_count(&grid);
                grid[row as usize][column as usize].tick(live_neighbour_count);
            }
        }

        for row in 0..ROW_COUNT as usize {
            for column in 0..COLUMN_COUNT as usize {
                grid[row as usize][column as usize].swap();
            }
        }

        draw_text(
            &format!("fps: {}", macroquad::time::get_fps()),
            2.0,
            12.0,
            16.0,
            PINK,
        );

        next_frame().await
    }
}
