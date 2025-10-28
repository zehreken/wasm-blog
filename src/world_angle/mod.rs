use std::f32::consts::PI;

use crate::{
    app::App,
    world_angle::config::{COLORS, COORDS, LATITUDE_OFFSET, LONGITUDE_OFFSET},
};
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
    angle: f32,
    earth_texture: Texture2D,
}

struct Coord {
    latitude: f32,  // + North, - South
    longitude: f32, // + East, - West
}

impl Coord {
    pub fn offset_radian(&self) -> Vec2 {
        vec2(
            (self.latitude + LATITUDE_OFFSET) * PI / 180.0,
            (self.longitude + LONGITUDE_OFFSET) * PI / 180.0,
        )
    }
}

impl WorldAngle {
    pub async fn new() -> Self {
        let earth_texture = load_texture("earth.png").await.unwrap();
        Self {
            // Stockholm
            coord_a: Coord {
                latitude: 59.3327,
                longitude: -18.0656,
            },
            // Ankara
            coord_b: Coord {
                latitude: 39.9334,
                longitude: -32.8597,
            },
            angle: 0.0,
            earth_texture,
        }
    }
}

impl App for WorldAngle {
    fn update(&mut self) {
        if is_key_down(KeyCode::A) {
            self.angle += 0.005;
        }
        if is_key_down(KeyCode::D) {
            self.angle -= 0.005;
        }
    }

    fn draw(&self) {
        let position = vec3(40.0 * self.angle.cos(), 0.0, 40.0 * self.angle.sin());

        set_camera(&Camera3D {
            position,
            up: vec3(0.0, 1.0, 0.0),
            target: vec3(0.0, 0.0, 0.0),
            ..Default::default()
        });
        let center = vec3(0.0, 0.0, 0.0);
        draw_sphere(center, 10.0, Some(&self.earth_texture), WHITE);

        let r = 0.2;
        let mut colors = COLORS.iter();
        for coord in COORDS {
            let pos = coord_to_point_on_sphere(&coord.offset_radian(), 10.0);
            draw_sphere(pos, r, None, *colors.next().unwrap());
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

fn coord_to_point_on_sphere(coord: &Vec2, radius: f32) -> Vec3 {
    let x = radius * coord.x.cos() * coord.y.cos();
    let y = radius * coord.x.sin();
    let z = radius * coord.x.cos() * coord.y.sin();

    return vec3(x, y, z);
}
