use std::clone::*;

#[derive(Copy, Clone)]
pub struct Rectangle {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Rectangle {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self { x, y, width, height }
    }
    pub fn mirror(&self, width: i32) -> Self {
        Rectangle::new(width - self.x - self.width, self.y, self.width, self.height)
    }

    pub fn intersect(&self, other: &Rectangle) -> Self {
        let x_left = self.x.max(other.x);
        let x_right = (self.x + self.width).min(other.x + other.width);
        let y_top = self.y.max(other.y);
        let y_bottom = (self.y + self.height).min(other.y + other.height);
        let w = x_right.max(x_left) - x_left;
        let h = y_bottom.max(y_top) - y_top;
        Rectangle::new(x_left, y_top, w, h)
    }

    pub fn intersects(&self, other: &Rectangle) -> bool {
        self.intersect(other).width > 0 && self.intersect(other).height > 0
    }

    pub fn includes(&self, x: i32, y: i32) -> bool {
        self.x <= x && x < self.x + self.width && self.y <= y && y < self.y + self.height
    }

    pub fn chop_start_h(&self, x: i32) -> Rectangle {
        Rectangle::new(self.x, self.y, x - self.x, self.height)
    }

    pub fn chop_end_h(&self, x: i32) -> Rectangle {
        Rectangle::new(x, self.y, self.width - x + self.x, self.height)
    }

    pub fn chop_start_v(&self, y: i32) -> Rectangle {
        Rectangle::new(self.x, self.y, self.width, y - self.y)
    }

    pub fn chop_end_v(&self, y: i32) -> Rectangle {
        Rectangle::new(self.x, y, self.width, self.height - y + self.y)
    }

    pub fn chop_pattern(&self, pattern: &str) -> Rectangle {
        let mut result = self.clone();

        for c in pattern.chars() {
            if c == 't' {
                result = Rectangle::new(self.x, self.y, self.width, self.height/2)
            }

            if c == 'b' {
                result = Rectangle::new(self.x, self.y + self.height/2, self.width, self.height/2)
            }

            if c == 's' {
                result = Rectangle::new(self.x, self.y, self.width/2, self.height)
            }

            if c == 'e' {
                result = Rectangle::new(self.x + self.width/2, self.y, self.width/2, self.height)
            }
        }

        result
    }

    pub fn center(&self, other: &Rectangle) -> Rectangle {
        let horizontal_surplus = self.width - other.width;
        let vertical_surplus = self.height - other.height;
        let x = self.x + horizontal_surplus /2 - other.x;
        let y = self.y + vertical_surplus /2 - other.y;
        Rectangle::new(x, y, other.width, other.height)
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }
}
