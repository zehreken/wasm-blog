// cargo build --release --target wasm32-unknown-unknown

mod a_star;
mod app;
mod audio;
mod boids;
mod cycle_drive_train;
mod fuzzy_logic;
mod graphic_functions;
mod life;
mod proc_anim;
mod sandbox;
mod shared;
mod world_angle;

use crate::{
    a_star::AStar, app::App, boids::Boids, cycle_drive_train::Cycle,
    graphic_functions::GraphicFunctions, life::Life, proc_anim::ProcAnim, world_angle::WorldAngle,
};
use macroquad::{prelude::*, time};

fn config() -> Conf {
    let window_title = life::get_title();
    let window_title = cycle_drive_train::get_title();
    let window_title = graphic_functions::get_title();
    // audio::get_config()
    // fuzzy_logic::get_config()
    let window_title = boids::get_title();
    let window_title = a_star::get_title();
    let window_title = proc_anim::get_title();
    let window_title = world_angle::get_title();

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
    let app = Box::new(Life::new(screen_width(), screen_height()));
    let app = Box::new(Cycle::new());
    let app = Box::new(GraphicFunctions::new());
    // let future = audio::run();
    // let future = fuzzy_logic::run();
    let app = Box::new(Boids::new());
    let mut astar = AStar::new();
    astar.resize(screen_width(), screen_height());
    let app = Box::new(astar);
    let app = Box::new(ProcAnim::new());
    let app = Box::new(WorldAngle::new().await);
    let future = run(app);

    future.await
}

async fn run(mut app: Box<dyn App>) {
    let mut width = screen_width();
    let mut height = screen_height();
    loop {
        clear_background(WHITE);

        if (width - screen_width()).abs() > 1.0 || (height - screen_height()).abs() > 1.0 {
            width = screen_width();
            height = screen_height();
            app.resize(width, height);
        }

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
