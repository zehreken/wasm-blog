use macroquad::{
    color::RED,
    math::{Vec2, vec2},
    shapes::draw_circle,
};

pub struct Boid {
    position: Vec2,
}

impl Boid {
    pub fn new() -> Self {
        Self {
            position: vec2(100.0, 100.0),
        }
    }

    pub fn update(&mut self) {}

    pub fn draw(&self) {
        let (x, y) = (self.position.x, self.position.y);
        draw_circle(x, y, 10.0, RED);
    }
}
