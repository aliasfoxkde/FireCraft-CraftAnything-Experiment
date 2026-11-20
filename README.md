# FireCraft — Craft Anything Experiment

FireCraft is a from-scratch reimplementation of **Adobe Fireworks CS6** as a modern
raster *and* vector editor: dark-first UI with automatic light/dark system theme
detection, high-resolution canvases, modern image formats (AVIF, HEIC, TIFF, WebP,
… ), and real vector support including SVG round-tripping.

This repository is the **active experiment**: the application lives in `app/`, and
it is built and checked by the [Craft Anything](#craft-anything) harness on every
iteration. It supersedes the earlier `FireCraft-RAE` attempt, from which the
reverse-engineering evidence in `investigation.cany/` was carried forward.

> Fireworks CS6 was dropped by Adobe in 2013. Its auth and setup servers are gone,
> the software is unreliable on modern platforms, and there is no supported way to
> extend it. FireCraft is a clean-room-parity replacement built from observed
> behaviour of the original binary.

---

## Status

| Area | State |
|------|-------|
| Layout / menus / panels / tabs | Parity pass in progress, visual similarity to the original still low |
| Document model, canvas, zoom, undo | Implemented, values traced to `Fireworks.exe` |
| Import / export formats | PNG, JPEG, GIF, WebP, TIFF, BMP, ICO implemented; SVG / PSD / AI in progress |
| Reverse-engineering corpus | ~250 Ghidra evidence shards in `investigation.cany/evidence/` |
| Test / CI coverage | Not yet established — see [Roadmap](#roadmap) |

---

## Layout

```
app/                     The FireCraft application (Rust + egui/eframe)
  src/                   Application source (18 modules)
  Cargo.toml             Crate manifest
  mechanics.json         Machine-readable value sheet, read by the harness
  MECHANICS.md           The same values with Fireworks.exe addresses as sources
  scratch/               One-off build scripts (on disk, not versioned)
  shots/                 Build verification screenshots (not versioned)
  target/                Cargo build output (not versioned, ~4.6 GB)

screenshots/             Fireworks CS6 reference imagery used for visual parity
reference/               The reference document image used by the visual check
capture/                 The captured live window of the running app
checks/                  Per-round verification screenshots from the harness
investigation.cany/      Reverse-engineering ledger: evidence, annotations, unknowns
docs/                    Project documentation
```

---

## Building

```sh
cd app
cargo build --release
```

The build host for the harness is a Fedora machine; the app itself is a Windows
desktop program and is built and run locally on Windows.

To capture a screenshot of the running window for the visual check:

```sh
cd app
cargo run -- --screenshot capture/window.png
```

---

## Mechanics — how parity is anchored

Every numeric default in FireCraft is traced to a function address in the original
`Fireworks.exe` (SHA-256 `ab9a267ffe6a9fd23f8f3f09446997c770effceeaae66babcb3f10d15433053f`).
The values live in two places, and they must agree:

- `app/MECHANICS.md` — human-readable, with the function address and evidence id.
- `app/mechanics.json` — machine-readable, read by Craft Anything after each build.

`app/mechanics.json` uses a strict schema; `version` is a number and every value
carries either a `source` or `"estimated": true`:

```json
{
  "version": 1,
  "values": [
    { "name": "default canvas width", "group": "canvas", "value": 800,
      "unit": "px", "source": "function 0x402050" }
  ]
}
```

---

## Craft Anything

The application is driven by the [Craft Anything](https://github.com/craft-anything)
harness, which runs the model in a loop: build → check → refine. Per round it
writes:

| File | What it holds |
|------|---------------|
| `craft.json` | Project brief, backend configuration, per-round check results |
| `feed.json` | The chat feed |
| `transcript.json` | The model's conversation |
| `build-activity.log` | What happened each round |
| `usage.json` | Token and cost totals |
| `investigation.cany/` | The reverse-engineering ledger the model maintains |
| `checks/round-N/` | The screenshots taken during that round's check |

These are runtime state and are **not** versioned — they change on every turn and
would otherwise add megabytes of churn per commit. The durable audit trail is
`investigation.cany/` and `checks/`.

The model backend is a local llama.cpp server reached over an OpenAI-compatible
HTTP API. Large prompts are the failure mode to watch: prefill runs at roughly
60 tokens/second, and the proxy in front of the server drops connections that
take longer than 180 seconds. Keep `context_tokens` in
`%APPDATA%\craft-anything\config.toml` low enough that a full turn can be prefilled
within that window.

### Git protocol

The harness commits and pushes after each chunk of work:

```
git add -A
git commit -m "type(scope): short description"
git push origin main
```

A local commit alone is not a backup — the work only exists once it is on the
remote. `INSTRUCTIONS.md` carries the same instruction for the model.

---

## Roadmap

- **Layout parity** — match the original's window proportions, menu structure,
  panel arrangement and pixel-level styling.
- **Formats** — SVG import/export with real vector fidelity; PSD and AI interchange.
- **Interop** — Adobe Bridge and Illustrator integration.
- **Quality gates** — a test suite, CI on GitHub Actions, and lint rules strict
  enough that regressions cannot land silently.

---

## License

FireCraft is an independent reimplementation. Adobe, Fireworks and Fireworks CS6
are trademarks of Adobe Inc. This project is not affiliated with or endorsed by
Adobe Inc.
