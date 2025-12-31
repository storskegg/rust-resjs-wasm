pub struct Lite {}

/// NOTE: top-level constructor!
impl Lite {
    pub fn new() -> Self {
        Self {}
    }
}

/*

// Top constructor.
function ResLite(dir, size, groups) {
	this.dir = dir; // can be "hlr", "hrl", "vlr", "vrl"
	this.sizeMilEm = size;
	this.groups = groups;
}

ResLite.prototype.isH =
function() {
	return this.dir === "hlr" || this.dir === "hrl";
};

ResLite.prototype.isLR =
function() {
	return this.dir === "hlr" || this.dir === "vlr";
};

/////////////////////////////////////////////////////////////////////////////
// Parsing.

// Parse string into structure.
// return: ResLite
ResLite.parse =
function(str) {
	var resLite = new ResLite("hlr", 1000, null);
	var buffer = new ParseBuffer(str);
	buffer.readToNonspace();
	if (buffer.isEmpty())
		return;
	if (!ResLite.parseChunk(resLite, buffer))
		buffer.readToEnd();
	var remainder = buffer.remainder();
	if (remainder.replace(/^\s+|\s+$/gm,'').length !== 0)
		console.log("RESlite trailing symbols:" + remainder);
	return resLite;
};

// Parse header, then groups, then 'e'.
// return: whether successful
ResLite.parseChunk =
function(resLite, buffer) {
	var oldPos = buffer.pos;
	var newDir;
	var newSize;
	if (!buffer.readChar("$") ||
			(newDir = buffer.readDirection()) === undefined ||
			(newSize = buffer.readNum()) === undefined) {
		buffer.pos = oldPos;
		console.log("RESlite ill-formed header");
		return false;
	}
	resLite.dir = newDir;
	resLite.sizeMilEm = newSize;
	resLite.groups = ResLite.parseGroups(buffer);
	if (!buffer.readSingleChar("e")) {
		console.log("RESlite missing end");
		return false;
	} else
		return true;
};

// Parse zero or more groups.
// return: pointer to list, possibly null
ResLite.parseGroups =
function(buffer) {
	var oldPos = buffer.pos;
	if (!buffer.readChar("g"))
		return null;
	var groups = new ResLiteGroups();
	var advance;
	var length;
	if ((advance = buffer.readNum()) === undefined ||
			(length = buffer.readNum()) === undefined) {
		buffer.pos = oldPos;
		console.log("RESlite ill-formed group header");
		return groups;
	}
	groups.advanceMilEm = advance;
	groups.lengthMilEm = length;
	groups.exprs = ResLite.parseExprs(buffer);
	groups.notes = ResLite.parseNotes(buffer);
	groups.shades = ResLite.parseShades(buffer);
	if (!buffer.readChar("i")) {
		buffer.pos = oldPos;
		console.log("RESlite missing i in group");
		return groups;
	}
	groups.intershades = ResLite.parseShades(buffer);
	groups.tl = ResLite.parseGroups(buffer);
	return groups;
};

// Parse zero or more expressions.
// return: pointer to list, possibly null.
ResLite.parseExprs =
function(buffer) {
	if (buffer.peekChar("c")) {
		var exprs = ResLite.parseGlyph(buffer);
		exprs.tl = ResLite.parseExprs(buffer);
		return exprs;
	} else if (buffer.peekChar("(")) {
		var exprs = ResLite.parsePair(buffer);
		exprs.tl = ResLite.parseExprs(buffer);
		return exprs;
	} else
		return null;
};

// Parse glyph.
// return: ResLiteGlyph
ResLite.parseGlyph =
function(buffer) {
	var oldPos = buffer.pos;
	var glyph = new ResLiteGlyph();
	if (!buffer.readChar("c")) {
		console.log("RESlite missing c in glyph");
		return glyph;
	}
	var fileNumber;
	var glyphIndex;
	var mirror;
	var rotate;
	var colorCode;
	var xscale;
	var yscale;
	var x;
	var y;
	var xMin;
	var yMin;
	var width;
	var height;
	if ((fileNumber = buffer.readNum()) === undefined ||
			(glyphIndex = buffer.readNum()) === undefined ||
			(mirror = buffer.readNum()) === undefined ||
			(rotate = buffer.readNum()) === undefined ||
			(colorCode = buffer.readNum()) === undefined ||
			colorCode < 0 || colorCode >= 16 ||
			(xscale = buffer.readNum()) === undefined ||
			(yscale = buffer.readNum()) === undefined ||
			(x = buffer.readNum()) === undefined ||
			(y = buffer.readNum()) === undefined ||
			(xMin = buffer.readNum()) === undefined ||
			(yMin = buffer.readNum()) === undefined ||
			(width = buffer.readNum()) === undefined ||
			(height = buffer.readNum()) === undefined) {
		buffer.pos = oldPos;
		console.log("RESlite ill-formed glyph");
		return glyph;
	}
	glyph.fileNumber = fileNumber;
	glyph.glyphIndex = glyphIndex;
	glyph.mirror = (mirror !== 0);
	glyph.rotate = rotate;
	glyph.color = ResLite.numToColor(colorCode);
	glyph.xscaleMil = xscale;
	glyph.yscaleMil = yscale;
	glyph.xMilEm = x;
	glyph.yMilEm = y;
	glyph.xMinMil = xMin;
	glyph.yMinMil = yMin;
	glyph.widthMil = width;
	glyph.heightMil = height;
	return glyph;
};

// Parse pair.
// return: ResLitePair
ResLite.parsePair =
function(buffer) {
	var oldPos = buffer.pos;
	var pair = new ResLitePair();
	if (!buffer.readChar("(")) {
		console.log("RESlite missing ( in pair");
		return pair;
	}
	pair.list1 = ResLite.parseExprs(buffer);
	if (!buffer.readChar("o")) {
		buffer.pos = oldPos;
		console.log("RESlite missing o in pair");
		return pair;
	}
	pair.list2 = ResLite.parseExprs(buffer);
	if (!buffer.readChar(")")) {
		buffer.pos = oldPos;
		console.log("RESlite missing ) in pair");
		return pair;
	}
	return pair;
};

// Parse zero or more notes.
// return: pointer to list, possibly null
ResLite.parseNotes =
function(buffer) {
	var oldPos = buffer.pos;
	if (!buffer.readChar("n"))
		return null;
	var notes = new ResLiteNotes();
	var string;
	var fileNumber;
	var colorCode;
	var size;
	var x;
	var y;
	if ((string = buffer.readString()) === undefined ||
			(fileNumber = buffer.readNum()) === undefined ||
			(colorCode = buffer.readNum()) === undefined ||
			colorCode < 0 || colorCode >= 16 ||
			(size = buffer.readNum()) === undefined ||
			(x = buffer.readNum()) === undefined ||
			(y = buffer.readNum()) === undefined) {
		buffer.pos = oldPos;
		console.log("RESlite ill-formed note");
		return notes;
	}
	notes.string = string;
	notes.fileNumber = fileNumber;
	notes.color = ResLite.numToColor(colorCode);
	notes.sizeMilEm = size;
	notes.xMilEm = x;
	notes.yMilEm = y;
	notes.tl = ResLite.parseNotes(buffer);
	return notes;
};

// Parse zero or more shades.
// return: pointer to list, possibly null
ResLite.parseShades =
function(buffer) {
	var oldPos = buffer.pos;
	if (!buffer.readChar("s"))
		return null;
	var shades = new ResLiteShades();
	var x;
	var y;
	var width;
	var height;
	if ((x = buffer.readNum()) === undefined ||
			(y = buffer.readNum()) === undefined ||
			(width = buffer.readNum()) === undefined ||
			(height = buffer.readNum()) === undefined) {
		buffer.pos = oldPos;
		console.log("RESlite ill-formed shade");
		return shades;
	}
	shades.xMilEm = x;
	shades.yMilEm = y;
	shades.widthMilEm = width;
	shades.heightMilEm = height;
	shades.tl = ResLite.parseShades(buffer);
	return shades;
};

// Convert color number to name.
ResLite.numToColor =
function(num) {
	switch (num) {
		case 0: return "white";
		case 1: return "silver";
		case 2: return "gray";
		case 3: return "yellow";
		case 4: return "fuchsia";
		case 5: return "aqua";
		case 6: return "olive";
		case 7: return "purple";
		case 8: return "teal";
		case 9: return "red";
		case 10: return "lime";
		case 11: return "blue";
		case 12: return "maroon";
		case 13: return "green";
		case 14: return "navy";
		case 15: return "black";
		default: return "black";
	}
};

// Straightforward rendering.
ResLite.prototype.render =
function(canvas, size) {
	var context = new ResContext();
	context.emSizePx = size;
	var div = ResLiteDivision.make(this, context);
	div.render(canvas);
};

 */