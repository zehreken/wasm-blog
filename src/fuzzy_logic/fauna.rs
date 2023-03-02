use super::cell::Cell;

pub struct Fauna {
    cells: [Cell; 10],
}

impl Fauna {
    pub fn new() -> Self {
        Self {
            cells: [Cell::new(20_f32); 10],
        }
    }

    pub fn update(&self, delta_time: f32) {
        for cell in self.cells {
            cell.update(delta_time);
        }
    }

    pub fn draw(&self) {
        for cell in self.cells {
            cell.draw();
        }
    }
}
