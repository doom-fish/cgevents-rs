//! `CGEventTimestamp` newtype.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CGEventTimestamp(pub u64);

impl CGEventTimestamp {
    #[must_use]
    pub const fn raw(self) -> u64 {
        self.0
    }

    #[must_use]
    pub const fn from_raw(raw: u64) -> Self {
        Self(raw)
    }
}
