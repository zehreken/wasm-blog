use crate::{app::App, shared::MAIN_COLOR};
use macroquad::{
    color::BLACK,
    input::{self, KeyCode},
    math::vec2,
    shapes::{draw_circle, draw_line},
    text::draw_text,
};

mod matrix;

pub fn get_title() -> String {
    return "Perceptron".to_owned();
}

pub struct Perceptron {
    learning_rate: f32,
    threshold: f32,
    input_1: u8,
    weight_1_1: f32,
    input_2: u8,
    weight_1_2: f32,
    output: u8,
    desired_output: u8,
}

impl Perceptron {
    pub fn new() -> Self {
        Self {
            learning_rate: 0.1,
            threshold: 0.5,
            input_1: 1,
            weight_1_1: 0.1,
            input_2: 0,
            weight_1_2: 0.1,
            output: 0,
            desired_output: 0,
        }
    }
}

impl App for Perceptron {
    fn update(&mut self) {
        if input::is_key_pressed(KeyCode::A) {
            calculate(self);
        }

        egui_macroquad::ui(|ctx| {
            ctx.set_theme(egui::Theme::Light);
            ctx.style_mut(|style| style.visuals.window_shadow = egui::Shadow::NONE);
            egui::Window::new("Perceptron")
                .resizable(false)
                .max_width(200.0)
                .show(ctx, |ui| {
                    egui::ComboBox::from_label("Input 1")
                        .selected_text(format!("{}", self.input_1))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.input_1, 0, "0");
                            ui.selectable_value(&mut self.input_1, 1, "1");
                        });
                    egui::ComboBox::from_label("Input 2")
                        .selected_text(format!("{}", self.input_2))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.input_2, 0, "0");
                            ui.selectable_value(&mut self.input_2, 1, "1");
                        });
                    egui::ComboBox::from_label("Desired output")
                        .selected_text(format!("{}", self.desired_output))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.desired_output, 0, "0");
                            ui.selectable_value(&mut self.desired_output, 1, "1");
                        });
                    if ui.button(">>|").clicked() {
                        calculate(self);
                    }
                });
        });
    }

    fn draw(&self) {
        let pos_1 = vec2(200.0, 200.0);
        let pos_2 = vec2(200.0, 500.0);
        let pos_3 = vec2(500.0, 350.0);

        draw_line(pos_1.x, pos_1.y, pos_3.x, pos_3.y, 5.0, BLACK);
        draw_line(pos_2.x, pos_2.y, pos_3.x, pos_3.y, 5.0, BLACK);

        draw_circle(pos_1.x, pos_1.y, 50.0, MAIN_COLOR);
        let text = &format!("{}", self.input_1);
        draw_text(text, pos_1.x, pos_1.y, 40.0, BLACK);

        draw_circle(pos_2.x, pos_2.y, 50.0, MAIN_COLOR);
        let text = &format!("{}", self.input_2);
        draw_text(text, pos_2.x, pos_2.y, 40.0, BLACK);

        draw_circle(pos_3.x, pos_3.y, 50.0, MAIN_COLOR);
        let text = &format!("{}", self.output);
        draw_text(text, pos_3.x, pos_3.y, 40.0, BLACK);

        draw_text(
            &format!("{}", self.weight_1_1),
            pos_1.x / 2.0 + pos_3.x / 2.0,
            pos_1.y / 2.0 + pos_3.y / 2.0,
            40.0,
            BLACK,
        );
        draw_text(
            &format!("{}", self.weight_1_2),
            pos_2.x / 2.0 + pos_3.x / 2.0,
            pos_2.y / 2.0 + pos_3.y / 2.0,
            40.0,
            BLACK,
        );

        egui_macroquad::draw();
    }

    fn resize(&mut self, width: f32, height: f32) {}
}

fn calculate(p: &mut Perceptron) {
    if p.desired_output == 0 || (p.input_1 == 0 && p.input_2 == 0) {
        return;
    }
    let mut sum = p.input_1 as f32 * p.weight_1_1 + p.input_2 as f32 * p.weight_1_2;
    while sum < p.threshold {
        sum = p.input_1 as f32 * p.weight_1_1 + p.input_2 as f32 * p.weight_1_2;
        if p.input_1 > 0 {
            p.weight_1_1 += (p.desired_output - p.output) as f32 * p.learning_rate;
        }
        if p.input_2 > 0 {
            p.weight_1_2 += (p.desired_output - p.output) as f32 * p.learning_rate;
        }
        p.output = 0;
    }
    p.output = 1;
}
