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
        match value {
            0 => Self::CgError("success"),
            1000 => Self::CgError("failure"),
            1001 => Self::CgError("illegal argument"),
            1002 => Self::CgError("invalid connection"),
            1003 => Self::CgError("invalid context"),
            1004 => Self::CgError("cannot complete"),
            1006 => Self::CgError("not implemented"),
            1007 => Self::CgError("range check"),
            1008 => Self::CgError("type check"),
            1010 => Self::CgError("invalid operation"),
            1011 => Self::CgError("none available"),
            _ => Self::CgError("unknown"),
        }
    }
}
