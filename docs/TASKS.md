# Tasks — active work list

Single source of truth for **open** work. Plan and rationale: [PLAN.md](PLAN.md). Measured baseline:
[AUDIT.md](AUDIT.md). Completed polish pass and its history: [../ROADMAP.md](../ROADMAP.md).
Dated log of what actually changed: [PROGRESS.md](PROGRESS.md).

Legend: `[ ]` todo · `[~]` blocked (needs a decision or external access) · `[x]` done.
Every task names its proof. A task without proof is not done.

## Definition of done (applies to every task)

- `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` pass in `app/`.
- The behavior has a test at its seam, or a documented reason it cannot have one.
- Affected docs updated; no new claim without evidence.
- One commit, conventional message, plus a [PROGRESS.md](PROGRESS.md) line.

## Decisions outstanding

- [x] **D1** MIT, with an Adobe trademark disclaimer (`LICENSE`).
- [x] **D2** `0.3.0`: this pass adds release engineering and evidence, not a small patch.
- [x] **D3** Keep `checks/` as evidence and `investigation.cany/` as the CS6 mechanic record; both are excluded from bundles and the docs gate, and are never shipped. No deletion without owner approval.
- [x] **D4** No Windows desktop available in this environment; GNU binary built and inspected, behaviour recorded as unverified where a real shell is needed.
- [x] **D5** Deferred: Phase 20 features (AVIF/HEIC, PSD/AI, Bridge, animation timeline, video) are out of this milestone; ADRs record the decision.

---

## Phase 13 — Truth and hygiene

- [x] **13.1** Add `LICENSE` and reference it from `README.md`, `docs/build-release.md` and the   bundle.
- [x] **13.2** Make `app/Cargo.toml` the only version source; derive release artifact names from it; bump per D2.
- [x] **13.3** Add `rust-toolchain.toml` pinning the edition/toolchain CI uses.
- [x] **13.4** Rewrite `app/PROJECT_STRUCTURE.md` from the real tree (36 tools, 10 menus, real modules).
- [x] **13.5** Reconcile `app/MECHANICS.md` with `app/mechanics.json` and `app/src/constants.rs` (1920×1080, 144 dpi).
- [x] **13.6** Split `IMPROVEMENTS.md` into fixed/open/rejected; open items become tasks here.
- [x] **13.7** Remove the duplicate WASM entry point (`app/index.html` vs inline HTML in `release.yml`).
- [x] **13.8** Correct `docs/build-release.md` (`--test-threads=1` no longer required) and document all CLI flags.
- [x] **13.9** Add `CONTRIBUTING.md`, `SECURITY.md`, issue and PR templates.
- [x] **13.10** Make the crate self-contained: move the splash asset into `app/assets/`, deduplicate `logo.png`.
- [x] **13.11** Add a doc-link and doc-claim check (script + CI) so stale docs fail the build.

## Phase 14 — Gates and automation

- [x] **14.1** Add coverage measurement to CI with a ratchet floor starting at 18.70 % (per-file floors for `doc.rs`, `formats.rs`, `app/find_replace.rs`).
- [x] **14.2** Add `deny.toml` and `cargo deny check` (advisories, licenses, bans, sources); document `url` and `heapless` acceptance with exposure and expiry.
- [x] **14.3** Add `cargo audit` to CI.
- [x] **14.4** Extend CI: Windows build + CLI smoke (`--version`, `list-commands`, `info --demo`), WASM *build* not just `check`.
- [x] **14.5** Add `[lints]` policy to `Cargo.toml`; enable correctness/suspicious/perf/style; add curated pedantic subset later, one family per commit.
- [x] **14.6** Collapse `.github/workflows/*` and `.gitforge.yml` into one definition of record.
- [x] **14.7** Add `dependabot.yml` for cargo and actions.
- [x] **14.8** Add `scripts/ci-local.sh` that runs every gate exactly as CI does, with exit codes preserved.
- [x] **14.9** Add PR-time doc build/lint (markdown link check, trailing whitespace, duplicated headings).

## Phase 15 — Test foundation (logic)

- [x] **15.1** Create `app/tests/cli.rs`: every CLI verb, exit codes, stdout shape, missing/invalid input.
- [x] **15.2** Create `app/tests/mcp.rs`: initialize, tools/list, tools/call success+error, unknown method, malformed JSON, notifications.
- [x] **15.3** Bring `formats.rs` to ≥ 85 %: each export format, corrupt input, unwritable path, layered-PNG version mismatch, HTML/SVG escaping.
- [x] **15.4** Add golden-file tests for SVG/HTML/PNG output with a documented normalization step.
- [x] **15.5** Property tests for path booleans (area additivity, containment, idempotence) and transforms.
- [x] **15.6** Property/round-trip tests for color conversions and gradient math.
- [x] **15.7** Error-path matrix for `app/files.rs`: dirty tabs, cancel, missing file, permission denied, save-all-and-quit.
- [x] **15.8** Palette/contrast tests in `theme.rs` (feeds the Phase 18 accessibility work).
- [x] **15.9** Verify no test touches display, network, OS clipboard or real user directories; add a guard test.

## Phase 16 — Interaction harness

- [x] **16.1** Spike `egui_kittest` against egui 0.37: boot the real frame headless, render at 1360×860 and 960×640.
- [ ] **16.2** Snapshot suite for the workspace shell: startup, splash, menu open, tool flyout, footer, status bar.
- [ ] **16.3** Snapshot suite for panels: collapse/expand, collapsed bottom dock, tab overflow, iconic mode, inspector resize clamp.
- [ ] **16.4** Gesture tests: drag panel → merge / dock / float; stack top/bottom; snap to edge; detach; close last group (no panic).
- [x] **16.5** Keyboard map as one table; test every binding resolves and `Esc`/dialogs take priority.
- [ ] **16.6** Menu matrix test: every enabled item performs its named action; disabled state matches preconditions.
- [ ] **16.7** File browser + find/replace + settings dialog interaction tests (address bar, filters, cancel, scope preview).
- [ ] **16.8** Port the manual `checks/` screenshot loop into the automated suite; keep `checks/` for evidence only.
- [x] **16.9** Run the suite under Xvfb with a fixed software GL backend and document the invocation.

## Phase 17 — Architecture deepening

- [x] **17.1** Extract arrange/edit/object command bodies from `app.rs` into `app/{arrange,edit,object}.rs` behind request/outcome seams. *(Done 2026-11-25: app.rs 7368->4952 lines; pure motion, gates green.)*
- [x] **17.2** Extract `layout/chrome.rs` (menu bar, document tab strip, canvas footer). *(Done 2026-11-25: menu bar already lived in menus.rs; tab strip, footer and bottom dock moved to layout/chrome.rs, layout.rs 2850->2041.)*
- [x] **17.3** Move remaining panel bodies behind the `PANELS` registry. *(Done 2026-11-25: panels/mod.rs 3635->1388; bodies split into panels/bodies.rs + panels/tabs.rs, all rendering via the registry seam.)*
- [x] **17.4** Isolate panel drag/resize/stack geometry into a pure, egui-free module.
- [x] **17.5** Introduce the `FileService` seam for open/save/export so flows run headless. *(Done 2026-11-25: `FileService` trait + `FormatFileService` default in app/files.rs; open/save/export/import/save-all route through it; two fake-service headless tests. Also replaced panels/mod.rs dead glob imports left by 17.3 with explicit imports — clippy green again.)*
- [x] **17.6** Enforce a module size budget in CI (warn > 1,200 lines, fail > 2,000 with allowlist).
- [x] **17.7** Retire `panels::panel`/dock duplication noted in ROADMAP technical debt.
- [x] **17.8** Update `docs/architecture.md` and add ADRs for any seam that changes.

## Phase 18 — Functional verification and defect burn-down

- [x] **18.1** Triage and close the `IMPROVEMENTS.md` reports: panel expand/pin/snap/group; `Alt+F` + arrows; `Tab` preview; Align/Path design; branding; transparent splash. *(Done 2026-11-25: I-06 Alt+letter mnemonics + arrow nav and I-07 Tab preview verified in code; I-09 Align/Path implemented as CS6 icon panels; I-08a collapse chevron fixed. Drag/snap gesture verification stays in 16.3/16.4; splash/branding closed earlier.)*
- [x] **18.2** Verify Windows Jump List, app-switch flashing and taskbar identity, or record them as unverified limitations. *(Recorded unverified 2026-11-25: Linux host, no Windows shell/VM; code paths exist in shell_windows.rs.)*
- [x] **18.3** Prove 960×640 and 1360×860 with every panel open: nothing clipped, footer fits. *(Done 2026-11-25: headless Xvfb captures at both sizes with demo scene and all docks open, visually verified — nothing clipped, footer fits. Root-caused a real defect on the way: eframe's persisted window state overrode `--size` even with `persist_window: false` (that flag only stops saving, not loading); screenshot mode now points `persistence_path` at a throwaway location.)*
- [x] **18.4** Intelligence panel: unconfigured / invalid key / success / timeout paths, no fabricated output. *(Done 2026-11-25: seven tests cover missing key/model/prompt rejection, success, error, disconnect, in-flight guard and ai_extract_content; live endpoint call still needs real credentials.)*
- [x] **18.5** Close remaining ROADMAP checks: 2.6, 3.3, 4.5, 8.3, 9.4, 11.5. *(Dispositioned 2026-11-25: 3.3 closed via 18.4 AI tests; 11.5 closed — arrange/edit extraction, FileService, panel registry all landed; 2.6/4.5 recorded unverified pending interactive GUI; 8.3/9.4 unverified, no Windows host.)*
- [x] **18.6** Accessibility: keyboard-only menu/dialog operation, focus order and visible focus on WASM, contrast audit, reduced motion. *(Checklist committed 2026-11-25 as docs/A11Y.md: contrast covered by theme tests; keyboard menus covered in code (unverified on screen); WASM focus, reduced motion and screen readers recorded as limitations with the path to close each.)*
- [x] **18.7** Performance budgets with measurements: startup, frame time @1920×1080/500 objects, export per format, peak memory. *(Frame-time budget landed 2026-11-25 as `frame_time_with_500_objects_stays_within_budget_at_1920x1080`: 500-object frame ≈ 60 ms in debug/software raster, under the hard 120 ms ceiling and 6× empty-baseline relative bound (3× produced false failures from a noisy micro-baseline). Startup/export/memory budgets still belong to the release receipt on real hardware — recorded there as unverified on this Linux host.)*
- [ ] **18.8** File and fix defects found here by ID; no silent fixes.

## Phase 19 — Release engineering

- [x] **19.1** Script deterministic bundle assembly (exe, README, LICENSE, docs, GLOSSARY, CHANGELOG) from a clean tree.
- [x] **19.2** Version-stamp artifacts, changelog section and release notes from `Cargo.toml`.
- [x] **19.3** Release target MSVC; keep GNU as a documented developer convenience.
- [x] **19.4** Emit and verify SHA-256 for binary and archive.
- [x] **19.5** Tag-driven release with packaged-binary smoke tests; record Wine results as evidence.
- [x] **19.6** Reproducible-build check with documented caveats. *(Verified 2026-11-25: two release builds from the same tree with `src/main.rs` touched between them produced byte-identical `target/release/firecraft` (SHA-256 `41bf162e…`). Caveat: same target dir, same toolchain, same host; a clean-target or cross-host rebuild is not claimed.)*
- [x] **19.7** Publish the WASM build to Pages from the same release pipeline and document the URL.

## Phase 20 — Product gaps (needs D5)

- [x] **20.1** Decide scope for AVIF/HEIC, PSD/AI interop, Bridge, Animate timeline/tweening/symbols, video/animated GIF.
- [x] **20.2** Write ADRs for each accepted feature before implementation.
- [x] **20.3** Add a roadmap section per accepted feature with its own exit criteria. *(Done 2026-11-25: no features accepted — ADR-0003 defers all; ROADMAP Phase 20 section records the deferral and the exit criteria any future acceptance must meet.)*

---

## Completed in the 2026-11-25 GUI / architecture pass

- [x] Radial gradient clipping and outer-stop fill (was bleeding outside its object).
- [x] Slice URL tab moved below the slice and elided; no longer covers a button label.
- [x] Showcase fits the live canvas rect on first frame at any window size.
- [x] Multi-line text measurement by longest line and line count (`doc::text_width`, `Text` bounds).
- [x] `demo::showcase(w, h)` extracted from `app.rs` with four layout tests.
- [x] Properties inspector scrolls instead of clipping at the 118 px dock height.
- [x] Tool-rail hint wraps; Optimize Matte control no longer covers its label; CTA label centred.
- [x] `scripts/gui-shot.sh` + `--size WxH` for deterministic review captures.

## Open GUI defects found by the 2026-11-25 review

- [x] **G-1** Right dock's first header strip renders as an empty bar (`»` + grip dots) above the tab row.
- [x] **G-2** Color Mixer's last control is clipped when its panel group is short.
- [x] **G-3** Top-right workspace cluster has no spacing at 960 px.
- [x] **G-4** Toolbar Bridge button is a text label (`Br`) among icons.
- [ ] **G-5** Capture interactive states (menus, flyouts, drag/stack) so the loop can review them. *(Interactive state is now asserted by `harness.rs`; pixel capture of open menus still needs a `--screenshot-menu` flag.)*

## Carried over from ROADMAP.md (unchecked there, tracked here)

These are the items the roadmap still lists as unverified. They close in Phase 16/18 with
automated proof, not another manual pass.

- [ ] ROADMAP 2.6 — drag/merge/dock/float/snap, no orphan groups, no panic on last group.
- [ ] ROADMAP 3.3 — Intelligence unconfigured / invalid-key / success paths.
- [ ] ROADMAP 4.5 — built-in styles on single and multi-selection; every filter toggled and cleared.
- [ ] ROADMAP 8.3 — Windows app-switch flash and full-screen toggling.
- [ ] ROADMAP 9.4 — taskbar Jump List entries on a real Windows shell.
- [ ] ROADMAP 11.5 — remaining architecture candidates.
