use crate::key::Key;

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
    [ "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o",
    "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "0", "1", "2", "3", "4",
    "5", "6", "7", "8", "9", ";", "`", "[", "]", "-", "'", "\\", "/", ",", "." ],
    [ "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O",
    "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", ")", "!", "@", "#", "$",
    "%", "^", "&", "*", "(", ":", "~", "{", "}", "_", "\"", "|", "?", "<", ">" ]
);

fn is_shifted(word: &&str) -> bool {
    if (*word).len() != 1 { return false; };
    for i in SHIFT_ARR.1 {
        if i == (*word) {
            return true;
        }
    }

    false
}

fn word_to_shifted(word: &mut &str) {
    let mut index = 0;
    for i in SHIFT_ARR.0 {
        if &i == word {
            *word = SHIFT_ARR.1[index];
            return;
        }
        index += 1;
    }
}

fn word_matches_key_word(word: &str, key: &Key) -> bool {
    if word.len() == 1 {
        if word == key.word {
            return true;
        } else {
            return false;
        }
    }

    let word_low = word.to_lowercase();
    if word_low == key.word {
        return true;
    }

    for i in ALIASES {
        if i[0] == key.word {
            for j in i {
                    if &word_low == j {
                    return true;
                }
            }
            return false;
        }
    }

    false
}

// The key matcher. Matches patterns like <S-A-C-Home> to Key structs (S stands for shift A for alt C for
// control home for the home key in this example). The <>s are not necessary. Is case insensitive (except
// for capital letters (<a> != <A> but <s-something> == <S-SoMeTHIng>)).
pub fn pattern_matches_key(pattern: &str, key: &Key) -> bool {
    let mut s = pattern.trim();
    if s.starts_with("<") && s.ends_with(">") {
        s = &s[1 .. s.len() - 1];
    }

    let (mut shift, mut ctrl, mut alt) = (false, false, false);

    let mut word = Default::default();
    let (mut from, mut to) = (0, 0);
    let mut skip = true; // Helps skip the first char after a dash seperator in case it is a dash itself.
                         // I.e. makes <c--> be treated as ctrl-dash.

    for c in s.chars() {
        if c == '-' && !skip {
            word = &s[from .. to];

            if word == "S" || word == "s" { shift = true; }
            if word == "C" || word == "c" { ctrl = true; }
            if word == "A" || word == "a" { alt = true; }

            from = to + 1;
            skip = true;
        } else {
            skip = false;
        }
        to += 1;
    }

    word = &s[from .. to];

    if shift {
        if !is_shifted(&word) {
            word_to_shifted(&mut word);
        }
    } else {
        if is_shifted(&word) {
            shift = true;
        }
    }

    let mut key_ = key.clone();
    if key_.shift {
        if !is_shifted(&key_.word) {
            word_to_shifted(&mut key_.word);
        }
    }

    if !(key_.shift == shift && key_.ctrl == ctrl && key_.alt == alt) {
        return false;
    }

    // println!("String : s: {} c: {} a: {} word: {}", shift, ctrl, alt, word);
    // println!("Key    : s: {} c: {} a: {} word: {}", key_.shift, key_.ctrl, key_.alt, key_.word);
    // println!("");

    word_matches_key_word(word, &key_)
}
