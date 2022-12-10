use std::{num::ParseIntError, str::FromStr};

use clap::Parser;
use log::error;
use notify_rust::{Notification, Timeout};

mod package;

mod kernel;
use kernel::KernelChecker;

mod checks;
use checks::{Check, CheckResult};
mod critical_packages_check;
use critical_packages_check::CriticalPackagesCheck;
mod session;

fn timeout_from_str(s: &str) -> Result<Timeout, ParseIntError> {
    match s {
        "default" => Ok(Timeout::Default),
        "never" => Ok(Timeout::Never),
        milliseconds => Ok(Timeout::Milliseconds(u32::from_str(milliseconds)?)),
    }
}

#[derive(Debug, Parser)]
#[clap(
    version,
    about = "Check if a reboot is needed due to an updated kernel or other system packages."
)]
struct Args {
    /// Disable desktop notification
    #[clap(long)]
    disable_notification: bool,

    /// Timeout for the desktop notification in milliseconds.
    ///
    /// * "default" will leave the timeout to be set by the server.
    ///
    /// * "never" or "0" will cause the notification never to expire.
    ///
    /// * Any other number will be interpreted as the timeout in milliseconds.
    #[clap(long, value_parser(timeout_from_str), default_value = "default")]
    notification_timeout: Timeout,

    /// Comma separated list of packages were we should reboot after an upgrade.
    #[clap(
        long,
        use_value_delimiter = true,
        default_value = "systemd,linux-firmware,amd-ucode,intel-ucode"
    )]
    reboot_packages: Vec<String>,

    /// Comma separated list of packages were we should restart our session after an upgrade.
    #[clap(
        long,
        use_value_delimiter = true,
        default_value = "xorg-server,xorg-xwayland"
    )]
    session_restart_packages: Vec<String>,
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    // Initialize Pacman database
    let alpm = alpm::Alpm::new("/", "/var/lib/pacman/")
        .expect("Could not open pacman database at /var/lib/pacman");
    let db = alpm.localdb();

    let mut checkers: Vec<Box<dyn Check>> = vec![];

    match KernelChecker::new(db) {
        Ok(kernel_checker) => checkers.push(Box::new(kernel_checker)),
        Err(err) => {
            error!("Could not create kernel checker: {err:#}")
        }
    }

    match CriticalPackagesCheck::new(args.reboot_packages, args.session_restart_packages, db) {
        Ok(critical_packages_checker) => checkers.push(Box::new(critical_packages_checker)),
        Err(err) => {
            error!("Could not create critical package checker: {err:#}")
        }
    }

    let result = checkers
        .iter()
        .map(|v| v.check())
        .max()
        .expect("No checkers could run");

    if result > CheckResult::Nothing {
        println!("{}", result.summary());
        if !args.disable_notification {
            Notification::new()
                .summary(result.summary())
                .body(result.body())
                .timeout(args.notification_timeout)
                .show()
                .map_err(|e| error!("Couldn't send notification: {}", e))
                .ok();
        }
    }
}
