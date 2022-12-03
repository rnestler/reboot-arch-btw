use anyhow::Result;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct PackageInfo {
    pub version: String,
    pub install_date: Option<i64>,
}

impl PackageInfo {
    pub fn from_package(pkg: &alpm::Package) -> Self {
        Self {
            version: pkg.version().to_string(),
            install_date: pkg.install_date(),
        }
    }

    /// Read a decimal number from the input and return the parsed number and the remaining input
    pub fn read_number(input: &str) -> (Option<u32>, &str) {
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
    pub fn cleanup_kernel_version(raw_version: &str) -> Option<String> {
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
    pub fn installed_reltime(&self) -> String {
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
}

pub fn get_package_version(db: alpm::Db, package_name: &str) -> Result<PackageInfo> {
    let package = db.pkg(package_name)?;
    Ok(PackageInfo::from_package(&package))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cleanup_pkg_version() {
        assert_eq!(
            PackageInfo::cleanup_kernel_version("5.3.11.1-1"),
            Some("5.3.11.1.1".to_owned()),
        );
        assert_eq!(
            PackageInfo::cleanup_kernel_version("5.4.1.arch1-1"),
            Some("5.4.1.arch1.1".to_owned()),
        );
        assert_eq!(
            PackageInfo::cleanup_kernel_version("5.15.69-1"),
            Some("5.15.69.1".to_owned()),
        );
    }

    #[test]
    fn test_cleanup_pkg_version_missing_patch_digit() {
        assert_eq!(
            PackageInfo::cleanup_kernel_version("5.16.arch1-1"),
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
}
