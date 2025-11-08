use super::cell::*;
use crate::shared::MAIN_COLOR;
use macroquad::prelude::*;

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
            big_cell: BigCell::new(50_f32, MAIN_COLOR),
        }
    }

    pub fn update(&mut self) {
        let delta_time = macroquad::time::get_frame_time();
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

        egui_macroquad::draw();
    }

    pub fn ui(&mut self) {
        draw_text(
            &format!(
                "{:.2} {:.2} {:.2} {:.2}",
                self.big_cell.distance_weight,
                self.big_cell.distance_factor,
                self.big_cell.size_weight,
                self.big_cell.size_factor,
            ),
            200.0,
            12.0,
            16.0,
            BLACK,
        );
        egui_macroquad::ui(|ctx| {
            ctx.set_theme(egui::Theme::Light);
            ctx.style_mut(|style| style.visuals.window_shadow = egui::Shadow::NONE);
            egui::Window::new("Controls").show(ctx, |ui| {
                ui.label("distance weight");
                ui.add(egui::Slider::new(&mut self.big_cell.distance_weight, 0.0..=1.0).text(""));
                ui.label("size weight");
                ui.add(egui::Slider::new(&mut self.big_cell.size_weight, 0.0..=1.0).text(""));
            });
        });
    }
}
