#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoolType {
    Union,
    Difference,
    Intersection,
    Exclusion,
}

impl From<u8> for BoolType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Union,
            1 => Self::Difference,
            2 => Self::Intersection,
            3 => Self::Exclusion,
            _ => Self::default(),
        }
    }
}

impl Default for BoolType {
    fn default() -> Self {
        Self::Union
    }
}
