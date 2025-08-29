use crate::app::App;

mod cell;

pub fn get_title() -> String {
    return "AStar".to_owned();
}

pub struct AStar {}

impl AStar {
    pub fn new(width: f32, height: f32) -> Self {
        Self {}
    }
}

impl App for AStar {
    fn update(&mut self) {}

    fn draw(&self) {}

    fn resize(&mut self, width: f32, height: f32) {}
}
