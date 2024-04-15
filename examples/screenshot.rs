extern crate repng;
extern crate scrap;

use scrap::{Capturer, Display, Error};
use std::fs::File;
use std::io::ErrorKind;
use std::time::Duration;

fn main() {
    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;

    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    let (w, h) = (capturer.width(), capturer.height());

    loop {
        // Wait until there's a frame.

        let frame = match capturer.frame() {
            Ok(buffer) => buffer,
            Err(Error::Io(error)) => {
                if error.kind() == ErrorKind::WouldBlock {
                    // Keep spinning.
                    std::thread::sleep(one_frame);
                    continue;
                } else {
                    panic!("Error: {}", error);
                }
            }
            Err(error) => {
                panic!("Error: {}", error);
            }
        };

        println!("Captured! Saving...");

        // Flip the ARGB image into a BGRA image.

        let mut bitflipped = Vec::with_capacity(w * h * 4);
        let stride = frame.len() / h;
        for y in 0..h {
            for x in 0..w {
                let i = stride * y + 4 * x;
                bitflipped.extend_from_slice(&[frame[i + 2], frame[i + 1], frame[i], 255]);
            }
        }

        // Save the image.
        let file = File::create("screenshot.png").unwrap();
        repng::encode(file, w as u32, h as u32, &bitflipped).unwrap();

        println!("Image saved to `screenshot.png`.");
        break;
    }
}
