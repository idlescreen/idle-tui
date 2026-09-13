# tui

Runtime configuration TUI for the IdleScreen daemon — live state, saver
picker, timeouts, inhibitors. Part of
[IdleScreen](https://idlescreen.github.io) — modular Wayland screensavers
for Linux.

The installed binary is **`idle-tui`**.

## Use

```sh
idlescreen tui    # or: idle-tui
```

## Develop

Path dependency: a `runtime/` checkout inside this repo (or a symlink to a
sibling clone) provides `idle-dbus` + `idle-runner`.

```sh
sudo dnf install libdbus-1-devel wayland-devel libxkbcommon-devel \
    fontconfig-devel freetype-devel openssl-devel libudev-devel \
    pkgconf-pkg-config                                       # apt: -dev names
git clone https://github.com/idlescreen/tui.git && cd tui
git clone https://github.com/idlescreen/runtime runtime    # path dep
cargo build && cargo test
```

## License

Apache-2.0 · © 2026 IdleScreen
