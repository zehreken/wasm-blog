use macroquad::{
    color::{self, BLACK, BLUE, Color, GREEN, RED},
    input::mouse_position,
    math::{Vec2, vec2},
    shapes::{draw_circle, draw_circle_lines, draw_line},
    window::{screen_height, screen_width},
};

use crate::{boids::config::BOID_SPEED, shared::MAIN_COLOR};

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

        let side_factor = 5.0 * delta_time;
        let target_factor = 5.0 * delta_time;
        let cohesion_factor = 0.0 * delta_time;
        let separation_factor = 0.0 * delta_time;

        let pointer_diff =
            (vec2(mouse_position().0, mouse_position().1) - self.position).normalize();
        let cohesion_diff = (self.cohesion_target - self.position).normalize();
        let separation_diff = (self.separation_target - self.position).normalize();
        self.direction += pointer_diff * 10.0 + cohesion_diff * 0.1 - separation_diff * 5.0;

        // self.direction += s * 0.01;
        self.direction = self.direction.clamp_length(self.direction.length(), 200.0);

        self.position += self.direction * BOID_SPEED * delta_time;
    }

    pub fn draw(&self) {
        let (x, y) = (self.position.x, self.position.y);
        draw_circle(x, y, 6.0, MAIN_COLOR);
        let end = self.position + self.direction.normalize() * 10.0;

        draw_line(x, y, end.x, end.y, 2.0, BLACK);
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
