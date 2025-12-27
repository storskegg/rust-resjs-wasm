

/////////////////////////////////////////////////////////////////////////////
// Canvas operations.

function ResCanvas() {
}

// Make canvas with sizes, which must be at least 1.
ResCanvas.make =
function(width, height) {
	var canvas = document.createElement("canvas");
	canvas.width = Math.max(Math.round(width), 1);
	canvas.height = Math.max(Math.round(height), 1);
	return canvas;
};

// Clear canvas.
ResCanvas.clear =
function(canvas) {
	this.ctx = canvas.getContext("2d");
	this.ctx.clearRect(0, 0, canvas.width, canvas.height);
};

// Sum all values from pixel in data from canvas with dimensions.
// return: if there is non-blank pixel.
ResCanvas.isNotBlank =
function(data, width, x, y) {
	return data[y * width * 4 + x * 4 + 3] +
		data[y * width * 4 + x * 4 + 0] +
		data[y * width * 4 + x * 4 + 1] +
		data[y * width * 4 + x * 4 + 2] > 0;
};

// Find external pixels of sign.
// ctx: context of canvas
// w, h: dimensions
// return: 2-dimensional boolean array indicating which pixels are external
ResCanvas.externalPixels =
function(ctx, w, h) {
	var external = new Array(w);
	for (var x = 0; x < w; x++) {
		external[x] = new Array(h);
		for (var y = 0; y < h; y++) 
			external[x][y] = false;
	}
	if (w === 0 || h === 0)
		return external;
	var data = ctx.getImageData(0, 0, w, h).data;
	for (var x = 0; x < w; x++) {
		if (!ResCanvas.isNotBlank(data, w, x, 0))
			external[x][0] = true;
		if (!ResCanvas.isNotBlank(data, w, x, h-1))
			external[x][h-1] = true;
	}
	for (var y = 1; y < h-1; y++) {
		ResCanvas.externalPixelsSideways(data, w, 0, y, external);
		ResCanvas.externalPixelsSideways(data, w, w-1, y, external);
	}
	var changed = true;
	while (changed) {
		changed = false;
		for (var y = 1; y < h-1; y++) 
			for (var x = 1; x < w-1; x++) 
				if (external[x][y-1])
					changed |= ResCanvas.externalPixelsSideways(data, w, x, y, external);
		for (var y = h-2; y > 0; y--) 
			for (var x = 1; x < w-1; x++) 
				if (external[x][y+1])
					changed |= ResCanvas.externalPixelsSideways(data, w, x, y, external);
	}
	return external;
};
ResCanvas.externalPixelsSideways =
function(data, w, x, y, external) {
	if (external[x][y] || ResCanvas.isNotBlank(data, w, x, y)) 
		return false;
	external[x][y] = true; 
	for (var x2 = x-1; x2 >= 1; x2--) {
		if (!external[x2][y] && !ResCanvas.isNotBlank(data, w, x2, y)) 
			external[x2][y] = true; 
		else
			break;
	}
	for (var x2 = x+1; x2 < w-1; x2++) {
		if (!external[x2][y] && !ResCanvas.isNotBlank(data, w, x2, y)) 
			external[x2][y] = true; 
		else
			break;
	}
	return true;
};

// Erase pixels that are not valid. 
// valids: 2-dimensional boolean array 
ResCanvas.erasePixels =
function(ctx, w, h, valids) {
	var white = ctx.createImageData(1,1);
	var data = white.data;
	data[0] = 0;
	data[1] = 0;
	data[2] = 0;
	data[3] = 0;
	for (var x = 0; x < w; x++) 
		for (var y = 0; y < h; y++) 
			if (!valids[x][y]) 
				ctx.putImageData(white, x, y);
};

// Take big-enough canvas for sign, print sign, and measure
// dimensions and location of actual shape.
// str: string
// size: font size in px
// font: string describing font
// return: ResRectangle of bounding glyph as it is actually printed
ResCanvas.glyphRect =
function(str, size, xScale, font, rotate, mirror) {
	var sizedFont = "" + size + "px " + font;
	var measCanvas = document.createElement("canvas");
	var ctx = measCanvas.getContext("2d");
	ctx.font = sizedFont;
	ctx.fillStyle = "black";
	ctx.textBaseline = "alphabetic";
	var width = Math.max(1, Math.round(ctx.measureText(str).width));
	var height = Math.max(1, Math.round(size));
	if (rotate === 0) {
		var lMargin = Math.ceil(width / 5);
		var rMargin = Math.ceil(width / 5);
		var tMargin = Math.ceil(height / 5);
		var bMargin = Math.ceil(height / 2);
	} else {
		var dim = Math.max(width, height);
		var lMargin = Math.ceil(dim / 2);
		var rMargin = Math.ceil(dim / 2);
		var tMargin = Math.ceil(dim / 2);
		var bMargin = Math.ceil(dim / 2);
	}
	var maxRepeats = 3;
	for (var i = 0; i < maxRepeats; i++) {
		var w = width + lMargin + rMargin;
		var h = height + tMargin + bMargin;
		measCanvas.width = w;
		measCanvas.height = h;
		ctx = measCanvas.getContext("2d");
		ctx.font = sizedFont;
		ctx.fillStyle = "black";
		ctx.textBaseline = "alphabetic";
		if (rotate === 0 && !mirror && xScale === 1)
			ctx.fillText(str, lMargin, height + tMargin);
		else {
			ctx.translate(lMargin + width/2, tMargin + height/2);
			ctx.rotate(rotate*Math.PI/180);
			ctx.scale((mirror ? -1 : 1) * xScale, 1);
			ctx.fillText(str, -width/2, height/2);
		}
		var data = ctx.getImageData(0, 0, w, h).data;
		var margins = ResCanvas.margins(data, w, h);
		if (margins.t > 0 && margins.b > 0 && margins.l > 0 && margins.r > 0)
			break;
		lMargin *= 2;
		rMargin *= 2;
		tMargin *= 2;
		bMargin *= 2;
	}
	var realHeight = h - margins.t - margins.b;
	var realWidth = w - margins.l - margins.r;
	return new ResRectangle(margins.l - lMargin, bMargin - margins.b,
				realWidth, realHeight);
};
ResCanvas.printGlyph =
function(ctx, x, y, testRect, str, size, xScale, font, color, rotate, mirror) {
	var sizedFont = "" + size + "px " + font;
	ctx.save();
	ctx.font = sizedFont;
	ctx.fillStyle = color;
	ctx.textBaseline = "alphabetic";
	var width = Math.max(1, Math.round(ctx.measureText(str).width));
	var height = Math.max(1, Math.round(size));
	if (rotate === 0 && !mirror && xScale === 1) 
		ctx.fillText(str, x - testRect.x, y - testRect.y + testRect.height);
	else {
		var l = testRect.x;
		var r = width - l - testRect.width;
		var b = -testRect.y;
		var t = height - b - testRect.height;
		ctx.translate(x + width/2 - testRect.x, y - height/2 + testRect.height - testRect.y);
		ctx.rotate(rotate*Math.PI/180);
		ctx.scale((mirror ? -1 : 1) * xScale, 1);
		ctx.fillText(str, -width/2, height/2);
	}
	ctx.restore();
};

// Compute margins in data of canvas.
ResCanvas.margins =
function(data, w, h) {
	var t = 0;
	for (var y = 0; y < h; y++) {
		var rowEmpty = true;
		for (var x = 0; x < w; x++)
			if (ResCanvas.isNotBlank(data, w, x, y)) {
				rowEmpty = false;
				break;
			}
		if (rowEmpty)
			t++;
		else
			break;
	}
	var b = 0;
	for (var y = h-1; y >= 0; y--) {
		var rowEmpty = true;
		for (var x = 0; x < w; x++)
			if (ResCanvas.isNotBlank(data, w, x, y)) {
				rowEmpty = false;
				break;
			}
		if (rowEmpty)
			b++;
		else
			break;
	}
	var l = 0;
	for (var x = 0; x < w; x++) {
		var colEmpty = true;
		for (var y = 0; y < h; y++)
			if (ResCanvas.isNotBlank(data, w, x, y)) {
				colEmpty = false;
				break;
			}
		if (colEmpty)
			l++;
		else
			break;
	}
	var r = 0;
	for (var x = w-1; x >= 0; x--) {
		var colEmpty = true;
		for (var y = 0; y < h; y++)
			if (ResCanvas.isNotBlank(data, w, x, y)) {
				colEmpty = false;
				break;
			}
		if (colEmpty)
			r++;
		else
			break;
	}
	return {t: t, b: b, l: l, r: r};
};

// Take two images and compute how far they can approach while leaving sep
// between.
// First make aura around left-most pixels of second image.
ResCanvas.fitHor =
function(ctx1, ctx2, w1, w2, h, sepInit, sepMax) {
	var data1 = w1 > 0 && h > 0 ? ctx1.getImageData(0, 0, w1, h).data : null;
	var data2 = w2 > 0 && h > 0 ? ctx2.getImageData(0, 0, w2, h).data : null;
	var w = w2 + sepInit;
	var canvas = document.createElement("canvas");
	canvas.width = w;
	canvas.height = h;
	var ctx = canvas.getContext("2d");
	for (var y = 0; y < h; y++) 
		for (var x = 0; x < w2; x++) 
			if (ResCanvas.isNotBlank(data2, w2, x, y)) {
				ctx.fillStyle = "black";
				ctx.beginPath();
				ctx.arc(sepInit + x, y, Math.max(0.5,sepInit), 0.5*Math.PI, 1.5*Math.PI);
				ctx.stroke();
				break;
			}
	var data = w > 0 && h > 0 ? ctx.getImageData(0, 0, w, h).data : null;
	dist = -sepMax;
	for (var y = 0; y < h; y++) {
		for (var spLeft = 0; spLeft < w1; spLeft++)
			if (ResCanvas.isNotBlank(data1, w1, w1-1-spLeft, y))
				break;
		for (var spRight = 0; spRight < w; spRight++)
			if (ResCanvas.isNotBlank(data, w, spRight, y))
				break;
		if (spLeft < w1 && spRight < w)
			dist = Math.max(dist, sepInit - spLeft - spRight);
	}
	return dist;
};
ResCanvas.fitVert =
function(ctx1, ctx2, w, h1, h2, sepInit, sepMax) {
	var data1 = w > 0 && h1 > 0 ? ctx1.getImageData(0, 0, w, h1).data : null;
	var data2 = w > 0 && h2 > 0 ? ctx2.getImageData(0, 0, w, h2).data : null;
	var h = h2 + sepInit;
	var canvas = document.createElement("canvas");
	canvas.width = w;
	canvas.height = h;
	var ctx = canvas.getContext("2d");
	for (var x = 0; x < w; x++) 
		for (var y = 0; y < h2; y++) 
			if (ResCanvas.isNotBlank(data2, w, x, y)) {
				ctx.beginPath();
				ctx.arc(x, sepInit + y, Math.max(0.5,sepInit), 1.0*Math.PI, 2.0*Math.PI);
				ctx.stroke();
				break;
			}
	var data = w > 0 && h > 0 ? ctx.getImageData(0, 0, w, h).data : null;
	dist = -sepMax;
	for (var x = 0; x < w; x++) {
		for (var spTop = 0; spTop < h1; spTop++)
			if (ResCanvas.isNotBlank(data1, w, x, h1-1-spTop))
				break;
		for (var spBottom = 0; spBottom < h; spBottom++)
			if (ResCanvas.isNotBlank(data, w, x, spBottom))
				break;
		if (spTop < h1 && spBottom < h)
			dist = Math.max(dist, sepInit - spTop - spBottom);
	}
	return dist;
};

// Draw multiple times around coordinate.
ResCanvas.drawWithAura =
function(ctx, canvas, x, y, w, h, sep) {
	for (var angle = 0; angle < 0.99; angle += 1/8) {
		var p = ResPoint.fromAngle(angle, sep);
		ctx.drawImage(canvas, x + p.x, y + p.y, w, h);
	}
};

// Take two images of same size and compute whether they have black pixel in common.
ResCanvas.disjoint =
function(ctx1, ctx2, w, h) {
	if (w === 0 || h === 0)
		return false;
	var data1 = ctx1.getImageData(0, 0, w, h).data;
	var data2 = ctx2.getImageData(0, 0, w, h).data;
	for (var x = 0; x < w; x++)
		for (var y = 0; y < h; y++)
			if (ResCanvas.isNotBlank(data1, w, x, y) && ResCanvas.isNotBlank(data2, w, x, y))
				return false;
	return true;
};

// Internal space in horizontal box. 
ResCanvas.internalHor =
function(ctx, w, h) {
	var data = w > 0 && h > 0 ? ctx.getImageData(0, 0, w, h).data : null;
	var t = 0;
	for (var y = Math.round(1/3*h); y >= 0 && t === 0; y--)
		for (var x = 0; x < w; x++)
			if (ResCanvas.isNotBlank(data, w, x, y)) {
				t = y+1;
				break;
			}
	var b = 0;
	for (var y = Math.round(2/3*h); y < h && b === 0; y++)
		for (var x = 0; x < w; x++)
			if (ResCanvas.isNotBlank(data, w, x, y)) {
				b = h-y;
				break;
			}
	return {over: t, under: b};
};
ResCanvas.internalVert =
function(ctx, w, h) {
	var data = w > 0 && h > 0 ? ctx.getImageData(0, 0, w, h).data : null;
	var l = 0;
	for (var x = Math.round(1/3*w); x >= 0 && l === 0; x--)
		for (var y = 0; y < h; y++)
			if (ResCanvas.isNotBlank(data, w, x, y)) {
				l = x+1;
				break;
			}
	var r = 0;
	for (var x = Math.round(2/3*w); x < w && r === 0; x++)
		for (var y = 0; y < h; y++)
			if (ResCanvas.isNotBlank(data, w, x, y)) {
				r = w-x;
				break;
			}
	return {over: l, under: r};
};

// Find rectangle of given width within rectangle that is complete blank.
// Find it from right to left.
// return: top-left point, or false if unsuccesful.
ResCanvas.findFreeRectRightLeft =
function(ctx, w, h, rect, width) {
	var data = w > 0 && h > 0 ? ctx.getImageData(0, 0, w, h).data : null;
	var n = 0;
	for (var x = rect.x + rect.width-1; x >= rect.x; x--) {
		var isBlank = true;
		for (var y = rect.y; y < rect.y + rect.height; y++)
			if (0 <= x && x < w && 0 <= y && y < h && ResCanvas.isNotBlank(data, w, x, y)) {
				isBlank = false;
				break;
			}
		if (isBlank) {
			n++;
			if (n >= width)
				return new ResPoint(x, rect.y);
		} else 
			n = 0;
	}
	return false;
};
ResCanvas.findFreeRectLeftRight =
function(ctx, w, h, rect, width) {
	var data = w > 0 && h > 0 ? ctx.getImageData(0, 0, w, h).data : null;
	var n = 0;
	for (var x = rect.x; x < rect.x + rect.width; x++) {
		var isBlank = true;
		for (var y = rect.y; y < rect.y + rect.height; y++)
			if (0 <= x && x < w && 0 <= y && y < h && ResCanvas.isNotBlank(data, w, x, y)) {
				isBlank = false;
				break;
			}
		if (isBlank) {
			n++;
			if (n >= width)
				return new ResPoint(x-width, rect.y);
		} else 
			n = 0;
	}
	return false;
};
ResCanvas.findFreeRectBottomTop =
function(ctx, w, h, rect, height) {
	var data = w > 0 && h > 0 ? ctx.getImageData(0, 0, w, h).data : null;
	var n = 0;
	for (var y = rect.y + rect.height-1; y >= rect.y; y--) {
		var isBlank = true;
		for (var x = rect.x; x < rect.x + rect.width; x++)
			if (0 <= x && x < w && 0 <= y && y < h && ResCanvas.isNotBlank(data, w, x, y)) {
				isBlank = false;
				break;
			}
		if (isBlank) {
			n++;
			if (n >= height)
				return new ResPoint(rect.x, y);
		} else 
			n = 0;
	}
	return false;
};
ResCanvas.findFreeRectTopBottom =
function(ctx, w, h, rect, height) {
	var data = w > 0 && h > 0 ? ctx.getImageData(0, 0, w, h).data : null;
	var n = 0;
	for (var y = rect.y; y < rect.y + rect.height; y++) {
		var isBlank = true;
		for (var x = rect.x; x < rect.x + rect.width; x++)
			if (0 <= x && x < w && 0 <= y && y < h && ResCanvas.isNotBlank(data, w, x, y)) {
				isBlank = false;
				break;
			}
		if (isBlank) {
			n++;
			if (n >= height)
				return new ResPoint(rect.x, y-height);
		} else 
			n = 0;
	}
	return false;
};

/////////////////////////////////////////////////////////////////////////////
// Shading.

// Maps x coordinate (on x axis) to list of pairs of x coordinates 
// each delimiting a line segment of shading (hatching).
// context: ResContext
// mirror: boolean (whether mirrored)
// width: of canvas (in px)
function ResShading(context, mirror) {
	this.resContext = context;
	this.mirror = mirror;
	this.xToPairs = {};
}
ResShading.prototype.commonPattern =
function() {
	return this.resContext.shadingPattern === "x_is_y" && !this.mirror ||
		this.resContext.shadingPattern === "x_is_minus_y" && this.mirror;
};
ResShading.prototype.add =
function(x, xMin, xMax) {
	if (this.xToPairs[x] === undefined)
		this.xToPairs[x] = [];
	this.xToPairs[x].push([xMin, xMax]);
};
// rect: ResRectangle
ResShading.prototype.addRect =
function(rect) {
	var x = rect.x;
	var y = rect.y;
	var w = rect.width;
	var h = rect.height
	var sep = this.resContext.shadingSep;
	if (this.commonPattern()) {
		var xAxisMin = Math.floor((x + y) / sep) * sep;
		var xAxisMax = Math.ceil(x + w + y + h);
		for (var xAxis = xAxisMin; xAxis <= xAxisMax; xAxis += sep) {
			var xMin = Math.max(x, xAxis - y - h);
			var xMax = Math.min(x + w, xAxis - y);
			if (xMin < xMax)
				this.add(xAxis, xMin, xMax);
		}
	} else {
		var xAxisMin = Math.floor((x - y - h) / sep) * sep;
		var xAxisMax = Math.ceil(x + w - y);
		for (var xAxis = xAxisMin; xAxis <= xAxisMax; xAxis += sep) {
			var xMin = Math.max(x, xAxis + y);
			var xMax = Math.min(x + w, xAxis + y + h);
			if (xMin < xMax)
				this.add(xAxis, xMin, xMax);
		}
	}
};
// Connect lines of shading with small interruption.
ResShading.prototype.compress =
function() {
	for (var x in this.xToPairs) {
		var pairs = this.xToPairs[x];
		var changed = true;
		while (changed) {
			changed = false;
			for (var i = 0; i < pairs.length; i++) {
				var pair1 = pairs[i];
				for (var j = 0; j < i; j++) {
					var pair2 = pairs[j];
					if (this.overlap(pair1, pair2)) {
						pairs[j] = this.join(pair1, pair2);
						pairs.splice(i, 1);
						i--;
						changed = true;
						break;
					}
				}
			}
		}
	}
};
ResShading.prototype.overlap =
function(pair1, pair2) {
	var margin = 2;
	return (pair1[0] >= pair2[0] - margin && pair1[0] <= pair2[1] + margin) ||
		(pair1[1] >= pair2[0] - margin && pair1[1] <= pair2[1] + margin);
};
ResShading.prototype.join =
function(pair1, pair2) {
	return [Math.min(pair1[0], pair2[0]), Math.max(pair1[1], pair2[1])];
};
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
