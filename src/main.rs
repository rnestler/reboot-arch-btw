use clap::Parser;

use log::error;
use notify_rust::{Notification, Timeout, Urgency};

mod package;

mod kernel;
use kernel::KernelChecker;

mod checks;
use checks::{Check, CheckResult};
mod critical_packages_check;
use critical_packages_check::CriticalPackagesCheck;
mod session;

#[derive(Debug, Clone, Copy)]
enum NotificationUrgency {
    Auto,
    Low,
    Normal,
    Critical,
}

impl std::str::FromStr for NotificationUrgency {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(Self::Auto),
            "low" => Ok(Self::Low),
            "normal" => Ok(Self::Normal),
            "critical" => Ok(Self::Critical),
            _ => Err(format!(
                "Expected one of: auto, low, normal, critical. Got: {}",
                s
            )),
        }
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
    #[clap(long, default_value = "default")]
    notification_timeout: Timeout,

    /// Comma separated list of packages where we should reboot after an upgrade.
    #[clap(
        long,
        use_value_delimiter = true,
        default_value = "systemd,linux-firmware,amd-ucode,intel-ucode"
    )]
    reboot_packages: Vec<String>,

    /// Comma separated list of packages where we should restart our session after an upgrade.
    #[clap(
        long,
        use_value_delimiter = true,
        default_value = "xorg-server,xorg-xwayland"
    )]
    session_restart_packages: Vec<String>,

    /// Set the urgency level for desktop notifications.
    ///
    /// * "auto" sets the urgency based on the check result (the default):
    ///   kernel updates are critical, reboots are normal, session restarts are low.
    ///
    /// * "low", "normal", or "critical" sets a fixed urgency level for all notifications.
    #[clap(long, default_value = "auto")]
    notification_urgency: NotificationUrgency,

    /// Print kernel version info and show updated packages.
    #[clap(short, long)]
    verbose: bool,

    /// Return a non-zero exit code based on the check result:
    ///   0 = nothing to do
    ///   1 = reboot needed
    ///   2 = session restart needed
    /// Regardless of this flag, unexpected errors (e.g. failure to open the
    /// pacman database) will panic and exit with code 101, while invalid
    /// command line arguments will exit with code 64.
    #[clap(long)]
    exit_code: bool,
}

fn main() {
    env_logger::init();
    let args = match Args::try_parse() {
        Ok(args) => args,
        Err(e) => {
            let _ = e.print();
            if e.exit_code() == 0 {
                std::process::exit(0);
            }
            std::process::exit(64);
        }
    };

    // Initialize Pacman database
    let alpm = alpm::Alpm::new("/", "/var/lib/pacman/")
        .expect("Could not open pacman database at /var/lib/pacman");
    let db = alpm.localdb();

    let mut checkers: Vec<Box<dyn Check>> = vec![];

    match KernelChecker::new(db, args.verbose) {
        Ok(kernel_checker) => checkers.push(Box::new(kernel_checker)),
        Err(err) => {
            error!("Could not create kernel checker: {err:#}")
        }
    }

    match CriticalPackagesCheck::new(
        args.reboot_packages,
        args.session_restart_packages,
        db,
        args.verbose,
    ) {
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
            let urgency = match args.notification_urgency {
                NotificationUrgency::Auto => result.urgency(),
                NotificationUrgency::Low => Urgency::Low,
                NotificationUrgency::Normal => Urgency::Normal,
                NotificationUrgency::Critical => Urgency::Critical,
            };
            Notification::new()
                .summary(result.summary())
                .body(result.body())
                .urgency(urgency)
                .timeout(args.notification_timeout)
                .show()
                .map_err(|e| error!("Couldn't send notification: {}", e))
                .ok();
        }
    }

    if args.exit_code {
        let code = match result {
            CheckResult::Nothing => 0,
            CheckResult::Reboot | CheckResult::KernelUpdate => 1,
            CheckResult::RestartSession => 2,
        };
        std::process::exit(code);
    }
}
