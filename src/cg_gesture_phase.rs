//! `CGGesturePhase` values.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CGGesturePhase {
    #[default]
    None,
    Began,
    Changed,
    Ended,
    Cancelled,
    MayBegin,
}

impl CGGesturePhase {
    #[must_use]
    pub const fn raw(self) -> u32 {
        match self {
            Self::None => 0,
            Self::Began => 1,
            Self::Changed => 2,
            Self::Ended => 4,
            Self::Cancelled => 8,
            Self::MayBegin => 128,
        }
    }

    #[must_use]
    pub const fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            0 => Some(Self::None),
            1 => Some(Self::Began),
            2 => Some(Self::Changed),
            4 => Some(Self::Ended),
            8 => Some(Self::Cancelled),
            128 => Some(Self::MayBegin),
            _ => None,
        }
    }
}
