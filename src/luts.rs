pub mod luts;

use luts::*;

pub fn get_code_point_with_hiero(hiero: &str) -> i32 {
    HIERO_TO_CODE_POINTS_LUT.get(hiero).unwrap().clone()
}
