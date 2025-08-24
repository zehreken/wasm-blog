use macroquad::{
    color::{BLUE, GREEN, RED},
    input::mouse_position,
    math::{Vec2, vec2},
    shapes::{draw_circle, draw_circle_lines, draw_line},
    window::{screen_height, screen_width},
};

use crate::boids::config::BOID_SPEED;

#[derive(Clone, Copy)]
pub struct Boid {
    pub position: Vec2,
    pub direction: Vec2,
    pub cohesion_target: Vec2,
    pub separation_target: Vec2,
}

impl Boid {
    pub fn new() -> Self {
        Self {
            position: vec2(100.0, 100.0),
            direction: vec2(0.0, 0.0),
            cohesion_target: vec2(0.0, 0.0),
            separation_target: vec2(0.0, 0.0),
        }
    }

    pub fn update(&mut self) {
        let delta_time = macroquad::time::get_frame_time();

        if self.position.x < 10.0 {
            self.direction.x += 0.1;
        } else if self.position.x > screen_width() - 10.0 {
            self.direction.x -= 0.1;
        }
        if self.position.y < 10.0 {
            self.direction.y += 0.1;
        } else if self.position.y > screen_height() - 10.0 {
            self.direction.y -= 0.1;
        }

        self.position += self.direction * BOID_SPEED * delta_time;
    }

    pub fn draw(&self) {
        let (x, y) = (self.position.x, self.position.y);
        draw_circle(x, y, 10.0, RED);
    }

    pub fn draw_target(&self) {
        draw_circle(self.position.x, self.position.y, 5.0, BLUE);
        draw_circle_lines(
            self.cohesion_target.x,
            self.cohesion_target.y,
            4.0,
            2.0,
            GREEN,
        );
    }
}
