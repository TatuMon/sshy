use serde::Serialize;

#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ColorVariant {
    Success,
    Warning,
    Danger
}
