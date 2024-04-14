use crate::x11::{Display, Rect, Server};
use std::rc::Rc;
use xcb::randr::{GetMonitors, MonitorInfoBuf};
use xcb::x::{ScreenIterator, Window};

pub struct DisplayIter<'a> {
    outer: ScreenIterator<'a>,
    inner: Option<(Vec<MonitorInfoBuf>, Window)>,
    server: &'a Rc<Server>,
}

impl<'a> DisplayIter<'a> {
    pub fn new(server: &'a Rc<Server>) -> DisplayIter<'a> {
        let mut outer = server.setup().roots();
        let inner = Self::next_screen(&mut outer, server);
        DisplayIter {
            outer,
            inner,
            server,
        }
    }

    fn next_screen(
        outer: &mut ScreenIterator,
        server: &Server,
    ) -> Option<(Vec<MonitorInfoBuf>, Window)> {
        if let Some(screen) = outer.next() {
            let window = screen.root();

            let cookie = server.conn().send_request(&GetMonitors {
                window,
                get_active: true,
            });

            if let Ok(reply) = server.conn().wait_for_reply(cookie) {
                let monitors = reply.monitors().map(|monitor| monitor.to_owned()).collect();
                return Some((monitors, window));
            }
        }
        None
    }
}

impl<'a> Iterator for DisplayIter<'a> {
    type Item = Display;

    fn next(&mut self) -> Option<Display> {
        loop {
            match self.inner {
                Some((ref mut inner, root)) => {
                    // If there is something in the current screen, return that.
                    if let Some(monitor) = inner.pop() {
                        let display = Display::new(
                            self.server.clone(),
                            monitor.primary(),
                            Rect {
                                x: monitor.x(),
                                y: monitor.y(),
                                w: monitor.width(),
                                h: monitor.height(),
                            },
                            root,
                        );
                        return Some(display);
                    }
                }
                None => return None,
            }

            // The current screen was empty, so try the next screen.
            self.inner = Self::next_screen(&mut self.outer, self.server);
        }
    }
}
