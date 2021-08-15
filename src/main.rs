// cargo build --release --target wasm32-unknown-unknown

mod cycle_drive_train;
mod graphics_functions;
mod life;
use macroquad::prelude::Conf;

#[macroquad::main(config)]
async fn main() {
    // let future = life::run();
    // let future = cycle_drive_train::run();
    let future = graphics_functions::run();
    future.await
}

fn config() -> Conf {
    // life::get_config()
    // cycle_drive_train::get_config()
    graphics_functions::get_config()
}
