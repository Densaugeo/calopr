# calopr

Caddy Log Printer - Pretty-print Caddy's access logs 

[![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

## Supported Platforms

| Platform | Supported? |
|-|-|
| Linux | Yes | Tested on Fedora every release. |
| Windows | Yes | Occasional manual testing. Haven't noticed any obvious problems. |

## Installation

TBD

## Usage

```bash
# View last 10 entries and follow new ones
tail -n 10 /var/lib/caddy/access.log | calopr

# View entire log history in less. (-R enables colors and -S prevents line wrap)
cat /var/lib/caddy/access.log | calopr | less -RS
```

## Acknowledgements

Initial draft written by Hermes 3 405B Instruct.
