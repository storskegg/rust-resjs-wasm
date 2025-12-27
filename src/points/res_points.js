





ResContext.catNameStructure = /^([A-I]|[K-Z]|(?:Aa)|(?:NL)|(?:NU))([1-9](?:[0-9][0-9]?)?)([a-z]?)$/;
ResContext.nonCatNameStructure = /^(("([^\t\n\r\f\b"\\]|(\\")|(\\\\))")|(0|([1-9]([0-9][0-9]?)?)))$/;
ResContext.mnemonicStructure = /^[a-zA-Z]+$/;
// Mapping from category to ordered list of names, after call of ResContext.makeCatToNames.
ResContext.catToNames = {};
ResContext.makeCatToNames =
function() {
	for (var name in ResContext.hieroPoints) {
		var parts = ResContext.catNameStructure.exec(name);
		var cat = parts[1];
		if (ResContext.catToNames[cat] === undefined)
			ResContext.catToNames[cat] = [];
		ResContext.catToNames[cat].push(name);
	}
	for (var i = 0; i < ResContext.categories.length; i++) {
		var cat = ResContext.categories[i];
		ResContext.catToNames[cat].sort(ResContext.compareSignNames);
	}
	ResContext.catToNames["tall"] = ResContext.tallSigns;
	ResContext.catToNames["broad"] = ResContext.broadSigns;
	ResContext.catToNames["narrow"] = ResContext.narrowSigns;
};
ResContext.compareSignNames =
function(name1, name2) {
	var parts1 = ResContext.catNameStructure.exec(name1);
	var parts2 = ResContext.catNameStructure.exec(name2);
	var cat1 = parts1[1];
	var cat2 = parts2[1];
	var num1 = parseInt(parts1[2]);
	var num2 = parseInt(parts2[2]);
	var suf1 = parts1[3];
	var suf2 = parts2[3];
	if (cat1 === cat2) {
		if (num1 < num2)
			return -1;
		else if (num1 > num2)
			return 1;
		else if (suf1 < suf2)
			return -1;
		else if (suf1 > suf2)
			return 1;
		else
			return 0;
	} else 
		return ResContext.categories.indexOf(cat1) - ResContext.categories.indexOf(cat2);
};
