mod cell;
mod fauna;
mod utils;
use macroquad::prelude::*;

pub fn get_config() -> Conf {
    Conf {
        window_title: "Fuzzy Logic".to_owned(),
        window_width: 512,
        window_height: 512,
        fullscreen: false,
        ..Default::default()
    }
}

pub async fn run() {
    let mut fauna = fauna::Fauna::new();

    loop {
        clear_background(WHITE);

        fauna.update(macroquad::time::get_frame_time());

        fauna.draw();

        fauna.ui();

        next_frame().await
    }
}
