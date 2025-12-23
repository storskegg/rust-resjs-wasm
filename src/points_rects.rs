pub struct ResPoint {
    x: f64,
    y: f64,
}

pub fn NewResPoint(x: f64, y: f64) -> Box<ResPoint> {
    let p = ResPoint { x, y };

    Box::new(p)
}

pub fn NewResPointFromAngle(angle: f64, dist: f64) -> Box<ResPoint> {
    let theta: f64 = 2.0 * std::f64::consts::PI * angle;
    let x: f64 = dist * theta.cos();
    let y: f64 = dist * theta.sin();

    NewResPoint(x, y)
}
