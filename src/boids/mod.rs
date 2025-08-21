use macroquad::prelude::*;

pub fn get_config() -> Conf {
    Conf {
        window_title: "Boids".to_owned(),
        window_width: 512,
        window_height: 512,
        fullscreen: false,
        ..Default::default()
    }
}

struct App {}

impl App {
    fn new(width: f32, height: f32) -> Self {
        Self {}
    }
}
