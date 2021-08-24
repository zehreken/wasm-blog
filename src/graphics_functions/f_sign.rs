use egui::*;
pub struct Model {
    slider_value: f32,
}

impl Model {
    pub fn new() -> Self {
        Self { slider_value: 0.0 }
    }
    pub fn update(&mut self) {}

    pub fn draw(&mut self, ui: &mut Ui) {
        let min = -10.0;
        let max = 10.0;
        ui.add(
            Slider::new(&mut self.slider_value, min..=max)
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
