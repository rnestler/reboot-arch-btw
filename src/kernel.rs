use crate::checks::{Check, CheckResult};
use crate::package::{get_package_version, PackageInfo};
use anyhow::{anyhow, Context, Result};
use log::info;
use std::fmt::Display;
use std::process::Command;

#[derive(Debug, PartialEq, Eq)]
pub struct KernelInfo {
    pub version: String,
    pub variant: Option<String>,
    pub package_name: String,
}

impl Display for KernelInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.version)?;
        if let Some(variant) = &self.variant {
            write!(f, "-{}", variant)?;
        }
        Ok(())
    }
}

/// These variants trip up our auto-detection since they contain multiple dashes and numbers
const WELL_KNOWN_VARIANTS: [&str; 4] = [
    "ck-generic",
    "ck-generic-v2",
    "ck-generic-v3",
    "ck-generic-v4",
];

impl KernelInfo {
    pub fn from_uname() -> Result<KernelInfo> {
        let output_uname = Command::new("uname").arg("-r").output()?;
        let output_uname_stdout = String::from_utf8_lossy(&output_uname.stdout);
        Self::from_uname_output(&output_uname_stdout)
    }
    pub fn from_uname_output(uname_output: &str) -> Result<KernelInfo> {
        // uname output is in the form version-ARCH
        let uname_output = uname_output.trim();
        info!("uname -r output: {uname_output}");

        if let Some(variant) = WELL_KNOWN_VARIANTS
            .iter()
            .find(|variant| uname_output.ends_with(*variant))
        {
            return Ok(KernelInfo {
                version: uname_output
                    .trim_end_matches(variant)
                    .trim_end_matches('-')
                    .replace('-', "."),
                variant: Some(variant.to_string()),
                package_name: format!("linux-{variant}"),
            });
        }

        let last_dash = uname_output
            .rfind('-')
            .ok_or_else(|| anyhow!("Could not find '-' in uname output: {uname_output}"))?;
        let last_part = &uname_output[last_dash + 1..];
        // if the last part is text it is a kernel variant
        if last_part.chars().all(char::is_alphabetic) {
            let variant = last_part.to_string();
            let version = uname_output[0..last_dash].replace('-', ".");

            if variant == "MANJARO" {
                return Self::get_manjaro_kernel_info(version);
            }

            Ok(KernelInfo {
                version,
                package_name: format!("linux-{variant}"),
                variant: Some(variant),
            })
        } else {
            Ok(KernelInfo {
                version: uname_output.replace('-', "."),
                variant: None,
                package_name: "linux".to_string(),
            })
        }
    }

    fn get_manjaro_kernel_info(version: String) -> Result<KernelInfo> {
        let mut version_iter = version.split('.');
        let major = version_iter
            .next()
            .ok_or_else(|| anyhow!("Could not find major version"))?;
        let minor = version_iter
            .next()
            .ok_or_else(|| anyhow!("Could not find minor version"))?;

        Ok(KernelInfo {
            variant: Some("MANJARO".to_string()),
            package_name: format!("linux{major}{minor}"),
            version,
        })
    }
}

pub struct KernelChecker {
    kernel_info: KernelInfo,
    installed_kernel: PackageInfo,
    verbose: bool,
}

impl KernelChecker {
    pub fn new(db: &alpm::Db, verbose: bool) -> Result<KernelChecker> {
        let kernel_info = KernelInfo::from_uname()?;
        let kernel_package = &kernel_info.package_name;
        info!("Detected kernel package: {kernel_package}");
        let installed_kernel = get_package_version(db, kernel_package)
            .with_context(|| anyhow!("Could not get version of installed kernel"))?;
        info!("kernel package version: {}", installed_kernel.version);
        Ok(KernelChecker {
            kernel_info,
            installed_kernel,
            verbose,
        })
    }
}

impl Check for KernelChecker {
    fn check(&self) -> CheckResult {
        let cleaned_kernel_version =
            PackageInfo::cleanup_kernel_version(&self.installed_kernel.version)
                .expect("Could not clean version of installed kernel");
        let running_kernel_version = &self.kernel_info.version;
        let should_reboot = running_kernel_version != &cleaned_kernel_version;
        if self.verbose {
            println!("Kernel");
            println!(
                " installed: {} (since {})",
                cleaned_kernel_version,
                self.installed_kernel.installed_reltime()
            );
            println!(" running:   {}", self.kernel_info);
        }
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
        let kernel_version = KernelInfo::from_uname_output("5.6.13-arch1-1").unwrap();
        assert_eq!(
            KernelInfo {
                version: "5.6.13.arch1.1".to_string(),
                variant: None,
                package_name: "linux".to_string(),
            },
            kernel_version
        );
    }

    #[test]
    fn test_kernel_version_from_uname_output_zen() {
        let kernel_version = KernelInfo::from_uname_output("5.6.11-zen1-1-zen").unwrap();
        assert_eq!(
            KernelInfo {
                version: "5.6.11.zen1.1".to_owned(),
                variant: Some("zen".to_owned()),
                package_name: "linux-zen".to_owned(),
            },
            kernel_version
        );
    }

    #[test]
    fn test_kernel_version_from_uname_output_lts() {
        let kernel_version = KernelInfo::from_uname_output("5.15.69-1-lts").unwrap();
        assert_eq!(
            KernelInfo {
                version: "5.15.69.1".to_owned(),
                variant: Some("lts".to_owned()),
                package_name: "linux-lts".to_owned(),
            },
            kernel_version
        );
    }
    #[test]
    fn test_kernel_version_from_uname_output_rust() {
        let kernel_version = KernelInfo::from_uname_output("6.3.9-arch1-1-rust").unwrap();
        assert_eq!(
            KernelInfo {
                version: "6.3.9.arch1.1".to_owned(),
                variant: Some("rust".to_owned()),
                package_name: "linux-rust".to_owned(),
            },
            kernel_version
        );
    }

    #[test]
    fn test_kernel_verion_from_uname_output_manjaro() {
        let kernel_version = KernelInfo::from_uname_output("6.1.71-1-MANJARO").unwrap();
        assert_eq!(
            KernelInfo {
                version: "6.1.71.1".to_owned(),
                variant: Some("MANJARO".to_owned()),
                package_name: "linux61".to_owned()
            },
            kernel_version
        )
    }

    #[test]
    fn test_kernel_version_from_uname_output_ck_generic_v3() {
        let kernel_version = KernelInfo::from_uname_output("6.4.1-2-ck-generic-v3").unwrap();
        assert_eq!(
            KernelInfo {
                version: "6.4.1.2".to_owned(),
                variant: Some("ck-generic-v3".to_owned()),
                package_name: "linux-ck-generic-v3".to_owned(),
            },
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
            verbose: false,
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
            verbose: false,
        };

        assert_eq!(kernel_checker.check(), CheckResult::Nothing);
    }
}
