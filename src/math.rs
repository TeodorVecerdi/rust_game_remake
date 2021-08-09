#[allow(dead_code)]
pub(crate) fn lerp(a: f64, b: f64, t: f64) -> f64{
    a + t * (b - a)
}

#[allow(dead_code)]
pub(crate) fn clamp<T>(x: T, min: T, max: T) -> T
where T: PartialOrd 
{
    if x < min { min }
    else if x > max { max }
    else { x }
}

#[allow(dead_code)]
pub(crate) fn abs(x: f64) -> f64 {
    if x < 0.0 { -x }
    else { x }
}

#[allow(dead_code)]
pub(crate) fn max(x: f64, y: f64) -> f64 {
    if x > y { x }
    else { y }
}

#[allow(dead_code)]
pub(crate) fn min(a: f64, b: f64) -> f64 {
    if a < b { a }
    else { b }
}

#[allow(dead_code)]
pub(crate) fn min_max(a: f64, b: f64) -> (f64, f64) {
    if a < b { (a, b) }
    else { (b, a) }
}