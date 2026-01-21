use rand::Rng;

pub use std::f64::consts::PI;

pub fn degress_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    // [0.0, 1.0)
    rand::rng().random()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}
