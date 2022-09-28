use clap::Parser;
use notify_rust::Notification;

mod package;

mod kernel;
use kernel::{KernelChecker, KernelInfo};

mod checks;
use checks::{Check, CheckResult};
mod critical_packages_check;
use critical_packages_check::CriticalPackagesCheck;
mod session;

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
    #[clap(long, use_delimiter = true, default_value = "systemd,linux-firmware")]
    reboot_packages: Vec<String>,

    /// Comma separated list of packages were we should restart our session after an upgrade.
    #[clap(
        long,
        use_delimiter = true,
        default_value = "xorg-server,xorg-xwayland"
    )]
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

    if result > CheckResult::Nothing {
        println!("{}", result.summary());
        if !args.disable_notification {
            Notification::new()
                .summary(result.summary())
                .body(result.body())
                .timeout(6000) //milliseconds
                .show()
                .map_err(|e| println!("Couldn't send notification: {}", e))
                .ok();
        }
    }
}
