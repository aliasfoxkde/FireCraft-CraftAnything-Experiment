# Plan — from "compiles and looks right" to "proven and shippable"

Inputs: [AUDIT.md](AUDIT.md) (measured baseline, findings F-01…F-18), [ROADMAP.md](ROADMAP.md)
(completed polish pass and its open manual checks), [IMPROVEMENTS.md](IMPROVEMENTS.md)
(untriaged user reports), [architecture.md](architecture.md) (seams), [adr/](adr/) (decisions).

The plan continues the roadmap's numbering at **Phase 13** so IDs stay unique. Work items live in
[TASKS.md](TASKS.md); this file explains ordering, exit criteria and risk.

## The one-sentence diagnosis

The document model is tested and the UI is not, so every user-visible claim currently rests on a
manual Windows check — and the release story is blocked by a missing license and a drifting
version. Therefore: **make the product testable, then prove it, then ship it.**

## Ordering principle

Each phase must leave the tree green (`fmt`, `clippy -D warnings`, `test`) and must not start before
its dependencies are done, because the later phases add gates that would otherwise fail for
pre-existing reasons and pressure us to weaken them.

```text
13 Truth & hygiene ──► 14 Gates & automation ──► 15 Test foundation ──► 16 Interaction harness
                                                                              │
                                          18 Polish, perf & a11y ◄── 17 Architecture ──┘
                                                                              │
                                                                        19 Release engineering
```

Phases 13–14 are cheap and unblock honest reporting. 15–16 are the bulk of the value. 17 pays down
the structure that makes 15–16 expensive. 18 closes known user-visible defects. 19 makes shipping
repeatable.

---

## Phase 13 — Truth and hygiene

**Goal:** every statement in the repo is either true or marked unverified, and the legal/release
prerequisites exist.

Scope (F-03, F-04, F-08, F-09, F-11, F-12, F-13, F-17, F-18):

- Add `LICENSE` (owner decision), reference it from README and the bundle, include it in `dist/`.
- One version source: `app/Cargo.toml` → derive artifact names in `release.yml` from the crate
  version; bump to the intended next version (`0.2.1`+); add `rust-toolchain.toml`.
- Rewrite `app/PROJECT_STRUCTURE.md` from the real tree; reconcile `app/MECHANICS.md` with
  `app/mechanics.json` and `app/src/constants.rs`.
- Split `IMPROVEMENTS.md` into tracked tasks (fixed / open / rejected) and keep only open items.
- Remove the duplicate WASM entry point (one committed `index.html` template, referenced by CI).
- Fix the `--test-threads=1` claim; document the real CLI flags.
- Add `CONTRIBUTING.md`, `SECURITY.md`, issue/PR templates.

**Exit criteria:** no doc contradicts code (spot-checked with `rg`); `LICENSE` present and shipped;
version appears in exactly one place; every open user report has a task ID.

**Risk:** license choice is not an engineering decision — it blocks Phase 19 and must be asked for
early, not discovered at release time.

## Phase 14 — Quality gates and automation

**Goal:** the checks that protect quality run unattended, and a regression fails the build.

Scope (F-05, F-06, F-10):

- Coverage gate: `cargo llvm-cov` in CI with a floor that starts at the measured baseline and only
  ratchets up; per-file floors for `doc.rs`, `formats.rs`, `app/find_replace.rs`.
- Security gate: `deny.toml` + `cargo deny check` (advisories, bans, licenses, sources) and
  `cargo audit`; every accepted advisory documented with exposure and an expiry.
- CI matrix: Linux (fmt, clippy, test, coverage, wasm build), Windows (build + CLI smoke), plus
  `--version` / `list-commands` / `info --demo` smoke of the produced binary.
- Lint policy in `Cargo.toml` `[lints]`: correctness/suspicious/perf/style at warn, `-D warnings` in
  CI, then introduce a curated pedantic subset (start with the mechanically safe ones, document
  every allow with a reason).
- One CI definition of record; `.gitforge.yml` either generated from it or deleted.
- Dependabot for cargo + actions.

**Exit criteria:** a PR that lowers coverage, adds a vulnerable dependency, breaks formatting, or
breaks the Windows build is rejected automatically. Every gate is runnable locally by one script.

**Risk:** enabling 829 pedantic warnings at once creates a noise commit that hides real defects.
Sequence it: correctness first, pedantic later, one lint family per change.

## Phase 15 — Test foundation (logic)

**Goal:** the headless-testable half of the app is proven, including error paths.

Scope (F-02):

- `app/tests/` integration tests for the CLI verbs and MCP JSON-RPC surface (valid, invalid,
  unknown-command, malformed-JSON, missing-file responses).
- Raise `formats.rs` to ~90 %: every export format, corrupt input, unwritable path, layered PNG
  version mismatch, HTML/SVG escaping.
- Property tests for geometry: path booleans (area/idempotence/containment), transforms, gradient
  math, color conversions round-trip.
- Golden-file tests for SVG/HTML/PNG output (byte-stable or normalized).
- Error-path matrix for `app/files.rs`: dirty tabs, cancel, missing files, permission denied.
- `theme.rs` palette/contrast assertions (also feeds Phase 18 a11y numbers).

**Exit criteria:** overall coverage ≥ 45 %, `doc.rs`/`formats.rs`/`colors.rs` ≥ 85 %, `mcp.rs` ≥ 80 %;
no test depends on a display, network, or OS clipboard.

**Note on the 99 % goal:** `INSTRUCTIONS.md` asks for 99 % coverage. Chasing that on rendering code
buys little; the honest target is ≥ 90 % on logic and ≥ 70 % on the interaction surface, with the
remaining rendering branches either covered by snapshots or explicitly excluded and justified. If the
owner still wants a 99 % number, it becomes a Phase 18 ratchet once 15–16 land, not a Phase 15
blocker.

## Phase 16 — Interaction harness (the real unlock)

**Goal:** user-visible behavior is tested without a human and without Windows.

Scope (F-01, F-09):

- Add headless egui testing (evaluate `egui_kittest` for 0.37; snapshot rendering available) with a
  harness that boots the real frame at 1360×860 and 960×640.
- Convert the manual loop in `checks/` into automated snapshot tests for: startup workspace, splash,
  menu open, tool flyout, panel collapse/expand, docked + floating groups, stacked windows, tab
  overflow, file browser, find/replace, about/settings.
- Cover the gestures users reported broken: drag panel → merge/dock/float, stack top/bottom, snap to
  screen edge, detach, close last group, resize inspector (with the 118–340 px clamp).
- One keyboard map table as the single source for dispatch, menu labels, tooltips and the About
  shortcut list; test that every binding resolves and that dialogs win `Esc`.
- Menu action matrix: every enabled item produces its effect or a specific status; disabled state
  matches selection preconditions.

**Exit criteria:** `layout.rs`, `menus.rs`, `panels/*`, `dialogs/*` ≥ 70 % line coverage; the
`ROADMAP.md` open verification items 2.6/3.3/4.5 have automated proof; visual regressions fail CI.

**Risk:** snapshot tests can be brittle across platforms/fonts. Mitigate with a fixed font set,
tolerant pixel thresholds, and a single backend (software GL under Xvfb) — and keep logic assertions
alongside pixels, not instead of them.

## Phase 17 — Architecture deepening

**Goal:** the code that Phases 15–16 must test is arranged so it can be tested, before it becomes
expensive.

Scope (F-07):

- Split `app.rs` command bodies into `app/arrange.rs`, `app/edit.rs`, `app/object.rs` behind
  request/outcome seams, keeping `FireCraft` as the state owner (continue the `app/files.rs` slice).
- Extract `layout/chrome.rs` (menu bar, document tab strip, canvas footer) per architecture.md item 2.
- Move the remaining panel bodies behind the `PANELS` registry; isolate drag/resize/stack geometry
  into a pure module with no egui types.
- Introduce the `FileService` seam from architecture.md item 4 so open/save/export run headless.
- Enforce a module size budget in CI (e.g. warn > 1,200 lines, fail > 2,000 with a documented
  allowlist).

**Exit criteria:** no file above 2,000 lines without an ADR-level exception; every new command is
reachable from a test without an egui frame; architecture.md's "deepening opportunities" list is
either done or explicitly deferred with a reason.

## Phase 18 — Functional verification and defect burn-down

**Goal:** the things the user said felt broken are either fixed and proven, or documented as
limitations.

Scope (F-01, F-13, F-14, F-15, F-16):

- Triage `IMPROVEMENTS.md`: panel expand/pin/snap/group, `Alt+F` + arrow navigation, `Tab` preview,
  Align/Path panel design, branding (`Fc`), transparent splash.
- Windows-only items: Jump List rendering, app-switch flashing in full screen, taskbar identity —
  record evidence where a Windows desktop is available; otherwise keep them explicitly unverified.
- Layout floors: 960×640 and 1360×860 with every panel open, no clipping.
- Intelligence panel: unconfigured / invalid key / success / timeout, no fabrication.
- Accessibility pass: focus order on the WASM build, visible focus, keyboard-only operation of menus
  and dialogs, contrast checks against WCAG 2.1 AA (AAA where the palette allows), reduced motion.
- Performance budgets with measurements: startup time, frame time at 1920×1080 with 500 objects,
  export time per format, peak memory; record before/after rather than asserting "faster".

**Exit criteria:** zero open P0/P1 defects; every `ROADMAP.md` open check closed with evidence or
moved to a written limitation; a11y checklist and perf numbers committed.

## Phase 19 — Release engineering

**Goal:** one command produces a verified, reproducible release.

Scope (F-03, F-04, F-05):

- Deterministic bundle assembly script: exe + README + LICENSE + docs + GLOSSARY + CHANGELOG, built
  from a clean tree, never from `target/`.
- Version stamping from `Cargo.toml`; artifact names, changelog section and release notes all derive
  from it.
- Windows MSVC as the release target (GNU kept as a developer convenience), WASM + Pages deploy,
  SHA-256 for binary and archive, checksum verification step.
- Tag-driven release with smoke tests of the packaged binary; Wine smoke recorded as evidence.
- Reproducible-build check (same commit → same hash with documented caveats).

**Exit criteria:** a tag produces a release whose artifacts pass the smoke tests in CI, and whose
contents match `docs/build-release.md` exactly.

## Cross-repository direction reset — experiment-first workstream

**Status update — 2026-11-27:** PR #13 merged as `00b1fd0`, PR #14 as `62e004c`, and v0.3.1 release
prep as `3efe9c1`. All release artifacts and GitHub Pages deployment passed on that exact SHA. Phase
32 remains the UI/clipboard follow-up and Phase 33 tracks repository consolidation.

**Goal:** establish this repository as the single canonical FireCraft product and safely reconcile
the older root FireCraft repository without replacing the current app architecture.

The owner-selected direction is recorded in
[PRIMARY-REPO-RESET-AND-CONSOLIDATION-PLAN.md](PRIMARY-REPO-RESET-AND-CONSOLIDATION-PLAN.md),
with ticket/branch/PR mapping in [LEGACY-FIRECRAFT-MIGRATION-REGISTER.md](LEGACY-FIRECRAFT-MIGRATION-REGISTER.md)
and the immediate operating sequence in [AGENT-HANDOFF-2026-11-26.md](AGENT-HANDOFF-2026-11-26.md).

Required order:
1. Preserve and inventory local worktrees, stashes, active sessions, branches, tags and active PRs.
2. *(Complete for PR #13.)* Review the full changeset for clipboard/path/UI/dependency/release changes, then retain the hosted receipts and merge SHA in the progress log.
3. Reconcile experiment TASKS and release receipts against current CI and package hashes; v0.3.1 is the latest release and v0.3.0 remains an immutable historical tag.
4. Audit legacy issues/PRs/branches and map only still-useful work into one experiment task/issue.
5. Prove native Fireworks .fw.png compatibility against real files before replacing or extending the current document format.
6. Port behavior or selected algorithms/codecs behind the experiment's existing model/FileService/command seams.
7. Close or archive legacy work only after successor links and acceptance evidence exist.

**Exit criteria:** experiment is unambiguously canonical; legacy work has a complete disposition register; no active work is lost; native format compatibility has a tested decision; target tasks/PRs and release/docs point to this repository. No unrelated Git histories are merged and no duplicate task ledger is created.

This workstream does not authorize mass ticket closure, branch deletion, release publication, or production deployment.

## Phase 20 — Product gaps (the INSTRUCTIONS vision)

**Goal:** decide, scope and schedule the features that INSTRUCTIONS.md asks for beyond CS6 parity —
AVIF/HEIC, PSD/AI interop, Bridge integration, Animate-style timeline/tweening/symbols, video and
animated-GIF export, plugin/extension surface.

This phase is deliberately last and **needs an owner decision** (see below). It should enter as new
ADRs and its own roadmap section, not as a silent expansion of the polish pass.

---

## Decisions needed (blocking)

| # | Decision | Blocks | Default if unanswered |
|---|----------|--------|-----------------------|
| D1 | License for the project | Phase 13, 19; any public release | none — do not publish |
| D2 | Next version: `0.2.1` patch honesty vs `0.3.0` for the quality pass | Phase 13, 19 | `0.3.0` |
| D3 | Keep `checks/` + `investigation.cany/` in the main repo, move to a branch, or archive? | Phase 13 | keep, documented policy |
| D4 | Is a Windows desktop available for verification, or is Wine + automated UI the only path? | Phase 16, 18 | automate everything, mark Windows items unverified |
| D5 | Which Phase 20 features are in scope for the next milestone? | Phase 20 | none |

## What this plan does not do

- It does not rewrite the app or replace egui.
- It does not chase 99 % coverage as a number: it targets the untested *product surface* first, and
  lets the percentage follow. 99 % line coverage on rendering code is a poor use of effort; proven
  behavior at the seams is the goal.
- It does not weaken any existing gate to make progress; a failing gate is a finding.
