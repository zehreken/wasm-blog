use macroquad::prelude::*;
mod f_mod;
mod f_sign;
mod f_smoothstep;

pub fn get_config() -> Conf {
    Conf {
        window_title: "Graphics Functions".to_owned(),
        window_width: 800,
        window_height: 600,
        fullscreen: false,
        ..Default::default()
    }
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
struct Model {
    function: Function,
}

pub async fn run() {
    let mut model = Model {
        function: Function::Mod,
    };
    let mut f_mod = f_mod::Model::new();
    let mut f_sign = f_sign::Model::new();
    let mut f_smoothstep = f_smoothstep::Model::new();

    loop {
        clear_background(WHITE);

        // Process keys, mouse etc.

        egui_macroquad::ui(|egui_ctx| {
            egui::SidePanel::left("").show(egui_ctx, |ui| {
                // ui.horizontal(|ui| {
                //     ui.label("test");
                // });

                egui::ComboBox::from_label("Function")
                    .selected_text(format!("{:?}", model.function))
                    .show_ui(ui, |ui| {
                        for f in Function::all() {
                            ui.selectable_value(&mut model.function, f, format!("{:?}", f));
                        }
                    });
                match model.function {
                    Function::Mod => f_mod.draw(ui),
                    Function::Sign => f_sign.draw(ui),
                    Function::SmoothStep => f_smoothstep.draw(ui),
                    _ => {}
                }
            });
            // TODO: Remove later
            // egui::Window::new(format!("{:?}", model.function)).show(egui_ctx, |ui| {
            //     f_sign.draw(ui);
            // });
        });

        // Draw things before egui

        egui_macroquad::draw();

        next_frame().await
    }
}
