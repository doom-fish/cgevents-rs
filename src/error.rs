//! Errors returned by the `cgevents` crate.

use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CGError {
    /// `CGEventCreate*` returned NULL — typically out-of-memory or invalid arguments.
    EventCreateFailed(String),
    /// `CGEventTapCreate` returned NULL — usually missing Accessibility permission.
    TapCreateFailed,
    /// `CGEventSourceCreate` returned NULL.
    SourceCreateFailed,
    /// Invalid argument passed across the bridge.
    InvalidArgument(String),
    /// A CoreGraphics API returned a non-zero `CGError` code.
    CoreGraphicsError {
        operation: &'static str,
        code: i32,
    },
    /// A Swift bridge precondition failed.
    BridgeError(String),
}

impl fmt::Display for CGError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EventCreateFailed(message) => write!(f, "CGEventCreate failed: {message}"),
            Self::TapCreateFailed => write!(
                f,
                "CGEventTapCreate failed — usually missing Accessibility permission"
            ),
            Self::SourceCreateFailed => write!(f, "CGEventSourceCreate failed"),
            Self::InvalidArgument(message) => write!(f, "invalid argument: {message}"),
            Self::CoreGraphicsError { operation, code } => {
                write!(f, "{operation} failed with CGError code {code}")
            }
            Self::BridgeError(message) => write!(f, "Swift bridge error: {message}"),
        }
    }
}

impl std::error::Error for CGError {}
