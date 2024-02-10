use std::process::Command;

pub static fn_str: [(&str, fn(&str)); 2] = [
    ("sh", &sh),
    ("exec", &exec),
];

pub fn sh(cmd: &str) -> String {
    let mut iter = cmd.lines();

    let command = Command::new("sh");

    command.arg(cmd);

    command.spawn();
}
