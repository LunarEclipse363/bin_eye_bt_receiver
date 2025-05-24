# bin_eye_bt_receiver

A Quick and Dirty receiver app for Binary Eye's Forward scan via Bluetooth functionality Written in Rust.

You will need to install:
```bash
sudo apt install libdbus-1-dev pkg-config libxdo-dev
```

Compatibility on Linux with regards to keyboard emulation is as follows:

| Desktop Type | Supported  | Requirements                                             |
|--------------|------------|----------------------------------------------------------|
| Xorg/X11     | Yes        | `xdotool`                                                |
| Wayland      | Yes\*      | compositor supporting the [virtual_keyboard_v1] protocol |

\*as of 2025-05-25, neither KDE (KWin 6.3) nor GNOME (Mutter 48.0) support the aforementioned Wayland protocol.

```
Usage: bin_eye_bt_receiver [OPTIONS]
Options:
  -u, --uuid <UUID>  UUID of scaner [default: 8a8478c9-2ca8-404b-a0de-101f34ab71ae]
  -k, --keyboard     Keyboard output
  -h, --help         Print help
  -V, --version      Print version
```

[virtual_keyboard_v1]: https://wayland.app/protocols/virtual-keyboard-unstable-v1#compositor-support
