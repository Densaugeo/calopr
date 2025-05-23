# calopr

Caddy Log Printer - Pretty-print Caddy's access logs 

[![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

## Supported Platforms

| Platform | Supported? | Notes |
|-|-|-|
| Linux | Yes | Tested on Fedora every release. |
| Windows | Yes | Occasional manual testing. Haven't noticed any obvious problems. |
| Mac | No | |
| x64 | Yes | Tested every release. |
| ARM | Yes | Occasional manual testing. Haven't noticed any obvious problems. |

## Installation

Linux (x64):

```bash
sudo curl -L -o /usr/bin/calopr https://github.com/Densaugeo/calopr/releases/latest/download/calopr-x64-linux
sudo chmod 755 /usr/bin/calopr
```

For Linux on ARM, see https://github.com/Densaugeo/calopr/releases/ for available ARM builds. They can be installed in the same way with the associated filename.

Windows:

Download the latest .exe from https://github.com/Densaugeo/calopr/releases/ and save it somewhere you can find it.

## Usage

```bash
# Simplest way to view logs. Even works on Windows (with an appropriate path)
cat /var/lib/caddy/access.log | calopr

# View last 10 entries and follow new ones
tail -n 10 /var/lib/caddy/access.log | calopr

# View entire log history in less. (-R enables colors and -S prevents line wrap)
cat /var/lib/caddy/access.log | calopr | less -RS
```

## Acknowledgements

Initial draft written by Hermes 3 405B Instruct.
