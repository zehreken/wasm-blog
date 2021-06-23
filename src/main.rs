// cargo build --release --target wasm32-unknown-unknown

mod cellular_automata;

#[macroquad::main("wasm-blog")]
async fn main() {
    let future = cellular_automata::run();
    future.await
}
