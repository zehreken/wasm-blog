use super::utils::get_random_vector;
use macroquad::{
    prelude::*,
    shapes::{draw_circle, draw_circle_lines},
    text::draw_text,
};

#[derive(Clone, Copy)]
pub struct BigCell {
    position: Vec2,
    velocity: Vec2,
    target: Vec2,
    color: Color,
    size: f32,
    distance_weight: f32,
    size_weight: f32,
}

impl BigCell {
    pub fn new(size: f32, color: Color) -> Self {
        let position = get_random_vector();
        let target = get_random_vector();
        let velocity = (target - position).normalize() * 30.0;
        // println!("{}, {}, {}", position, target, velocity);
        Self {
            position,
            velocity,
            target,
            color,
            size,
            distance_weight: 1.0,
            size_weight: 1.0,
        }
    }

    pub fn update(&mut self, delta_time: f32, cells: &mut Vec<Cell>) {
        let mut closeness = [0.0; 10];
        let mut sizes = [0.0; 10];

        for i in 0..10 {
            if !cells[i].is_alive() {
                continue;
            }
            let distance = (self.position - cells[i].get_position()).length();
            closeness[i] = if distance > 300.0 {
                0.0
            } else {
                1.0 - distance / 250.0
            };

            let mut size_diff = (self.size - cells[i].get_size()).abs();
            size_diff /= self.size;

            sizes[i] = 1.0 - size_diff;
        }

        let mut temp = 0.0;
        let mut targetId = 0;
        for i in 0..10 {
            let prio = self.distance_weight * closeness[i] + self.size_weight * sizes[i];
            if prio > temp {
                targetId = i;
                temp = prio;
            }
        }

        // println!("target {}", targetId);
        self.target = cells[targetId].get_position();
        self.velocity = (self.target - self.position).normalize() * 30.0;
        self.position += self.velocity * delta_time;

        if (self.target - self.position).length_squared() < 1.0 {
            cells[targetId].die();
        }
    }

    pub fn draw(&self) {
        let (x, y) = (self.position.x, self.position.y);
        draw_circle_lines(x, y, self.size, 3.0, self.color);
    }

    pub fn get_distance_weight(&self) -> f32 {
        self.distance_weight
    }

    pub fn set_distance_weight(&mut self, value: f32) {
        self.distance_weight += value;
    }

    pub fn get_size_weight(&self) -> f32 {
        self.size_weight
    }

    pub fn set_size_weight(&mut self, value: f32) {
        self.size_weight += value;
    }
}

#[derive(Clone, Copy)]
pub struct Cell {
    position: Vec2,
    velocity: Vec2,
    target: Vec2,
    color: Color,
    size: f32,
    id: i8,
    is_alive: bool,
}

impl Cell {
    pub fn new(id: i8, color: Color) -> Self {
        let position = get_random_vector();
        let target = get_random_vector();
        let velocity = (target - position).normalize() * 30.0;
        // println!("{}, {}, {}", position, target, velocisty);
        Self {
            position,
            velocity,
            target,
            color,
            size: (macroquad::rand::rand() % 20) as f32 + 10.0,
            id,
            is_alive: true,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.is_alive {
            self.position += self.velocity * delta_time;
            if (self.target - self.position).length_squared() < 1.0 {
                self.target = get_random_vector();
                self.velocity = (self.target - self.position).normalize() * 10.0;
            }
        }
    }

    pub fn draw(&self) {
        let (x, y) = (self.position.x, self.position.y);
        draw_text(
            &format!("{}, {}", self.id, self.size),
            x - 4.0,
            y + 4.0,
            16.0,
            RED,
        );
        draw_circle_lines(x, y, self.size, 3.0, self.color);
    }

    pub fn die(&mut self) {
        self.is_alive = false;
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn get_size(&self) -> f32 {
        self.size
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }
}
