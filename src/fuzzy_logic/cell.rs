use crate::shared::MAIN_COLOR;

use super::utils::get_random_vector;
use macroquad::{prelude::*, shapes::draw_circle_lines, text::draw_text};

const VELOCITY: f32 = 30.0;

#[derive(Clone, Copy)]
pub struct BigCell {
    position: Vec2,
    velocity: Vec2,
    target: Vec2,
    color: Color,
    size: f32,
    pub distance_weight: f32,
    pub distance_factor: f32,
    pub size_weight: f32,
    pub size_factor: f32,
}

impl BigCell {
    pub fn new(size: f32, color: Color) -> Self {
        let position = get_random_vector();
        let target = get_random_vector();
        let velocity = (target - position).normalize() * VELOCITY;
        // println!("{}, {}, {}", position, target, velocity);
        Self {
            position,
            velocity,
            target,
            color,
            size,
            distance_weight: 1.0,
            distance_factor: 0.0,
            size_weight: 1.0,
            size_factor: 0.0,
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
        let mut target_id = 0;
        for i in 0..10 {
            let prio = self.distance_weight * closeness[i] + self.size_weight * sizes[i];
            if prio > temp {
                self.distance_factor = closeness[i];
                self.size_factor = sizes[i];
                target_id = i;
                temp = prio;
            }
        }

        // println!("target {}", targetId);
        self.target = cells[target_id].get_position();
        self.velocity = (self.target - self.position).normalize() * VELOCITY * 0.9;
        self.position += self.velocity * delta_time;

        if (self.target - self.position).length_squared() < 1.0 {
            cells[target_id].die();
        }
    }

    pub fn draw(&self) {
        let (x, y) = (self.position.x, self.position.y);
        draw_circle_lines(x, y, self.size, 5.0, self.color);
        draw_circle_lines(self.target.x, self.target.y, 5.0, 3.0, MAIN_COLOR);
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
        let velocity = (target - position).normalize() * VELOCITY;
        // println!("{}, {}, {}", position, target, velocity);
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
                self.velocity = (self.target - self.position).normalize() * VELOCITY;
            }
        }
    }

    pub fn draw(&self) {
        let (x, y) = (self.position.x, self.position.y);
        draw_text(
            &format!("{}|{}", self.id, self.size),
            x - 12.0,
            y + self.size + 12.0,
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
