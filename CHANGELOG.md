# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v0.6.0] - 2024-01-20

 * Print the cleaned kernel version
   ([#160](https://github.com/rnestler/reboot-arch-btw/pull/160))
 * Update dependencies

## [v0.5.7] - 2023-10-19

 * Update dependencies

## [v0.5.6] - 2023-07-27

 * Fix detection of the ck-generic- kernel variants
   ([#137](https://github.com/rnestler/reboot-arch-btw/pull/137))
 * Add more log messages regarding the kernel version
   ([#138](https://github.com/rnestler/reboot-arch-btw/pull/138))
 * Update dependencies

## [v0.5.5] - 2023-05-23

 * Update dependencies
 * Use Rust 2021 edition

## [v0.5.4] - 2023-01-14

 * Show variant of the running kernel in output
   ([#111](https://github.com/rnestler/reboot-arch-btw/pull/111)
 * Update dependencies

## [v0.5.3] - 2022-12-10

 * Make desktop notification timeout configurable
   ([#106](https://github.com/rnestler/reboot-arch-btw/pull/106))
 * Update dependencies

## [v0.5.2] - 2022-12-04

 * Update dependencies
 * Add logging support and replace some println! calls with log messages. This
   results in different stdout and stderr outputs.
   ([#98](https://github.com/rnestler/reboot-arch-btw/pull/98))
 * Refactor error handling. Most errors will now not panic, but log the error
   and continue (but not with the full functionality)
   ([#99](https://github.com/rnestler/reboot-arch-btw/pull/99)

## [v0.5.1] - 2022-11-11

 * Add amd- and intel-ucode packages to default reboot packages
   ([#89](https://github.com/rnestler/reboot-arch-btw/pull/89))
 * Update dependencies

## [v0.5.0] - 2022-10-25

 * Allow to also watch other packages which could make a reboot or restart of
   the session necessary.
   ([#78](https://github.com/rnestler/reboot-arch-btw/pull/78))
 * Upgrade to clap 4.0 this slightly changes the CLI interface, mostly the
   `--help` page.
   ([#82](https://github.com/rnestler/reboot-arch-btw/pull/82))

## [v0.4.0] - 2022-08-30

 * Update dependencies
 * Migrate from structopt to pure clap
   ([#71](https://github.com/rnestler/reboot-arch-btw/pull/71))

## [v0.3.4] - 2022-05-17

 * Update dependencies

## [v0.3.3] - 2022-01-17

 * Fix bug when the kernel patch version is 0
   ([#8](https://github.com/rnestler/reboot-arch-btw/issues/8) and
   [#60](https://github.com/rnestler/reboot-arch-btw/pull/60))
 * Update dependencies

## [v0.3.2] - 2021-10-03

 * Update dependencies
 * Add option to disable desktop notification

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
   I did a best out of 5 comparison for the runtime btw.
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

[Unreleased]: https://github.com/rnestler/reboot-arch-btw/compare/v0.6.0...master
[v0.6.0]: https://github.com/rnestler/reboot-arch-btw/compare/v0.5.7..v0.6.0
[v0.5.7]: https://github.com/rnestler/reboot-arch-btw/compare/v0.5.6..v0.5.7
[v0.5.6]: https://github.com/rnestler/reboot-arch-btw/compare/v0.5.5..v0.5.6
[v0.5.5]: https://github.com/rnestler/reboot-arch-btw/compare/v0.5.4..v0.5.5
[v0.5.4]: https://github.com/rnestler/reboot-arch-btw/compare/v0.5.3..v0.5.4
[v0.5.3]: https://github.com/rnestler/reboot-arch-btw/compare/v0.5.2..v0.5.3
[v0.5.2]: https://github.com/rnestler/reboot-arch-btw/compare/v0.5.1..v0.5.2
[v0.5.1]: https://github.com/rnestler/reboot-arch-btw/compare/v0.5.0..v0.5.1
[v0.5.0]: https://github.com/rnestler/reboot-arch-btw/compare/v0.4.0..v0.5.0
[v0.4.0]: https://github.com/rnestler/reboot-arch-btw/compare/v0.3.4..v0.4.0
[v0.3.4]: https://github.com/rnestler/reboot-arch-btw/compare/v0.3.3..v0.3.4
[v0.3.3]: https://github.com/rnestler/reboot-arch-btw/compare/v0.3.2..v0.3.3
[v0.3.2]: https://github.com/rnestler/reboot-arch-btw/compare/v0.3.1..v0.3.2
[v0.3.1]: https://github.com/rnestler/reboot-arch-btw/compare/v0.3.0..v0.3.1
[v0.3.0]: https://github.com/rnestler/reboot-arch-btw/compare/v0.2.1..v0.3.0
[v0.2.1]: https://github.com/rnestler/reboot-arch-btw/compare/v0.2.0..v0.2.1
[v0.2.0]: https://github.com/rnestler/reboot-arch-btw/compare/v0.1.3..v0.2.0
[v0.1.3]: https://github.com/rnestler/reboot-arch-btw/compare/v0.1.2..v0.1.3
[v0.1.2]: https://github.com/rnestler/reboot-arch-btw/compare/v0.1.1..v0.1.2
[v0.1.1]: https://github.com/rnestler/reboot-arch-btw/releases/tag/v0.1.1
