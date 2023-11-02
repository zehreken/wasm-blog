use macroquad::{
    hash,
    prelude::*,
    ui::{root_ui, widgets::Group},
};

use super::cell::*;

pub struct Fauna {
    cells: Vec<Cell>,
    big_cell: BigCell,
}

impl Fauna {
    pub fn new() -> Self {
        let cells: Vec<Cell> = (0..10)
            .map(|i| Cell::new(i, Color::from_rgba(28, 28, 28, 255)))
            .collect();
        Self {
            cells,
            big_cell: BigCell::new(50_f32, Color::from_rgba(255, 157, 11, 255)),
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
        Group::new(hash!(), vec2(200.0, 120.0)).ui(&mut *root_ui(), |ui| {
            ui.label(None, &format!("fps: {}", macroquad::time::get_fps()));
            ui.label(None, "distance weight");
            ui.slider(hash!(), "", 0.0..1.0, &mut self.big_cell.distance_weight);

            ui.label(None, "size weight");
            ui.slider(hash!(), "", 0.0..1.0, &mut self.big_cell.size_weight);
        });
    }
}
