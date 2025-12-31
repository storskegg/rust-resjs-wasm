use std::clone::*;

#[derive(Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

pub fn new_res_point(x: i32, y: i32) -> Box<Point> {
    let p = Point { x, y };

    Box::new(p)
}

pub fn new_res_point_from_angle(angle: f64, dist: f64) -> Box<Point> {
    let theta: f64 = 2.0 * std::f64::consts::PI * angle;
    let x: i32 = (dist * theta.cos()) as i32;
    let y: i32 = (dist * theta.sin()) as i32;

    new_res_point(x, y)
}
