use regex::Regex;
use regex_static::{lazy_regex};
use regex_static::once_cell::sync::Lazy;

pub static CAT_NAME_STRUCTURE: Lazy<Regex> = lazy_regex!(r##"^([A-I]|[K-Z]|(?:Aa)|(?:NL)|(?:NU))([1-9](?:[0-9][0-9]?)?)([a-z]?)$"##);
pub static MNEMONIC_STRUCTURE: Lazy<Regex> = lazy_regex!(r##"^[a-zA-Z]+$"##);
// pub static NON_CAT_NAME_STRUCTURE: Lazy<Regex> = lazy_regex!(r##"^(("([^\t\n\r\f\b"\\]|(")|(\\))")|(0|([1-9]([0-9][0-9]?)?)))$"##);
//
// /*
// ResContext.catNameStructure = /^([A-I]|[K-Z]|(?:Aa)|(?:NL)|(?:NU))([1-9](?:[0-9][0-9]?)?)([a-z]?)$/;
// ResContext.nonCatNameStructure = /^(("([^\t\n\r\f\b"\\]|(\\")|(\\\\))")|(0|([1-9]([0-9][0-9]?)?)))$/;
// ResContext.mnemonicStructure = /^[a-zA-Z]+$/;
//  */