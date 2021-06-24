use macroquad::prelude::Conf;

// cargo build --release --target wasm32-unknown-unknown

mod cellular_automata;
use cellular_automata::*;

#[macroquad::main(automata)]
async fn main() {
    let future = cellular_automata::run();
    future.await
}
