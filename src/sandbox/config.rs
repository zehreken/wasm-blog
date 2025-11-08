pub const PARTICLE_SIZE: u32 = 4;
pub const BRUSH_SIZES: &[u8] = &[1, 2, 4, 8, 16];

#[derive(Debug, PartialEq)]
pub enum BrushId {
    _1,
    _2,
    _4,
    _8,
    _16,
}

pub fn get_brush_index(brush: &BrushId) -> usize {
    return match brush {
        BrushId::_1 => 0,
        BrushId::_2 => 1,
        BrushId::_4 => 2,
        BrushId::_8 => 3,
        BrushId::_16 => 4,
    };
}
