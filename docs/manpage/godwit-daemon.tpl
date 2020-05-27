= godwit-daemon(1)

== NAME

godwit-daemon - a daemon runner for Godwit.


== SYNOPSIS

*godwit-daemon* [_FLAGS_] [_OPERATION_] [_OPTIONS_]

*godwit-daemon* [_FLAGS_] [_OPTIONS_] --help

*godwit-daemon* [_FLAGS_] [_OPTIONS_] --version


== DESCRIPTION

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

For more information visit https://github.com/Passeriform/GodWit-Daemon/README.md


== EXIT STATUS

* `0` exit status occurs when a command works as expected (even in case of errors),
unless *--quiet* was given.
* `1` exit status occurs when a running thread failed before joining or the thread-group
failed itself. If a valid process is failing, feel free to file an issue
https://github.com/Passeriform/GodWit-Daemon/issues/new. This may have multiple implications
and affecting environmental factors. Thus, every bug report must contain complete
OS information at the point of failure.


== CONFIGURATION FILES

Configuration files can be found as:
    1.  ~/.godwit/*daemon.gwcore* - Configuration file containing godwit-daemon working settings and runner evironment options.
    2.  *~/.gdrc* - Configuration file identical to *daemon.gwcore* bar the state-related
    environments options, which will be ignored.


_Note:_

    1.  Both files are idiomatic(equal) and only one is picked up for configuration.
    2.  If *~/.gdrc* exists, *~/.godwit/daemon.gwcore* will be ignored.
    3.  Order of precedence for config files is: *.gdrc* > *daemon.gwcore*
    4.  If no config is specified, daemon will assume default options.


== SHELL COMPLETION

Shell completion files are included in the release tarball for Bash, Fish, Zsh
and PowerShell.

For *bash*, move *godwit.bash* to *$HOME/bash_completion*
or */etc/bash_completion.d/*.

For *fish*, move *godwit.fish* to *$HOME/.config/fish/completions*.

For *zsh*, move *godwit.zsh* to one of your *$fpath* directories.


== VERSION

{VERSION}


== HOMEPAGE

https://github.com/Passeriform/GodWit-Daemon

Please report bugs and feature requests in the issue tracker. Please do your
best to provide a reproducible test case for bugs. Include both the output of
*godwit-daemon status -vvv* and the command you ran with a *-vvv* flag.


== AUTHORS

Utkarsh Bhardwaj (Passeriform) <passeriform.ub@gmail.com>
