# FireCraft — Experiment-First Consolidation, Repository Reset, and Product Roadmap

> **Prepared:** 2026-11-26 (America/Chicago).
> **Canonical repository:** `aliasfoxkde/FireCraft-CraftAnything-Experiment`.
> **Baseline branch / commit:** `main` at `e415a333f6a474bd738f47bb64d56704d711f1ce`.
> **Release baseline at preparation:** published `v0.3.0` (2026-11-26); current source is preparing `0.3.1`.
> **Legacy reference repository:** `aliasfoxkde/FireCraft`.
> **Execution ledger:** [TASKS.md](TASKS.md) is the only active checkbox ledger.
> **Primary supporting docs:** [AGENTS.md](../AGENTS.md), [CLAUDE.md](../CLAUDE.md), [PLAN.md](PLAN.md), [PRODUCT_ROADMAP.md](PRODUCT_ROADMAP.md), [AUDIT.md](AUDIT.md), [architecture.md](architecture.md), [build-release.md](build-release.md), and [PROGRESS.md](PROGRESS.md).

> **Status update — 2026-11-27:** PR #13 merged as `00b1fd0`; PR #14's reconciled plan merged as
> `62e004c`; v0.3.1 release prep merged as `3efe9c1`. The all-platform release and GitHub Pages
> deployment passed on `3efe9c1`. See the newest [`PROGRESS.md`](PROGRESS.md) entry and Phases 32–34
> in [`TASKS.md`](TASKS.md) for current status and remaining acceptance work.

---

## 1. Executive decision — change the direction, not the working product

The owner has decided that the CraftAnything experiment has become the stronger product foundation: it is cleaner to navigate, more coherent visually, better organized for focused AI-agent work, and already has an independent product/release process.

**Make this repository the sole canonical FireCraft development repository.**

The former `aliasfoxkde/FireCraft` repository is now a legacy source/reference repository. It is not the destination architecture and must not be treated as the branch to which the experiment is merged. It remains valuable because it contains mature engine, file-format, import/export, parity, scripting, test, documentation, and platform work that may be selectively brought into this project after a compatibility review.

This is a reset in project direction, not a destructive restart:

- Keep the experiment's application, UI, document model, repository layout, agent instructions, tests, release pipeline, history, and published releases as the baseline.
- Keep all current experiment branches, PRs, tags, user-visible behavior, and evidence.
- Audit the older repository for capabilities and tickets that should be verified, adapted, ported, deferred, or retired.
- Port behavior and tested capabilities across the repository boundary; do not blindly merge unrelated Git histories or copy the old workspace wholesale.
- Avoid broad rewrites. Improve the new product incrementally at its existing behavioral seams.
- Make the decision explicit in README, agent instructions, roadmap, task ledger, release docs, and the migration register so future agents do not resume the previous direction accidentally.

### 1.1 Source-of-truth policy

| Resource | Authority after this reset |
|---|---|
| Experiment repository `main` | Canonical application and product history |
| Experiment `docs/TASKS.md` | Sole active task list and completion checkboxes |
| Experiment `docs/PRODUCT_ROADMAP.md` | User-facing long-term product direction |
| Experiment `docs/PLAN.md` | Engineering sequence, dependencies, and exit criteria |
| Experiment `docs/AUDIT.md` + dated audits | Evidence and measured current state |
| Experiment `docs/PROGRESS.md` | Append-only dated implementation record |
| Experiment `docs/architecture.md` + `docs/adr/` | Current architecture and binding decisions |
| Legacy `FireCraft` repository | Historical source, implementation donor, feature/ticket reference |
| Legacy PRs/issues/branches/tags | Historical records; retain links and provenance during disposition |

Do not maintain two active FireCraft implementations or two competing task ledgers. The old repository can remain public and intact while migration is in progress. Do not archive or delete it until its useful code, release assets, issue history, and provenance are accounted for.

---

## 2. Why this is the right reset

### 2.1 Experiment repository baseline

The GitHub audit on 2026-11-26 found:

- Public repository: [FireCraft-CraftAnything-Experiment](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment).
- Default branch: `main`; audited commit: `e415a333f6a474bd738f47bb64d56704d711f1ce`.
- Root application crate: `app/`; Rust 2024, Rust toolchain pinned to 1.100.0.
- Approximately 606 tracked paths at the audited tree; about 35 Rust source/test modules under `app/src` and `app/tests`.
- Existing provider-neutral `AGENTS.md`, FireCraft-specific `CLAUDE.md`, project glossary, architecture guide, ADRs, tutorials, feature pages, task ledger, product roadmap, dated audits, test harness, packaging scripts, and release workflows.
- Two remote branches were visible: `main` and `fix/version-only-release-title`.
- No open GitHub issues were returned by the issue search.
- One open PR was returned: [#13](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/pull/13).

The tree contains substantial visual/review evidence: 191 entries under `checks/`, 225 under `investigation.cany/`, 19 screenshot assets, and three capture files/directories. Existing task decision D3 explicitly retains `checks/` as evidence and `investigation.cany/` as the Fireworks mechanic record, excludes them from distribution bundles, and disallows deletion without owner approval. Preserve that rule; audit size/duplication, but do not casually prune the evidence.

### 2.2 Product surface already present

The repository documents and implements a substantial editor foundation:

- CS6-inspired application frame, ten menus, grouped tool palette, dock/panel system, multi-document tabs, Original/Preview/2-Up/4-Up views, and full-screen canvas.
- 36 tools and a shared command catalogue of 68 commands.
- Infinite/god-view canvas with page-bounded flat export semantics.
- Layers, pages, states, symbols, styles, live effects, selection, paths, and vector operations.
- Raster import/placement, paint/retouch/selection tools, color handling, and bitmap objects.
- Layered `.fw.png` save/reopen path.
- Find & Replace for text/names and colors, including fills, strokes, gradients, and live-effect colors.
- In-app file browser, CLI, MCP JSON-RPC 2.0 surface, and Intelligence panel with explicit unconfigured/error behavior.
- Native Rust/egui application and WASM build path.

These are repository claims and implementation findings, not a statement that every feature is polished or verified on every platform. Use the feature pages, tests, current audit, GUI receipts, and target-specific release evidence as the proof.

### 2.3 Existing quality and architecture foundation

Positive engineering assets to retain:

- One Rust application crate with module boundaries already extracted around file intents, arrange/edit/object behavior, panel registry, panel geometry, layout chrome, and rendering.
- `FileService` seam for headless file workflows.
- A headless egui harness that boots the real UI at 1360×860 and 960×640, feeds key events, and inspects application state.
- Cross-module CLI, MCP, hygiene, format, and interaction tests.
- A strict formatting/Clippy/test baseline, coverage ratchet, dependency policy, module-size checks, documentation/link checks, and CodeQL workflow.
- Pinned Rust toolchain and GitHub Actions dependencies.
- A cleanly organized `docs/` source of truth and clear agent instructions.

The dated 2026-11-26 progress/audit material reports 180 automated tests and a measured coverage range around 67–68% lines, 69–70% functions, and about 70% regions on the cited revisions. The latest cited run is not automatically proof for a later commit. Re-run gates on the exact current SHA before recording current coverage or readiness. The owner's aspirational 99% target is not achieved and must remain separated into meaningful metrics rather than being reported as one vague quality score.

### 2.4 Current release baseline

The GitHub Releases page contains a published `v0.3.0` release. Its asset set included:

- Linux x86_64: AppImage, Flatpak, tar.gz, and DEB.
- macOS universal: DMG and ZIP.
- Windows x86_64: portable ZIP.
- Browser: `wasm32-web.zip`.
- Integrity/metadata: `PACKAGE-MANIFEST.json`, `RELEASE-MANIFEST.json`, `SHA256SUMS.txt`, and `SOURCE-SHA.txt`.

This is materially ahead of the old repository's previously published Linux-only release set. It does **not** prove every advertised platform is certified. Existing release docs explicitly say macOS output is unsigned/unnotarized until signing credentials are configured. The current workflow builds on hosted Windows, macOS, and Linux runners, but build/CLI smoke is not the same as interactive user acceptance on Windows 11 or macOS. No RPM or Linux ARM64 artifact was in the observed release. A standalone CLI asset was not listed separately. There is not yet a complete native installer matrix (for example MSI/Inno Setup) or a signed/SBOM-attested release trust chain.

Keep the current v0.3.0 release intact. Do not retag or replace its published bytes. Treat future releases as new immutable tags.

---

## 3. What was found in the legacy FireCraft repository

The older [`aliasfoxkde/FireCraft`](https://github.com/aliasfoxkde/FireCraft) repository is not simply a less-polished copy. It has a different and substantially more modular Rust workspace, with a broader engine and compatibility surface.

Its 2026-11-26 GitHub snapshot showed:

- 31 open issues.
- 21 visible remote branches.
- PR #186 (`temp/consolidation`) closed **unmerged**.
- PR #187 (`feat/experiment-merge`) closed **unmerged**.
- PR #188 still open, containing the previous experiment-to-root-repository consolidation/1.0 planning direction.
- The old branch set includes `feat/experiment-merge`, `temp/consolidation`, `feat/phase1-visual-polish`, `feat/inspector-field-sets`, `feat/post-mvp-execution`, `feat/release-provenance`, `feat/wasm-cdn-deploy`, `fix/web-deploy-p0`, `fix/xtask-gate-repair-2026-11-24`, `release/v0.2.5`, and other topic/audit branches.

The old codebase has approximately 957 tracked paths, including about 296 entries under `crates/`, 382 under `docs/`, packaging, fuzz, corpus, website, and WASM components. Its key technical areas include:

| Legacy source area | Why it matters | Migration posture |
|---|---|---|
| `crates/doc`, `crates/fwpng` | Structured document model and native Fireworks PNG chunk handling, including `mkTS`, `mkBT`, and unknown-node preservation | Highest-priority compatibility investigation; port only with explicit adapter and real-file tests |
| `crates/engine` | Shared command registry, typed commands, history/journal/session, access rules, bitmap and vector operations | Reuse command behavior/tests selectively; do not import the entire workspace as a first step |
| `crates/io` | Mature codecs and format fidelity paths, including PSD and additional raster/vector formats | Candidate for selective codec/behavior port behind the experiment's format boundary |
| `crates/compose`, `crates/filters`, `crates/raster` | Rendering/compositing, masks, effects, raster storage | Benchmark and port proven behavior only where experiment gaps justify it |
| `crates/vector`, `crates/geom`, `crates/text`, `crates/color` | Geometry, advanced paths, vector editing, text and color behavior | Compare output/semantics, then port tested algorithms or adapters |
| `crates/scripting` | ExtendScript/CS6 DOM scripting host | Strategic capability; future API boundary and security review required |
| `crates/webexport`, `web/`, `WEBOS.md` | HTML/CSS/sprite export and browser-host integration | Reuse specs and test cases; verify host contract before implementation |
| `crates/app-cli` | Broader headless file/automation workflow | Compare with experiment CLI/MCP and avoid duplicate commands |
| `docs/CS6-PARITY.md`, `docs/FEATURES.md` | Detailed UI parity and capability maps | Reuse as source material; re-audit every claim against the current experiment |
| `docs/POST-MVP-ROADMAP.md`, `docs/PRODUCT-UX-ROADMAP.md` | Broader feature aspirations and prioritization | Reconcile against the new product roadmap; keep one active roadmap |
| `docs/RELEASE-ENGINEERING-PLAN.md`, `docs/RELEASING.md`, `docs/FEEDBACK-LOOP.md` | Release, feedback, and trust patterns | Port useful operating rules into experiment docs; do not copy old environment assumptions |

### 3.1 The most important technical compatibility risk

The two repositories' `.fw.png` claims must not be treated as equivalent without proof.

The experiment documentation describes a normal page-bounded PNG with a versioned embedded project payload (iTXt) that preserves pages, layers, states, symbols, styles, and off-page objects. The legacy repo has a dedicated Fireworks PNG implementation with native chunks such as `mkTS` and `mkBT`, tiled raster data, and unknown-node preservation.

Those may represent different levels or variants of compatibility. The agent must compare both implementations against a legally usable corpus of real Fireworks-generated files. The migration must prove:
- Read and write of actual Fireworks files, not just self-round-trip of files written by the same FireCraft implementation.
- Preservation of known and unknown chunks/nodes.
- Raster tile dimensions, layer/object references, metadata, thumbnails, and export intent.
- Correct visual output and editability after open/save/reopen.
- Safe handling of malformed, truncated, oversized, and version-mismatched input.
- Non-destructive behavior where the format contains unsupported constructs.

Do not replace the experiment's document model with the old model just to obtain a parser. Prefer a boundary adapter with explicit feature limitations. Preserve both test suites and golden fixtures as migration evidence.

### 3.2 Other legacy capabilities that require a feature-level decision

The old registry reported 257 commands and a CS6 DOM matrix of 268 carried / 3 partial / 0 missing across 356 verbs on its dated baseline. Those are not equivalent to the experiment's 68 command entries or 36 visible tools. A command count does not establish user workflow completeness.

Re-audit and prioritize:
- Advanced bitmap selections/retouch and non-destructive masks.
- Advanced path editing, Bezier handles, compound paths and polygon operations.
- Rich text, text-on-path, typography, and text layout fidelity.
- Symbols, nested symbols, nine-slice, shared styles, and reusable assets.
- Pages/states, animation timeline, onion-skin, playback and animated export.
- PSD/PSB and other format compatibility, color profiles and print/CMYK.
- HTML/CSS/sprite export, slices/hotspots, WebOS and browser-host bridge.
- ExtendScript compatibility, command registry semantics, MCP and CLI parity.
- Recovery, journals/history, large-document behavior, performance, accessibility, and user diagnostics.

For each, the decision must be one of: already present and verified in experiment; present but UX/evidence incomplete; port behavior; port selected source after adapter; defer; reject/not applicable. Include exact evidence and a next action.

---

## 4. Target architecture and repository rules

### 4.1 Keep the experiment's current architectural direction

The experiment is a modular single application crate. Preserve that design unless a measured requirement justifies extraction.

- Keep `app/src/doc.rs` as the target document-state authority until a deliberate ADR changes it.
- Keep UI behavior behind app methods and narrow modules; dialogs own layout, app methods own multi-step effects.
- Keep file operations behind `FileService`.
- Keep rendering, document semantics, UI layout, protocol handling, and platform integration separated.
- Keep CLI and MCP on the same command implementation path.
- Keep platform code optional and isolated.
- Preserve undo/redo, dirty state, selection ownership, document switching, export immutability, and god-view/page-boundary semantics.

Do not import the old 12-crate workspace wholesale, create two document models, or create parallel command registries as a shortcut. A crate extraction is warranted only when it produces a genuinely reusable/testable boundary with a clear owner and a deletion test.

### 4.2 Architecture improvements that are justified by current evidence

The 2026-11-26 architecture review identifies these high-value seams:
1. Pointer gesture routing in `app.rs`: extract one gesture family at a time, preserving selection, undo, canvas bounds, cancellation and lost-focus behavior.
2. Format codecs in `formats.rs`: split one codec at a time only after golden, round-trip, malformed-input and error-path tests exist.
3. MCP/CLI protocol boundary in `mcp.rs`: separate transport parsing/serialization from command execution only if it reduces coupling and keeps both transports on the shared command catalogue.

The existing architecture guide's "deletion test" rule is mandatory: removing a proposed module should make its behavior disappear from the old owner. Reject extractions that merely replace a direct call with a thin forwarding wrapper.

### 4.3 Single sources of truth

- One task ledger: `docs/TASKS.md`.
- One product roadmap: `docs/PRODUCT_ROADMAP.md`.
- One implementation/quality plan: `docs/PLAN.md`.
- One current-state audit: `docs/AUDIT.md`, with dated snapshots retained separately.
- One release workflow and one version source: `app/Cargo.toml` + `scripts/version.sh`.
- One user-facing capability source: `docs/features/` and a refreshed generated/maintained feature index.
- One architecture authority: `docs/architecture.md` + ADRs.
- One authoritative source SHA per release.

Do not create another overlapping task list or leave legacy root FireCraft docs as a second live source of truth.

---

## 5. Phase 0 — Freeze the direction and preserve work

**Goal:** make the experiment-first decision unambiguous and protect every active change.

1. Confirm current checkout, branch, HEAD, dirty/staged/untracked state, worktrees, stashes and remotes on the owner's machine.
2. Preserve local agent work in the experiment repo, especially the current `fix/version-only-release-title` branch and any unpushed changes.
3. Create a local, owner-controlled backup or reference tag before integration work. Never use destructive reset/clean/stash-pop/force-push to make the checkout match a remote SHA.
4. Re-fetch GitHub state for both repositories; this document is a dated snapshot, not a live state guarantee.
5. Mark experiment `main` as the integration base after confirming no newer local branch is the intended baseline.
6. Add this direction policy to agent entry points and roadmap docs.
7. Keep the older repository available as a read-only donor during migration. Do not archive it yet.
8. Keep the published v0.3.0 release and tag immutable.

**Exit evidence:** dated local/remote branch + worktree report, backup/reference SHA, owner confirmation that the experiment is canonical, no lost local edits, and current GitHub PR/issue/tag inventory.

## 6. Phase 1 — Reconcile the experiment's own live work first

**Goal:** land or disposition the only open experiment PR and remove confusing task/status drift before importing legacy features.

### 6.1 PR #13 was not a one-line release-title change

This subsection preserves the review scope at the time the plan was drafted. The work is complete:
PR #13's full changeset was reviewed, its title/body were corrected, exact-SHA hosted checks passed,
and it was merged as `00b1fd0` on 2026-11-27. See [PROGRESS.md](PROGRESS.md) for the release receipt.

At the audit snapshot, [experiment PR #13](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/pull/13) was open, mergeable, two commits ahead and not behind `main`, with **20 changed files (+787 / -249)**.

Its title mentions only the release title, but the diff also includes:
- Browser clipboard image paste bridge and WASM-specific payload handling.
- Selected path-node state and multi-anchor selection.
- Path simplification, reversal, closure, delete-point and alignment behavior.
- A safer truthful unsupported-operation response for text-to-path conversion.
- File-browser sizing/viewport behavior.
- Tool palette labels and UI/menu cleanup.
- Dependency/lockfile and release workflow updates.
- Task/audit/progress documentation changes.

Do not assess or merge this PR by its title. Produce a file-level review, verify each behavior and the release-title change, update the PR description/title to represent its real scope, run all relevant tests including WASM/browser behavior, and merge only after required hosted checks pass. If changes need separation, split them into reviewable PRs rather than dropping unique work.

### 6.2 Repair stale status without erasing evidence

At preparation time, the main task ledger had Phase 29 release tasks unchecked even though the public v0.3.0 release and assets existed. That reconciliation is now supplemented by the v0.3.1 receipts in Phase 34. Do not use "published asset exists" as proof of install/launch certification; native install and interactive acceptance remain separate.

Also reconcile:
- Phase 22.7 (repo/PR/branch/issue audit).
- Phase 22.8 (tutorial publication to GitHub Wiki).
- Phase 22.9 (coverage target measurement; current evidence is around 67–68%, not 99%).
- Phase 23 (recovery, feedback, update flow).
- Phase 28 (CI/release/deployment and repository integration).
- Phase 29 (v0.3.0 publication and native-user validation).

**Exit:** PR #13 is merged, split, or retained with a precise next action; required checks are green on the chosen SHA; task statuses match actual evidence; no hidden local work is lost.

---

## 7. Phase 2 — Legacy feature and repository reconciliation

**Goal:** determine what the older repo contributes without creating a second active implementation.

Use [LEGACY-FIRECRAFT-MIGRATION-REGISTER.md](LEGACY-FIRECRAFT-MIGRATION-REGISTER.md) as the detailed mapping. The first task is an evidence matrix, not file copying.

### 7.1 Required comparison dimensions

For every donor feature:
- Legacy path/module, symbol and tests.
- Experiment path/module, symbol and tests.
- User-visible result and intended workflow.
- Document model / serialization dependencies.
- Command API and undo/redo semantics.
- Native vs WASM behavior and platform dependencies.
- License/provenance and asset attribution.
- Performance/memory/size cost.
- Test fixture and acceptance evidence.
- Disposition and target issue/task ID.

### 7.2 Porting method

Preferred order:
1. Port missing behavior through the experiment's existing API and model.
2. Port pure algorithms or codecs behind a narrow adapter with deterministic tests.
3. Port larger subsystems only after a written architecture decision compares API fit, dependencies, binary/WASM size, licensing and migration cost.
4. Retain the donor implementation as historical reference until the target implementation passes.
5. Remove duplicated logic only after parity tests and owner review.

Never use `git merge --allow-unrelated-histories` to combine the repositories as a shortcut. Cherry-pick only self-contained commits with understood dependencies. For larger incompatible features, reimplement against the target architecture and reference the source commit/tests.

### 7.3 Priority donor areas

- **P0 compatibility:** real native `.fw.png` chunk fidelity and unknown data preservation; file/document recovery; safe open/save/export; real-world sample corpus.
- **P1 editing:** raster pixel operations, masks, selection modes, vector/path algorithms, text fidelity, effects, styles, pages/states, animation, advanced export.
- **P1 automation:** command registry coverage, browser bridge parity, CLI/MCP interoperability and scripting feasibility.
- **P2 ecosystem:** extension APIs, resource packs, WebOS embedding, HTML/CSS/sprite workflows, color/print management.
- **P3 expansion:** mobile, HDR/RAW, video, collaborative/cloud editing and advanced AI features; each requires a separate feasibility/security decision.

**Exit:** every high-value legacy capability has a verified experiment status, migration disposition, owner, test plan, and canonical target task. No legacy issue or branch is left floating without a decision.

---

## 8. Phase 3 — Make the product reliably usable

**Goal:** close the daily-use reliability gaps before pursuing breadth.

Prioritize complete end-to-end workflows:
1. First launch, welcome/demo, new/open/recent/save/save-as/close/quit.
2. Multi-document state, dirty indicators, safe switching, atomic save and recovery.
3. Vector creation/selection/path editing/transform/align/group and undo/redo.
4. Raster import and actual pixel editing (brush/erase/crop/selection/retouch) plus undo/redo.
5. Text create/edit/format and explicit limitations for unsupported operations.
6. Layers/pages/states, visibility, lock, reorder, rename and persistence.
7. Fill/stroke/gradients/styles/live effects.
8. PNG/JPEG/SVG/GIF/WebP/PSD and native `.fw.png` as individually verified formats, not a single all-or-nothing label.
9. Desktop and browser clipboard, drag/drop, file dialogs, download and open flows.
10. Corrupt/unsupported input, permission denied, full disk, export failure, crash-like restart and recovery.

Do not call raster editing complete if only bitmap objects can be placed and transformed. Do not call animation complete merely because states or GIF import exists. Do not call interoperability complete from self-round-trip alone.

**Exit:** P0 user workflows pass with regression tests on native and WASM where supported; no known P0 document corruption/data-loss defect remains; UI controls have behavior or are explicitly disabled.

---

## 9. Phase 4 — Architecture and agent productivity

**Goal:** keep the qualities that make the experiment easier for both people and agents to work in.

- Keep modules aligned to one user behavior/domain.
- Maintain explicit owner files and dependency directions.
- Use the single task list and a short next-action entry, not repeated planning documents.
- Put current state in the audit, history in PROGRESS, future direction in PRODUCT_ROADMAP, decisions in ADRs.
- Enforce one task/PR workstream per agent with declared file ownership.
- Require agents to read AGENTS/CLAUDE, inspect status and tests before edits, and report exact verification.
- Avoid multiple agents concurrently editing shared files such as `app.rs`, `formats.rs`, `menus.rs`, `TASKS.md`, release workflow and package scripts.
- Keep automated code review findings tied to real code/tests; do not blindly apply bot suggestions.
- Continue the architecture review's three seams in order of payoff: pointer gesture routing, format codecs, protocol boundary.
- Keep module-size checks as warning/fail guardrails, but do not extract code just to satisfy a line count.
- Preserve the headless interaction harness; add interaction tests at the behavior seam before adding visual snapshot baselines.

**Exit:** a clean agent workflow, low-conflict file ownership, focused PRs, no duplicated behavior, and docs/CI that tell agents what is actually true.

---

## 10. Phase 5 — Test, reliability and security ratchet

**Goal:** raise confidence by user workflow and risk, not by a vanity percentage.

1. Re-run `./scripts/ci-local.sh` on the exact baseline and hosted CI on the same SHA.
2. Verify formatting, strict Clippy, full tests, docs checks, module-size, dependency audit/deny, CodeQL, coverage, Windows build/CLI smoke, WASM release build and packaging jobs.
3. Re-run coverage with the pinned toolchain. Preserve current floor and only raise it from an evidenced run. Report line/function/region/branch metrics separately; branch metrics are not currently emitted in the cited Cobertura artifact.
4. Add behavioral tests for save/recovery, file format round-trips, corrupt input, selection/path behavior, canvas/panel gestures, clipboard, and generated export bytes.
5. Add format corpus/property/fuzz tests for parsers, serialization, geometry and imported media.
6. Test supported target workflows, not only cross-compilation.
7. Keep `cargo audit` and `cargo deny` findings visible; document accepted advisories, upstream state and expiry/review date.
8. Track accessibility criterion-by-criterion. Do not claim WCAG conformance from palette contrast tests alone.
9. Keep performance budgets measured on named hardware/build modes; do not extrapolate debug/software rendering measurements to release GPU performance.

**Exit:** exact-SHA CI green; known dependency/security risks dispositioned; core workflow tests automated; performance/coverage baselines reproducible.

---

## 11. Phase 6 — Feedback, recovery and controlled improvement

**Goal:** users can report a problem and agents can work from a reproducible ticket without uploading private artwork.

Port the useful contract in the legacy `docs/FEEDBACK-LOOP.md`, but update it to point to this repository and its issue templates.

Feedback surface:
- Report bug.
- Request feature.
- Suggest change.
- Report documentation/accessibility problem.
- Copy/export diagnostics offline.

Diagnostic schema should include only minimal, user-consented information: app version/build, native/WASM target, OS/architecture, browser when relevant, user-provided description/repro steps, error code and bounded sanitized logs. No document contents, file paths/names, screenshots, credentials, environment dump or automatic attachment by default. Show a preview and explain public GitHub issue visibility. Offer clipboard/file fallback without a network connection.

Workflow:
`User report → deduplicate/triage → reproduce → canonical issue → isolated worktree → fix + regression test → PR → CI → human review → staging/beta → human-approved production`.

Autosave/recovery, clipboard behavior, updates, privacy and support all belong in this reliability loop. Agents do not close their own issue, merge their own PR, or publish production.

**Exit:** feedback creates a useful issue or offline report; diagnostics are user-reviewed; report-to-fix-to-release can be demonstrated on a test issue.

---

## 12. Phase 7 — Distribution, public site, Wiki and release process

### 12.1 Release direction

The v0.3.0 release is a useful platform baseline. Preserve its current asset names and manifests. Future release work should:
- Keep one tag/version/source SHA for every artifact.
- Validate archive/package inventory and manifest/checksum before publication.
- Record platform, OS version, architecture, runner/toolchain and tests.
- Add independent installation/launch/user workflow receipts per target.
- Decide whether Windows needs a signed installer in addition to portable ZIP.
- Configure Windows signing and macOS Developer ID/notarization only through protected secrets and a documented owner process.
- Assess RPM and Linux ARM64 from user demand/maintenance burden, not checklist pressure.
- Add SBOM and provenance/attestation if they can be generated and independently verified.
- Consider a draft/manual-approval stage for future production releases; the current tag workflow's automatic publication behavior must be reviewed before enabling protected stable releases.
- Treat browser deployment, self-hosted WASM ZIP and marketing website as separate targets with source-SHA evidence.

### 12.2 Marketing site

The experiment repository currently has a release/WASM workflow but no root `site/` marketing application in its tracked tree. The legacy repository has a small `site/` and Cloudflare deployment documentation/workflow; audit them as a donor rather than assuming they are ready to transplant.

Build one public front door that accurately explains:
- FireCraft's purpose and current verified capabilities.
- Screenshots/video of the actual experiment-based UI.
- Supported platform/artifact matrix and limitations.
- Browser try/demo entry, GitHub downloads and installation.
- Tutorials, documentation/wiki, issue/feedback and contribution links.
- License, privacy, security and Adobe non-affiliation.

Do not mix the static editor WASM output with marketing files. Use separate project/output paths, preview/staging/prod environments, scoped secrets and rollback receipts.

### 12.3 Documentation and Wiki

Keep repository docs as the canonical detailed source. Publish the user-facing tutorial/docs subset to GitHub Wiki only after link/heading validation. Add a repeatable sync/publish mechanism and test it. Never assume a `docs/wiki` directory or local tutorial automatically becomes the live Wiki.

Refresh README and docs index with direct links to:
- Getting started / tutorials.
- Feature inventory and supported file formats.
- Platform downloads and release notes.
- Architecture and contributing guide.
- Known limitations, accessibility and security.
- Roadmap and feedback.

**Exit:** accurate public product pages, working verified downloads, current Wiki/tutorials, and release instructions consistent with GitHub.

---

## 13. Phase 8 — Full issue, PR, branch and worktree closeout

Use [LEGACY-FIRECRAFT-MIGRATION-REGISTER.md](LEGACY-FIRECRAFT-MIGRATION-REGISTER.md) for the 31 legacy open issues and old branch/PR inventory.

### 13.1 Rules

- Read every issue body and all comments, labels, linked PRs, task IDs, and current implementation before disposition.
- Each issue must become resolved, duplicate, superseded, deferred, blocked, active, or not planned—with reason and successor link where relevant.
- Do not create 31 copies of legacy tickets in the experiment repo.
- Create canonical experiment issues only for work that remains valuable and has a defined acceptance test.
- The new issue must link to the old issue and carry over unique reproduction steps, design requirements and evidence.
- Close/transfer the legacy item only after the new canonical record is present or the old acceptance is actually met. Preserve old history.
- Keep infrastructure issues in the repository that owns the GitForge/runner defect; do not pollute the product board with unrelated runner operations.
- Close PRs only when their unique changes are integrated or explicitly rejected/deferred.
- Delete branches only after all unique commits, open PRs, stashes, active worktrees and agent ownership have been checked.
- Never force-push, reset active worktrees, or use broad branch-prune commands as a cleanup strategy.

### 13.2 Required reports

1. Experiment branch/PR/issue/worktree audit.
2. Legacy branch/PR/issue crosswalk.
3. Feature donor/migration decision matrix.
4. Canonical new issue list with old ticket cross-links.
5. Closeout ledger showing what is closed, why, where the successor lives and the validating commit/test.
6. Preservation/archival report for historical source, checks, screenshots, references and tags.

### 13.3 Legacy PR disposition

- #186 and #187 are closed unmerged. Do not reopen or merge them as a batch. Mine their unique changes and evidence through the migration register.
- #188 is an open old-repository planning PR built around the previous direction. The new experiment-first plan supersedes that direction. Leave it untouched during the audit; once this new plan is accepted, close it as superseded with a link to the canonical experiment plan if the owner agrees.
- Experiment PR #13 was reviewed and merged as `00b1fd0`; future donor work should use the Phase 33 migration sequence.

---

## 14. Product scope and future feature sequence

Do not chase every legacy issue or every feature from the wishlist at once. The following is the prioritization model; detailed items remain in [PRODUCT_ROADMAP.md](PRODUCT_ROADMAP.md), [TASKS.md](TASKS.md), feature pages and the migration register.

### P0 — trust and complete daily workflows

- One authoritative repository and task/roadmap system.
- PR #13 audited and correctly integrated.
- Real `.fw.png` compatibility verified with external samples.
- Document lifecycle, undo/redo, save integrity and crash recovery.
- Reliable core vector and actual bitmap pixel-editing workflows.
- Layers/pages/states and reliable text editing.
- Core export and import correctness.
- Native and WASM parity for shared capabilities.
- Current CI, tests and release receipts on exact SHAs.
- In-app feedback and clear limitations.
- Accurate installation/docs/support information.

### P1 — professional creative workflows

- Advanced path editing and vector boolean operations.
- Raster selection, masks, brushes, retouch, gradient/fill, filters and color controls.
- Live effects, styles, reusable symbols, shared assets and inspectors.
- Optimize/export panel tied to actual exported bytes.
- Pages, states, animation timeline/playback/onion-skin where supported.
- More format compatibility (PSD/PSB, TIFF, animated formats, SVG, WebP and other high-demand inputs/outputs).
- CLI/MCP improvements and better in-app help/search.
- Autosave/recovery and robust clipboard/drag-drop.
- Real Windows/macOS GUI review and support matrix.

### P2 — extensibility and workflow automation

- Versioned extension API and permission model.
- Local resource/extension manager and safe-mode/update/rollback.
- Community extension catalog and author documentation.
- ExtendScript/CS6 DOM compatibility subset.
- HTML/CSS, slice/hotspot, sprite and web-asset export.
- WebOS/browser embedding SDK and stable host contract.
- Color management/ICC, typography/font workflows, brush packs and print output.
- Batch workflows and automation templates.

### P3 — advanced, separately gated expansion

- Android/mobile/touch/stylus.
- RAW/HDR/OpenEXR and advanced color pipelines.
- Video timeline/editing/encoding.
- Collaboration/cloud documents/history.
- Advanced AI/skills/memory/scheduled work; only with explicit permissions, privacy, provenance, local-first behavior and clear user action.
- Marketplace scale and complex third-party runtime sandbox.

Any change in feature priority must be reflected in the single product roadmap and the single task ledger, not another parallel plan.

---

## 15. Definition of a successful consolidation

The reset is complete only when:

1. The experiment repository is unambiguously canonical.
2. All active experiment work and PR #13 are reconciled.
3. The older repo's useful features, issues and decisions are inventoried with an explicit disposition.
4. Native Fireworks document compatibility has an evidence-backed decision and test suite.
5. No duplicate application/document/command implementation is kept active without a reason.
6. P0 editing workflows are reliable and verified on target surfaces.
7. CI is green on the exact current source SHA; coverage/quality metrics are reproducible.
8. Release assets, platform support claims and deployment state are evidence-backed.
9. README, feature pages, tutorials, Wiki and release docs describe the same product.
10. Legacy issues/PRs/branches have a documented final disposition and preserve links/history.
11. In-app feedback and the fix/review/release loop are usable.
12. The owner can give a new agent this repository, `AGENTS.md`, `CLAUDE.md`, and `docs/TASKS.md` without needing to explain the old repository merger again.

---

## 16. Agent handoff report contract

Every phase handoff must state:

- Starting/ending SHA and branch.
- Local dirty state, worktrees and active agent ownership.
- PRs/issues/branches reviewed and exact disposition.
- Donor legacy files/commits reviewed and whether behavior/code was reused.
- Source files and modules changed.
- Exact commands and test/CI results.
- Release/package/target platform evidence.
- New or changed tasks and docs.
- Known gaps and the one next action.

**Do not report a phase as complete based on a plan, code review, successful compile, screenshot, or merged PR alone. Attach the evidence required by that phase.**
