use crate::utils::bit;

const MAX_VSH_SIZE: usize = 512;


pub enum Comp {
    COMP_X = 0,
    COMP_Y,
    COMP_Z,
    COMP_W
}

pub enum Cond {
    COND_EQ = 0,
    COND_NE,
    COND_LT,
    COND_GT,
    COND_GE
}

pub enum Se {
    SE_PROC,
    SE_FOR,
    SE_IF,
    SE_ARRAY
}

pub fn mask_from_swizzling(mut sw: i32, reverse: bool) -> i32 {
    sw >>= 1; // get rid of negation bit
    let mut out = 0;
    for i in 0..4 {
        let mut bitid = (sw>>(i*2))&3;
        if reverse {
            bitid = 3 - bitid;
        }
        out |= bit(bitid);
    }

    return out;
}


#[inline(always)]
pub fn is_bad_input_reg_combination2(a: i32, b: i32) -> bool {
    a < 0x10 && b < 0x10 && a != b
}

#[inline(always)]
pub fn is_bad_input_reg_combination3(a: i32, b: i32, c: i32) -> bool {
    is_bad_input_reg_combination2(a, b) ||
    is_bad_input_reg_combination2(b, c) ||
    is_bad_input_reg_combination2(c, a)
}