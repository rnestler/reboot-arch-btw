use crate::checks::{Check, CheckResult};
use crate::package::{get_package_version, PackageInfo};
use crate::session::SessionInfo;
use log::{info, warn};

pub struct CriticalPackagesCheck<'a> {
    /// Compares the installation time of packages to the time since the last boot.
    reboot_package_names: Vec<String>,
    restart_session_package_names: Vec<String>,
    session_info: SessionInfo,
    alpm_db: alpm::Db<'a>,
}

impl CriticalPackagesCheck<'_> {
    pub fn new(
        reboot_package_names: Vec<String>,
        restart_session_package_names: Vec<String>,
        session_info: SessionInfo,
        alpm_db: alpm::Db,
    ) -> CriticalPackagesCheck {
        CriticalPackagesCheck {
            reboot_package_names,
            restart_session_package_names,
            session_info,
            alpm_db,
        }
    }

    fn check_package_list(&self, package_list: &[String], max_install_date: i64) -> bool {
        for package_name in package_list {
            info!("Checking {package_name}");
            let package_info = get_package_version(self.alpm_db, package_name);
            if let Some(PackageInfo {
                install_date: Some(install_date),
                ..
            }) = package_info
            {
                if install_date > max_install_date {
                    println!(
                        "{package_name} updated {}",
                        package_info.unwrap().installed_reltime()
                    );
                    return true;
                }
            } else {
                warn!("Failed to get package info for {package_name}");
            }
        }
        false
    }
}

impl Check for CriticalPackagesCheck<'_> {
    fn check(&self) -> CheckResult {
        let boot_time = self.session_info.boot_time.unix_timestamp();
        let session_time = self.session_info.session_time.unix_timestamp();

        if self.check_package_list(&self.reboot_package_names, boot_time) {
            return CheckResult::Reboot;
        }
        if self.check_package_list(&self.restart_session_package_names, session_time) {
            return CheckResult::RestartSession;
        }
        CheckResult::Nothing
    }
}
