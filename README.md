# app-tui

Optional terminal UI for controlling the [IdleScreen](https://github.com/idlescreen/idle-core)
daemon (effect selection, enable/disable, status).

This is a **controller app** (`app-*`), not the core daemon. Platform products
(`app-cosmic`, etc.) may Recommend this package.

## Install

```bash
sudo dnf install app-tui
# binaries: app-tui (also idlescreen-tui / trance-tui for upgrades)
```

## Build

Requires a sibling checkout of idle-core (path dependencies):

```bash
git clone https://github.com/idlescreen/idle-core.git
git clone https://github.com/idlescreen/app-tui.git
cd app-tui
cargo build --release
```

## License

Apache-2.0.
