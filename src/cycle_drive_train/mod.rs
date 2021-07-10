use macroquad::prelude::*;
mod config;
use config::*;

pub fn get_config() -> Conf {
    Conf {
        window_title: "Cycle".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        fullscreen: false,
        ..Default::default()
    }
}

pub async fn run() {
    loop {
        clear_background(WHITE);

        let rotation = 0.05 * macroquad::time::get_time() as f32;
        let gear_ratio = FRONT_GEAR_SIZE / REAR_GEAR_SIZE;
        draw_wheel(200.0, 256.0, 100.0, rotation * FRONT_GEAR_SIZE);

        draw_circle_with_line(200.0, 256.0, REAR_GEAR_SIZE, rotation * FRONT_GEAR_SIZE);

        draw_circle_with_line(400.0, 256.0, FRONT_GEAR_SIZE, rotation * REAR_GEAR_SIZE);

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

fn draw_wheel(x: f32, y: f32, radius: f32, rotation: f32) {
    draw_poly_lines(x, y, 30, radius, rotation, 1.0, GRAY);

    let spoke_count = 24;

    for i in 0..spoke_count {
        let t = i as f32 * 2.0 * std::f32::consts::PI / spoke_count as f32;
        let x2 = x + radius * (rotation + t).cos();
        let y2 = y + radius * (rotation + t).sin();
        draw_line(x, y, x2, y2, 1.0, GRAY);
    }
}

fn draw_circle_with_line(x: f32, y: f32, radius: f32, rotation: f32) {
    draw_poly_lines(x, y, 30, radius, rotation, 1.0, BLACK);

    let x2 = x + radius * rotation.cos();
    let y2 = y + radius * rotation.sin();
    draw_line(x, y, x2, y2, 1.0, PINK);
}
