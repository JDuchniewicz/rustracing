use rand::prelude::*;

#[inline]
pub fn random_f64() -> f64 {
    rand::thread_rng().gen()
}
