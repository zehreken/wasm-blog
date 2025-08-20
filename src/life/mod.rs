use macroquad::prelude::*;
mod cell;
mod config;
use cell::*;
use config::*;

pub fn get_config() -> Conf {
    Conf {
        window_title: "Life".to_owned(),
        window_width: 512,
        window_height: 512,
        fullscreen: false,
        ..Default::default()
    }
}

struct App {
    width: f32,
    height: f32,
    row_count: i32,
    column_count: i32,
    grid: Vec<Vec<Cell>>,
}

impl App {
    fn new(width: f32, height: f32) -> Self {
        let row_count = height as i32 / CELL_SIZE;
        let column_count = width as i32 / CELL_SIZE;
        let mut grid: Vec<Vec<Cell>> = Vec::new();
        for row in 0..row_count {
            grid.push(Vec::new());
            for column in 0..column_count {
                let random_state = if rand::gen_range(0, 2) < 1 { 0 } else { 1 };
                grid[row as usize].push(Cell::new(column, row, random_state));
            }
        }

        Self {
            width,
            height,
            row_count,
            column_count,
            grid,
        }
    }

    fn update(&mut self) {
        for row in 0..self.row_count as usize {
            for column in 0..self.column_count as usize {
                let cell: &Cell = &self.grid[row as usize][column as usize];
                if cell.get_current_state() == 1 {
                    draw_rectangle(
                        column as f32 * CELL_SIZE as f32,
                        row as f32 * CELL_SIZE as f32,
                        CELL_SIZE as f32,
                        CELL_SIZE as f32,
                        BLACK,
                    );
                }

                let live_neighbour_count: i32 = cell.get_live_neighbour_count(&self.grid);

                // Borrow cell mutably
                let cell = &mut self.grid[row as usize][column as usize];
                cell.tick(live_neighbour_count);
            }
        }

        for row in 0..self.row_count as usize {
            for column in 0..self.column_count as usize {
                self.grid[row as usize][column as usize].swap();
            }
        }
    }

    fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
        self.row_count = height as i32 / CELL_SIZE;
        self.column_count = width as i32 / CELL_SIZE;
        let mut grid: Vec<Vec<Cell>> = Vec::new();
        for row in 0..self.row_count {
            grid.push(Vec::new());
            for column in 0..self.column_count {
                let random_state = if rand::gen_range(0, 2) < 1 { 0 } else { 1 };
                grid[row as usize].push(Cell::new(column, row, random_state));
            }
        }

        self.grid = grid;
    }
}

pub async fn run() {
    let mut app = App::new(screen_width(), screen_height());

    loop {
        clear_background(WHITE);

        if (app.width - screen_width()).abs() > 1.0 || (app.height - screen_height()).abs() > 1.0 {
            app.resize(screen_width(), screen_height());
        }

        app.update();

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
