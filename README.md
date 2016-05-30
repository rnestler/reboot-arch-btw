[![Travis Build Status](https://travis-ci.org/rnestler/kernel-updated.svg?branch=master)](https://travis-ci.org/rnestler/kernel-updated)
Kernel Updated
==============

This is a small utility which shows the installed and running Linux kernel on
[ArchLinux](https://www.archlinux.org). It is useful if one didn't notice that
the kernel got updated and suddenly your USB drive won't mount because the
needed kernel module can't get loaded.

To get the version of the installed kernel it uses `pacman -Q linux` and to get
the version of the running kernel it uses `uname -r`.

Usage
-----

```Shell
% kernel-updated
installed: 4.5.4-1
running: 4.5.4-1
```

