use color_eyre::eyre::{Context, Result};

pub fn parse_usize_to_u16(value: usize) -> Result<u16> {
    u16::try_from(value).wrap_err("Value out of range")
}
