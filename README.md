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
pkg install libxkbcommon wayland libX11 libXcursor libXrandr libXi libxcb mesa-libs vulkan-loader gtk3 fontconfig freetype2 alsa-lib
tar -xzf photocraft-<version>-freebsd-x86_64.tar.gz --strip-components 1 -C /usr/local
photocraft
```

Maintainers: [`docs/releasing.md`](docs/releasing.md) explains how releases are built, signed and published.

> [!IMPORTANT]
> **Status:** PhotoCraft is in early alpha, and we want to be straight about where it stands: much of Photoshop's feature surface exists in some form, but **it is not yet a Photoshop replacement for daily professional work**. The biggest gaps are AI/generative features, about twenty missing tools, depth in typography and pro workflows, and plug-in compatibility. Every Photoshop menu item is wired to a command ([`docs/parity.md`](docs/parity.md)), but that measures wiring, not behaviour. The honest, dimension-by-dimension picture and where we're going next are in the [roadmap's parity assessment](docs/roadmap.md#honest-parity-assessment-2026-10-05). Expect rough edges, and please file issues (include your OS, document size, layer count and a screenshot). You can also tell us what broke on [Discord](https://discord.gg/artcraft).

## Documentation

Developer, architecture, automation, format, and security documentation is maintained in the [PhotoCraft documentation book](book/).

## Security

Security architecture, threat modeling, parser hardening, fuzzing, and vulnerability reporting are covered in the [security documentation](book/src/security/) and the repository [security policy](SECURITY.md).

## Test corpora

PhotoCraft is tested against real files: our own Photoshop-authored oracle PSDs in
[photocraft-corpus](https://github.com/storytold/photocraft-corpus) plus the psd-tools, ag-psd and PngSuite sets, pinned and
sha256-verified. Fetch them with `cargo xtask corpus --all` and run the tests with
`cargo xtask test-corpus` (details in [docs/development.md](docs/development.md#test-corpora)).

## The Crafting Apps

PhotoCraft is one of the **Crafting Apps**: free, open-source creative tools from the
[ArtCraft](https://getartcraft.com/) team, each written from scratch in Rust and each able to
stand on its own.

| | App | What it's for | Code | Learn more |
|:-:|---|---|---|---|
| <img src="https://raw.githubusercontent.com/storytold/photocraft/main/assets/app-icon/hicolor/64x64/apps/ai.storyteller.photocraft.png" alt="" width="32" height="32"> | **PhotoCraft** | **Image editing: layers, masks, type and real PSD files · you are here** | [GitHub](https://github.com/storytold/photocraft) | [Website](https://getartcraft.com/apps/photocraft) |
| <img src="https://raw.githubusercontent.com/storytold/vectorcraft/main/assets/app-icon/hicolor/64x64/apps/ai.storyteller.vectorcraft.png" alt="" width="32" height="32"> | **VectorCraft** | Vector illustration | [GitHub](https://github.com/storytold/vectorcraft) | [Website](https://getartcraft.com/apps/vectorcraft) |
| <img src="https://raw.githubusercontent.com/storytold/filmcraft/main/assets/app-icon/hicolor/64x64/apps/ai.storyteller.filmcraft.png" alt="" width="32" height="32"> | **FilmCraft** | Video editing, color and sound | [GitHub](https://github.com/storytold/filmcraft) | [Website](https://getartcraft.com/apps/filmcraft) |
| <img src="https://raw.githubusercontent.com/storytold/lightcraft/main/assets/app-icon/hicolor/64x64/apps/ai.storyteller.lightcraft.png" alt="" width="32" height="32"> | **LightCraft** | Photo library and raw development | [GitHub](https://github.com/storytold/lightcraft) | [Website](https://getartcraft.com/apps/lightcraft) |
| <img src="https://raw.githubusercontent.com/storytold/printcraft/main/assets/app-icon/hicolor/64x64/apps/ai.storyteller.printcraft.png" alt="" width="32" height="32"> | **PrintCraft** | Reading, organizing and protecting PDFs | [GitHub](https://github.com/storytold/printcraft) | [Website](https://getartcraft.com/apps/printcraft) |
| <img src="https://raw.githubusercontent.com/storytold/effectcraft/main/assets/app-icon/hicolor/64x64/apps/ai.storyteller.effectcraft.png" alt="" width="32" height="32"> | **EffectCraft** | Motion graphics and visual effects | [GitHub](https://github.com/storytold/effectcraft) | [Website](https://getartcraft.com/apps/effectcraft) |
| <img src="https://raw.githubusercontent.com/storytold/designcraft/main/assets/app-icon/hicolor/64x64/apps/ai.storyteller.designcraft.png" alt="" width="32" height="32"> | **DesignCraft** | Page layout and publishing | [GitHub](https://github.com/storytold/designcraft) | [Website](https://getartcraft.com/apps/designcraft) |

And [**ArtCraft**](https://getartcraft.com/) itself, our AI image and video studio for artists who want real control.

The Crafting Apps share the same conventions: clean-room and pure Rust, native on macOS, Windows and Linux, in the browser via WebAssembly, and fully drivable by agents.

<br>

<p align="center">
  <a href="https://discord.gg/artcraft"><img alt="Join the ArtCraft community on Discord" src="https://img.shields.io/badge/Join%20us%20on%20Discord-5865F2?style=for-the-badge&logo=discord&logoColor=white" height="40"></a>
</p>

<h3 align="center">Come make things with us</h3>

<p align="center">
  Our Discord is where artists of every kind hang out: people who paint, shoot, draw, cut film,
  set type, and people still figuring out what they like to make. Share what you're working on,
  ask for help, tell us what's broken, or tell us what you wish these tools could do.
  Whatever your medium and however long you've been at it, you're welcome here.
</p>

<p align="center">
  <a href="https://discord.gg/artcraft"><b>discord.gg/artcraft</b></a> ·
  <a href="https://getartcraft.com/">getartcraft.com</a> ·
  <a href="https://getartcraft.com/apps">The Crafting Apps</a> ·
  <a href="https://getartcraft.com/apps/photocraft">PhotoCraft</a>
</p>

---

## License and credits

PhotoCraft is dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
Copyright (c) 2026 ArtCraft Team and the PhotoCraft contributors. Required notices are in [NOTICE](NOTICE).

Bundled fonts, icons, images and other assets keep their own open licenses; each one is listed
with its author, source and license in [ATTRIBUTION.md](ATTRIBUTION.md).

Every artwork shown is in the public domain (Wikimedia Commons, NASA, U.S. National Archives); sources are listed in [`docs/images/SOURCES.md`](docs/images/SOURCES.md).

The ArtCraft name, wordmark and logos in [`docs/brand/`](docs/brand/) are trademarks of the
ArtCraft Team and are not covered by this license. They may be used only unmodified, and only as
part of this repository and PhotoCraft, under [`docs/brand/LICENSE-brand.txt`](docs/brand/LICENSE-brand.txt).
Forks and modified versions must remove them.

<sub>Adobe, Photoshop, Illustrator, Premiere Pro, Lightroom, Acrobat, After Effects and InDesign are trademarks or registered trademarks of Adobe Inc. in the United States and/or other countries. PhotoCraft is an independent, open-source project and is not affiliated with, sponsored by or endorsed by Adobe Inc.; these names are used only to describe the workflows it is compatible with.</sub>

<p align="center">
  <a href="https://getartcraft.com/"><img alt="ArtCraft" src="docs/brand/artcraft-mark.svg" width="28"></a><br>
  <sub>Made by the <a href="https://getartcraft.com/">ArtCraft</a> team and community.</sub>
</p>


ArtCraft

## Star history

[![Star History Chart](https://api.star-history.com/svg?repos=storytold/photocraft&type=Date&legend=top-left)](https://www.star-history.com/?repos=storytold%2Fphotocraft&type=date&legend=top-left)
