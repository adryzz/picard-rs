#[inline(always)]
pub fn bit<T>(n: T) -> T where T: std::ops::Shl<T, Output = T> + From<u8> {
    T::from(1_u8) << n
}


// TODO: check this for correctness (and search for a better way)
pub fn f32tof24(f: f32) -> u32 {
    let mut i: u32 = 0;
    unsafe {
        std::ptr::copy_nonoverlapping(&f, &mut i as *mut u32 as *mut f32, 1);
    }

    let mantissa: u32 = (i << 9) >> 9;
    let exponent: i32 = ((i << 1) as i32) >> 24;
    let sign: u32 = (i << 0) >> 31;

    // Truncate mantissa
    let mantissa = mantissa >> 7;

    // Re-bias exponent
    let mut exponent = exponent - 127 + 63;
    if exponent < 0 {
        // Underflow: flush to zero
        return sign << 23;
    } else if exponent > 0x7F {
        // Overflow: saturate to infinity
        return (sign << 23) | (0x7F << 16);
    }

    (sign << 23) | ((exponent as u32) << 16) | mantissa
}