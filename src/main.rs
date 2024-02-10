#![feature(ascii_char)]

mod hardware;
mod handler;
mod key;

use handler::{Branch, Action};
use hardware::HardwareServer;
use key::Key;
// mod key;
// mod handler;

// fn key_loop() {
//     loop {
//         //println!("match returned {}", keyhandler::string_matches_key("<c-i>", keyboard::get_key()));
//         let key = keyboard::get_key();
//         println!("match returned {}", key.matches_pattern("c-i"));
//         if key.word == "esc" {
//             return;
//         }
//         println!("{} {} {} {}", key.shift, key.ctrl, key.alt, key.word);
//     }
// }

fn main() {
    println!("start");
    println!("");

    let mut root = Branch::new();
    root.new_shortcut_from_str("dcm", Action::Cmd("notify-send 'Hello World!!!'".to_string()));

    let a = HardwareServer::init("xorg").unwrap();
    a.grab_keyboard().unwrap();
    loop {
        let (k, press) = a.get_next_key().into_key();
        if !press { continue }
        println!("{}", k.to_string());
        if k == Key::from_str("<esc>").unwrap() {
            break;
        }
    }
    a.ungrab_keyboard();
    // println!("{:?}", drivers::get_entries());

    println!("");
    println!("end");
}
