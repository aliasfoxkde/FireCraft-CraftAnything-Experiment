# Reference project study: Fireworks revival and adjacent tools

**Reviewed:** 2026-11-26
**Purpose:** record transferable product and engineering ideas for FireCraft's roadmap. This is a desk study of public repository/docs plus a local inspection of `/nas/Temp/repos/FireCraft` at commit `72318ae7`. It is not a code audit of every upstream project or a license/legal opinion.

## How to use this study

Treat references as evidence for workflows, interaction patterns, architecture questions, and ecosystem needs. Do not copy source, artwork, names, package contents, proprietary file samples, or extension bundles into FireCraft without a separate license and provenance review. FireCraft is clean-room software; the implementation must follow FireCraft's own architecture and compatibility rules.

## Findings and recommendations

| Reference | Verified public facts | Transferable insight for FireCraft | Constraints / status |
|---|---|---|---|
| [Sibling FireCraft repository](https://github.com/aliasfoxkde/FireCraft) and local checkout `/nas/Temp/repos/FireCraft` | At local commit `72318ae7`, the repo has `crates/scripting` with a Boa-based sandbox and tests; a shared engine command registry; fuzz targets; a large corpus; `.github/workflows/{ci,ci-cross,release,deploy-site}.yml`; least-privilege workflow permission declarations; and a detailed release plan, task board, ADR series, accessibility decision, release receipts, and CI checks. Its CS6 matrix explicitly records the missing extension installer. | Reuse its *process patterns* in this experiment: command registry as the extension authority, constrained scripting host, feature/task/ADR traces, parser fuzzing, release receipts, per-platform evidence, and current-state docs. Evaluate whether the stable root repository should become canonical before duplicating major capabilities. | It is a separate repository/architecture, not a dependency of this egui app. Inspect the intended migration/ownership boundary before porting code or promising parity. Local checkout contents can be ahead of public main; record tested SHA whenever comparing. |
| [Emberframe](https://github.com/gregstein/emberframe) | Public README describes an open-source Fireworks-like editor rebuilt in Rust with GPUI, a docked workspace, and an enumerated tool/panel surface; repository is small/early (four commits in the page snapshot). | Compare tool grouping, workspace hierarchy, and onboarding vocabulary; use its explicit tool/panel inventory as a cross-check when validating Fireworks workflows. Keep FireCraft's own visual system and command/document model. | Early project; feature list is not proof of parity, quality, file compatibility, or production readiness. Its public listing identifies MIT licensing; recheck the actual LICENSE before any reuse. |
| [BonBon / Libreworks](https://github.com/librebon/BonBon) | GitHub repository title is BonBon; README identifies the application as Libreworks, a native Linux editor in the Fireworks MX 2004 spirit, C++17/Qt 6, and advertises modern raster, vector, animation, HDR and interchange formats. The repository lists GPL-3.0. | Study broad format discovery, keyboard preference behavior, menu/task organization, and how legacy workflows can coexist with modern formats. Use its format inventory to identify research questions for FireCraft's explicit import/export compatibility matrix. | GPL-3.0 is incompatible with casually copying code into FireCraft's MIT-licensed codebase. Advertised format support needs independent behavior checks; do not treat a feature list as fidelity evidence. |
| [bomberstudios/fireworks](https://github.com/bomberstudios/fireworks) | Repository is described as Adobe Fireworks support files. | Useful historical material for workflow vocabulary and extension concepts, if each item is individually licensed and relevant. | Legacy support files are not a maintained application architecture. No bulk import or redistribution. |
| [fwextensions/fwlib](https://github.com/fwextensions/fwlib) | Utility library for Fireworks extension authors; source documents areas such as document/layer utilities, files, preferences, font inspection, and persistent data. Repository page lists MIT. | Build a FireCraft Extension SDK around typed document/selection/page/layer helpers, persistent settings, filesystem abstractions, error reporting, and versioned APIs. Make common jobs easier than direct model internals. | Fireworks DOM APIs and host assumptions do not transfer directly. Reimplement concepts against FireCraft's public command API; recheck per-file notices before any code reuse. |
| [fwextensions/fwconsole](https://github.com/fwextensions/fwconsole) and [John Dunning's extension catalog](https://www.johndunning.com/fireworks/) | fwconsole is a JavaScript console panel for Fireworks. Dunning's catalog documents productivity extensions including QuickFire (search across commands, symbols, auto-shapes, textures, panels, layers/pages/states and recent files), page batching, linked images, shortcuts, clipboard, asset export and developer tools. | Command discovery should search more than command names: tools, panels, document targets, recent files, saved snippets, tutorials, and installed extensions. Add developer diagnostics and examples to the SDK. The catalog shows that high-value extensions often solve narrow repeated tasks. | Legacy extension downloads may be unavailable, unsafe, or differently licensed. Catalog descriptions document ideas, not endorsement or compatibility. |
| [Smashing Magazine: Introduction to Developing Fireworks Extensions](https://www.smashingmagazine.com/2014/06/introduction-to-developing-fireworks-extensions-theyre-just-javascript/) | The article distinguishes small command scripts from interactive command panels, presents history/replay as an inspection aid, and describes a console/DOM-inspector developer loop. | FireCraft should make actions discoverable and scriptable from one command authority; provide a safe extension console in a development-only mode, inspectable command history, examples, and a clear choice between one-shot commands and interactive panels. Prefer undoable command batches and typed APIs over an unrestricted evaluation console. | Article describes Fireworks' legacy JavaScript/Flash model. Flash panels are obsolete and must not be reproduced. Treat the article as workflow history, not an implementation specification. |
| [Krita](https://github.com/KDE/krita) | Mature cross-platform digital-painting application with a deep brush, tablet, stabilizer, resource, and customization ecosystem. | Research brush engines/resources, pressure/tilt/rotation mapping, stabilizers, brush presets, tablet calibration, touch gestures, and resource import UX. Start with a smaller brush-engine contract that uses FireCraft's existing raster/document model and has deterministic tests. | Krita is a large painting application with its own architecture and resource ecosystem. Brush parity is a separate workstream; do not add a thin UI that suggests a mature paint engine. Test real tablets and platforms. |
| [darktable](https://github.com/darktable-org/darktable) | Mature non-destructive photo workflow/raw developer with lighttable/darkroom concepts, modular image operations, a pixelpipe architecture, and separate developer/user documentation. Its README explicitly avoids claiming to be a free Lightroom replacement. | For photography features, study non-destructive operation stacks, parameter snapshots, history, color-management boundaries, export recipes, scalable processing pipelines, and honest platform/hardware support. Prototype an optional photo workspace only after FireCraft has a color-managed, non-destructive raster pipeline. | A raw-photo workflow is not a small “brightness/contrast” panel. RAW camera support, ICC/OCIO-like color management, HDR, metadata, profiles, and performance each need scope and format tests. Avoid branding claims that imply Lightroom equivalence. |
| [CorridorKey2.0](https://github.com/nikopueringer/CorridorKey2.0) | Public repository provides green/blue-screen video matting and CLI workflows with FFmpeg, CUDA-enabled PyTorch, NVIDIA GPU requirements, multi-gigabyte model weights, and a license with additional terms. The README documents input hints, alpha/foreground outputs, EXR output, memory profiles, and integration contact. | Future integration should be an optional external processing provider: select a video/sequence, configure a matte job, run with progress/cancellation, import alpha/foreground outputs as image sequences, preserve source media, and make hardware/model/license requirements clear. Design a provider interface so other keyers can be added. | It is not a small in-process Rust library. Its stated CUDA/PyTorch requirements, large weights, FFmpeg dependency, and custom license require an isolated optional install/worker and explicit license/security review. Confirm rights and integration terms before distributing or bundling it. |

## Cross-project product lessons

1. **A useful Fireworks successor is workflow-first.** Common work spans objects, pages, states, layers, styles, slices, asset export, and command discovery. Improve complete user tasks rather than just accumulating tool icons.
2. **The extension system should deepen the shared command model.** Keep GUI, CLI, scripts, MCP, and plugins on the same typed operations, validation, undo, and error behavior. Avoid a plugin-only back door that mutates document internals.
3. **A marketplace is an ecosystem and security project.** Deliver in this order: API/manifest and capability model; local developer extensions; isolated execution and lifecycle; local install/disable/update with rollback; compatibility and signature verification; curated searchable catalog; contributor publishing/review/reporting. One-click install should show publisher, permissions, compatibility, version, and network/filesystem access before consent. Do not execute downloaded native code without explicit user approval. Never make network services mandatory for local editing.
4. **Scripting should be useful without being an unsafe REPL.** Start with signed/locally trusted packages, an allowlisted API, time/memory limits, cancellation, no ambient filesystem/network access, and document-scoped undo transactions. A developer console can be opt-in and clearly privileged.
5. **Feature claims need receipts.** Build/install/launch/interactive tests, real OS coverage, accessibility evidence, compatibility corpus results, and release provenance are stronger than feature matrices alone. The sibling repository's decision records, receipts, coverage measurements, and multi-platform release board provide a process model to adapt.
6. **Keep future-heavy media workflows modular.** Brush engine, raw/photo processing, video timeline, and keying have different data models and performance profiles. Define extension/provider seams only after a user workflow and resource contract is clear.

## Proposed staged roadmap additions

These are recommendations for the active FireCraft task board and product roadmap; they are not implemented by this research document.

### Phase A — Current core readiness

- Reconcile the experiment repository with the canonical sibling FireCraft repository and decide the ownership/migration boundary.
- Complete current shell fixes, reliable clipboard/keyboard flows, autosave/recovery, file dialogs, in-app feedback, updater/changelog, command search, and documented tutorials.
- Establish current platform test/coverage/accessibility baselines before setting numeric targets.

### Phase B — Local automation and extension SDK

- Publish a versioned manifest schema and a read-only compatibility declaration.
- Expose typed, documented commands and events; use existing command registry and scripting host where available.
- Add a minimal command-extension package format, development folder loading, enable/disable, logs, permissions, failure isolation, undo integration, and sample extensions.
- Add CLI validation/lint and test harness for extension packages; never require catalog connectivity.

### Phase C — Safe one-click extension catalog

- Implement local package management first (install, inspect, enable, disable, update, rollback, uninstall, export/import).
- Define signing, publisher identity, provenance, compatibility ranges, vulnerability reporting, moderation, revocation, and reproducible package metadata.
- Add the searchable in-app catalog only after the lifecycle and trust UX work; permission prompts must name concrete file/network/process capabilities.
- Gate automatic updates behind explicit preferences; retain installed versions and allow rollback.

### Phase D — Creative-resource ecosystem

- Resource import/export and provenance for brushes, palettes, gradients, patterns, templates, symbols, and UI themes.
- Tablet/touch input API and a small brush-engine milestone with pressure/tilt tests on real devices.
- Keep optional resources independently licensed and packageable; no unsupported “Krita brush compatibility” claim.

### Phase E — Photo workflow research

- User interviews and sample corpus for raw/photo editing; define color-management requirements and export boundaries.
- Prototype non-destructive adjustment stack and color-managed preview behind a feature branch/ADR.
- Decide if RAW decode and HDR are native modules, external providers, or out of scope; benchmark representative files before commitment.

### Phase F — Timeline and keying integration

- Define frame/sequence/timeline document requirements, proxy/cache behavior, project storage, and render/export contracts.
- Build an external media-provider interface with capability and hardware checks, progress, cancellation, temp storage, and failure recovery.
- Evaluate CorridorKey2.0 licensing, weight distribution, hardware requirements, and service terms before shipping integration. Start with user-selected local installation and interchange of image sequences/mattes; do not embed model weights in the base installer.

## Research questions to close before implementation

- Is `/nas/Temp/repos/FireCraft` the canonical product repository, with this experiment merged into it, or does this repository have a separate maintained role?
- What is the stable document/command API that extensions are allowed to invoke, and how are API compatibility/version migrations handled?
- Which extension runtimes meet desktop and WASM needs (WASM sandbox, constrained JS, or both) without ambient host access?
- Who reviews extension submissions and handles vulnerable, abandoned, or malicious packages?
- What are the target tablet devices, raw-photo sample corpus, and actual video/keying use cases?
- Which future scopes belong in core, which are optional providers, and which remain documented as out of scope?

## Source notes

- GitHub README and repository metadata for Emberframe, BonBon, fwlib, fwconsole, Krita, darktable, and CorridorKey2.0 were reviewed on 2026-11-26.
- John Dunning's extension descriptions and Smashing Magazine's 2014 developer article were reviewed on 2026-11-26.
- The sibling FireCraft checkout was inspected locally at commit `72318ae7`; details are a local-worktree observation and may differ from public `main`.
- Product-stage recommendations and security requirements above are FireCraft-specific analysis inferred from those sources, not claims made by the referenced projects.
