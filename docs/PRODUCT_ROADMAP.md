# FireCraft product roadmap

**Product:** FireCraft — Open Source Raster and Vector Graphics Editor
**Audience:** users, contributors, maintainers, extension authors
**Status date:** 2026-11-28
**Status:** living plan; this document describes current foundations and proposed work, not a promise of delivery dates.
**Execution record:** [`TASKS.md`](TASKS.md) is the only active checkbox ledger.
**Architecture and quality:** [`architecture.md`](architecture.md), [`PLAN.md`](PLAN.md), and [`AUDIT.md`](AUDIT.md).

## How to read this roadmap

- **Available** means present in the current repository; maturity and platform coverage may vary.
- **In progress** means tracked work exists, but the feature does not yet meet its exit criteria.
- **Planned** means proposed work with no completion claim.
- A phase advances only when its acceptance evidence is recorded in `TASKS.md` and the relevant audit or release receipt is updated.
- Sequence is dependency-driven. Dates are intentionally omitted until a maintainer breaks work into estimates and confirms capacity.

## Product direction

FireCraft brings vector and raster editing, web graphics production, and design handoff into one local-first application. It should preserve the speed and compactness of the Fireworks workflow while building a reliable, extensible editor with a stable document model and automation surface.

### Product principles

1. **Editable source of truth:** preserve structured objects, layers, pages, states, styles, effects, and export intent where the format supports them.
2. **Local-first:** creating, opening, editing, saving, and exporting must work without an account or network connection.
3. **Reversible changes:** destructive transforms, imports, extension actions, and AI-assisted operations need undo, previews, backups, or an explicit irreversible boundary.
4. **Honest interoperability:** explain what stays editable, is transformed, flattened, or lost before export or overwrite.
5. **One command authority:** UI, CLI, scripting, MCP, and future extensions should call shared, validated operations rather than reimplement document semantics.
6. **Accessibility by design:** keyboard access, focus visibility, readable contrast, scaling, and reduced motion are product requirements; WCAG 2.1 AAA is an aspiration to assess criterion by criterion, not a current conformance claim.
7. **Evidence before claims:** a rendered control, build, or passing unit test alone does not prove a complete end-user workflow.
8. **Clean-room implementation:** reference projects inform design and feature priorities; their code, assets, names, and licenses are not imported without explicit compatibility review.

## Canonical repository and consolidation direction — 2026-11-26

This repository, `aliasfoxkde/FireCraft-CraftAnything-Experiment`, is the canonical FireCraft product.
The earlier `aliasfoxkde/FireCraft` repository is retained as a donor/reference source, not as the
destination architecture. Its richer engine, native Fireworks format handling, codecs, scripting,
automation, tests, feature specifications, issues and release work will be evaluated for selective
migration into this application.

This is a preserve-and-improve strategy, not a wholesale merge. Keep the current egui application,
document model, FileService, command/CLI/MCP paths, test harness, CI, agent instructions and existing
release history. Do not merge unrelated repository histories, create a second app/document model,
or maintain duplicate task and product roadmaps.

The operating plan is [PRIMARY-REPO-RESET-AND-CONSOLIDATION-PLAN.md](PRIMARY-REPO-RESET-AND-CONSOLIDATION-PLAN.md);
the donor capability/ticket mapping is [LEGACY-FIRECRAFT-MIGRATION-REGISTER.md](LEGACY-FIRECRAFT-MIGRATION-REGISTER.md);
the immediate agent sequence is [AGENT-HANDOFF-2026-11-26.md](AGENT-HANDOFF-2026-11-26.md).
Execution checkboxes remain exclusively in [TASKS.md](TASKS.md).

## Current product foundations

The repository currently contains a Rust/egui desktop editor with a CS6-inspired shell, raster/vector tools, layers and panel groups, pages/states, styles/effects, import/export paths, layered `.fw.png` persistence, a command palette, keybinding/help material, an Intelligence panel, CLI/MCP surfaces, and a WebAssembly build path. See the [feature inventory](features/README.md), [tutorials](tutorials/README.md), and the dated [audit](AUDIT.md) for verified details and limitations.

The latest published release is [v0.3.8](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/releases/tag/v0.3.8),
with Windows x86_64, universal macOS, Linux tar/DEB/AppImage/Flatpak, and WASM packages. Its exact
source is `9f1dbe1350a6baa51e7775abcfb3b04989c15d84`. GitHub Pages serves the editor at `/` and the
documentation portal at `/documentation/`. The matching [Cloudflare review preview](https://wasm-review-v0-3-8.firecraft-web.pages.dev/)
serves the same release marker. Release publication does not certify native Windows 11 interactive
acceptance, macOS signing/notarization, or production marketing-site deployment.
The older repository's feature and release history is still being audited; no unverified donor
capability is included in the current product claims.

## Phases and exit criteria

### Phase 0 — Review build and repository stabilization

**Goal:** deliver a reviewable build and make the current repository state trustworthy.

1. Fix verified UI regressions in collapse controls, focus, clipboard, dialogs, splash behavior, and pop-out canvases; attach an interaction or platform test to each fix.
2. Validate the requested CS6-inspired palette, dock headers, and panel layout against supplied screenshots. Record intentional differences in the parity table.
3. Reconcile local and remote branches, pull requests, issues, stale task claims, and release workflows without merging failed or unreviewed changes.
4. Produce a Windows review package from the recorded source SHA, with README, license, changelog, third-party notices, binary hash, and launch/smoke evidence. Mark actual Windows 11 interaction as unverified until exercised on Windows 11.
5. Run Aegis and local CI checks; document unavailable or degraded GitForge capacity instead of substituting a simulated green run.

**Exit:** a clean recorded source revision; reproducible package manifest/hash; passing applicable local gates; a list of external checks still blocked; no release/deployment claim without its separate gate.

### Phase 1 — Trustworthy editing and recovery

**Goal:** protect users' documents and make core work dependable.

1. Default autosave to every 15 minutes, with user-configurable interval, enabled/disabled state, visible save status, and explicit recovery prompt on startup.
2. Write recovery generations separately from the source document; preserve the last valid file on failed save and bound recovery storage.
3. Test recovery after crash-like interruption, invalid document, write failure, disk-full/error where feasible, and version migration.
4. Fix and regression-test internal and system clipboard flows for vectors, text, images, and cross-application interchange; make unsupported clipboard types explain themselves.
5. Improve file dialogs: clear type filters, extension/format choice, overwrite confirmation, path validation, and keyboard navigation.
6. Make application update discovery, changelog viewing, download/open-release actions, and version status simple and understandable. Do not silently install an update; support platform-appropriate verification and rollback.
7. Add in-app feedback routes for bug reports, feature ideas, and change requests using the repository's issue templates. Include app version and optional diagnostics only after user review; never attach a document by default.

**Exit:** save/recovery and clipboard acceptance suite passes; every data-loss path has a regression test; update and feedback actions are clearly surfaced and privacy-preserving.

### Phase 2 — Discoverability, keyboard, and core design workflow

**Goal:** make common actions easy to discover and the editor comfortable for daily use.

1. Publish in-app Help screens for keybindings, tools, supported formats, panels, accessibility, tutorials, troubleshooting, and release notes.
2. Expand command search across commands, tools, panels, pages, layers, states, styles, recent documents, and help topics. Add aliases, fuzzy matching, keyboard navigation, category filters, and usage ranking before considering semantic/AI search.
3. Benchmark search latency and result quality on a realistically large command/resource catalogue; semantic retrieval must remain optional, explainable, and privacy-preserving.
4. Improve consistent typography, icon sizing, control spacing, focus/hover/disabled states, high-DPI scaling, and reduced-motion behavior. Keep a design token inventory and visual regression captures.
5. Validate menu items against the Fireworks CS6 parity reference; label unsupported actions and remove inert affordances rather than leaving controls that appear functional.
6. Improve floating/pop-out canvases with clear ownership, drag/drop file import, keyboard focus transfer, window lifecycle behavior, and persistence; validate native desktop and WASM separately.
7. Complete tutorials in the repository Wiki only after the local tutorial pages are reviewed and published. Link tutorials from Help and README.

**Exit:** documented keybinding matrix; command-search scenario suite; no known inert menu item without an explicit status; accessibility audit with criterion-level results and an issue for every unmet target criterion.

### Phase 3 — Extension API and one-click Extension Library

**Goal:** create a useful community ecosystem while keeping extension installation understandable, reversible, and bounded.

#### 3A. Extension contract (prerequisite)

1. Define a versioned extension manifest: stable ID, name, author, version, description, license, supported FireCraft API versions, entry points, declared permissions, checksums/signing metadata, and optional screenshots/docs.
2. Define supported extension types: command/action, menu or palette contribution, panel/widget, import/export adapter, brush/resource pack, and automation provider. Do not promise arbitrary native code support in the first release.
3. Expose a documented, typed extension API over the shared command registry. Separate read-only document access from mutation requests; mutations must validate, be undoable where applicable, and report structured failures.
4. Version the API independently from the app. Provide compatibility checks, deprecation windows, migration guidance, and a test harness for authors.
5. Threat-model extension inputs, file/network/process access, document access, resource consumption, update channels, and malicious packages. Choose a restricted runtime (for example, WASM capabilities) or explicit trust boundary before marketplace implementation.
6. Record package licenses and dependencies; prevent extension metadata from implying official endorsement.

#### 3B. Local extension management

1. Build a local manager that can install from a package file or trusted URL, inspect metadata and permissions, enable/disable, update, uninstall, and roll back.
2. Install into a versioned per-user directory using atomic package verification. A failed or incompatible install must leave the prior version available.
3. Make permissions visible before first enable. Ask separately for high-impact capabilities such as network access, arbitrary file access, external process launch, or access to secrets.
4. Provide a safe mode that launches without third-party extensions, plus a readable extension error log and compatibility report.
5. Support offline package installation and manual update checking.

#### 3C. Public catalog and one-click install

1. Publish a searchable catalog with categories, tags, app/API compatibility, screenshots, changelog, license, maintainer identity, source repository, and security/permission summary.
2. Add one-click install only after package provenance and permissions are shown. One click may download and stage; enabling code with elevated permissions still requires clear user intent.
3. Add signed release metadata or another verifiable trust model, immutable versioned artifacts, malware/policy review, report/takedown path, and maintainer ownership rules.
4. Provide a contributor submission flow, automated package validation, API compatibility tests, license checks, accessibility and privacy checks, and human review for catalog publication.
5. Never auto-enable newly downloaded code. Updates may be notified or downloaded, but permission changes require renewed consent; allow pinning and rollback.
6. Measure adoption and failures only through opt-in, privacy-respecting mechanisms; catalog operation must not require account telemetry for local use.
7. Keep **FireCraft official** extensions (maintained, supported, and eligible for bundling after release review) distinct from **community** extensions (author-maintained, independently versioned, reviewed for catalog policy and compatibility, but not bundled by default). Promotion to official status requires explicit maintainer approval, provenance and license checks, API compatibility and security tests, support ownership, and a release note.

**Exit:** threat model and ADR accepted; API contract tests pass; safe mode and rollback tested; malicious/incompatible package tests pass; catalog validation is reproducible; one-click flow displays source, license, compatibility, permissions, and rollback before install.

### Phase 4 — Brushes, touch, and broader input

**Goal:** support expressive raster/vector creation on mouse, pen, and touch hardware.

1. Define brush resources and presets: tip/shape, size, spacing, hardness, opacity/flow, texture, blend mode, pressure response, tilt where supported, and per-device defaults.
2. Separate brush stroke sampling from rasterization so the model can be unit/property tested without UI input.
3. Implement pressure-aware tablet input, event coalescing, predictable smoothing/stabilization, undoable stroke commits, and CPU fallback.
4. Add brush library import/export with attribution and license metadata; do not bundle third-party brush packs without permission.
5. Design touch gestures with explicit tool modes, palm rejection where platform APIs allow it, accessible alternatives, and user-configurable mappings.
6. Evaluate mobile UI constraints through a responsive workspace prototype before claiming mobile support; native file access, memory ceilings, stylus input, keyboard, and accessibility need target-device testing.

**Exit:** deterministic stroke tests, pressure/tilt capability matrix, performance benchmarks, and real-device touch/stylus evidence for each supported platform.

### Phase 5 — Photography, RAW, and advanced color

**Goal:** add optional photo workflows without weakening the graphics editor's predictable color behavior.

1. Specify a non-destructive image adjustment graph with ordered operations, editable parameters, per-layer masks, preview/final quality, and stable serialization.
2. Establish color-management foundations: ICC profiles, working spaces, transfer functions, soft proofing, display transforms, and explicit import/export policy.
3. Prototype raw development as an isolated workflow or plugin using a maintained decoder library; record format support and legal/technical constraints.
4. Add tonal/color operations (exposure, white balance, tone curve, local contrast, denoise, sharpen) behind measurable image-quality and performance tests.
5. Add HDR/scene-linear support with OpenEXR and high-bit-depth formats only after working-space and UI semantics are documented. Never silently clamp HDR values.
6. Provide before/after preview, reset, history, masks, and export metadata; compare outputs against known reference fixtures.

**Exit:** color-management ADR, deterministic fixtures, round-trip tests for supported profiles, and measured image pipeline behavior. “Lightroom-like” is a direction, not a parity claim.

### Phase 6 — Timeline, animation, video, and CorridorKey integration

**Goal:** extend frame/state workflows toward motion graphics and optional greenscreen processing.

1. Stabilize the current animation/state/timeline model: frame duration, loop behavior, playback clock, onion skinning, selection, and undo.
2. Design a timeline abstraction for keyframes, tracks, easing, transforms, visibility, effects, and audio/video references. Keep frame animation and video clips distinct where their semantics differ.
3. Add import/export and rendering milestones (animated GIF/APNG/WebP/video) only with fixtures, timing/color rules, cancellation/progress, and memory budgets.
4. Define CorridorKey as an optional external integration, not an embedded mandatory dependency. First prototype a job adapter that accepts selected media plus user-selected matte hints and returns validated alpha/foreground outputs.
5. Respect CorridorKey's published license, model weight terms, hardware/software prerequisites, third-party model notices, and large downloads. Verify the exact source revision and terms before distributing or invoking weights.
6. Keep heavy inference outside the interactive UI process; expose progress, cancellation, resource checks, output locations, and failure diagnostics. Do not send media to a hosted service without explicit consent.
7. Start with still/frame-sequence workflows, then evaluate clip workflows, timeline integration, compositing, spill suppression, and color-space behavior.

**Exit:** ADR plus dependency/license review; no bundled weights absent distribution rights; isolated integration tests with real permitted inference only when suitable hardware/assets are available; clear “unverified” status otherwise.

### Phase 7 — Documentation, community, delivery, and platform expansion

**Goal:** make the project approachable and keep releases trustworthy.

1. Maintain a public roadmap, completed feature list, tutorials, keybinding guide, format compatibility matrix, contributor guide, and changelog as dedicated documents linked from README and in-app Help.
2. Add a dedicated FireCraft marketing-site page for extensions. Explain official and community status, compatibility, permissions, review process, support expectations, licensing, and a clear submit-extension path; link to the public catalog when it is operational.
3. Establish issue templates and in-app feedback links; review submissions and publish decisions in the repository issue tracker. The [support pipeline proposal](SUPPORT-FEEDBACK-PIPELINE-HANDOFF-2026-11-27.md) is a design reference only; it does not approve autonomous merge or release authority.
4. Reconcile experiment and main FireCraft repositories through an explicit migration plan: source-of-truth decision, history/branch mapping, duplicate issue and PR mapping, remote settings, release tags, and rollback path.
5. Repair GitHub Actions and local GitForge CI drift; use one source SHA and record build/test/artifact identities. Require semantic application smoke tests, not health checks alone.
6. Build target-native packages and validate install, launch, edit, save, reopen, export, update, and uninstall on supported operating systems. Cross-compilation alone is not platform certification.
7. Publish release artifacts with manifest, SHA-256, license/notices, SBOM, provenance/attestation where supported, and clear release notes.
8. Deploy the marketing site and updated WebAssembly app from the canonical repository using preview deployments and a verified production artifact; confirm domain, cache, rollback, and download checksums.
9. Maintain security, accessibility, dependency, fuzz, and documentation link checks without weakening gates to force green status.
10. Keep any ArtCraft WebOS host integration optional and separate from standalone WASM. The [host contract](WEBOS-HOST-INTEGRATION.md) is a draft requiring cross-repository review and conformance tests before implementation.

**Exit:** repository ownership and release source are explicit; required workflows pass on the exact tag SHA; platform receipts are recorded; site deployment can be traced to source and rolled back.

### Phase 8 — Version history, page output, and publishing

**Goal:** make a design's history and downstream delivery understandable without turning FireCraft into a source-control client or web-hosting service by default.

1. Build on Phase 1's autosave/recovery work to define durable local document snapshots: trigger rules, retention limits, storage location, corruption handling, and the distinction between recovery snapshots and user-named versions.
2. Add a version browser only after snapshot integrity is proven. Show timestamps, labels, source/current comparison, restore preview, and a reversible restore action; never overwrite the sole good copy.
3. Keep history local and offline by default. Evaluate optional Git/GitHub integration as a separate adapter with explicit repository selection, credential isolation, conflict behavior, and clear network actions; do not require an account for snapshots.
4. Define page-layout and PDF use cases before expanding export: multipage/artboard mapping, physical units, bleed/margins, font embedding or substitution, vector/raster preservation, transparency, color profiles, hyperlinks, and accessibility metadata.
5. Add deterministic PDF and print-output fixtures with visual/text inspection, size/performance bounds, and cancellation/error recovery. Clearly document unsupported PDF import/edit behavior.
6. Define web-publishing output as a generated, reviewable package first: asset paths, responsive variants, CSS/image export, local preview, manifest, and reproducible output. Keep design export distinct from an in-app website builder.
7. Evaluate optional deploy adapters for Cloudflare Pages, Vercel, and self-hosted static hosts (including IIS/Apache). Require a dry run, target preview, scoped credentials, explicit publish confirmation, immutable deployment identity, and rollback instructions. Never persist secrets in a document or export bundle.
8. Treat Dreamweaver-era workflows as interoperability research: export clean assets, CSS snippets, HTML handoff, and predictable filenames. Do not promise a bundled site IDE until a validated user workflow and ownership case justify it.

**Exit:** snapshot/restore tests prove non-destructive recovery; PDF/export acceptance corpus is published; web packages are reproducible; deployment adapters have least-privilege credentials, preview/rollback evidence, and per-host support limits.

### Phase 9 — Provider-based AI and creative assistance

**Goal:** add useful optional AI capabilities without making editing dependent on a provider or obscuring what was generated or transmitted.

1. Extend Phase 30's Intelligence design into a provider-neutral capability inventory: text assistance, image generation/edit/extend, speech-to-text, text-to-speech, and document-aware suggestions. Each capability gets a user story, data-flow diagram, failure behavior, cost model, and explicit decision before implementation.
2. Define BYOK/provider configuration as a separate secure settings concern: secret storage by platform, provider/model selection, endpoint allowlist, per-capability consent, budget/rate controls, connectivity status, and one-click credential deletion. Never log keys or silently select paid providers.
3. Require a preview/accept/reject loop for generated images, text, and edits. Store provenance and prompt/model metadata only when the user chooses; make generated assets undoable and distinguish them from authored content.
4. Design speech features for explicit microphone/playback activation, visible recording state, transcript correction, data-retention controls, accessibility, and offline/provider capability reporting. No background recording or hidden upload.
5. Keep skills, memory, and scheduled jobs opt-in, inspectable, bounded, cancelable, and auditable. Scheduled work must not access documents, networks, or credentials beyond grants made for that job.
6. Test provider adapters with deterministic local contract tests, but do not treat mocks as proof of live provider behavior. Report live-provider qualifications only from permitted real calls with consent, bounded cost, and no private document data unless explicitly authorized.
7. Compare Pixlr and Photopea's web-native interaction patterns, and mainstream vector workflows in Affinity Designer, Illustrator, and Inkscape. Record workflow observations and target-device limitations; do not infer internals, copy assets, or claim parity from screenshots.

**Exit:** each shipped capability has a privacy/data-flow review, user consent and cost controls, keyboard/accessibility coverage, cancellation/error tests, provider-specific support matrix, and an off switch that leaves core editing usable.

### Phase 10 — 3D and expanded workspace research

**Goal:** decide whether lightweight 3D viewing/editing belongs in FireCraft and define a strict boundary if it does.

1. Collect concrete user workflows that need 3D alongside raster/vector design: asset inspection, simple model placement, camera/light staging, material preview, or rendered image compositing. Rank them against core editor work before selecting scope.
2. Compare embedded viewer, 2.5D compositing, and separate optional 3D editor/provider approaches. Record file-format licensing, GPU requirements, scene/document ownership, undo, accessibility, and project-size implications.
3. Prototype only a bounded viewer/asset-import spike after a written decision; keep 3D scenes isolated from the 2D document schema until round-trip and performance needs are proven.
4. Evaluate whether an integrated terminal, SSH client, or AI/IDE workspace solves a demonstrated creator workflow. Treat process launch, remote hosts, filesystem access, credentials, extension scripts, and AI tools as one threat-model surface; prefer launching a user's configured external tool when that meets the need.

**Exit:** a documented continue/defer/reject decision with user evidence, a bounded spike report if accepted, license/security review, and explicit non-goals. No 3D or agent-workbench support is advertised before target-hardware interaction tests.

## Deferred exploration and boundaries

- **Mobile/touch-first editor:** research and prototype after the tablet input model; do not advertise support without real devices.
- **Full video editor:** out of scope until the timeline and media pipeline are stable and an owner accepts its storage/performance cost.
- **AI agents, skills, memories, scheduled jobs:** Phase 30 specifies the expanded Intelligence section before implementation. Keep it opt-in and transparent, with bounded capabilities, explicit permissions, auditable actions, reversible edits, and no fabricated results.
- **System terminal inside the app:** Phase 30 requires a threat model and ADR before implementation. A terminal can execute arbitrary commands and is a distinct security boundary; define process isolation, consent, environment exposure, cancellation, and output handling first.
- **Git/GitHub version control:** Phase 8 keeps local snapshots independent; any remote adapter remains opt-in and must isolate credentials and make network writes visible.
- **PDF/page layout and web publishing:** Phase 8 starts with measurable export/handoff workflows; a full page-layout or web-authoring suite requires separate product evidence.
- **AI speech and image tools:** Phase 9 treats providers as optional, consented adapters with cost, data-flow, and attribution controls; no provider behavior is represented as live-tested by a mock.
- **3D modeling/viewing and embedded SSH/AI IDE:** Phase 10 is research and decision work first. The editor remains a 2D raster/vector product unless a bounded 3D use case earns a separately tested scope.
- **Cloud collaboration and accounts:** not required for local editing; would need a separate privacy, identity, conflict-resolution, and operations plan.
- **Adobe/Fireworks legacy extensions:** treat as behavioral and API research only. Existing `.jsf`, Flash/SWF panels, undocumented DOM details, and extension code do not imply binary/API compatibility.

## Reference projects and research

The research notes record the source links, observations, licensing caveats, and transfer recommendations: [`research/reference-projects.md`](research/reference-projects.md). Fireworks extension categories, historical API constraints, and marketplace submission/promotion requirements are detailed in [`research/fireworks-extensions.md`](research/fireworks-extensions.md). CI/release comparison is in [`research/ci-cd-patterns.md`](research/ci-cd-patterns.md). The separate root FireCraft repository is a sibling project and must not be conflated with this experiment repository.

| Reference | Roadmap use |
|---|---|
| [Emberframe](https://github.com/gregstein/emberframe) | Fireworks-inspired Rust shell, tool/panel inventory, and clean-room product positioning |
| [BonBon / Libreworks](https://github.com/librebon/BonBon) | Format matrix, shortcut-set inheritance, native-app packaging, and workflow research; GPL-3.0 code remains separate |
| [Fireworks extension resources](https://github.com/bomberstudios/fireworks), [fwlib](https://github.com/fwextensions/fwlib), [fwconsole](https://github.com/fwextensions/fwconsole), [John Dunning's catalog](https://www.johndunning.com/fireworks/), and [Smashing Magazine extension article](https://www.smashingmagazine.com/2014/06/introduction-to-developing-fireworks-extensions-theyre-just-javascript/) | Extension categories, command discovery, author tooling, API introspection, and community marketplace requirements |
| [Krita](https://github.com/KDE/krita) | Brush engine, stylus/touch, resource packs, and painting workflow research |
| [Photopea](https://www.photopea.com/) and [Pixlr](https://pixlr.com/) | Browser-native onboarding, import/export, shortcut discoverability, selection/layer workflows, responsiveness, and offline/network behavior to compare through hands-on scenarios; no parity claims from marketing material |
| [Affinity Designer](https://www.affinity.studio/), [Adobe Illustrator](https://www.adobe.com/products/illustrator.html), and [Inkscape](https://inkscape.org/) | Vector document, node editing, artboard, typography, color, export, and accessibility workflows; use public product documentation and observable behavior, not copied code/assets |
| [darktable](https://github.com/darktable-org/darktable) | Non-destructive photo pipeline, color management, RAW workflow, and modular processing architecture |
| [CorridorKey2.0](https://github.com/nikopueringer/CorridorKey2.0) | Optional greenscreen/keying job integration after license, hardware, model-weight, and process-isolation review |
| `/nas/Temp/repos/FireCraft` (separate local repository) | Local sibling source of CI, release gates, task ledgers, parity tables, fuzzing, corpus, and accessibility process patterns; see research notes for its exact reviewed revision |

## User-visible wishlist traceability

The following requests are represented above and must remain traceable in [`TASKS.md`](TASKS.md): tagline and identity; keyboard/help screens; command palette search; public roadmap and done/future feature docs; in-app feedback; app updates/changelog; CS6 menu review; system terminal; Intelligence panel expansion; autosave/recovery; typography/icon/layout consistency; pop-out canvases and file dialogs; styles/layers; sample design page; logo; Windows packages; Cloudflare site and WebAssembly app; repository consolidation; issues/PR/branch hygiene; tutorials/Wiki; Phosphor iconography; extension library/marketplace; brush/touch/mobile; photo/RAW/HDR; timeline/video; CorridorKey; local/remote version history; PDF/page-layout output; static web publishing and hosting adapters; AI BYOK, image generation/editing, speech input/output; 3D viewing/modeling research; SSH/AI-workbench evaluation; splash, focus/Tab, and clipboard bugs.

This list is a scope index, not a completion checklist. `TASKS.md` records current status, blockers, and acceptance evidence.
