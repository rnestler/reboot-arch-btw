# Improvement Ideas for reboot-arch-btw

This document collects potential improvements and new features for the
`reboot-arch-btw` project.

---

## 1. Address Existing TODOs in `session.rs`

There are two open TODOs in `src/session.rs`:

- **Newest session detection**: The code currently takes the *last*
  `UserProcess` entry it sees, not necessarily the *newest*. This can be
  incorrect on multi-user or multi-session systems. We should compare times to
  find the newest login.
- **Graceful degradation**: Missing boot/session times currently cause hard
  failures (`anyhow!`). Instead, the tool could log a warning and skip
  session-related checks while still performing the kernel check.

## 2. Machine-Readable Output & Exit Codes

- **JSON / formatted output**: Add `--json` or `--format` flags so users can
  integrate the output into status bars (polybar, waybar, eww, custom scripts,
  etc.).
- ✅ **Meaningful exit codes**: Implemented via the `--exit-code` flag. Returns
  `0` = nothing, `1` = reboot needed, `2` = session restart needed, `64` = CLI
  usage error. Regardless of `--exit-code`, unexpected errors that cause a panic
  still exit with code `101`.

## 3. Configuration File Support

Right now, users rely on shell aliases to persist custom package lists. Adding
support for a config file (e.g.
`~/.config/reboot-arch-btw/config.toml`) would let users set defaults for:

- `reboot_packages`
- `session_restart_packages`
- `notification_timeout`
- `disable_notification`
- Preferred output format

## 4. Enhanced Reporting in Verbose Mode

- ✅ **List all updated packages**: Implemented — in verbose mode all matching
  updated packages are now reported instead of only the first one.
- **Urgency levels in notifications**: Set `notify-rust` urgency to `Critical`
  for kernel updates, `Normal` for reboots, and `Low` for session restarts.

## 5. Watch / Polling Mode

Add a `--watch` or `--interval <SECONDS>` mode that runs continuously and emits
a notification *only when the state changes* from "all good" to "action needed".
This is useful for users who don't want to use a pacman hook but still want
timely alerts.

## 6. Systemd Integration Examples

Provide example systemd user service/timer files (e.g. in `contrib/systemd/`)
so users can easily set up periodic checks via `systemd --user` instead of (or
in addition to) pacman hooks.

## 7. CI & Maintenance Improvements

- Add `cargo audit` to the GitHub Actions workflow to catch security
  vulnerabilities in dependencies.
- Add a `cargo publish --dry-run` or `cargo package` step to catch packaging
  regressions before release.
- Consider a scheduled CI run (e.g. weekly) in addition to push/PR triggers.

## 8. Error Handling Hardening

A few places use `.expect()` (e.g. opening the pacman database, cleaning the
kernel version). Some of these could be converted to graceful errors so the
tool doesn't panic in edge cases.
