use phf::phf_ordered_set;

pub static TALL_SIGNS: phf::OrderedSet<&'static str> = phf_ordered_set! {
    "M40","Aa28","Aa29","P11","D16","T34","T35","U28",
    "U29","U32","U33","S43","U36","T8","T8a","M13","M17",
    "H6","H6a","M4","M12","S29","M29","M30","S37","R14",
    "R15","R16","R17","P6","S40","R19","S41","F10",
    "F11","F12","S38","S39","T14","T15","T13","Aa26",
    "O30","Aa21","U39","F45","O44","Aa27","R8","R9",
    "T7a","T3","T4","V24","V25","U23","S42",
    "U34","S36","F28","U26","U27","U24","U25","Y8",
    "F35","F36","U41","W19","P8","T22","T23","Z11",
    "S44","Aa25","M44","V38","Aa31","Aa30","Aa20","V36",
    "F31","M32","L7","V17","V18","S34","V39","Q7",
    "T18","T19","T20","R21","R11","O28","O11","O36",
    "Aa32","V28","V29",
};

pub static BROAD_SIGNS: phf::OrderedSet<&'static str> = phf_ordered_set! {
    "N1","N37","N38","N39","S32","N18","X4","X5",
    "N17","N16","N20","Aa10","Aa11","Aa12","Aa13","Aa14",
    "Aa15","N35","Aa8","Aa9","V26","V27","R24","W8",
    "V32","Y1","Y2","R4","N11","N12","F42","D24",
    "D25","D13","D15","F20","Z6","F33","T2","T7",
    "F30","V22","V23","R5","R6","O34","V2","V3",
    "S24","R22","R23","T11","O29","T1","T21","U20",
    "U19","U21","D17","U31","T9","T9a","T10","F32",
    "V13","V14","F46","F47","F48","F49","M11","U17",
    "U18","U14","Aa7","F18","D51","U15","U16","Aa24",
    "N31","O31","N36","D14","D21","D22","T30","T31",
    "T33","D48","V30","V31","W3","S12","N30","O42",
    "O43","V16",
};

pub static NARROW_SIGNS: phf::OrderedSet<&'static str> = phf_ordered_set! {
    "Q3","O39","Z8","O47","N22","N21","N23","N29",
    "X7","O45","O46","Y6","M35","X3","X2","X1",
    "N28","Aa17","I6","W10","W10a","Aa4","R7","M39",
    "M36","F43","F41","N34","U30","W11","W12","W13",
    "T28","N41","N42","V37","M31","F34","W6","W7",
    "W21","W20","V6","V33","V34","V7","V8","S20",
    "V20","V19","Aa19","Aa2","Aa3","N32","F52","V35",
    "H8","M41","F51","D11","K6","L6","F21","D26",
    "N33","D12","S21","N5","N9","N10","Aa1","O50",
    "O49","O48","X6","V9","S10","N6","N8","S11",
    "N15","M42","F38","V1","Z7","Aa16","Z9","Z10",
};

pub static CATEGORIES: phf::OrderedSet<&'static str> = phf_ordered_set! {
    "A","B","C","D","E","F","G","H","I",
    "K","L","M","N","NL","NU","O","P","Q","R","S","T","U","V",
    "W","X","Y","Z","Aa",
};

pub static EXTRA_CATEGORIES: phf::OrderedSet<&'static str> = phf_ordered_set! {
    "tall","broad","narrow",
};

// TODO: Examine the code that makes use of this to see about refactoring to use both of the above arrays
pub static CATEGORIES_AND_EXTRA: phf::OrderedSet<&'static str> = phf_ordered_set! {
    "A","B","C","D","E","F","G","H","I",
    "K","L","M","N","NL","NU","O","P","Q","R","S","T","U","V",
    "W","X","Y","Z","Aa","tall","broad","narrow",
};
