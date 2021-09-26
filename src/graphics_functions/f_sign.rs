use super::traits::View;
use egui::*;
pub struct Model {
    slider_value: f32,
}

impl Model {
    pub fn new() -> Self {
        Self { slider_value: 0.0 }
    }
}

impl View for Model {
    fn draw(&mut self) {}

    fn draw_ui(&mut self, ui: &mut Ui) {
        let range = -10.0..=10.0;
        ui.add(
            Slider::new(&mut self.slider_value, range)
                .logarithmic(false)
                .clamp_to_range(false)
                .smart_aim(false),
        );
        let value: i32 = if self.slider_value < 0.0 {
            -1
        } else if self.slider_value > 0.0 {
            1
        } else {
            0
        };
        ui.label(format!("sign({:.1}) = {}", self.slider_value, value));
    }
}
