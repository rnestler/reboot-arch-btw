use crate::checks::{Check, CheckResult};
use crate::package::{get_package_version, PackageInfo};
use std::process::Command;

#[derive(Debug, PartialEq, Eq)]
pub struct KernelInfo {
    pub version: String,
    pub variant: Option<String>,
}

impl KernelInfo {
    pub fn from_uname() -> Option<KernelInfo> {
        let output_uname = Command::new("uname")
            .arg("-r")
            .output()
            .expect("Could not execute uname");
        let output_uname_stdout = String::from_utf8_lossy(&output_uname.stdout);
        Self::from_uname_output(&output_uname_stdout)
    }
    pub fn from_uname_output(uname_output: &str) -> Option<KernelInfo> {
        // uname output is in the form version-ARCH
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

pub struct KernelChecker {
    kernel_info: KernelInfo,
    installed_kernel: PackageInfo,
}

impl KernelChecker {
    pub fn new(kernel_info: KernelInfo, db: alpm::Db) -> KernelChecker {
        let kernel_package = if let Some(variant) = &kernel_info.variant {
            format!("linux-{}", variant)
        } else {
            "linux".to_owned()
        };
        let installed_kernel = get_package_version(db, &kernel_package)
            .expect("Could not get version of installed kernel");
        KernelChecker {
            kernel_info,
            installed_kernel,
        }
    }
}

impl Check for KernelChecker {
    fn check(&self) -> CheckResult {
        let cleaned_kernel_version =
            PackageInfo::cleanup_kernel_version(&self.installed_kernel.version)
                .expect("Could not clean version of installed kernel");
        println!("Kernel");
        println!(
            " installed: {} (since {})",
            cleaned_kernel_version,
            self.installed_kernel.installed_reltime()
        );
        let running_kernel_version = &self.kernel_info.version;
        println!(" running:   {}", running_kernel_version);
        let should_reboot = running_kernel_version != &cleaned_kernel_version;
        if should_reboot {
            CheckResult::KernelUpdate
        } else {
            CheckResult::Nothing
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

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

    #[test]
    fn test_kernel_version_from_uname_output_lts() {
        let kernel_version = KernelInfo::from_uname_output("5.15.69-1-lts");
        assert_eq!(
            Some(KernelInfo {
                version: "5.15.69.1".to_owned(),
                variant: Some("lts".to_owned()),
            }),
            kernel_version
        );
    }

    #[test]
    fn test_kernel_checker_should_reboot() {
        let kernel_checker = KernelChecker {
            kernel_info: KernelInfo::from_uname_output("5.19.9-arch1-1").unwrap(),
            installed_kernel: PackageInfo {
                version: "5.19.11.arch1-1".to_owned(),
                install_date: None,
            },
        };

        assert_eq!(kernel_checker.check(), CheckResult::KernelUpdate);
    }

    #[test]
    fn test_kernel_checker_should_not_reboot() {
        let kernel_checker = KernelChecker {
            kernel_info: KernelInfo::from_uname_output("5.19.9-arch1-1").unwrap(),
            installed_kernel: PackageInfo {
                version: "5.19.9.arch1-1".to_owned(),
                install_date: None,
            },
        };

        assert_eq!(kernel_checker.check(), CheckResult::Nothing);
    }
}
