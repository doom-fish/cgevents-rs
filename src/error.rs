//! Errors returned by the `cgevents` crate.

use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CGError {
    /// `CGEventCreate*` returned NULL — typically out-of-memory or
    /// invalid arguments.
    EventCreateFailed(String),
    /// `CGEventTapCreate` returned NULL — usually missing Accessibility
    /// permission. Open System Settings → Privacy & Security →
    /// Accessibility and grant your binary.
    TapCreateFailed,
    /// `CGEventSourceCreate` returned NULL.
    SourceCreateFailed,
    InvalidArgument(String),
}

impl fmt::Display for CGError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EventCreateFailed(m) => write!(f, "CGEventCreate failed: {m}"),
            Self::TapCreateFailed => write!(
                f,
                "CGEventTapCreate failed — usually missing Accessibility permission"
            ),
            Self::SourceCreateFailed => write!(f, "CGEventSourceCreate failed"),
            Self::InvalidArgument(m) => write!(f, "invalid argument: {m}"),
        }
    }
}

impl std::error::Error for CGError {}
