[![Travis Build Status](https://travis-ci.org/rnestler/kernel-updated.svg?branch=master)](https://travis-ci.org/rnestler/kernel-updated)
Kernel Updated
==============

This is a small utility which shows the installed and running Linux kernel on
[ArchLinux](https://www.archlinux.org). It is useful if one didn't notice that
the kernel got updated and suddenly your USB drive won't mount because the
needed kernel module can't get loaded.

To get the version of the installed kernel it uses `pacman -Q linux` and to get
the version of the running kernel it uses `uname -r`.

Build
-----

This project requires Rust 1.18.0 or newer. Also you need to have
libdbus-glib-1-dev installed.

```Shell
sudo apt install libdbus-glib-1-dev
cargo build
```

Usage
-----

```Shell
% kernel-updated
installed: 4.5.4-1
running: 4.5.4-1
```

