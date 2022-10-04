Reboot Arch BTW
===============

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/rnestler/reboot-arch-btw/CI)](https://github.com/rnestler/reboot-arch-btw/actions)
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

Install
-------

You may just install it from the AUR:
 * https://aur.archlinux.org/packages/reboot-arch-btw for the latest release
 * https://aur.archlinux.org/packages/reboot-arch-btw-git for the latest master

Alternatively one can install it with cargo:
```
cargo install reboot-arch-btw
```

Build
-----

This project requires Rust 1.63.0 or newer. Also you need to have dbus
installed.

```Shell
sudo pacman -S dbus
cargo build
```

Usage
-----

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
