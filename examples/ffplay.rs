extern crate scrap;

use std::io::ErrorKind;

fn main() {
    use scrap::{Capturer, Display};
    use std::io::Write;
    use std::process::{Command, Stdio};

    let d = Display::primary().unwrap();
    let (w, h) = (d.width(), d.height());

    let child = Command::new("ffplay")
        .args([
            "-f",
            "rawvideo",
            "-pixel_format",
            "bgr0",
            "-video_size",
            &format!("{}x{}", w, h),
            "-framerate",
            "60",
            "-",
        ])
        .stdin(Stdio::piped())
        .spawn()
        .expect("This example requires ffplay.");

    let mut capturer = Capturer::new(d).unwrap();
    let mut out = child.stdin.unwrap();

    loop {
        match capturer.frame() {
            Ok(frame) => {
                out.write_all(&frame).unwrap();
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                // Wait for the frame.
            }
            Err(_) => {
                // We're done here.
                break;
            }
        }
    }
}
