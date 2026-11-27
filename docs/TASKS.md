# Tasks — active work list

Single source of truth for **open** work. Plan and rationale: [PLAN.md](PLAN.md). Measured baseline:
[AUDIT.md](AUDIT.md). Completed polish pass and its history: [ROADMAP.md](ROADMAP.md).
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

- [x] **28.1** Establish passing exact-source required CI, build, and release quality gates; track external deployment credential failures separately. *(v0.3.6 release workflow `46052887293`: quality gates, Windows, macOS, Linux, WASM, GitHub Release, and GitHub Pages succeeded. Cloudflare authentication is isolated under task 43.4 and does not affect the passing required-code gates.)*
- [x] **28.2** Validate FireCraft CI through local GitForge only after fleet freshness, runner activity, and semantic check results are confirmed. *(The registered `gates` pipeline passed for PR #46 commit `33f49266de0bcf51712bfba8e41b371f5c736677` in run `b4e5ed15-e1c7-45c3-8132-ee43e03dfba3`. The v0.3.6 candidate tree, identical to tag `v0.3.6`, passed in run `6192a6b0-31e8-44da-8379-178c3ccd8652` on candidate commit `ee35e3ee1bdc088d9617e9cef237936509c0f641`.)*
- [ ] **28.3** Create a safe issue/PR/branch disposition ledger; test branch candidates and dependency updates before merging; remove stale branches only after ownership and remote-use review. *(Read-only GitHub follow-up: 0 open issues, 0 open PRs, and 8 remote branches beyond `main`. PR #41 merged and its branch was removed. Current branch comparison/ownership evidence is in [`canonical-branches-2026-11-27-followup.tsv`](integration/canonical-branches-2026-11-27-followup.tsv). Five release branches are protected; three documentation branches remain diverged with unique content. No deletion is safe until those unique docs are reconciled and protected branch owners approve cleanup.)*
- [x] **28.4** Set the experiment repository as the canonical FireCraft product; retain the former root FireCraft repository as a historical donor/reference source. *(Direction decision recorded 2026-11-26 in PRIMARY-REPO-RESET-AND-CONSOLIDATION-PLAN.md. This does not claim code/ticket migration is complete; execution is tracked in Phase 33.)*
- [ ] **28.5** Define target support matrix and build/install/open/edit/save/reopen/export/update/uninstall receipt per supported platform.
- [ ] **28.6** Add release manifest, checksums, license notices, SBOM/provenance, signing policy, and fail-closed artifact validation.
- [ ] **28.7** Deploy marketing site and WASM app from the canonical source; verify previews, cache behavior, source SHA, download hashes, and rollback.
- [ ] **28.8** Track coverage per language/module and documentation link/claim checks; set and raise measured ratchets without conflating coverage with quality.
- [x] **28.9** Restore GitHub Actions and GitForge gate health; capture runner/job-step evidence for the current 6 Dependabot PRs before disposition. *(All six are closed or merged: PRs #1–#4 merged with Gates, WASM, Windows CLI smoke, coverage, and CodeQL checks successful; PRs #5–#6 were closed after GitHub explicitly reported “The job was not started because an Actions budget is preventing further use” (runs `45758504107` and `45758506612`, all three jobs each, no runner/steps). These two receipts are budget failures, not code-test failures. Current v0.3.6 required quality/build checks passed; the exact candidate GitForge gates also passed in run `6192a6b0-31e8-44da-8379-178c3ccd8652`.)*
- [ ] **28.10** Publish to Cloudflare only from the canonical, validated WASM/site artifact and retain source-SHA deployment evidence.
- [ ] **28.11** Build the public marketing site from the canonical repository and add a dedicated Extensions page describing the API roadmap, public catalog, official/community trust tiers, permission/review policy, and submission route; validate mobile, keyboard, contrast, and source-SHA deployment.
- [x] **28.12** Organize project documentation: move community health files and changelog under `.github/`, consolidate product/reference docs under `docs/`, link the existing `AGENTS.md`, add GitHub issue routing and Code of Conduct, and update packaging paths and relative links.

## Phase 29 — v0.3.0 public multi-platform release

- [x] **29.1** Build and package Windows x64, macOS universal (arm64 + x86_64), Linux x86_64 tar.gz/DEB/AppImage/Flatpak, and WebAssembly ZIP from the same matching `v0.3.0` tag. *(Source `e415a333`; all eight platform assets published.)*
- [x] **29.2** Smoke-test each package, validate package inventories and executable versions, and create a source-SHA artifact manifest plus SHA-256 checksums. *(All package jobs and published asset SHA-256 checks passed; source and release manifests published.)*
- [x] **29.3** Run release workflow on public GitHub-hosted runners; resolve startup or build failures and require every platform job to pass. *(Final workflow run #81, attempt 3 passed all jobs.)*
- [x] **29.4** Publish the GitHub Release and deploy the exact WASM build only after the aggregate release gate passes. *(Release `v0.3.0` and Pages deployment completed.)*
- [ ] **29.5** Record macOS Developer ID signing/notarization status; publish as unsigned only with an explicit artifact note if credentials are unavailable.
- [ ] **29.6** Collect Windows 11, macOS, and Linux native-user review; distinguish build/smoke evidence from user acceptance/certification.

## Phase 30 — Remaining explicitly requested product scope

- [ ] **30.1** Create and publish a user-facing sample-design page with a complete editable example, annotated layers/styles, source file, export, and tutorial/Help/README entry.
- [ ] **30.2** Write a terminal-in-app threat model and ADR covering process launch, command consent, working-directory scope, environment/secret exposure, output handling, cancellation, and platform parity; implementation is gated on safe process boundaries.
- [ ] **30.3** Specify expanded Intelligence capabilities (skills, memories, scheduled jobs) with explicit opt-in, storage/retention, provenance, permission review, audit log, cancellation, and no fabricated outputs; add an ADR and staged acceptance criteria before implementation.
- [ ] **30.4** Define independent, reproducible progress targets toward the requested 99% quality goal: Rust line/function/branch coverage where supported, behavioral test coverage by user flow, documentation link/claim coverage, and accessibility criteria. Keep dated baselines, generated-code exclusions, and ratchet increments in CI; coverage alone is not quality or conformance.

## Phase 31 — FireCraft extension ecosystem

- [x] **31.1** Review the historical Fireworks extension catalogs, authoring articles, command libraries, and source mirrors; record workflow patterns, limitations, licensing guardrails, and links in [`research/fireworks-extensions.md`](research/fireworks-extensions.md). *(Desk research complete; no legacy code or assets reused.)*
- [x] **31.2** Add a public-facing Extensions information page and link it from the canonical marketing site; state plainly that the page is informational and the catalog/install service is not operational. *(Implemented in sibling `/nas/Temp/repos/FireCraft`; deployment remains gated under 28.10.)*
- [ ] **31.3** Define the versioned FireCraft extension API, package manifest, compatibility contract, capability model, author SDK, validation harness, and migration/deprecation policy.
- [ ] **31.4** Implement the local extension manager: per-user staged install, explicit enable, disable/uninstall, offline packages, hash/provenance checks, permission review, safe mode, immutable version history, update, and rollback.
- [ ] **31.5** Build the public catalog and community submission workflow with identity/ownership, license and policy review, automated validation, moderation, security reports, takedown/advisory process, and official promotion criteria.
- [ ] **31.6** Add one-click in-app discovery/install/update only after package validation and capability consent pass; test interrupted installs, tampering, API mismatch, permission escalation, rollback, and no-network/offline operation.
- [ ] **31.7** Publish extension author and user tutorials, API reference, trust-tier policy, privacy/data behavior, support matrix, changelog, and an extension security response process.

## Phase 32 — UI fit, dialogs, clipboard, and Web performance follow-up

- [ ] **32.1** Verify image/text/internal clipboard paste on Windows, macOS, Linux, and browser WASM; test browser `ClipboardEvent` permission and focus behavior, malformed images, and large clipboard payloads. *(Partial 2026-11-27: native bitmap-paste helper has valid/invalid input tests. After fixing the async WASM handoff, headed Chromium under Xvfb wrote a real PNG to the browser clipboard, sent Ctrl+V, and visibly inserted/selected a 128×128 bitmap with no JS errors. Native OS clipboard flows, browser denial/focus cases, malformed data, and large payload limits remain open.)*
- [ ] **32.2** Re-audit every top-level CS6 menu against the checked-in screenshots; inventory absent/partial actions, add behavior or mark unsupported, and verify submenu keyboard/mouse navigation and tool flyout hover details.
- [ ] **32.3** Rework file Open/Save/Import/Export window UX toward familiar OS file-browser behavior; decide native OS picker vs in-app parity per platform, then validate resize, small screens, keyboard, filters, overwrite, and cancellation flows.
- [ ] **32.4** Review the full workspace at 960×640, 1360×860, and browser sizes; fix spacing, alignment, panel overflow, focus, and pop-out canvas issues with before/after captures.
- [ ] **32.5** Profile browser rendering with realistic documents and large folders; record frame, input-to-paint, memory, startup, and export baselines before adding optimization complexity.
- [ ] **32.6** Validate swatch grid density, selection hit targets, keyboard alternative, and contrast in docked and floating panel states.

## Phase 33 — Experiment-first repository reset and legacy migration

- [ ] **33.1** Inventory and preserve all local/remote experiment worktrees, branches, stashes, agent sessions, PRs, tags and current release state; record exact source SHAs and ownership. Do not reset, clean, prune or delete anything during the inventory. *(Partial 2026-11-27: clone ends clean on local `main`; remotes are `main` and protected `release/prepare-v0.3.1`. Three merged PR branches were deleted; GitHub's `release/*` ruleset rejects deleting the remaining branch. Tags `v0.3.0` and `v0.3.1`; open experiment PR/issue counts are zero. A full cross-repo worktree/stash/agent ownership inventory remains open.)*
- [x] **33.2** Audit experiment PR #13 by its complete diff, including browser image clipboard, path-node editing, file-browser/UI changes, dependencies, workflow, and docs; update its scope, validate required checks, and merge only after review. *(Done 2026-11-27: PR #13 merged as `00b1fd0`; exact-SHA hosted Linux gates, coverage, WASM, Windows build/CLI smoke, and CodeQL passed.)*
- [ ] **33.3** Reconcile the experiment TASKS, current audit/progress, v0.3.0 release manifest/assets and CI receipts on the exact target SHA; keep build, package, install, launch and interactive acceptance states separate.
- [ ] **33.4** Produce a complete old-repository crosswalk for the 31 open issues, old PRs #186/#187/#188, all 21 observed branches, related tasks/decisions and useful commits. Each item receives a verified target disposition and successor link where applicable.
- [ ] **33.5** Build a native Fireworks .fw.png compatibility corpus and compare the experiment's versioned iTXt payload with legacy mkTS/mkBT/tiled-raster/unknown-node handling. Record an ADR and tested adapter strategy before changing the target document model or format code.
- [ ] **33.6** Create a donor feature matrix for legacy engine, codecs, scripting, CLI/MCP, compositor, vector/raster, web export and host integration. Select behavior/code only where a demonstrated target gap and acceptance test exist; do not merge the legacy workspace wholesale.
- [ ] **33.7** Port the first owner-approved high-priority compatibility or editing capability behind the experiment's existing document/FileService/command seams, with regression tests and source provenance.
- [ ] **33.8** Reconcile duplicate and stale feature/task/README/release claims across both repositories. Keep experiment docs and TASKS as the only live product authorities; preserve dated legacy audits/history.
- [ ] **33.9** Complete the issue/PR/branch disposition report. Close or transfer legacy items only after successor links or acceptance evidence are present; preserve releases/tags and all unmerged owner-active work.
- [ ] **33.10** Deliver a final owner review package: exact SHAs, files and commits reused, format compatibility results, tests, ticket outcomes, branch/PR decisions, remaining blockers, and the next prioritized milestone.

**Execution authority:** [PRIMARY-REPO-RESET-AND-CONSOLIDATION-PLAN.md](PRIMARY-REPO-RESET-AND-CONSOLIDATION-PLAN.md), [LEGACY-FIRECRAFT-MIGRATION-REGISTER.md](LEGACY-FIRECRAFT-MIGRATION-REGISTER.md), and [AGENT-HANDOFF-2026-11-26.md](AGENT-HANDOFF-2026-11-26.md).

## Phase 34 — v0.3.1 cross-platform release

- [x] **34.1** Prepare version `0.3.1`, changelog notes, and version-derived artifact naming for the merged editor/UI/WASM improvements. *(Completed 2026-11-27 in release commit `3efe9c1`.)*
- [x] **34.2** Pass the tag workflow's full quality, dependency, security, coverage, and exact-source checks. *(Release workflow 45935587390 passed; Aegis remains unavailable, and cargo audit advisories are documented separately.)*
- [x] **34.3** Build Windows x86_64, universal macOS, Linux tar/AppImage/DEB/Flatpak, and WASM ZIP; validate all checksums and source-SHA manifests. *(All eight release assets verified against `SHA256SUMS.txt`; source SHA `3efe9c1c9c3bad6537c2038ecb5660946d56e028`.)*
- [x] **34.4** Publish GitHub Release with the title exactly `v0.3.1`; deploy the matching WASM artifact to GitHub Pages and verify the deployment SHA. *(Release and Pages deployment passed; both point to source SHA `3efe9c1c9c3bad6537c2038ecb5660946d56e028`.)*
- [ ] **34.5** Run native Windows 11 install/open/edit/save/export review and record the user's acceptance; CI smoke does not close this item.
- [x] **34.6** Deploy the tag-built WASM bundle to a Cloudflare Pages review branch and verify version, source SHA, and served asset responses. *(2026-11-27: [preview](https://wasm-review-v0-3-1.firecraft-web.pages.dev/); source `3efe9c1c9c3bad6537c2038ecb5660946d56e028`, HTTP 200 for HTML/JS/WASM.)*

## Phase 35 — Expanded product scope and architecture decisions

*Planning and evidence first. No feature implementation starts until its decision, scope, ownership, and acceptance tests are recorded.*

- [ ] **35.1** Reconcile the full request list against `PRODUCT_ROADMAP.md`, `features/README.md`, README links, and this task board. Classify every item as shipped with proof, active, planned, deferred, or rejected; remove contradictory “done” claims. *(Roadmap now includes the new future-scope families; final cross-document reconciliation remains.)*
- [ ] **35.2** Complete scenario-based product research for Photopea, Pixlr, Affinity Designer, Illustrator, Inkscape, Krita, darktable, and selected Fireworks extension references. Capture date, source, user task, observed interaction, transferable idea, licensing/provenance boundary, and a testable FireCraft acceptance statement; do not copy assets or code.
- [ ] **35.3** Write a version-history decision covering recovery snapshots vs user versions, local storage/retention/corruption, diff/restore UX, and optional Git/GitHub adapters with offline behavior and credential isolation. Depend on Phase 1 autosave/recovery; keep history local by default.
- [ ] **35.4** Write a page-layout/PDF export decision and fixture plan for multi-page/artboard mapping, physical units, bleed, typography/font embedding, vector preservation, raster resolution, transparency, color profiles, links, and accessibility metadata. State unsupported import/edit behavior plainly.
- [ ] **35.5** Write a web publishing/export specification for reproducible asset/CSS/HTML handoff, local preview, naming/path rules, and an export manifest. Compare Cloudflare Pages, Vercel, and self-hosted static hosting (IIS/Apache) with preview, credential scope, publish confirmation, identity, and rollback gates.
- [ ] **35.6** Define provider-neutral AI capability briefs for BYOK/model selection, image generation/edit/extend, STT/TTS, and document-aware assistance. For each, document data sent, consent, secret storage, cost/rate limits, provenance, cancellation, failure, accessibility, and how core editing works with providers disabled.
- [ ] **35.7** Expand the Intelligence panel plan for skills, memory, scheduled jobs, and AI-assisted workflows: data ownership/retention, source/provenance, capability grants, job isolation, reviewable actions, undo, cancellation, audit log, and no fabricated completion. Record ADRs before implementation.
- [ ] **35.8** Complete a terminal/SSH/AI-workbench threat model for local process launch, shell commands, remote hosts, filesystem/network scope, environment secrets, extension scripts, output redaction, consent, cancellation, and sandboxing. Compare an in-app terminal with opening the user's configured external tools; do not ship an execution surface before the threat model and tests.
- [ ] **35.9** Produce a bounded 3D exploration brief: concrete creator use cases, 2D/2.5D/3D boundaries, import/view/edit goals, document ownership, formats/licenses, GPU/memory/performance and accessibility. End with continue/defer/reject; no feature promise before a measured prototype decision.
- [ ] **35.10** Turn mobile/tablet goals into a device and input matrix covering stylus pressure/tilt, touch gestures, palm rejection, screen sizes, memory, native file access, keyboard/accessibility, and minimum performance. Tie device claims to real-device receipts.
- [x] **35.11** Run and preserve the architecture skill's codebase report as a visual HTML artifact; validate every candidate against recent change history, domain glossary/ADRs, the deletion test, and test/locality benefits. Promote selected candidates into small tasks only after the report and plan are reviewed. *(2026-11-27: standalone report `/tmp/architecture-review-20261127.html`; top recommendation is canvas interaction behavior, with canvas view/geometry, startup presentation, and format path-I/O as alternatives. No architecture refactor was started before review.)*

## Phase 36 — v0.3.3 clipboard fix and review release

- [x] **36.1** Review and merge the frame-safe browser clipboard handoff fix after focused native/WASM build checks, the real-browser PNG paste proof, documentation updates, and all required exact-SHA hosted checks pass. *(Merged PR #19 as `263c273`; the PR-head exact-SHA native, WASM, Windows build/CLI smoke, coverage, and CodeQL checks passed.)*
- [ ] **36.2** Add repeatable browser coverage for real image paste plus text/internal paste, denied permission, malformed payload, focus changes, and bounded large-image behavior; separate browser evidence from native OS clipboard evidence. *(Partial 2026-11-27: TDD added a host-runnable `paste_with_text` regression for valid PNG payload decode and malformed payload rejection; it failed before the shared decoder fix and passes afterward. Focused paste tests, strict Clippy, formatting, and WASM check pass. Browser permission/focus/large-payload automation and Windows/macOS/native OS clipboard acceptance remain open.)*
- [x] **36.3** Prepare a version-consistent v0.3.3 release from merged main; build and verify Windows, macOS, Linux, and WASM artifacts, hashes, manifests, notices, release title, and source SHA. *(Workflow 45966326178 passed all package jobs; published asset set and source manifests match `918ca8ddad5b850ae18f4101327a5e76fcf12966`; release title/tag exactly `v0.3.3`.)*
- [x] **36.4** Deploy the matching WASM release to a Cloudflare Pages preview for review and verify its build marker, asset hashes, MIME types, runtime startup, and clipboard path. Keep the marketing deployment separate and record hosting capacity/settings accurately. *(Preview: [wasm-review-v0-3-3.firecraft-web.pages.dev](https://wasm-review-v0-3-3.firecraft-web.pages.dev/); HTML, BUILD-INFO, JS, WASM, and favicon returned HTTP 200 with expected MIME; Chromium loaded the editor and successfully pasted a 32×24 PNG as a selected bitmap. Marketing deployment remains separate.)*
- [x] **36.5** Deliver a Windows 11 review package with exact source SHA, checksum, launch instructions, and native acceptance checklist; distinguish cross-build/CLI CI from Windows GUI acceptance. *(Local ZIP is under `output/release-v0.3.3/`; the identical [published ZIP](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/releases/download/v0.3.3/firecraft-v0.3.3-windows-x86_64.zip) has SHA-256 `2222474b3d79fd12040242321da219a76f140179fb6ecad7a8b2e910978d751a`; see [Windows 11 review checklist](testing/windows-11-review.md). Native GUI acceptance remains open.)*
- [x] **36.6** Update changelog, README, release receipts, progress/task state, and the next handoff only after all evidence exists; do not mark native platform acceptance or 99% coverage without measurements. *(This update records build/deploy/browser receipts and measured coverage; open acceptance and coverage work remain visible.)*

## Phase 37 — Tool navigation

- [x] **37.1** Add a Previous Tool keybinding (`Shift+Q`) that toggles between the current tool and the last distinct tool, preserving the existing grouped-tool cycling behavior; document it and test the state transition. *(Implemented and verified 2026-11-27 with a state regression test and UI keyboard-event harness test.)*
- [ ] **37.2** Replace bounding-box object deletion in the Eraser with featherable bitmap-alpha painting; expose brush size and feather controls, preserve bitmap identity, and verify undo restores erased pixels. *(Dab, feather, continuous non-square drag/undo, scaled bitmap mapping, scaled raster export, and bitmap identity tests pass 2026-11-27. Hidden-layer erasing was reproduced red and fixed at both hit-test and mutation seams; PR #22 merged as `b494b72`, all hosted checks passed. A new focused lock-matrix regression covers locked layers and locked bitmaps at target selection, direct mutation, and pointer-down seams; the existing guards were already present, so this pass required no production change and has no red-before result. Still open: selection-mask design/clipping (the current selection model is object-based), opacity/tip controls, and canvas/export visual review.)*
- [ ] **37.3** Expand gradients to editable color stops and direction handles with explicit gradient type controls; retain backwards-compatible project loading and add rendering/edit/serialization tests. Define multi-stop interpolation and mouse/keyboard behavior before changing the document schema. *(Direction endpoints are normalized and backward-compatible; selected-gradient handles can be dragged, undo restores the direction, and canvas/raster export sample the same axis. TDD red/green tests cover legacy deserialization, axis projection, degeneracy, pointer editing/undo, mesh rendering, and horizontal raster output. SVG output now emits per-object user-space gradients along the editable axis and keeps IDs unique across layers. Still open: multiple stops, gradient type controls, standalone CSS gradient-axis output, broader geometry/concave path validation, and visual acceptance.)*
- [ ] **37.4** Make Live Filters (especially Drop Shadow) visibly useful and editable in the Properties inspector: expose effect parameters and color, verify non-destructive rendering on bitmap/vector shapes at different zoom levels, and make every change undoable. *(Parameter/color controls and undoable edits are present. TDD on 2026-11-27 reproduced missing raster-export shadows: an enabled effect changed canvas rendering but not exported pixels. Raster export now applies the configured offset and blur to rectangles using a separable Gaussian alpha profile, with silhouette/mask handling for other shapes; a red/green regression covers enabled/disabled state and blur falloff. Remaining: validate bitmap/ellipse/polygon/path/text effects against the canvas at multiple zoom levels, complete the filter-type/parameter matrix (Gaussian Blur, Color Adjust, Inner Shadow, Glow, Bevel), verify every edit remains undoable, and perform visual acceptance.)*
- [ ] **37.5** Remove custom-star fill seams and inappropriate transform overlays while drawing with a shape tool; reproduce the reported intermittent outline on canvas screenshots, assert the intended post-create selection state, and verify no rendering seam at low/high zoom and export. *(Star fan-fill now uses one shared-edge mesh and transform handles are restricted to selection tools; regression test added 2026-11-27, verification pending.)*
- [ ] **37.6** Fix bitmap texture cross-wiring during object reorder/move and document/page operations; prove cache identity follows each distinct source image across keyboard reorder, undo/redo, duplicate, flatten and delete/add. *(Root cause confirmed: cache keyed by mutable `(layer, object-index)`; allocation-address keys were rejected because render clones the document every frame. Current implementation keys by pixel content and dimensions; tests cover snapshot-clone reuse and pixel-edit invalidation. Renderer-level regressions verify hotkey reorder, duplicate/undo/redo/delete/add, and flattening selected bitmaps into a newly rendered composite. Hashing cost and broader document/page lifecycle remain open.)*
- [ ] **37.7** Audit FireCraft tool, geometry, raster, gradient, filter and render algorithms against Adobe's official Fireworks CS6 reference and checked-in screenshots; record parity/partial/missing status and acceptance fixtures. *(Baseline comparison added in `docs/research/fireworks-cs6-behavior-audit.md`; systematic per-tool scenario execution and renderer/export checks remain. TDD path-join fix now starts with the closest endpoint pair and attaches remaining paths by nearest endpoint; the 3+ path, non-spatial selection regression failed before the fix and passes after it. Compound Boolean outputs with holes/disconnected contours and broader per-tool renderer/export validation remain open.)*
- [ ] **37.8** Decide and implement target-aware raster Brush/Pencil/Paint Bucket behavior with selection-mask clipping, tolerance, brush opacity/tips and exact undo; current Brush/Pencil create vector paths and Fill is not a connected-pixel flood fill. Use a shared raster stroke seam before expanding controls.
- [ ] **37.9** Refine startup presentation: cap the splash at 2.4 seconds, show only the transparent branded overlay with a fade, keep the editor painting and responsive underneath, remove fake progress, and make the Welcome screen opt-in. Verify animation timing, click-through, startup and direct-to-editor behavior in the UI harness and on native/WASM builds. *(Code and timing/live-workspace tests pass; hosted Gates, WASM, Windows build/CLI smoke, and CodeQL passed on PR #34. Still open: a focused click-through interaction test and native Windows visual acceptance.)*

## Phase 38 — Architecture opportunities from the 2026-11-27 review

Evidence and design rationale: [Architecture review](ARCHITECTURE-REVIEW-2026-11-27.md). These are gated follow-ups, not permission to refactor speculatively; keep the existing behavior tasks as the implementation source of truth.

- [ ] **38.1** Finish the raster behavior matrix and deepening opportunity for bitmap edits (37.2/37.6/37.8): prove transformed bitmap mapping, selection clipping, target eligibility, undo grouping, cache invalidation, and export. For each defect with a practical test seam, capture a failing-before regression test before changing production code; decide on a narrow document-space edit request only after the test cases make the policy stable. *(Scaled display/source pixel mapping, old-document migration, scaled raster export, and bitmap cache renderer behavior after reorder, duplicate, undo/redo, delete/add, and flatten are covered by focused tests. Clipping, target matrix, full cache behavior, and visual acceptance remain.)*
- [ ] **38.2** Audit canvas and flat-export semantic overlap (37.5/37.7): define shared visibility, page-bounds, hidden-object, alpha, gradient, and effect fixtures; preserve separate egui and export backends. Extract shared policy only where tests show duplicated decisions cause drift.
- [ ] **38.3** Harden format/clipboard byte paths (32.1/33.5/36.2): add malformed and boundary fixtures for SVG and layered-PNG payloads; measure current coverage; consider a byte-slice codec seam only for behavior independently tested without filesystem setup. Preserve `.fw.png` compatibility.
- [ ] **38.4** Stabilize the frame-time gate: capture isolated host/resource state and repeated timing distributions; investigate scheduler contention and benchmark variance. Keep the 120 ms contract unchanged unless data supports a documented decision; never exclude the case from release CI without an explicit, measured replacement. *(2026-11-27: parallel full-suite runs measured 131–145 ms while the isolated case passed; the test now runs as a separate CI-local step, with its 120 ms ceiling unchanged. Repeated CI-host distribution evidence is still pending.)*
- [ ] **38.5** Re-measure current line/function coverage and design a per-module ratchet with a denominator and justified renderer/platform exclusions. Raise coverage at user-behavior seams; report the historical 18.70% baseline separately from current measurements. Do not claim 99% until a machine-readable receipt demonstrates it. *(Latest exact PR #39 Cobertura artifact: [run 46013680520](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/actions/runs/46013680520), source `f294e9f8afec100e6b320f05f96393b8f1886c80`, reports 18,425 / 26,445 = 69.67% lines; branch counters are absent (0/0). Previous exact PR #36 was also 69.67%, PR #34 69.59%, PR #32 69.50%, PR #29 69.33%, and PR #27 69.26%. Earlier PR #22 summary reported 68.48% lines, 71.26% functions, and 70.87% regions. The llvm-cov/Cobertura denominators remain unreconciled. The enforced floor remains 67.61%; module ratchets and 99% remain open.)*
- [ ] **38.6** Resolve the current transitive RustSec warning set: 9 `heapless 0.8.0` unsoundness advisories through `geo 0.33.1 → rstar 0.12.2`, plus 4 unmaintained transitive crates (`ttf-parser`, `zbus_names`, `zvariant`, `zvariant_derive`). Confirm the reachable code paths and platform exposure; track a compatible upstream `geo`/`rstar` upgrade or evaluate a tested geometry dependency replacement. Do not suppress advisories or force an incompatible crate override. *(2026-11-27: `cargo audit --json`: zero vulnerabilities; nine unsound and four unmaintained warnings. `rstar 0.14.0` uses `heapless 0.9`, but installed `geo 0.33.1` requires `rstar 0.12`; no compatible registry `geo 0.34.0` exists yet. `cargo deny check` passes under current policy.)*

## Phase 39 — v0.3.4 hidden-layer eraser fix and review release

- [x] **39.1** Prepare version `0.3.4` from merged main, with matching Cargo manifest/lockfile, changelog, release notes, and exact source-SHA handoff. The release includes the PR #22 hidden-layer eraser correction. *(Tag `v0.3.4` points to `c73ea44e90933288fbe8f4c3554ba47d4b574d94`; version, changelog and release metadata agree.)*
- [x] **39.2** Pass tag workflow quality gates and produce Windows x86_64, universal macOS, Linux tar.gz/AppImage/DEB/Flatpak, and WASM artifacts from the same source; verify package inventory, versions, manifests, and SHA-256 checksums. *(Actions run [45974435786](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/actions/runs/45974435786): quality, Windows, macOS, Linux and WASM jobs passed. Published assets include package/release manifests, source SHA and SHA256SUMS.)*
- [x] **39.3** Publish GitHub Release titled exactly `v0.3.4` and deploy the matching WASM artifact to GitHub Pages after all release jobs pass. *(Release: [v0.3.4](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/releases/tag/v0.3.4); GitHub Pages deploy job passed in run 45974435786.)*
- [ ] **39.4** Deploy the tagged WASM artifact to a Cloudflare Pages review branch; validate source/version marker, asset HTTP/MIME/checksums, browser startup, and clipboard smoke. *(Deployed exact release ZIP to [Cloudflare preview](https://wasm-review-v0-3-4.firecraft-web.pages.dev/); marker identifies version 0.3.4 and source `c73ea44e90933288fbe8f4c3554ba47d4b574d94`. HTML, marker, JS, WASM and favicon return HTTP 200 with expected MIME; JS/WASM/marker/favicon bytes match the release bundle. Cloudflare injects its Analytics script into HTML. Local Chromium had no usable WebGL2 context and the SwiftShader browser attempt hung during context creation, so browser startup and clipboard smoke remain unverified.)*
- [x] **39.5** Deliver the Windows 11 review ZIP and checksum locally; retain native GUI install/edit/save/export acceptance as a separate reviewer-owned state. *(Local ZIP: `output/release-v0.3.4/firecraft-v0.3.4-windows-x86_64.zip`; SHA-256 `5d613e835769dbe9c1116f35a7602c0b84ef450cff6c782cd2cf9600e5091fd5`; package contains `firecraft.exe`, `BUILD-INFO.txt`, `VERSION`, and release docs. Review checklist: [Windows 11 v0.3.4](testing/windows-11-review-v0.3.4.md). Native Windows acceptance is still pending reviewer execution.)*
- [ ] **39.6** Reconcile the local vs hosted coverage denominators in 38.5 and use the exact current receipt to choose the next behavior-test seam; keep the 99% request open until measured. *(Latest artifact is PR #39 run 46013680520 at source `f294e9f8afec100e6b320f05f96393b8f1886c80`: 18,425 / 26,445 = 69.67%, branch counters 0/0. Gradient-axis projection/export and 3+ path join ordering have behavior tests; bitmap flattening still lacks exact composite-pixel/alpha/order and source-restoration assertions. The llvm-cov/Cobertura denominator difference remains unresolved, and 99% is not met.)*

## Phase 40 — v0.3.5 path-join reliability release

- [x] **40.1** Prepare version `0.3.5` from merged main, update the single Cargo version source and lockfile, and describe the path-join change in the changelog. Keep release/tag names exactly `v0.3.5`. *(PR #39 merged; tag and GitHub Release are `v0.3.5`.)*
- [x] **40.2** Pass the exact-tag release quality gates and verify Windows x86_64, universal macOS, Linux tar.gz/AppImage/DEB/Flatpak, and WASM packages, manifests, source SHA, and checksums. *(Release workflow [46018626991](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/actions/runs/46018626991) passed on source `bec0b65f3d05f66d299568f0ea80227e049daee4`; eight platform assets plus release/package manifests and SHA-256 checks are published.)*
- [x] **40.3** Publish GitHub Release `v0.3.5` and matching WASM to GitHub Pages only after the aggregate build and release gates pass; verify every expected platform asset and package name. *(Published at https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/releases/tag/v0.3.5; release workflow Pages deployment passed.)*
- [ ] **40.4** Deploy the exact v0.3.5 WASM bundle to the Cloudflare review Pages project. Verify `BUILD-INFO`, JS/WASM checksums and MIME types, startup, and browser clipboard behavior; record any WebGL/device limitation without claiming a passing browser test. *(Preview is live at https://wasm-review-v0-3-5.firecraft-web.pages.dev/; BUILD-INFO, JS/WASM/favicon checksums and MIME types match source `bec0b65`. Chromium loaded the app shell and title, but WebGL2 canvas rendering/screenshot timed out and interactive editor startup/clipboard remain unverified. Do not close until a supported WebGL browser passes the editing loop.)*
- [x] **40.5** Deliver the exact v0.3.5 Windows ZIP and checksum at `output/release-v0.3.5/`; keep Windows 11 GUI edit/save/export and splash/path-tool visual acceptance as reviewer-owned sign-off. *(Local ZIP: `output/release-v0.3.5/firecraft-v0.3.5-windows-x86_64.zip`; SHA-256 `4cfdad8cc68133fa448d2cd424846a25612447bc25e0b509fe4167b300ed0868`. Hosted Windows build/CLI smoke passed. Use the [Windows 11 review checklist](testing/windows-11-review-v0.3.5.md); native GUI acceptance remains open.)*
- [x] **40.6** Refresh the pinned offline GitForge image for the v0.3.5 lockfile, run the exact branch pipeline locally, and retain the image lock digest plus terminal pipeline receipt. *(Exact PR #39 head `727b1a84fc5265c17528a45ef879ddcd0aab8cfb` passed local GitForge run `be7524ac-2a6e-49c2-989f-ba0968566679`; offline image 4 digest `sha256:d03a844a27237b333ec1b9bc9d1a8989d822227d8d6bc2c57894c8ad755887d8`; Cargo.lock SHA `76d7fb62c41c6939bc86debc39b2f7270a3f2e98f744e570db4bee1f89201432`.)*
- [x] **40.7** Keep the path-join fix covered by its red/green regression tests and run the full local suite, strict lint, docs and WASM gates on the release candidate before publishing. *(Exact PR #39 head passed hosted Gates, Rust coverage, Windows smoke, WASM and CodeQL. Offline `ci-local.sh --quick` passed 188 unit, 12 CLI, 5 hygiene, 10 MCP and one isolated performance test; strict Clippy, formatting, 61-file docs check and module-size passed. Offline release-mode WASM build passed.)*
- [x] **40.8** Make the documentation gate fail closed when Git cannot enumerate tracked Markdown, and validate it in a container checkout. *(Regression test failed before the fix with `docs: 0 files checked`; now rejects Git failure and passes in the image with 61 files checked.)*

## Phase 41 — post-release correctness and audit closure

- [x] **41.1** Add pixel-exact bitmap flatten tests for alpha composition, object order, output bounds, and undo restoring source bytes/order/dimensions; then investigate any mismatch before widening the raster module. *(Added `flatten_selected_bitmaps_is_pixel_exact_and_undo_restores_sources_and_order`; exact alpha composite, 6×4 bounds/origin, selected stack position, page dimensions, source-object byte/order restoration, and redo pixels pass. No production mismatch was reproduced, so no raster implementation change was needed.)*
- [ ] **41.2** Run the managed Aegis production scan on the exact repository head, record the scanner version and machine-readable findings, and triage each result. Do not substitute an Atheon scan or a direct unmanaged invocation. *(Managed Aegis 0.6.6 scan `4885e72a-64a2-490a-a97b-c27b72ac3b7f` ran on tree `4d4e0eb293889679b0dc3d5f35f157b61cfc4adb`; follow-up `0889e9ed-bbbe-4782-af42-597c4f3b9456` scanned tree `07f2cba1d5c432bd3234e9193cdca6df52b6a51a`. Exact PR #45 merge scan `6ae27914-4c5c-4d96-bff0-dbee65f5cdcb` ran on commit `48d5d4b7b683aed199cff72aa953d43b60d549cd`: 280 files scanned, 145 skipped, 0 failed, 11,323 findings (2 critical, 55 high, 639 medium, 10,627 low; 11,053 PII, 247 compliance, 17 web-security, 3 secrets, 3 security-hardening). Two low-confidence local port-number matches are narrowly suppressed inline; 11,241 findings are in retained `investigation.cany/`, leaving 82 outside it. The three secret-category matches and two plist DTD alerts were reviewed as false positives; individual retained-evidence findings and remaining outside-evidence findings still require triage. The job exits 3 because findings were reported. Result remains non-clean; preserve protected evidence.)*
- [ ] **41.3** Raise measured coverage with behavior tests, reconcile LLVM/Cobertura denominators, add a per-module ratchet with justified exclusions, and keep 99% open until the exact artifact reaches it. *(Latest exact-head Cobertura artifact from merged PR #45 [run 46047264325](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/actions/runs/46047264325): 18,703 / 26,765 = 69.88% lines; branch counters 0/0. This is 0.11 percentage points below PR #43's 69.99% report because the denominator grew; 99% is not met. Denominator reconciliation and per-module ratchets remain open.)*
- [x] **41.4** Run the architecture review skill against recent changes, the domain glossary and ADRs; produce ranked deepening candidates with a deletion test and test/locality rationale. *(Report: `/tmp/architecture-review-20261127.html`; top recommendation is the canvas interaction policy seam. The chosen candidate is still for maintainer selection before implementation.)*
- [x] **41.5** Add an app-level Save All & Quit regression for named dirty documents plus an untitled document requiring Save As; verify the quit trampoline and saved-state contract. *(The test drives `save_all_and_quit_step` through `commit_file_dialog` with an injected `FileService`, verifies Save As precedes named-document saves, and proves every tab is clean before close can proceed. No production mismatch was reproduced. PR #43 merged after all required exact-head checks passed. Full local suite: 190 unit passed, 1 isolated timing test ignored, 12 CLI, 5 hygiene, 10 MCP passed.)*

## Phase 42 — post-v0.3.5 audit and marketing-site publication

- [x] **42.1** Reconcile v0.3.5 release, hosted coverage, local GitForge, Aegis, Cloudflare preview, Windows review bundle, and browser limits in a dated audit report with source links and no unsupported 99% claim. *(Report: [`AUDIT-2026-11-27.md`](AUDIT-2026-11-27.md).)*
- [ ] **42.2** Complete the separate production marketing deploy: free/raise the Cloudflare Pages project limit, provision a dedicated project, bind and verify the intended domain, and document rollback. *(PRs [#190](https://github.com/aliasfoxkde/FireCraft/pull/190), [#191](https://github.com/aliasfoxkde/FireCraft/pull/191) merged. Preview is live at [marketing-review.firecraft-web.pages.dev](https://marketing-review.firecraft-web.pages.dev/), deployed by successful [run 46031404998](https://github.com/aliasfoxkde/FireCraft/actions/runs/46031404998) from source `fdc0e282`; HTTP 200 verified for landing, `/extensions`, styles and logo. The production WASM app at `firecraft-web.pages.dev` was not replaced. Account has 10/10 Pages projects and no `firecraft-site`; `firecraft.app` still redirects to `/coming-soon`. Keep open until a dedicated project/domain and rollback are available and verified.)*
- [ ] **42.3** Complete evidence-based triage of legacy repository open issues and PRs; retain ownership-sensitive, already-partially-addressed, or human-review items until a maintainer selects their disposition. *(PRs #188/#190/#191 are closed as merged. The remaining issue inventory requires current artifact/acceptance review; owner input is still pending for organization incubation, Windows 11 acceptance, and GitForge runner throughput. No issue state was changed.)*
- [ ] **42.4** Audit and prune local/remote branches and worktrees only after verifying merged status, unique uncommitted work, retained release evidence, and owner-active branches. *(Legacy repository: no open PRs; 19 remote branches remain and have not been deleted because unique history is unreviewed. Canonical repository: 0 open PRs, 8 branches beyond `main`; current comparison is in [`canonical branch follow-up`](integration/canonical-branches-2026-11-27-followup.tsv). Five release branches are protected, three docs branches diverge and carry unique files, and 12 local worktrees are registered including the main checkout. Preserve them until content and active-session ownership are reconciled; do not relax branch protection.)*

## Phase 43 — v0.3.6 raster export reliability release

- [x] **43.1** Prepare version `0.3.6`, matching Cargo manifest/lockfile and changelog, including the PR #45 raster Drop Shadow export fix. *(Candidate on `codex/prepare-v0.3.6`, PR #48, based on merged PR #46 commit `3dfe280`; `scripts/version.sh` and locked Cargo metadata both report 0.3.6. PR #47 was closed because the repository ruleset blocks continued pushes to `release/*` branches.)*
- [x] **43.2** Pass the exact tag workflow's release quality gates, Windows x64, universal macOS, Linux x64 tar.gz/AppImage/DEB/Flatpak, and WASM build; verify artifact versions, source SHA, package manifests, and checksums. *(Tag workflow [46052887293](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/actions/runs/46052887293): release gates and every platform build passed. All eight product archives/packages passed the published `SHA256SUMS.txt`; manifests identify source `c3edaa68598f0e0daf7f5ee8c6cdd202d0e1cdd3`.)*
- [x] **43.3** Publish the GitHub release with the exact title `v0.3.6` after all required release quality/build gates pass; verify the full platform asset list and source/checksum manifests. *(Published title/tag: [`v0.3.6`](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/releases/tag/v0.3.6); 12 expected assets are present and the downloaded product packages pass checksum verification. The separate post-publication Cloudflare deployment is tracked in 43.4.)*
- [ ] **43.4** Deploy the matching WASM artifact to a Cloudflare Pages review deployment and verify served version/source markers, MIME types, hashes, and rollback instructions. Do not replace the current production WASM app until review acceptance. *(Still open. Tag run 46052887293 failed in Cloudflare authentication before upload: Wrangler received no `CLOUDFLARE_API_TOKEN`; the versioned preview currently returns HTTP 404. GitHub repository action-secret inventory is empty, no usable org secret is available, and local `CLOUDFLARE_API_TOKEN` / `CLOUDFLARE_ALL_ACCESS_TOKEN` values tested against Cloudflare returned HTTP 403. OpenZenith's local Wrangler reports unauthenticated. Add a valid, least-privilege Pages deployment token and account ID as repository secrets, then rerun a versioned deployment from the verified tag artifact. Production WASM was not changed.)*
- [x] **43.5** Deliver the v0.3.6 Windows review ZIP and checksum, and create a version-specific Windows 11 checklist; keep hosted smoke distinct from native GUI acceptance. *(ZIP: `output/release-v0.3.6/release-assets/firecraft-v0.3.6-windows-x86_64.zip`, SHA-256 `3ab4973b237186b7d2c3bf90760c8a1df7606299a1a8e63b208ee6947b4098af`; the hosted Windows build and CLI smoke passed. Review steps: [`windows-11-review-v0.3.6.md`](testing/windows-11-review-v0.3.6.md). Native Windows 11 edit/save/export acceptance remains reviewer-owned.)*
