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

## Start here

| I want to… | Go to |
|------------|-------|
| Build or package a release | [Build & Release](docs/build-release.md) |
| Understand the codebase | [Architecture](docs/architecture.md) |
| Browse the feature inventory | [Feature Surface](docs/features/README.md) |
| Work on the infinite/god-view canvas | [Infinite Canvas](docs/features/infinite-canvas.md) |
| Work on Find & Replace colors | [Find & Replace](docs/features/find-replace.md) |
| Work on the file browser | [File Browser](docs/features/file-browser.md) |
| See all documentation | [Documentation index](docs/README.md) |
| Check remaining work | [Tasks](docs/TASKS.md) |
| See the public product roadmap | [Product Roadmap](docs/PRODUCT_ROADMAP.md) |
| Explore extension ecosystem research and marketplace plan | [Fireworks Extensions Research](docs/research/fireworks-extensions.md) |
| See the measured state and gaps | [Audit](docs/AUDIT.md) |
| Read the current full audit snapshot | [2026-11-26 Audit](docs/AUDIT-2026-11-26.md) |
| Understand the plan and its exit criteria | [Plan](docs/PLAN.md) |
| Read the completed UI polish history | [Polish history](docs/ROADMAP.md) |
| Read release notes | [Changelog](.github/CHANGELOG.md) |
| Contribute, report a security issue, or read the community code | [Contributing](.github/CONTRIBUTING.md) · [Security](.github/SECURITY.md) · [Code of Conduct](.github/CODE_OF_CONDUCT.md) |

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
