//! `CGMomentumScrollPhase` values used with `kCGScrollWheelEventMomentumPhase`.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CGMomentumScrollPhase {
    #[default]
    None,
    Begin,
    Continue,
    End,
}

impl CGMomentumScrollPhase {
    #[must_use]
    pub const fn raw(self) -> u32 {
        match self {
            Self::None => 0,
            Self::Begin => 1,
            Self::Continue => 2,
            Self::End => 3,
        }
    }

    #[must_use]
    pub const fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            0 => Some(Self::None),
            1 => Some(Self::Begin),
            2 => Some(Self::Continue),
            3 => Some(Self::End),
            _ => None,
        }
    }
}
