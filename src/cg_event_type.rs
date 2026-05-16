//! `CGEventType` values and event-mask helpers.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CGEventType {
    Null,
    LeftMouseDown,
    LeftMouseUp,
    RightMouseDown,
    RightMouseUp,
    MouseMoved,
    LeftMouseDragged,
    RightMouseDragged,
    KeyDown,
    KeyUp,
    FlagsChanged,
    ScrollWheel,
    TabletPointer,
    TabletProximity,
    OtherMouseDown,
    OtherMouseUp,
    OtherMouseDragged,
    TapDisabledByTimeout,
    TapDisabledByUserInput,
}

impl CGEventType {
    #[must_use]
    pub const fn raw(self) -> u32 {
        match self {
            Self::Null => 0,
            Self::LeftMouseDown => 1,
            Self::LeftMouseUp => 2,
            Self::RightMouseDown => 3,
            Self::RightMouseUp => 4,
            Self::MouseMoved => 5,
            Self::LeftMouseDragged => 6,
            Self::RightMouseDragged => 7,
            Self::KeyDown => 10,
            Self::KeyUp => 11,
            Self::FlagsChanged => 12,
            Self::ScrollWheel => 22,
            Self::TabletPointer => 23,
            Self::TabletProximity => 24,
            Self::OtherMouseDown => 25,
            Self::OtherMouseUp => 26,
            Self::OtherMouseDragged => 27,
            Self::TapDisabledByTimeout => 0xFFFF_FFFE,
            Self::TapDisabledByUserInput => 0xFFFF_FFFF,
        }
    }

    #[must_use]
    pub const fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            0 => Some(Self::Null),
            1 => Some(Self::LeftMouseDown),
            2 => Some(Self::LeftMouseUp),
            3 => Some(Self::RightMouseDown),
            4 => Some(Self::RightMouseUp),
            5 => Some(Self::MouseMoved),
            6 => Some(Self::LeftMouseDragged),
            7 => Some(Self::RightMouseDragged),
            10 => Some(Self::KeyDown),
            11 => Some(Self::KeyUp),
            12 => Some(Self::FlagsChanged),
            22 => Some(Self::ScrollWheel),
            23 => Some(Self::TabletPointer),
            24 => Some(Self::TabletProximity),
            25 => Some(Self::OtherMouseDown),
            26 => Some(Self::OtherMouseUp),
            27 => Some(Self::OtherMouseDragged),
            0xFFFF_FFFE => Some(Self::TapDisabledByTimeout),
            0xFFFF_FFFF => Some(Self::TapDisabledByUserInput),
            _ => None,
        }
    }

    #[must_use]
    pub const fn mask_bit(self) -> u64 {
        let raw = self.raw();
        if raw < 64 {
            1_u64 << raw
        } else {
            0
        }
    }
}

/// Equivalent of `kCGAnyInputEventType`.
pub const CG_ANY_INPUT_EVENT_TYPE: u32 = u32::MAX;
/// Equivalent of `kCGEventMaskForAllEvents`.
pub const CG_EVENT_MASK_FOR_ALL_EVENTS: u64 = u64::MAX;
