use std::process::Command;

fn main() {
    let output_pacman = Command::new("pacman")
        .arg("-Q")
        .arg("linux")
        .output()
        .expect("Could not execute pacman");
    // pacman output is in the form "linux version"
    let output_pacman = String::from_utf8_lossy(&output_pacman.stdout);
    let output_pacman = output_pacman.split_whitespace()
        .skip(1)
        .next()
        .expect("Could not parse pacman output");

    // uname output is in the form version-ARCH
    let output_uname = Command::new("uname")
        .arg("-r")
        .output()
        .expect("Could not execute uname");
    let output_uname = String::from_utf8_lossy(&output_uname.stdout);
    let output_uname = output_uname.split("-ARCH")
        .next()
        .expect("Could not parse uname output");


    print!("installed: {}\nrunning: {}\n", output_pacman, output_uname);
}

