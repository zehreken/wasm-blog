use crate::{
    app::App,
    boids::{boid::Boid, config::*},
};
use macroquad::{input, prelude::*, rand::gen_range};

mod boid;
mod config;

pub fn get_title() -> String {
    return "Boids".to_owned();
}

pub struct Boids {
    boids: [Boid; BOIDS_COUNT],
    is_paused: bool,
}

impl Boids {
    pub fn new() -> Self {
        let mut boids: [Boid; BOIDS_COUNT] = [Boid::new(); BOIDS_COUNT];
        for i in 0..BOIDS_COUNT {
            boids[i].position = vec2(gen_range(0.0, 500.0), gen_range(0.0, 500.0));
            boids[i].direction = vec2(gen_range(-1.0, 1.0), gen_range(-1.0, 1.0));
        }

        Self {
            boids,
            is_paused: false,
        }
    }
}

impl App for Boids {
    fn update(&mut self) {
        if input::is_key_pressed(KeyCode::P) {
            self.is_paused = !self.is_paused;
        }
        if input::is_key_pressed(KeyCode::U) {
            calculate_targets(&mut self.boids);

            for boid in &mut self.boids {
                boid.update();
            }
        }
        if self.is_paused {
            return;
        }
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

    fn resize(&mut self, width: f32, height: f32) {}
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
            Vec2::ZERO
        } else {
            cohesion_distances[i].0 / cohesion_distances[i].1 as f32
        };
        let separation_target = if separation_distances[i].1 == 0 {
            Vec2::ZERO
        } else {
            separation_distances[i].0 / separation_distances[i].1 as f32
        };
        boids[i].cohesion_target = cohesion_target;
        boids[i].separation_target = separation_target;
    }
}
