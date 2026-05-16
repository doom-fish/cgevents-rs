//! `CGEventFlags` / modifier-state helpers.

bitflags::bitflags! {
    /// Event flags captured in `CGEvent` / `CGEventSource` state.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct CGEventFlags: u64 {
        const ALPHA_SHIFT = 0x0001_0000;
        const SHIFT = 0x0002_0000;
        const CONTROL = 0x0004_0000;
        const ALTERNATE = 0x0008_0000;
        const COMMAND = 0x0010_0000;
        const NUMERIC_PAD = 0x0020_0000;
        const HELP = 0x0040_0000;
        const SECONDARY_FN = 0x0080_0000;
        const NON_COALESCED = 0x0000_0100;
    }
}

impl CGEventFlags {
    #[must_use]
    pub const fn raw(self) -> u64 {
        self.bits()
    }
}

/// Backwards-compatible alias used by the pre-v0.5 safe API.
pub use CGEventFlags as ModifierFlags;

/// Bit-union of every known public `CGEventFlags` value.
pub const CG_EVENT_FLAGS_KNOWN_MASK: u64 = CGEventFlags::ALPHA_SHIFT.bits()
    | CGEventFlags::SHIFT.bits()
    | CGEventFlags::CONTROL.bits()
    | CGEventFlags::ALTERNATE.bits()
    | CGEventFlags::COMMAND.bits()
    | CGEventFlags::NUMERIC_PAD.bits()
    | CGEventFlags::HELP.bits()
    | CGEventFlags::SECONDARY_FN.bits()
    | CGEventFlags::NON_COALESCED.bits();
