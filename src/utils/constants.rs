#[cfg(windows)]
pub const LINE_TERMINATOR: &[u8;2] = b"\r\n";

#[cfg(not(windows))]
pub const LINE_TERMINATOR: &[u8;1] = b"\n";
