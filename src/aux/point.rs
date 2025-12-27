pub mod aux;

use std::clone::*;

#[derive(Copy, Clone)]
pub struct Point {
    x: f64,
    y: f64,
}

pub fn new_res_point(x: f64, y: f64) -> Box<Point> {
    let p = Point { x, y };

    Box::new(p)
}

pub fn new_res_point_from_angle(angle: f64, dist: f64) -> Box<Point> {
    let theta: f64 = 2.0 * std::f64::consts::PI * angle;
    let x: f64 = dist * theta.cos();
    let y: f64 = dist * theta.sin();

    new_res_point(x, y)
}
