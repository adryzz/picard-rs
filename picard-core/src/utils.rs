#[inline(always)]
pub fn bit<T>(n: T) -> T where T: std::ops::Shl<T, Output = T> + From<u8> {
    T::from(1_u8) << n
}