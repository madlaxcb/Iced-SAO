# Iced-SAO

SAO-style UI component library built on [iced](https://github.com/iced-rs/iced) 0.14, targeting Windows 10 22H2 / 11. Developed on Linux with MSVC cross-compilation; visual behavior is verified on a real Windows machine.

[中文文档](README.zh-CN.md)

## Highlights

- **Token-driven theming** — colors / radii / spacing / shadows / motion durations all live in `orb-tokens`, hot-reloadable from TOML; widgets are forbidden from hardcoding literal values
- **G1 pseudo-frosted glass** panels, each theme ships with an opaque fallback mode
- **Deterministic animation** — tween / sequence / stagger driven by an injectable manual clock, fully unit-testable
- **SAO signature widgets** — circular menu rail, HP bar with band thresholds, loop scroll, rings
- **Offline-reproducible builds** — vendored dependencies plus `cargo xtask` automation (`ci` / `win-pack` / `dist` / `doctor` / `vendor`)

## Workspace

```
code/
├─ crates/  orb-tokens → orb-core → orb-theme → orb-widgets (+ orb-icons / orb-platform / orb-testkit)
├─ apps/    gallery (component showcase), sample-launcher (demo app)
└─ xtask/   local build automation
```

Dependency direction is strictly one-way: `tokens → core → theme → widgets`.

## Getting started

```sh
# offline build (deps are vendored locally, not committed)
cd code
cargo build --offline --locked
cargo run --offline -p gallery

# full gate: fmt + dual-target clippy + tests + Windows release build
cargo xtask ci
```

Windows cross-build from a Linux host:

```sh
cargo xwin build --offline --release --target x86_64-pc-windows-msvc -p gallery
```

## Documentation

- `doc/00-总览.md` — project overview and current status
- `doc/design/` — token spec, component API catalog, Windows verification checklist
- `doc/guides/` — how to add a new component / new theme
- `doc/adr/` — architecture decision records (001–006)

## Status

Milestones M0–M6 are complete; distribution packaging (M7) is automated via `cargo xtask dist`. Windows checkpoints W2–W5 are executed on a dedicated verify machine; anything not verified there is explicitly labeled "unverified".

Prebuilt Windows x64 binaries are published on the [Releases](https://github.com/madlaxcb/Iced-SAO/releases) page with SHA-256 checksums and third-party license notes.
