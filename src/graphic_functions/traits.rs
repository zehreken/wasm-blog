use egui::*;

pub trait View {
    fn draw(&mut self);
    fn draw_ui(&mut self, ui: &mut Ui);
}
