# Audit — 2026-11-25

Snapshot of what the repository actually is, measured rather than recalled. Re-run the commands in
[Reproducing this audit](#reproducing-this-audit) before trusting any number here.

- Commit: `3133c87` (`feat(ui): polish CS6 workspace and document tab navigation`), branch `main`,
  working tree clean.
- Version in `app/Cargo.toml`: `0.2.0`. Version in `CHANGELOG.md`: `0.2.1` released.
- Toolchain: `rustc 1.100.0`, edition 2024, no `rust-toolchain.toml`.

## 1. Gate status (all run locally on 2026-11-25)

| Gate | Command | Result |
|------|---------|--------|
| Formatting | `cargo fmt --all -- --check` | **pass** |
| Lints | `cargo clippy --all-targets -- -D warnings` | **pass** |
| Unit tests | `cargo test -- --test-threads=1` | **pass**, 46 passed |
| Unit tests, parallel | `cargo test` | **pass**, 46 passed |
| WASM type-check | `cargo check --target wasm32-unknown-unknown` | **pass** |
| Unused deps | `cargo machete` | **pass**, none |
| Security | `cargo audit` | **fail**, 1 vulnerability + 13 warnings |
| Coverage | `cargo llvm-cov --summary-only` | **18.70 % lines**, 22.87 % functions |

The Linux-native build is green and clean. Nothing below contradicts that; the gaps are about
*proof*, not about a broken tree.

## 2. Coverage baseline (lines)

`cargo llvm-cov --summary-only -- --test-threads=1`, 22,563 lines total, 18,343 missed.

| Module | Lines | Cover | Reading |
|--------|------:|------:|---------|
| `constants.rs`, `page.rs` | 20 | 100 % | trivial/constant |
| `colors.rs` | 194 | 94.3 % | genuinely covered |
| `doc.rs` | 875 | 85.6 % | the model is the best-covered layer |
| `app/find_replace.rs` | 247 | 55.1 % | partial |
| `formats.rs` | 1,118 | 52.4 % | round trips covered, edge paths not |
| `app/files.rs` | 232 | 39.7 % | save/close intents partial |
| `app.rs` | 5,972 | 37.5 % | command layer partial |
| `mcp.rs` | 475 | 24.4 % | one dispatch test only |
| `panels/tools.rs` | 452 | 10.4 % | inventory assertions only |
| `theme.rs` | 112 | 9.8 % | palette math untested |
| `canvas.rs` | 1,770 | 1.1 % | rendering + hit testing untested |
| `panels/mod.rs` | 3,073 | 0.7 % | dock/window management untested |
| `layout.rs` | 2,390 | 0 % | frame, menus, shortcuts untested |
| `menus.rs` | 1,125 | 0 % | every menu action untested |
| `dialogs/*` | 2,103 | 0 % | file browser, find/replace, settings untested |
| `icons.rs`, `main.rs`, `panels/{color,properties}.rs` | 2,405 | 0 % | — |

Interpretation: the **document model and serialization are tested; the product surface is not**.
Everything a user clicks is in the 0 % band. That is why "hand-test it on Windows" keeps showing up
as the verification story in `ROADMAP.md`.

There are no integration tests (`app/tests/` does not exist), no UI tests, no snapshot tests, no
property tests, and no benchmarks.

## 3. Dependency health

| Item | Detail | Exposure |
|------|--------|----------|
| `url 2.5.8` — RUSTSEC-2026-0592 (stack overflow in `Url::origin()` on nested `blob:` URLs) | via `eframe → egui-winit → webbrowser` | Low: FireCraft only hands plain `http(s)` help URLs to the browser; no patched release published at audit time |
| `heapless 0.8.0` — 9 × "unsound" advisories | via `geo → rstar` | Low-severity warnings; rstar only uses fixed-capacity storage internally |
| `ttf-parser`, `zbus_names`, `zvariant`, `zvariant_derive` — unmaintained | via egui font/DBus stack | No action available from this tree; track upstream |

No `deny.toml`, no `cargo audit`/`cargo deny` CI gate, and no written policy for accepting an
advisory. `cargo machete` reports no unused dependencies.

## 4. Code shape

| File | Lines | Functions |
|------|------:|----------:|
| `app.rs` | 7,567 | 233 |
| `panels/mod.rs` | 3,628 | 172 |
| `layout.rs` | 2,836 | 135 |
| `canvas.rs` | 2,177 | 69 |
| `icons.rs` | 1,943 | 11 |
| `dialogs/mod.rs` | 1,590 | 108 |
| `formats.rs` | 1,427 | 68 |

Positives worth keeping: zero `TODO`/`FIXME`/`todo!`/`unimplemented!`; `unsafe` confined to
`shell_windows.rs`; the seams described in `docs/architecture.md` are real; `DocumentFixture`
already exists as the start of an intent-level test style.

`cargo clippy -- -W clippy::pedantic` reports **829 warnings**, dominated by numeric casts
(`f32↔u8/u32/i32`, 300+), `map(..).unwrap_or(..)` (41), and `format!` appended to `String` (16).
There is no `[lints]` policy in `Cargo.toml`, so the lint level is whatever the default happens to be.

## 5. Feature claims vs code

Verified present and implemented:

| Claim | Evidence |
|-------|----------|
| 36 tools | `Tool` enum has 36 variants; `panels/tools.rs` asserts each appears once |
| 68 MCP/CLI commands | `COMMAND_CATALOG.len() == 68`, all unique |
| Layered `.fw.png` round trip | `formats::tests::layered_fw_png_roundtrip_preserves_out_of_page_objects` |
| Real polygon booleans | `geo` dependency; `polygon_booleans_are_area_correct` |
| Splash, clipboard paste, Ctrl/Shift/Alt+scroll, `Tab`/`F4`/`F11` full screen, `Alt+F` menus | present in `dialogs/mod.rs`, `app.rs`, `canvas.rs`, `layout.rs` |
| God-view canvas, tab overflow, bulk close, Windows shell module | present, with targeted tests |

Implemented but **unproven** (no automated coverage, and the last human report in
`IMPROVEMENTS.md` said some of these felt broken):

- panel expand/collapse, pin, snap, tab-group merging, window stacking and tearing off
  (`panels/mod.rs`); `ROADMAP.md` items 2.6, 4.5, 3.3, 8.3, 9.4 remain unchecked;
- every menu action end to end (0 % coverage), `Alt+F` focus/arrow navigation;
- the Intelligence panel against a live OpenAI-compatible endpoint;
- Windows Jump List rendering and app-switch flashing;
- the 960×640 minimum layout.

## 6. Documentation and repository hygiene

| Finding | Detail |
|---------|--------|
| No `LICENSE` file | `README.md`, `docs/build-release.md` and ADR-0001 all describe a distribution bundle containing a license; there is none in the tree or in `app/dist/windows/` |
| `app/PROJECT_STRUCTURE.md` is stale | claims 28 tools (actual 36), 5 menus (actual 10), and lists `shape.rs`, `paths.rs`, `history.rs`, `panels.rs`, `tools.rs`, `properties.rs`, none of which exist |
| `app/MECHANICS.md` contradicts `app/mechanics.json` | 800×600 vs 1920×1080 default canvas, 72 vs 144 dpi export; `INSTRUCTIONS.md` requires the two to agree |
| `docs/build-release.md` says `--test-threads=1` is required | parallel `cargo test` passes in 0.94 s; the constraint no longer holds |
| `app/index.html` is committed | `release.yml` regenerates an equivalent page inline, so there are two sources of truth for the web entry point |
| Build reads outside the crate | `dialogs/mod.rs` does `include_bytes!("../../../screenshots/FireCraft Flame Tech Logo.png")`, so `app/` alone cannot build; `logo.png` (732 KB) is embedded from both `main.rs` and `app.rs` |
| `IMPROVEMENTS.md` is untriaged | raw escaped-markdown notes mixing fixed items (splash, paste, scroll) with open complaints (panel snapping, Align/Path polish); no IDs, owner, or status |
| 233 of 310 tracked files are audit artifacts | `checks/` (106) and `investigation.cany/` (127); no documented retention policy |
| Missing community/process files | `CONTRIBUTING.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md`, issue/PR templates, `dependabot.yml`, `CODEOWNERS` |
| CLI surface undocumented | `--screenshot`, `--splash`, `--tab`, `--new`, `--about` exist in `main.rs` but appear in no user doc |
| Accessibility unexamined | the WASM page is one `<canvas>` with an `aria-label`; no focus-order, keyboard-trap, contrast or reduced-motion checks, no WCAG statement |

## 7. CI/CD state

`.github/workflows/ci.yml`: format, clippy, tests, WASM `check` on Linux only.
`.github/workflows/release.yml`: Windows MSVC build + WASM build and Pages deploy, release on tags,
hardcoded `v0.2.0` artifact names.
`.gitforge.yml`: a parallel definition of the same checks with a pinned `rust:1.99-bookworm` image.

Not gated anywhere: coverage, coverage regression, `cargo audit`/`cargo deny`, license policy,
Windows build on pull requests, WASM *build* (only `check`), CLI smoke tests of the produced binary
(`--version`, `list-commands`, `info --demo`), archive checksum verification, `cargo fmt`/`clippy`
for the release job's own tree, or a pinned toolchain.

## 8. Summary judgement

The tree is in good mechanical health and the architecture writing is better than the average
project of this size. The real gap is **evidence**: two-thirds of the product is only known to
compile, the panel/window interaction that users reported as broken has no automated proof, and the
release/distribution story is blocked by a missing license and drifting version numbers. The audit
artifacts (`checks/` screenshots, `investigation.cany/`) show a disciplined manual loop; the next
step is turning that loop into tests that run without a human.

## 9. Findings index

The plan and task list refer to these IDs. Severity reflects user/release impact, not effort.

| ID | Sev | Finding | Evidence in this file |
|----|-----|---------|-----------------------|
| F-01 | P0 | User-visible behavior (panels, menus, dialogs, gestures) is unproven; the last human report says some of it felt broken | §5, §2 |
| F-02 | P0 | Coverage 18.70 % lines / 22.87 % functions; UI layer 0 % | §2 |
| F-03 | P1 | No `LICENSE` file, though docs and bundles promise one | §6 |
| F-04 | P1 | Version is not single-sourced (`0.2.0` crate, `0.2.1` changelog, `v0.2.0` artifacts, no toolchain pin) | §1, §6, §7 |
| F-05 | P1 | CI does not gate coverage, security, Windows build on PRs, WASM build, or binary smoke tests | §7 |
| F-06 | P1 | 1 vulnerability + 13 advisory warnings with no policy or gate | §3 |
| F-07 | P1 | `app.rs` 7,567 lines/233 functions; `panels/mod.rs` 3,628; `layout.rs` 2,836 | §4 |
| F-08 | P2 | Docs contradict code: `PROJECT_STRUCTURE.md`, `MECHANICS.md`, `--test-threads=1` claim | §6 |
| F-09 | P2 | No integration tests, UI harness, snapshot tests, property tests or benchmarks | §2 |
| F-10 | P2 | No lint policy; 829 pedantic warnings, correctness lints not enabled | §4 |
| F-11 | P2 | Build reads a splash asset from outside the crate; 732 KB logo embedded twice | §6 |
| F-12 | P2 | Two sources of truth for the WASM host page | §6 |
| F-13 | P2 | `IMPROVEMENTS.md` untriaged; open user reports have no IDs or status | §6 |
| F-14 | P3 | 233 of 310 tracked files are audit artifacts, with no retention policy | §6 |
| F-15 | P3 | Accessibility unexamined (WASM is one canvas with an aria-label) | §6 |
| F-16 | P3 | No performance budgets or measurements despite size/speed claims | §5, §6 |
| F-17 | P3 | Missing `CONTRIBUTING`, `SECURITY`, `CODE_OF_CONDUCT`, templates, dependabot, CODEOWNERS | §6 |
| F-18 | P3 | CLI flags `--screenshot/--splash/--tab/--new/--about` documented nowhere | §6 |

## Reproducing this audit

```sh
cd app
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo check --target wasm32-unknown-unknown
cargo machete
cargo audit
cargo llvm-cov --summary-only -- --test-threads=1
cargo clippy --all-targets -- -W clippy::pedantic 2>&1 | grep -c '^warning'
```

Note: the local `cargo` is wrapped by a host admission gate, so concurrent cargo invocations queue.
Run one at a time.
