// cargo build --release --target wasm32-unknown-unknown
// The default resolution I set for lab is 700x500

mod a_star;
mod app;
mod audio;
mod boids;
mod cycle_drive_train;
mod fuzzy_logic;
mod graphics_functions;
mod life;

use crate::{app::App, life::Life};
use macroquad::prelude::*;

fn config() -> Conf {
    let window_title = life::get_title();
    // life::get_config()
    // cycle_drive_train::get_config()
    // graphics_functions::get_config()
    // audio::get_config()
    // fuzzy_logic::get_config()
    // boids::get_config()

    Conf {
        window_title,
        window_width: 512,
        window_height: 512,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(config)]
async fn main() {
    // let future = life::run();
    // let future = cycle_drive_train::run();
    // let future = graphics_functions::run();
    // let future = audio::run();
    // let future = fuzzy_logic::run();
    // let future = boids::run();

    let app = Box::new(Life::new(screen_width(), screen_height()));
    let future = run(app);

    future.await
}

async fn run(mut app: Box<dyn App>) {
    loop {
        clear_background(WHITE);

        // if (app.width - screen_width()).abs() > 1.0 || (app.height - screen_height()).abs() > 1.0 {
        //     app.resize(screen_width(), screen_height());
        // }

        app.update();

        app.draw();

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
