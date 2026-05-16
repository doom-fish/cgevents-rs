//! `CGScrollPhase` values used with `kCGScrollWheelEventScrollPhase`.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CGScrollPhase {
    Began,
    Changed,
    Ended,
    Cancelled,
    MayBegin,
}

impl CGScrollPhase {
    #[must_use]
    pub const fn raw(self) -> u32 {
        match self {
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
            1 => Some(Self::Began),
            2 => Some(Self::Changed),
            4 => Some(Self::Ended),
            8 => Some(Self::Cancelled),
            128 => Some(Self::MayBegin),
            _ => None,
        }
    }
}
