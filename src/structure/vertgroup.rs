pub struct VertGroup {}

impl VertGroup {
    pub fn new() -> Self {
        Self {}
    }
}

/*
function ResVertgroup(args) {
	if (args === null)
		this.setDefaults();
	else if (args.l) {
		var group1 = args.g1;
		var argList = args.l;
		var switchs = args.sw;
		var group2 = args.g2;
		this.setDefaults();
		this.groups.push(group1);
		this.ops.push(new ResOp({l:argList}, true));
		this.switches.push(switchs);
		this.groups.push(group2);
	} else {
		this.groups = args.groups;
		this.ops = args.ops;
		this.switches = args.switches;
	}
}
ResVertgroup.prototype.setDefaults =
function() {
	this.groups = [];
	this.ops = [];
	this.switches = [];
};
ResVertgroup.make =
function(groups, ops, switches) {
	var subgroups = [];
	for (var i = 0; i < groups.length; i++)
		subgroups.push(new ResVertsubgroup({b: groups[i]}));
	return new ResVertgroup({groups: subgroups, ops: ops, switches: switches});
};
ResVertgroup.prototype.toString =
function() {
	var s = this.groups[0].toString();
	for (var i = 0; i < this.ops.length; i++)
		s += ":" + this.ops[i].toString(i === 0) + this.switches[i].toString() +
			this.groups[i+1].toString();
	return s;
};
ResVertgroup.prototype.addGroup =
function(argList, switchs, group) {
	this.ops.push(new ResOp({l:argList}, false));
	this.switches.push(switchs);
	this.groups.push(group);
	return this;
};
ResVertgroup.prototype.addGroupAt =
function(group, i) {
	this.groups.splice(i, 0, new ResVertsubgroup({b: group}));
	this.ops.splice(Math.min(i, this.ops.length), 0, new ResOp(null));
	this.switches.splice(Math.min(i, this.switches.length), 0, new ResSwitch(null));
};
ResVertgroup.prototype.propagateBack =
function(sw) {
	for (var i = 1; i < this.groups.length; i++) {
		var swGroup = (i === this.groups.length - 1) ?
			this.groups[i].propagateBack(sw) :
				this.groups[i].propagateBack(new ResSwitch(null));
		this.switches[i-1] = this.switches[i-1].join(swGroup);
	}
	return this.groups[0].propagateBack(new ResSwitch(null));
};
ResVertgroup.prototype.propagate =
function(globals) {
	this.globals = globals;
	globals = this.groups[0].propagate(globals);
	for (var i = 0; i < this.ops.length; i++) {
		this.ops[i].propagate(globals);
		globals = this.switches[i].update(globals);
		globals = this.groups[i+1].propagate(globals);
	}
	return globals;
};
ResVertgroup.prototype.effectiveSize =
function() {
	return this.ops[0].size !== null ? this.ops[0].size : this.globals.size;
};
ResVertgroup.prototype.subGroups =
function() {
	var l = [];
	for (var i = 0; i < this.groups.length; i++)
		l.push(this.groups[i].group);
	return l;
};
ResVertgroup.prototype.nPaddable =
function() {
	var n = 0;
	for (var i = 0; i < this.ops.length; i++)
		if (!this.ops[i].fix)
			n++;
	return n;
};
ResVertgroup.prototype.namedGlyphs =
function() {
	var l = [];
	for (var i = 0; i < this.groups.length; i++)
		l = l.concat(this.groups[i].group.namedGlyphs());
	return l;
};
 */