use crate::world_angle::Coord;
use macroquad::color::{BLUE, Color, GRAY, GREEN, MAGENTA, ORANGE, RED, YELLOW};

pub const LATITUDE_OFFSET: f32 = -7.0;
pub const LONGITUDE_OFFSET: f32 = -90.0;

pub const CITIES: &[&str] = &[
    "Ankara",
    "Stockholm",
    "Barcelona",
    "Tokyo",
    "Sydney",
    "Johannesburg",
    "Mexico City",
    "Kyiv",
];
// N is +, S is -
// W is +, E is -
pub const COORDS: &[Coord] = &[
    Coord {
        // Ankara 39.9334° N, 32.8597° E
        latitude: 39.9334,
        longitude: -32.8597,
    },
    Coord {
        // Stockholm 59.3327° N, 18.0656° E
        latitude: 59.3327,
        longitude: -18.0656,
    },
    Coord {
        // Barcelona 41.3874° N, 2.1686° E
        latitude: 41.3874,
        longitude: -2.1686,
    },
    Coord {
        // Tokyo 35.6764° N, 139.6500° E
        latitude: 35.6764,
        longitude: -139.6500,
    },
    Coord {
        // Sydney 33.8727° S, 151.2057° E
        latitude: -33.8727,
        longitude: -151.2057,
    },
    Coord {
        // Johannesburg 26.2056° S, 28.0337° E
        latitude: -26.2056,
        longitude: -28.0337,
    },
    Coord {
        // Mexico City 19.4326° N, 99.1332° W
        latitude: 19.4326,
        longitude: 99.1332,
    },
    Coord {
        // Kyiv 50.4504° N, 30.5245° E
        latitude: 50.4504,
        longitude: -30.5245,
    },
];

pub const COLORS: &[Color] = &[RED, GREEN, BLUE, YELLOW, ORANGE, MAGENTA, GRAY];
