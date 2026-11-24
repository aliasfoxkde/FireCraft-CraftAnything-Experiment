# FireCraft

**A clean-room, modern reimagining of Adobe Fireworks CS6** — hybrid vector and bitmap editing,
web graphics, animation states, live effects, MCP automation, and a portable desktop workflow.
Built in Rust with egui/eframe.

> FireCraft is not affiliated with Adobe. It contains no Adobe code.

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
| Check remaining work | [Roadmap](ROADMAP.md) |
| Read release notes | [Changelog](CHANGELOG.md) |

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
cargo test -- --test-threads=1
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

## Repository conventions

- Domain vocabulary: [GLOSSARY.md](GLOSSARY.md)
- Architecture decisions: [docs/adr/](docs/adr/)
- Do not commit build artifacts.
- Keep modules and docs focused; split by user goal before a file becomes a monolith.
