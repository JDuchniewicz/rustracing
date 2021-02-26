use rand::prelude::*;

#[inline]
pub fn random_f64() -> f64 {
    rand::thread_rng().gen()
}

#[inline]
pub fn random_f64_minmax(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}
