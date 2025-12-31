pub struct EmptyGlyph {}

impl EmptyGlyph {
    pub fn new() -> Self {
        Self {}
    }
}

/*
function ResEmptyglyph(args) {
	if (args === null)
		this.setDefaults();
	else if (args.l) {
		var argList = args.l;
		var note = args.n;
		var switchs = args.sw;
		this.width = 1;
		this.height = 1;
		this.shade = null;
		this.shades = [];
		this.firm = false;
		for (var i = 0; i < argList.length; i++) {
			var arg = argList[i];
			if (arg.hasLhs("width") && arg.hasRhsReal())
					this.width = arg.getRhs();
			else if (arg.hasLhs("height") && arg.hasRhsReal())
				this.height = arg.getRhs();
			else if (arg.is("shade"))
				this.shade = true;
			else if (arg.is("noshade"))
				this.shade = false;
			else if (arg.isPattern())
				this.shades.push(arg.getLhs());
			else if (arg.is("firm"))
				this.firm = true;
		}
		this.note = note;
		this.switchs = switchs;
	} else if (args.n) {
		var note = args.n;
		var switchs = args.sw;
		this.width = 0;
		this.height = 0;
		this.shade = null;
		this.shades = [];
		this.firm = false;
		this.note = note;
		this.switchs = switchs;
	} else {
		this.width = args.width;
		this.height = args.height;
		this.shade = args.shade;
		this.shades = args.shades;
		this.firm = args.film;
		this.note = args.note;
		this.switchs = args.switchs;
	}
}
ResEmptyglyph.prototype.setDefaults =
function() {
	this.width = 1;
	this.height = 1;
	this.shade = null;
	this.shades = [];
	this.firm = false;
	this.note = null;
	this.switchs = new ResSwitch(null);
};
ResEmptyglyph.pointArgs =
function() {
	return [new ResArg("width",0), new ResArg("height",0)];
};
ResEmptyglyph.prototype.toString =
function() {
	var args = [];
	var noPointArgs = false;
	if (this.width !== 1)
		args.push("width=" + ResArg.realStr(this.width));
	if (this.height !== 1)
		args.push("height=" + ResArg.realStr(this.height));
	if (this.shade === true) {
		args.push("shade");
		noPointArgs = true;
	}
	if (this.shade === false) {
		args.push("noshade");
		noPointArgs = true;
	}
	for (var i = 0; i < this.shades.length; i++) {
		args.push(this.shades[i]);
		noPointArgs = true;
	}
	if (this.firm) {
		args.push("firm");
		noPointArgs = true;
	}
	var s;
	if (this.width === 0 && this.height === 0 && !noPointArgs)
		s = ".";
	else
		s = "empty" + ResArg.argsStr(args);
	if (this.note !== null)
		s += this.note.toString();
	s += this.switchs.toString();
	return s;
};
ResEmptyglyph.prototype.propagateBack =
function(sw) {
	this.switchs = this.switchs.join(sw);
	return new ResSwitch(null);
};
ResEmptyglyph.prototype.propagate =
function(globals) {
	this.globals = globals;
	if (this.note !== null)
		this.note.propagate(globals);
	return this.switchs.update(globals);
};
ResEmptyglyph.prototype.namedGlyphs =
function() {
	return [];
};
 */