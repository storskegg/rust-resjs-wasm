pub use std::clone::*;
use std::iter::Map;
use points::*;
use crate::aux::{shading, ShadingPattern};
use crate::points;

#[derive(Clone, Debug)]
pub enum Fonts {
    Hieroglyphic,
    HieroglyphicAux,
    HieroglyphicPlain,
}

pub fn get_font(font: &str) -> Fonts {
    match font {
        "Hieroglyphic" => Fonts::Hieroglyphic,
        "HieroglyphicAux" => Fonts::HieroglyphicAux,
        "HieroglyphicPlain" => Fonts::HieroglyphicPlain,
        _ => panic!("Unknown font: {}", font),
    }
}

pub fn get_font_text(font: &Fonts) -> &'static str {
    match font {
        Fonts::Hieroglyphic => "Hieroglyphic",
        Fonts::HieroglyphicAux => "HieroglyphicAux",
        Fonts::HieroglyphicPlain => "HieroglyphicPlain",
        _ => panic!("Unknown font: {:?}", font),
    }
}

pub struct Context {
    // fonts
    pub fonts: Vec<Fonts>,
    // unit font size
    pub em_size_px: i32,
    // note size
    pub note_size_px: i32,

    // separation between groups
    pub op_sep_em: f64, // normal separation for operators
    pub box_sep_em: f64, // separation in box
    pub padding_factor: f64, // as factor of unpadded separation between groups
    pub padding_allowed: bool,

    // shading
    pub shading_sep: f64,
    pub shading_thickness: f64,
    pub shading_color: String,
    pub shading_pattern: shading::ShadingPattern,

    // formatting
    pub iterate_limit: f64, // how often attempted to scaled down group
    pub scale_limit_em: f64, // no group can be scaled down smaller than this

    // insert
    pub scale_init: f64, // initial scale down for insert
    pub scale_step: f64, // initial step for increasing scale
    pub scale_step_min: f64, // smallest step for increasing scale
    pub scale_step_factor: f64, // for decreasing scale step
    pub move_step_min: f64, // minimal move step
    pub move_step_factor: f64, // for decreasing move step

    // rendering
    pub margin_px: i32,
    pub box_overlap_px: i32,

    // notes
    pub note_color: String, // default color of notes
    pub note_margin: f64, // pixels around note

    // forced direction
    pub dir: Option<String>, // for RESlite can be null, "lr", "rl"

    pub aux_points: points::AuxPoints,

    catToNames: Option<Map<&'static str, Vec<&'static str>>>,
}

impl Context {
    pub fn new() -> Self {
        let fonts: Vec<Fonts> = [Fonts::Hieroglyphic, Fonts::HieroglyphicAux, Fonts::HieroglyphicPlain].to_vec();

        Context {
            // fonts
            fonts,
            // unit font size
            em_size_px: 36,
            // note size
            note_size_px: 12,

            // separation between groups
            op_sep_em: 0.15,
            box_sep_em: 0.04,
            padding_factor: 1.0,
            padding_allowed: false,

            // shading
            shading_sep: 4.0,
            shading_thickness: 1.0,
            shading_color: "gray".to_string(),
            shading_pattern: ShadingPattern::XIsY,

            // formatting
            iterate_limit: 4.0,
            scale_limit_em: 0.01,

            // insert
            scale_init: 0.05,
            scale_step: 1.0,
            scale_step_min: 0.02,
            scale_step_factor: 0.4,
            move_step_min: 0.02,
            move_step_factor: 0.6,

            // rendering
            margin_px: 2,
            box_overlap_px: 1,

            note_color: "black".to_string(),
            note_margin: 2.0,

            // forced direction
            dir: None,

            aux_points: AuxPoints::new(),
            catToNames: None,
        }
    }

    pub fn mil_em_to_px(&self, size_mil_em: f64) -> f64 {
        size_mil_em * (self.em_size_px as f64) / 1000.0
    }

    pub fn px_to_mil_em(&self, size_px: i32) -> f64 {
        if (size_px as f64) == f64::MAX {
            f64::MAX // HAHAHA ja probably not. But here for good practice and to be safe :-)
        } else {
            1000.0 * (size_px as f64) / (self.em_size_px as f64)
        }
    }

    pub fn un_mnemonic(&self, code: &'static str) -> Option<&'static str> {
        let key = get_mnemonic_res(code);
        match key {
            Some(key) => Some(key),
            None => Some(code.clone()),
        }
    }

    pub fn un_bracket(&self, code: &'static str) -> Option<&'static str> {
        match code {
            "open" => Some("V11a"),
            "close" => Some("V11b"),
            _ => Some(code.clone()),
        }
    }

    fn make_cat_to_names(&self) {
        // let iter = points::HIERO_POINTS.lock().unwrap().keys();
        // for x in iter {
        //
        // }
    }
}

/*
// Mapping from category to ordered list of names, after call of ResContext.makeCatToNames.
ResContext.makeCatToNames = function() {
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
 */