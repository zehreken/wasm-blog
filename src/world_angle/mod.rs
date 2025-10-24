use std::f32::consts::PI;

use crate::{app::App, world_angle::config::COORDS};
use macroquad::{
    math::vec3,
    prelude::*,
    ui::{hash, root_ui, widgets},
};

mod config;

pub fn get_title() -> String {
    return "World Angle".to_owned();
}

pub struct WorldAngle {
    coord_a: Coord,
    coord_b: Coord,
    earth_texture: Texture2D,
}

struct Coord {
    latitude: f32,  // + North, - South
    longitude: f32, // + East, - West
}

impl Coord {
    pub fn new(latitude: f32, longitude: f32) -> Self {
        Self {
            latitude: latitude * PI / 180.0,
            longitude: longitude * PI / 180.0,
        }
    }
}

impl WorldAngle {
    pub async fn new() -> Self {
        let earth_texture = load_texture("earth.png").await.unwrap();
        Self {
            // Stockholm
            coord_a: Coord::new(59.3327, -18.0656),
            // Ankara
            coord_b: Coord::new(39.9334, -32.8597),
            earth_texture,
        }
    }
}

impl App for WorldAngle {
    fn update(&mut self) {}

    fn draw(&self) {
        let time = macroquad::time::get_time() as f32 / 2.0;
        let position = vec3(40.0 * time.cos(), 0.0, 40.0 * time.sin());

        set_camera(&Camera3D {
            position,
            up: vec3(0.0, 1.0, 0.0),
            target: vec3(0.0, 0.0, 0.0),
            ..Default::default()
        });
        let center = vec3(0.0, 0.0, 0.0);
        draw_sphere(center, 10.0, Some(&self.earth_texture), WHITE);

        let r = 0.5;
        for coord in COORDS {
            let pos = coord_to_point_on_sphere(&coord, 10.0);
            draw_sphere(pos, r, None, GREEN);
        }

        set_default_camera();

        widgets::Window::new(
            hash!(),
            vec2(screen_width() / 2.0, screen_height() / 2.0),
            vec2(200.0, 140.0),
        )
        .label("Controls")
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
            let mut city_a: usize = 0;
            ui.combo_box(hash!(), "City A", config::CITIES, &mut city_a);
            let mut city_b: usize = 0;
            ui.combo_box(hash!(), "City B", config::CITIES, &mut city_b);
            if ui.button(None, "Reset") {
            }
            let mut data = "Drag and drop start and end\ncells. Click any cell to\ntoggle blocked.\nDrag this window.".to_string();
            ui.editbox(hash!(), vec2(194.0, 80.0), &mut data);
        });
    }

    fn resize(&mut self, width: f32, height: f32) {}
}

fn coord_to_point_on_sphere(coord: &Coord, radius: f32) -> Vec3 {
    let x = radius * coord.latitude.cos() * coord.longitude.cos();
    let y = radius * coord.latitude.sin();
    let z = radius * coord.latitude.cos() * coord.longitude.sin();

    return vec3(x, y, z);
}
