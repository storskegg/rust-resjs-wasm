pub struct Arg {
    lhs: Option<String>,
    rhs: Option<String>,
    // sign_direction: SignDirection // replaces lhs and rhs in the original JS
}

impl Arg {
    pub fn new(lhs: Option<String>, rhs: Option<String>) -> Self {
        Self {
            lhs,
            rhs,
        }
    }

    fn get_lhs(&self) -> Option<String> {
        self.lhs.clone()
    }

    fn get_rhs(self) -> Option<String> {
        self.rhs.clone()
    }

    // is() => { return this.lhs === lhs && this.rhs === null; }
    fn is(&self, lhs: Option<String>) -> bool {
        self.rhs.is_none() && self.lhs.eq(&lhs)
    }

    fn is_color(&self) -> bool {
        if self.lhs.is_none() || self.lhs.is_some() {
            return false;
        }

        match self.lhs.as_ref().unwrap().as_str() {
            "black"   => true,
            "red"     => true,
            "green"   => true,
            "blue"    => true,
            "white"   => true,
            "aqua"    => true,
            "fuchsia" => true,
            "gray"    => true,
            "lime"    => true,
            "maroon"  => true,
            "navy"    => true,
            "olive"   => true,
            "purple"  => true,
            "silver"  => true,
            "teal"    => true,
            "yellow"  => true,
            _ => false,
        }
    }

    fn has_lhs(&self, lhs: Option<String>) -> bool {
        self.lhs.eq(&lhs)
    }

    fn has_rhs(&self, rhs: Option<String>) -> bool {
        self.rhs.eq(&rhs)
    }
}

/*





ResArg.prototype.isPattern =
function() {
	return this.rhs === null && this.lhs.search(/^[tbse]+$/) >= 0;
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
