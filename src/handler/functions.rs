use std::process::Command;

pub static fn_str: [(&str, fn(&str)); 2] = [
    ("sh", &sh),
    ("exec", &exec),
];

pub fn sh(cmd: &str) {
    let mut iter = cmd.lines();

    let command = Command::new("sh");

    command.arg(cmd);

    command.spawn();
}

// Separate arguments with empty line
pub fn exec(args: &str) {
    let mut iter = cmd.lines();

    let cmd = Command::new(iter.next());

    for i in iter {
        cmd.arg(i);
    }

    cmd.spawn();
}
