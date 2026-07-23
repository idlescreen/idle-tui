# idle-tui

Optional terminal UI for controlling the [IdleScreen](https://github.com/idlescreen/idle-core)
daemon (effect selection, enable/disable, status).

This is an **engine/frontend**, not a platform app. Platform products
(`app-cosmic`, etc.) may Recommend this package.

## Status

Extracted from `idle-core`. Binary and Debian package name remain **`trance-tui`**
for install continuity.

## Build

Requires a sibling checkout of `idle-core` (path dependencies):

```bash
git clone https://github.com/idlescreen/idle-core.git
git clone https://github.com/idlescreen/idle-tui.git
cd idle-tui
cargo build --release
```

## Run

```bash
# daemon must be available on D-Bus
trance-tui
```

## Install

After adding the [packages](https://idlescreen.github.io/packages/) apt/dnf source:

```bash
sudo apt install trance-tui
# or: sudo dnf install trance-tui
```

## Related

| Repo | Role |
|------|------|
| [idle-core](https://github.com/idlescreen/idle-core) | Daemon + CLI |
| [app-cosmic](https://github.com/idlescreen/app-cosmic) | COSMIC app |
| [packages](https://github.com/idlescreen/packages) | Package host |

## License

Apache-2.0.
