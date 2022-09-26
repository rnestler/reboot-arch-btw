use std::process::Command;

use clap::Parser;
use notify_rust::Notification;

mod package;
use package::get_package_version;

mod kernel;
use kernel::{KernelChecker, KernelInfo};

mod checks;
use checks::{Check, CheckResult};
mod critical_packages_check;
use critical_packages_check::CriticalPackagesCheck;
mod session;

/// Parse the output of `xdpyinfo`
fn parse_xdpyinfo_output(xdpyinfo_output: &str) -> Option<&str> {
    for line in xdpyinfo_output.lines() {
        if line.starts_with("X.Org version: ") {
            return Some(line.split_at(15).1);
        }
    }
    None
}

#[derive(Debug, Parser)]
#[clap(
    version,
    about = "Check the currently installed kernel against the currently running one."
)]
struct Args {
    /// Disable desktop notification
    #[clap(long)]
    disable_notification: bool,

    /// Comma separated list of packages were we should reboot after an upgrade.
    #[clap(long, use_delimiter = true)]
    reboot_packages: Vec<String>,

    /// Comma separated list of packages were we should restart our session after an upgrade.
    #[clap(long, use_delimiter = true)]
    session_restart_packages: Vec<String>,
}

fn main() {
    let args = Args::from_args();

    // Initialize Pacman database
    let alpm = alpm::Alpm::new("/", "/var/lib/pacman/")
        .expect("Could not open pacman database at /var/lib/pacman");
    let db = alpm.localdb();

    let kernel_info = KernelInfo::from_uname().expect("Failed to parse uname output");

    let mut checkers: Vec<Box<dyn Check>> = vec![Box::new(KernelChecker::new(kernel_info, db))];

    let session_info = session::SessionInfo::from_utmp();
    if let Ok(session_info) = session_info {
        let critical_packages_checker = CriticalPackagesCheck::new(
            args.reboot_packages,
            args.session_restart_packages,
            session_info,
            db,
        );
        checkers.push(Box::new(critical_packages_checker));
    }

    let result = checkers
        .iter()
        .map(|v| v.check())
        .max()
        .expect("No checkers could run");

    if result == CheckResult::Reboot {
        println!("You should reboot arch btw!");
        if !args.disable_notification {
            Notification::new()
                .summary("Reboot arch btw")
                .body("System got updated. You should reboot your system!")
                .timeout(6000) //milliseconds
                .show()
                .map_err(|e| println!("Couldn't send notification: {}", e))
                .ok();
        }
    }

    let output_xdpyinfo = Command::new("xdpyinfo")
        .output()
        .map_err(|err| println!("Could not execute xdpyinfo: {}", err));
    if let Ok(output_xdpyinfo) = output_xdpyinfo {
        let output_xdpyinfo = String::from_utf8_lossy(&output_xdpyinfo.stdout);
        let running_xorg_version =
            parse_xdpyinfo_output(&output_xdpyinfo).expect("Could not parse xdpyinfo output");
        let installed_xorg = get_package_version(db, "xorg-server")
            .expect("Could not get version of installed xserver");

        println!("Xorg server");
        println!(
            " installed: {} (since {})",
            installed_xorg.version,
            installed_xorg.installed_reltime()
        );
        println!(" running:   {}", running_xorg_version);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_xdpyinfo_output() {
        assert_eq!(
            Some("1.18.4"),
            parse_xdpyinfo_output("X.Org version: 1.18.4")
        );
    }
}
