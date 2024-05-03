# bin_eye_bt_receiver

A Quick and Dirty receiver app for Binary Eye's Forward scan via Bluetooth functionality Written in Rust.

You will need to install `sudo apt install libdbus-1-dev pkg-config libxdo-dev`
Currently only compatible with Linux runing Xorg.

```
Usage: bin_eye_bt_receiver [OPTIONS]
Options:
  -u, --uuid <UUID>  UUID of scaner [default: 8a8478c9-2ca8-404b-a0de-101f34ab71ae]
  -k, --keyboard     Keyboard output
  -h, --help         Print help
  -V, --version      Print version
```