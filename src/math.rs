pub use math::*;

/*
    Wrapped in another module just to be able 
    to #[allow(dead_code)] for eveything at once.
*/

#[allow(dead_code)]
mod math {
    pub fn lerp(a: f64, b: f64, t: f64) -> f64{
        a + t * (b - a)
    }

    pub fn clamp<T>(x: T, min: T, max: T) -> T
    where T: PartialOrd 
    {
        if x < min { min }
        else if x > max { max }
        else { x }
    }

    pub fn abs(x: f64) -> f64 {
        if x < 0.0 { -x }
        else { x }
    }

    pub fn max(x: f64, y: f64) -> f64 {
        if x > y { x }
        else { y }
    }

    pub fn min(a: f64, b: f64) -> f64 {
        if a < b { a }
        else { b }
    }

    pub fn min_max(a: f64, b: f64) -> (f64, f64) {
        if a < b { (a, b) }
        else { (b, a) }
    }
}