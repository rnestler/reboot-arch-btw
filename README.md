# Reboot Arch BTW

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/rnestler/reboot-arch-btw/ci.yml?branch=master)](https://github.com/rnestler/reboot-arch-btw/actions)
[![Crates.io Version](https://img.shields.io/crates/v/reboot-arch-btw.svg)](https://crates.io/crates/reboot-arch-btw)
[![Crates.io Downloads](https://img.shields.io/crates/d/reboot-arch-btw.svg)](https://crates.io/crates/reboot-arch-btw)
[![AUR version](https://img.shields.io/aur/version/reboot-arch-btw?label=AUR)](https://aur.archlinux.org/packages/reboot-arch-btw)

This is a small utility which shows the installed and running Linux kernel on
[ArchLinux](https://www.archlinux.org). It is useful if one didn't notice that
the kernel got updated and suddenly your USB drive won't mount because the
needed kernel module can't get loaded.

It can also detect if critical packages like systemd got updated which may also
make a reboot necessary.

To get the version of the installed kernel it uses libalpm (shipped with
pacman) to query the local pacman database. To get the version of the running
kernel it uses `uname -r`.

## Install

You may just install it from the AUR:
 * https://aur.archlinux.org/packages/reboot-arch-btw for the latest release
 * https://aur.archlinux.org/packages/reboot-arch-btw-git for the latest master

Alternatively one can install it with cargo:
```
cargo install reboot-arch-btw
```

## Build

This project requires the latest stable Rust version but may also be compatible
with older Rust versions. Also you need to have dbus installed.

```Shell
sudo pacman -S dbus
cargo build
```

## Usage

```Shell
$ reboot-arch-btw
Kernel
 installed: 5.19.13.arch1.1 (since 4 minutes ago)
 running:   5.19.12.arch1.1
systemd updated 4 minutes ago
Reboot arch btw
```

It will also show a [desktop
notification](https://wiki.archlinux.org/title/Desktop_notifications)
indicating that you probably want to reboot your system.

One can use `--reboot-packages` or `--reboot-packages` to set the list of
packages which should also trigger a notification if they are updated.

```
$ reboot-arch-btw --help
Check if a reboot is needed due to an updated kernel or other system packages.

Usage: reboot-arch-btw [OPTIONS]

Options:
      --disable-notification
          Disable desktop notification

      --notification-timeout <NOTIFICATION_TIMEOUT>
          Timeout for the desktop notification in milliseconds.

          * "default" will leave the timeout to be set by the server.

          * "never" or "0" will cause the notification never to expire.

          * Any other number will be interpreted as the timeout in milliseconds.

          [default: default]

      --reboot-packages <REBOOT_PACKAGES>
          Comma separated list of packages were we should reboot after an upgrade

          [default: systemd,linux-firmware,amd-ucode,intel-ucode]

      --session-restart-packages <SESSION_RESTART_PACKAGES>
          Comma separated list of packages were we should restart our session after an upgrade

          [default: xorg-server,xorg-xwayland]

  -v, --verbose
          Print kernel version info and show updated packages

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

### [Pacman Hook](https://wiki.archlinux.org/title/Pacman#Hooks)

You can configure `pacman` to run `reboot-arch-btw` after every upgrade to
check immediatly if you should reboot. For that create
`/etc/pacman.d/hooks/99-reboot-arch-btw.hook` with the following content:

```
[Trigger]
Operation = Upgrade
Type = Package
Target = *

[Action]
Description = Check whether a reboot is required
Depends = reboot-arch-btw
When = PostTransaction
Exec = /usr/bin/sudo -u $USER DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/$UID/bus /usr/bin/reboot-arch-btw
```

Note: You need to replace `$USER` and `$UID` with your actual username and user
ID.
