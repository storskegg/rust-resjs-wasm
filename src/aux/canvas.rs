// use wasm_bindgen::prelude::wasm_bindgen;
// // use web_sys::{self, Element};
// use crate::get_document;
//
// #[wasm_bindgen]
// struct Canvas {
//     el_ctx: Option<web_sys::CanvasRenderingContext2d>,
// }
//
// impl Canvas {
//     pub fn new() -> Self {
//         Self {
//             el_ctx: None,
//         }
//     }
//
//     /// Make canvas with sizes, which must be at least 1.
//     // fn make(width: f64, height: f64) -> &'static Element {
//     //     let _document = get_document();
//     //
//     //     let _width: f64 = width.max(1.0);
//     //     let _height: f64 = height.max(1.0);
//     //
//     //     let el_canvas = _document.create_element("canvas").expect("could not create <canvas>");
//     //     el_canvas.set_attribute("width", &format!("{}", _width)).expect("could not set width of <canvas>");
//     //     el_canvas.set_attribute("height", &format!("{}", _height)).expect("could not set height of <canvas>");
//     //
//     //     &el_canvas
//     // }
//
//     /// Clear canvas.
//     fn clear(&mut self, canvas: &Element) {
//         self.el_ctx = canvas.get_context("2d");
//         self.el_ctx.unwrap().clear_rect(0.0, 0.0, canvas.width(), canvas.height());
//     }
//
//     // fn is_not_blank(&self, data) [}]
// }
//
// /*
// /////////////////////////////////////////////////////////////////////////////
// // Canvas operations.
//
//
//
//
//
//
//
// // Sum all values from pixel in data from canvas with dimensions.
// // return: if there is non-blank pixel.
// ResCanvas.isNotBlank =
// function(data, width, x, y) {
// 	return data[y * width * 4 + x * 4 + 3] +
// 		data[y * width * 4 + x * 4 + 0] +
// 		data[y * width * 4 + x * 4 + 1] +
// 		data[y * width * 4 + x * 4 + 2] > 0;
// };
//
// // Find external pixels of sign.
// // ctx: context of canvas
// // w, h: dimensions
// // return: 2-dimensional boolean array indicating which pixels are external
// ResCanvas.externalPixels =
// function(ctx, w, h) {
// 	var external = new Array(w);
// 	for (var x = 0; x < w; x++) {
// 		external[x] = new Array(h);
// 		for (var y = 0; y < h; y++)
// 			external[x][y] = false;
// 	}
// 	if (w === 0 || h === 0)
// 		return external;
// 	var data = ctx.getImageData(0, 0, w, h).data;
// 	for (var x = 0; x < w; x++) {
// 		if (!ResCanvas.isNotBlank(data, w, x, 0))
// 			external[x][0] = true;
// 		if (!ResCanvas.isNotBlank(data, w, x, h-1))
// 			external[x][h-1] = true;
// 	}
// 	for (var y = 1; y < h-1; y++) {
// 		ResCanvas.externalPixelsSideways(data, w, 0, y, external);
// 		ResCanvas.externalPixelsSideways(data, w, w-1, y, external);
// 	}
// 	var changed = true;
// 	while (changed) {
// 		changed = false;
// 		for (var y = 1; y < h-1; y++)
// 			for (var x = 1; x < w-1; x++)
// 				if (external[x][y-1])
// 					changed |= ResCanvas.externalPixelsSideways(data, w, x, y, external);
// 		for (var y = h-2; y > 0; y--)
// 			for (var x = 1; x < w-1; x++)
// 				if (external[x][y+1])
// 					changed |= ResCanvas.externalPixelsSideways(data, w, x, y, external);
// 	}
// 	return external;
// };
// ResCanvas.externalPixelsSideways =
// function(data, w, x, y, external) {
// 	if (external[x][y] || ResCanvas.isNotBlank(data, w, x, y))
// 		return false;
// 	external[x][y] = true;
// 	for (var x2 = x-1; x2 >= 1; x2--) {
// 		if (!external[x2][y] && !ResCanvas.isNotBlank(data, w, x2, y))
// 			external[x2][y] = true;
// 		else
// 			break;
// 	}
// 	for (var x2 = x+1; x2 < w-1; x2++) {
// 		if (!external[x2][y] && !ResCanvas.isNotBlank(data, w, x2, y))
// 			external[x2][y] = true;
// 		else
// 			break;
// 	}
// 	return true;
// };
//
// // Erase pixels that are not valid.
// // valids: 2-dimensional boolean array
// ResCanvas.erasePixels =
// function(ctx, w, h, valids) {
// 	var white = ctx.createImageData(1,1);
// 	var data = white.data;
// 	data[0] = 0;
// 	data[1] = 0;
// 	data[2] = 0;
// 	data[3] = 0;
// 	for (var x = 0; x < w; x++)
// 		for (var y = 0; y < h; y++)
// 			if (!valids[x][y])
// 				ctx.putImageData(white, x, y);
// };
//
// // Take big-enough canvas for sign, print sign, and measure
// // dimensions and location of actual shape.
// // str: string
// // size: font size in px
// // font: string describing font
// // return: ResRectangle of bounding glyph as it is actually printed
// ResCanvas.glyphRect =
// function(str, size, xScale, font, rotate, mirror) {
// 	var sizedFont = "" + size + "px " + font;
// 	var measCanvas = document.createElement("canvas");
// 	var ctx = measCanvas.getContext("2d");
// 	ctx.font = sizedFont;
// 	ctx.fillStyle = "black";
// 	ctx.textBaseline = "alphabetic";
// 	var width = Math.max(1, Math.round(ctx.measureText(str).width));
// 	var height = Math.max(1, Math.round(size));
// 	if (rotate === 0) {
// 		var lMargin = Math.ceil(width / 5);
// 		var rMargin = Math.ceil(width / 5);
// 		var tMargin = Math.ceil(height / 5);
// 		var bMargin = Math.ceil(height / 2);
// 	} else {
// 		var dim = Math.max(width, height);
// 		var lMargin = Math.ceil(dim / 2);
// 		var rMargin = Math.ceil(dim / 2);
// 		var tMargin = Math.ceil(dim / 2);
// 		var bMargin = Math.ceil(dim / 2);
// 	}
// 	var maxRepeats = 3;
// 	for (var i = 0; i < maxRepeats; i++) {
// 		var w = width + lMargin + rMargin;
// 		var h = height + tMargin + bMargin;
// 		measCanvas.width = w;
// 		measCanvas.height = h;
// 		ctx = measCanvas.getContext("2d");
// 		ctx.font = sizedFont;
// 		ctx.fillStyle = "black";
// 		ctx.textBaseline = "alphabetic";
// 		if (rotate === 0 && !mirror && xScale === 1)
// 			ctx.fillText(str, lMargin, height + tMargin);
// 		else {
// 			ctx.translate(lMargin + width/2, tMargin + height/2);
// 			ctx.rotate(rotate*Math.PI/180);
// 			ctx.scale((mirror ? -1 : 1) * xScale, 1);
// 			ctx.fillText(str, -width/2, height/2);
// 		}
// 		var data = ctx.getImageData(0, 0, w, h).data;
// 		var margins = ResCanvas.margins(data, w, h);
// 		if (margins.t > 0 && margins.b > 0 && margins.l > 0 && margins.r > 0)
// 			break;
// 		lMargin *= 2;
// 		rMargin *= 2;
// 		tMargin *= 2;
// 		bMargin *= 2;
// 	}
// 	var realHeight = h - margins.t - margins.b;
// 	var realWidth = w - margins.l - margins.r;
// 	return new ResRectangle(margins.l - lMargin, bMargin - margins.b,
// 				realWidth, realHeight);
// };
// ResCanvas.printGlyph =
// function(ctx, x, y, testRect, str, size, xScale, font, color, rotate, mirror) {
// 	var sizedFont = "" + size + "px " + font;
// 	ctx.save();
// 	ctx.font = sizedFont;
// 	ctx.fillStyle = color;
// 	ctx.textBaseline = "alphabetic";
// 	var width = Math.max(1, Math.round(ctx.measureText(str).width));
// 	var height = Math.max(1, Math.round(size));
// 	if (rotate === 0 && !mirror && xScale === 1)
// 		ctx.fillText(str, x - testRect.x, y - testRect.y + testRect.height);
// 	else {
// 		var l = testRect.x;
// 		var r = width - l - testRect.width;
// 		var b = -testRect.y;
// 		var t = height - b - testRect.height;
// 		ctx.translate(x + width/2 - testRect.x, y - height/2 + testRect.height - testRect.y);
// 		ctx.rotate(rotate*Math.PI/180);
// 		ctx.scale((mirror ? -1 : 1) * xScale, 1);
// 		ctx.fillText(str, -width/2, height/2);
// 	}
// 	ctx.restore();
// };
//
// // Compute margins in data of canvas.
// ResCanvas.margins =
// function(data, w, h) {
// 	var t = 0;
// 	for (var y = 0; y < h; y++) {
// 		var rowEmpty = true;
// 		for (var x = 0; x < w; x++)
// 			if (ResCanvas.isNotBlank(data, w, x, y)) {
// 				rowEmpty = false;
// 				break;
// 			}
// 		if (rowEmpty)
// 			t++;
// 		else
// 			break;
// 	}
// 	var b = 0;
// 	for (var y = h-1; y >= 0; y--) {
// 		var rowEmpty = true;
// 		for (var x = 0; x < w; x++)
// 			if (ResCanvas.isNotBlank(data, w, x, y)) {
// 				rowEmpty = false;
// 				break;
// 			}
// 		if (rowEmpty)
// 			b++;
// 		else
// 			break;
// 	}
// 	var l = 0;
// 	for (var x = 0; x < w; x++) {
// 		var colEmpty = true;
// 		for (var y = 0; y < h; y++)
// 			if (ResCanvas.isNotBlank(data, w, x, y)) {
// 				colEmpty = false;
// 				break;
// 			}
// 		if (colEmpty)
// 			l++;
// 		else
// 			break;
// 	}
// 	var r = 0;
// 	for (var x = w-1; x >= 0; x--) {
// 		var colEmpty = true;
// 		for (var y = 0; y < h; y++)
// 			if (ResCanvas.isNotBlank(data, w, x, y)) {
// 				colEmpty = false;
// 				break;
// 			}
// 		if (colEmpty)
// 			r++;
// 		else
// 			break;
// 	}
// 	return {t: t, b: b, l: l, r: r};
// };
//
// // Take two images and compute how far they can approach while leaving sep
// // between.
// // First make aura around left-most pixels of second image.
// ResCanvas.fitHor =
// function(ctx1, ctx2, w1, w2, h, sepInit, sepMax) {
// 	var data1 = w1 > 0 && h > 0 ? ctx1.getImageData(0, 0, w1, h).data : null;
// 	var data2 = w2 > 0 && h > 0 ? ctx2.getImageData(0, 0, w2, h).data : null;
// 	var w = w2 + sepInit;
// 	var canvas = document.createElement("canvas");
// 	canvas.width = w;
// 	canvas.height = h;
// 	var ctx = canvas.getContext("2d");
// 	for (var y = 0; y < h; y++)
// 		for (var x = 0; x < w2; x++)
// 			if (ResCanvas.isNotBlank(data2, w2, x, y)) {
// 				ctx.fillStyle = "black";
// 				ctx.beginPath();
// 				ctx.arc(sepInit + x, y, Math.max(0.5,sepInit), 0.5*Math.PI, 1.5*Math.PI);
// 				ctx.stroke();
// 				break;
// 			}
// 	var data = w > 0 && h > 0 ? ctx.getImageData(0, 0, w, h).data : null;
// 	dist = -sepMax;
// 	for (var y = 0; y < h; y++) {
// 		for (var spLeft = 0; spLeft < w1; spLeft++)
// 			if (ResCanvas.isNotBlank(data1, w1, w1-1-spLeft, y))
// 				break;
// 		for (var spRight = 0; spRight < w; spRight++)
// 			if (ResCanvas.isNotBlank(data, w, spRight, y))
// 				break;
// 		if (spLeft < w1 && spRight < w)
// 			dist = Math.max(dist, sepInit - spLeft - spRight);
// 	}
// 	return dist;
// };
// ResCanvas.fitVert =
// function(ctx1, ctx2, w, h1, h2, sepInit, sepMax) {
// 	var data1 = w > 0 && h1 > 0 ? ctx1.getImageData(0, 0, w, h1).data : null;
// 	var data2 = w > 0 && h2 > 0 ? ctx2.getImageData(0, 0, w, h2).data : null;
// 	var h = h2 + sepInit;
// 	var canvas = document.createElement("canvas");
// 	canvas.width = w;
// 	canvas.height = h;
// 	var ctx = canvas.getContext("2d");
// 	for (var x = 0; x < w; x++)
// 		for (var y = 0; y < h2; y++)
// 			if (ResCanvas.isNotBlank(data2, w, x, y)) {
// 				ctx.beginPath();
// 				ctx.arc(x, sepInit + y, Math.max(0.5,sepInit), 1.0*Math.PI, 2.0*Math.PI);
// 				ctx.stroke();
// 				break;
// 			}
// 	var data = w > 0 && h > 0 ? ctx.getImageData(0, 0, w, h).data : null;
// 	dist = -sepMax;
// 	for (var x = 0; x < w; x++) {
// 		for (var spTop = 0; spTop < h1; spTop++)
// 			if (ResCanvas.isNotBlank(data1, w, x, h1-1-spTop))
// 				break;
// 		for (var spBottom = 0; spBottom < h; spBottom++)
// 			if (ResCanvas.isNotBlank(data, w, x, spBottom))
// 				break;
// 		if (spTop < h1 && spBottom < h)
// 			dist = Math.max(dist, sepInit - spTop - spBottom);
// 	}
// 	return dist;
// };
//
// // Draw multiple times around coordinate.
// ResCanvas.drawWithAura =
// function(ctx, canvas, x, y, w, h, sep) {
// 	for (var angle = 0; angle < 0.99; angle += 1/8) {
// 		var p = ResPoint.fromAngle(angle, sep);
// 		ctx.drawImage(canvas, x + p.x, y + p.y, w, h);
// 	}
// };
//
// // Take two images of same size and compute whether they have black pixel in common.
// ResCanvas.disjoint =
// function(ctx1, ctx2, w, h) {
// 	if (w === 0 || h === 0)
// 		return false;
// 	var data1 = ctx1.getImageData(0, 0, w, h).data;
// 	var data2 = ctx2.getImageData(0, 0, w, h).data;
// 	for (var x = 0; x < w; x++)
// 		for (var y = 0; y < h; y++)
// 			if (ResCanvas.isNotBlank(data1, w, x, y) && ResCanvas.isNotBlank(data2, w, x, y))
// 				return false;
// 	return true;
// };
//
// // Internal space in horizontal box.
// ResCanvas.internalHor =
// function(ctx, w, h) {
// 	var data = w > 0 && h > 0 ? ctx.getImageData(0, 0, w, h).data : null;
// 	var t = 0;
// 	for (var y = Math.round(1/3*h); y >= 0 && t === 0; y--)
// 		for (var x = 0; x < w; x++)
// 			if (ResCanvas.isNotBlank(data, w, x, y)) {
// 				t = y+1;
// 				break;
// 			}
// 	var b = 0;
// 	for (var y = Math.round(2/3*h); y < h && b === 0; y++)
// 		for (var x = 0; x < w; x++)
// 			if (ResCanvas.isNotBlank(data, w, x, y)) {
// 				b = h-y;
// 				break;
// 			}
// 	return {over: t, under: b};
// };
// ResCanvas.internalVert =
// function(ctx, w, h) {
// 	var data = w > 0 && h > 0 ? ctx.getImageData(0, 0, w, h).data : null;
// 	var l = 0;
// 	for (var x = Math.round(1/3*w); x >= 0 && l === 0; x--)
// 		for (var y = 0; y < h; y++)
// 			if (ResCanvas.isNotBlank(data, w, x, y)) {
// 				l = x+1;
// 				break;
// 			}
// 	var r = 0;
// 	for (var x = Math.round(2/3*w); x < w && r === 0; x++)
// 		for (var y = 0; y < h; y++)
// 			if (ResCanvas.isNotBlank(data, w, x, y)) {
// 				r = w-x;
// 				break;
// 			}
// 	return {over: l, under: r};
// };
//
// // Find rectangle of given width within rectangle that is complete blank.
// // Find it from right to left.
// // return: top-left point, or false if unsuccesful.
// ResCanvas.findFreeRectRightLeft =
// function(ctx, w, h, rect, width) {
// 	var data = w > 0 && h > 0 ? ctx.getImageData(0, 0, w, h).data : null;
// 	var n = 0;
// 	for (var x = rect.x + rect.width-1; x >= rect.x; x--) {
// 		var isBlank = true;
// 		for (var y = rect.y; y < rect.y + rect.height; y++)
// 			if (0 <= x && x < w && 0 <= y && y < h && ResCanvas.isNotBlank(data, w, x, y)) {
// 				isBlank = false;
// 				break;
// 			}
// 		if (isBlank) {
// 			n++;
// 			if (n >= width)
// 				return new ResPoint(x, rect.y);
// 		} else
// 			n = 0;
// 	}
// 	return false;
// };
// ResCanvas.findFreeRectLeftRight =
// function(ctx, w, h, rect, width) {
// 	var data = w > 0 && h > 0 ? ctx.getImageData(0, 0, w, h).data : null;
// 	var n = 0;
// 	for (var x = rect.x; x < rect.x + rect.width; x++) {
// 		var isBlank = true;
// 		for (var y = rect.y; y < rect.y + rect.height; y++)
// 			if (0 <= x && x < w && 0 <= y && y < h && ResCanvas.isNotBlank(data, w, x, y)) {
// 				isBlank = false;
// 				break;
// 			}
// 		if (isBlank) {
// 			n++;
// 			if (n >= width)
// 				return new ResPoint(x-width, rect.y);
// 		} else
// 			n = 0;
// 	}
// 	return false;
// };
// ResCanvas.findFreeRectBottomTop =
// function(ctx, w, h, rect, height) {
// 	var data = w > 0 && h > 0 ? ctx.getImageData(0, 0, w, h).data : null;
// 	var n = 0;
// 	for (var y = rect.y + rect.height-1; y >= rect.y; y--) {
// 		var isBlank = true;
// 		for (var x = rect.x; x < rect.x + rect.width; x++)
// 			if (0 <= x && x < w && 0 <= y && y < h && ResCanvas.isNotBlank(data, w, x, y)) {
// 				isBlank = false;
// 				break;
// 			}
// 		if (isBlank) {
// 			n++;
// 			if (n >= height)
// 				return new ResPoint(rect.x, y);
// 		} else
// 			n = 0;
// 	}
// 	return false;
// };
// ResCanvas.findFreeRectTopBottom =
// function(ctx, w, h, rect, height) {
// 	var data = w > 0 && h > 0 ? ctx.getImageData(0, 0, w, h).data : null;
// 	var n = 0;
// 	for (var y = rect.y; y < rect.y + rect.height; y++) {
// 		var isBlank = true;
// 		for (var x = rect.x; x < rect.x + rect.width; x++)
// 			if (0 <= x && x < w && 0 <= y && y < h && ResCanvas.isNotBlank(data, w, x, y)) {
// 				isBlank = false;
// 				break;
// 			}
// 		if (isBlank) {
// 			n++;
// 			if (n >= height)
// 				return new ResPoint(rect.x, y-height);
// 		} else
// 			n = 0;
// 	}
// 	return false;
// };
//  */