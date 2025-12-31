pub struct Arg {}

impl Arg {
    pub fn new() -> Self {
        Self {}
    }
}

/*
function ResArg(lhs, rhs) {
	this.lhs = lhs;
	this.rhs = rhs;
}
ResArg.prototype.getLhs =
function() {
	return this.lhs;
};
ResArg.prototype.getRhs =
function() {
	return this.rhs;
};
ResArg.prototype.is =
function(lhs) {
	return this.lhs === lhs && this.rhs === null;
};
ResArg.prototype.isColor =
function() {
	return this.is("black") || this.is("red") || this.is("green") || this.is("blue") ||
			this.is("white") || this.is("aqua") || this.is("fuchsia") || this.is("gray") ||
			this.is("lime") || this.is("maroon") || this.is("navy") || this.is("olive") ||
			this.is("purple") || this.is("silver") || this.is("teal") || this.is("yellow");
};
ResArg.prototype.isPattern =
function() {
	return this.rhs === null && this.lhs.search(/^[tbse]+$/) >= 0;
};
ResArg.prototype.hasLhs =
function(lhs) {
	return this.lhs === lhs;
};
ResArg.prototype.hasRhs =
function(rhs) {
	return this.rhs === rhs;
};
ResArg.prototype.hasRhsNatnum =
function() {
	return typeof this.rhs === 'number' && this.rhs % 1 === 0;
};
ResArg.prototype.hasRhsReal =
function() {
	return typeof this.rhs === 'number';
};
ResArg.prototype.hasRhsNonzeroReal =
function() {
	return typeof this.rhs === 'number' && this.rhs > 0;
};
ResArg.prototype.hasRhsLowReal =
function() {
	return typeof this.rhs === 'number' && this.rhs <= 1;
};
ResArg.argsStr =
function(args) {
	if (args.length === 0)
		return "";
	else {
		var s = "[" + args[0];
		for (var i = 1; i < args.length; i++)
			s += "," + args[i];
		s += "]";
		return s;
	}
};
ResArg.realStr =
function(val) {
	val -= Math.floor(val / 10) * 10;
	val = Math.floor(val * 100.0);
	var hundreds = Math.floor(val / 100);
	val -= hundreds * 100;
	var tens = Math.floor(val / 10);
	val -= tens * 10;
	var s = hundreds > 0 ? ("" + hundreds) : "0";
	if (tens > 0 || val > 0) {
		s += "." + tens;
		if (val > 0)
			s += val;
	}
	return s;
};

 */
