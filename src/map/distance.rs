use std::f64::consts::PI;
use std::f64;

pub fn get_distance(loc1: &Location, loc2: &Location) -> f64 {
    distance(loc1.get_position(), loc2.get_position())
}

fn distance((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> f64 {
    f64::sqrt(f64::powi(x2 - x1, 2) + f64::powi(y2 - y1, 2))
}

