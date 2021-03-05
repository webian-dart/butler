use std::process::Command;
use crate::paths::Paths;
use std::borrow::Borrow;
use std::ops::Deref;
use std::env;

pub(crate) fn pwd() {
    let path = env::current_dir().expect("Failed to get current dir");
    println!("pwd: {}", path.display());
}

pub(crate) fn ls() {
    let mut list_dir = Command::new("ls");
    // Execute `ls` in the current directory of the program.
    list_dir.status().expect("process failed to execute");

    println!();

    // Change `ls` to execute in the root directory.
    let path = Paths::normalize_for_os(&String::from("../"));
    list_dir.current_dir(path);

    // And then execute `ls` again but in the root directory.
    list_dir.status().expect("process failed to execute");
}
