mod matcher;
mod keyboard;
mod key;
//mod drivers;

fn key_loop() {
    loop {
        //println!("match returned {}", keyhandler::string_matches_key("<c-i>", keyboard::get_key()));
        let key = keyboard::get_key();
        println!("match returned {}", key.matches_pattern("c-i"));
        if key.word == "esc" {
            return;
        }
        println!("{} {} {} {}", key.shift, key.ctrl, key.alt, key.word);
    }
}

fn main() {
    println!("start");
    println!("");

    keyboard::init();
    keyboard::grab_keyboard();
    key_loop();
    keyboard::ungrab_keyboard();
    // println!("{:?}", drivers::get_entries());

    println!("");
    println!("end");
}
