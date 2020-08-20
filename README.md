# flathub-stats

[![github actions](https://github.com/ElXreno/flathub-stats/workflows/Flatpak%20build/badge.svg)](https://github.com/ElXreno/filesorter/actions)
[![github actions](https://github.com/ElXreno/flathub-stats/workflows/Rust/badge.svg)](https://github.com/ElXreno/filesorter/actions)

**Utility for fast grepping stats from Flathub.**

---

```bash
flathub-stats 0.2.0
ElXreno <elxreno@gmail.com>


USAGE:
    flathub-stats [FLAGS] [OPTIONS] [APPID] [SUBCOMMAND]

FLAGS:
    -f, --force         Override already downloaded stats
    -h, --help          Prints help information
    -i, --ignore-404    Ignore 404 status code
    -r, --refresh       Refreshes current stats cache
    -V, --version       Prints version information

OPTIONS:
    -e, --end-date <end-date>        End date
    -s, --start-date <start-date>    Start date

ARGS:
    <APPID>    Get stats by application ID

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    refresh    Refreshes current stats cache
```