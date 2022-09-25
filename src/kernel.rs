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
}
