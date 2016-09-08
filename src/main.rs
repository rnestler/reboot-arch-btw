extern crate notify_rust;
extern crate rustc_serialize;
extern crate docopt;
extern crate alpm;


use std::process::Command;

use notify_rust::Notification;
use docopt::Docopt;

macro_rules! t {
    ( $e : expr ) => (
        match $e {
            Some(r) => r,
            None => return None,
        }
    )
}


/// Parse the many flavors of pacmans version strings
fn parse_pacman_version(version_str: &str) -> Option<String> {
    let mut main_patch_pkg = version_str.split("-");

    let main_patch = t!(main_patch_pkg.next());
    let pkg = t!(main_patch_pkg.next());

    let mut main_patch = main_patch.split("_").clone();
    let main = t!(main_patch.next());
    let patch = match main_patch.next() {
        Some(p) => format!("_{}", p),
        None => "".into(),
    };

    // reconstruct a semver compatible string by appending ".0"s as needed
    let mut main_split: Vec<_> = main.split(".").collect();
    while main_split.len() < 3 {
        main_split.push("0");
    }
    let main = main_split.join(".");

    Some(format!("{}{}-{}", main, patch, pkg))
}

/// Parse the output of `uname -r`
fn parse_uname_output(uname_output: &str) -> Option<&str> {
    uname_output.split("-ARCH")
        .next()
}

const USAGE: &'static str = "
Check the currently installed kernel against the currently running one.

Usage:
  kernel-updated
  kernel-updated (-h | --help)
  kernel-updated --version

Options:
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_version: bool,
}


fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    if args.flag_version {
        println!("kernel-updated: {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    
    let pacman = alpm::Alpm::new().unwrap();
    let output_pacman = pacman.query_package_version("linux").unwrap();
    let output_pacman = parse_pacman_version(&output_pacman)
        .expect("Could not parse pacman output");

    // uname output is in the form version-ARCH
    let output_uname = Command::new("uname")
        .arg("-r")
        .output()
        .expect("Could not execute uname");
    let output_uname = String::from_utf8_lossy(&output_uname.stdout);
    let output_uname = parse_uname_output(&output_uname)
        .expect("Could not parse uname output");

    println!("installed: {}", output_pacman);
    println!("running:   {}", output_uname);

    let should_reboot = output_pacman != output_uname;

    if should_reboot {
        println!("You should reboot your system!");
        Notification::new()
            .summary("Reboot needed")
            .body("Kernel got updated! You should reboot your system!")
            .timeout(6000) //milliseconds
            .show().unwrap();
    }
}


#[cfg(test)]
mod test {
    use super::{parse_pacman_version, parse_uname_output};

    #[test]
    fn test_parse_pacman_output() {
        assert_eq!(Some("4.5.4-1".into()), parse_pacman_version("4.5.4-1"));
    }

    #[test]
    fn test_parse_pacman_zero_output_0() {
        assert_eq!(Some("1.10.0_patch1-1".into()), parse_pacman_version("1.10.0_patch1-1"));
    }

    #[test]
    fn test_parse_pacman_zero_output_1() {
        assert_eq!(Some("4.7.0-2".into()), parse_pacman_version("4.7-2"));
    }

    #[test]
    fn test_parse_uname_output() {
        assert_eq!(Some("4.5.4-1"), parse_uname_output("4.5.4-1-ARCH"));
    }
}

