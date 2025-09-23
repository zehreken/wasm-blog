use macroquad::prelude::*;
mod config;
use config::*;

use crate::app::App;

pub fn get_title() -> String {
    return "Cycle".to_owned();
}

pub struct Cycle {
    rotation: f32,
}

impl Cycle {
    pub fn new() -> Self {
        Self { rotation: 0.0 }
    }
}

impl App for Cycle {
    fn update(&mut self) {
        self.rotation = 10.0 * macroquad::time::get_time() as f32;
    }

    fn draw(&self) {
        let gear_ratio = FRONT_GEAR_SIZE / REAR_GEAR_SIZE;
        draw_wheel(200.0, 256.0, 100.0, self.rotation / REAR_GEAR_SIZE);

        draw_circle_with_line(200.0, 256.0, REAR_GEAR_SIZE, self.rotation / REAR_GEAR_SIZE);

        draw_circle_with_line(
            400.0,
            256.0,
            FRONT_GEAR_SIZE,
            self.rotation / FRONT_GEAR_SIZE,
        );
    }

    fn resize(&mut self, width: f32, height: f32) {}
}

fn draw_wheel(x: f32, y: f32, radius: f32, rotation: f32) {
    draw_poly_lines(x, y, 30, radius, rotation, 1.0, GRAY);

    let spoke_count = 24;

    for i in 0..spoke_count {
        let t = i as f32 * 2.0 * std::f32::consts::PI / spoke_count as f32;
        let x2 = x + radius * (rotation + t).cos();
        let y2 = y + radius * (rotation + t).sin();
        draw_line(x, y, x2, y2, 1.0, GRAY);
    }
}

fn draw_circle_with_line(x: f32, y: f32, radius: f32, rotation: f32) {
    draw_poly_lines(x, y, 30, radius, rotation, 1.0, BLACK);

    let x2 = x + radius * rotation.cos();
    let y2 = y + radius * rotation.sin();
    draw_line(x, y, x2, y2, 1.0, PINK);
}
