use x11::xlib::*;
use x11::keysym::*;
// use crate::key::*;
use std::convert::TryInto;
use anyhow::Result;
use thiserror::Error;
use super::KeyEv;
use crate::key::Modifiers;

#[derive(Error, Debug)]
enum XorgError {
    #[error("could not open display")]
    CouldNotOpenDisplay,
    #[error("could not grab keyboard")]
    GrabKeyboardFailed,
    #[error("could not ungrab keyboard")]
    UngrabKeyboardFailed,
    #[error("could not grab key {0}")]
    GrabKeyFailed(String),
    #[error("could not ungrab key {0}")]
    UngrabKeyFailed(String),
    #[error("could not ungrab key {0}")]
    KeyStringDoesntMatchToKeysym(String),
}

fn modifiers_to_mask(modifiers: Modifiers) -> u32 {
    let mut ret = 0;

    if modifiers.shift  { ret |= ShiftMask; }
    if modifiers.ctrl   { ret |= ControlMask; }
    if modifiers.alt    { ret |= Mod1Mask; }
    if modifiers.super_ { ret |= Mod4Mask; }

    ret
}

static KEYS: [(u32, &'static str); 88] = [
    ( XK_Num_Lock,    "numlock" ),
    ( XK_Caps_Lock,   "capslock" ),
    ( XK_Scroll_Lock, "scrolllock" ),
    ( XK_Shift_L,     "shift_l" ),
    ( XK_Shift_R,     "shift_r" ),
    ( XK_Control_L,   "ctrl_l" ),
    ( XK_Control_R,   "ctrl_r" ),
    ( XK_Alt_L,       "alt_l" ),
    ( XK_Alt_R,       "alt_r" ),
    ( XK_Super_L,     "super_l" ),
    ( XK_Super_R,     "super_r" ),

    ( XK_a, "a" ),
    ( XK_b, "b" ),
    ( XK_c, "c" ),
    ( XK_d, "d" ),
    ( XK_e, "e" ),
    ( XK_f, "f" ),
    ( XK_g, "g" ),
    ( XK_h, "h" ),
    ( XK_i, "i" ),
    ( XK_j, "j" ),
    ( XK_k, "k" ),
    ( XK_l, "l" ),
    ( XK_m, "m" ),
    ( XK_n, "n" ),
    ( XK_o, "o" ),
    ( XK_p, "p" ),
    ( XK_q, "q" ),
    ( XK_r, "r" ),
    ( XK_s, "s" ),
    ( XK_t, "t" ),
    ( XK_u, "u" ),
    ( XK_v, "v" ),
    ( XK_w, "w" ),
    ( XK_x, "x" ),
    ( XK_y, "y" ),
    ( XK_z, "z" ),

    ( XK_0, "0" ),
    ( XK_1, "1" ),
    ( XK_2, "2" ),
    ( XK_3, "3" ),
    ( XK_4, "4" ),
    ( XK_5, "5" ),
    ( XK_6, "6" ),
    ( XK_7, "7" ),
    ( XK_8, "8" ),
    ( XK_9, "9" ),

    ( XK_equal,         "=" ),
    ( XK_semicolon,     ";" ),
    ( XK_grave,         "`" ),
    ( XK_bracketleft,   "[" ),
    ( XK_bracketright,  "]" ),
    ( XK_minus,         "-" ),
    ( XK_apostrophe,    "'" ),
    ( XK_backslash,     "\\" ),
    ( XK_slash,         "/" ),
    ( XK_comma,         "," ),
    ( XK_period,        "." ),
    ( XK_space,         " " ),

    ( XK_Up,        "up" ),
    ( XK_Down,      "down" ),
    ( XK_Left,      "left" ),
    ( XK_Right,     "right" ),
    ( XK_F1,        "f1" ),
    ( XK_F2,        "f2" ),
    ( XK_F3,        "f3" ),
    ( XK_F4,        "f4" ),
    ( XK_F5,        "f5" ),
    ( XK_F6,        "f6" ),
    ( XK_F7,        "f7" ),
    ( XK_F8,        "f8" ),
    ( XK_F9,        "f9" ),
    ( XK_F10,       "f10" ),
    ( XK_F11,       "f11" ),
    ( XK_F12,       "f12" ),
    ( XK_Page_Up,   "pgup" ),
    ( XK_Page_Down, "pgdn" ),
    ( XK_Home,      "home" ),
    ( XK_End,       "end" ),
    ( XK_BackSpace, "bs" ),
    ( XK_Delete,    "del" ),
    ( XK_Insert,    "ins" ),
    ( XK_Escape,    "esc" ),
    ( XK_Tab,       "tab" ),
    ( XK_Return,    "ret" ),
    ( XK_Print,     "prtscr" ),
    ( XK_Pause,     "pau" ),
    ( XK_Menu,      "menu" ),
];

#[derive(Debug)]
pub struct ServerInfo {
    dpy: *mut Display,
    root: Window
}

impl Default for ServerInfo {
    fn default() -> Self {
        Self {
            dpy: 0 as *mut Display,
            root: 0,
        }
    }
}

pub fn init() -> Result<ServerInfo> {
    let mut ret: ServerInfo = Default::default();
    unsafe {
        ret.dpy = XOpenDisplay(0 as *const i8);
        if ret.dpy == 0 as *mut Display { return Err(XorgError::CouldNotOpenDisplay.into()); }
        ret.root = XDefaultRootWindow(ret.dpy);
    }

    Ok(ret)
}

pub fn grab_key(server: &ServerInfo, key: &str, modifier: Modifiers) -> Result<()> {
    let mut keysym = 0;
    let mut found = false;
    for k in KEYS {
        if key == k.1 {
            keysym = k.0;
            found = true;
            break;
        }
    }

    if !found {
        return Err(XorgError::KeyStringDoesntMatchToKeysym(key.to_string()).into());
    }

    unsafe {
        XSelectInput(server.dpy, server.root, KeyPressMask | KeyReleaseMask);

        let keycode = XKeysymToKeycode(server.dpy, keysym.into());
        if XGrabKey(server.dpy, keycode.into(), modifiers_to_mask(modifier), server.root, 1, GrabModeAsync, GrabModeAsync) != 0 {
            return Err(XorgError::GrabKeyFailed(key.to_string()).into());
        }
    }

    Ok(())
}

pub fn ungrab_key(server: &ServerInfo, key: &str, modifier: Modifiers) -> Result<()> {
    let mut keysym = 0;
    let mut found = false;
    for k in KEYS {
        if key == k.1 {
            keysym = k.0;
            found = true;
            break;
        }
    }

    if !found {
        return Err(XorgError::KeyStringDoesntMatchToKeysym(key.to_string()).into());
    }

    unsafe {
        XSelectInput(server.dpy, server.root, KeyPressMask | KeyReleaseMask);

        let keycode = XKeysymToKeycode(server.dpy, keysym.into());
        if XUngrabKey(server.dpy, keycode.into(), modifiers_to_mask(modifier), server.root) != 0 {
            return Err(XorgError::UngrabKeyFailed(key.to_string()).into());
        }
    }

    Ok(())
}

pub fn grab_keyboard(server: &ServerInfo) -> Result<()> {
    unsafe {
        XSelectInput(server.dpy, server.root, KeyPressMask | KeyReleaseMask);

        if XGrabKeyboard(server.dpy, server.root, True, GrabModeAsync, GrabModeAsync, CurrentTime) != 0 {
            return Err(XorgError::GrabKeyboardFailed.into());
        }
    }

    Ok(())
}

pub fn ungrab_keyboard(server: &ServerInfo) -> Result<()> {
    unsafe {
        if XUngrabKeyboard(server.dpy, CurrentTime) != 0 {
            return Err(XorgError::UngrabKeyboardFailed.into());
        }
    }

    Ok(())
}

pub fn get_next_key(server: &ServerInfo) -> KeyEv {
    let mut ret: KeyEv = Default::default();

    unsafe {
        let mut ev = std::mem::MaybeUninit::<XEvent>::uninit();
        XNextEvent(server.dpy, ev.as_mut_ptr());
        let ev = ev.assume_init_mut();

        ret.ctrl = (ev.key.state & ControlMask) != 0;
        ret.shift = (ev.key.state & ShiftMask) != 0;
        ret.alt = (ev.key.state & Mod1Mask) != 0;
        ret.super_ = (ev.key.state & Mod4Mask) != 0;
        ret.capslock = (ev.key.state & LockMask) != 0;
        ret.numlock = (ev.key.state & Mod2Mask) != 0;
        ret.scrolllock = (ev.key.state & Mod5Mask) != 0;

        let keysym = XkbKeycodeToKeysym(server.dpy, ev.key.keycode.try_into().unwrap(), 0, 0);
        for key in KEYS {
            if key.0 as u64 == keysym {
                ret.key = key.1;
            }
        }

        if ev.key.type_ == KeyPress {
            ret.press = true;
        } else if ev.key.type_ == KeyRelease {
            ret.press = false;
        }
    }

    return ret;
}
