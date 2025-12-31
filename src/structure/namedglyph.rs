pub struct NamedGlyph {}

impl NamedGlyph {
    pub fn new() -> Self {
        Self {}
    }
}

/*
function ResNamedglyph(args) {
	if (args === null) 
		this.setDefaults();
	else if (args.na) { 
		var name = args.na;
		var argList = args.l;
		var notes = args.no;
		var switchs = args.sw;
		this.name = name;
		this.mirror = null;
		this.rotate = 0;
		this.scale = 1;
		this.xscale = 1;
		this.yscale = 1;
		this.color = null;
		this.shade = null;
		this.shades = [];
		for (var i = 0; i < argList.length; i++) {
			var arg = argList[i];
			if (arg.is("mirror"))
				this.mirror = true;
			else if (arg.is("nomirror"))
				this.mirror = false;
			else if (arg.hasLhs("rotate") && arg.hasRhsNatnum())
				this.rotate = arg.getRhs() % 360;
			else if (arg.hasLhs("scale") && arg.hasRhsNonzeroReal())
				this.scale = arg.getRhs();
			else if (arg.hasLhs("xscale") && arg.hasRhsNonzeroReal())
				this.xscale = arg.getRhs();
			else if (arg.hasLhs("yscale") && arg.hasRhsNonzeroReal())
				this.yscale = arg.getRhs();
			else if (arg.isColor())
				this.color = arg.getLhs();
			else if (arg.is("shade"))
				this.shade = true;
			else if (arg.is("noshade"))
				this.shade = false;
			else if (arg.isPattern())
				this.shades.push(arg.getLhs());
		}
		this.notes = notes;
		this.switchs = switchs;
	} else {
		this.name = args.name;
		this.mirror = args.mirror;
		this.rotate = args.rotate;
		this.scale = args.scale;
		this.xscale = args.xscale;
		this.yscale = args.yscale;
		this.color = args.color;
		this.shade = args.shade;
		this.shades = args.shades;
		this.notes = args.notes;
		this.switchs = args.switchs;
	}
}
ResNamedglyph.prototype.setDefaults =
function() {
		this.name = '\"?\"';
		this.mirror = null;
		this.rotate = 0;
		this.scale = 1;
		this.xscale = 1;
		this.yscale = 1;
		this.color = null;
		this.shade = null;
		this.shades = [];
		this.notes = [];
		this.switchs = new ResSwitch(null);
};
ResNamedglyph.prototype.toString =
function() {
	var args = [];
	if (this.mirror === true)
		args.push("mirror");
	else if (this.mirror === false)
		args.push("nomirror");
	if (this.rotate !== 0)
		args.push("rotate=" + this.rotate);
	if (this.scale !== 1)
		args.push("scale=" + ResArg.realStr(this.scale));
	if (this.xscale !== 1)
		args.push("xscale=" + ResArg.realStr(this.xscale));
	if (this.yscale !== 1)
		args.push("yscale=" + ResArg.realStr(this.yscale));
	if (this.color !== null)
		args.push(this.color);
	if (this.shade === true)
		args.push("shade");
	else if (this.shade === false)
		args.push("noshade");
	for (var i = 0; i < this.shades.length; i++)
		args.push(this.shades[i]);
	var s = this.name + ResArg.argsStr(args);
	for (var i = 0; i < this.notes.length; i++)
			s += this.notes[i].toString();
	s += this.switchs.toString();
	return s;
};
ResNamedglyph.prototype.propagateBack =
function(sw) {
	this.switchs = this.switchs.join(sw);
	return new ResSwitch(null);
};
ResNamedglyph.prototype.propagate =
function(globals) {
	this.globals = globals;
	for (var i = 0; i < this.notes.length; i++)
		this.notes[i].propagate(globals);
	return this.switchs.update(globals);
};
ResNamedglyph.prototype.effectiveMirror =
function() {
	return this.mirror !== null ? this.mirror : this.globals.mirror;
};
ResNamedglyph.prototype.effectiveColor =
function() {
	return this.color !== null ? this.color : this.globals.color;
};
ResNamedglyph.prototype.namedGlyphs =
function() {
	return [this];
};
 */