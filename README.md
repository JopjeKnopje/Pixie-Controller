# psu-toggle

Small script based on the [ka3005p](https://github.com/Nicoretti/ka3005p) crate, to toggle a compatible PSU.
I mainly made this so I can have a widget in my [i3status-rs](https://github.com/greshake/i3status-rust) bar.

## State

Atm its very hacky and takes a second to refresh.
There is also a bug where sometimes the serial port is busy, this may occur after you've just changed state.
I still need to properly read the docx to make the icon update on every click instantly.

## i3status-rs config snippet

```toml
[[block]]
block = "custom"
command = "~/.local/bin/psu-toggle status"
interval = 1
[[block.click]]
button = "left"
cmd = "~/.local/bin/psu-toggle toggle"
```

## Upgrades

Because this version is absolte ass here are some improvements.

A standalone background service to send commands to using a cli. it will also keep the serial connection alive, which should make it respond wayyyy faster than it is now.

yea thats about it actually.

## TODO

- [x] Build system for multiple bins
- [ ] Rename bins to: `psu-service`, `psu-cli`.
- [x] Send IPC commands to server.
- [ ] Remove ambiguity around the type of socket we're using, enfore the UNIX specific one.
- [ ] Support output status
- [ ] Handle the interpretation of the output from the psu (the emoji for example) on the `cli` side.
- [ ] Wrap default commands?
- [ ] Keep listening for new connection when PSU power cycles.

### psu-cli

This will take in the exact same commands as the [ka3005p](https://docs.rs/ka3005p/latest/ka3005p/) crate. But instead of calling the serial port we wrap that and call the `psu-service` through IPC.

### psu-service

A daemon which listens for IPC, and runs the actual command through the serial port. Then passes the result back through IPC.
