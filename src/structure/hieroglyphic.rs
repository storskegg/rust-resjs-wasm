pub struct Hieroglyphic {
}

impl Hieroglyphic {
    pub fn new() -> Self {
        Self {}
    }
}

/*
function ResHieroglyphic(args) {
	this.groups = [];
	this.ops = [];
	this.switches = [];
	if (args.g) {
		var group = args.g;
		this.groups.push(group);
	} else {
		this.groups = args.groups;
		this.ops = args.ops;
		this.switches = args.switches;
	}
}
ResHieroglyphic.prototype.toString =
function() {
	var s = this.groups[0].toString();
	for (var i = 0; i < this.ops.length; i++)
		s += "-" + this.ops[i].toString(false) + this.switches[i].toString() +
				this.groups[i+1].toString();
	return s;
};
ResHieroglyphic.prototype.addGroup =
function(group, argList, switchs) {
	this.groups.unshift(group);
	this.ops.unshift(new ResOp({l:argList}, false));
	this.switches.unshift(switchs);
	return this;
};
ResHieroglyphic.prototype.addGroupAt =
function(group, i) {
	this.groups.splice(i, 0, group);
	this.ops.splice(Math.min(i, this.ops.length), 0, new ResOp(null));
	this.switches.splice(Math.min(i, this.switches.length), 0, new ResSwitch(null));
};
ResHieroglyphic.prototype.propagateBack =
function() {
	for (var i = 0; i < this.switches.length; i++) {
		var sw = this.groups[i+1].propagateBack(new ResSwitch(null));
		this.switches[i] = this.switches[i].join(sw);
	}
	return this.groups[0].propagateBack(new ResSwitch(null));
};
ResHieroglyphic.prototype.propagate =
function(globals, direction) {
	this.globals = globals;
	this.globalss = [];
	this.globalss.push(globals);
	globals = this.groups[0].propagate(globals);
	for (var i = 0; i < this.ops.length; i++) {
		this.ops[i].propagate(globals);
		globals = this.switches[i].update(globals);
		this.globalss.push(globals);
		globals = this.groups[i+1].propagate(globals);
	}
	this.direction = direction;
	return globals;
};
ResHieroglyphic.prototype.effectiveIsH =
function() {
	return ResGlobals.isH(this.direction);
};
ResHieroglyphic.prototype.effectiveIsV =
function() {
	return ResGlobals.isV(this.direction);
};
ResHieroglyphic.prototype.namedGlyphs =
function() {
	var l = [];
	for (var i = 0; i < this.groups.length; i++)
		l = l.concat(this.groups[i].namedGlyphs());
	return l;
};
 */