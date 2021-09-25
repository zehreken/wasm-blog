use egui::*;

pub struct Model {
    lower_edge: f32,
    upper_edge: f32,
    x: f32,
}

impl Model {
    pub fn new() -> Self {
        Self {
            lower_edge: 0.0,
            upper_edge: 0.0,
            x: 0.0,
        }
    }

    pub fn draw(&mut self, ui: &mut Ui) {
        let range = -10.0..=10.0;
        ui.add(
            Slider::new(&mut self.lower_edge, range.clone())
                .logarithmic(false)
                .clamp_to_range(false)
                .smart_aim(false)
                .text("lower edge"),
        );
        ui.add(
            Slider::new(&mut self.upper_edge, range.clone())
                .logarithmic(false)
                .clamp_to_range(false)
                .smart_aim(false)
                .text("upper edge"),
        );
        ui.add(
            Slider::new(&mut self.x, range)
                .logarithmic(false)
                .clamp_to_range(false)
                .smart_aim(false)
                .text("x"),
        );

        let value =
            ((self.x - self.lower_edge) / (self.upper_edge - self.lower_edge)).clamp(0.0, 1.0);
        ui.label(format!("smoothstep({:.1}) = {}", self.x, value));
    }
}
