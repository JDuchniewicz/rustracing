use rand::prelude::*;

pub fn random_f64() -> f64 {
    rand::thread_rng().gen()
}

pub fn random_f64_minmax(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}

pub fn degrees_to_radians(deg: f64) -> f64 {
    deg * std::f64::consts::PI / 180.0
}
