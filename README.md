[![CircleCI status](https://circleci.com/gh/rnestler/reboot-arch-btw.svg?style=shield)](https://circleci.com/gh/rnestler/reboot-arch-btw/tree/master)
Reboot Arch BTW
===============

This is a small utility which shows the installed and running Linux kernel on
[ArchLinux](https://www.archlinux.org). It is useful if one didn't notice that
the kernel got updated and suddenly your USB drive won't mount because the
needed kernel module can't get loaded.

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
 installed: 5.12.8.arch1.1 (since 2 minutes ago)
 running:   5.12.6.arch1.1
You should reboot arch btw!
Xorg server
 installed: 1.20.11.1 (since 51 days ago)
 running:   1.20.11
```

It will also show a [desktop
notification](https://wiki.archlinux.org/title/Desktop_notifications)
indicating that you probably want to reboot your system.
