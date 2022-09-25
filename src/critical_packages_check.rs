use crate::checks::{Check, CheckResult};
use crate::package::{get_package_version, PackageInfo};
use crate::session::SessionInfo;

pub struct CriticalPackagesCheck<'a> {
    /// Compares the installation time of packages to the time since the last boot.
    package_names: Vec<String>,
    session_info: SessionInfo,
    alpm_db: alpm::Db<'a>,
}

impl CriticalPackagesCheck<'_> {
    pub fn new(
        package_names: Vec<String>,
        session_info: SessionInfo,
        alpm_db: alpm::Db,
    ) -> CriticalPackagesCheck {
        CriticalPackagesCheck {
            package_names,
            session_info,
            alpm_db,
        }
    }
}

impl Check for CriticalPackagesCheck<'_> {
    fn check(&self) -> CheckResult {
        let boot_time = self.session_info.boot_time.unix_timestamp();
        for package_name in &self.package_names {
            let package_info = get_package_version(self.alpm_db, package_name);
            if let Some(PackageInfo {
                install_date: Some(install_date),
                ..
            }) = package_info
            {
                if install_date > boot_time {
                    println!(
                        "{package_name} updated {}",
                        package_info.unwrap().installed_reltime()
                    );
                    return CheckResult::Reboot;
                }
            }
        }
        CheckResult::Nothing
    }
}
