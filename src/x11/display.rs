use crate::x11::Server;
use std::rc::Rc;
use xcb::x::Window;

pub struct Display {
    server: Rc<Server>,
    default: bool,
    rect: Rect,
    root: Window,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Rect {
    pub x: i16,
    pub y: i16,
    pub w: u16,
    pub h: u16,
}

impl Display {
    pub fn new(server: Rc<Server>, default: bool, rect: Rect, root: Window) -> Display {
        Display {
            server,
            default,
            rect,
            root,
        }
    }

    pub fn server(&self) -> &Rc<Server> {
        &self.server
    }
    pub fn is_default(&self) -> bool {
        self.default
    }
    pub fn rect(&self) -> Rect {
        self.rect
    }
    pub fn root(&self) -> Window {
        self.root
    }
}
