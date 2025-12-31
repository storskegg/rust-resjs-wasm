pub struct Switch {}

impl Switch {
    pub fn new() -> Self {
        Self {}
    }
}

/*
function ResSwitch(args) {
	if (args === null)
		this.setDefaults();
	else if (args.l) {
		var argList = args.l;
		this.setDefaults();
		for (var i = 0; i < argList.length; i++) {
			var arg = argList[i];
			if (arg.isColor())
				this.color = arg.getLhs();
			else if (arg.is("shade"))
				this.shade = true;
			else if (arg.is("noshade"))
				this.shade = false;
			else if (arg.hasLhs("sep") && arg.hasRhsReal())
				this.sep = arg.getRhs();
			else if (arg.is("fit"))
				this.fit = true;
			else if (arg.is("nofit"))
				this.fit = false;
			else if (arg.is("mirror"))
				this.mirror = true;
			else if (arg.is("nomirror"))
				this.mirror = false;
		}
	} else {
		this.color = args.color;
		this.shade = args.shade;
		this.sep = args.sep;
		this.fit = args.fit;
		this.mirror = args.mirror;
	}
}
ResSwitch.prototype.setDefaults =
function() {
	this.color = null;
	this.shade = null;
	this.sep = null;
	this.fit = null;
	this.mirror = null;
};
ResSwitch.prototype.toString =
function() {
	var args = [];
	if (this.color !== null)
		args.push(this.color);
	if (this.shade === true)
		args.push("shade");
	else if (this.shade === false)
		args.push("noshade");
	if (this.sep !== null)
		args.push("sep=" + ResArg.realStr(this.sep));
	if (this.fit === true)
		args.push("fit");
	else if (this.fit === false)
		args.push("nofit");
	if (this.mirror === true)
		args.push("mirror");
	else if (this.mirror === false)
		args.push("nomirror");
	if (args.length > 0)
		return "!" + ResArg.argsStr(args);
	else
		return "";
};
ResSwitch.prototype.hasDefaultValues =
function() {
	return this.color === null &&
		this.shade === null &&
		this.sep === null &&
		this.fit === null &&
		this.mirror === null;
};
ResSwitch.prototype.join =
function(other) {
	var copy = new ResSwitch(null);
	for (var i = 0; i < ResGlobals.properties.length; i++) {
		var global = ResGlobals.properties[i];
		if (other[global] !== null)
			copy[global] = other[global];
		else
			copy[global] = this[global];
	}
	return copy;
};
ResSwitch.prototype.update =
function(globals) {
	var allNull = true;
	for (var i = 0; i < ResGlobals.properties.length; i++) {
		var global = ResGlobals.properties[i];
		if (this[global] !== null) {
			allNull = false;
			break;
		}
	}
	if (allNull)
		return globals;
	else {
		var copy = globals.clone();
		for (var i = 0; i < ResGlobals.properties.length; i++) {
			var global = ResGlobals.properties[i];
			if (this[global] !== null)
				copy[global] = this[global];
		}
		return copy;
	}
};
 */