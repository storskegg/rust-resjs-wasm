pub struct ParseBuffer {}

impl ParseBuffer {
    pub fn new() -> Self {
        Self {}
    }
}

/*
function ParseBuffer(string) {
	this.string = string;
	this.pos = 0;
	this.error = false;
}
ParseBuffer.prototype.isEmpty =
function() {
	return this.pos === this.string.length;
};
ParseBuffer.prototype.remainder =
function() {
	return this.string.substring(this.pos);
};
ParseBuffer.prototype.readToNonspace =
function() {
	while (!this.isEmpty() &&
			this.string.charAt(this.pos).replace(/^\s+|\s+$/gm,'').length === 0)
		this.pos++;
};
ParseBuffer.prototype.readToSpace =
function() {
	while (!this.isEmpty() &&
			this.string.charAt(this.pos).replace(/^\s+|\s+$/gm,'').length !== 0)
		this.pos++;
};
ParseBuffer.prototype.readToEnd =
function() {
	while (!this.isEmpty() && this.string.charAt(this.pos) !== 'e')
		this.pos++;
	if (!this.isEmpty())
		this.pos++;
};
ParseBuffer.prototype.readChar =
function(c) {
	var oldPos = this.pos;
	this.readToSpace();
	if (this.pos === oldPos+1 && this.string.charAt(oldPos) === c) {
		this.readToNonspace();
		return true;
	} else {
		this.pos = oldPos;
		return false;
	}
};
ParseBuffer.prototype.readSingleChar =
function(c) {
	if (!this.isEmpty() && this.string.charAt(this.pos) === c) {
		this.pos++;
		return true;
	} else
		return false;
};
ParseBuffer.prototype.peekChar =
function(c) {
	return !this.isEmpty() && this.string.charAt(this.pos) === c;
}
ParseBuffer.prototype.readDirection =
function() {
	var dir = undefined;
	var oldPos = this.pos;
	this.readToSpace();
	if (this.pos !== oldPos+3) {
		this.pos = oldPos;
		return dir;
	} else if (this.string.indexOf("hlr", oldPos) === oldPos)
		dir = "hlr";
	else if (this.string.indexOf("hrl", oldPos) === oldPos)
		dir = "hrl";
	else if (this.string.indexOf("vlr", oldPos) === oldPos)
		dir = "vlr";
	else if (this.string.indexOf("vrl", oldPos) === oldPos)
		dir = "vrl";
	else {
		this.pos = oldPos;
		return dir;
	}
	this.readToNonspace();
	return dir;
};
ParseBuffer.prototype.readNum =
function() {
	var i = undefined;
	var oldPos = this.pos;
	this.readToSpace();
	if (this.pos <= oldPos)
		return i;
	if (this.string.substring(oldPos, this.pos).replace(/^\-?[0-9]*$/gm,'').length === 0)
		i = parseInt(this.string.substring(oldPos, this.pos));
	this.readToNonspace();
	return i;
};
ParseBuffer.prototype.readString =
function() {
	var end = this.readAcrossString();
	if (end >= this.pos + 3) {
		var sub = "";
		for (var i = this.pos+1; i < end-1; i++) {
			if (this.string.charAt(i) === '\\') {
				if (i+1 < end-1)
					sub += this.string.charAt(i+1);
				i++;
			} else
				sub += this.string.charAt(i);
		}
		this.pos = end;
		this.readToNonspace();
		return sub;
	} else
		return undefined;
};
ParseBuffer.prototype.readAcrossString =
function() {
	var newPos = this.pos;
	if (this.string.charAt(newPos) === '\"') {
		newPos++;
		while (newPos < this.string.length) {
			if (this.string.charAt(newPos) === '\"') {
				newPos++;
				if (newPos >= this.string.length ||
						this.string.charAt(newPos).replace(/^\s+|\s+$/gm,'').length === 0)
					return newPos;
				else
					return undefined;
			} else if (this.string.charAt(newPos) === '\\') {
				newPos++;
				if (this.string.charAt(newPos) === '\"' ||
						this.string.charAt(newPos) === '\\')
					newPos++;
				else
					return undefined;
			} else if (this.string.charAt(newPos) === ' ' ||
					this.string.charAt(newPos).replace(/^\s+|\s+$/gm,'').length !== 0) {
				newPos++;
			} else
				return undefined;
		}
		return undefined;
	} else
		return undefined;
};
 */