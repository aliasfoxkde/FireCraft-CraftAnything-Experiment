# FireCraft Quality, Validation, WebOS Interoperability & Delivery Roadmap

**Prepared:** 2026-11-27
**Repository:** `aliasfoxkde/FireCraft-CraftAnything-Experiment`
**Inspected main SHA:** `284e00a53001da9bcb93d6de9e2c7f46abcdf2b3`
**Latest inspected PR validation SHA:** `36e9cc0918e06d03b131726c3f50e9fc303d02e6`
**Latest published release observed:** `v0.3.2` (2026-11-27)
**Separate host repository:** `aliasfoxkde/ArtCraft-WebOS`
**Inspected WebOS main SHA:** `d08d2d11f6c546cf94f786f8f409b846a0549c60`

> **Historical baseline:** This planning audit records the source state at FireCraft `284e00a` /
> release `v0.3.2`. Its tests, coverage, and release figures are not current. For the current measured
> state, see [the 2026-11-27 audit](AUDIT-2026-11-27.md), [progress log](PROGRESS.md), and
> [task ledger](TASKS.md). Architecture recommendations below remain proposals until they are adopted
> into the active roadmap and implementation tasks.

This document is the FireCraft-side plan of record for quality improvements, distribution, and the integration contract with ArtCraft WebOS. It deliberately does **not** move WebOS source code, workflows, app catalog, or host runtime into this repository. WebOS owns its own host/runtime roadmap in its repository.

This is a source/configuration review plus inspection of recent GitHub Actions evidence. It is not a local execution of either repository's tests. The status figures below are tied to the exact run and source revisions and must be refreshed as new changes land.

---

## 1. Executive assessment

FireCraft has a credible Rust quality foundation: pinned toolchain, strict Clippy, Rust safety linting, headless egui interaction harness, CLI/MCP integration tests, format golden tests, dependency checks, a coverage ratchet, cross-target builds, release manifests, and checksum verification.

The current gap is not an absence of tests. It is the distance between a useful Rust test suite and proof that a creator can safely use the whole product across desktop, standalone browser WASM, and a separately maintained WebOS host.

The next quality hill climb must improve **test relevance, reproducibility, release equivalence, and integration evidence**, not simply increase a percentage.

### 1.1 Current FireCraft evidence

| Area | Latest evidence inspected | Interpretation |
|---|---|---|
| Rust tests | CI on PR SHA `36e9cc0`: 174 unit tests passed, 1 ignored; 12 CLI; 4 hygiene; 10 MCP. **200 passed, 1 ignored**. | A good automated baseline for core and process-level behavior. Not proof of all UI and platform behavior. |
| Strict lint | Linux gate completed successfully on the same PR SHA. | `cargo fmt` and Clippy with `-D warnings` are useful mandatory gates. |
| WASM | Separate optimized WASM build succeeded on the PR SHA. | Compile/package success does not establish real browser clipboard, canvas, host bridge, or offline behavior. |
| Windows | Windows Release build, CLI smoke and archive succeeded on the PR SHA. | This is runner build/smoke evidence, not native Windows 11 GUI, install/update, or downgrade acceptance. |
| CodeQL | PR workflow succeeded on the same PR SHA. | Keep the scan exact-SHA and required. |
| Coverage | Latest Rust coverage report: **70.79% statements, 71.18% functions, 68.40% lines**. Branch counters were not emitted (`0/0`). | The existing 67.61% line floor is a real regression guard, but branch behavior is not measured yet. |
| Dependency security | `cargo audit` completed with **13 allowed warnings**, including unmaintained and unsound upstream dependency advisories. | Green means policy allowed these known warnings; it does not mean the dependency graph is clean. |
| Release | Public `v0.3.2` exists with Windows, macOS, Linux, WASM ZIP, package/release manifests, source SHA and checksums. | Keep and extend the existing artifact identity work. |
| Browser acceptance | The dated task ledger records a real Chromium image-paste success path; denial, malformed/large payloads, focus and native clipboard cases remain open. | Add repeatable browser E2E. Do not treat a WASM compile as browser validation. |

The 200 passing tests and 68.40% coverage came from PR SHA `36e9cc0`, not a fresh local run performed for this roadmap. The public release metadata confirms `v0.3.2` is published; it does not certify every platform's interactive acceptance.

### 1.2 Current WebOS evidence (separate repository)

WebOS main SHA `d08d2d1` had a successful GitHub CI run on 2026-11-23:
- 95 Vitest files passed.
- 1,475 Vitest tests passed.
- Browser sweep reported 31/31 phases green with zero attributed page/console errors.
- ESLint uses zero-warning mode; four-metric V8 coverage thresholds are configured; the pipeline also runs contrast/docs gates, Aegis, ShellCheck, build, and CodeQL.

The WebOS coverage floors in `webos/vitest.config.js` are statements 98.6%, branches 94.85%, functions 98.8%, and lines 99.85%. Its dated progress snapshot recorded 98.88/94.86/98.95/99.91 on 2026-11-02. The 2026-11-23 CI run passed the current floors; do not label the old measured percentages as a fresh 2026-11-27 measurement.

WebOS's test logs also showed React DOM-nesting warnings for nested buttons and Testing Library `act(...)` environment warnings. These did not fail CI. The Files UI contains clickable outer button tiles with nested delete/download/import buttons, which is invalid interactive HTML and a real accessibility/interaction issue to fix.

**Repository ownership boundary:** FireCraft owns editor behavior, portable WASM artifacts, document format compatibility, and a versioned host-client contract. WebOS owns app catalog/install/update/cache/rollback, sandboxing, permissions, storage mediation, desktop shell, and its quality/release workflows.

---

## 2. Audit findings and disposition

### 2.1 FireCraft: test and validation gaps

| Priority | Finding | Evidence / why it matters | Planned correction |
|---|---|---|---|
| P1 | Release checks are not yet equivalent to integration checks | CI tests PRs and pushes to `main`; there is no `staging` integration branch or `merge_group` trigger. Coverage workflow checks the PR head SHA while the primary CI workflow tests GitHub's PR merge candidate. | Align all required workflows to the same merge candidate SHA, add `merge_group`, and protect staging/main with required checks and merge queue. |
| P1 | Local CI can silently omit security and coverage tools | `scripts/ci-local.sh` prints “skip” when `cargo deny`, `cargo audit`, or `cargo llvm-cov` is absent, then can still print “all gates passed”. Hosted CI installs them, but local/release agents can misread that message. | Add explicit `--quick` vs `--release` modes. Release mode fails closed; quick mode prints a machine-readable list of gates not run. |
| P1 | Browser validation is weaker than the WASM build claim | CI compiles/builds WASM but does not run the packaged artifact through a headless browser journey on each PR. | Add Playwright/Chromium E2E over the built bundle, with canvas paint, clipboard, save/open/export and JS-console assertions. |
| P1 | Format acceptance tests do not establish the full format matrix | `formats.rs` is ~1,590 lines; the inspected module-local suite has about ten focused tests (SVG/HTML/PNG goldens, ICO downscale, unsupported AVIF save, I/O/corrupt PNG, escaping, SVG round-trip, layered PNG, rasterize sample). Several important encoders, edge cases and adversarial inputs are not proven by that set. | Create a format capability matrix and test each claimed import/export path, error path, alpha behavior, round-trip expectations, dimensions, metadata and malformed data. |
| P1 | WASM build emits a target-feature metadata warning | The latest WASM job warns that target_features metadata is missing, likely due to `strip = true`, and suggests avoiding strip before wasm-bindgen / running wasm-opt afterwards. | Reproduce, compare output with and without strip, pin wasm-bindgen, and optimize the post-bindgen artifact. Reject unexplained feature-metadata loss. |
| P2 | Coverage is line/function oriented and branch coverage is absent | Current report: 68.40% lines, 71.18% functions, branch counters 0/0. | Add branch coverage or document tool limitations and use explicit branch/behavior tests for critical state machines. Raise floors only from reproducible CI measurements. |
| P2 | Dependency warnings are allowed without a fully visible expiry workflow | Latest audit reports 13 allowed upstream warnings, including unsound/unmaintained packages. | Record each advisory, dependency path, exploitability decision, owner, expiration date, and upstream/upgrade trigger. Block new critical/high advisories. |
| P2 | AI provider secrets are passed as process arguments | `start_ai_request()` builds an Authorization header containing the API key and passes it to `curl` via argv. Process command lines can be observable locally. | Replace with an in-process HTTP client and add key-redaction / error-body tests. |
| P2 | Unknown file extensions silently select PNG | `Format::from_path()` defaults unknown extensions to `Format::Png`; saving can create PNG bytes with a misleading suffix. | Return an explicit Unknown/Unsupported format and test empty, unknown and misleading extensions. |
| P2 | The hardcoded-version hygiene scan only recognizes the current 0.2/0.3 literal families | `app/tests/hygiene.rs` checks for quoted strings beginning with `0.2.` or `0.3.`; it can miss a future 1.x/other version literal or non-matching encoding. | Use a robust version-source policy: derive app version from Cargo metadata and validate manifests/artifact naming against it; use a focused SemVer-literal scanner with fixtures for multiple major versions. |
| P2 | Current updater is discovery-only and synchronous | It calls `/releases/latest` through `curl` synchronously on the UI thread, with a six-second timeout; it cannot install, select channels, verify artifacts, downgrade or roll back. | Implement channel-aware staged updates, async checks, artifact verification, health checks and rollback. |
| P2 | macOS release-page opener uses `xdg-open` | The non-Windows desktop path uses `xdg-open`, which is not the standard macOS opener. | Use platform-specific openers and test on native OS runners. |
| P2 | Native UI/platform acceptance is not the same as successful cross-builds | Windows CI does CLI smoke, while macOS/Linux packages are built in release jobs. Real user workflows and updater behavior need platform receipts. | Define per-OS install/open/edit/save/reopen/export/update/downgrade/uninstall scenarios. |

The current `cargo fmt`, Clippy and Rust lints are not the problem. Preserve strictness while making the tests and delivery evidence more representative of real use.

### 2.2 WebOS: tests, validation and release-process gaps

| Priority | Finding | Evidence / why it matters | WebOS-side correction |
|---|---|---|---|
| P1 Security | FireCraft iframe messages are source-checked but not origin-checked | `EmbedFrame.jsx` special-cases `event.source === frame.contentWindow` and `fd.source === 'firecraft'`, but does not validate `event.origin` before accepting `ready`. The restore path reads a user's autosaved document and `postToApp()` sends it to the iframe with target origin `*`. A navigated cross-origin page can potentially spoof `ready` and receive those bytes. | Bind messages to the expected origin and exact frame; use a capability-based MessageChannel for sandboxed opaque-origin packages; revoke channels on navigation/reload. Add hostile-navigation, wrong-origin, forged-ready, stale-port and data-non-disclosure tests. |
| P1 | FireCraft shipped version is stale in WebOS | WebOS registry, `fetch-apps.sh`, and PWA injection list FireCraft `0.2.3`; FireCraft Release `v0.3.2` is now published. The existing registry test only asserts that those three internal copies agree. | Replace duplicated static version values with a catalog descriptor resolved from the upstream release manifest. Add a freshness/allowlist check and an explicit adoption test for v0.3.2. |
| P1 | Tag release bypasses parts of the normal WebOS CI gate | WebOS CI runs Aegis, ShellCheck, docs/contrast gates, coverage, build and the 31-phase browser sweep. `.github/workflows/release.yml` reruns lint, coverage and build, then publishes/deploys; it does not rerun Aegis, ShellCheck, contrast/docs gates or browser sweep. | Make release depend on the same reusable, exact-SHA quality workflow. Split build, GitHub release, and Pages deployment into least-privilege jobs. |
| P1 | External FireCraft E2E checks can self-skip | `webos-sweep.mjs` drives the deployed `firecraft-web.pages.dev` and skips some FireCraft-specific checks if the external host is unreachable. A nominal 31/31 pass can therefore omit the integration path. | Replace required FireCraft integration phases with a local deterministic package fixture; keep the live external-site probe as a separate optional/informational check. |
| P2 | Invalid nested interactive controls in Files | Folder/file/device tiles are outer buttons containing inner action buttons for delete/download/import. CI logs React `validateDOMNesting` warnings. | Use non-interactive tile containers with keyboard semantics or sibling action controls; assert valid DOM and keyboard behavior. |
| P2 | Test warnings are not treated as regressions | CI passed with DOM nesting and `act(...)` warnings plus aborted external fetch/background-task diagnostics. | Fail on unexpected React warnings/console errors in tests; allowlist exact expected aborts with reasons. Ensure timers/listeners/tasks are cleaned up after unmount. |
| P2 | `npm run ci` is not the complete GitHub gate | The package script runs lint, coverage and build; CI also runs Aegis, ShellCheck, standalone docs/contrast gates and browser E2E. | Provide explicit `ci:quick` and `ci:full` scripts, document prerequisites, and make the full script match hosted CI. |
| P2 | Accessibility evidence is mainly static structure and contrast | JSX a11y and token contrast tests are valuable, but they do not prove keyboard-only, screen-reader, focus restoration, touch, or reduced-motion behavior across real browsers. | Preserve automated gates and add targeted browser interaction tests plus a documented manual accessibility matrix. |

This is a read-only finding. The origin-navigation scenario has not been dynamically exploited; the combination of missing event-origin validation and wildcard host-to-frame messaging is sufficient to make it a high-priority defect requiring a regression test before cached local packages are enabled.

### 2.3 Cross-repository integration facts

1. WebOS is a separate application/runtime repository. Do not add WebOS source files, CI workflows, app registry, desktop shell, or package manager to FireCraft.
2. FireCraft currently produces a complete browser/WASM ZIP release asset, but the inspected current FireCraft main source does not contain the historic WebOS `ready/applied/openDocument/exportDocument/exportPng` protocol implementation. A current versioned bridge must be reintroduced and tested in Rust/WASM.
3. WebOS currently consumes FireCraft as a hosted iframe/app fetched from a release ZIP and then PWA-injected. The current hardcoded WebOS FireCraft version is 0.2.3; latest FireCraft release is 0.3.2.
4. WebOS owns cache/install/rollback/permissions. FireCraft owns a portable browser bundle and a versioned host-client implementation. The contract belongs in docs on both sides, with independent branch/PRs.
5. Keep a standalone browser entrypoint. WebOS hosting must be additive and must not make normal FireCraft browser use dependent on WebOS.

---

## 3. Quality contract: what must be proven

A green build is necessary, not sufficient. Organize the verification system into layers that each have a clear owner and failure meaning.

### 3.1 Required verification layers

| Layer | Test method | FireCraft examples | When required |
|---|---|---|---|
| Pure unit | Rust unit tests | geometry, color, math, command dispatch, format detection, manifest parsing, feature flags | Every PR |
| Property / generative | Proptest and fuzz targets | transforms, path operations, document round-trips, parser/codec robustness, archive paths | Every relevant PR; scheduled fuzz runs |
| Serialization compatibility | Versioned fixtures + golden corpus | .fw.png payloads, SVG, PNG, layered documents, legacy versions, migrations | Every format/schema PR |
| Service integration | Fake/injected services and local test services | file IO, updater manifest/download, support API client, WebOS bridge messages | Every service change |
| Headless UI | Existing egui harness plus behavior assertions | menus, panels, dialogs, focus, selection, undo/redo | Every UI PR |
| Browser E2E | Playwright/Chromium against the built artifact | boot, canvas paint, menu/tool path, clipboard, save/reopen/export, browser host handshake | Every WASM/host/UI PR |
| Platform smoke | Native OS runners and signed/packaged output | Windows/macOS/Linux install, launch, save, update, downgrade | Release candidate; relevant platform PRs |
| Performance | Benchmarks and trace capture | canvas frame budget, import/export, WASM startup, memory, large docs | Performance-sensitive PRs and releases |
| Accessibility | Automated + manual checks | semantic controls, focus order, keyboard, contrast, screen reader, reduced motion | Every UI surface; release audit |
| Security / supply chain | cargo audit/deny, CodeQL, Aegis/pattern scan, secret scan, SBOM/provenance | dependency advisories, unsafe boundary, message origin, archive extraction | Every PR and release |
| Operational integration | failure injection and end-to-end rehearsal | support intake → agent → PR → staging → nightly → cohort → rollback | Before autonomous production rollout |

### 3.2 Coverage policy

Do not set a single blanket 99% number as the definition of quality. Keep a reproducible coverage ratchet, but supplement it with module- and behavior-level proof.

For FireCraft:
- Preserve the current 67.61% line floor until a fresh exact-SHA report establishes the correct new floor.
- Add branch coverage with a stable toolchain and a reproducible report; do not claim branch coverage until CI actually emits it.
- Track lines, functions and branches separately, with per-module targets for core logic, serialization, state transitions, updater and bridge code.
- For UI/rendering paths that do not map meaningfully to line coverage, use scenario and state assertions, not trivial getter tests.
- Raise floors only after two consecutive CI measurements agree on the same commit and environment. No floor lowering to repair a red build.

For WebOS:
- Keep statements, branches, functions and lines ratchets from `vitest.config.js`.
- Preserve the four known V8/browser-attribution exclusions with a short rationale, not a broad exclusion list.
- Add new feature tests alongside implementation. Avoid tests written only to execute a line or inflate coverage.

### 3.3 Test quality rules

- Assert externally visible behavior and invariants, not internal call counts unless the count itself is a contract.
- Test both success and meaningful failure paths.
- Do not silently swallow unexpected test logs, warnings, panics or browser errors.
- Use fake timers for time-dependent behavior; no test may pass or fail because the runner was faster.
- Avoid public internet dependencies in required E2E. Use local fixtures for deterministic flows; optional live probes must be labeled optional and cannot masquerade as a passed required test.
- Every regression fix adds a test that failed before the fix.
- Keep golden files deterministic and make golden regeneration explicit and reviewable.
- Use timeouts, cancellation, cleanup and teardown assertions for tasks, event listeners, MessagePorts, workers and child processes.
- Track ignored tests with an owner and reason. A growing ignore count blocks release until dispositioned.

---

## 4. FireCraft CI and lint hardening

### 4.1 One exact-SHA quality workflow

Refactor the workflows around one reusable required-check definition. It must run for:
- pull requests to `main` and `staging`;
- pushes to protected `main` and `staging`;
- GitHub merge queue candidates via `merge_group`;
- manual diagnostics (without granting extra write permissions).

Every status must be attributable to the same expected integration SHA. In particular:
- Align the coverage workflow with the primary CI merge candidate, or combine coverage in the reusable workflow.
- Do not allow coverage on PR head to be used as a substitute for testing the merge result.
- Pin Rust toolchain, cargo tools and wasm-bindgen CLI. Use `--locked` in all build/test/install commands where dependencies are locked.
- Make branch-specific required checks report reliably; avoid conditions that skip a required check and leave the queue waiting or accidentally passing.

### 4.2 Required pull-request gates

1. Formatting: `cargo fmt --all -- --check`.
2. Strict lint: `cargo clippy --all-targets -- -D warnings`; preserve `unsafe_code = deny` and Clippy correctness denial.
3. Build/tests: `cargo test --locked --all-targets` and doc tests.
4. Docs: link, claims, spelling/formatting and module-size gates.
5. Coverage: statement/line/function/branch ratchet where supported.
6. Dependencies: `cargo deny` and `cargo audit` with a tracked advisory exception policy.
7. Security: CodeQL and secret/pattern scan; unsafe-code boundary report.
8. WASM: release-mode build, pinned wasm-bindgen, complete bundle validation, size budget and feature metadata check.
9. Browser: Playwright E2E against the exact built bundle; no dependency on the public FireCraft Pages deployment.
10. Native build smoke: Windows/Linux on PR; macOS PR build where runner cost permits, otherwise a required exact-SHA release matrix plus scheduled main check.
11. Artifact identity: source SHA, tree SHA, manifest, file inventory, checksums and provenance.
12. Accessibility: automated structural checks, keyboard/focus behavior harness, and a manual release checklist.

The complete workflow should upload test results, coverage artifacts, browser screenshots/traces, SBOM and build manifests. A passing check without retained diagnostics is not enough for a release incident review.

### 4.3 Local validation modes

Replace the ambiguous “all gates passed” behavior in `scripts/ci-local.sh` with explicit modes:

| Mode | Purpose | Behavior |
|---|---|---|
| `--quick` | Fast developer feedback | fmt, Clippy, focused tests, docs/module-size; explicitly labels omitted gates |
| `--full` | Reproduce PR CI | all local gates available and required; fails if a mandatory tool is missing |
| `--release` | Release/agent promotion | fail-closed full gate plus WASM package/browser smoke, security and artifact verification |

A skipped optional developer convenience is not a skipped required release gate. Emit a summary such as “PASS 12 / SKIP 0 / FAIL 0” with a machine-readable artifact. The full/release script must never print “all gates passed” when audit/coverage/browser validation did not run.

### 4.4 Lint policy

Keep the current Rust policy. Extend carefully:
- turn on `rustdoc::broken_intra_doc_links` and `rustdoc::bare_urls` warnings/denial where practical;
- add curated pedantic Clippy rules by module or rule family after measuring existing debt, not a blanket flood of warnings;
- use `cargo deny` license/source/bans policy and a dependency update cadence;
- preserve explicit comments for the small reviewed Windows COM unsafe boundary;
- add code-size/complexity reports to pull requests, but do not treat a module-size warning as a correctness failure unless the hard budget is exceeded.

### 4.5 Dependency advisory policy

The 13 allowed audit warnings must be dispositioned individually:
- name/version and advisory;
- direct/transitive dependency path;
- advisory severity and exploitability in FireCraft's enabled features;
- fixed version / upstream issue / blocker;
- owner and review/expiry date;
- why temporary acceptance is safe enough for the current release tier.

Block new critical/high vulnerabilities by default. New unsound advisories require explicit policy disposition even if the affected code appears behind an unused feature. Keep the lockfile and cargo-deny policy reviewable. Re-run after every dependency update and before release.

---

## 5. FireCraft test expansion matrix

### 5.1 Core/document state

Add scenario tests for:
- create/open/save/save-as/import/export/close/quit with dirty documents, multiple tabs, failed write and recovery;
- undo/redo after every high-value mutation, including multi-selection and multi-page/state actions;
- document schema version read/write, unknown fields, legacy payloads and future-version refusal;
- selection, layer, page and state invariants after failed commands;
- crash recovery and atomic snapshot replacement.

### 5.2 Format compatibility

Create an explicit matrix for every declared format. Each row must state:
- import supported? export supported? preserve editability? expected fidelity?
- alpha/transparency and color space behavior;
- page/dimension/resolution handling;
- metadata and animation/state behavior;
- failure behavior and size limits;
- golden fixture(s), round-trip assertions and any known loss.

Prioritize:
1. PNG, layered .fw.png, SVG, JPEG, GIF, WebP, TIFF, BMP, ICO and HTML exports currently advertised.
2. SVG import parser edge corpus: namespaces, path commands, transforms, text escaping, malformed attributes, external references and XML entity safety.
3. PNG iTXt container validation: invalid lengths, CRC, duplicate FireCraft chunks, unsupported payload versions, truncated IEND, enormous payload sizes.
4. Raster rendering: transparent backgrounds, alpha composition, gradients, groups, opacity, hidden layers, unsupported shapes, high dimensions and memory bounds.
5. Unknown extensions: reject rather than default to PNG.
6. Add fuzz targets for SVG parsing, PNG metadata extraction, project deserialization and geometry/path operations; seed them with sanitized real-world and adversarial fixtures.

Do not claim Photoshop/Illustrator import parity for placeholder Format enum values. Tests and UI must distinguish “extension recognized” from “codec implemented”.

### 5.3 UI and native interaction

Extend the existing egui harness around user-visible state transitions:
- keyboard focus and accelerator priority across menus, text inputs, tabs, dialogs and canvas;
- file browser filter/extension/overwrite/cancel flows;
- selection/multi-selection, property edits, tool changes and undo;
- panel docking, collapse, floating, resize, snap and persisted workspace;
- high-DPI and minimum-size scenarios;
- accessibility labels and keyboard equivalents for canvas-only interactions.

Snapshot/pixel comparison may supplement but must not replace state and action assertions. For interaction tests, assert document state, focus owner, selection, dirty-state and undo result.

### 5.4 Browser/WASM E2E

Use the exact release-mode artifact built in CI, not the public hosted app. Required journeys:
- start and wait for WebRunner ready; verify canvas has a non-empty painted frame and no uncaught JS/WASM error;
- create a document, draw a vector and bitmap, undo/redo, save/reopen and export;
- text clipboard and image clipboard paste; denied permission, unsupported MIME, malformed image, focus changes and large payload limits;
- browser file download/upload path and no silent save;
- standalone browser mode without WebOS available;
- WebOS host protocol handshake, capability denial, file open/save, shutdown/reload and timeouts (after the bridge contract phase);
- WASM load on supported browsers and reduced-memory failure handling.

E2E reports must state browser version, OS/runner, WASM SHA, generated bundle SHA, test duration and screenshots/traces. Keep an offline/no-egress mode and test it.

### 5.5 Performance budgets

Maintain repeatable benchmarks for:
- editor startup and first canvas paint;
- p50/p95 input-to-paint for canvas selection/move/zoom at 500, 5,000 and 20,000 objects;
- large bitmap import, layered project open/save, SVG parse and export;
- peak resident memory / browser heap for large documents;
- WASM initial download size, decompressed size, compilation and first interactive frame.

The recorded 500-object debug/software-raster frame test is a useful guard, not a universal performance certification. Establish optimized native and browser baselines on named runner hardware before setting tighter ceilings.

---

## 6. FireCraft ↔ ArtCraft WebOS ownership and integration contract

### 6.1 Clear project boundary

| Responsibility | FireCraft repository | ArtCraft WebOS repository |
|---|---|---|
| Editor and document model | Owns | Does not copy/editor-fork |
| Standalone browser/WASM build | Builds and publishes immutable package | Downloads package |
| Host API client | Implements in Rust/WASM against a documented protocol | — |
| Host API server | — | Implements and enforces it |
| App catalog / store metadata | Publishes app package metadata and compatibility | Owns catalog display and discovery |
| Install/update/cache/rollback | Supplies versioned releases and package manifests | Owns install lifecycle and last-known-good versions |
| User file persistence | Requests scoped operations | Owns IndexedDB/OPFS storage and permissions |
| Sandboxing/capability grants | Declares requested capability requirements | Enforces grants and browser isolation |
| Desktop OS shell / windows | Does not implement | Owns |
| Support and engineering pipeline | Owns FireCraft tickets and code PRs | Owns WebOS tickets and code PRs |

Do not make WebOS a subdirectory or dependency of FireCraft. Do not commit WebOS sources or fetched FireCraft builds into the other repository. Cross-project coordination is through a versioned package manifest, host API contract, compatibility fixtures and independent PRs.

### 6.2 Required FireCraft-side deliverables

- Add `docs/WEBOS-HOST-INTEGRATION.md` or equivalent specification linked from this roadmap.
- Add a versioned Rust/WASM client module for the host API, keeping the standalone browser fallback functional.
- Add message schemas and host protocol conformance fixtures to FireCraft tests.
- Advertise exact host API compatibility range and requested capabilities in every WASM package manifest.
- Ensure no host bridge receives arbitrary shell commands, arbitrary filesystem paths, secrets, raw user identity, or unsanitized support payloads.
- Make every host request typed, scoped, cancellable, bounded by a timeout and safe to retry where appropriate.
- Include an explicit fallback state if the app is opened outside WebOS.

### 6.3 Host API protocol rules

The protocol must:
- be versioned and negotiated at startup;
- have a dedicated app identity and per-launch session nonce;
- use request IDs, typed request/response schemas, bounded payloads, timeouts, cancellation and deterministic error codes;
- carry capabilities granted by WebOS, not infer permission from origin alone;
- restrict file operations to `/Apps/<app-id>/` unless the user explicitly chooses a file through a host picker;
- distinguish autosave/recovery from explicit save/export;
- not imply that data was saved until WebOS confirms durable storage;
- close/revoke channels on iframe navigation, app shutdown, uninstall or permission revocation.

The current WebOS generic `webos:file-*` protocol is a useful starting point for storage scoping. Do not create a second unauthenticated file API in FireCraft. Agree a single host-owned contract and versioned schema in the WebOS plan.

### 6.4 Package and release contract

FireCraft release asset for WebOS is the **complete browser package ZIP**, not only a raw .wasm binary. A package manifest must record:
- app ID, product/version, release channel and package-schema version;
- repository, immutable tag, source commit SHA and workflow identity;
- artifact filename, digest, byte size and complete asset inventory;
- entrypoint, wasm-bindgen glue, WASM file, static assets and MIME types;
- host API version range and WebOS minimum version;
- required/optional capability IDs and defaults;
- feature-flag schema version and per-feature default state;
- document schema read/write compatibility;
- build provenance/attestation and dependency/SBOM references.

FireCraft publishes immutable GitHub Release assets and manifests. WebOS resolves channel pointers, verifies the release/package identity and installs a versioned copy. Cloudflare Pages remains available for the standalone web editor, preview/staging and public demo; it is not a requirement to host one permanent application website per WebOS installation.

---

## 7. Autonomous support and engineering delivery

The full system design remains in [Autonomous Support & Release Pipeline](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/blob/docs/support-feedback-pipeline-handoff-2026-11-27/docs/SUPPORT-FEEDBACK-PIPELINE-HANDOFF-2026-11-27.md) on the earlier planning branch. This roadmap refines its quality and integration dependencies.

### 7.1 Required policy

- Reproducible bugs: prove failure on the baseline, create a regression test, implement a minimal fix, run all relevant tests and promote through staged validation.
- Feature/change requests: score independent demand and strategic fit; build suitable work behind a unique Experimental flag, disabled by default.
- Coding agent: isolated branch/worktree, ticket-scoped credentials, bounded tools, strict budget and no direct writes to main/release tags.
- Routine R0/R1 tasks: PR can auto-merge to staging only after exact-SHA required checks and merge-queue verification.
- Release controller: deploy staging, publish immutable nightly assets, collect opt-in cohort data and promote/rollback automatically using deterministic thresholds.
- Hard stop: unresolved security, data-loss, licensing, provenance, migration or uncertain test results. A hard stop is automatic fail-closed behavior, not a human approval queue.
- Rollback: feature kill switch first when isolated; otherwise restore prior channel/artifact pointer, verify health, record source SHA and notify affected opted-in users.

### 7.2 Do not confuse three separate states

1. **Release channel**: stable / beta / nightly distribution source.
2. **Feature exposure**: per-feature Experimental flag, off by default until user explicitly opts in.
3. **Rollout percentage**: cohort promotion of a candidate build.

Nightly opt-in never implies that all Experimental features become enabled. Binary promotion to stable does not automatically force a new feature to Stable/default-on.

---

## 8. Hill climb: ordered phases and measurable exit gates

The sequence deliberately closes validation weaknesses before granting autonomous control over staging and stable production.

### H0 — Repository and release rebaseline

- Reconcile current main, open PRs, tags, latest release and task ledger.
- Resolve the stale Phase 36 entries against the actual v0.3.2 release and source SHA; preserve any unverified native acceptance items.
- Re-run the full local/hosted baseline on exact main and record test counts, lint, coverage, advisories, browser evidence and package identity.
- Confirm WebOS current main and its actual latest CI run separately.

**Exit:** one current evidence snapshot per repo; no historical number is presented as a current test result.

### H1 — Make required gates fail closed

- Split local quick/full/release commands.
- Ensure required tools cannot silently skip in full/release mode.
- Align PR and coverage checks to the merge candidate; add `merge_group`.
- Pin Rust, wasm-bindgen, Node and action/tool versions; require lockfile-respecting builds.
- Reusable quality workflow for both PR and release paths.

**Exit:** a deliberately removed required tool or a deliberately failing check makes the full/release gate fail; exact merge SHA is visible in every required status.

### H2 — Expand high-value regression coverage

- Build the file-format matrix and tests.
- Add branch coverage and reproducible per-module reporting.
- Add lifecycle/recovery/undo invariants and serialization compatibility/fuzz corpus.
- Address known unknown-extension and secret-handling defects with regression tests.
- Add test-log policy and warning cleanup.

**Exit:** every advertised format path has a capability/acceptance row; critical document/state invariants have executable regression tests; no new advisory or ignored test is unexplained.

### H3 — Browser and platform proof

- Browser E2E against the built WASM artifact.
- Clipboard, save/reopen/export and startup failure matrix.
- Windows/macOS/Linux package install/update smoke plus manual platform acceptance receipts.
- Capture performance and memory baselines.

**Exit:** standalone WASM journey passes on the required browser matrix; package and runtime identity match source SHA; untested platform claims are explicitly limited.

### H4 — Formalize WebOS host protocol

- Agree the versioned capability/message contract with WebOS.
- Implement the FireCraft Rust/WASM client without handwritten JS application logic.
- Add a local host fixture and contract tests in FireCraft.
- Add standalone-mode fallback tests.

**Exit:** FireCraft operates through the WebOS protocol using scoped capabilities; standalone browser mode remains functional; tests prove malformed, stale, unauthorized and timed-out messages fail safely.

### H5 — WebOS secure package runtime

WebOS implementation is tracked in the **WebOS repository**, not this one:
- fix origin/navigation trust and wildcard data messaging;
- add sandbox/capability broker;
- download complete FireCraft ZIP from GitHub Releases;
- verify digest/provenance/manifest/host ABI before install;
- versioned cache, current + previous known-good, explicit channel selection and rollback;
- local deterministic test package; no public FireCraft site required for app launch.

**Exit:** WebOS can install and run a local FireCraft package with no per-app website, cannot leak autosave to a navigated/hostile frame, and can roll back offline.

### H6 — Autonomous support intake and coding-agent execution

- Worker/D1/R2 private intake and user-consented diagnostics.
- Deterministic state machine, duplicate/demand scoring and bug reproducer.
- Provider-neutral agent adapter, evaluated OpenCodifier/fallbacks, isolated branch/worktree, short-lived tokens and bounded retry.
- Regression tests and hidden acceptance tests before PR.

**Exit:** a supported bug report can autonomously become a passing exact-SHA PR; untrusted issue text cannot elevate permissions or escape the task sandbox.

### H7 — Staging, nightly, cohorts and rollback

- Protected staging and merge queue.
- Staging Pages review build with protected access and synthetic test data.
- Immutable GitHub nightly artifacts and channel manifests.
- Opt-in test cohort; per-feature flags disabled by default.
- Sample/time and health thresholds; feature kill switch and channel rollback.

**Exit:** simulated fix and experimental feature ship to staging/nightly, then promotion; a deliberately injected regression triggers rollback and restores last-known-good without touching user documents.

### H8 — Desktop updater and public stable release

- Stable/nightly channels, explicit switch, pin and downgrade.
- Platform-specific staging/update logic; package-manager-managed DEB/Flatpak remain owned by their package managers.
- Signed/attested releases where keys and platform support allow; checksum verification always required.
- Health-check/rollback tests and user-facing release notes.
- Stable rollout with platform-specific evidence and automatic pause/rollback.

**Exit:** an interrupted or bad update recovers to the previous binary; user documents/settings remain intact; stable release is promoted only from a verified immutable artifact.

### H9 — Continuous improvement

- Monitor test efficacy, flaky rate, false-positive scans, bug escape rate, update success, rollback time and cost per ticket.
- Expand formats, accessibility and performance tests based on actual user reports and uncovered risks.
- Rotate/re-evaluate model providers against held-out ticket benchmarks.
- Raise coverage/performance/security floors based on measured baselines.

**Exit:** quarterly quality review is evidence-based; no ratchet is lowered without a documented replacement metric and explicit risk disposition.

---

## 9. Release-readiness scorecard

Do not average away a failed security or data-integrity gate. Use the scorecard as a visible hill climb; hard gates remain non-negotiable.

| Dimension | Green condition | Stop condition |
|---|---|---|
| Source identity | commit/tree SHA recorded, clean build provenance | unknown/stale/mismatched SHA |
| Tests | exact-SHA required suites green; no unexplained ignored/flaky tests | failed, skipped required test, or untriaged flaky behavior |
| Lint | fmt + strict Clippy with zero warnings | warning suppression to force green |
| Coverage | reproducible line/function/branch reports; floors met | floor reduced or report absent |
| Security | no new critical/high; existing advisories individually governed | unknown exploitability or unapproved advisory |
| Formats | capability matrix and regression fixtures pass | corrupted/lost documents or misleading export |
| Browser | canvas paint and core workflows pass on built artifact | WASM compiles but does not run/paint |
| Platform | install/update/save/reopen/export receipts per supported OS | claim based only on cross-build |
| WebOS | secure capability bridge, exact package hash, offline rollback | wildcard secret-bearing messaging or package not verified |
| Rollout | telemetry sample sufficient and all thresholds healthy | missing signal, suspected data loss, rollback failure |
| Documentation | advertised support matches proven behavior | unsupported capabilities presented as complete |

---

## 10. Agent handoff and first-run checklist

The implementation agent must work the phases in order. Do not attempt a broad UI rewrite and pipeline rewrite in one branch.

- [ ] Read `AGENTS.md`, `CLAUDE.md`, current task ledger, product roadmap, architecture docs and release workflows.
- [ ] Record git status, current branch, main/tag/PR state; preserve all existing changes.
- [ ] Reconcile Phase 36 with the published v0.3.2 release and attach exact-SHA receipts before changing task states.
- [ ] Re-run FireCraft full gates and create a dated audit snapshot.
- [ ] Reconcile the latest WebOS version/release state with its own maintainer workflow; do not edit its repository from a FireCraft PR.
- [ ] Fix fail-open local gate behavior and workflow SHA/merge-group mismatch.
- [ ] Prioritize the P1/P2 code and validation findings with a regression test for every confirmed fix.
- [ ] Write and test the protocol before implementing any FireCraft host-specific behavior.
- [ ] Coordinate with a separate WebOS agent/branch for iframe security, package manager and release-gate changes.
- [ ] Keep PRs small and sequenced: tests/gates → formats/security → browser → protocol → support automation → staged rollout → updater.
- [ ] Update this roadmap, task ledger, ADRs, user docs, and dated evidence snapshot in each implementation PR.
- [ ] Mark work done only when the exact exit gate passes.

## 11. Evidence and linked plans

- [Autonomous Support & Release Pipeline Handoff](SUPPORT-FEEDBACK-PIPELINE-HANDOFF-2026-11-27.md) — earlier detailed support automation specification; retain its branch for historical continuity.
- [TASKS.md](TASKS.md) — active FireCraft checkboxes.
- [PRODUCT_ROADMAP.md](PRODUCT_ROADMAP.md) — user-facing scope and feature sequencing.
- [AUDIT-2026-11-26.md](AUDIT-2026-11-26.md) — historical audit, not a substitute for the newer PR CI results.
- [quality-gap-analysis.md](research/quality-gap-analysis.md) — architecture and testing review.
- [CI workflow](../.github/workflows/ci.yml), [coverage workflow](../.github/workflows/coverage.yml), [release workflow](../.github/workflows/release.yml).
- WebOS plan of record: `aliasfoxkde/ArtCraft-WebOS/docs/planning/FIRECRAFT-WASM-PACKAGE-AND-RUNTIME-HANDOFF-2026-11-27.md` on branch `docs/firecraft-wasm-package-runtime-roadmap-2026-11-27`.

**Completion rule:** FireCraft is not ready for autonomous stable rollout merely because the Rust suite is green. It is ready when a reproducible browser artifact, secure WebOS integration, exact-SHA release gates, independent platform evidence, and tested automatic rollback all work together.
