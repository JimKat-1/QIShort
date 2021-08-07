extern crate x11;
use x11::xlib::*;
use x11::keysym::*;
use crate::key::*;

static SHIFT: u32 = ShiftMask;
static CONTROL: u32 = ControlMask;
static ALT: u32 = Mod1Mask;

static KEYS: [(u32, &'static str); 75] = [
    ( XK_a,         "a" ),
    ( XK_b,         "b" ),
    ( XK_c,         "c" ),
    ( XK_d,         "d" ),
    ( XK_e,         "e" ),
    ( XK_f,         "f" ),
    ( XK_g,         "g" ),
    ( XK_h,         "h" ),
    ( XK_i,         "i" ),
    ( XK_j,         "j" ),
    ( XK_k,         "k" ),
    ( XK_l,         "l" ),
    ( XK_m,         "m" ),
    ( XK_n,         "n" ),
    ( XK_o,         "o" ),
    ( XK_p,         "p" ),
    ( XK_q,         "q" ),
    ( XK_r,         "r" ),
    ( XK_s,         "s" ),
    ( XK_t,         "t" ),
    ( XK_u,         "u" ),
    ( XK_v,         "v" ),
    ( XK_w,         "w" ),
    ( XK_x,         "x" ),
    ( XK_y,         "y" ),
    ( XK_z,         "z" ),

    ( XK_0,	  "0" ),
    ( XK_1,	  "1" ),
    ( XK_2,	  "2" ),
    ( XK_3,	  "3" ),
    ( XK_4,	  "4" ),
    ( XK_5,	  "5" ),
    ( XK_6,	  "6" ),
    ( XK_7,	  "7" ),
    ( XK_8,	  "8" ),
    ( XK_9,	  "9" ),

    ( XK_equal,         "=" ),
    ( XK_semicolon,     ";" ),
    ( XK_grave,         "`" ),
    ( XK_bracketleft,   "[" ),
    ( XK_bracketright,  "]" ),
    ( XK_minus,         "-" ),
    ( XK_apostrophe,    "'" ),
    ( XK_minus,         "\\" ),
    ( XK_slash,         "/" ),
    ( XK_comma,         "," ),
    ( XK_period,        "." ),
    ( XK_space,         " " ),

    ( XK_Up,            "up" ),
    ( XK_Down,          "down" ),
    ( XK_Left,          "left" ),
    ( XK_Right,         "right" ),
    ( XK_F1,            "f1" ),
    ( XK_F2,            "f2" ),
    ( XK_F3,            "f3" ),
    ( XK_F4,            "f4" ),
    ( XK_F5,            "f5" ),
    ( XK_F6,            "f6" ),
    ( XK_F7,            "f7" ),
    ( XK_F8,            "f8" ),
    ( XK_F9,            "f9" ),
    ( XK_F10,           "f10" ),
    ( XK_F11,           "f11" ),
    ( XK_F12,           "f12" ),
    ( XK_Page_Up,       "pgup" ),
    ( XK_Page_Down,     "pgdn" ),
    ( XK_Home,          "home" ),
    ( XK_End,           "end" ),
    ( XK_BackSpace,     "bs" ),
    ( XK_Delete,        "del" ),
    ( XK_Insert,        "ins" ),
    ( XK_Escape,        "esc" ),
    ( XK_Tab,           "tab" ),
    ( XK_Return,        "ret" ),
    ( XK_Print,         "prtscr" ),
];

static mut DPY: *mut Display = 0 as *mut Display;
static mut ROOT: Window = 0 as Window;

pub fn init() {
    unsafe {
        DPY = XOpenDisplay(0 as *const i8);
        ROOT = XDefaultRootWindow(DPY);

        XSelectInput(DPY, ROOT, KeyPressMask);
    }
}

pub fn grab_keyboard() {
    unsafe {
        XGrabKeyboard(DPY, ROOT, True, GrabModeAsync, GrabModeAsync, CurrentTime);
    }
}

pub fn ungrab_keyboard() {
    unsafe {
        XUngrabKeyboard(DPY, CurrentTime);
    }
}

pub fn get_key() -> Key {
    loop {
        match event_to_key(wait_get_event()) {
            Some(key) => return key,
            None => {},
        }
    }
}

fn wait_get_event() -> XEvent {
    let mut xev: XEvent = XEvent { type_: 0 };

    unsafe { XNextEvent(DPY, &mut xev); }

    xev
}

fn event_to_key(xev: XEvent) -> Option<Key> {
    let mut key: Key = Key::default();

    let kev: &XKeyEvent = unsafe { &xev.key };

    unsafe {
        if xev.key.type_ != KeyPress {
            return None ;
        }
    }

    let keysym = unsafe { XKeycodeToKeysym(DPY, kev.keycode as KeyCode, 0) };

    key.ctrl = (kev.state & CONTROL) != 0;
    key.shift = (kev.state & SHIFT) != 0;
    key.alt = (kev.state & ALT) != 0;

    for i in KEYS {
        if keysym == i.0.into() {
            key.word = i.1;
            return Some(key);
        }
    }

    None
}
