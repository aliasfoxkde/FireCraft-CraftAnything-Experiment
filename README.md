# FireCraft

**FireCraft - Open Source Raster and Vector Graphics Editor.** A clean-room, modern reimagining of
Adobe Fireworks CS6, built in Rust with egui/eframe. It combines raster and vector editing, web
graphics, animation states, live effects, MCP automation, and a portable desktop workflow.

> FireCraft is not affiliated with Adobe. It contains no Adobe code.

## Project direction (2026-11-26)

This repository is the **canonical FireCraft project**. The earlier
[FireCraft repository](https://github.com/aliasfoxkde/FireCraft) is retained as a historical
source/reference repository. Its useful capabilities, issues and engineering work are being
audited for selective migration into this codebase; the two application histories are not being
merged wholesale.

Start with the [experiment-first consolidation plan](docs/PRIMARY-REPO-RESET-AND-CONSOLIDATION-PLAN.md),
the [legacy migration register](docs/LEGACY-FIRECRAFT-MIGRATION-REGISTER.md), and the
[agent handoff](docs/AGENT-HANDOFF-2026-11-26.md). The current release and product direction
remain grounded in this repository's code, tests, tasks and release receipts.

## Download and try FireCraft

- **[Download the latest release — v0.3.7](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/releases/tag/v0.3.7)**
  for Windows, macOS, Linux, and WebAssembly. Linux packages include AppImage, DEB, and Flatpak.
- **[Open the v0.3.7 Web app on GitHub Pages](https://aliasfoxkde.github.io/FireCraft-CraftAnything-Experiment/)**;
  its served build marker matches source `5bfdcfd`.
- **[Open the Cloudflare review preview — v0.3.5](https://wasm-review-v0-3-5.firecraft-web.pages.dev/)**.
  This preview is older than the current release; the v0.3.7 Cloudflare deployment has not completed.
- Explore the companion [ArtCraft WebOS demo](https://artcraft-webos.pages.dev/) or browse the
  [documentation and tutorials](docs/README.md).

## Documentation

The [documentation map](docs/README.md) groups guides by purpose and links each topic to its
focused detail.

- **Product and releases:** [roadmap](docs/PRODUCT_ROADMAP.md), [changelog](.github/CHANGELOG.md),
  and [Windows 11 review checklist](docs/testing/windows-11-review-v0.3.7.md).
- **Learn:** [tutorials](docs/tutorials/README.md) and [keybindings](docs/tutorials/keybindings.md).
- **Feature guides:** [feature inventory](docs/features/README.md), [infinite canvas](docs/features/infinite-canvas.md),
  [Find & Replace](docs/features/find-replace.md), and [file browser](docs/features/file-browser.md).
- **Develop:** [architecture](docs/architecture.md), [build and release](docs/build-release.md),
  [contribution guide](.github/CONTRIBUTING.md), and [agent instructions](AGENTS.md).
- **Planning and evidence:** [active tasks](docs/TASKS.md), [audit](docs/AUDIT-2026-11-27.md),
  and [workflow documentation plan](docs/DOCUMENTATION-AND-WORKFLOW-EVIDENCE.md).

## What is in the box

- **CS6-style workspace**: 10 menus, tool palette, dual dock, collapsible/floating/snapping panel
  groups, multi-document tabs, Original/Preview/2-Up/4-Up, full-screen canvas.
- **36 tools**, including oval marquee, freeform vector paths, Replace Color, and Red Eye Removal.
- **God-view infinite canvas** that extends the page background, reveals off-page objects, and
  ghosts hidden layers while keeping flat exports page-bounded.
- **Find & Replace** for text/names **and colors** (fills, strokes, gradient stops, live-effect
  colors), with live match counts and Find Next.
- **Layered `.fw.png`** round-trip: normal PNG plus embedded project data for pages, layers,
  symbols, styles, and out-of-page objects.
- **Real Intelligence panel** using any OpenAI-compatible endpoint; it reports setup/errors instead
  of inventing output.
- **MCP JSON-RPC 2.0 server** and headless CLI (`info`, `export`, `convert`, `batch`, `apply`,
  `list-commands`).
- **In-app file browser** with editable address bar, compact type filters, and smooth dragging.
- **Windows integration**: AppUserModelID and best-effort Jump List for recent documents and tasks.

## Quick start

```sh
cd app
cargo test
cargo clippy --all-targets -- -D warnings
cargo run --release -- --demo
```

Windows release build:

```sh
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

The release profile uses LTO, one codegen unit, aborting panics, and stripped symbols. See
[Build & Release](docs/build-release.md) for the distribution layout and why FireCraft ships one
portable executable rather than a directory of DLLs.

## CLI examples

```sh
firecraft info --demo
firecraft export --demo showcase.svg
firecraft batch ./assets --format webp --out ./dist
firecraft list-commands
firecraft mcp --demo
```

## License

FireCraft is released under the [MIT License](LICENSE). It is a clean-room project and is not
affiliated with Adobe; see the trademark note at the end of the license.

## Repository conventions

- Domain vocabulary: [GLOSSARY.md](docs/GLOSSARY.md)
- Architecture decisions: [docs/adr/](docs/adr/)
- Reference research: [docs/research/](docs/research/)
- Agent guidance: [AGENTS.md](AGENTS.md) and [CLAUDE.md](CLAUDE.md)
- Do not commit build artifacts.
- Keep modules and docs focused; split by user goal before a file becomes a monolith.
