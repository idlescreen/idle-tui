# AGENT.md — idle-tui

- Strict Rust, Apache-2.0.
- Max 256 lines per `.rs` file.
- Zero production unwrap/expect.
- Path-deps: `idle/crates/idle-dbus` and `idle/idle-runner` (symlink or checkout `idle` monorepo as `./idle`).
- Product binary/package: `idle-tui` only.
- Default branch: master.

## Process kit

This repo follows the global process kit — local copies of the always-on process documents live next to this file:

- `AXIOMS.md` — always-on axioms (hygiene, security, entropy)
- `OODA.md` — Observe/Orient/Decide/Act rotation
- `PROBE.md` — assumption-hunt protocol

