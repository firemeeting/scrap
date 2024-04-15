use crate::quartz::display::Display;
use crate::Error;
use core_foundation::data::CFData;
use core_graphics::display::{kCGNullWindowID, kCGWindowImageDefault, kCGWindowListOptionAll};
use core_graphics::window::create_image;
use std::io;
use std::io::ErrorKind;

pub struct Capturer {
    display: Display,
}

impl Capturer {
    pub fn new(display: Display) -> Capturer {
        Capturer { display }
    }

    pub fn width(&self) -> usize {
        self.display.width()
    }
    pub fn height(&self) -> usize {
        self.display.height()
    }

    pub fn frame(&mut self) -> Result<CFData, Error> {
        let cg_image = create_image(
            self.display.bounds(),
            kCGWindowListOptionAll,
            kCGNullWindowID,
            kCGWindowImageDefault,
        )
        .ok_or(io::Error::from(ErrorKind::WouldBlock))?;
        Ok(cg_image.data())
    }
}
