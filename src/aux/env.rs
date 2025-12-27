use crate::aux::{Context, Rectangle};

pub struct Env {
    min_left_margin_px: f64,
    min_top_margin_px: f64,
    min_right_margin_px: f64,
    min_bottom_margin_px: f64,

    left_margin_px: f64,
    top_margin_px: f64,
    right_margin_px: f64,
    bottom_margin_px: f64,

    total_width_px: f64,
    total_height_px: f64,
}

impl Env {
    pub fn new(ctx: &Context) -> Self {
        let initial_margin_px = ctx.margin_px;
        let mut env = Env {
            min_left_margin_px: initial_margin_px,
            min_top_margin_px: initial_margin_px,
            min_right_margin_px: initial_margin_px,
            min_bottom_margin_px: initial_margin_px,

            left_margin_px: 0.0,
            top_margin_px: 0.0,
            right_margin_px: 0.0,
            bottom_margin_px: 0.0,

            total_width_px: 0.0,
            total_height_px: 0.0,
        };
        env.update_margins();

        env
    }

    /// Ensure that rectangle fits. If not, adjust dimensions for next time.
    pub fn ensure_rect(&mut self, rect: &Rectangle) {
        let x_min = rect.x();
        let x_max = rect.x() + rect.width();
        let y_min = rect.y();
        let y_max = rect.y() + rect.height();

        self.ensure_left_margin(self.left_margin_px - x_min);
        self.ensure_right_margin(self.right_margin_px + x_max - self.total_width_px);
        self.ensure_top_margin(self.top_margin_px - y_min);
        self.ensure_bottom_margin(self.bottom_margin_px + y_max - self.total_height_px);
    }

    fn ensure_left_margin(&mut self, margin_px: f64) {
        self.min_left_margin_px = self.min_left_margin_px.max(margin_px);
    }

    fn ensure_right_margin(&mut self, margin_px: f64) {
        self.min_right_margin_px = self.min_right_margin_px.max(margin_px);
    }

    fn ensure_top_margin(&mut self, margin_px: f64) {
        self.min_top_margin_px = self.min_top_margin_px.max(margin_px);
    }

    fn ensure_bottom_margin(&mut self, margin_px: f64) {
        self.min_bottom_margin_px = self.min_bottom_margin_px.max(margin_px);
    }

    fn update_margins(&mut self) {
        self.left_margin_px = self.min_left_margin_px;
        self.top_margin_px = self.min_top_margin_px;
        self.right_margin_px = self.min_right_margin_px;
        self.bottom_margin_px = self.min_bottom_margin_px;
    }

    pub fn margins_unchanged(&self) -> bool {
        let mut result: bool = true;

        result = result && self.min_left_margin_px == self.left_margin_px;
        result = result && self.min_top_margin_px == self.top_margin_px;
        result = result && self.min_right_margin_px == self.right_margin_px;
        result = result && self.min_bottom_margin_px == self.bottom_margin_px;

        result
    }
}
