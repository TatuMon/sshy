use serde::Serialize;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Debug)]
pub enum ColorVariant {
    Success,
    Warning,
    Danger
}
