#[cfg(quartz)]
pub mod quartz;

#[cfg(x11)]
pub mod x11;

#[cfg(dxgi)]
pub mod dxgi;

mod common;

pub use common::*;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    #[cfg(target_os = "linux")]
    #[error("xcb: {0}")]
    Xcb(#[from] xcb::Error),

    #[cfg(target_os = "macos")]
    #[error("{0}")]
    CgError(&'static str),

    #[cfg(target_os = "macos")]
    #[error("macos: create a null image")]
    CreateNullImage,
}

#[cfg(target_os = "macos")]
impl From<core_graphics::display::CGError> for Error {
    fn from(value: core_graphics::display::CGError) -> Self {
        if value == core_graphics::base::kCGErrorSuccess {
            return Self::CgError("success");
        } else if value == core_graphics::base::kCGErrorFailure {
            return Self::CgError("failure");
        } else if value == core_graphics::base::kCGErrorIllegalArgument {
            return Self::CgError("illegal argument");
        } else if value == core_graphics::base::kCGErrorInvalidConnection {
            return Self::CgError("invalid connection");
        } else if value == core_graphics::base::kCGErrorInvalidContext {
            return Self::CgError("invalid context");
        } else if value == core_graphics::base::kCGErrorCannotComplete {
            return Self::CgError("cannot complete");
        } else if value == core_graphics::base::kCGErrorNotImplemented {
            return Self::CgError("not implemented");
        } else if value == core_graphics::base::kCGErrorRangeCheck {
            return Self::CgError("range check");
        } else if value == core_graphics::base::kCGErrorTypeCheck {
            return Self::CgError("type check");
        } else if value == core_graphics::base::kCGErrorInvalidOperation {
            return Self::CgError("invalid operation");
        } else if value == core_graphics::base::kCGErrorNoneAvailable {
            return Self::CgError("none available");
        }
        Self::CgError("unknown")
    }
}
