# idle-tui

Terminal UI client for IdleScreen. Talks to `idle-daemon` over D-Bus.

Website: [https://idlescreen.github.io](https://idlescreen.github.io)

### Launch

```bash
idlescreen tui    # preferred (from idle-cli)
idle-tui          # direct binary
```

Package: `idle-tui` (included by the main install script).

### Keybindings

| Key | Action |
|-----|--------|
| `Tab` | Switch panes (Dashboard / Savers / Settings) |
| `Space` / `Enter` | Toggle service or activate (depends on pane) |
| `↑` / `↓` | Navigate lists |
| `←` / `→` | Adjust timeout / scale on Dashboard where shown |
| `p` | Preview selected saver (Savers pane) |
| `c` | Install `idle-cosmic` when COSMIC is detected and the applet is missing |
| `q` / `Esc` | Quit |
