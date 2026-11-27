# Documentation structure and workflow evidence plan

## Purpose

FireCraft documentation should guide a reader from a product overview into a focused task, feature
explanation, reference page, or engineering detail. The docs should be generated and maintained from
verified use of the application wherever possible. One tested workflow should provide the reusable
steps for the automated test, visual review, tutorial, feature reference, issue report, and release
notes.

This is the process plan and the baseline audit. It does not claim that all workflows have been
visually exercised. Current release and verification facts live in [the audit](AUDIT-2026-11-27.md),
[the progress log](PROGRESS.md), and [the active task list](TASKS.md).

## Information architecture

Use progressive detail. Each level answers one question and points to the next narrower level.

| Level | Page role | Reader question | Content |
|---|---|---|---|
| 0 | Repository `README.md` | What is FireCraft and what can I download? | Product description, current release, demo links, concise documentation entry points. |
| 1 | `docs/README.md` | Where should I read next? | Named categories with plain descriptions and links to category indexes. |
| 2 | Category index | Which guide, tutorial, reference, or engineering page covers this area? | Small curated lists grouped by purpose and feature area. |
| 3 | Focused page | How does this feature or workflow work? | Prerequisites, tested steps, expected results, screenshots, limitations, and links to related detail. |
| 4 | Evidence and implementation | What proves this behavior, and where is it implemented? | Test/scenario IDs, captured artifacts, source links, ADRs, and code-level ownership. |

The documentation map uses these reader-facing categories:

1. **Product and releases** — overview, current release, changelog, roadmap, and supported surfaces.
2. **Tutorials** — guided lessons with a defined result, tested steps, and actual application screenshots.
3. **Feature guides** — focused behavior, constraints, and cross-links for each editor feature.
4. **Reference** — keybindings, menus, file formats, command palette, CLI, MCP, and settings.
5. **Troubleshooting and support** — known issues, recovery, bug reporting, and accessibility.
6. **Development and architecture** — build/test instructions, module ownership, ADRs, CI, and contribution rules.
7. **Planning and evidence** — public roadmap, active tasks, dated audits, workflow catalog, and release receipts.

A topic has one canonical page. Tutorials explain a sequence; feature guides define behavior and
limits; reference pages give complete lookup tables; architecture pages explain ownership. Link
between these roles instead of copying the same instructions into multiple documents. Use explicit
status labels such as **verified**, **partially verified**, **unverified**, and **planned** when the
reader could otherwise mistake a proposal for working behavior.

## Baseline audit — 2026-11-27

- `docs/README.md` has a long “I want to…” table that mixes tutorials, research, audits, release
  checks, and architecture. It makes the reader scan many unrelated rows.
- The root README also has a “Start here” task table and its release/demo links still refer to older
  versions. Update those links only after the current release and deployment are confirmed.
- `docs/tutorials/README.md` lists four tutorials and keybindings, but the tutorials do not yet form
  a full feature-by-feature learning path with verified screenshots and example documents.
- `scripts/gui-suite.sh` captures six static states (`home`, `splash`, `menu`, `flyout`,
  `find-replace`, and `about`) using the app screenshot path under Xvfb. These are visual state
  captures, not interactive end-to-end workflow tests.
- `scripts/gui-shot.sh` provides deterministic, exact-size screenshots. The Windows 11 checklists
  provide manual steps, but desktop acceptance is still reviewer-owned.
- The active gaps include native/browser clipboard acceptance, pop-out editing, complete path and
  raster behavior, dialogs at multiple sizes, layer reorder, filters and gradients, autosave
  recovery, and Windows 11 interaction. See [`TASKS.md`](TASKS.md) for the current prioritized list.

This baseline prevents the project from claiming comprehensive visual or documentation coverage
before the workflow system exists.

## Evidence-driven workflow

For every user-visible feature change or tutorial, follow this loop. Keep the workflow ID stable
across source tests, browser/desktop scripts, screenshots, docs, and bug reports.

### 1. Define the user scenario

- Choose one concrete user outcome and its start state, inputs, expected result, recovery/undo path,
  and relevant keyboard-only path.
- Record its platform/surface: native desktop, browser/WASM, or both; include viewport and display
  scale where layout changes matter.
- Link related requirements and existing open issues. Do not copy an issue description as the
  expected behavior without confirming it against product intent or a reference audit.
- Assign a stable ID in the workflow catalog, for example `FC-WF-CANVAS-PASTE-01`.

### 2. Run the exact application build

- Record the release/version, source commit, binary or web artifact checksum, platform, browser/GPU,
  viewport, scale, theme, and test command.
- Use the same real app binary that the visual reviewer would receive. For web flows, use the
  deployed or locally served WASM build being reviewed.
- Seed the flow with a checked-in, licensed sample document or a document created through the app.
  Do not invent mock UI, fake results, or substitute generated screenshots for application output.
- Keep secrets and personal content out of screenshots and sample files.

### 3. Exercise the workflow and save evidence

- Prefer interaction-level automation that performs the same pointer, keyboard, clipboard, file,
  and dialog actions as a user. A renderer screenshot alone proves appearance only.
- Save the test trace, console/runtime errors, assertion output, and screenshots as one run artifact.
- Capture before/after or state-transition images only where they explain behavior. Include failure
  evidence when a bug is found.
- For native-only behavior or unavailable automation, use a versioned manual checklist. Record the
  device/environment and reviewer; do not convert a manual checklist into an automated pass.

Suggested evidence manifest fields:

- workflow ID and scenario revision;
- FireCraft version, Git commit, artifact checksum, and build timestamp;
- OS, browser/runtime, GPU/render backend, window size, display scale, and theme;
- test command or manual checklist revision and reviewer;
- test result, assertion summary, trace/screenshot paths, and checksums;
- linked documentation pages and issue/PR IDs.

### 4. Diagnose and fix failures before authoring a “working” tutorial

- If the scenario fails, save the red reproduction first and file or update one focused issue with
  exact steps, expected/actual behavior, artifact identity, and evidence.
- Add a test at the narrowest reliable boundary, confirm it fails on the defect, implement the fix,
  then confirm it passes along with the relevant broader suite.
- Repeat the same workflow on the candidate build. Keep the before/after evidence and note any
  remaining platform limitation.
- A screenshot of a failed run is diagnostic evidence, not proof that the flow works.

### 5. Write documentation from the passing run

- The tutorial states the outcome, prerequisites, numbered steps, expected result after each
  important transition, undo/recovery option, and related reference links.
- Screenshots come from the passing app run. Add concise alt text that describes the relevant UI
  state, not every decorative detail. Crop only to focus attention; do not alter app content or hide
  a defect.
- A feature guide links to the scenario ID, supported/unsupported cases, test coverage, and known
  issues. The reference page contains the complete lookup table rather than a partial tutorial list.
- Label limitations and platform differences beside the affected instructions.
- Put durable screenshots under a topic-specific `docs/assets/` path with a manifest and source
  license. Large traces remain downloadable CI artifacts; do not commit build outputs.

### 6. Review and maintain the evidence

- A reviewer opens the captured images and trace, checks that the app build marker matches the
  manifest, verifies the steps are reproducible, and checks keyboard focus, contrast, clipping,
  scaling, and error recovery where relevant.
- Run `scripts/check-docs.sh` and `git diff --check`. Verify every link, image, version, keybinding,
  menu label, and feature claim against current code or a current application run.
- Changed user-visible behavior cannot merge with stale usage docs. Changed keybindings update the
  complete reference table and its tests together.
- At release time, regenerate the affected workflow artifacts and reconcile README download/demo
  links, changelog, tutorials, feature inventory, audit, and release notes against the tagged build.

## Workflow coverage rollout

Prioritize the workflows users have already reported. The list below is a test plan, not a claim of
current acceptance.

| Priority | Workflow | Required visual/behavior evidence | Current state |
|---|---|---|---|
| P0 | Launch and workspace handoff | Splash duration/transparency/fade; startup focus; standard and compact sizes | Needs Windows and browser visual acceptance |
| P0 | External clipboard paste | Image/text paste with canvas pointer over it and a dock field focused; context menu; browser permission path | egui regressions pass; native/browser acceptance open |
| P0 | Floating canvas editing | Edit, select, move, shortcut dispatch, document synchronization, drag/drop | Code-level fixes merged; real window acceptance open |
| P0 | Tools palette and flyouts | Expanded/collapsed widths, every group, flyout labels/icons, selection and hover details | Layout fixes merged; visual sweep still open |
| P0 | Path creation and point editing | Pen/freeform create, close, select/move points, undo/redo, save/reopen, export | Known behavior needs complete end-to-end verification |
| P0 | File open/import/export dialogs | Open, filter types, browse, resize, confirm/cancel, small/high-DPI viewport | Current app-level review remains open |
| P1 | Layering and image movement | Move, reorder, undo/redo, pixel identity, save/reopen | Regression reports require repeatable visual evidence |
| P1 | Eraser, gradients, filters, and styles | Raster pixel changes, feather/size, direction/stops/type, visible effect, edit/reopen/export | Feature correctness review remains open |
| P1 | Save, autosave, and recovery | Dirty state, periodic save, crash/restart recovery, conflict handling | Autosave/recovery sign-off remains open |
| P1 | Update and changelog | Detect update, inspect release notes, download, verify, recover from failure | In-app flow requires end-to-end audit |
| P2 | Keyboard and accessibility | Fullscreen/Tab focus, shortcuts, menu navigation, focus order, contrast, zoom | Needs desktop/browser pass and WCAG 2.1 AA/AAA target review |
| P2 | Export and sample gallery | Create a real sample design, save layered source, export assets, reopen/export comparison | Sample project and tutorial evidence still need production |

## Delivery phases and exit criteria

1. **Map and fact-check:** replace task-phrase navigation with categories; inventory pages and stale claims;
   trace each public claim to code, tests, a current build, or an explicit planned label.
2. **Scenario catalog:** assign stable workflow IDs and map every shipped feature to its automated or
   manual scenario, focused doc, reference entry, screenshot, and acceptance status.
3. **Reusable visual harness:** extend existing GUI capture tooling for interactive workflows;
   support desktop and WASM where relevant; produce repeatable traces, screenshots, and manifests.
4. **Tutorial production:** run each priority workflow, fix failures with red/green tests, capture
   verified evidence, and write tutorials/feature references from those exact steps.
5. **Documentation completeness:** cover every shipped feature and keybinding; add troubleshooting,
   accessibility, and platform notes; ensure no obsolete release links remain.
6. **Release maintenance:** make docs checks and scenario evidence part of release readiness; rerun
   affected flows against the tagged binaries/deployment and keep screenshots versioned and traceable.

A phase is done only when its listed artifact exists, links resolve, and its exit evidence is
reviewed. “Written” does not mean “tested”; “screenshot captured” does not mean “workflow passed.”

## Documentation quality gate

A user-facing change is ready when:

- the behavior and its limitations are described in one canonical feature page;
- the workflow has an automated test or an explicitly assigned manual checklist;
- the passing run has traceable source/build metadata and relevant real screenshots;
- tutorials, keybindings, menus, release notes, and demo links affected by the change are updated;
- accessibility and responsive/window-size checks are recorded for the touched interface;
- docs links/checks and the relevant code/test gates pass; and
- any unverified platform behavior is labeled as such.
