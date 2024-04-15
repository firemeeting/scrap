use crate::Error;
use core_graphics::display::{CGDirectDisplayID, CGDisplay, CGError, CGMainDisplayID, CGRect};

#[repr(C)]
pub struct Display {
    display: CGDisplay,
    rect: Rect,
    scale_factor: Option<f32>,
    is_primary: bool,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

impl Display {
    fn new(id: CGDirectDisplayID) -> Result<Display, CGError> {
        let display = CGDisplay::new(id);
        display.show_cursor()?;

        let bounds = display.bounds();

        let mut scale_factor = None;
        if let Some(mode) = display.display_mode() {
            let pixel_width = mode.pixel_width();
            scale_factor = Some(pixel_width as f32 / bounds.size.width as f32);
        }

        Ok(Display {
            display,
            scale_factor,
            is_primary: display.is_main(),
            rect: Rect {
                x: bounds.origin.x as i32,
                y: bounds.origin.y as i32,
                w: bounds.size.width as u32,
                h: bounds.size.height as u32,
            },
        })
    }

    pub fn primary() -> Result<Display, Error> {
        let id = unsafe { CGMainDisplayID() };
        Ok(Display::new(id)?)
    }

    pub fn online() -> Result<Vec<Display>, CGError> {
        let displays = CGDisplay::active_displays()?;

        let mut online = Vec::with_capacity(displays.len());
        for display in displays {
            online.push(Display::new(display)?);
        }
        Ok(online)
    }

    pub fn width(&self) -> usize {
        match self.scale_factor {
            Some(factor) => self.rect.w as usize * factor as usize,
            None => self.rect.w as usize,
        }
    }

    pub fn height(&self) -> usize {
        match self.scale_factor {
            Some(factor) => self.rect.h as usize * factor as usize,
            None => self.rect.h as usize,
        }
    }

    pub fn bounds(&self) -> CGRect {
        self.display.bounds()
    }
}
