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

        next_frame().await
    }
}
