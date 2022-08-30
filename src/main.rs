use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use notify_rust::Notification;
use clap::StructOpt;

struct PackageInfo {
    version: String,
    install_date: Option<i64>,
}

impl PackageInfo {
    fn from_package(pkg: &alpm::Package) -> Option<Self> {
        Some(Self {
            version: Self::cleanup_pkg_version(pkg.version())?,
            install_date: pkg.install_date(),
        })
    }

    /// Read a decimal number from the input and return the parsed number and the remaining input
    fn read_number(input: &str) -> (Option<u32>, &str) {
        let res = input.find(|ch| !('0'..='9').contains(&ch));
        if res.is_none() {}
        match res {
            None => (input.parse().ok(), ""),
            Some(0) => (None, input),
            Some(x) => (input[0..x].parse().ok(), &input[x..]),
        }
    }

    /// Read a `.` or not and return the remaining input
    fn read_dot(input: &str) -> &str {
        input.strip_prefix('.').unwrap_or(input)
    }

    /// Clean up Arch package versions.
    fn cleanup_pkg_version(raw_version: &str) -> Option<String> {
        let mut version = String::new();
        let (n, mut remaining) = Self::read_number(raw_version);
        version += &n?.to_string();
        remaining = Self::read_dot(remaining);
        version += ".";

        let (n, mut remaining) = Self::read_number(remaining);
        version += &n?.to_string();
        remaining = Self::read_dot(remaining);
        version += ".";

        // if third digit is missing insert a 0.
        let (n, _) = Self::read_number(remaining);
        if n.is_none() {
            version += "0."
        }
        version += remaining;

        Some(version.replace('-', "."))
    }

    /// Return a string representing the "time ago" when this package was
    /// installed.
    fn installed_reltime(&self) -> String {
        let install_date = match self.install_date {
            Some(d) => d,
            None => return "unknown".to_string(),
        } as u64;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards!")
            .as_secs();
        let delta = now - install_date;
        if delta < 60 {
            format!("{} seconds ago", delta)
        } else if delta < 7200 {
            format!("{} minutes ago", delta / 60)
        } else if delta < 3600 * 36 {
            format!("{} hours ago", delta / 3600)
        } else {
            format!("{} days ago", delta / (3600 * 24))
        }
    }

    #[inline]
    fn version_matches(&self, other_version: &str) -> bool {
        self.version == other_version
    }
}

#[derive(Debug, PartialEq, Eq)]
struct KernelInfo {
    version: String,
    variant: Option<String>,
}

impl KernelInfo {
    pub fn from_uname_output(uname_output: &str) -> Option<KernelInfo> {
        let uname_output = uname_output.trim();
        let last_dash = uname_output.rfind('-')?;
        let last_part = &uname_output[last_dash + 1..];
        // if the last part is text it is a kernel variant
        if last_part.chars().all(char::is_alphabetic) {
            Some(KernelInfo {
                version: uname_output[0..last_dash].replace('-', "."),
                variant: Some(last_part.to_string()),
            })
        } else {
            Some(KernelInfo {
                version: uname_output.replace('-', "."),
                variant: None,
            })
        }
    }
}

fn get_package_version(db: &alpm::Db, package_name: &str) -> Option<PackageInfo> {
    let pkg = match db.pkg(package_name) {
        Ok(pkg) => pkg,
        Err(_) => return None,
    };
    PackageInfo::from_package(&pkg)
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

#[derive(Debug, StructOpt)]
#[structopt(about = "Check the currently installed kernel against the currently running one.")]
struct Args {
    /// Disable desktop notification
    #[structopt(long)]
    disable_notification: bool,
}

fn main() {
    let args = Args::from_args();

    // Initialize Pacman database
    let alpm = alpm::Alpm::new("/", "/var/lib/pacman/")
        .expect("Could not open pacman database at /var/lib/pacman");
    let db = alpm.localdb();

    // uname output is in the form version-ARCH
    let output_uname = Command::new("uname")
        .arg("-r")
        .output()
        .expect("Could not execute uname");
    let output_uname_stdout = String::from_utf8_lossy(&output_uname.stdout);
    let kernel_info =
        KernelInfo::from_uname_output(&output_uname_stdout).expect("Failed to parse uname output");
    let running_kernel_version = kernel_info.version;

    let kernel_package = if let Some(variant) = kernel_info.variant {
        format!("linux-{}", variant)
    } else {
        "linux".to_owned()
    };

    let installed_kernel = get_package_version(&db, &kernel_package)
        .expect("Could not get version of installed kernel");

    println!("Kernel");
    println!(
        " installed: {} (since {})",
        installed_kernel.version,
        installed_kernel.installed_reltime()
    );
    println!(" running:   {}", running_kernel_version);

    let should_reboot = !installed_kernel.version_matches(&running_kernel_version);
    if should_reboot {
        println!("You should reboot arch btw!");
        if !args.disable_notification {
            Notification::new()
                .summary("Reboot arch btw")
                .body("Kernel got updated. You should reboot your system!")
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
        let installed_xorg = get_package_version(&db, "xorg-server")
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
    fn test_cleanup_pkg_version() {
        assert_eq!(
            PackageInfo::cleanup_pkg_version("5.3.11.1-1"),
            Some("5.3.11.1.1".to_owned()),
        );
        assert_eq!(
            PackageInfo::cleanup_pkg_version("5.4.1.arch1-1"),
            Some("5.4.1.arch1.1".to_owned()),
        );
    }

    #[test]
    fn test_cleanup_pkg_version_missing_patch_digit() {
        assert_eq!(
            PackageInfo::cleanup_pkg_version("5.16.arch1-1"),
            Some("5.16.0.arch1.1".to_owned()),
        );
    }

    #[test]
    fn test_read_number_none() {
        assert_eq!((None, "foo"), PackageInfo::read_number("foo"));
    }

    #[test]
    fn test_read_number_only_number() {
        assert_eq!((Some(90), ""), PackageInfo::read_number("90"));
    }

    #[test]
    fn test_read_number() {
        assert_eq!((Some(1), ".1-foo"), PackageInfo::read_number("1.1-foo"));
    }

    #[test]
    fn test_version_matches() {
        let ver = "5.3.11-arch1-1";
        let info = PackageInfo {
            version: ver.to_string(),
            install_date: None,
        };
        assert!(info.version_matches(ver));
    }

    #[test]
    fn test_parse_xdpyinfo_output() {
        assert_eq!(
            Some("1.18.4"),
            parse_xdpyinfo_output("X.Org version: 1.18.4")
        );
    }

    #[test]
    fn test_kernel_version_from_uname_output_mainline() {
        let kernel_version = KernelInfo::from_uname_output("5.6.13-arch1-1");
        assert_eq!(
            Some(KernelInfo {
                version: "5.6.13.arch1.1".to_string(),
                variant: None,
            }),
            kernel_version
        );
    }

    #[test]
    fn test_kernel_version_from_uname_output_zen() {
        let kernel_version = KernelInfo::from_uname_output("5.6.11-zen1-1-zen");
        assert_eq!(
            Some(KernelInfo {
                version: "5.6.11.zen1.1".to_owned(),
                variant: Some("zen".to_owned()),
            }),
            kernel_version
        );
    }
}
