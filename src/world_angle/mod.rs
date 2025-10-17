use crate::app::App;
use macroquad::{math::vec3, prelude::*};

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

impl WorldAngle {
    pub async fn new() -> Self {
        let earth_texture = load_texture("earth.png").await.unwrap();
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
            earth_texture,
        }
    }
}

impl App for WorldAngle {
    fn update(&mut self) {}

    fn draw(&self) {
        let time = macroquad::time::get_time() as f32;
        let position = vec3(40.0 * time.cos(), 0.0, 40.0 * time.sin());

        set_camera(&Camera3D {
            position,
            up: vec3(0.0, 1.0, 0.0),
            target: vec3(0.0, 0.0, 0.0),
            ..Default::default()
        });
        let center = vec3(0.0, 0.0, 0.0);
        draw_sphere(center, 10.0, Some(&self.earth_texture), WHITE);

        set_default_camera();
    }

    fn resize(&mut self, width: f32, height: f32) {}
}
