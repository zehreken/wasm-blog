mod cellular_automata;

#[macroquad::main("wasm-blog")]
async fn main() {
    let future = cellular_automata::run();
    future.await
}
