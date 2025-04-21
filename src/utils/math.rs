use std::ops::{Div, Mul};

pub fn percentage_representation<T: Mul<Output = T> + Div<Output = T> + From<u8> + PartialOrd + Copy>(
    total: T,
    part: T,
) -> T {
    // TODO
    // I'm guessing this is wrong or could be improved. I'm dumb.
    if part <= total {
        part * 100.into() / total
    } else {
        total * 100.into() / total
    }
}
