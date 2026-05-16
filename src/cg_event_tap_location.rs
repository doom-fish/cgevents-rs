//! `CGEventTapLocation` values.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CGEventTapLocation {
    Hid,
    #[default]
    Session,
    AnnotatedSession,
}

impl CGEventTapLocation {
    #[must_use]
    pub const fn raw(self) -> u32 {
        match self {
            Self::Hid => 0,
            Self::Session => 1,
            Self::AnnotatedSession => 2,
        }
    }

    #[must_use]
    pub const fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            0 => Some(Self::Hid),
            1 => Some(Self::Session),
            2 => Some(Self::AnnotatedSession),
            _ => None,
        }
    }
}

/// Backwards-compatible alias used by the pre-v0.5 safe API.
pub use CGEventTapLocation as TapLocation;
