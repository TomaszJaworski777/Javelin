use std::ops::{Shl, Shr, Not, BitOr, BitOrAssign, BitAnd, BitAndAssign, ShlAssign};
use colored::*;

#[inline]
pub fn set_bit_to_one<T>(value: &mut T, index: u8)
where
    T: Copy
        + Shl<u8, Output = T>
        + BitOrAssign
        + From<u8>,
{
    *value |= T::from(1u8) << index;
}

#[inline]
pub fn set_bit_to_zero<T>(value: &mut T, index: u8)
where
    T: Copy
        + Shl<u8, Output = T>
        + Not<Output = T>
        + BitAndAssign
        + From<u8>,
{
    *value &= !(T::from(1u8) << index);
}

#[inline]
pub fn get_bit<T>(value: T, index: u8) -> T
where
    T: Copy
        + Shl<u8, Output = T>
        + BitAnd<Output = T>
        + From<u8>,
{
    value & (T::from(1u8) << index)
}

#[inline]
pub fn set_bit_chunk<T>(value: &mut T, index: u8, mask: T, new_value: T)
where
    T: Copy
        + Shl<u8, Output = T>
        + Not<Output = T>
        + BitOr<Output = T>
        + BitAnd<Output = T>
        + BitAndAssign
        + ShlAssign,
{
    *value = (*value & !(mask << index)) | (new_value << index);
}

#[inline]
pub fn get_bit_chunk<T>(value: T, index: u8, mask: T) -> T
where
    T: Copy
        + Shl<u8, Output = T>
        + Shr<u8, Output = T>
        + BitAnd<Output = T>,
{
    (value & (mask << index)) >> index
}