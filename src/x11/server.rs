use crate::x11::DisplayIter;
use std::rc::Rc;
use xcb::x::Setup;
use xcb::{ConnError, Connection};

pub struct Server {
    conn: Connection,
    screen: i32,
}

impl Server {
    pub fn displays(server: &Rc<Server>) -> DisplayIter {
        DisplayIter::new(server)
    }

    pub fn new() -> Result<Server, Error> {
        let (conn, screen) = Connection::connect(None)?;
        Ok(Server { conn, screen })
    }

    pub fn conn(&self) -> &Connection {
        &self.conn
    }
    pub fn screen(&self) -> i32 {
        self.screen
    }
    pub fn setup(&self) -> &Setup {
        self.conn.get_setup()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Xcb connection error: {0}")]
    Conn(#[from] ConnError),
}
