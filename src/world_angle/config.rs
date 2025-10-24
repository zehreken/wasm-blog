use crate::world_angle::Coord;
use std::f32::consts::PI;

pub const CITIES: &[&str] = &[
    "Ankara",
    "Stockholm",
    "Barcelona",
    "Tokyo",
    "Sydney",
    "Johannesburg",
];
// N is +, S is -
// W is +, E is -
pub const COORDS: &[Coord] = &[
    Coord {
        // Ankara 39.9334° N, 32.8597° E
        latitude: 39.9334 * PI / 180.0,
        longitude: -32.8597 * PI / 180.0,
    },
    Coord {
        // Stockholm 59.3327° N, 18.0656° E
        latitude: 59.3327 * PI / 180.0,
        longitude: -18.0656 * PI / 180.0,
    },
    Coord {
        // Barcelona 41.3874° N, 2.1686° E
        latitude: 41.3874 * PI / 180.0,
        longitude: -2.1686 * PI / 180.0,
    },
    Coord {
        // Tokyo 35.6764° N, 139.6500° E
        latitude: 35.6764 * PI / 180.0,
        longitude: -139.6500 * PI / 180.0,
    },
    Coord {
        // Sydney 33.8727° S, 151.2057° E
        latitude: -33.8727 * PI / 180.0,
        longitude: -151.2057 * PI / 180.0,
    },
    Coord {
        // Johannesburg 26.2056° S, 28.0337° E
        latitude: -26.2056 * PI / 180.0,
        longitude: -28.0337 * PI / 180.0,
    },
];
