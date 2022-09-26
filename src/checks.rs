#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum CheckResult {
    Nothing,
    RestartSession,
    Reboot,
}

impl CheckResult {
    pub fn summary(&self) -> &'static str {
        match self {
            CheckResult::Nothing => "All good",
            CheckResult::RestartSession => "Restart your session btw",
            CheckResult::Reboot => "Reboot arch btw",
        }
    }

    pub fn body(&self) -> &'static str {
        match self {
            CheckResult::Nothing => "Nothing relevant got updated",
            CheckResult::RestartSession => {
                "System packages got updated. You should logout to restart your session."
            }
            CheckResult::Reboot => "System packages got updated. You should reboot your system!",
        }
    }
}

pub trait Check {
    fn check(&self) -> CheckResult;
}
