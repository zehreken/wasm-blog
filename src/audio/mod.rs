use macroquad::{audio::*, prelude::*};

use crate::app::App;
const SIZE: usize = 1024 * 2;

pub fn get_title() -> String {
    return "Audio".to_owned();
}

pub struct Audio {
    sound: Sound,
}

impl Audio {
    pub async fn new() -> Self {
        let sound = load_sound("assets/test.wav").await.unwrap();
        Self { sound }
    }
}

impl App for Audio {
    fn update(&mut self) {
        // let raw: Vec<[f32; 2]> = vec![[0.0; 2]; SIZE];
        // let data: Vec<u8> = raw
        //     .iter()
        //     .flat_map(|e| e.iter().flat_map(|f| f.to_ne_bytes()))
        //     .collect();
        // let data = &data[..];

        egui_macroquad::ui(|ctx| {
            egui::SidePanel::left("").show(ctx, |ui| {
                if ui.button("test").clicked() {
                    println!("test");
                    play_sound(
                        &self.sound,
                        PlaySoundParams {
                            looped: false,
                            volume: 1.0,
                        },
                    );
                }
            });
        });

        egui_macroquad::draw();
    }

    fn draw(&self) {}

    fn resize(&mut self, _width: f32, _height: f32) {}
}
