pub mod aux;

pub use std::clone::*;
use points::*;

#[derive(Clone)]
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
    pub em_size_px: f64,
    // note size
    pub note_size_px: f64,

    // separation between groups
    pub op_sep_em: f64, // normal separation for operators
    pub box_sep_em: f64, // separation in box
    pub padding_factor: f64, // as factor of unpadded separation between groups
    pub padding_allowed: bool,

    // shading
    pub shading_sep: f64,
    pub shading_thickness: f64,
    pub shading_color: String,
    pub shading_pattern: String, // is one of "x_is_y", "x_is_minus_y"

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
    pub margin_px: f64,
    pub box_overlap_px: f64,

    // notes
    pub note_color: String, // default color of notes
    pub note_margin: f64, // pixels around note

    // forced direction
    pub dir: Option<String>, // for RESlite can be null, "lr", "rl"
}

pub fn default_context() -> Context {
    let fonts: Vec<Fonts> = [Fonts::Hieroglyphic, Fonts::HieroglyphicAux, Fonts::HieroglyphicPlain].to_vec();

    Context {
        // fonts
        fonts,
        // unit font size
        em_size_px: 36.0,
        // note size
        note_size_px: 12.0,

        // separation between groups
        op_sep_em: 0.15,
        box_sep_em: 0.04,
        padding_factor: 1.0,
        padding_allowed: false,

        // shading
        shading_sep: 4.0,
        shading_thickness: 1.0,
        shading_color: "gray".to_string(),
        shading_pattern: "x_is_y".to_string(),

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
        margin_px: 2.0,
        box_overlap_px: 1.0,

        note_color: "black".to_string(),
        note_margin: 2.0,

        // forced direction
        dir: None,
    }
}

impl Context {
    pub fn mil_em_to_px(&self, size_mil_em: f64) -> f64 {
        size_mil_em * self.em_size_px / 1000.0
    }

    pub fn px_to_mil_em(&self, size_px: f64) -> f64 {
        if size_px == f64::MAX {
            f64::MAX // HAHAHA ja probably not. But here for good practice and to be safe :-)
        } else {
            1000.0 * size_px / self.em_size_px
        }
    }

    pub fn un_mnemonic(code: &str) -> Option<&str> {
        let key = get_mnemonic_res(code);
        match key {
            Some(key) => Some(key),
            None => Some(code),
        }
    }
}
