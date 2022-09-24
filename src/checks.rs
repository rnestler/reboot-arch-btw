#[derive(PartialEq, Eq)]
pub enum CheckResult {
    Nothing,
    Reboot,
}

pub trait Check {
    fn check(&self) -> CheckResult;
}
