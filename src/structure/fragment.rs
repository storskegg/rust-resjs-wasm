use crate::structure::{Direction, Globals};

pub struct FragmentArgs {
    globals: Globals,
    direction: Direction,
    size: f64,
    switches: String,
    hiero: String
}


pub struct Fragment {}

impl Fragment {
    pub fn new() -> Self {
        Self {}
    }
}

/*
function ResFragment(args) {
	if (args.l) {
		var argList = args.l;
		var switches = args.sw;
		var hiero = args.h;
		this.direction = null;
		this.size = null;
		for (var i = 0; i < argList.length; i++) {
			var arg = argList[i];
			if (arg.is("hlr") || arg.is("hrl") ||
					arg.is("vlr") || arg.is("vrl"))
				this.direction = arg.getLhs();
			else if (arg.hasLhs("size") && arg.hasRhsNonzeroReal())
				this.size = arg.getRhs();
		}
		this.switches = switches;
		this.hiero = hiero;
		this.propagateBack();
		this.propagate();
	} else {
		this.direction = args.direction;
		this.size = args.size;
		this.switches = args.switches;
		this.hiero = args.hiero;
		this.propagateBack();
		this.propagate();
	}
}
ResFragment.prototype.headerString =
function() {
	var args = [];
	if (this.direction !== null)
		args.push(this.direction);
	if (this.size !== null)
		args.push("size=" + ResArg.realStr(this.size));
	var s = ResArg.argsStr(args);
	return s;
};
ResFragment.prototype.toString =
function() {
	var s = this.headerString();
	s += this.switches.toString();
	if (this.hiero !== null)
		s += this.hiero.toString();
	return s;
};
ResFragment.prototype.propagateBack =
function() {
	if (this.hiero !== null) {
		var sw = this.hiero.propagateBack();
		this.switches = this.switches.join(sw);
	}
};
ResFragment.prototype.propagate =
function() {
	this.globals = new ResGlobals(this.direction, this.size);
	this.globals = this.switches.update(this.globals);
	if (this.hiero !== null)
		this.globals = this.hiero.propagate(this.globals, this.globals.direction);
};
ResFragment.prototype.namedGlyphs =
function() {
	return this.hiero === null ? [] : this.hiero.namedGlyphs();
};
 */