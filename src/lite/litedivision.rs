pub struct LiteDivision {}

impl LiteDivision {
    pub fn new() -> Self {
        Self {}
    }
}

/*

/////////////////////////////////////////////////////////////////////////////
// Formatting.

function ResLiteDivision() {
	this.original = null; // complete ResLite before dividing
	this.resContext = null; // ResContext
	this.initialNumber = 0; // that fit within limit
	this.initialSizeMilEm = 0; // size of initial prefix, in 1000 * EM
	this.padMilEm = 0; // padding between groups, in 1000 * EM
	this.remainder; // remaining ResLite that does not fit within limit.
}
ResLiteDivision.prototype.isH =
function() {
	return this.original.isH();
};
ResLiteDivision.prototype.isLR =
function() {
	return this.original.isLR() && this.resContext.dir === undefined ||
		this.resContext === "lr";
};
ResLiteDivision.prototype.getWidthMilEm = 
function() {
	var pad = this.initialNumber > 1 ? this.padMilEm * (this.initialNumber-1) : 0;
	return this.isH() ? this.initialSizeMilEm + pad : this.original.sizeMilEm;
};
ResLiteDivision.prototype.getHeightMilEm = 
function() {
	var pad = this.initialNumber > 1 ? this.padMilEm * (this.initialNumber-1) : 0;
	return this.isH() ? this.original.sizeMilEm : this.initialSizeMilEm + pad;
};
ResLiteDivision.prototype.getWidthPx = 
function() {
	return Math.round(this.resContext.milEmToPx(this.getWidthMilEm())); 
};
ResLiteDivision.prototype.getHeightPx =
function() {
	return Math.round(this.resContext.milEmToPx(this.getHeightMilEm())); 
};
// Process input until length limit is reached.
// Limit can be infinite (= no limit).
ResLiteDivision.makeTo =
function(resLite, lenPx, context) {
	var div = new ResLiteDivision();
	div.original = resLite;
	div.resContext = context;
	var lenMilEm = context.pxToMilEm(lenPx);
	ResLiteDivision.makeToMilEm(div, lenMilEm);
	if (context.paddingAllowed &&
			lenMilEm !== Number.MAX_VALUE &&
			div.initialNumber > 1) {
		var diff = lenMilEm - div.initialSizeMilEm;
		var maxDiff = 1000 * context.paddingFactor * context.opSepEm * (div.initialNumber-1);
		if (maxDiff >= diff)
			div.padMilEm = diff / (div.initialNumber-1);
	}
	return div;
};
ResLiteDivision.make =
function(resLite, resContext) {
	return ResLiteDivision.makeTo(resLite, Number.MAX_VALUE, resContext);
};
// Process until limit. Store remainder.
ResLiteDivision.makeToMilEm =
function(div, lenMilEm) {
	var groups = div.original.groups;
	div.initialNumber = 0;
	div.initialSizeMilEm = 0;
	var start = 0;
	var isFirst = true;
	while (groups !== null) {
		var advance = (isFirst ? 0 : groups.advanceMilEm);
		if (start + advance + groups.lengthMilEm <= lenMilEm) {
			start += advance;
			div.initialNumber++;
			div.initialSizeMilEm = start + groups.lengthMilEm;
			groups = groups.tl;
			isFirst = false;
		} else
			break;
	}
	div.remainder = new ResLite(div.original.dir, div.original.sizeMilEm, groups);
};

/////////////////////////////////////////////////////////////
// Rendering.

// Render with initial margin. If things were printed outside margin 
// (so if margin became bigger) do again.
ResLiteDivision.prototype.render = 
function(canvas) {
	this.env = new ResEnv(this.resContext);
	while (true) {
		this.env.totalWidthPx = this.getWidthPx() + this.env.leftMarginPx + this.env.rightMarginPx;
		this.env.totalHeightPx = this.getHeightPx() + this.env.topMarginPx + this.env.bottomMarginPx;
		canvas.width = this.env.totalWidthPx;
		canvas.height = this.env.totalHeightPx;
		this.ctx = canvas.getContext("2d");
		this.ctx.clearRect(0, 0, canvas.width, canvas.height);
		if (!this.isLR()) {
			this.ctx.translate(canvas.width, 0);
			this.ctx.scale(-1, 1);
		}
		var p = new ResPoint(this.env.leftMarginPx, this.env.topMarginPx);
		this.shading = new ResShading(this.resContext, !this.isLR());
		this.renderGroups(this.original.groups, this.initialNumber, p);
		if (this.env.marginsUnchanged()) {
			this.shading.compress();
			this.shading.print(this.ctx);
			break;
		} else
			this.env.updateMargins();
	}		
};
ResLiteDivision.prototype.renderGroups =
function(groups, n, p) {
	if (n <= 0 || groups === null) 
		return;
	var lenPx = this.resContext.milEmToPx(groups.lengthMilEm);
	var rect = this.makeRect(p, lenPx);
	this.renderExprs(groups.exprs, rect);
	this.renderNotes(groups.notes, rect);
	this.renderShades(groups.shades, rect);
	if (n > 1 && groups !== null) {
		this.renderShades(groups.intershades, rect);
		var advancePx = this.resContext.milEmToPx(groups.tl.advanceMilEm + this.padMilEm);
		var nextP = this.advancePoint(rect, advancePx);
		this.renderGroups(groups.tl, n-1, nextP);
	}
};
ResLiteDivision.prototype.renderExprs =
function(exprs, rect) {
	if (exprs === null)
		return;
	if (exprs instanceof ResLiteGlyph)
		this.renderGlyph(exprs, rect);
	else 
		this.renderPair(exprs, rect);
	this.renderExprs(exprs.tl, rect);
};
ResLiteDivision.prototype.renderPair =
function(pair, rect) {
	var extra = this.resContext.milEmToPx(2000);
	if (this.isH()) {
		var pre = Math.min(extra, rect.x);
		var post = Math.min(extra, this.env.totalWidthPx - (rect.x + rect.width));
		var w = Math.round(rect.width + pre + post);
		var h = Math.round(rect.height + this.env.topMarginPx + this.env.bottomMarginPx);
		var x = pre;
		var y = this.env.topMarginPx;
	} else {
		var pre = Math.min(extra, rect.y);
		var post = Math.min(extra, this.env.totalHeightPx - (rect.y + rect.height));
		var w = Math.round(rect.width + this.env.leftMarginPx + this.env.rightMarginPx);
		var h = Math.round(rect.height + pre + post);
		var x = this.env.leftMarginPx;
		var y = pre;
	}
	var subRect = new ResRectangle(x, y, rect.width, rect.height);
	var xRef = rect.x - x;
	var yRef = rect.y - y;
	var canvas1 = ResCanvas.make(w, h);
	var ctx1 = canvas1.getContext("2d");
	var canvas2 = ResCanvas.make(w, h);
	var ctx2 = canvas2.getContext("2d");
	var savedCtx = this.ctx;
	var savedTotalWidth = this.env.totalWidthPx;
	var savedTotalHeight = this.env.totalHeightPx;
	this.env.totalWidthPx = w;
	this.env.totalHeightPx = h;
	this.ctx = ctx1;
	this.renderExprs(pair.list1, subRect);
	this.ctx = ctx2;
	this.renderExprs(pair.list2, subRect);
	var external = ResCanvas.externalPixels(ctx2, w, h);
	ResCanvas.erasePixels(ctx1, w, h, external);
	this.ctx = savedCtx;
	this.env.totalWidthPx = savedTotalWidth;
	this.env.totalHeightPx = savedTotalHeight;
	this.ctx.drawImage(canvas1, xRef, yRef);
	this.ctx.drawImage(canvas2, xRef, yRef);
};
ResLiteDivision.prototype.renderGlyph =
function(glyph, rect) {
	var size = Math.round(this.resContext.milEmToPx(glyph.yscaleMil));
	var font = this.resContext.fonts[glyph.fileNumber-1];
	var sizedFont = "" + size + "px " + font;
	var str = String.fromCharCode(glyph.glyphIndex);
	this.ctx.save();
	this.ctx.font = sizedFont;
	this.ctx.fillStyle = glyph.color;
	this.ctx.textBaseline = "alphabetic";
	var x = rect.x + this.resContext.milEmToPx(glyph.xMilEm);
	var y = rect.y + this.resContext.milEmToPx(glyph.yMilEm);
	var gRect = ResCanvas.glyphRect(str, size, 1, font, 0, false);
	if (glyph.rotate === 0 && !glyph.mirror && glyph.xscaleMil === glyph.yscaleMil) {
		var printedRect = new ResRectangle(x - gRect.width/2, y - gRect.height/2, 
					gRect.width, gRect.height);
		var cutOut = this.cutOutRect(glyph, printedRect);
		if (cutOut === undefined) 
			this.env.ensureRect(printedRect);
		else {
			this.ctx.beginPath();
			this.ctx.rect(cutOut.x, cutOut.y, cutOut.width, cutOut.height);
			this.ctx.clip();
			this.env.ensureRect(cutOut);
		}
		this.ctx.fillText(str, x - gRect.x - gRect.width/2, y - gRect.y + gRect.height/2);
	} else {
		var rotRect = ResCanvas.glyphRect(str, size, 1, font, glyph.rotate, glyph.mirror);
		var l = rotRect.x;
		var r = gRect.width - l - rotRect.width;
		var b = -rotRect.y;
		var t = gRect.height - b - rotRect.height;
		var hDiff = (l-r) / 2;
		var vDiff = (t-b) / 2;
		var xDiff = -gRect.x - gRect.width/2;
		var yDiff = -gRect.y + gRect.height/2;
		var printedRect = new ResRectangle(x-hDiff - rotRect.width/2, y-vDiff - rotRect.height/2, 
						rotRect.width, rotRect.height);
		var cutOut = this.cutOutRect(glyph, printedRect);
		if (cutOut === undefined)
			this.env.ensureRect(printedRect);
		else {
			this.ctx.beginPath();
			this.ctx.rect(cutOut.x, cutOut.y, cutOut.width, cutOut.height);
			this.ctx.clip();
			this.env.ensureRect(cutOut);
		}
		this.ctx.translate(x-hDiff, y-vDiff);
		this.ctx.rotate(glyph.rotate*Math.PI/180);
		this.ctx.scale((glyph.mirror ? -1 : 1) * glyph.xscaleMil / glyph.yscaleMil, 1);
		this.ctx.fillText(str, xDiff, yDiff);
	}
	this.ctx.restore();
};
ResLiteDivision.prototype.renderNotes =
function(notes, rect) {
	if (notes !== null) {
		this.renderNote(notes, rect);
		this.renderNotes(notes.tl);
	}
};
ResLiteDivision.prototype.renderNote =
function(notes, rect) {
	var size = Math.round(this.resContext.milEmToPx(notes.sizeMilEm));
	var font = "" + size + "px " + this.resContext.fonts[notes.fileNumber-1];
	this.ctx.save();
	this.ctx.font = font;
	this.ctx.fillStyle = notes.color;
	this.ctx.textBaseline = "alphabetic";
	var gRect = ResCanvas.glyphRect(notes.string, size, 1, font, 0, false);
	var x = rect.x + this.resContext.milEmToPx(notes.xMilEm);
	var y = rect.y + this.resContext.milEmToPx(notes.yMilEm);
	var xDiff = -gRect.x - gRect.width/2;
	var yDiff = -gRect.y + gRect.height/2;
	this.ctx.translate(x, y);
	if (this.isLR())
		this.ctx.scale(1, 1);
	else
		this.ctx.scale(-1, 1);
	this.ctx.fillText(notes.string, xDiff, yDiff);
	this.ctx.restore();
	this.env.ensureRect(new ResRectangle(x - gRect.width/2, y - gRect.height/2, 
				gRect.width, gRect.height));
};
ResLiteDivision.prototype.renderShades =
function(shades, rect) {
	if (shades !== null) {
		this.renderShade(shades, rect);
		this.renderShades(shades.tl, rect);
	}
};
ResLiteDivision.prototype.renderShade =
function(shades, rect) {
	var x = rect.x + this.resContext.milEmToPx(shades.xMilEm);
	var y = rect.y + this.resContext.milEmToPx(shades.yMilEm);
	var w = this.resContext.milEmToPx(shades.widthMilEm);
	var h = this.resContext.milEmToPx(shades.heightMilEm);
	var shadeRect = new ResRectangle(x-w/2, y-h/2, w, h);
	this.shading.addRect(shadeRect);
};
ResLiteDivision.prototype.makeRect =
function(p, len) {
	if (this.original.dir === "hlr" || this.original.dir === "hrl")
		return new ResRectangle(p.x, p.y, len, this.getHeightPx());
	else
		return new ResRectangle(p.x, p.y, this.getWidthPx(), len);
};
ResLiteDivision.prototype.advancePoint =
function(rect, adv) {
	if (this.original.dir === "hlr" || this.original.dir === "hrl")
		return new ResPoint(rect.x + adv, rect.y);
	else
		return new ResPoint(rect.x, rect.y + adv);
};
ResLiteDivision.prototype.cutOutRect =
function(glyph, rect) {
	if (glyph.xMinMil === 0 && glyph.yMinMil === 0 && 
			glyph.widthMil === 1000 && glyph.heightMil === 1000)
		return undefined;
	var subX = rect.x + glyph.xMinMil * rect.width / 1000;
	var subY = rect.y + glyph.yMinMil * rect.height / 1000;
	var subWidth = glyph.widthMil * rect.width / 1000;
	var subHeight = glyph.heightMil * rect.height / 1000;
	return new ResRectangle(subX, subY, subWidth, subHeight);
};


 */