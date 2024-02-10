/// The hardware module turns hardware events into unique representations of the state of the
/// events. The keyboard representations are done through the Key struct which contains the state
/// of all possibly useful parameters during the event.

use anyhow::Result;
use thiserror::Error;
use crate::key::{Modifiers, Key};

mod xorg;

#[derive(Debug, Error)]
pub enum HardwareServerError {
    #[error("jio")]
    Could,
    #[error("report a bug")]
    HowDidWeGetHere,
}

#[derive(Debug)]
pub enum HardwareServer {
    Xorg(xorg::ServerInfo)
}

impl HardwareServer {
    pub fn init(backend: &str) -> Result<Self> {
        if backend == "xorg" {
            return Ok(Self::Xorg(xorg::init()?));
        }

        Err(HardwareServerError::HowDidWeGetHere.into())
    }

    pub fn grab_keyboard(&self) -> Result<()> {
        match self {
            HardwareServer::Xorg(server) => { xorg::grab_keyboard(server)?; }
        }

        Ok(())
    }

    pub fn ungrab_keyboard(&self) -> Result<()> {
        match self {
            HardwareServer::Xorg(server) => { xorg::ungrab_keyboard(&server)?; }
        }

        Ok(())
    }

    pub fn grab_key(&self, key: &str, modifier: Modifiers) -> Result<()> {
        match self {
            HardwareServer::Xorg(server) => { return Ok(xorg::grab_key(&server, key, modifier)?); }
        }
    }

    pub fn ungrab_key(&self, key: &str, modifier: Modifiers) -> Result<()> {
        match self {
            HardwareServer::Xorg(server) => { return Ok(xorg::ungrab_key(&server, key, modifier)?); }
        }
    }

    pub fn get_next_key(&self) -> KeyEv {
        match self {
            HardwareServer::Xorg(server) => { return xorg::get_next_key(&server); },
        }
    }
}

/// Key corresponds to a keypress or release and 
#[derive(Debug)]
pub struct KeyEv {
    pub press: bool, // Press (true) or release (false)
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub super_: bool,
    pub capslock: bool,
    pub numlock: bool,
    pub scrolllock: bool,
    pub key: &'static str,
}

impl Default for KeyEv {
    fn default() -> Self {
        Self {
            press: false, // Press (1) or release (0)
            shift: false,
            ctrl: false,
            alt: false,
            super_: false,
            capslock: false,
            numlock: false,
            scrolllock: false,
            key: ""
        }
    }
}

impl KeyEv {
    pub fn into_key(&self) -> (Key, bool) {
        (Key::from(self), self.press)
    }
}
