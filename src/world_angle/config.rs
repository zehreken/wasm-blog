use crate::world_angle::Coord;
use std::f32::consts::PI;

pub const CITIES: &[&str] = &["Ankara", "Stockholm", "Barcelona", "Tokyo"];
pub const COORDS: &[Coord] = &[
    Coord {
        // Barcelona
        latitude: (90.0 - 41.3874) * PI / 180.0,
        longitude: -2.1686 * PI / 180.0,
    },
    Coord {
        // Tokyo
        latitude: (90.0 - 35.6764) * PI / 180.0,
        longitude: -139.6500 * PI / 180.0,
    },
    Coord {
        // Sydney 33.8727° S, 151.2057° E
        latitude: (90.0 - 33.8727) * PI / 180.0,
        longitude: -151.2057 * PI / 180.0,
    },
    Coord {
        // Johannesburg 26.2056° S, 28.0337° E
        latitude: (90.0 - 26.2056) * PI / 180.0,
        longitude: -28.0337 * PI / 180.0,
    },
];
