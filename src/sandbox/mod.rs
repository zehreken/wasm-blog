use crate::{
    app::App,
    sandbox::particle::{Particle, ParticleModel, Rock, Sand, Water},
    shared::BG_COLOR,
};
use config::*;
use macroquad::{
    color::{Color, WHITE},
    input::{KeyCode, MouseButton, is_key_pressed, is_mouse_button_down, mouse_position},
    math::vec2,
    texture::{DrawTextureParams, FilterMode, Image, Texture2D, draw_texture_ex},
    window::{screen_height, screen_width},
};

mod config;
mod particle;

pub fn get_title() -> String {
    return "Sandbox".to_owned();
}

pub struct Sandbox {
    row_count: u32,
    column_count: u32,
    particle_model: ParticleModel,
    image: Image,
    brush_id: BrushId,
    brush_size: u8,
    particle_id: u8,
}

impl Sandbox {
    pub fn new(width: f32, height: f32) -> Self {
        let row_count = height as u32 / PARTICLE_SIZE;
        let column_count = width as u32 / PARTICLE_SIZE;
        Self {
            row_count: height as u32 / PARTICLE_SIZE,
            column_count: width as u32 / PARTICLE_SIZE,
            particle_model: ParticleModel::new(column_count, row_count),
            image: Image::gen_image_color(column_count as u16, row_count as u16, WHITE),
            brush_id: BrushId::_1,
            brush_size: 0,
            particle_id: 0,
        }
    }
}

impl App for Sandbox {
    fn update(&mut self) {
        self.particle_model.update();

        for row in 0..self.row_count {
            for column in 0..self.column_count {
                let index = row * self.column_count + column;
                if let Some(p) = &self.particle_model._particles[index as usize] {
                    self.image
                        .set_pixel(column, row, Color::from_hex(p.get_properties().color()));
                } else {
                    self.image.set_pixel(column, row, BG_COLOR);
                }
            }
        }

        if is_key_pressed(KeyCode::Key1) {
            self.particle_id = 0;
        }
        if is_key_pressed(KeyCode::Key2) {
            self.particle_id = 1;
        }
        if is_key_pressed(KeyCode::Key3) {
            self.particle_id = 2;
        }

        let mut egui_wants_mouse = false;
        egui_macroquad::ui(|ctx| {
            egui_wants_mouse = ctx.wants_pointer_input();
            ctx.set_theme(egui::Theme::Light);
            ctx.style_mut(|style| style.visuals.window_shadow = egui::Shadow::NONE);
            egui::Window::new("Controls")
                .resizable(false)
                .max_width(200.0)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Sand").clicked() {
                            self.particle_id = 0;
                        }
                        if ui.button("Water").clicked() {
                            self.particle_id = 1;
                        }
                        if ui.button("Rock").clicked() {
                            self.particle_id = 2;
                        }
                        if ui.button("Erase").clicked() {
                            self.particle_id = u8::MAX;
                        }
                        if ui.button("Clear").clicked() {
                            self.particle_model.clear();
                        }
                    });
                    // ui..combo_box(hash!(), "Brush size", BRUSH_LABELS, &mut self.brush_id);
                    egui::ComboBox::from_label("Brush size")
                        .selected_text(format!("{:?}", self.brush_id))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.brush_id, BrushId::_1, "1");
                            ui.selectable_value(&mut self.brush_id, BrushId::_2, "2");
                            ui.selectable_value(&mut self.brush_id, BrushId::_4, "4");
                            ui.selectable_value(&mut self.brush_id, BrushId::_8, "8");
                            ui.selectable_value(&mut self.brush_id, BrushId::_16, "16");
                        });
                    self.brush_size = BRUSH_SIZES[get_brush_index(&self.brush_id)];
                });
        });

        // Prevents mouse clicks going through the ui
        if egui_wants_mouse {
            return;
        }

        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();
            let row = y as u32 / PARTICLE_SIZE;
            let column = x as u32 / PARTICLE_SIZE;

            let half_brush_size = (self.brush_size / 2) as u32;
            let row_start = row - half_brush_size;
            let row_range = row_start..row_start + self.brush_size as u32;
            let column_start = column - half_brush_size;
            let column_range = column_start..column_start + self.brush_size as u32;

            for r in row_range {
                for c in column_range.clone() {
                    if r >= 0 && r < self.row_count && c >= 0 && c < self.column_count {
                        let index = r * self.column_count + c;
                        self.particle_model._particles[index as usize] = match self.particle_id {
                            0 => Some(Box::new(Sand::new())),
                            1 => Some(Box::new(Water::new())),
                            2 => Some(Box::new(Rock::new())),
                            _ => None,
                        };
                    }
                }
            }
        }
    }

    fn draw(&self) {
        let texture = Texture2D::from_image(&self.image);
        texture.set_filter(FilterMode::Nearest);

        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        egui_macroquad::draw();
    }

    fn resize(&mut self, width: f32, height: f32) {
        self.row_count = height as u32 / PARTICLE_SIZE;
        self.column_count = width as u32 / PARTICLE_SIZE;
        self.particle_model = ParticleModel::new(self.column_count, self.row_count);
        self.image = Image::gen_image_color(self.column_count as u16, self.row_count as u16, WHITE);
    }
}
