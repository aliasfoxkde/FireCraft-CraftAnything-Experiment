# Legacy FireCraft → Experiment Repository Migration Register

> **Prepared:** 2026-11-26 (America/Chicago).
> **Target repository:** `aliasfoxkde/FireCraft-CraftAnything-Experiment`.
> **Donor/reference repository:** `aliasfoxkde/FireCraft`.
> **Companion plan:** [PRIMARY-REPO-RESET-AND-CONSOLIDATION-PLAN.md](PRIMARY-REPO-RESET-AND-CONSOLIDATION-PLAN.md).
> **Rule:** this is an audit/disposition register, not authorization to mass-transfer, close or delete anything.

---

## 1. Migration principle

The Experiment repository is now the canonical FireCraft product. The old FireCraft repository is a donor/reference repository. Do not try to make the old repository the destination or merge both histories into a single unrelated Git graph.

Every legacy feature, ticket, PR and branch must be assigned one of these outcomes:

- **VERIFY IN TARGET** — appears to exist already in the experiment; re-run acceptance on the exact target SHA.
- **PORT BEHAVIOR** — valuable capability is missing; implement against the experiment model/API with tests.
- **PORT SELECTED SOURCE** — a self-contained algorithm/codec can be adapted after dependency, license, API and test review.
- **REFERENCE ONLY** — preserve design, issue or research details; no code transfer is justified.
- **DEFER** — keep in the future roadmap with owner/review trigger; do not block core stabilization.
- **EXTERNAL / DO NOT MIGRATE** — belongs to infrastructure, an organization proposal or a different project.
- **CLOSE AS SUPERSEDED / COMPLETE** — only after a canonical successor exists or exact acceptance evidence proves it is done.

Do not close any issue on the basis of the preliminary classification below. Read its full body, comments, linked PRs/tasks, labels and current code first.

## 2. Repository snapshots

### 2.1 Experiment target

| Item | Audit observation |
|---|---|
| Repository | [FireCraft-CraftAnything-Experiment](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment) |
| Default branch | `main` |
| Audited main SHA | `e415a333f6a474bd738f47bb64d56704d711f1ce` |
| Application version | `0.3.0` |
| Visible remote branches | `main`, `fix/version-only-release-title` |
| Open issues | 0 returned by GitHub issue search |
| Open PRs | [#13](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/pull/13) |
| Latest observed public release | [v0.3.0](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/releases/tag/v0.3.0) |
| Tracked paths | 606 at the audited tree snapshot |

PR #13 is **20 files, +787/-249, two commits ahead and zero behind main** at the audit snapshot. Its title describes only the release-title tweak, but its patch also includes WASM image clipboard support, path-node selection/editing, file-browser sizing and UI changes. It requires a real code review and truthful PR description.

### 2.2 Legacy donor

| Item | Audit observation |
|---|---|
| Repository | [FireCraft](https://github.com/aliasfoxkde/FireCraft) |
| Open issues | 31 |
| Visible remote branches | 21 |
| Open PRs | #188 at audit time |
| PR #186 | Closed, unmerged |
| PR #187 | Closed, unmerged |
| PR #188 | Open; previous consolidation/1.0 handoff direction; now superseded in strategy, not yet closed |
| Tracked paths | 957 at the audited tree snapshot |
| Major source organization | Rust workspace with separate app, engine, document, I/O, format, vector, raster, composition, scripting and related crates |

### 2.3 Legacy remote branch inventory

These are names observed at the preparation snapshot. Refresh every ref before any action:

```text
audit/parity-hardening-2026-11-22
climb/C4.08-selective-jpeg
climb/C8.04-missing-docs-campaign
docs/firecraft-v1-consolidation-handoff
feat/dev-roadmap-improvements
feat/experiment-merge
feat/inspector-field-sets
feat/phase1-visual-polish
feat/post-mvp-execution
feat/release-provenance
feat/wasm-cdn-deploy
feature/adam7-png-interlace
feature/issue-93-behavior-export
fix/ci-fresh-workspace-crates
fix/wasm-size-budget-repin
fix/web-deploy-p0
fix/xtask-gate-repair-2026-11-24
main
release/v0.2.5
review/post-mvp-execution
temp/consolidation
```

Do not delete any of these from the legacy repository during this reset. Establish whether each branch has unique commits, an open/closed PR, active local worktree, stash, release dependency, or useful provenance. Keep any historical release branches/tags needed to reproduce v0.2.x.

---

## 3. Legacy PR register

| PR | Branch | Current state at audit | Disposition |
|---|---|---|---|
| [#186](https://github.com/aliasfoxkde/FireCraft/pull/186) | `temp/consolidation` | Closed, unmerged | Do not reopen/merge wholesale. Compare its unique production-readiness, WASM CDN, shell extraction, release and docs changes against the experiment. Port only justified behavior/specs. |
| [#187](https://github.com/aliasfoxkde/FireCraft/pull/187) | `feat/experiment-merge` | Closed, unmerged | The original experiment integration attempt is historical. Compare its diff with the current experiment `main` and PR #13; do not assume it is still the best or newest experiment snapshot. |
| [#188](https://github.com/aliasfoxkde/FireCraft/pull/188) | `docs/firecraft-v1-consolidation-handoff` | Open | Written for the former direction (merging into the root FireCraft repo). This register and the experiment-first plan supersede that strategy. After owner review, close #188 as superseded with a link to this repository's canonical plan. Do not merge it into the donor merely to preserve the old direction. |

The old PR heads are not complete substitutes for the experiment's current branch. Preserve the full patches and commits until the unique-change mapping is complete.

---

## 4. Legacy open issue inventory — 31 issues

The issue numbers and titles below were returned by GitHub search on 2026-11-26. The suggested disposition is **preliminary**. Confirm current state, full comments, successor tasks, tests, code and owner intent before closing or creating new issues.

| Legacy issue | Subject | Preliminary target decision | Evidence / next action |
|---|---|---|---|
| [#1](https://github.com/aliasfoxkde/FireCraft/issues/1) | ArtCraft-candidates incubation proposal | EXTERNAL / OWNER DECISION | Not an editor defect. Ask whether the owner still wants incubation/mirroring; record decision and do not migrate as a code task. |
| [#3](https://github.com/aliasfoxkde/FireCraft/issues/3) | Windows feature request | VERIFY IN TARGET | v0.3.0 includes a Windows x86_64 ZIP. Verify clean Windows 11 install/launch/edit/save/export; distinguish portable ZIP from installer and native GUI certification. |
| [#23](https://github.com/aliasfoxkde/FireCraft/issues/23) | Optimize panel and live preview | VERIFY / PORT BEHAVIOR | Experiment has an Optimize panel in its panel inventory. Confirm the live preview and actual exported bytes use the same configuration; migrate only remaining acceptance gaps. |
| [#24](https://github.com/aliasfoxkde/FireCraft/issues/24) | Animation timeline, onion-skin and playback | DEFER / PORT BEHAVIOR | Experiment has states and animated-format ambitions, but verify an actual timeline exists. Reuse the GIF order/duration/onion-skin acceptance criteria; keep post-core unless the target implementation is already close. |
| [#26](https://github.com/aliasfoxkde/FireCraft/issues/26) | CS6 keymap bindings | VERIFY IN TARGET | Experiment TASKS Phase 16.5 records keyboard-map unification/testing as complete. Re-run key-to-command and dialog-priority tests on current main; port only missing mappings. |
| [#27](https://github.com/aliasfoxkde/FireCraft/issues/27) | Main toolbar and CS6 menu rows | VERIFY IN TARGET | Experiment has 36 tools and 10 menus, with some visual QA still open. Verify visible affordances and command dispatch; do not import legacy shell code without need. |
| [#31](https://github.com/aliasfoxkde/FireCraft/issues/31) | Windows/macOS release assets need real build receipts | VERIFY / UPDATE | v0.3.0 now has Windows ZIP and macOS universal ZIP/DMG. Review exact tag workflow receipts, then run native-user install/workflow acceptance. |
| [#36](https://github.com/aliasfoxkde/FireCraft/issues/36) | GitForge Cargo registry/cache issue | EXTERNAL / DO NOT MIGRATE | Appears tied to the old GitForge runner pipeline. Determine whether it is fixed or belongs in GitForge infrastructure; migrate only if the experiment's current CI reproduces it. |
| [#37](https://github.com/aliasfoxkde/FireCraft/issues/37) | GitForge runner reap/health failure path | EXTERNAL / DO NOT MIGRATE | Runner service behavior is not editor product behavior. Transfer/link to the owning infrastructure project if still open. |
| [#42](https://github.com/aliasfoxkde/FireCraft/issues/42) | Baseline build receipts and artifact digests | VERIFY / CLOSE WHEN PROVEN | Compare v0.3.0 release manifest, package manifest, tag SHA, published asset hashes and workflow run. Keep any reproducibility gap as a new target issue. |
| [#45](https://github.com/aliasfoxkde/FireCraft/issues/45) | DEB desktop integration and install/remove/purge | VERIFY IN TARGET | v0.3.0 contains a DEB. Inspect package contents and test install, launch, file operations, upgrade, remove, purge and reinstall in a clean Debian/Ubuntu environment. |
| [#48](https://github.com/aliasfoxkde/FireCraft/issues/48) | Old PR #8 release-gate task | SUPERSEDED CANDIDATE | Its body says PR #8 closed unmerged and findings moved elsewhere. Verify all findings have canonical successors; close as superseded only after cross-references are complete. |
| [#49](https://github.com/aliasfoxkde/FireCraft/issues/49) | AppImage build | VERIFY IN TARGET | v0.3.0 publishes an AppImage. Check the exact build receipt, package contents, runtime dependencies and digest. |
| [#50](https://github.com/aliasfoxkde/FireCraft/issues/50) | AppImage inspection and launch | VERIFY IN TARGET | Download the public AppImage, inspect its contents and launch normally and with extract-and-run on a clean Linux host. |
| [#51](https://github.com/aliasfoxkde/FireCraft/issues/51) | AppImage clean-host functional smoke | VERIFY IN TARGET | Run create/open/edit/save/export smoke on the downloaded v0.3.0 AppImage; attach OS, host and digest. |
| [#52](https://github.com/aliasfoxkde/FireCraft/issues/52) | Flatpak bundle build | VERIFY IN TARGET | v0.3.0 publishes a Flatpak. Confirm immutable source/manifest, runtime/SDK, app ID, permissions and source SHA. |
| [#53](https://github.com/aliasfoxkde/FireCraft/issues/53) | Flatpak sandbox install/runtime smoke | VERIFY IN TARGET | Install the actual bundle in a clean user installation; test launch, portals/file access, open/edit/export, update and uninstall. |
| [#54](https://github.com/aliasfoxkde/FireCraft/issues/54) | RPM deferral decision | DEFER / OWNER DECISION | No RPM is listed in the experiment v0.3.0 release. Decide from user demand and packaging maintenance cost; do not add RPM merely to satisfy the old task list. |
| [#55](https://github.com/aliasfoxkde/FireCraft/issues/55) | Linux ARM64 assessment | DEFER / ASSESS | No ARM64 release asset was observed. Produce a target/toolchain/runtime cost assessment only if demand justifies it. |
| [#59](https://github.com/aliasfoxkde/FireCraft/issues/59) | Windows portable ZIP clean-VM smoke | VERIFY IN TARGET | Windows ZIP exists in v0.3.0. Test downloaded package on clean Windows 11; Wine is not a replacement for native acceptance. |
| [#60](https://github.com/aliasfoxkde/FireCraft/issues/60) | Windows installer lifecycle/upgrade/uninstall | RE-SCOPE / DEFER | v0.3.0 exposes a portable ZIP, not an MSI/Inno Setup installer. Decide whether a formal installer is a 1.x requirement; if so create a bounded target issue. |
| [#61](https://github.com/aliasfoxkde/FireCraft/issues/61) | Windows ARM64 assessment | DEFER / ASSESS | Not in the observed release. Assess demand and dependency/runner viability before promising support. |
| [#62](https://github.com/aliasfoxkde/FireCraft/issues/62) | macOS packaging script and both architectures | VERIFY IN TARGET | Universal DMG/ZIP exists. Inspect `lipo -info`, package metadata and release workflow receipt for both slices. |
| [#63](https://github.com/aliasfoxkde/FireCraft/issues/63) | macOS app bundle and DMG metadata | VERIFY IN TARGET | Inspect the downloaded app/DMG identity, icon, bundle metadata, file handling and checksum. Signing/notarization remain separate. |
| [#64](https://github.com/aliasfoxkde/FireCraft/issues/64) | macOS native install/launch and workflow | VERIFY / BLOCKED ON HOST | Build/package presence is not interactive macOS acceptance. Test drag-to-Applications, launch, open/edit/save/export, replacement and removal on supported macOS. |
| [#65](https://github.com/aliasfoxkde/FireCraft/issues/65) | Android feasibility decision | DEFER / FEASIBILITY GATE | Experiment is a desktop + WASM app; no native Android target was observed. Keep this as a decision only until toolkit, lifecycle, file access and touch interaction are proven. |
| [#66](https://github.com/aliasfoxkde/FireCraft/issues/66) | Native Android port | DO NOT START WITHOUT #65 | No native target/receipt. Do not copy a web wrapper and call it a native port. |
| [#80](https://github.com/aliasfoxkde/FireCraft/issues/80) | Quality ratchet, edge tests, coverage and docs | PORT / RECONCILE | Experiment has coverage and test gates. Compare acceptance to its 67–68% measured baselines; port any unique property/corpus tests with provenance. No blanket 99% claim. |
| [#95](https://github.com/aliasfoxkde/FireCraft/issues/95) | AVIF decode | DEFER / FEATURE BACKLOG | Verify the target codecs and WASM size budget. Prioritize only after a user-demand and compatibility review. |
| [#96](https://github.com/aliasfoxkde/FireCraft/issues/96) | Windows MSVC build and Inno Setup installer | PARTIAL / RE-SCOPE | v0.3.0 Windows build and ZIP exist; no installer was listed. Merge acceptance with #3/#31/#59/#60 after confirming whether a formal installer is required. |
| [#126](https://github.com/aliasfoxkde/FireCraft/issues/126) | GitForge runner throughput and infrastructure timeouts | EXTERNAL / DO NOT MIGRATE | Old pipeline/runner operation. Transfer/link to the infrastructure owner; retain in FireCraft only if current target CI can reproduce the failure. |

### 4.1 How to complete the ticket transfer

For each of the 31 rows:
1. Re-fetch the issue and all comments; confirm it remains open and inspect linked tasks/PRs.
2. Search the experiment code, docs, tests, PRs and current releases.
3. Verify whether the acceptance criteria are already met.
4. If unfinished and still wanted, create one canonical issue in this repository with updated scope and direct link to the legacy ticket.
5. Carry over unique requirements, reproduction details and test data; do not copy stale descriptions verbatim.
6. Add the new issue URL to the old ticket and explain whether it remains open, is transferred, closed completed, or closed superseded.
7. Close only after the new record is live or acceptance is demonstrably complete.
8. Update `docs/TASKS.md` and the migration register in the same reviewed PR.

Keep a final summary of counts by outcome: completed, migrated, superseded, deferred, blocked, external, and not planned.

---

## 5. Feature migration matrix

| Capability from legacy | Experiment state to verify | Recommended action |
|---|---|---|
| Fireworks `.fw.png` binary layout, mkTS/mkBT tiles, unknown-node round-trip | Experiment stores project data in iTXt and documents a layered PNG workflow | P0 external-file compatibility investigation; preserve experiment document model and adapt format layer |
| 257-command shared engine catalogue / 356-verb CS6 DOM matrix | Experiment documents 68 commands and 36 tools | Do not chase count parity. Build a behavior-level matrix and port only demanded commands |
| ExtendScript host / scripting DOM | No equivalent full legacy host documented as shipped in experiment | P2/P3; threat model and API ADR before implementation |
| Advanced PSD/PSB I/O and raster codecs | Experiment uses image crate for common raster formats and its own SVG/HTML paths | Build a format fidelity matrix; port selective mature codecs with corpus tests |
| Compositor, masks, advanced filters, onion-skin | Experiment has live effects/filters, but full legacy compositor breadth is not assumed | Compare visual output and undo semantics; port selected algorithms only |
| Web export (HTML/CSS/sprites/slices/hotspots) | Experiment's release/docs emphasize editor, CLI/MCP and basic SVG/HTML export | P2 candidate; define an end-to-end asset handoff workflow |
| History journal/replay | Experiment has undo/redo and command operations; legacy has deeper journal/history APIs | Compare requirements; add persistent history only if it solves a real user need |
| CLI/MCP | Experiment already includes both | Keep existing target APIs; port only missing command semantics or reliability improvements |
| Browser bridge / WebOS host integration | Experiment has WASM build; no legacy host equivalence should be assumed | Stabilize browser app first; then adopt a versioned host bridge |
| Cloud/site/release workflow | Legacy repo contains a marketing `site/`, Cloudflare work and a large release plan | Audit as donor; deploy separately from the experiment's WASM build |
| Feedback loop and diagnostics | Legacy has `docs/FEEDBACK-LOOP.md`; experiment Phase 23.4 is open | Port the zero-token/privacy contract, update repo destinations and implement in experiment |
| In-app update discovery | Legacy release plan contains update ideas; experiment roadmap requests update checks | Keep opt-in/manual update behavior; no silent install or client-embedded credentials |
| Accessibility and UI parity | Legacy has CS6-PARITY and accessibility decisions; experiment has `docs/A11Y.md` | Use both as checklists; re-measure against experiment UI and mark unsupported behaviors honestly |

---

## 6. Experiment task-ledger issues that must be reconciled

The experiment's own TASKS snapshot contains 168 checkboxes: 100 marked complete, 66 open, and two blocked. It already has more than enough future scope; do not create a competing task board.

Important current tasks/workstreams:
- Phase 21.5–21.6: CS6 menu/tool visuals and remaining splash/canvas-focus/clipboard/file-dialog defects.
- Phase 22.7: reconcile PRs, branches, issues, workflow runs and stale docs.
- Phase 22.8: publish local tutorials to the GitHub Wiki and verify canonical links.
- Phase 22.9: define measured coverage goals and accessibility/docs metrics; current coverage is far below the aspirational 99%.
- Phase 23: autosave/recovery, clipboard, in-app feedback, updates, file dialogs.
- Phase 24: help, search, keyboard and accessibility.
- Phase 25: extension API, local manager and catalog.
- Phase 26: brushes, stylus/touch and mobile exploration.
- Phase 27: photo/RAW/HDR, animation/video and optional AI/compute integrations.
- Phase 28: CI/repo/release/site/deployment and old-repo reconciliation.
- Phase 29: v0.3.0 release assets, workflow and native-user validation; its unchecked state must be reconciled against the already-published v0.3.0 tag and receipts.
- Phase 30: sample design, safe in-app terminal threat model, advanced Intelligence capabilities and independent quality metrics.
- Phase 31: extension ecosystem, with research/info page complete but API/runtime/catalog implementation still open.

The exact task ledger on the target branch remains the authority. Re-run checkbox counts after PR #13 and the new reset plan are integrated.

---

## 7. Evidence and source references to retain

### Experiment sources
- `AGENTS.md`, `CLAUDE.md`.
- `docs/AUDIT.md`, `docs/AUDIT-2026-11-26.md`.
- `docs/architecture.md`, `docs/architecture-review-2026-11-26.md`, `docs/adr/`.
- `docs/PLAN.md`, `docs/TASKS.md`, `docs/PROGRESS.md`, `docs/PRODUCT_ROADMAP.md`.
- `docs/features/`, `docs/tutorials/`, `docs/A11Y.md`.
- `docs/build-release.md`, `.github/workflows/`, `scripts/package-*.sh`, `scripts/version.sh`, `scripts/ci-local.sh`.

### Legacy sources to read as needed
- `docs/CS6-PARITY.md`, `docs/FEATURES.md`, `docs/PLANNING.md`, `docs/POST-MVP-ROADMAP.md`, `docs/PRODUCT-UX-ROADMAP.md`.
- `docs/RELEASE-ENGINEERING-PLAN.md`, `docs/RELEASING.md`, `docs/ARTIFACT-MATRIX.md`, `docs/TOOLCHAINS.md`.
- `docs/FEEDBACK-LOOP.md`, `docs/MARKETING-SITE-ROADMAP.md`, `docs/WEBOS-PLATFORM-INTEGRATION.md`, `docs/WEBOS.md`.
- `crates/doc`, `crates/fwpng`, `crates/engine`, `crates/io`, `crates/compose`, `crates/vector`, `crates/scripting`, `crates/webexport`, `crates/app-cli`.

Do not copy any Adobe code/assets or material with incompatible licensing. Keep the clean-room boundary and update THIRD-PARTY-NOTICES if any permitted dependency or asset is imported.

## 8. Completion checklist

- [ ] Experiment-first direction is reflected in README, AGENTS, CLAUDE, PLAN and PRODUCT_ROADMAP.
- [ ] PR #13 is reviewed by changed file, tests and workflow evidence.
- [ ] Experiment branch/worktree/stash inventory is current.
- [ ] Legacy repo's 31 open issues each have a final disposition and linked evidence.
- [ ] Legacy PRs #186/#187/#188 have an intentional final state.
- [ ] All 21 legacy remote branches are classified and preserved or safely archived/deleted only after owner review.
- [ ] `.fw.png` compatibility matrix and external-file corpus exist.
- [ ] Legacy feature donor map is complete and each item has a target disposition.
- [ ] New canonical issues are created only for still-relevant work; no duplicates.
- [ ] Experiment TASKS/PRODUCT_ROADMAP/PROGRESS and release docs match current evidence.
- [ ] Public release, platform, site, wiki and feedback gaps are recorded as actionable tasks.
