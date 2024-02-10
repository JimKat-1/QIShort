use std::fs;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::error::Error;
use crate::lazy_static::lazy_static;
use super::tree::*;

// Not necessarily the actual directory of the process.
// pub fn cwd(cd: Option<&Path>) -> Result<&'static PathBuf, Box<dyn Error>> {
//     lazy_static! {
//         // TODO: This only works for systems complying to the XDG standard.
//         static ref cwd: PathBuf = match std::env::var("XDG_CONFIG_DIR") {
//             Ok(dir) => dir.into().push("qishort"),
//             Err(_) => match std::env::var("HOME") {
//                 Ok(dir) => dir.push(".config/qishort"),
//                 Err(err) => return Err(err)
//             }
//         };
//     }

//     if let Some(dir) = cd {
//         cwd.push(dir);
//         if !cwd.is_dir() {
//             for i in dir.iter(){
//                 cwd.pop();
//             }
//         }
//     }

//     Ok(&cwd)
// }

pub fn get_dir_entries(dir: &Path) -> Result<Vec<OsString>, Box<dyn Error>> {
    let mut entries = Vec::<String>::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            entries.push(entry?.file_name()
                               .into_string()
                               .or_else(|f| Err(format!("Invalid directory entry: {:?}", f)))?);
        }
    }

    Ok(entries)
}

fn fill_tree_from_dir(dir: PathBuf, tree: &mut Tree) {
    let mut dirent = get_dir_entries(dir)?;

    for i in dirent {
        dir.push(i);

        if dir.is_dir() {
            let mut tree = Tree::new();
            fill_tree_from_dir(dir, tree);
            tree.insert(i, TreeEntry::Tree(tree));
        } else {
            // TODO
            tree.insert(i, TreeEntry::Func(Func {}));
        }

        dir.pop();
    }
}fn get_keys(s: &str) -> Option<Vec<Key>> {
    let mut rest = s;
    let mut ret = Vec::new();

    while rest.len() > 0 {
        let cs;
        (cs, rest) = rest.split_at(1);
        let c = cs.chars().next().unwrap();

        if cs == "<" {
            match get_key_after_lt(s) {
                Some(tmp) => {
                    ret.push(tmp.0);
                    rest = tmp.1;
                },
                None => {
                    let mut key = Key::default();
                    key.shift = true;
                    key.key = ",";

                    ret.push(key);
                }
            }
        } else {
            let mut key = Key::default();

            if let Some(unshi) = unshift(cs) {
                key.key = unshi;
                key.shift = true;

                ret.push(key);
            } else if let Some(_) = shift(cs) {
                key.key = cs;

                ret.push(key);
            } else {
                return None;
            }
        }
    }

    Some(ret)
}

// fn pattern_match_after_lt(s: &str) -> (Option<Key>, &str) {
//     fn unalias(s: &str) -> Option<&str> {
//         Some(ALIASES.iter().find(|a| (*a).contains(&s))?[0])
//     }
//
//     fn get_modifiers() -> {
//
//     }
//
//     let (c, rest) = match s.split_once('>') { Some(a) => a, None => { return (None, s) } };
//     let cdash = c.split('-').collect::<Vec<&str>>();
//     if cdash.len() == 1 {
//         match unalias(c) {
//             Some(tmp) => { return (Some(Key::default().set_key(tmp)), rest) },
//             None => { return (None, s) }
//         }
//     }
//
//     if cdash.last().unwrap().is_empty() && cdash[cdash.len()-2].is_empty() {
//         return (Some(Key { shift: false, ctrl: false, alt: false, super_: false, key: "-" }), rest)
//     }
//
//     let last = sdash.last();
//     if last
// }

// fn word_matches_key_word(word: &str, key: &Key) -> bool {
//     if word.len() == 1 {
//         if word == key.key {
//             return true;
//         } else {
//             return false;
//         }
//     }
//
//     let word_low = word.to_lowercase();
//     if word_low == key.key {
//         return true;
//     }
//
//     for i in ALIASES {
//         if i[0] == key.key {
//             for j in i {
//                     if &word_low == j {
//                     return true;
//                 }
//             }
//             return false;
//         }
//     }
//
//     false
// }
//
// pub fn pattern_to_key(pattern: &str, key: &Key) -> bool {
//     let (mut shift, mut ctrl, mut alt, mut super_) = (false, false, false, false);
//
//
// }
//
// // The key matcher. Matches patterns like <S-A-C-Home> to Key structs (S stands for shift A for alt C for
// // control home for the home key in this example). The <>s are not necessary. Is case insensitive (except
// // for capital letters (<a> != <A> but <s-something> == <S-SoMeTHIng>)).
// pub fn pattern_matches_key(pattern: &str, key: &Key) -> bool {
//     let mut s = pattern.trim();
//     if s.starts_with("<") && s.ends_with(">") {
//         s = &s[1 .. s.len() - 1];
//     }
//
//     let (mut shift, mut ctrl, mut alt) = (false, false, false);
//
//     let mut word = Default::default();
//     let (mut from, mut to) = (0, 0);
//     let mut skip = true; // Helps skip the first char after a dash seperator in case it is a dash itself.
//                          // I.e. makes <c--> be treated as ctrl-dash.
//
//     for c in s.chars() {
//         if c == '-' && !skip {
//             word = &s[from .. to];
//
//             if word == "S" || word == "s" { shift = true; }
//             if word == "C" || word == "c" { ctrl = true; }
//             if word == "A" || word == "a" { alt = true; }
//
//             from = to + 1;
//             skip = true;
//         } else {
//             skip = false;
//         }
//         to += 1;
//     }
//
//     word = &s[from .. to];
//
//     if shift {
//         if !shift(&word) {
//             word_to_shifted(&mut word);
//         }
//     } else {
//         if is_shifted(&word) {
//             shift = true;
//         }
//     }
//
//     let mut key_ = key.clone();
//     if key_.shift {
//         if !is_shifted(&key_.word) {
//             word_to_shifted(&mut key_.word);
//         }
//     }
//
//     if !(key_.shift == shift && key_.ctrl == ctrl && key_.alt == alt) {
//         return false;
//     }
//
//     // println!("String : s: {} c: {} a: {} word: {}", shift, ctrl, alt, word);
//     // println!("Key    : s: {} c: {} a: {} word: {}", key_.shift, key_.ctrl, key_.alt, key_.word);
//     // println!("");
//
//     word_matches_key_word(word, &key_)
// }

