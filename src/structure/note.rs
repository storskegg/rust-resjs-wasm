pub struct Note {}

impl Note {
    pub fn new() -> Self {
        Self {}
    }
}

/*
function ResNote(args) {
	if (args === null)
		this.setDefaults();
	else if (args.l) {
		var str = args.s;
		var argList = args.l;
		this.color = null;
		for (var i = 0; i < argList.length; i++) {
			var arg = argList[i];
			if (arg.isColor())
				this.color = arg.getLhs();
		}
		this.str = str;
	} else {
		this.color = args.color;
		this.str = args.str;
	}
}
ResNote.prototype.setDefaults =
function() {
	this.color = null;
	this.str = '"?"';
};
ResNote.prototype.toString =
function() {
	var args = [];
	if (this.color !== null)
		args.push(this.color);
	return "^" + this.str + ResArg.argsStr(args);
};
ResNote.prototype.displayString =
function() {
	var str = this.str.substring(1, this.str.length-1);
	str = str.replace(/\\(["\\])/g, "$1");
	return str;
};
ResNote.escapeString =
function(str) {
	str = str.replace(/\\/g, '\\\\');
	str = str.replace(/"/g, '\\"');
	return '\"' + str + '\"';
};
ResNote.prototype.propagate =
function(globals) {
	this.globals = globals;
};
 */
