pub struct ResBox {}

impl ResBox {
    pub fn new() -> Self {
        Self {}
    }
}

/*
function ResBox(args) {
	if (args === null)
		this.setDefaults();
	else if (args.l) {
		var type = args.na;
		var argList = args.l;
		var switchs1 = args.sw1;
		var hiero = args.h;
		var notes = args.no;
		var switchs2 = args.sw2;
		this.type = type;
		this.direction = null;
		this.mirror = null;
		this.scale = 1;
		this.color = null;
		this.shade = null;
		this.shades = [];
		this.size = 1;
		this.opensep = null;
		this.closesep = null;
		this.undersep = null;
		this.oversep = null;
		for (var i = 0; i < argList.length; i++) {
			var arg = argList[i];
			if (arg.is("h") || arg.is("v"))
				this.direction = arg.getLhs();
			else if (arg.is("mirror"))
				this.mirror = true;
			else if (arg.is("nomirror"))
				this.mirror = false;
			else if (arg.hasLhs("scale") && arg.hasRhsNonzeroReal())
				this.scale = arg.getRhs();
			else if (arg.isColor())
				this.color = arg.getLhs();
			else if (arg.is("shade"))
				this.shade = true;
			else if (arg.is("noshade"))
				this.shade = false;
			else if (arg.isPattern())
				this.shades.push(arg.getLhs());
			else if (arg.hasLhs("size") && arg.hasRhsNonzeroReal())
				this.size = arg.getRhs();
			else if (arg.hasLhs("opensep") && arg.hasRhsReal())
				this.opensep = arg.getRhs();
			else if (arg.hasLhs("closesep") && arg.hasRhsReal())
				this.closesep = arg.getRhs();
			else if (arg.hasLhs("undersep") && arg.hasRhsReal())
				this.undersep = arg.getRhs();
			else if (arg.hasLhs("oversep") && arg.hasRhsReal())
				this.oversep = arg.getRhs();
		}
		this.switchs1 = switchs1;
		this.hiero = hiero;
		this.notes = notes;
		this.switchs2 = switchs2;
	} else {
		this.type = args.type;
		this.direction = args.direction;
		this.mirror = args.mirror;
		this.scale = args.scale;
		this.color = args.color;
		this.shade = args.shade;
		this.shades = args.shades;
		this.size = args.size;
		this.opensep = args.opensep;
		this.closesep = args.closesep;
		this.undersep = args.undersep;
		this.oversep = args.oversep;
		this.switchs1 = args.switchs1;
		this.hiero = args.hiero;
		this.notes = args.notes;
		this.switchs2 = args.switchs2;
	}
}
ResBox.prototype.setDefaults =
function() {
		this.type = 'cartouche';
		this.direction = null;
		this.mirror = null;
		this.scale = 1;
		this.color = null;
		this.shade = null;
		this.shades = [];
		this.size = 1;
		this.opensep = null;
		this.closesep = null;
		this.undersep = null;
		this.oversep = null;
		this.switchs1 = new ResSwitch(null);
		this.hiero = null;
		this.notes = [];
		this.switchs2 = new ResSwitch(null);
};
ResBox.prototype.toString =
function() {
	var args = [];
	if (this.direction === "h" || this.direction === "v")
		args.push(this.direction);
	if (this.mirror === true)
		args.push("mirror");
	else if (this.mirror === false)
		args.push("nomirror");
	if (this.scale !== 1)
		args.push("scale=" + ResArg.realStr(this.scale));
	if (this.color !== null)
		args.push(this.color);
	if (this.shade === true)
		args.push("shade");
	else if (this.shade === false)
		args.push("noshade");
	for (var i = 0; i < this.shades.length; i++)
		args.push(this.shades[i]);
	if (this.size !== 1)
		args.push("size=" + ResArg.realStr(this.size));
	if (this.opensep !== null)
		args.push("opensep=" + ResArg.realStr(this.opensep));
	if (this.closesep !== null)
		args.push("closesep=" + ResArg.realStr(this.closesep));
	if (this.undersep !== null)
		args.push("undersep=" + ResArg.realStr(this.undersep));
	if (this.oversep !== null)
		args.push("oversep=" + ResArg.realStr(this.oversep));
	var s = this.type + ResArg.argsStr(args) +
		"(" + this.switchs1.toString() +
		(this.hiero === null ? "" : this.hiero.toString()) +
		")";
	for (var i = 0; i < this.notes.length; i++)
			s += this.notes[i].toString();
	s += this.switchs2.toString();
	return s;
};
ResBox.prototype.propagateBack =
function(sw) {
	this.switchs2 = this.switchs2.join(sw);
	if (this.hiero !== null) {
		var swHiero = this.hiero.propagateBack();
		this.switchs1 = this.switchs1.join(swHiero);
	}
	return new ResSwitch(null);
};
ResBox.prototype.propagate =
function(globals) {
	this.globals = globals;
	globals = this.switchs1.update(globals);
	if (this.hiero !== null) {
		var savedSize = globals.size;
		globals = globals.update(this.size);
		globals = this.hiero.propagate(globals, this.effectiveDir());
		globals = globals.update(savedSize);
	}
	for (var i = 0; i < this.notes.length; i++)
		this.notes[i].propagate(globals);
	return this.switchs2.update(globals);
};
ResBox.prototype.effectiveDir =
function() {
	if (this.direction === "h") {
		if (this.globals.direction === "vlr")
			return "hlr";
		else if (this.globals.direction === "vrl")
			return "hrl";
	} else if (this.direction === "v") {
		if (this.globals.direction === "hlr")
			return "vlr";
		else if (this.globals.direction === "hrl")
			return "vrl";
	}
	return this.globals.direction;
};
ResBox.prototype.effectiveIsH =
function() {
	return ResGlobals.isH(this.effectiveDir());
};
ResBox.prototype.effectiveMirror =
function() {
	return this.mirror !== null ? this.mirror : this.globals.mirror;
};
ResBox.prototype.effectiveColor =
function() {
	return this.color !== null ? this.color : this.globals.color;
};
ResBox.prototype.effectiveOpensep =
function() {
	return this.opensep !== null ? this.opensep : this.globals.sep;
};
ResBox.prototype.effectiveClosesep =
function() {
	return this.closesep !== null ? this.closesep : this.globals.sep;
};
ResBox.prototype.effectiveUndersep =
function() {
	return this.undersep !== null ? this.undersep : this.globals.sep;
};
ResBox.prototype.effectiveOversep =
function() {
	return this.oversep !== null ? this.oversep : this.globals.sep;
};
ResBox.prototype.namedGlyphs =
function() {
	return this.hiero === null ? [] : this.hiero.namedGlyphs();
};
 */