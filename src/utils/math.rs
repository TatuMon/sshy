use std::ops::{Div, Mul};

pub fn percentage_representation<T: Mul<Output = T> + Div<Output = T> + From<u8>>(
    total: T,
    part: T,
) -> T {
    part * 100.into() / total
}
