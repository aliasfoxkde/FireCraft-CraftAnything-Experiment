# CLAUDE.md — working agreement

Read this before changing anything. It is the entry point for agents; deeper material lives in
[docs/](docs/README.md).

FireCraft is a clean-room Rust/egui reimplementation of Adobe Fireworks CS6: one native binary plus
a WASM build. Domain vocabulary is defined in [GLOSSARY.md](docs/GLOSSARY.md) — use those names in code,
UI copy, commits and docs.

## Where the truth lives

| Question | File |
|----------|------|
| What is open right now? | [docs/TASKS.md](docs/TASKS.md) |
| Why this order, and what "done" means? | [docs/PLAN.md](docs/PLAN.md) |
| What is actually measured? | [docs/AUDIT.md](docs/AUDIT.md) |
| What changed, when? | [docs/PROGRESS.md](docs/PROGRESS.md) |
| Completed polish pass and manual backlog | [ROADMAP.md](docs/ROADMAP.md) |
| Module map, seams, invariants | [docs/architecture.md](docs/architecture.md) |
| Build, package, release | [docs/build-release.md](docs/build-release.md) |
| Feature behavior and non-regression rules | [docs/features/](docs/features/) |
| Decisions not to re-litigate | [docs/adr/](docs/adr/) |

Do not create a second task list. New work goes into `docs/TASKS.md`; completed work moves to
`docs/PROGRESS.md` and, if it is a milestone, `.github/CHANGELOG.md`.

## Invariants that cause silent bugs

1. **`FireCraft.doc` is the single source of truth.** Call `sync_active_doc_slot()` before switching,
   closing, saving all, or opening another document. Selection indices are layer-local: changing
   `selected_layer` requires a selection reset or an intentional re-selection.
2. **God view never changes export semantics.** Serialization must not read `infinite_canvas`.
3. **Shortcut dispatch gates on `ctx.text_edit_focused()`**, never `egui_wants_keyboard_input()`.
4. **Dialogs own layout; app methods own effects.** Replace-all, save-all-and-quit, open, export are
   app methods so they are testable without an egui frame.
5. **Platform code is isolated and optional.** `shell_windows.rs` is Windows-only; every COM failure
   is logged and ignored; startup never depends on it.
6. **One implementation per behavior.** No duplicate controls, parallel collapse logic, or second
   state transporter.
7. **No placeholders, stubs, simulated data or fabricated integrations.** An unconfigured
   Intelligence/MCP panel says so and offers setup. Planning docs are the only place for example
   code.

Reversing an ADR requires a new ADR that explains the cost.

## Commands

All cargo commands run from `app/`.

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo check --target wasm32-unknown-unknown
cargo llvm-cov --summary-only -- --test-threads=1   # coverage baseline 18.70 % on 2026-11-25
cargo audit
cargo machete
```

GUI smoke work (only when asked):

```sh
cargo run --release -- --demo
cargo run -- --screenshot /tmp/shot.png          # headless capture, exits 0
# Xvfb + software GL is required on this host; no display is available otherwise.
```

The local `cargo` is wrapped by a host admission gate: concurrent cargo invocations queue with
"waiting for host build admission". Run one at a time and expect gate messages on stderr.

## Testing rules

- Put tests at the behavior seam (model or app method), not on pixel output, unless the pixel output
  *is* the behavior.
- Prefer the existing `DocumentFixture` intent-level style over re-deriving setup.
- Never open the OS clipboard in tests; it can block headless X11 indefinitely.
- `--test-threads=1` is no longer required (parallel passes in <1 s) but is harmless.
- A visual or Windows-only change you could not run is **unverified**. Say so plainly; do not
  describe it as working.

## Change protocol

1. Pick the next open task in `docs/TASKS.md`; keep IDs stable.
2. Keep the tree green. If a gate fails for a pre-existing reason, fix the cause — do not disable,
   skip or `#[allow]` the check to make progress. Any intentional lint suppression carries a comment
   explaining why.
3. Update the affected doc in the same change. A claim without evidence is a bug.
4. Commit with a conventional message and add a `docs/PROGRESS.md` line: what changed, gate results,
   what remains unverified.
5. Push only when the user asks. Instructions files that say "always commit and push" describe the
   owner's workflow, not standing authorization to publish.

## Known traps

- `screenshots/` is read at compile time by `dialogs/mod.rs` (splash asset); moving or deleting it
  breaks the build outside the crate. Fix is task 13.10.
- `release.yml` hardcodes `v0.2.0` artifact names while `Cargo.toml` says `0.2.0` and `.github/CHANGELOG.md`
  says `0.2.1`. Version is not single-sourced yet (task 13.2).
- `app/index.html` and the inline HTML in `release.yml` are two copies of the web entry point
  (task 13.7).
- `docs/ROADMAP.md` marks phases complete while listing their verification as unchecked; treat unchecked
  verify items as open defects, tracked in `docs/TASKS.md`.
