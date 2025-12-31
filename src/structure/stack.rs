pub struct Stack {}

impl Stack {
    pub fn new() -> Self {
        Self {}
    }
}

/*
function ResStack(args) {
	if (args === null)
		this.setDefaults();
	else if (args.l) {
		var argList = args.l;
		var switchs1 = args.sw1;
		var group1 = args.g1;
		var switchs2 = args.sw2;
		var group2 = args.g2;
		var switchs3 = args.sw3;
		this.x = 0.5;
		this.y = 0.5;
		this.onunder = null;
		for (var i = 0; i < argList.length; i++) {
			var arg = argList[i];
			if (arg.hasLhs("x") && arg.hasRhsLowReal())
				this.x = arg.getRhs();
			else if (arg.hasLhs("y") && arg.hasRhsLowReal())
				this.y = arg.getRhs();
			else if (arg.is("on") || arg.is("under"))
				this.onunder = arg.getLhs();
		}
		this.switchs1 = switchs1;
		this.group1 = group1;
		this.switchs2 = switchs2;
		this.group2 = group2;
		this.switchs3 = switchs3;
	} else {
		this.x = args.x;
		this.y = args.y;
		this.onunder = args.onunder;
		this.switchs1 = args.switchs1;
		this.group1 = args.group1;
		this.switchs2 = args.switchs2;
		this.group2 = args.group2;
		this.switchs3 = args.switchs3;
	}
}
ResStack.prototype.setDefaults =
function() {
	this.x = 0.5;
	this.y = 0.5;
	this.onunder = null;
	this.switchs1 = new ResSwitch(null);
	this.group1 = null;
	this.switchs2 = new ResSwitch(null);
	this.group2 = null;
	this.switchs3 = new ResSwitch(null);
};
ResStack.prototype.toString =
function() {
	var args = [];
	if (this.x !== 0.5)
		args.push("x=" + ResArg.realStr(this.x));
	if (this.y !== 0.5)
		args.push("y=" + ResArg.realStr(this.y));
	if (this.onunder !== null)
		args.push(this.onunder);
	return "stack" + ResArg.argsStr(args) + "(" + this.switchs1.toString() +
			this.group1.toString() + "," + this.switchs2.toString() +
			this.group2.toString() + ")" + this.switchs3.toString();
};
ResStack.prototype.propagateBack =
function(sw) {
	this.switchs3 = this.switchs3.join(sw);
	var swAfter = this.group2.propagateBack(new ResSwitch(null));
	var swBefore = this.switchs2.join(swAfter);
	this.switchs2 = new ResSwitch(null);
	var swStart = this.group1.propagateBack(swBefore);
	this.switchs1 = this.switchs1.join(swStart);
	return new ResSwitch(null);
};
ResStack.prototype.propagate =
function(globals) {
	this.globals = globals;
	globals = this.switchs1.update(globals);
	globals = this.group1.propagate(globals);
	globals = this.switchs2.update(globals);
	globals = this.group2.propagate(globals);
	return this.switchs3.update(globals);
};
ResStack.prototype.namedGlyphs =
function() {
	return this.group1.namedGlyphs().concat(this.group2.namedGlyphs());
};
 */