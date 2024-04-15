#[cfg(quartz)]
pub mod quartz;

#[cfg(x11)]
pub mod x11;

#[cfg(dxgi)]
extern crate winapi;
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
    #[error("macos: create a null image")]
    CreateNullImage,
}
