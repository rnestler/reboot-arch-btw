# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

 * Update dependencies

## [v0.3.1] - 2021-08-15

 * Fix panic when unable to send desktop notification
 * Update alpm to 2.1

## [v0.3.0] - 2021-06-06

 * Update dependencies and bump minimal supported Rust version to 1.48.0
 * Update alpm to 2.0 to be compatible with pacman 6 / libalpm 13

## [v0.2.1] - 2020-05-23

 * Support non default kernels like linux-zen
 * Bump minimal supported Rust version to 1.42.0

## [v0.2.0] - 2020-05-12

 * Use libalpm instead of calling pacman btw.
   ([#32](https://github.com/rnestler/reboot-arch-btw/pull/32))
   Improves performance by almost a factor of two:
   ```bash
   # Version 0.1.3
   $ time /usr/bin/reboot-arch-btw
   Kernel
    installed: 5.6.11.1-1
    running:   5.6.11.1-1
   Xorg server
    installed: 1.20.8-2
    running:   1.20.8
   /usr/bin/reboot-arch-btw  0.00s user 0.02s system 100% cpu 0.021 total

   # Version 0.2.0
   $ time target/release/reboot-arch-btw
   Kernel
    installed: 5.6.11-arch1-1 (since 2 days ago)
    running:   5.6.11-arch1-1
   Xorg server
    installed: 1.20.8-2 (since 2 days ago)
    running:   1.20.8
   target/release/reboot-arch-btw  0.01s user 0.00s system 95% cpu 0.012 total
   ```
   I did a best out of 5 comparision for the runtime btw.
 * Do not panic if xdpyinfo is not available
   ([#31](https://github.com/rnestler/reboot-arch-btw/pull/31))

## [v0.1.3] - 2019-12-07

 * Adapt to new pacman output
   ([#29](https://github.com/rnestler/reboot-arch-btw/pull/29))

## [v0.1.2] - 2019-11-19

 * Fix detection of running kernel version
   ([#26](https://github.com/rnestler/reboot-arch-btw/pull/26))

## [v0.1.1] - 2019-10-15

 * First public release

[Unreleased]: https://github.com/rnestler/reboot-arch-btw/compare/v0.3.1...master
[v0.3.1]: https://github.com/rnestler/reboot-arch-btw/releases/tag/v0.3.0..v0.3.1
[v0.3.0]: https://github.com/rnestler/reboot-arch-btw/releases/tag/v0.2.1..v0.3.0
[v0.2.1]: https://github.com/rnestler/reboot-arch-btw/releases/tag/v0.2.0..v0.2.1
[v0.2.0]: https://github.com/rnestler/reboot-arch-btw/releases/tag/v0.1.3..v0.2.0
[v0.1.3]: https://github.com/rnestler/reboot-arch-btw/releases/tag/v0.1.2..v0.1.3
[v0.1.2]: https://github.com/rnestler/reboot-arch-btw/releases/tag/v0.1.1..v0.1.2
[v0.1.1]: https://github.com/rnestler/reboot-arch-btw/releases/tag/v0.1.1
