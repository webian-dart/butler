use std::process::Command;

pub(crate) fn pwd() {
    let mut command = Command::new("pwd");
    let output = command.output().expect("failed to execute process");
    print!("pwd: {}", String::from_utf8_lossy(&output.stdout));
}

pub(crate) fn ls() {
    let mut list_dir = Command::new("ls");
    // Execute `ls` in the current directory of the program.
    list_dir.status().expect("process failed to execute");

    println!();

    // Change `ls` to execute in the root directory.
    list_dir.current_dir("../");

    // And then execute `ls` again but in the root directory.
    list_dir.status().expect("process failed to execute");
}
