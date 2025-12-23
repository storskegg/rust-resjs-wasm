use std::clone::*;

#[derive(Copy, Clone)]
pub struct ResPoint {
    x: f64,
    y: f64,
}

pub fn new_res_point(x: f64, y: f64) -> Box<ResPoint> {
    let p = ResPoint { x, y };

    Box::new(p)
}

pub fn new_res_point_from_angle(angle: f64, dist: f64) -> Box<ResPoint> {
    let theta: f64 = 2.0 * std::f64::consts::PI * angle;
    let x: f64 = dist * theta.cos();
    let y: f64 = dist * theta.sin();

    new_res_point(x, y)
}

#[derive(Copy, Clone)]
pub struct ResRectangle {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

pub fn new_res_rectangle(x: f64, y: f64, width: f64, height: f64) -> ResRectangle {
    ResRectangle { x, y, width, height }
}

impl ResRectangle {
    pub fn mirror(&self, width: f64) -> ResRectangle {
        new_res_rectangle(width - self.x - self.width, self.y, self.width, self.height)
    }

    pub fn intersect(&self, other: &ResRectangle) -> ResRectangle {
        let x_left = self.x.max(other.x);
        let x_right = (self.x + self.width).min(other.x + other.width);
        let y_top = self.y.max(other.y);
        let y_bottom = (self.y + self.height).min(other.y + other.height);
        let w = x_right.max(x_left) - x_left;
        let h = y_bottom.max(y_top) - y_top;
        new_res_rectangle(x_left, y_top, w, h)
    }

    pub fn intersects(&self, other: &ResRectangle) -> bool {
        self.intersect(other).width > 0.0 && self.intersect(other).height > 0.0
    }

    pub fn includes(&self, x: f64, y: f64) -> bool {
        self.x <= x && x < self.x + self.width && self.y <= y && y < self.y + self.height
    }

    pub fn chop_start_h(&self, x: f64) -> ResRectangle {
        new_res_rectangle(self.x, self.y, x - self.x, self.height)
    }

    pub fn chop_end_h(&self, x: f64) -> ResRectangle {
        new_res_rectangle(x, self.y, self.width - x + self.x, self.height)
    }

    pub fn chop_start_v(&self, y: f64) -> ResRectangle {
        new_res_rectangle(self.x, self.y, self.width, y - self.y)
    }

    pub fn chop_end_v(&self, y: f64) -> ResRectangle {
        new_res_rectangle(self.x, y, self.width, self.height - y + self.y)
    }

    pub fn chop_pattern(&self, pattern: &str) -> ResRectangle {
        let mut result = self.clone();

        for c in pattern.chars() {
            if c == 't' {
                result = new_res_rectangle(self.x, self.y, self.width, self.height/2.0)
            }

            if c == 'b' {
                result = new_res_rectangle(self.x, self.y + self.height/2.0, self.width, self.height/2.0)
            }

            if c == 's' {
                result = new_res_rectangle(self.x, self.y, self.width/2.0, self.height)
            }

            if c == 'e' {
                result = new_res_rectangle(self.x + self.width/2.0, self.y, self.width/2.0, self.height)
            }
        }

        result
    }
    
    pub fn center(&self, other: &ResRectangle) -> ResRectangle {
        let horizontal_surplus = self.width - other.width;
        let vertical_surplus = self.height - other.height;
        let x = self.x + horizontal_surplus /2.0 - other.x;
        let y = self.y + vertical_surplus /2.0 - other.y;
        new_res_rectangle(x, y, other.width, other.height)
    }
}
