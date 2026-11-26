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
- [x] **16.2** Snapshot suite for the workspace shell: startup, splash, menu open, tool flyout, footer, status bar. *(Done 2026-11-26: `app/src/harness.rs` test `workspace_shell_states_paint_at_the_minimum_size` — shell frame at both sizes, splash, Tab key, Alt+F menu, iconic flyout, footer/status. 101 tests passing.)*
- [x] **16.3** Snapshot suite for panels: collapse/expand, collapsed bottom dock, tab overflow, iconic mode, inspector resize clamp. *(Done 2026-11-26: `app/src/harness.rs` test `panel_states_paint_collapse_tab_overflow_iconic_and_inspector_clamp` — collapse/expand paint delta, last-tab overflow, iconic strip vs flyout, bottom clamp at 5000/0, collapsed-bottom delta. 102 tests passing.)*
- [x] **16.4** Gesture tests: drag panel → merge / dock / float; stack top/bottom; snap to edge; detach; close last group (no panic). *(Done 2026-11-26: `app/src/harness.rs` test `panel_gestures_float_merge_stack_detach_snap_and_close` — ungroup → float, merge, stack, detach, snap to edge, no-panic close-last. 101 tests passing.)*
- [x] **16.5** Keyboard map as one table; test every binding resolves and `Esc`/dialogs take priority.
- [x] **16.6** Menu matrix test: every enabled item performs its named action; disabled state matches preconditions. *(Done 2026-11-26: `app/src/harness.rs` test `every_menu_row_triggers_and_closes_the_menu` — walks 10 menus, 60 rows, Enter/Escape close. 101 tests passing.)*
- [x] **16.7** File browser + find/replace + settings dialog interaction tests (address bar, filters, cancel, scope preview). *(Done 2026-11-26: `app/src/harness.rs` tests `find_replace_text_mode_counts_cycles_and_replaces_across_scope`, `find_replace_color_mode_reports_no_match_and_real_match`, `dialog_toggles_open_and_close_without_frames_panicking` — text replace across scope, color find/replace both paths, dialog toggles. 101 tests passing.)*
- [x] **16.8** Port the manual `checks/` screenshot loop into the automated suite; keep `checks/` for evidence only. *(Done 2026-11-26: `scripts/gui-suite.sh` captures the six-state set (home, splash, menu, flyout, find-replace, about) in one invocation; new `--state <menu|flyout|find-replace>` flag reaches interactive states from the capture path. Evidence: `checks/gui-suite-2026-11-26/`.)*
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
- [x] **18.8** File and fix defects found here by ID; no silent fixes. *(Done 2026-11-26: verification-phase defects were test-premise bugs fixed in place — flyout gated on Iconic Mode preset, stale `replace_color` making color replace a no-op, snap distance outside the 12 px threshold, unstable status assert under zoom clamp. ROADMAP 4.5 closed with two new harness tests: built-in style presets on single and multi-selection with undo to per-object baselines, and all five live-filter kinds added/toggled/removed/cleared plus repeat-last on multi-selection. Demo-scene objects carry baseline live effects; tests count from per-object baselines. 104 tests passing.)*

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
- [x] **G-5** Capture interactive states (menus, flyouts, drag/stack) so the loop can review them. *(Done 2026-11-26: `--state <menu|flyout|find-replace>` flag + `scripts/gui-suite.sh`; pixel evidence in `checks/gui-suite-2026-11-26/` — File menu open, iconic flyout expanded, find-replace dialog. Drag/stack states remain gesture-code-asserted in `harness.rs`, not pixel-captured.)*

## Carried over from ROADMAP.md (unchecked there, tracked here)

These are the items the roadmap still lists as unverified. They close in Phase 16/18 with
automated proof, not another manual pass.

- [x] ROADMAP 2.6 — drag/merge/dock/float/snap, no orphan groups, no panic on last group. *(Done 2026-11-26: closed by `panel_gestures_float_merge_stack_detach_snap_and_close` in harness.rs.)*
- [x] ROADMAP 3.3 — Intelligence unconfigured / invalid-key / success paths. *(Done 2026-11-25: closed by the 18.4 seven-test suite; live endpoint needs real credentials, recorded in 18.4.)*
- [x] ROADMAP 4.5 — built-in styles on single and multi-selection; every filter toggled and cleared. *(Done 2026-11-26: harness tests `styles_and_live_filters_apply_to_single_and_multi_selection` and `every_live_filter_kind_adds_toggles_and_clears_on_selection`.)*
- [~] ROADMAP 8.3 — Windows app-switch flash and full-screen toggling. *(Blocked: no Windows host in this environment, per D4; full-screen toggle itself is code-verified in harness tests.)*
- [~] ROADMAP 9.4 — taskbar Jump List entries on a real Windows shell. *(Blocked: no Windows host in this environment, per D4; `shell_windows.rs` code paths exist and are logged-and-ignored on failure.)*
- [x] ROADMAP 11.5 — remaining architecture candidates. *(Dispositioned 2026-11-25 via 18.5: Phase 17 extractions landed; remaining candidates deferred in the architecture review report.)*

## Phase 21 — UI interaction, iconography, and review build (2026-11-26)

- [x] **21.1** Fix Properties collapse chevron being shadowed by the header's full-width double-click target; verify collapse and expand by one click at the live widget bounds. *(Regression test: `collapse_chevrons_work_with_a_single_click`.)*
- [x] **21.2** Replace application UI icon glyphs with Phosphor and retain attribution/license; preserve specialized path/align drawings where needed.
- [x] **21.3** Refresh application mark, favicon, and Windows icon assets; package third-party notices with Windows distribution.
- [x] **21.4** Build and smoke the updated Windows review package; record source SHA, archive SHA-256, executable version output, and actual Windows 11 feedback separately. *(Built from source `8b0084e4e8fd89619ef4b6e3dca894d6a91f5583`; executable SHA-256 `f922ebfb38951b2667bf0184ce305828f71f4671bf716670bb0137bdfcca7799`; ZIP SHA-256 `43cf928016f5763bbb02c172bd393798de86053cf800809854fe74a5b4960bff`; Wine `--version` = 0.3.0, 68 command rows, demo info JSON valid. Native Win11 user review remains open in 29.6.)*
- [ ] **21.5** Verify menu/tool/panel visuals against supplied CS6 references at small and normal windows; capture interactive states and record differences.
- [ ] **21.6** Resolve remaining splash, canvas focus, clipboard, and file-dialog defects with targeted reproduction tests.

## Phase 22 — Baseline audit and documentation system

- [x] **22.1** Add [`PRODUCT_ROADMAP.md`](PRODUCT_ROADMAP.md) with implemented foundations, sequenced future scope, exit criteria, and user-request traceability.
- [x] **22.2** Add [`AGENTS.md`](../AGENTS.md) as a provider-neutral repository guide aligned with `CLAUDE.md`.
- [x] **22.3** Research sibling FireCraft and requested reference projects; document transferable practices and licensing/compatibility limits in [`research/reference-projects.md`](research/reference-projects.md).
- [x] **22.4** Compare current GitHub Actions and sibling CI/release controls in [`research/ci-cd-patterns.md`](research/ci-cd-patterns.md).
- [x] **22.5** Preserve the architecture review and top seam recommendations in [`architecture-review-2026-11-26.md`](architecture-review-2026-11-26.md); top seam is pointer gesture routing in `app.rs`.
- [x] **22.6** Add a dated audit snapshot with actual Aegis/test/lint/coverage/E2E/GitForge/GitHub results and explicit limits. *(Evidence: [`AUDIT-2026-11-26.md`](AUDIT-2026-11-26.md); distinguishes reruns from earlier coverage and remote-run evidence.)*
- [ ] **22.7** Reconcile all current open PRs, branches, issues, workflow runs, and stale docs; no issue/PR is closed or merged without evidence.
- [ ] **22.8** Publish local tutorial documents to the GitHub Wiki and verify the Help/README links point to the canonical version.
- [ ] **22.9** Measure code coverage using the pinned project toolchain; track the user's aspirational 99% target separately for Rust line/function coverage, testable behavior, documentation links/claims, and accessibility criteria. Define denominators and exclusions before setting CI ratchets; do not claim 99% until each result is reproducible.

## Phase 23 — Reliability, feedback, and updates

- [ ] **23.1** Specify autosave and recovery state machine, 15-minute default, retention, location, migration, and failure behaviors; add ADR.
- [ ] **23.2** Implement atomic recovery generations and startup recovery chooser with failure-injection tests.
- [ ] **23.3** Define and test clipboard type coverage, cross-document paste, cross-application import/export, and unsupported-format feedback.
- [ ] **23.4** Add user-reviewed in-app bug/feature/change submission using issue templates; attach no document or diagnostics by default.
- [ ] **23.5** Design update discovery, signed/hash-verified platform downloads, changelog view, update notification, and rollback behavior; no silent install.
- [ ] **23.6** Improve file dialogs with format filters, extension choices, path validation, overwrite review, drag/drop, and keyboard access.

## Phase 24 — Help, search, and accessible workflow

- [ ] **24.1** Add Help screens for keybindings, formats, tools, panels, tutorials, troubleshooting, accessibility, and changelog.
- [ ] **24.2** Expand palette search to commands, tools, panels, documents, layers, pages, states, styles, and help; add aliases, fuzzy search, and ranking tests.
- [ ] **24.3** Benchmark catalogue search at realistic scale before exploring opt-in semantic search; preserve offline/private baseline search.
- [ ] **24.4** Conduct CS6 menu and options census; document available/partial/unavailable semantics and remove misleading inert controls.
- [ ] **24.5** Audit pop-out canvases, keyboard focus transfer, Tab mode switching, drag/drop, workspace persistence, and per-platform behavior.
- [ ] **24.6** Complete criterion-level WCAG 2.1 AA/AAA assessment, remediation backlog, reduced-motion behavior, and real screen-reader/manual checks. No blanket conformance claim without evidence.

## Phase 25 — Extension API and Extension Library

- [x] **25.0** Research historical Fireworks extension categories, author workflows, source mirrors, licensing, and modern marketplace safeguards in [`research/fireworks-extensions.md`](research/fireworks-extensions.md); no legacy code/assets copied.
- [ ] **25.1** Inventory current scripting/MCP/command surfaces and select a stable shared extension command API; record an ADR.
- [ ] **25.2** Define versioned extension manifest, supported extension kinds, dependency rules, signature/provenance, license, API compatibility, and migration policy.
- [ ] **25.3** Threat-model extension capabilities and choose a restricted runtime or explicit permission boundary before allowing third-party code.
- [ ] **25.4** Implement local install/enable/disable/update/uninstall, safe mode, error logs, version pinning, and rollback with malformed/malicious package tests.
- [ ] **25.5** Create an author SDK, examples, API documentation, validation CLI, and compatibility test harness.
- [ ] **25.6** Build a searchable community catalog with source/license/permissions/compatibility/changelog display and maintainer verification.
- [ ] **25.7** Implement staged one-click install; show permissions before enabling, require renewed consent for permission escalation, and never auto-enable downloaded code.
- [ ] **25.8** Add submission, CI validation, review/takedown, vulnerability reporting, ownership transfer, and immutable versioned artifact policies.
- [ ] **25.9** Define two public trust tiers: FireCraft-maintained official extensions and reviewed community submissions. Document the criteria, owner approval, compatibility/support commitment, provenance/signing, tests, licensing, and release-note process required before a community extension can be promoted into an official release.
- [ ] **25.10** Publish the extension catalog with a clear contribution guide and submission form; show review status and never imply an unreviewed community extension is officially supported.

## Phase 26 — Brushes, touch, and mobile exploration

- [ ] **26.1** Specify brush model, resource format, pressure/tilt semantics, smoothing, and versioned persistence.
- [ ] **26.2** Implement/test deterministic stroke sampling, undoable raster/vector strokes, and performance budgets.
- [ ] **26.3** Add stylus pressure/tilt mapping, device profile settings, coalescing, and native-device verification.
- [ ] **26.4** Add attributed brush import/export packs and resource manager.
- [ ] **26.5** Prototype touch-first navigation/tool gestures and responsive mobile layout; do not claim mobile support until on-device workflow is verified.

## Phase 27 — Photo, HDR, animation, video, and CorridorKey

- [ ] **27.1** Define non-destructive adjustment graph and color-management model; record an ADR and representative image corpus.
- [ ] **27.2** Prototype optional RAW workflow with explicit decoder/license review and non-destructive round-trip expectations.
- [ ] **27.3** Define HDR/scene-linear and OpenEXR semantics; test unclamped data and metadata preservation.
- [ ] **27.4** Stabilize frame/timeline model and animation export timing, playback, onion-skin, and undo behavior.
- [ ] **27.5** Design video track/media model, codec boundary, streaming, cancellation, proxy, memory, and benchmark contracts.
- [ ] **27.6** Assess CorridorKey2.0 license, model weights, upstream runtime/hardware, dependency notices, and process isolation before integration.
- [ ] **27.7** Prototype optional user-invoked CorridorKey job adapter for still/frame sequences; validate inputs/outputs and report unavailable GPU/runtime clearly.
- [ ] **27.8** Gate full clip/video integration on usable native-hardware evidence, permissions, color/matte quality tests, and maintainer decision.

## Phase 28 — Repository, CI, release, and deployment

- [ ] **28.1** Repair GitHub Actions failures and establish a passing required-check baseline on the exact target revision.
- [ ] **28.2** Validate FireCraft CI through local GitForge only after fleet freshness, runner activity, and semantic check results are confirmed.
- [ ] **28.3** Create a safe issue/PR/branch disposition ledger; test branch candidates and dependency updates before merging; remove stale branches only after ownership and remote-use review.
- [ ] **28.4** Decide and execute experiment-to-root-repository consolidation with explicit source-of-truth, history/PR/issue mapping, and rollback plan.
- [ ] **28.5** Define target support matrix and build/install/open/edit/save/reopen/export/update/uninstall receipt per supported platform.
- [ ] **28.6** Add release manifest, checksums, license notices, SBOM/provenance, signing policy, and fail-closed artifact validation.
- [ ] **28.7** Deploy marketing site and WASM app from the canonical source; verify previews, cache behavior, source SHA, download hashes, and rollback.
- [ ] **28.8** Track coverage per language/module and documentation link/claim checks; set and raise measured ratchets without conflating coverage with quality.
- [ ] **28.9** Restore GitHub Actions and GitForge gate health; capture runner/job-step evidence for the current 6 Dependabot PRs before disposition.
- [ ] **28.10** Publish to Cloudflare only from the canonical, validated WASM/site artifact and retain source-SHA deployment evidence.
- [ ] **28.11** Build the public marketing site from the canonical repository and add a dedicated Extensions page describing the API roadmap, public catalog, official/community trust tiers, permission/review policy, and submission route; validate mobile, keyboard, contrast, and source-SHA deployment.

## Phase 29 — v0.3.0 public multi-platform release

- [ ] **29.1** Build and package Windows x64, macOS universal (arm64 + x86_64), Linux x86_64 tar.gz/DEB/AppImage/Flatpak, and WebAssembly ZIP from the same matching `v0.3.0` tag.
- [ ] **29.2** Smoke-test each package, validate package inventories and executable versions, and create a source-SHA artifact manifest plus SHA-256 checksums.
- [ ] **29.3** Run release workflow on public GitHub-hosted runners; resolve startup or build failures and require every platform job to pass.
- [ ] **29.4** Publish the GitHub Release and deploy the exact WASM build only after the aggregate release gate passes.
- [ ] **29.5** Record macOS Developer ID signing/notarization status; publish as unsigned only with an explicit artifact note if credentials are unavailable.
- [ ] **29.6** Collect Windows 11, macOS, and Linux native-user review; distinguish build/smoke evidence from user acceptance/certification.

## Phase 30 — Remaining explicitly requested product scope

- [ ] **30.1** Create and publish a user-facing sample-design page with a complete editable example, annotated layers/styles, source file, export, and tutorial/Help/README entry.
- [ ] **30.2** Write a terminal-in-app threat model and ADR covering process launch, command consent, working-directory scope, environment/secret exposure, output handling, cancellation, and platform parity; implementation is gated on safe process boundaries.
- [ ] **30.3** Specify expanded Intelligence capabilities (skills, memories, scheduled jobs) with explicit opt-in, storage/retention, provenance, permission review, audit log, cancellation, and no fabricated outputs; add an ADR and staged acceptance criteria before implementation.
- [ ] **30.4** Define independent, reproducible progress targets toward the requested 99% quality goal: Rust line/function/branch coverage where supported, behavioral test coverage by user flow, documentation link/claim coverage, and accessibility criteria. Keep dated baselines, generated-code exclusions, and ratchet increments in CI; coverage alone is not quality or conformance.
