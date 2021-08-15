use macroquad::prelude::*;

pub fn get_config() -> Conf {
    Conf {
        window_title: "Graphics Functions".to_owned(),
        window_width: 100,
        window_height: 100,
        fullscreen: false,
        ..Default::default()
    }
}

pub async fn run() {
    loop {
        clear_background(WHITE);

        // Process keys, mouse etc.

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("egui ❤ macroquad").show(egui_ctx, |ui| {
                ui.label("Test");
            });
        });

        // Draw things before egui

        egui_macroquad::draw();

        next_frame().await
    }
}
