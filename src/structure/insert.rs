pub struct Insert {}

impl Insert {
    pub fn new() -> Self {
        Self {}
    }
}

/*
function ResInsert(args) {
	if (args === null)
		this.setDefaults();
	else if (args.l) {
		var argList = args.l;
		var switchs1 = args.sw1;
		var group1 = args.g1;
		var switchs2 = args.sw2;
		var group2 = args.g2;
		var switchs3 = args.sw3;
		this.place = "";
		this.x = 0.5;
		this.y = 0.5;
		this.fix = false;
		this.sep = null;
		for (var i = 0; i < argList.length; i++) {
			var arg = argList[i];
			if (arg.is("t") || arg.is("b") || arg.is("s") || arg.is("e") ||
					arg.is("ts") || arg.is("te") || arg.is("bs") || arg.is("be"))
				this.place = arg.getLhs();
			else if (arg.hasLhs("x") && arg.hasRhsLowReal())
				this.x = arg.getRhs();
			else if (arg.hasLhs("y") && arg.hasRhsLowReal())
				this.y = arg.getRhs();
			else if (arg.is("fix"))
				this.fix = true;
			else if (arg.hasLhs("sep") && arg.hasRhsReal())
				this.sep = arg.getRhs();
		}
		this.switchs1 = switchs1;
		this.group1 = group1;
		this.switchs2 = switchs2;
		this.group2 = group2;
		this.switchs3 = switchs3;
	} else {
		this.place = args.place;
		this.x = args.x;
		this.y = args.y;
		this.fix = args.fix;
		this.sep = args.sep;
		this.switchs1 = args.switchs1;
		this.group1 = args.group1;
		this.switchs2 = args.switchs2;
		this.group2 = args.group2;
		this.switchs3 = args.switchs3;
	}
}
ResInsert.prototype.setDefaults =
function() {
	this.place = "";
	this.x = 0.5;
	this.y = 0.5;
	this.fix = false;
	this.sep = null;
	this.switchs1 = new ResSwitch(null);
	this.group1 = null;
	this.switchs2 = new ResSwitch(null);
	this.group2 = null;
	this.switchs3 = new ResSwitch(null);
};
ResInsert.prototype.toString =
function() {
	var args = [];
	if (this.place !== "")
		args.push(this.place);
	if (this.x !== 0.5)
		args.push("x=" + ResArg.realStr(this.x));
	if (this.y !== 0.5)
		args.push("y=" + ResArg.realStr(this.y));
	if (this.fix)
		args.push("fix");
	if (this.sep !== null)
		args.push("sep=" + ResArg.realStr(this.sep));
	return "insert" + ResArg.argsStr(args) + "(" + this.switchs1.toString() +
			this.group1.toString() + "," + this.switchs2.toString() +
			this.group2.toString() + ")" + this.switchs3.toString();
};
ResInsert.prototype.propagateBack =
function(sw) {
	this.switchs3 = this.switchs3.join(sw);
	var swAfter = this.group2.propagateBack(new ResSwitch(null));
	var swBefore = this.switchs2.join(swAfter);
	this.switchs2 = new ResSwitch(null);
	var swStart = this.group1.propagateBack(swBefore);
	this.switchs1 = this.switchs1.join(swStart);
	return new ResSwitch(null);
};
ResInsert.prototype.propagate =
function(globals) {
	this.globals = globals;
	globals = this.switchs1.update(globals);
	globals = this.group1.propagate(globals);
	globals = this.switchs2.update(globals);
	globals = this.group2.propagate(globals);
	return this.switchs3.update(globals);
};
ResInsert.prototype.effectiveSep =
function() {
	return this.sep !== null ? this.sep : this.globals.sep;
};
ResInsert.prototype.namedGlyphs =
function() {
	return this.place === "s" || this.place === "t" ?
		this.group2.namedGlyphs().concat(this.group1.namedGlyphs()) :
		this.group1.namedGlyphs().concat(this.group2.namedGlyphs());
};
 */