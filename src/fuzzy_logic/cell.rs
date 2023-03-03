use super::utils::{self, get_random_vector};
use macroquad::{
    prelude::{vec2, Color, Vec2},
    shapes::draw_circle,
};

#[derive(Clone, Copy)]
pub struct Cell {
    position: Vec2,
    velocity: Vec2,
    target: Vec2,
    size: f32,
}

impl Cell {
    pub fn new(size: f32) -> Self {
        let position = get_random_vector();
        let target = get_random_vector();
        let velocity = (target - position).normalize() * 30.0;
        println!("{}, {}, {}", position, target, velocity);
        Self {
            position,
            velocity,
            target,
            size,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.position += self.velocity * delta_time;
        if (self.target - self.position).length_squared() < 1.0 {
            self.target = get_random_vector();
            self.velocity = (self.target - self.position).normalize() * 30.0;
        }
    }

    pub fn draw(&self) {
        let (x, y) = (self.position.x, self.position.y);
        draw_circle(x, y, self.size, Color::new(255.0, 0.0, 0.0, 255.0))
    }
}
