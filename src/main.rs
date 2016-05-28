use std::process::Command;

fn main() {
    let output_pacman = Command::new("pacman")
        .arg("-Q")
        .arg("linux")
        .output()
        .expect("Could not execute pacman");

    let output_uname = Command::new("uname")
        .arg("-r")
        .output()
        .expect("Could not execute uname");

    let output_pacman = String::from_utf8_lossy(&output_pacman.stdout);
    let output_uname = String::from_utf8_lossy(&output_uname.stdout);
    print!("installed: {}running: {}", output_pacman, output_uname);
}

