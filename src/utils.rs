#[inline]
pub fn degrees_to_radians(deg: f64) -> f64 {
    deg * std::f64::consts::PI / 180.0
}
