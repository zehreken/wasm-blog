pub trait App {
    fn update(&mut self);
    fn draw(&self);
    fn resize(&mut self, width: f32, height: f32);
}
