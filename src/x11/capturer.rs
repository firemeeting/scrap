use crate::x11::Display;
use std::{io, ptr, slice};
use xcb::shm::{Attach, Detach, GetImage, Seg};
use xcb::x::{Drawable, ImageFormat};

pub struct Capturer {
    display: Display,
    shmid: i32,
    xcbid: Seg,

    buffer: *const u8,
    size: usize,
}

impl Capturer {
    pub fn new(display: Display) -> io::Result<Capturer> {
        // Calculate dimensions.
        let pixel_width = 4;
        let rect = display.rect();
        let size = (rect.w as usize) * (rect.h as usize) * pixel_width;

        // Create a shared memory segment.
        let shmid = unsafe {
            libc::shmget(
                libc::IPC_PRIVATE,
                size * 2,
                // Everyone can do anything.
                libc::IPC_CREAT | 0o777,
            )
        };

        if shmid == -1 {
            return Err(io::Error::last_os_error());
        }

        // Attach the segment to a readable address.
        let buffer = unsafe { libc::shmat(shmid, ptr::null(), libc::SHM_RDONLY) } as *mut u8;

        if buffer as isize == -1 {
            return Err(io::Error::last_os_error());
        }

        // Attach the segment to XCB.
        let server = display.server().conn();
        let xcbid: Seg = server.generate_id();

        let _ = server.send_request(&Attach {
            shmid: shmid as u32,
            shmseg: xcbid,
            read_only: false,
        });

        Ok(Capturer {
            display,
            shmid,
            xcbid,
            buffer,
            size,
        })
    }

    pub fn display(&self) -> &Display {
        &self.display
    }

    pub fn frame(&mut self) -> &[u8] {
        let rect = self.display.rect();
        let cookie = self.display.server().conn().send_request(&GetImage {
            drawable: Drawable::Window(self.display.root()),
            x: rect.x,
            y: rect.y,
            width: rect.w,
            height: rect.h,
            shmseg: self.xcbid,
            plane_mask: !0,
            offset: 0,
            format: ImageFormat::ZPixmap as u8,
        });

        let _ = self.display.server().conn().wait_for_reply(cookie).unwrap();
        unsafe { slice::from_raw_parts(self.buffer, self.size) }
    }
}

impl Drop for Capturer {
    fn drop(&mut self) {
        // Detach segment from XCB.
        let _ = self
            .display
            .server()
            .conn()
            .send_request(&Detach { shmseg: self.xcbid });

        unsafe {
            // Detach segment from our space.
            libc::shmdt(self.buffer as *mut _);
            // Destroy the shared memory segment.
            libc::shmctl(self.shmid, libc::IPC_RMID, ptr::null_mut());
        }
    }
}
