// cargo build --release --target wasm32-unknown-unknown
// to use features add: --features boids --no-default-features

mod a_star;
mod app;
mod audio;
mod boids;
mod cycle_drive_train;
mod fuzzy_logic;
mod graphic_functions;
mod life;
mod proc_anim;
mod raycasting_engine;
mod sandbox;
mod shared;
mod world_angle;

use crate::app::App;
use macroquad::prelude::*;

fn config() -> Conf {
    let window_title: String = {
        #[cfg(feature = "a_star")]
        {
            a_star::get_title()
        }
        #[cfg(feature = "audio")]
        {
            audio::get_title()
        }
        #[cfg(feature = "boids")]
        {
            boids::get_title()
        }
        #[cfg(feature = "cycle_drive_train")]
        {
            cycle_drive_train::get_title()
        }
        #[cfg(feature = "fuzzy_logic")]
        {
            fuzzy_logic::get_title()
        }
        #[cfg(feature = "graphic_functions")]
        {
            graphic_functions::get_title()
        }
        #[cfg(feature = "life")]
        {
            life::get_title()
        }
        #[cfg(feature = "proc_anim")]
        {
            proc_anim::get_title()
        }
        #[cfg(feature = "sandbox")]
        {
            sandbox::get_title()
        }
        #[cfg(feature = "world_angle")]
        {
            world_angle::get_title()
        }
    };

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
    #[cfg(feature = "a_star")]
    {
        let mut astar = AStar::new();
        astar.resize(screen_width(), screen_height());
        let app = Box::new(astar);
    }

    #[cfg(feature = "audio")]
    let app = Box::new(audio::Audio::new().await);

    #[cfg(feature = "boids")]
    let app = Box::new(Boids::new());

    #[cfg(feature = "cycle_drive_train")]
    let app = Box::new(Cycle::new());

    #[cfg(feature = "graphic_functions")]
    let app = Box::new(GraphicFunctions::new());

    #[cfg(feature = "fuzzy_logic")]
    let app = Box::new(fuzzy_logic::FuzzyLogic::new());

    #[cfg(feature = "life")]
    let app = Box::new(life::Life::new(screen_width(), screen_height()));

    #[cfg(feature = "proc_anim")]
    let app = Box::new(ProcAnim::new());

    #[cfg(feature = "sandbox")]
    let app = Box::new(Sandbox::new(screen_width(), screen_height()));

    #[cfg(feature = "world_angle")]
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
