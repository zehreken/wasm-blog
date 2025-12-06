use crate::{app::App, proc_anim::config::NODE_COUNT, shared::MAIN_COLOR};
use macroquad::{input, prelude::*, time};
use std::f32::{self, consts::PI};

mod config;

pub fn get_title() -> String {
    return "Proc Anim".to_owned();
}

pub struct ProcAnim {
    nodes: Vec<Node>,
    config: Config,
}

struct Config {
    speed: f32,
    node_count: usize,
    feet_radius: f32,
    joint_distance: f32,
}

#[derive(Clone, Copy)]
struct Node {
    position: Vec2,
    direction: Vec2,
    left_joint_position: Vec2,
    left_foot_position: Vec2,
    right_joint_position: Vec2,
    right_foot_position: Vec2,
}

impl Node {
    fn new() -> Self {
        Self {
            position: Vec2::new(10.0, 10.0),
            direction: Vec2::ONE,
            left_joint_position: Vec2::ZERO,
            left_foot_position: Vec2::ZERO,
            right_joint_position: Vec2::ZERO,
            right_foot_position: Vec2::ZERO,
        }
    }

    fn update(&mut self, config: &Config) {
        let angle = self.direction.y.atan2(self.direction.x);
        // println!("{}", angle);
        let range = config.feet_radius * config.feet_radius;
        let left_distance = (self.left_foot_position - self.position).length_squared();
        if left_distance > range {
            let left_angle: f32 = angle - PI / 3.0;
            self.left_foot_position.x = self.position.x + left_angle.cos() * config.feet_radius;
            self.left_foot_position.y = self.position.y + left_angle.sin() * config.feet_radius;
        }
        let left_angle = (self.position.y - self.left_foot_position.y)
            .atan2(self.position.x - self.left_foot_position.x)
            + PI / 2.0;
        self.left_joint_position = (self.position + self.left_foot_position) / 2.0
            + vec2(
                config.joint_distance * left_angle.cos(),
                config.joint_distance * left_angle.sin(),
            );

        let right_distance = (self.right_foot_position - self.position).length_squared();
        if right_distance > range {
            let right_angle: f32 = angle + PI / 3.0;
            self.right_foot_position.x = self.position.x + right_angle.cos() * config.feet_radius;
            self.right_foot_position.y = self.position.y + right_angle.sin() * config.feet_radius;
        }
        let right_angle = (self.position.y - self.right_foot_position.y)
            .atan2(self.position.x - self.right_foot_position.x)
            + PI / 2.0;
        self.right_joint_position = (self.position + self.right_foot_position) / 2.0
            - vec2(
                config.joint_distance * right_angle.cos(),
                config.joint_distance * right_angle.sin(),
            );
    }

    fn draw(&self) {
        draw_line(
            self.position.x,
            self.position.y,
            self.left_joint_position.x,
            self.left_joint_position.y,
            2.0,
            BLACK,
        );
        draw_line(
            self.left_joint_position.x,
            self.left_joint_position.y,
            self.left_foot_position.x,
            self.left_foot_position.y,
            2.0,
            BLACK,
        );
        draw_line(
            self.position.x,
            self.position.y,
            self.right_joint_position.x,
            self.right_joint_position.y,
            2.0,
            BLACK,
        );
        draw_line(
            self.right_joint_position.x,
            self.right_joint_position.y,
            self.right_foot_position.x,
            self.right_foot_position.y,
            2.0,
            BLACK,
        );
        draw_circle(
            self.left_foot_position.x,
            self.left_foot_position.y,
            2.0,
            BLACK,
        );
        draw_circle(
            self.right_foot_position.x,
            self.right_foot_position.y,
            2.0,
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
        let nodes = vec![Node::new(); NODE_COUNT];
        Self {
            nodes,
            config: Config {
                speed: 100.0,
                node_count: 13,
                feet_radius: 30.0,
                joint_distance: 5.0,
            },
        }
    }
}

impl App for ProcAnim {
    fn update(&mut self) {
        egui_macroquad::ui(|ctx| {
            ctx.set_theme(egui::Theme::Light);
            ctx.style_mut(|style| style.visuals.window_shadow = egui::Shadow::NONE);
            egui::Window::new("Controls")
                .resizable(false)
                .max_width(200.0)
                .show(ctx, |ui| {
                    ui.add(egui::Slider::new(&mut self.config.speed, 0.0..=500.0).text("Speed"));
                    ui.add(
                        egui::Slider::new(&mut self.config.node_count, 1..=100).text("Node count"),
                    );
                    ui.add(
                        egui::Slider::new(&mut self.config.feet_radius, 2.0..=500.0)
                            .text("Feet radius"),
                    );
                    ui.add(
                        egui::Slider::new(&mut self.config.joint_distance, 0.0..=15.0)
                            .text("Joint distance"),
                    );
                });
        });
        let mouse_pos: Vec2 = input::mouse_position().into();

        let time = time::get_time() as f32 * 2.0;
        let mut cursor_pos = vec2(
            mouse_pos.x + time.cos() as f32 * 100.0,
            mouse_pos.y + time.sin() as f32 * 100.0,
        );
        for node in self.nodes.iter_mut().rev().take(self.config.node_count) {
            let diff = cursor_pos - node.position;
            node.direction = diff.normalize();
            if diff.length_squared() > 20.0 * 20.0 {
                node.position += diff.normalize() * self.config.speed * time::get_frame_time();
            }
            cursor_pos = node.position;

            node.update(&self.config);
        }
    }

    fn draw(&self) {
        for node in self.nodes.iter().rev().take(self.config.node_count) {
            node.draw();
        }

        egui_macroquad::draw();
    }

    fn resize(&mut self, _width: f32, _height: f32) {}
}
