use egui::*;

pub struct Model {
    param_1: f32,
    param_2: f32,
}

impl Model {
    pub fn new() -> Self {
        Self {
            param_1: 0.0,
            param_2: 0.0,
        }
    }

    pub fn draw(&mut self, ui: &mut Ui) {
        let range = -10.0..=10.0;
        ui.add(
            Slider::new(&mut self.param_1, range.clone())
                .logarithmic(false)
                .clamp_to_range(false)
                .smart_aim(false)
                .text("param 1"),
        );
        ui.add(
            Slider::new(&mut self.param_2, range)
                .logarithmic(false)
                .clamp_to_range(false)
                .smart_aim(false)
                .text("param 2"),
        );

        let value = self.param_1 - self.param_2 * (self.param_1 / self.param_2).floor();
        ui.label(format!(
            "mod({:.1}, {:.1}) = {}",
            self.param_1, self.param_2, value
        ));
    }
}
