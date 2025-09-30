use std::f32::consts::PI;

use macroquad::{
    color::BLACK,
    math::{Vec3, vec3},
    models::draw_sphere_wires,
    prelude::*,
    window::{screen_height, screen_width},
};

use crate::app::App;

pub fn get_title() -> String {
    return "World Angle".to_owned();
}

pub struct WorldAngle {
    coord_a: Coord,
    coord_b: Coord,
}

struct Coord {
    latitude: f32,  // + North, - South
    longitude: f32, // + East, - West
}

impl WorldAngle {
    pub fn new() -> Self {
        Self {
            // Stockholm
            coord_a: Coord {
                latitude: 59.3327,
                longitude: 18.0656,
            },
            // Ankara
            coord_b: Coord {
                latitude: 39.9334,
                longitude: 32.8597,
            },
        }
    }
}

impl App for WorldAngle {
    fn update(&mut self) {}

    fn draw(&self) {
        let position = vec3(0.0, 0.0, -50.0);
        let yaw: f32 = PI;
        let pitch: f32 = 0.0;
        let front = vec3(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        )
        .normalize();

        set_camera(&Camera3D {
            position,
            up: vec3(0.0, 1.0, 0.0),
            target: vec3(0.0, 0.0, 0.0),
            ..Default::default()
        });
        let center = vec3(0.0, 0.0, 0.0);
        draw_sphere_wires(center, 10.0, None, BLACK);

        set_default_camera();
    }

    fn resize(&mut self, width: f32, height: f32) {}
}
