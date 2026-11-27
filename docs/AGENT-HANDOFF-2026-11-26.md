# FireCraft — Experiment-First Reset: Agent Handoff

> **Canonical repository:** aliasfoxkde/FireCraft-CraftAnything-Experiment
> **Legacy donor:** aliasfoxkde/FireCraft
> **Baseline SHA:** e415a333f6a474bd738f47bb64d56704d711f1ce
> **Full plan:** [PRIMARY-REPO-RESET-AND-CONSOLIDATION-PLAN.md](PRIMARY-REPO-RESET-AND-CONSOLIDATION-PLAN.md)
> **Migration register:** [LEGACY-FIRECRAFT-MIGRATION-REGISTER.md](LEGACY-FIRECRAFT-MIGRATION-REGISTER.md)
> **Only active task ledger:** [TASKS.md](TASKS.md)

---

## Mission

The experiment repository is now the primary FireCraft product. Preserve its cleaner UI, repository organization, architecture, agent workflow, tests, and published releases. Treat the older FireCraft repository as a read-only donor/reference source during migration. Selectively port valuable, verified capabilities; do not merge the repositories' histories or replace the experiment's architecture wholesale.

The immediate objective is a reliable, maintainable, feature-rich FireCraft product—not a large mechanical merge or a new architecture project for its own sake.

## Phase 0 — Required first session

Do not begin broad refactoring or new feature implementation until this baseline is complete.

1. Read AGENTS.md, CLAUDE.md, docs/PLAN.md, docs/TASKS.md, docs/AUDIT.md, docs/PRODUCT_ROADMAP.md, docs/architecture.md, and the two new documents above.
2. On the owner's development machine, inspect all local FireCraft and experiment clones, worktrees, branches, stashes, dirty/untracked changes, and active agent sessions. The GitHub API cannot reveal unpushed commits, local edits, or an agent's active worktree.
3. Confirm the current experiment main SHA and refresh all remote PR, branch, issue, tag, and release states. The baseline in the docs is dated and may already have changed.
4. Preserve all local work. No reset, clean, stash-pop, forced update, branch deletion, or worktree pruning during the inventory.
5. Audit experiment PR #13 by its **full 20-file patch**, not its misleadingly narrow title. It contains WASM image clipboard support, path-node editing, UI/file-browser changes, dependency changes, docs and release workflow edits. Produce a file-by-file review with test evidence and identify whether it should be merged as-is or split into focused PRs.
6. Reconcile the experiment's Phase 22.7/28.x/29.x task states with current PRs, workflow runs and the published v0.3.0 release. Keep dated audit evidence; do not rewrite historical reports.
7. Produce the Phase 0 baseline report and update TASKS.md with precise migration tasks. Stop and report blockers before destructive operations.

## Phase 1 — Legacy donor audit

Use LEGACY-FIRECRAFT-MIGRATION-REGISTER.md.

- Read all 31 legacy open issues and their comments, linked PRs/tasks and current code.
- Re-fetch old PR #186/#187/#188 and the 21-branch inventory. #186/#187 are closed unmerged; #188 describes the superseded direction.
- Do not close, transfer or recreate tickets until a target disposition and successor link are ready.
- Compare legacy code with the experiment at the behavior level. For every proposed donor, record paths/symbols/tests, dependencies, target module, license/provenance, platform impact, and acceptance criteria.
- Keep infrastructure-owned GitForge issues outside the product backlog.
- Preserve old releases, tags and historical commits.

## Priority 1 — Prove native Fireworks compatibility

The most important donor question is whether the experiment's versioned iTXt-based project payload fully supports actual Fireworks files, or whether the legacy mkTS/mkBT chunk implementation carries formats and unknown data that the experiment currently cannot preserve.

Create a legal sample corpus of genuine Fireworks-generated files. Test open → inspect → edit → save → reopen → export. Cover raster tiles, layers, pages/states, symbols, metadata, unknown chunks/nodes, thumbnails and malformed inputs. Keep the experiment document model as the target unless an ADR establishes a better design. Prefer a format adapter over replacing the model.

## Priority 2 — Port useful behavior selectively

Order:
1. Native .fw.png fidelity and document integrity.
2. Real-world raster/vector import/export, codec coverage and malformed-input handling.
3. Bitmap selection/retouch/masks, advanced path operations, text, symbols and reusable styles.
4. Pages/states and animation only where the target has a complete user workflow.
5. CLI/MCP, scripting, HTML/CSS/sprite export and browser/WebOS host APIs.
6. Color management, brushes, accessibility, advanced AI and other roadmap items.

A feature is not complete because an engine command or menu row exists. Require an end-user path, undo/redo behavior, platform boundaries, tests and documentation.

## Required implementation rules

- Keep one document model, one command authority, one file-operation seam and one task ledger.
- Do not add the legacy workspace wholesale or create a second active FireCraft app.
- Use the architecture review's deletion test for every new module extraction.
- Put tests at the behavior seam; preserve existing lint, module-size, docs, security and coverage gates.
- Treat the aspirational 99% coverage target as multiple defined metrics. Current experiment evidence is around 67–68% lines, not 99%.
- Treat target build, package, install, launch, interactive acceptance, signing and publication as separate evidence states.
- Never claim Windows/macOS GUI acceptance based on cross-compilation or Wine.
- Keep FireCraft independent of Adobe code/artwork and review all third-party licenses/assets.
- Update TASKS.md, the relevant issue, PROGRESS.md and affected user/developer docs in the same reviewed change.
- Do not merge your own PR, close your own issue, publish a stable release or deploy production without owner authorization.

## Definition of done for each work package

Record:
- Starting and ending SHA; branch/worktree and clean/dirty state.
- Exact files/modules changed and why.
- Source donor commit/path if ported.
- Tests and commands run with pass/fail/skipped results.
- Native/WASM/platform evidence and package hashes where relevant.
- Updated task IDs and docs.
- Known limitations and one next action.

## First deliverables

1. Dated local + remote baseline / ownership report.
2. PR #13 file-level review and merge/split recommendation.
3. Legacy issue/PR/branch disposition report with target successor IDs where necessary.
4. .fw.png format/real-file compatibility plan and test corpus proposal.
5. Updated single task ledger with a dependency-ordered experiment-first sequence.

Do not start broad code migration until the owner can review these five deliverables.
