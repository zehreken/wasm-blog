// cargo build --release --target wasm32-unknown-unknown

mod life;
use life::*;

#[macroquad::main(automata)]
async fn main() {
    let future = life::run();
    future.await
}
