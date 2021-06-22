mod cellular_automata;
// use futures::executor::block_on;

#[macroquad::main("BasicShapes")]
async fn main() {
    let future = cellular_automata::run();
	future.await
    // block_on(future);
}
