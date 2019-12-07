#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate notify_rust;

use std::process::Command;

use docopt::Docopt;
use notify_rust::Notification;

/// Parse the output of `pacman -Q linux`
fn parse_pacman_output(pacman_ouput: &str) -> Option<String> {
    pacman_ouput
        .split_whitespace()
        .nth(1)
        .map(|version| version.trim().replace(".arch", "."))
}

fn get_package_version(package_name: &str) -> Option<String> {
    let output_pacman = Command::new("pacman")
        .arg("-Q")
        .arg(package_name)
        .output()
        .expect("Could not execute pacman");
    // pacman output is in the form "package version"
    let output_pacman = String::from_utf8_lossy(&output_pacman.stdout);

    parse_pacman_output(&output_pacman).map(|output| output.to_string())
}

/// Parse the output of `uname -r`
fn parse_uname_output(uname_output: &str) -> Option<String> {
    Some(uname_output.trim().replace("-arch", "."))
}

/// Parse the output of `xdpyinfo`
fn parse_xdpyinfo_output(xdpyinfo_output: &str) -> Option<&str> {
    for line in xdpyinfo_output.lines() {
        if line.starts_with("X.Org version: ") {
            return Some(line.split_at(15).1);
        }
    }
    None
}

const USAGE: &str = "
Check the currently installed kernel against the currently running one.

Usage:
  reboot-arch-btw
  reboot-arch-btw (-h | --help)
  reboot-arch-btw --version

Options:
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_version: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    if args.flag_version {
        println!("reboot-arch-btw: {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    // uname output is in the form version-ARCH
    let output_uname = Command::new("uname")
        .arg("-r")
        .output()
        .expect("Could not execute uname");
    let output_uname = String::from_utf8_lossy(&output_uname.stdout);
    let running_kernel_version =
        parse_uname_output(&output_uname).expect("Could not parse uname output");

    let installed_kernel_version =
        get_package_version("linux").expect("Could not get version of installed kernel");

    println!("Kernel");
    println!(" installed: {}", installed_kernel_version);
    println!(" running:   {}", running_kernel_version);

    let should_reboot = installed_kernel_version != running_kernel_version;

    if should_reboot {
        println!("You should reboot arch btw!");
        Notification::new()
            .summary("Reboot arch btw")
            .body("Kernel got updated. You should reboot your system!")
            .timeout(6000) //milliseconds
            .show()
            .unwrap();
    }

    let output_xdpyinfo = Command::new("xdpyinfo")
        .output()
        .expect("Could not execute xdpyinfo");
    let output_xdpyinfo = String::from_utf8_lossy(&output_xdpyinfo.stdout);
    let running_xorg_version =
        parse_xdpyinfo_output(&output_xdpyinfo).expect("Could not parse xdpyinfo output");
    let installed_xorg_version =
        get_package_version("xorg-server").expect("Could not get version of installed xserver");

    println!("Xorg server");
    println!(" installed: {}", installed_xorg_version);
    println!(" running:   {}", running_xorg_version);
}

#[cfg(test)]
mod test {
    use super::{parse_pacman_output, parse_uname_output, parse_xdpyinfo_output};

    #[test]
    fn test_parse_pacman_output() {
        assert_eq!(
            Some("5.3.11.1-1".to_owned()),
            parse_pacman_output("linux 5.3.11.1-1")
        );
        assert_eq!(
            Some("5.4.1.1-1".to_owned()),
            parse_pacman_output("linux 5.4.1.arch1-1")
        );
    }

    #[test]
    fn test_parse_uname_output() {
        assert_eq!(
            Some("5.3.11.1-1".to_owned()),
            parse_uname_output("5.3.11-arch1-1")
        );
        assert_eq!(
            Some("5.4.1.1-1".to_owned()),
            parse_uname_output("5.4.1-arch1-1")
        );
    }

    #[test]
    fn test_parse_xdpyinfo_output() {
        assert_eq!(
            Some("1.18.4"),
            parse_xdpyinfo_output("X.Org version: 1.18.4")
        );
    }
}
