use crate::{
    app::App,
    proc_anim::config::{FEET_CIRCLE_RADIUS, RANGE},
    shared::MAIN_COLOR,
};
use macroquad::{input, prelude::*, time};
use std::f32::{self, consts::PI};

mod config;

pub fn get_title() -> String {
    return "Proc Anim".to_owned();
}

pub struct ProcAnim {
    nodes: Vec<Node>,
}

#[derive(Clone, Copy)]
struct Node {
    position: Vec2,
    direction: Vec2,
    left_foot_position: Vec2,
    right_foot_position: Vec2,
}

impl Node {
    fn new() -> Self {
        Self {
            position: Vec2::new(10.0, 10.0),
            direction: Vec2::ONE,
            left_foot_position: Vec2::ZERO,
            right_foot_position: Vec2::ZERO,
        }
    }

    fn update(&mut self) {
        let angle = self.direction.y.atan2(self.direction.x);
        // println!("{}", angle);
        if (self.left_foot_position - self.position).length_squared() > RANGE {
            let left_angle: f32 = angle - PI / 3.0;
            self.left_foot_position.x = self.position.x + left_angle.cos() * FEET_CIRCLE_RADIUS;
            self.left_foot_position.y = self.position.y + left_angle.sin() * FEET_CIRCLE_RADIUS;
        }

        if (self.right_foot_position - self.position).length_squared() > RANGE {
            let right_angle: f32 = angle + PI / 3.0;
            self.right_foot_position.x = self.position.x + right_angle.cos() * FEET_CIRCLE_RADIUS;
            self.right_foot_position.y = self.position.y + right_angle.sin() * FEET_CIRCLE_RADIUS;
        }
    }

    fn draw(&self) {
        draw_line(
            self.position.x,
            self.position.y,
            self.left_foot_position.x,
            self.left_foot_position.y,
            2.0,
            BLACK,
        );
        draw_line(
            self.position.x,
            self.position.y,
            self.right_foot_position.x,
            self.right_foot_position.y,
            2.0,
            BLACK,
        );
        draw_circle(
            self.left_foot_position.x,
            self.left_foot_position.y,
            3.0,
            BLACK,
        );
        draw_circle(
            self.right_foot_position.x,
            self.right_foot_position.y,
            3.0,
            BLACK,
        );

        draw_circle(self.position.x, self.position.y, 10.0, MAIN_COLOR);
        draw_line(
            self.position.x,
            self.position.y,
            self.position.x + self.direction.x * 10.0,
            self.position.y + self.direction.y * 10.0,
            2.0,
            BLACK,
        );
    }
}

impl ProcAnim {
    pub fn new() -> Self {
        let nodes = vec![Node::new(); 13];
        Self { nodes }
    }
}

impl App for ProcAnim {
    fn update(&mut self) {
        const SPEED: f32 = 200.0;
        let mouse_pos: Vec2 = input::mouse_position().into();

        let time = time::get_time() as f32 * 2.0;
        let mut cursor_pos = vec2(
            mouse_pos.x + time.cos() as f32 * 100.0,
            mouse_pos.y + time.sin() as f32 * 100.0,
        );
        for node in self.nodes.iter_mut().rev() {
            let diff = cursor_pos - node.position;
            node.direction = diff.normalize();
            if diff.length_squared() > 20.0 * 20.0 {
                node.position += diff.normalize() * SPEED * time::get_frame_time();
            }
            cursor_pos = node.position;

            node.update();
        }
    }

    fn draw(&self) {
        for node in &self.nodes {
            node.draw();
        }
    }

    fn resize(&mut self, _width: f32, _height: f32) {}
}
