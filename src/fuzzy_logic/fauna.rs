use macroquad::{prelude::*, ui::root_ui};

use super::cell::*;

pub struct Fauna {
    cells: Vec<Cell>,
    big_cell: BigCell,
}

impl Fauna {
    pub fn new() -> Self {
        let cells: Vec<Cell> = (0..10).map(|i| Cell::new(i, RED)).collect();
        Self {
            cells,
            big_cell: BigCell::new(50_f32, BLUE),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        for cell in &mut self.cells {
            cell.update(delta_time);
        }
        self.big_cell.update(delta_time, &mut self.cells);
    }

    pub fn draw(&self) {
        for cell in &self.cells {
            cell.draw();
        }
        self.big_cell.draw();
    }

    pub fn ui(&mut self) {
        root_ui().label(None, &format!("fps: {}", macroquad::time::get_fps()));
        root_ui().label(
            None,
            &format!("distance weight {}", self.big_cell.get_distance_weight()),
        );
        if root_ui().button(None, "-") {
            self.big_cell.set_distance_weight(-0.1);
        }
        if root_ui().button(None, "+") {
            self.big_cell.set_distance_weight(0.1);
        }
        root_ui().label(
            None,
            &format!("size weight {}", self.big_cell.get_size_weight()),
        );
        if root_ui().button(None, "-") {
            self.big_cell.set_size_weight(-0.1);
        }
        if root_ui().button(None, "+") {
            self.big_cell.set_size_weight(0.1);
        }
    }
}
