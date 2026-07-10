#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CheckResult {
    Nothing,
    RestartSession,
    Reboot,
    KernelUpdate,
}

impl CheckResult {
    pub fn summary(&self) -> &'static str {
        match self {
            CheckResult::Nothing => "All good",
            CheckResult::RestartSession => "Restart your session btw",
            CheckResult::Reboot | CheckResult::KernelUpdate => "Reboot arch btw",
        }
    }

    pub fn body(&self) -> &'static str {
        match self {
            CheckResult::Nothing => "Nothing relevant got updated",
            CheckResult::RestartSession => {
                "System packages got updated. You should logout to restart your session."
            }
            CheckResult::Reboot => "System packages got updated. You should reboot your system!",
            CheckResult::KernelUpdate => "Kernel got updated. You should reboot your system!",
        }
    }
}

pub trait Check {
    fn check(&self) -> CheckResult;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_checkresult_ordering() {
        // main() relies on `.max()` picking the most severe result.
        assert!(CheckResult::Nothing < CheckResult::RestartSession);
        assert!(CheckResult::RestartSession < CheckResult::Reboot);
        assert!(CheckResult::Reboot < CheckResult::KernelUpdate);
    }

    #[test]
    fn test_max_picks_most_severe() {
        let results = [
            CheckResult::Nothing,
            CheckResult::KernelUpdate,
            CheckResult::RestartSession,
        ];
        assert_eq!(results.iter().max(), Some(&CheckResult::KernelUpdate));
    }

    #[test]
    fn test_summary() {
        assert_eq!(CheckResult::Nothing.summary(), "All good");
        assert_eq!(
            CheckResult::RestartSession.summary(),
            "Restart your session btw"
        );
        // Both reboot variants share the same summary.
        assert_eq!(CheckResult::Reboot.summary(), "Reboot arch btw");
        assert_eq!(CheckResult::KernelUpdate.summary(), "Reboot arch btw");
    }

    #[test]
    fn test_body_distinguishes_reboot_reason() {
        // The summary is shared, but the body should explain the actual cause.
        assert_ne!(CheckResult::Reboot.body(), CheckResult::KernelUpdate.body());
    }
}
