pub struct Modify {}

impl Modify {
    pub fn new() -> Self {
        Self {}
    }
}

/*
function ResModify(args) {
	if (args === null)
		this.setDefaults();
	else if (args.l) {
		var argList = args.l;
		var switchs1 = args.sw1;
		var group = args.g;
		var switchs2 = args.sw2;
		this.width = null;
		this.height = null;
		this.above = 0;
		this.below = 0;
		this.before = 0;
		this.after = 0;
		this.omit = false;
		this.shade = null;
		this.shades = [];
		for (var i = 0; i < argList.length; i++) {
			var arg = argList[i];
			if (arg.hasLhs("width") && arg.hasRhsNonzeroReal())
				this.width = arg.getRhs();
			else if (arg.hasLhs("height") && arg.hasRhsNonzeroReal())
				this.height = arg.getRhs();
			else if (arg.hasLhs("above") && arg.hasRhsReal())
				this.above = arg.getRhs();
			else if (arg.hasLhs("below") && arg.hasRhsReal())
				this.below = arg.getRhs();
			else if (arg.hasLhs("before") && arg.hasRhsReal())
				this.before = arg.getRhs();
			else if (arg.hasLhs("after") && arg.hasRhsReal())
				this.after = arg.getRhs();
			else if (arg.is("omit"))
				this.omit = true;
			else if (arg.is("shade"))
				this.shade = true;
			else if (arg.is("noshade"))
				this.shade = false;
			else if (arg.isPattern())
				this.shades.push(arg.getLhs());
		}
		this.switchs1 = switchs1;
		this.group = group;
		this.switchs2 = switchs2;
	} else {
		this.width = args.width;
		this.height = args.height;
		this.above = args.above;
		this.below = args.below;
		this.before = args.before;
		this.after = args.after;
		this.omit = args.omit;
		this.shade = args.shade;
		this.shades = args.shades;
		this.switchs1 = args.switchs1;
		this.group = args.group;
		this.switchs2 = args.switchs2;
	}
}
ResModify.prototype.setDefaults =
function() {
	this.width = null;
	this.height = null;
	this.above = 0;
	this.below = 0;
	this.before = 0;
	this.after = 0;
	this.omit = false;
	this.shade = null;
	this.shades = [];
	this.switchs1 = new ResSwitch(null);
	this.group = null;
	this.switchs2 = new ResSwitch(null);
};
ResModify.prototype.toString =
function() {
	var args = [];
	if (this.width !== null)
		args.push("width=" + ResArg.realStr(this.width));
	if (this.height !== null)
		args.push("height=" + ResArg.realStr(this.height));
	if (this.above !== 0)
		args.push("above=" + ResArg.realStr(this.above));
	if (this.below !== 0)
		args.push("below=" + ResArg.realStr(this.below));
	if (this.before !== 0)
		args.push("before=" + ResArg.realStr(this.before));
	if (this.after !== 0)
		args.push("after=" + ResArg.realStr(this.after));
	if (this.omit)
		args.push("omit");
	if (this.shade === true)
		args.push("shade");
	else if (this.shade === false)
		args.push("noshade");
	for (var i = 0; i < this.shades.length; i++)
		args.push(this.shades[i]);
	return "modify" + ResArg.argsStr(args) + "(" + this.switchs1.toString() +
			this.group.toString() + ")" + this.switchs2.toString();
};
ResModify.prototype.propagateBack =
function(sw) {
	this.switchs2 = this.switchs2.join(sw);
	var swGroup = this.group.propagateBack(new ResSwitch(null));
	this.switchs1 = this.switchs1.join(swGroup);
	return new ResSwitch(null);
};
ResModify.prototype.propagate =
function(globals) {
	this.globals = globals;
	globals = this.switchs1.update(globals);
	globals = this.group.propagate(globals);
	return this.switchs2.update(globals);
};
ResModify.prototype.namedGlyphs =
function() {
	return this.group.namedGlyphs();
};
 */