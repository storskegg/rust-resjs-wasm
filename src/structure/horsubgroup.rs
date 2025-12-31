pub struct HorSubgroup {}

impl HorSubgroup {
    pub fn new() -> Self {
        Self {}
    }
}

/*
function ResHorsubgroup(args) {
	if (args.g) {
		this.switchs1 = args.sw1;
		this.group = args.g;
		this.switchs2 = args.sw2;
	} else if (args.b) {
		this.switchs1 = new ResSwitch(null);
		this.group = args.b;
		this.switchs2 = new ResSwitch(null);
	} else {
		this.switchs1 = args.switchs1;
		this.group = args.group;
		this.switchs2 = args.switchs2;
	}
};
ResHorsubgroup.prototype.toString =
function() {
	if (this.group instanceof ResVertgroup)
		return "(" + this.switchs1.toString() + this.group.toString() + ")" +
			this.switchs2.toString();
	else
		return this.switchs1.toString() + this.group.toString() +
			this.switchs2.toString();
};
ResHorsubgroup.prototype.propagateBack =
function(sw) {
	var swEnd = this.switchs2.join(sw);
	this.switchs2 = new ResSwitch(null);
	var swGroup = this.group.propagateBack(swEnd);
	var swStart = this.switchs1.join(swGroup);
	this.switchs1 = new ResSwitch(null);
	return swStart;
};
ResHorsubgroup.prototype.propagate =
function(globals) {
	globals = this.switchs1.update(globals);
	globals = this.group.propagate(globals);
	return this.switchs2.update(globals);
};
 */