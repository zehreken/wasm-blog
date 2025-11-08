use macroquad::prelude::*;
mod f_mod;
mod f_sign;
mod f_smoothstep;
mod traits;
use traits::View;

use crate::app::App;

pub fn get_title() -> String {
    return "Graphic Functions".to_owned();
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Function {
    Mod,
    Sign,
    SmoothStep,
}

impl Function {
    pub fn all() -> impl Iterator<Item = Function> {
        [Function::Mod, Function::Sign, Function::SmoothStep]
            .iter()
            .copied()
    }
}

pub struct GraphicFunctions {
    function: Function,
    f_mod: f_mod::Model,
    f_sign: f_sign::Model,
    f_smoothstep: f_smoothstep::Model,
}

impl GraphicFunctions {
    pub fn new() -> Self {
        Self {
            function: Function::Mod,
            f_mod: f_mod::Model::new(),
            f_sign: f_sign::Model::new(),
            f_smoothstep: f_smoothstep::Model::new(),
        }
    }
}

impl App for GraphicFunctions {
    fn update(&mut self) {
        egui_macroquad::ui(|egui_ctx| {
            egui::SidePanel::left("").show(egui_ctx, |ui| {
                // ui.horizontal(|ui| {
                //     ui.label("test");
                // });

                egui::ComboBox::from_label("Function")
                    .selected_text(format!("{:?}", self.function))
                    .show_ui(ui, |ui| {
                        for f in Function::all() {
                            ui.selectable_value(&mut self.function, f, format!("{:?}", f));
                        }
                    });
                match self.function {
                    Function::Mod => self.f_mod.draw_ui(ui),
                    Function::Sign => self.f_sign.draw_ui(ui),
                    Function::SmoothStep => self.f_smoothstep.draw_ui(ui),
                }
            });
            // TODO: Remove later
            // egui::Window::new(format!("{:?}", model.function)).show(egui_ctx, |ui| {
            //     f_sign.draw(ui);
            // });
        });
    }

    fn draw(&self) {
        egui_macroquad::draw();
    }

    fn resize(&mut self, _width: f32, _height: f32) {}
}
