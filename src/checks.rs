#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum CheckResult {
    Nothing,
    RestartSession,
    Reboot,
}

pub trait Check {
    fn check(&self) -> CheckResult;
}
