//! `CGEventMouseSubtype` values used with `kCGMouseEventSubtype`.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CGEventMouseSubtype {
    Default,
    TabletPoint,
    TabletProximity,
}

impl CGEventMouseSubtype {
    #[must_use]
    pub const fn raw(self) -> u32 {
        match self {
            Self::Default => 0,
            Self::TabletPoint => 1,
            Self::TabletProximity => 2,
        }
    }

    #[must_use]
    pub const fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            0 => Some(Self::Default),
            1 => Some(Self::TabletPoint),
            2 => Some(Self::TabletProximity),
            _ => None,
        }
    }
}
