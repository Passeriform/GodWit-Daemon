# GodWit Daemon

[![crate](https://img.shields.io/crates/v/godwit-daemon)](https://crates.io/crates/godwit-daemon)
[![docs](https://docs.rs/godwit-daemon/badge.svg)](https://docs.rs/godwit-daemon)
[![build](https://travis-ci.org/Passeriform/GodWit-Daemon.svg?branch=master)](https://travis-ci.org/Passeriform/GodWit-Daemon)
[![codecov](https://codecov.io/gh/Passeriform/GodWit-Daemon/branch/master/graph/badge.svg)](https://codecov.io/gh/Passeriform/GodWit-Daemon)
[![maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://github.com/Passeriform/GodWit-Daemon/graphs/commit-activity)

[![homepage](https://img.shields.io/website-up-down-green-red/http/passeriform.com.svg?label=Passeriform)](http://www.passeriform.com/prod/GodWit/daemon)
[![repo](https://img.shields.io/badge/github-GodWit--Daemon-blue?logo=github)](https://github.com/Passeriform/GodWit-Daemon)

A daemon runner for [GodWit](https://github.com/Passeriform/GodWit).

Godwit-daemon is a runner for managing godwit processes. This utility is implicitly
used by godwit itself. The utility is exposed for developers/power-users to create
their own projects, but for a normal user something better might already exist on
the godwit command itself as a plugin.

For developers, this utility should be considered as a *crucial botch*. A project
made using this utility is also recommended to have a plugin-like front usable by
godwit. The intention is to keep godwit as a *complete* cover over the daemon.

The "daemon" etymology does not represent daemons in conventional sense. The utility
considers any process running on managed thread as a daemon. Unlike daemons though,
the runner never detaches/loses focus of the process at hand. Thus, it's better to
look at the utility itself as a daemon, which doesn't follow written states like
godwit.


## Install
[![release](https://img.shields.io/github/release/Passeriform/GodWit-Daemon.svg)](https://github.com/Passeriform/GodWit-Daemon/releases/)
[![aur](https://img.shields.io/aur/version/godwit-daemon)](https://aur.archlinux.org/packages/godwit-daemon)
[![debian](https://img.shields.io/debian/v/godwit-daemon)](https://packages.debian.org/unstable/utils/godwit-daemon)
[![homebrew](https://img.shields.io/homebrew/v/godwit-daemon)](https://formulae.brew.sh/formula/godwit-daemon)

The daemon is bundled with GodWit and can be installed as a dependency. If only the daemon is required, it is alse available on AUR and PPA repositories as a separate package.

Simply install it using
```bash
$ yay -Syu godwit-daemon
```
OR
```bash
$ sudo add-apt-repository ppa:passeriform/ppa
$ sudo apt update
$ sudo apt install godwit-daemon
```

## Use
Start by creating a new tracer.

```bash
$ godwit-daemon new trace <application>
```

To store the trace to log.

```bash
$ godwit-daemon new trace <application> > my.trace
```

To run process through trace pruning.
```bash
# The file my.trace will continuously update as steps are pruned.
$ godwit-daemon regress trace <application>
 > my.trace
```

To kill all associated nodes and drop resources.
```bash
$ godwit-daemon die
```

## Contributing
[![issues](https://img.shields.io/github/issues/Passeriform/GodWit-Daemon.svg)](https://gitHub.com/Passeriform/GodWit-Daemon/issues/)
[![pull-requests](https://img.shields.io/github/issues-pr/Passeriform/GodWit-Daemon)](https://github.com/Passeriform/GodWit-Daemon/pulls)

If you want to contribute, start by cloning this repository.
```bash
    git clone https://github.com/Passeriform/GodWit Godwit-Daemon
```
Checkout to a new branch.
```bash
    # Use kebab-case categorization format.


    # Ex: A new feature.
    git checkout feature-<feature>

    # Ex: A bugfix.
    git checkout bugfix-<bug>

    # Ex: A meta update.
    git checkout meta-<title>

    # Ex: A documentation update.
    git checkout docs-<title>

    # Ex: A CI update.
    git checkout ci-<title>

```
Do your thing...

```bash
Code up
```
**Squash commits** and issue a PR at
[https://github.com/Passeriform/GodWit-Daemon](https://github.com/Passeriform/GodWit-Daemon)

## License
![license](https://img.shields.io/crates/l/godwit-daemon)

Licensed under either of

 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

### Credition

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
