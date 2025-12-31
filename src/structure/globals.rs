use std::ops::Deref;
use phf::phf_map;

#[derive(Clone)]
pub enum Direction {
    Hlr,
    Hrl,
    Vlr,
    Vrl
}

/// Internally maps string to enum. keys must be string literals, so we cannot DRY this out any
/// further (pointing also to the as_str method below)
static MAP_DIRECTIONS: phf::Map<&'static str, Direction> = phf_map! {
    "hlr" => Direction::Hlr,
    "hrl" => Direction::Hrl,
    "vlr" => Direction::Vlr,
    "vrl" => Direction::Vrl
};

impl Direction {
    pub fn is_horizontal(&self) -> bool {
        match self {
            Direction::Hlr | Direction::Hrl => true,
            Direction::Vlr | Direction::Vrl => false
        }
    }

    pub fn is_vertical(&self) -> bool {
        !self.is_horizontal()
    }

    pub fn is_lr(&self) -> bool {
        match self {
            Direction::Hlr | Direction::Vlr => true,
            Direction::Hrl | Direction::Vrl => false
        }
    }

    pub fn is_rl(&self) -> bool {
        !self.is_lr()
    }

    pub fn get(s: &str) -> Option<Direction> {
        MAP_DIRECTIONS.get(s).cloned()
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Direction::Hlr => "hlr",
            Direction::Hrl => "hrl",
            Direction::Vlr => "vlr",
            Direction::Vrl => "vrl"
        }
    }

    pub fn as_string(&self) -> String {
        self.as_str().to_string()
    }
}

#[derive(Clone)]
pub struct Globals {
    direction: Direction,
    size_header: i32,
    size: i32,
    color: String,
    shade: bool,
    sep: f64,
    fit: bool,
    mirror: bool
}

impl Globals {
    pub fn new(direction: Option<Direction>, size: Option<i32>) -> Self {
        Self {
            direction: direction.unwrap_or(Direction::Hlr),
            size_header: size.unwrap_or(1),
            size: size.unwrap_or(1),
            color: "black".to_string(),
            shade: false,
            sep: 1.0,
            fit: false,
            mirror: false
        }
    }

    pub fn direction(&self) -> Direction {
        self.direction.clone()
    }

    pub fn is_horizontal(&self) -> bool {
        self.direction.is_horizontal()
    }

    pub fn is_vertical(&self) -> bool {
        self.direction.is_vertical()
    }

    pub fn is_lr(&self) -> bool {
        self.direction.is_lr()
    }

    pub fn is_rl(&self) -> bool {
        self.direction.is_rl()
    }

    fn update(&self, size: i32) -> Self {
        let cur_direction = self.direction();
        if self.size == size {
            return self.clone();
        }

        Self::new(Some(cur_direction), Some(size))
    }
}
