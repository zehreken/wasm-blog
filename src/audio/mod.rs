use macroquad::{audio::*, prelude::*};
const SIZE: usize = 1024 * 2;

pub fn get_config() -> Conf {
    Conf {
        window_title: "Audio".to_owned(),
        window_width: 800,
        window_height: 600,
        fullscreen: false,
        ..Default::default()
    }
}

pub async fn run() {
    let raw: Vec<[f32; 2]> = vec![[0.0; 2]; SIZE];
    let data: Vec<u8> = raw
        .iter()
        .flat_map(|e| e.iter().flat_map(|f| f.to_ne_bytes()))
        .collect();
    let data = &data[..];
    // let sound = load_sound_from_bytes(data).await;
    let sound = load_sound("test.wav").await;
    let sound = sound.unwrap();

    loop {
        clear_background(WHITE);

        egui_macroquad::ui(|egui_ctx| {
            egui::SidePanel::left("").show(egui_ctx, |ui| {
                if ui.button("test").clicked() {
                    println!("test");
                    play_sound(
                        &sound,
                        PlaySoundParams {
                            looped: false,
                            volume: 1.0,
                        },
                    );
                }
            });
        });

        egui_macroquad::draw();

        draw_text(
            &format!("fps: {}", macroquad::time::get_fps()),
            2.0,
            12.0,
            16.0,
            PINK,
        );

        next_frame().await
    }
}
