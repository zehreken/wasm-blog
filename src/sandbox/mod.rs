use crate::{
    app::App,
    sandbox::particle::{Particle, ParticleModel, Rock, Sand, Water},
};
use config::*;
use macroquad::{
    color::{BLACK, Color, RED, WHITE},
    input::{
        KeyCode, MouseButton, is_key_pressed, is_mouse_button_down, is_mouse_button_pressed,
        mouse_position,
    },
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
            image: Image::gen_image_color(column_count as u16, row_count as u16, RED),
            particle_id: 0,
        }
    }
}

impl App for Sandbox {
    fn update(&mut self) {
        self.particle_model.update();

        if is_key_pressed(KeyCode::Key1) {
            self.particle_id = 0;
        }
        if is_key_pressed(KeyCode::Key2) {
            self.particle_id = 1;
        }
        if is_key_pressed(KeyCode::Key3) {
            self.particle_id = 2;
        }

        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();
            let row = y as u32 / PARTICLE_SIZE;
            let column = x as u32 / PARTICLE_SIZE;

            let index = row * self.column_count + column;
            self.particle_model._particles[index as usize] = match self.particle_id {
                0 => Some(Box::new(Sand::new())),
                1 => Some(Box::new(Water::new())),
                2 => Some(Box::new(Rock::new())),
                _ => None,
            };
        }

        for row in 0..self.row_count {
            for column in 0..self.column_count {
                let index = row * self.column_count + column;
                if let Some(p) = &self.particle_model._particles[index as usize] {
                    self.image
                        .set_pixel(column, row, Color::from_hex(p.get_properties().color));
                } else {
                    self.image.set_pixel(column, row, WHITE);
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
    }

    fn resize(&mut self, width: f32, height: f32) {
        self.row_count = height as u32 / PARTICLE_SIZE;
        self.column_count = width as u32 / PARTICLE_SIZE;
    }
}
