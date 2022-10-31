# oi

> Timer app with text input, made for humans

## Get started

### Manual installation

Build steps are intended for Linux systems, also for user spaces.
For anything else you'd have to build and install yourself.

1. clone this repo
2. run `make` to 
  - build `oi` and `oid`
  - place `oi` and `oid` into `/usr/local/bin`
  - create folder in `/usr/share/oi` to place sound effect

### Run

Project consists of 2 parts: the oid (daemon) and the client. Right
now we have one client, which you run from the cli, it's called `oi`,
its source code is located at `oi-cli`.

You have to run the daemon (oid), after which you'll be able to run
`oi run "<runic input>"` to start any timers. Check out the 
[runic](https://github.com/viktor-ku/runic) to know which input is possible.

Anytime you feel lost, run `oid --help` or `oi --help`.

#### Uninstall

1. run `make uninstall`
