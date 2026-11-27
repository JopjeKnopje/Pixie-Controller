# Pixie Controller

Software to [control](https://github.com/Nicoretti/ka3005p) a [Korad](https://www.eleshop.nl/korad-ka3005d-power-supply.html) Power supply, running as a systemd service to keep the serial connection alive. It listens to [IPC](https://docs.rs/interprocess/latest/interprocess/) calls for commands from the `pixie-cli`.

My power supply is sitting slightly too far away from my lazy ass. So I wanted to have a widget in my [i3status-rs](https://github.com/greshake/i3status-rust) bar to toggle the output.
While I was writing this project I realized having a seperate service also opens up the possiblity for some cool embedded integration-tests.

> [!CAUTION]
> This repo contains the worst rust code you will ever read, I cobbled this together in one evening, but will def improve on it.

## i3status-rs config snippet

```toml
[[block]]
block = "custom"
command = "~/.local/bin/pixie-cli status"
interval = 1
[[block.click]]
button = "left"
cmd = "~/.local/bin/pixie-cli toggle"
```

The bar only refreshes every 1 sec, and I haven't found a way to make it faster :(

## Installation

Currently there is no installer script or anything like that, so here I'll list the manual steps to get it up and running.

Files in the right place.

```bash
# this will copy the binaries into your ~/.local/bin.
just install
# create a local ~/.local/share/systemd/user/ and copy the service into it.
just service
```

Systemd commands.

```bash
# enable the service
systemctl --user enable pixie-controller
# start it
systemctl --user start pixie-controller


# check status
systemctl --user status pixie-controller
# tail the logs
journalctl --user -u pixie-controller -f
```

## Toolchain

- prek, just, cargo (duhh), text-editor, keyboard, 0.5L mate, monitor (I use a 12 inch 4:3, but you could go smaller), [lungs](https://en.wikipedia.org/wiki/Lung) (gotta keep breathing)

## TODO

- [x] Build system for multiple bins
- [ ] Rename bins to: `pixie-controller`, `pixie-cli`.
- [x] Send IPC commands to server.
- [ ] Remove ambiguity around the type of socket we're using, enfore the UNIX specific one.
- [ ] Support output status
- [ ] Handle the interpretation of the output from the psu (the emoji for example) on the `cli` side.
- [ ] Wrap default commands?
- [ ] `pixie-cli` error when `pixie-controller` is not online.
- [ ] Keep listening for new connection when PSU power cycles.
- [ ] Improve logging especially in the `pixie-controller`.
- [ ] in `pixie-cli` print to stdout.
- [ ] Improve code A LOT and release to AUR.
- [ ] Config file?

### pixie-cli

This will take in the exact same commands as the [ka3005p](https://docs.rs/ka3005p/latest/ka3005p/) crate. But instead of calling the serial port we wrap that and call the `pixie-controller` through IPC.

### pixie-controller

A daemon which listens for IPC, and runs the actual command through the serial port. Then passes the result back through IPC.
