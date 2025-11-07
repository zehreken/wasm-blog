mod cell;
mod fauna;
mod utils;
use macroquad::prelude::*;

use crate::{app::App, fuzzy_logic::fauna::Fauna};

pub fn get_title() -> String {
    return "Fuzzy Logic".to_owned();
}

pub struct FuzzyLogic {
    fauna: Fauna,
}

impl FuzzyLogic {
    pub fn new() -> Self {
        Self {
            fauna: Fauna::new(),
        }
    }
}

impl App for FuzzyLogic {
    fn update(&mut self) {
        self.fauna.update();

        self.fauna.ui();
    }

    fn draw(&self) {
        self.fauna.draw();
    }

    fn resize(&mut self, width: f32, height: f32) {}
}
