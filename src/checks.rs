#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum CheckResult {
    Nothing,
    Reboot,
}

pub trait Check {
    fn check(&self) -> CheckResult;
}
