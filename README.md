<p align="center">
  <img src="new_redesign/bigicon512.png" width="160" alt="Photoslop logo">
</p>

<h1 align="center">Photoslop</h1>

<p align="center">
  A native Rust image editor for artists on Linux and Wayland.<br>
  <b>English</b> · <a href="README.ru.md">Русский</a>
</p>

---

Photoslop is an independent fork of [PhotoCraft](https://github.com/storytold/photocraft) focused
on Linux and Wayland: digital painting, pen tablets, and the layer, mask and PSD workflow that
Photoshop users already know.

The project is in **early alpha**. The goal is a free Photoshop alternative for art on Linux; it is
not yet a replacement for professional work.

> **Naming.** The fork is called Photoslop and has its own logo (see
> [`new_redesign/`](new_redesign)). Inside the app, the binaries, package names and file IDs are
> still `photocraft` / **PhotoCraft** for now, so existing files, settings and packages keep
> working.

## Where the fork is going

- **Linux and Wayland:** running in a native session, UI scaling, input and desktop integration.
- **Artist tools:** brushes, a responsive canvas, pen pressure and tilt, comfortable layers and
  selections.
- **Compatibility:** editable documents, PSD exchange, several bit depths.
- **Reliability:** your work is safe and behaviour is predictable — that matters more than the
  number of menu items.

These are priorities, not a list of finished features. In particular, pen input through Wayland
`tablet-v2` is **not implemented yet**; tablet support on Linux currently uses X11/XInput2, so on
a Wayland desktop the pen works through XWayland. The engine's real state is tracked in the
[roadmap](docs/roadmap.md) and the [scorecard](docs/scorecard.md).

## What's inside

- Layers, masks, blend modes, adjustment layers and layer effects.
- Brushes, selections, transforms, type and vector shapes.
- PSD read/write, the native `.pcraft` format, and image export.
- A native egui/eframe UI rendered with wgpu (no Electron, no webview).
- One Rust engine shared by the app, the CLI and MCP automation.

Photoshop compatibility is incomplete. A command being in the menu doesn't guarantee identical
behaviour or byte-exact files.

## Download

Ready-made Linux builds (AppImage, `.tar.gz`, and `.deb`/`.rpm` when available) are on the
[Releases](https://github.com/hlophlopgaming/photoslop/releases) page.

```sh
chmod +x photocraft-*-linux-x86_64.AppImage
./photocraft-*-linux-x86_64.AppImage
```

## Building from source

You need Rust stable **1.95 or newer** ([rustup](https://rustup.rs)) and a few system libraries.

```sh
# Debian / Ubuntu
sudo apt install build-essential pkg-config libxkbcommon-dev libwayland-dev \
  libx11-dev libxrandr-dev libxi-dev libgl1-mesa-dev libgtk-3-dev

# Fedora
sudo dnf install gcc pkgconf-pkg-config libxkbcommon-devel wayland-devel \
  libX11-devel libXrandr-devel libXi-devel mesa-libGL-devel gtk3-devel

# Arch
sudo pacman -S --needed base-devel pkgconf libxkbcommon wayland \
  libx11 libxrandr libxi mesa gtk3
```

Then, from the repository root:

```sh
cargo run --release -p photocraft                       # start the app
cargo run --release -p photocraft -- path/to/image.psd  # open a file on start
```

Run it from your Wayland session to test it there. Other platforms and the web build are
inherited from upstream; this fork focuses on Linux. Fonts, diagnostics and more:
[developer guide](docs/development.md).

## Build and release in one command

[`scripts/release.sh`](scripts/release.sh) builds the Linux packages and publishes a GitHub
Release:

```sh
scripts/release.sh 0.4.0        # bump to 0.4.0, build, tag v0.4.0, push, publish the release
scripts/release.sh              # release the version already in Cargo.toml
scripts/release.sh --build-only # just build the packages into dist/release/
```

It bumps the version (`Cargo.toml` + `Cargo.lock`) and commits it, builds the AppImage and
`.tar.gz` (plus `.deb`/`.rpm` if [nfpm](https://nfpm.goreleaser.com) is installed), writes
`SHA256SUMS.txt`, creates the `v<version>` tag, pushes, and uploads everything with the
[GitHub CLI](https://cli.github.com). Versions like `0.4.0-rc.1` become pre-releases.

Requirements: a clean working tree, `gh auth login` done once, and the build dependencies above.
`scripts/release.sh --help` lists every option (`--draft`, `--formats`, `--yes`).

## Development

```sh
cargo test -p photocraft-ui-egui
cargo xtask layers
cargo xtask wasm
```

- [Architecture and workspace layout](docs/architecture.md)
- [Contributing rules](docs/contributing.md)
- [Driving the app and taking screenshots](docs/control-protocol.md)
- [UI design](docs/ui-design.md)
- [Upstream roadmap](docs/roadmap.md)

When reporting a bug, include your distro, Wayland compositor or X11, GPU, tablet model (for pen
issues), steps to reproduce, and the document size.

## Origin and license

Photoslop is a fork of the [original PhotoCraft](https://github.com/storytold/photocraft).
Upstream's proprietary logos and promotional links have been removed; the new Photoslop logo lives
in [`new_redesign/`](new_redesign).

The code is available under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
Upstream copyright notices are kept in [NOTICE](NOTICE) and the license files. Other assets are
listed in [ATTRIBUTION.md](ATTRIBUTION.md).
