use regex::Regex;

use crate::hardware::KeyEv;

// static KEYS: [&str; 88] = [ "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "numlock", "capslock", "scrolllock", "shift_l", "shift_r", "ctrl_l", "ctrl_r", "alt_l", "alt_r", "super_l", "super_r", "=", ";", "`", "[", "]", "-", "'", "\\", "/", ",", ".", " ", "up", "down", "left", "right", "f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8", "f9", "f10", "f11", "f12", "pgup", "pgdn", "home", "end", "bs", "del", "ins", "esc", "tab", "ret", "prtscr", "pau", "menu" ];

/// Aliases for key names. The first one is the default and all comparisons between keys happen
/// with the first one. DON'T CHANGE IT!!!
static ALIASES: [&[&str]; 39] = [
    &["!", "exclamation"],
    &["@", "at"],
    &["#", "hash"],
    &["$", "dollar"],
    &["%", "percent"],
    &["^", "carat"],
    &["&", "and", "ampersand"],
    &["*", "asterisk", "star"],
    &["(", "parenth_l"],
    &[")", "parenth_r"],
    &[";", "semicolon"],
    &[":", "colon"],
    &["`", "grave"],
    &["~", "tilde"],
    &["[", "square_l"],
    &["{", "curly_l"],
    &["]", "square_r"],
    &["}", "curly_r"],
    &["-", "minus", "dash"],
    &["_", "underscore"],
    &["'", "singlequote", "apostrophe"],
    &["\"", "doublequote"],
    &["\\", "backslash"],
    &["|", "bar", "pipe"],
    &["/", "slash"],
    &["?", "question"],
    &[",", "comma"],
    &["<", "less", "lt"],
    &[".", "dot"],
    &[">", "greater", "gt"],
    &[" ", "space"],
    &["pgup", "pageup"],
    &["pgdn", "pagedown"],
    &["bs", "backspace", "bsp"],
    &["del", "delete"],
    &["ins", "insert"],
    &["esc", "escape"],
    &["ret", "return", "enter"],
    &["prtscr", "printscreen", "print"],
];

static SHIFT_ARR: ([&str; 46], [&str; 46]) = (
    [ "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
        ";", "`", "[", "]", "-", "'", "\\", "/", ",", "." ],
    [ "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z", ")", "!", "@", "#", "$", "%", "^", "&", "*", "(",
        ":", "~", "{", "}", "_", "\"", "|", "?", "<", ">" ]
);

fn unshift(s: &str) -> Option<&'static str> {
    // if s.is_ascii_uppercase() { return Some(ch.to_ascii_lowercase()) }
    Some(SHIFT_ARR.0[SHIFT_ARR.1.iter().position(|&r| r == s)?])
}

fn shift(s: &str) -> Option<&'static str> {
    // if ch.is_ascii_lowercase() { return Some(ch.to_ascii_uppercase()) }
    Some(SHIFT_ARR.1[SHIFT_ARR.0.iter().position(|&r| r == s)?])
}

fn unalias(s: &str) -> Option<&'static str> {
    Some(ALIASES.iter().find(|a| (*a).contains(&s))?[0])
}

/// This function takes the string after the first '<' has been encountered (without the '<') and
/// returns a `Some(key)` if found of `None` if no matching '>' was found. The returned &str
/// is the rest of the str after what has been checked.
///
/// NOTE: if the string has an '>' in it that doesn't necessarily mean that there is a match. For
/// example wijofewfijoe> is not a match and the character '>' will be treated literally
fn get_key_after_lt(s: &str) -> Option<(Key, &str)> {
    let regex = Regex::new(r#"^(?:([scawSCAW])-)?(?:([scawSCAW])-)?(?:([scawSCAW])-)?(?:([scawSCAW])-)?(\w+|>)>"#).unwrap();

    let cap = regex.captures(s)?;
    let mut key = Key::default();

    for i in 1..=4 {
        if cap.get(i).is_none() { continue }
        match cap.get(i).unwrap().as_str() {
            "s" | "S" => { key.shift = true },
            "c" | "C" => { key.ctrl = true },
            "a" | "A" => { key.alt = true },
            "w" | "W" => { key.super_ = true },
            _ => { unreachable!(); }
        }
    }

    let k = cap.get(5).unwrap().as_str();
    if let Some(tmp) = unshift(k) {
        key.shift = true;
        key.key = tmp;
    } else {
        let a = k.to_ascii_lowercase();
        key.key = unalias(a.as_str())?;
    }

    let rest = unsafe { std::str::from_utf8_unchecked(&s.as_bytes()[cap.get(0).unwrap().len()..]) };

    Some((key, rest))
}

// Takes a str gives the next key it finds and the rest of the str. If it finds no key (which it
// shouldn't unless the str is empty) it returns None 
fn get_next_key(s: &str) -> Option<(Key, &str)> {
    let mut key = Key::default();
    if s.len() == 0 { return None }

    let (cs, rest) = s.split_at(1);

    if cs == "<" {
        match get_key_after_lt(rest) {
            Some(tmp) => {
                return Some(tmp);
            },
            None => {}
        }
    }

    if let Some(unshi) = unshift(cs) {
        key.key = unshi;
        key.shift = true;
    } else if let Some(_) = shift(cs) {
        key.key = SHIFT_ARR.0.iter().find(|a| *a == &cs).unwrap();
    } else {
        return None;
    }

    Some((key, rest))
}

pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub super_: bool,
}

impl Default for Modifiers {
    fn default() -> Self {
        Self {
            shift: false,
            ctrl: false,
            alt: false,
            super_: false,
        }
    }
}

impl Modifiers {
    pub fn new(s: &str) -> Self {
        Self {
            shift: s.contains("s") | s.contains("S"),
            ctrl: s.contains("c") | s.contains("C"),
            alt: s.contains("a") | s.contains("A"),
            super_: s.contains("w") | s.contains("W"),
        }
    }
}

/// Key represents an actual keypress (with modifier state).
///
/// NOTE: Valid keys are unique and thus the chosen representation for shifted keys is with `shift`
/// set to `true` and the key to it's unshifted value. For example `@` is represented as
/// `Key { key: "2", shift: true, .. }` and that is the only valid way to represent it.
#[derive(Clone, Default, PartialEq, Eq, Debug, Copy, Hash)]
pub struct Key {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub super_: bool,
    pub key: &'static str,
}

impl Key {
    pub fn from_str_many(s: &str) -> Option<Vec<Self>> {
        let mut keys = Vec::new();
        while let Some((key, s)) = get_next_key(s) {
            keys.push(key);
        }

        Some(keys)
    }

    pub fn from_str(s: &str) -> Option<Self> {
        let (key, rest) = get_next_key(s)?;
        if rest != "" { return None }

        Some(key)
    }

    pub fn set_key(&mut self, key: &'static str) {
        self.key = key;
    }

    pub fn set_modifiers(&mut self, modifiers: Modifiers) {
        self.shift = modifiers.shift;
        self.ctrl = modifiers.ctrl;
        self.alt = modifiers.alt;
        self.super_ = modifiers.super_;
    }

    pub fn to_string(&self) -> String {
        return Self::keys_to_string(&[*self]);
    }

    fn keys_to_string(value: &[Key]) -> String {
        let mut ret = String::new();

        for k in value {
            let mut moded = false;
            if k.ctrl || k.alt || k.super_ || k.shift {
                ret.push_str("<");
                if k.super_ { ret.push_str("W-"); }
                if k.alt { ret.push_str("A-"); }
                if k.ctrl { ret.push_str("C-"); }
                if k.shift { ret.push_str("S-"); }
                moded = true;
            }

            if let Some(s) = unalias(k.key) {
                ret.push_str(s);
            } else {
                ret.push_str(k.key);
            };

            if moded { ret.push_str(">"); }
        }

        ret
    }

    // pub fn matches_pattern(&self, pattern: &str) -> bool {
    //     matcher::pattern_matches_key(pattern, &self)
    // }
}

impl From<&KeyEv> for Key {
    fn from(value: &KeyEv) -> Key {
        Key {
            shift: value.shift,
            ctrl: value.ctrl,
            alt: value.alt,
            super_: value.super_,
            key: value.key
        }
    }
}
