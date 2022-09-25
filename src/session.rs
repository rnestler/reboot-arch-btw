use time::OffsetDateTime;
use utmp_rs::UtmpEntry;

#[derive(Clone, Copy)]
pub struct SessionInfo {
    pub boot_time: OffsetDateTime,
    pub session_time: OffsetDateTime,
}

impl SessionInfo {
    pub fn from_utmp() -> Result<SessionInfo, String> {
        let entries = utmp_rs::parse_from_path("/var/run/utmp")
            .map_err(|err| format!("Could not read utmp: {}", err))?;
        Self::from_utmp_entries(&entries)
    }
    pub fn from_utmp_entries(utmp_entries: &[UtmpEntry]) -> Result<SessionInfo, String> {
        let mut boot_time = None;
        let mut session_time = None;
        for entry in utmp_entries {
            match entry {
                UtmpEntry::BootTime {
                    kernel_version: _,
                    time,
                } => boot_time = Some(time),
                UtmpEntry::UserProcess { time, .. } => {
                    // TODO: currently we just take the latest time. But we could also store it per
                    // user and try to notify all users. Also we should probably compare the time
                    // to at least find the newest login.
                    session_time = Some(time);
                }
                _ => {}
            }
        }
        Ok(SessionInfo {
            // TODO: Should we make this stuff optional and just print warnings?
            boot_time: *boot_time.ok_or_else(|| "No boot time available".to_owned())?,
            session_time: *session_time.ok_or_else(|| "No session time available".to_owned())?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use time::Duration;
    #[test]
    fn test_from_utmp_entries() {
        let now = OffsetDateTime::now_utc();
        let utmp_entries = [
            UtmpEntry::BootTime {
                kernel_version: "5.19.9-arch1-1".to_owned(),
                time: now.checked_sub(Duration::HOUR).unwrap(),
            },
            UtmpEntry::UserProcess {
                pid: 0,
                line: "tty1".to_owned(),
                user: "user".to_owned(),
                host: ":0".to_owned(),
                session: 0,
                time: now,
            },
        ];
        let session_info = SessionInfo::from_utmp_entries(&utmp_entries).unwrap();
        assert_eq!(
            session_info.boot_time,
            now.checked_sub(Duration::HOUR).unwrap()
        )
    }
}
