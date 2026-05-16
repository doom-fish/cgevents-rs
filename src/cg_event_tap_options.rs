//! `CGEventTapOptions` values.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CGEventTapOptions {
    #[default]
    Default,
    ListenOnly,
}

impl CGEventTapOptions {
    #[must_use]
    pub const fn raw(self) -> u32 {
        match self {
            Self::Default => 0,
            Self::ListenOnly => 1,
        }
    }

    #[must_use]
    pub const fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            0 => Some(Self::Default),
            1 => Some(Self::ListenOnly),
            _ => None,
        }
    }
}
