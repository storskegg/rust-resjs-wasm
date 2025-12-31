pub struct Op {}

impl Op {
    pub fn new() -> Self {
        Self {}
    }
}

/*
function ResOp(args, isFirst) {
	if (args === null)
		this.setDefaults();
	else if (args.l) {
		var argList = args.l;
		this.sep = null;
		this.fit = null;
		this.fix = false;
		this.shade = null;
		this.shades = [];
		this.size = null;
		for (var i = 0; i < argList.length; i++) {
			var arg = argList[i];
			if (arg.hasLhs("sep") && arg.hasRhsReal())
				this.sep = arg.getRhs();
			else if (arg.is("fit"))
				this.fit = true;
			else if (arg.is("nofit"))
				this.fit = false;
			else if (arg.is("fix"))
				this.fix = true;
			else if (arg.is("shade"))
				this.shade = true;
			else if (arg.is("noshade"))
				this.shade = false;
			else if (arg.isPattern())
				this.shades.push(arg.getLhs());
			else if (isFirst) {
				if (arg.hasLhs("size") &&
						(arg.hasRhsReal() || arg.hasRhs("inf")))
					this.size = arg.getRhs();
			}
		}
	} else {
		this.sep = args.sep;
		this.fit = args.fit;
		this.fix = args.fix;
		this.shade = args.shade;
		this.shades = args.shades;
		this.size = args.size;
	}
}
ResOp.prototype.setDefaults =
function() {
	this.sep = null;
	this.fit = null;
	this.fix = false;
	this.shade = null;
	this.shades = [];
	this.size = null;
};
ResOp.prototype.propagate =
function(globals) {
	this.globals = globals;
};
ResOp.prototype.toString =
function(isFirst) {
	var args = [];
	if (this.sep !== null)
		args.push("sep=" + ResArg.realStr(this.sep));
	if (this.fit === true)
		args.push("fit");
	else if (this.fit === false)
		args.push("nofit");
	if (this.fix)
		args.push("fix");
	if (this.shade === true)
		args.push("shade");
	if (this.shade === false)
		args.push("noshade");
	for (var i = 0; i < this.shades.length; i++)
		args.push(this.shades[i]);
	if (this.size === "inf") {
		if (isFirst)
			args.push("size=inf");
	} else if (this.size !== null)
		args.push("size=" + ResArg.realStr(this.size));
	return ResArg.argsStr(args);
};
ResOp.prototype.effectiveSep =
function() {
	return this.sep !== null ? this.sep : this.globals.sep;
};
ResOp.prototype.effectiveFit =
function() {
	return this.fit !== null ? this.fit : this.globals.fit;
};
 */