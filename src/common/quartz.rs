use crate::{quartz, Error};
use std::{io, ops};

pub struct Capturer(quartz::Capturer);

impl Capturer {
    pub fn new(display: Display) -> io::Result<Capturer> {
        let inner = quartz::Capturer::new(display.0);

        Ok(Capturer(inner))
    }

    pub fn width(&self) -> usize {
        self.0.width()
    }

    pub fn height(&self) -> usize {
        self.0.height()
    }

    pub fn frame(&mut self) -> Result<Frame, Error> {
        Ok(Frame(self.0.frame()?))
    }
}

pub struct Frame(Vec<u8>);

impl ops::Deref for Frame {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.0
    }
}

pub struct Display(quartz::Display);

impl Display {
    pub fn primary() -> Result<Display, Error> {
        Ok(Display(quartz::Display::primary()?))
    }

    pub fn all() -> io::Result<Vec<Display>> {
        Ok(quartz::Display::online()
            .map_err(|_| io::Error::from(io::ErrorKind::Other))?
            .into_iter()
            .map(Display)
            .collect())
    }

    pub fn width(&self) -> usize {
        self.0.width()
    }

    pub fn height(&self) -> usize {
        self.0.height()
    }
}
