use crate::boids::{boid::Boid, config::*};
use macroquad::{prelude::*, rand::gen_range};

mod boid;
mod config;

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
    boids: [Boid; BOIDS_COUNT],
}

impl App {
    fn new(width: f32, height: f32) -> Self {
        let mut boids: [Boid; BOIDS_COUNT] = [Boid::new(); BOIDS_COUNT];
        for i in 0..BOIDS_COUNT {
            boids[i].position = vec2(gen_range(0.0, 500.0), gen_range(0.0, 500.0));
            boids[i].direction = vec2(gen_range(-1.0, 1.0), gen_range(-1.0, 1.0));
        }

        Self { boids }
    }

    fn update(&mut self) {
        calculate_targets(&mut self.boids);

        for boid in &mut self.boids {
            boid.update();
        }
    }

    fn draw(&self) {
        for i in 0..BOIDS_COUNT {
            self.boids[i].draw();
            if i == 0 {
                self.boids[i].draw_target();
            }
        }
    }

    fn resize(width: f32, height: f32) {}
}

fn calculate_targets(boids: &mut [Boid; BOIDS_COUNT]) {
    // cohesion rule and separation rule can be combined
    // do it in one nested for loop
    // I think the fastest would be doing a nested for loop
    // but starting the inner one from i+1
    let mut cohesion_distances: [(Vec2, i8); BOIDS_COUNT] = [(vec2(0.0, 0.0), 0); BOIDS_COUNT];
    let mut separation_distances: [(Vec2, i8); BOIDS_COUNT] = [(vec2(0.0, 0.0), 0); BOIDS_COUNT];
    for i in 0..boids.len() - 1 {
        for j in i + 1..boids.len() {
            if (boids[i].position - boids[j].position).length_squared() < COHESION_RANGE_SQ {
                cohesion_distances[i].0 += boids[j].position;
                cohesion_distances[i].1 += 1;

                cohesion_distances[j].0 += boids[i].position;
                cohesion_distances[j].1 += 1;
            }
            if (boids[i].position - boids[j].position).length_squared() < SEPARATION_RANGE_SQ {
                separation_distances[i].0 += boids[j].position;
                separation_distances[i].1 += 1;

                separation_distances[j].0 += boids[i].position;
                separation_distances[j].1 += 1;
            }
        }
    }

    for i in 0..BOIDS_COUNT {
        let cohesion_target = if cohesion_distances[i].1 == 0 {
            boids[i].position
        } else {
            cohesion_distances[i].0 / cohesion_distances[i].1 as f32
        };
        let separation_target = if separation_distances[i].1 == 0 {
            boids[i].position
        } else {
            separation_distances[i].0 / separation_distances[i].1 as f32
        };
        boids[i].cohesion_target = cohesion_target;
        boids[i].separation_target = separation_target;
    }
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
