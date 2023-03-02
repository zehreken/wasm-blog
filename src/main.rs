// cargo build --release --target wasm32-unknown-unknown

mod audio;
mod cycle_drive_train;
mod fuzzy_logic;
mod graphics_functions;
mod life;
use macroquad::prelude::Conf;

#[macroquad::main(config)]
async fn main() {
    // let future = life::run();
    // let future = cycle_drive_train::run();
    // let future = graphics_functions::run();
    // let future = audio::run();
    let future = fuzzy_logic::run();
    future.await
}

fn config() -> Conf {
    // life::get_config()
    // cycle_drive_train::get_config()
    // graphics_functions::get_config()
    // audio::get_config()
    fuzzy_logic::get_config()
}
