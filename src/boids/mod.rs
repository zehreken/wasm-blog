use macroquad::prelude::*;

use crate::boids::boid::Boid;

mod boid;

pub fn get_config() -> Conf {
    Conf {
        window_title: "Boids".to_owned(),
        window_width: 512,
        window_height: 512,
        fullscreen: false,
        ..Default::default()
    }
}

struct App {
    boids: Vec<Boid>,
}

impl App {
    fn new(width: f32, height: f32) -> Self {
        Self {
            boids: vec![Boid::new()],
        }
    }

    fn update(&mut self) {}

    fn draw(&self) {
        for boid in &self.boids {
            boid.draw();
        }
    }

    fn resize(width: f32, height: f32) {}
}

pub async fn run() {
    let mut app = App::new(screen_width(), screen_height());

    loop {
        clear_background(WHITE);

        app.update();

        app.draw();

        draw_text(
            &format!("fps: {}", macroquad::time::get_fps()),
            2.0,
            12.0,
            16.0,
            PINK,
        );

        next_frame().await
    }
}
