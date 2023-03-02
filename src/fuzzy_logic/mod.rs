mod cell;
mod fauna;
use macroquad::prelude::*;

pub fn get_config() -> Conf {
    Conf {
        window_title: "Fuzzy Logic".to_owned(),
        window_width: 600,
        window_height: 500,
        fullscreen: false,
        ..Default::default()
    }
}

pub async fn run() {
    let fauna = fauna::Fauna::new();
    loop {
        clear_background(WHITE);

        fauna.update(macroquad::time::get_frame_time());

        fauna.draw();

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
