# flathub-stats

[![github actions](https://github.com/ElXreno/flathub-stats/workflows/Flatpak%20build/badge.svg)](https://github.com/ElXreno/filesorter/actions)
[![github actions](https://github.com/ElXreno/flathub-stats/workflows/Rust/badge.svg)](https://github.com/ElXreno/filesorter/actions)

**Utility for fast grepping stats from Flathub.**

---

```bash
flathub-stats 0.2.1
ElXreno <elxreno@gmail.com>


USAGE:
    flathub-stats [FLAGS] [OPTIONS] [APP-ID] [SUBCOMMAND]

FLAGS:
    -d, --disable-refresh         Don't refresh current stats cache
    -f, --force-refresh           Override already cached stats
    -h, --help                    Prints help information
    -i, --disable-404-ignoring    Disable 404 code ignoring
    -a, --show-all                Show stats for all days (by default shows only for 180 days)
    -V, --version                 Prints version information

OPTIONS:
    -e, --end-date <end-date>        End date (default is today)
    -s, --start-date <start-date>    Start date (default is 2018/04/29 if --show-all is present)

ARGS:
    <APP-ID>    Get stats by application ID

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    refresh    Refreshes current stats cache
```