use crate::quartz::display::Display;
use crate::Error;
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

    pub fn frame(&mut self) -> Result<Vec<u8>, Error> {
        let cg_image = create_image(
            self.display.bounds(),
            kCGWindowListOptionAll,
            kCGNullWindowID,
            kCGWindowImageDefault,
        )
        .ok_or(io::Error::from(ErrorKind::WouldBlock))?;

        let width = cg_image.width();
        let height = cg_image.height();
        let bytes = Vec::from(cg_image.data().bytes());

        let mut buffer = Vec::with_capacity(width * height * 4);
        for row in bytes.chunks_exact(cg_image.bytes_per_row()) {
            buffer.extend_from_slice(&row[..width * 4]);
        }

        Ok(buffer)
    }
}
