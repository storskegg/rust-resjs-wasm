use std::cmp::PartialEq;
use std::collections::HashMap;
use crate::aux::{Context, Rectangle};

pub enum ShadingPattern {
    XIsY,
    XIsMinusY,
}

impl PartialEq for ShadingPattern {
    fn eq(&self, other: &Self) -> bool {
        if self == &ShadingPattern::XIsY && other == &ShadingPattern::XIsY {
            return true
        }

        false
    }
}

/// MinMaxPair could have been a tuple, but a struct doesn't use any extra memory, and the human-
/// expressive "min" and "max" fields improve readability, and thus fewer mistakes while maintaining
/// code.
#[derive(Copy, Clone)]
struct MinMaxPair {
    min: i32,
    max: i32
}

type MinMaxPairs = Vec<MinMaxPair>;


// Maps x coordinate (on x axis) to list of pairs of x coordinates
// each delimiting a line segment of shading (hatching).
// context: ResContext
// mirror: boolean (whether mirrored)
// width: of canvas (in px)
pub struct Shading {
    res_context: Option<Context>,
    is_mirrored: bool,
    x_to_pairs: HashMap<i32, MinMaxPairs>
}

impl Shading {
    pub fn new(context: Option<Context>, mirror: bool) -> Self {
        Shading {
            res_context: context,
            is_mirrored: mirror,
            x_to_pairs: HashMap::new()
        }
    }

    fn common_pattern(&self) -> bool {
        self.res_context.is_some() && self.res_context.as_ref().unwrap().shading_pattern == ShadingPattern::XIsY && !self.is_mirrored ||
        self.res_context.is_some() && self.res_context.as_ref().unwrap().shading_pattern == ShadingPattern::XIsMinusY && self.is_mirrored
    }

    fn add(&mut self, x: i32, x_min: i32, x_max: i32) {
        let pair: MinMaxPair = MinMaxPair{min: x_min, max: x_max};

        // "add-sert"
        if self.x_to_pairs.contains_key(&x) {
            self.x_to_pairs.get_mut(&x).unwrap().push(pair);
        } else {
            self.x_to_pairs.insert(x, MinMaxPairs::from([pair]));
        }
    }

    /// add_rect abides the original JS as closely as possible. Because Rust prohibits comparison
    /// of f64 values (e.g. a < b) as part of its built-in safeties, and NaN being an f64 type, this
    /// method does a lot of flip-flopping between i32 and f64 types. I've done my best to minimize
    /// this to help maintain efficiency.
    fn add_rect(&mut self, rect: Rectangle) {
        let x: f64 = rect.x() as f64;
        let y: f64 = rect.y() as f64;
        let w: f64 = rect.width() as f64;
        let h: f64 = rect.height() as f64;

        let ctx = self.res_context.as_ref().unwrap();
        let sep = ctx.shading_sep;

        // TODO: investigate whether sep is really a float. make adjustments below if it is i32
        if self.common_pattern() {
            let x_axis_min: i32 = (((x + y) / sep).floor() * sep) as i32;
            let x_axis_max: i32 = (((x + w + y + h) / sep).ceil() * sep) as i32;

            // TIL... Rust doesn't have a classic for loop, so let's stitch one together
            let mut x_axis: i32 = x_axis_min; // for x_axis = x_axis_min; ...; ...
            let mut x_axis_f: f64 = x_axis as f64; // save on memory allocations per loop cycle
            while x_axis <= x_axis_max { // for ...; x_axis <= x_axis_max; ...
                let x_min: i32 = x.max(x_axis_f - y - h) as i32;
                let x_max: i32 = (x+w).min(x_axis_f - y) as i32;

                if x_min < x_max {
                    self.add(x_axis, x_min, x_max);
                }
                x_axis += sep as i32; // for ...; ...; x_axis += sep
                x_axis_f = x_axis as f64; // halve the i32/f64 conversions per loop cycle
            }
            // END our "for" loop
        } else {
            let x_axis_min: i32 = (((x - y - h) / sep).floor() * sep) as i32;
            let x_axis_max: i32 = (((x + w - y) / sep).ceil() * sep) as i32;

            let mut x_axis: i32 = x_axis_min;
            let mut x_axis_f: f64 = x_axis as f64;
            while x_axis <= x_axis_max {
                let x_min: i32 = x.max(x_axis_f + y) as i32;
                let x_max: i32 = (x+w).min(x_axis_f + y + h) as i32;

                if x_min < x_max {
                    self.add(x_axis, x_min, x_max);
                }
                x_axis += sep as i32;
                x_axis_f = x_axis as f64;
            }
        }
    }

    // Connect lines of shading with small interruption.
    fn compress(&mut self) {
        for pairs in self.x_to_pairs.values_mut() {
            let mut changed = true;

            while changed {
                for i in 0..pairs.len() {
                    let pair1 = pairs[i];

                    for j in 0..i {
                        let pair2 = pairs[j];

                        if overlap(pair1, pair2) {
                            pairs[j] = join(pair1, pair2);
                            pairs.remove(i);
                            changed = true;
                            break;
                        }
                    }
                }
            }
        }
    }
}

/////////////////////////////////////////////////////////////////////////////
// GEOMETRY.shading

fn overlap(pair1: MinMaxPair, pair2: MinMaxPair) -> bool {
    let margin: i32 = 2; // TODO: Magic number?
    (pair1.min >= pair2.min - margin && pair1.min <= pair2.max + margin) ||
        (pair1.max >= pair2.min - margin && pair1.max <= pair2.max + margin)
}

fn join(pair1: MinMaxPair, pair2: MinMaxPair) -> MinMaxPair {
    MinMaxPair{min: pair1.min.min(pair2.min), max: pair1.max.max(pair2.max)}
}

/*
/////////////////////////////////////////////////////////////////////////////
// Shading.







ResShading.prototype.print =
function(ctx) {
	for (var xAxis in this.xToPairs) {
		var pairs = this.xToPairs[xAxis];
		for (var i = 0; i < pairs.length; i++) {
			var xMin = pairs[i][0];
			var xMax = pairs[i][1];
			if (this.commonPattern()) {
				var yMin = xAxis - xMin;
				var yMax = xAxis - xMax;
			} else {
				var yMin = xMin - xAxis;
				var yMax = xMax - xAxis;
			}
			ctx.strokeStyle = this.resContext.shadingColor;
			ctx.lineWidth = this.resContext.shadingThickness;
			ctx.beginPath();
			ctx.moveTo(xMin,yMin);
			ctx.lineTo(xMax,yMax);
			ctx.stroke();
		}
	}
};
ResShading.shadeBasicgroup =
function(env, g, shadeRect) {
	if (g.shade !== null || g.shades.length > 0) {
		if (g.shade === true)
			env.shading.addRect(shadeRect);
		else {
			for (var i = 0; i < g.shades.length; i++) {
				var s = shadeRect.chopPattern(g.shades[i]);
				env.shading.addRect(s);
			}
		}
	} else if (g.globals.shade)
		env.shading.addRect(shadeRect);
};
ResShading.shadeBox =
function(env, g, shadeRect, innerRect) {
	var x0 = shadeRect.x;
	var x1 = innerRect.x;
	var x2 = innerRect.x + innerRect.width;
	var x3 = shadeRect.x + shadeRect.width;
	var y0 = shadeRect.y;
	var y1 = innerRect.y;
	var y2 = innerRect.y + innerRect.height;
	var y3 = shadeRect.y + shadeRect.height;
	var topSlice = new ResRectangle(x0, y0, x3-x0, y1-y0);
	var bottomSlice = new ResRectangle(x0, y2, x3-x0, y3-y2);
	var leftSlice = new ResRectangle(x0, y0, x1-x0, y3-y0);
	var rightSlice = new ResRectangle(x2, y0, x3-x2, y3-y0);
	var slices = [topSlice, bottomSlice, leftSlice, rightSlice];
	if (g.shade !== null || g.shades.length > 0) {
		if (g.shade === true)
			for (var j = 0; j < slices.length; j++)
				env.shading.addRect(slices[j]);
		else {
			for (var i = 0; i < g.shades.length; i++) {
				var s = shadeRect.chopPattern(g.shades[i]);
				for (var j = 0; j < slices.length; j++) {
					var inters = s.intersect(slices[j]);
					env.shading.addRect(inters);
				}
			}
		}
	} else if (g.globals.shade)
		for (var j = 0; j < slices.length; j++)
			env.shading.addRect(slices[j]);
};

 */