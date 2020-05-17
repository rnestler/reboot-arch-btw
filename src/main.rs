use alpm;
use docopt;
use notify_rust;
use serde_derive::Deserialize;

use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use docopt::Docopt;
use notify_rust::Notification;

struct PackageInfo {
    version: String,
    install_date: Option<i64>,
}

impl PackageInfo {
    fn from_package(pkg: &alpm::Package) -> Self {
        Self {
            version: Self::cleanup_pkg_version(pkg.version()),
            install_date: pkg.install_date(),
        }
    }

    /// Clean up Arch package versions.
    #[inline]
    fn cleanup_pkg_version(raw_version: &str) -> String {
        raw_version.replace(".arch", "-arch")
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
        let last_part = uname_output.split('-').last()?;
        let last_dash = uname_output.rfind('-')?;
        // if the last part is text it is a kernel variant
        if last_part.chars().all(char::is_alphabetic) {
            Some(KernelInfo {
                version: uname_output[0..last_dash].to_string(),
                variant: Some(uname_output[last_dash + 1..].to_string()),
            })
        } else {
            Some(KernelInfo {
                version: uname_output.to_string(),
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
    Some(PackageInfo::from_package(&pkg))
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
        Notification::new()
            .summary("Reboot arch btw")
            .body("Kernel got updated. You should reboot your system!")
            .timeout(6000) //milliseconds
            .show()
            .unwrap();
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
            "5.3.11.1-1".to_owned(),
        );
        assert_eq!(
            PackageInfo::cleanup_pkg_version("5.4.1.arch1-1"),
            "5.4.1-arch1-1".to_owned(),
        );
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
                version: "5.6.13-arch1-1".to_string(),
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
                version: "5.6.11-zen1-1".to_owned(),
                variant: Some("zen".to_owned()),
            }),
            kernel_version
        );
    }
}
