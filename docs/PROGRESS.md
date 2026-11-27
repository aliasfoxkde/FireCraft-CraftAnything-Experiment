# Progress log

Newest first.

### 2026-11-27 — TDD and verification follow-up

- Re-ran the Save All & Quit regression by its full filter and the full suite on the current tree: 190 unit tests passed, one isolated frame-time test was ignored, and 12 CLI, 5 hygiene, and 10 MCP tests passed. Strict all-target Clippy (`-D warnings`), WASM target check, docs/link validation (63 files), rustfmt, diff check, and module-size hard budget passed. The new behavior test passes against existing production code; it did not reproduce a defect, so no production change was fabricated.
- Recorded managed Aegis scan `0889e9ed-bbbe-4782-af42-597c4f3b9456` (Aegis 0.6.6, production profile, tree `07f2cba1d5c432bd3234e9193cdca6df52b6a51a`): 11,323 findings, with 11,241 in retained `investigation.cany/` evidence. Secret-shaped prose/temp-directory names and two static Apple plist DTD alerts were reviewed as false positives. The scan is non-clean; 82 findings outside retained evidence and individual evidence matches remain open under TASK 41.2. No raw candidate values were copied into project docs.
- GitForge services currently answer HTTP 200 on ports 42780–42782, but the local `gitforge-status` CLI is absent and the installed server state is documented as stale. No passing local pipeline is claimed; branch-level verification is still pending.

### 2026-11-27 — TDD: Save All & Quit across untitled and named documents

- Added `save_all_and_quit_waits_for_untitled_save_as_then_saves_named_dirty_documents` in
  `app/src/app/files.rs`. The regression drives the app-level quit step, confirms it switches to
  the untitled tab and opens Save As, completes that dialog through the real commit callback, then
  asserts the named dirty document is saved afterward and no dirty document remains to intercept
  close. A shared in-memory `FileService` keeps the flow deterministic and disk-free.
- The focused test passed without production changes; the existing flow met the asserted contract.
  Full local verification passed: 190 unit tests, 12 CLI, 5 hygiene, and 10 MCP tests; one isolated
  frame-time test remains ignored. Strict all-target Clippy (`-D warnings`), WASM target check,
  rustfmt, `git diff --check`, and module-size passed. Docs/link validation is run with this update.
- This expands behavior coverage in the multi-document close path; overall 99% coverage remains
  unverified for this branch and stays open under TASK 41.3.

### 2026-11-27 — TDD: pixel-exact bitmap flatten and undo contract

- Added `flatten_selected_bitmaps_is_pixel_exact_and_undo_restores_sources_and_order` at the app
  behavior seam. It verifies selection order does not change stack placement, source-over alpha
  composition is exactly `[127, 0, 128, 255]`, the composite uses the expected origin and 6×4
  bounds, page dimensions remain stable, undo restores the original objects and bytes, and redo
  restores the exact composite bytes. The focused test passes; no production mismatch was found,
  so this closes a test-coverage gap without changing raster behavior.
- Verification: the focused test passed; `cargo test --locked` passed 189 unit tests, 12 CLI tests,
  5 hygiene tests, and 10 MCP tests (one isolated frame-time test ignored); strict all-target
  Clippy (`-D warnings`), WASM target check, rustfmt, `git diff --check`, the 63-file docs/link
  checker, module-size gate, and `cargo machete` passed. `cargo audit` completed with 13 allowed
  transitive advisories (9 unsound `heapless 0.8.0` notices and 4 unmaintained-crate notices);
  dependency task 38.6 remains open. `cargo update -p rstar` confirmed no compatible lockfile update
  under the current `geo 0.33.1` constraints. Exact PR-head coverage artifact from
  [run 46036827007](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/actions/runs/46036827007)
  measures 18,496 / 26,504 = **69.79%** lines; branch counters are 0/0 and 99% is not met.
- The first targeted Cargo invocation used `--lib`, which is invalid for this binary-only crate and
  exited before compilation. The corrected focused command omitted `--lib` and passed. A red-before
  behavior failure was not claimed because production behavior already met the newly asserted
  contract; the added test is a regression guard for the previously untested exact output.
- GitForge accepted the branch push, but the current local service snapshot from
  `/nas/Temp/repos/GitForge/scripts/gitforge-status --json` is `degraded`: the deployed build
  `gitforge-989b33e2-20260925` is 273 commits behind its local `origin/main`. No passing GitForge
  receipt is claimed for this test branch; TASK 28.2 remains open. The current GitHub PR checks are
  the executable hosted validation for this change.

### 2026-11-27 — marketing deployment and repository closeout follow-up

- Merged the v0.3.5 audit/task documentation as [PR #40](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/pull/40). Gates, coverage upload, Windows build/CLI smoke, WASM build, and Analyze Rust passed.
- Merged the legacy repository's 1.0 execution plan [PR #188](https://github.com/aliasfoxkde/FireCraft/pull/188), marketing refresh [PR #190](https://github.com/aliasfoxkde/FireCraft/pull/190), and deployment/accessibility follow-up [PR #191](https://github.com/aliasfoxkde/FireCraft/pull/191). The follow-up includes failing-first nav/contrast and Cloudflare destination tests; the full site suite is 10/10, both shell self-test suites pass, and `actionlint` passes.
- Cloudflare confirmed the account has reached 10 Pages projects and `firecraft-site` does not exist. To preserve the production WASM app, the site workflow targets a stable preview branch on `firecraft-web`. Merged source `0c80ad76` was manually deployed to [marketing-review.firecraft-web.pages.dev](https://marketing-review.firecraft-web.pages.dev/); landing, clean `/extensions`, styles, and logo return HTTP 200. Production `firecraft-web.pages.dev` still serves the app; `firecraft.app` still redirects to `/coming-soon`. The separate marketing project/custom-domain production launch and rollback are still open.
- The first GitHub deploy failed because the Pages project was missing and the repository token secret was absent. The verified token has been added as `CLOUDFLARE_API_TOKEN`; follow-up workflow [46031404998](https://github.com/aliasfoxkde/FireCraft/actions/runs/46031404998) then passed and deployed source `fdc0e282` to the stable preview alias.
- The legacy repository has no open PRs, but 19 remote branches remain. The canonical repository has eight non-main remote branches, including four protected release branches and three docs branches without active PRs. Histories are divergent or protected, so none were deleted; see the [legacy](integration/legacy-branches-2026-11-27.tsv) and [canonical](integration/canonical-branches-2026-11-27.tsv) inventories. Issue triage remains open and no issues were changed pending maintainer choices and artifact-level acceptance.

### 2026-11-27 — v0.3.5 release candidate and TDD follow-up

- PR [#39](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/pull/39) is based on merged main commit `87089407bbdc35d379aae42531327d3da187c71d`; exact PR head `f294e9f8afec100e6b320f05f96393b8f1886c80` passed GitHub Gates, Rust coverage, Windows build/CLI smoke, WASM and CodeQL. CodeRabbit remains skipped by repository configuration. PR #37 was closed after its protected branch refused force-push; GitHub also refused remote deletion of that protected branch.
- The latest Cobertura artifact ([run 46013680520](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/actions/runs/46013680520)) reports 18,425 / 26,445 = 69.67% lines, with no branch counters. This is unchanged from PR #36 and remains below 99%.
- TDD caught `scripts/check-docs.sh` silently passing when `git ls-files` failed and returned an empty list. New hygiene test failed before the fix with `docs: 0 files checked`; the checker now fails closed, explicitly allows its mounted checkout as a Git safe directory, and passes with 61 tracked Markdown files.
- Image 4 is `sha256:d03a844a27237b333ec1b9bc9d1a8989d822227d8d6bc2c57894c8ad755887d8`, labeled for Cargo.lock SHA `76d7fb62c41c6939bc86debc39b2f7270a3f2e98f744e570db4bee1f89201432`. Exact GitForge run `d8664d77-8b38-41f5-99cf-b00175acc266` for PR head `f294e9f8afec100e6b320f05f96393b8f1886c80` succeeded at 14:26:37 UTC, including the offline WASM release build.
- The Windows review artifact from the PR workflow is `output/review-pr-39/firecraft-windows-x86_64/firecraft-v0.3.5-windows-x86_64.zip`, SHA-256 `316a5d27cf3450b68e16593b70bb326aba1e07dfc0f558d5f3a352d3f670728d`. Its build info says version 0.3.5 and source `ec359286cb2e738340863f40f98d2020c2edb83f`, the workflow's PR merge ref. It is a review ZIP, not the published release. Windows 11 GUI edit/save/export acceptance is still reviewer-owned.
- Offline image-4 gates passed: formatting, strict Clippy, 188 unit tests, 12 CLI tests, 5 hygiene tests, 10 MCP tests, the isolated frame-time test, docs and module-size. Offline release-mode WASM build passed. Native GUI, browser WebGL2 startup/clipboard, Cloudflare deployment, managed Aegis scan, exact tagged multi-platform release and 99% coverage remain open.

### 2026-11-27 — TDD: join selected paths by endpoint proximity

- Added `join_orders_three_paths_by_proximity_instead_of_selection_order`. The red run failed with connector lengths `[100.0, 12.0]` for paths selected in spatially poor order. After independent review, tightened the regression to assert the complete expected six-point chain so every source segment and connector order is checked; that focused test passes.
- `combine_paths_op("join")` now selects the closest endpoint pair as the initial chain, then appends each remaining contour from the endpoint nearest the current tail. Equal distances use stable contour order and orientation tie-breaks; two-path behavior and open-path state are preserved.
- Green verification: `cargo test --locked` passed 188 unit tests, 12 CLI tests, 4 hygiene tests, and 10 MCP tests; one isolated frame-time test remains ignored. Strict Clippy, formatting, `git diff --check`, and `scripts/check-docs.sh` (60 files) passed. Exact merged PR #36 head passed Gates, coverage, WASM, Windows build/CLI smoke, and CodeQL; CodeRabbit is skipped. Native visual path-tool acceptance, compound Boolean outputs with holes/disconnected contours, and full renderer/export parity remain open.
- Cobertura run [46007345363](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/actions/runs/46007345363) for source `1600d761333ba1ddab19e6b75455805a108ffdaa` reports 18,425 / 26,445 = 69.67% line coverage; branch counters are absent (0/0).
- The passing Windows workflow artifact was copied to `output/review-pr-36/firecraft-v0.3.4-windows-x86_64.zip`; SHA-256 `f3dcd4e5adb6ffbd62c2d6c904b35a871ae9e9b9466f93f36e888619f158d89f`. Its `BUILD-INFO.txt` source is the workflow's PR merge ref `27ecf1aed0064db675b997d63ca92ce8418f4c94` (parents: main `5d65027` and PR head `1600d76`). This is a review artifact carrying the current release version string, not a replacement release; native Windows GUI acceptance remains with the reviewer.

### 2026-11-27 — Offline GitForge CI image

- Added a FireCraft CI image build definition and script using a digest-pinned Rust 1.100 base,
  baked `rustfmt`, Clippy, the WASM target, Linux native headers, and dependencies from the locked
  Cargo manifest. `.gitforge.yml` now uses this image and declares Cargo offline; job steps no
  longer install OS packages, Rust components, targets, or crates.
- Built local image `firecraft-ci-rust:3`, ID
  `sha256:45d384318f910a9fd6efdd9d205b8d512177c6649e1816331f6c2864aa4dcb51`; its Cargo lock
  label is `27a923efe41aa380a23c452c86b921dbf38b23a698878b5473a9d64500756bdd`. The offline
  `scripts/ci-local.sh --quick` run passed: fmt, strict Clippy, 187 unit tests, 12 CLI tests,
  4 hygiene tests, 10 MCP tests, the isolated frame-time test, all 60 documentation files, and
  the module-size gate. One timing test remains ignored in the ordinary suite and passed in its
  isolated run.
- `cargo build --release --locked --target wasm32-unknown-unknown` passed with Docker
  `--network none`; produced a 14 MiB WASM artifact. Local image validation does not yet prove
  the GitForge runner can access this Docker image tag. A successful GitForge receipt remains
  open in TASK 28.2; if runner and build host do not share an image store, the image must be
  pushed to a runner-accessible registry and the pipeline tag updated.

### 2026-11-27 — Gradient SVG axis export TDD (PR #34 merged)

- Added a vector-export regression that failed because per-object gradients were omitted from SVG; after adding `userSpaceOnUse` gradient definitions, a second regression caught duplicate gradient IDs when objects on separate layers reused local indices. SVG object IDs now come from one traversal counter, including grouped objects.
- The gradient axis endpoints are projected from normalized object bounds into document-space SVG coordinates. Zero-length axes export a stable midpoint color. Focused SVG tests cover object-axis coordinates, stops, unique IDs across layers, and the inline SVG included by HTML export. Standalone CSS output still ignores editable gradient axes. Multiple stops, explicit linear/radial controls, broader geometry/concave path validation, and visual acceptance remain open.
- Local verification passed: the full native suite (213 passed, 1 isolated timing test ignored), strict Clippy, WASM check, formatting, whitespace, and the 60-file docs/link check. Exact PR head `7b23745118666591948e135c0d69a5d3c5288c0a` passed Gates, Rust coverage, WASM, Windows build/CLI smoke, and CodeQL. PR [#34](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/pull/34) merged as `f775b8d7a3c2edc6d812252dafde4a756916831e`.
- Exact-head Cobertura artifact for run [46000312418](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/actions/runs/46000312418) measures 18,341 / 26,355 = **69.59%** lines. Branch counters are absent (0/0); this remains below 99% and does not establish behavioral or documentation coverage.


### 2026-11-27 — Gradient direction editing TDD (PR #32 merged; task partial)

- Added normalized start/end axis coordinates to per-object gradient fills. Missing coordinates in older documents default to the existing vertical direction. The Gradient tool now exposes draggable color endpoints for a selected gradient object; moving a handle changes only the fill axis and is undoable.
- Canvas mesh rendering and PNG/raster export now project pixels through the same axis math. TDD added tests for legacy serde defaults, horizontal projection, zero-length axes, pointer editing with undo, renderer mesh sampling, and horizontal raster export. The model test first failed on the missing projection method, and the interaction test first failed on missing handles/drag states; each passes after implementation. Raster test also first caught an expected-value mismatch (gamma interpolation is 128 at the midpoint); corrected expectation follows the existing interpolation behavior.
- Verification passed locally: `cargo test --locked` (211 passed, 1 isolated timing test ignored), strict Clippy, WASM check, formatting, whitespace, and the 60-file docs/link check. SVG/HTML export direction, multiple color stops, explicit linear/radial controls, broader path geometry parity, and GUI visual acceptance remain open; this does not complete TASK 37.3.
- PR [#32](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/pull/32) merged as `ff564a0a8c28e98104382f1e82258dccf00ce945`. The exact final head `50a11768e11270e2d1e4823857647a5ab648c0d1` passed Gates, Rust coverage, WASM, Windows build/CLI smoke, and CodeQL. GitHub reports CodeRabbit as skipped, not passed.
- Exact final-head Cobertura run [45996121990](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/actions/runs/45996121990) measures 18,253 / 26,265 = **69.50%** lines; branch counters are absent (0/0). This remains below 99% and the llvm-cov/Cobertura denominator gap remains open.
- Windows review package from the passing PR workflow is at `output/review-pr-32/firecraft-v0.3.4-windows-x86_64.zip`, SHA-256 `fbfef6db36aa49823909b7934486b71bf11184308839dc42ba9f66da2340025c`. Its embedded build info records source `a84db32f82563d519c4b94ed3c358a71909647e4` (the workflow's PR merge ref); it is a review build, not a published release. Native Windows 11 GUI acceptance remains pending.
- Triage snapshot after merge: zero open GitHub issues and zero open PRs. No external issue or PR needed triage. The issue templates only define bug/enhancement categories, with no canonical triage-state label mapping; no labels or tracker mutations were invented.
- The Aegis 0.6.6 scan remains unavailable: the shell hook requires managed admission, and `aegis-scan` is absent from the managed job profiles. No substitute scan or clean result is claimed.
- The architecture scan's four source-backed deepening candidates are pointer/tool interaction (strong; recommended first), canvas hit-testing vs painting (strong), format codecs vs disk transport (worth exploring), and Welcome/splash ownership (speculative). The visual report is `/tmp/architecture-review-2026-11-27-1205.html`; these are planning candidates, not implemented changes.

### 2026-11-27 — Exact-head coverage receipt refresh

- Retrieved the Cobertura artifact for PR #29 run [45989583242](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/actions/runs/45989583242), exact source `982b952945af5deea00b7b1c0eda274dbd5ac80f` (the PR head before squash merge).
- The report measures 18,002 / 25,964 lines = **69.33%**. It has no branch counters (0/0). This supersedes 69.26% from PR #27 as the newest hosted Cobertura receipt; it does not resolve the different llvm-cov denominator or meet the 99% goal.
- Updated TASKS 38.5 and 39.6. Kept prior dated coverage snapshots intact.

### 2026-11-27 — TDD verification: locked Eraser targets

- Added a focused regression test for locked layers and individually locked bitmaps. It checks
  target eligibility, direct pixel mutation, and pointer-down behavior, and confirms the original
  bitmap alpha stays unchanged.
- The current implementation already rejected these targets at both the eligibility and mutation
  seams, so the new test passed without a production edit. This is green-baseline verification;
  no failing-before run or bug fix is claimed. The existing hidden-layer test remains a separate
  coverage case.
- Verification on `origin/main` plus this test: `cargo test --locked` passed 180 unit tests, 12
  CLI tests, 4 hygiene tests, and 10 MCP tests (206 passed; 1 isolated timing test ignored);
  strict `cargo clippy --all-targets --locked -- -D warnings` passed; `cargo fmt --all -- --check`,
  `git diff --check`, and `scripts/check-docs.sh` (60 files) passed. The WASM check
  `cargo check --locked --target wasm32-unknown-unknown` passed.

### 2026-11-27 — v0.3.4 cross-platform release and review handoff

- Published [GitHub Release v0.3.4](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/releases/tag/v0.3.4), tagged from source `c73ea44e90933288fbe8f4c3554ba47d4b574d94`. Release workflow [45974435786](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/actions/runs/45974435786) passed quality, Windows x86_64, universal macOS, Linux x86_64, WASM, publisher, and GitHub Pages deployment jobs.
- Published assets include Windows ZIP, universal macOS ZIP/DMG, Linux tar.gz/AppImage/DEB/Flatpak, WASM ZIP, package/release manifests, source SHA, and SHA256SUMS. The Windows ZIP was downloaded locally and checked against the published checksum: `5d613e835769dbe9c1116f35a7602c0b84ef450cff6c782cd2cf9600e5091fd5`.
- Deployed the exact tagged WASM ZIP to [Cloudflare Pages preview](https://wasm-review-v0-3-4.firecraft-web.pages.dev/). The deployed marker names version 0.3.4 and the tag source SHA. HTML, marker, JS, WASM, and favicon return HTTP 200 with expected MIME; the marker, JS, WASM, and favicon bytes match the release bundle. Cloudflare injects its Analytics script into the served HTML.
- Browser validation remains partial: Playwright loaded the correct page and canvas element, but this local Chromium session did not provide WebGL2. Retrying with SwiftShader stalled while creating the context, so this environment did not prove editor startup or clipboard paste. Do not treat static asset checks as runtime acceptance. Native Windows 11 GUI review is also pending; see [the v0.3.4 checklist](testing/windows-11-review-v0.3.4.md).
- Current coverage remains well below the 99% target; the local/hosted denominator reconciliation remains open under TASK 39.6. The hidden-layer eraser fix is verified separately with its red/green regression and complete repository suite as recorded below.

### 2026-11-27 — TDD: prevent erasing hidden-layer bitmaps

- Added `eraser_does_not_edit_bitmap_on_hidden_layer` at the app interaction seam. Red reproduced the defect: pointer-down changed the target pixel alpha from 255 to 0 while its layer was hidden.
- Updated both bitmap-target hit-testing and the eraser mutation method to reject hidden layers. The regression now proves the direct mutation method returns false, pointer-down leaves the pixel untouched, and hidden-layer bitmap targets are ineligible.
- Focused verification: the red test failed with the observed alpha change before the fix; the green test and all four `eraser_*` tests pass afterward. PR #22 merged as `b494b721acff5f2da6160499236da57afd413906` after hosted Gates, Rust coverage, WASM, Windows build/CLI smoke, and CodeQL checks passed.
- Current local validation: `cargo test --locked` passed 176 unit tests, 12 CLI tests, 4 hygiene tests, and 10 MCP tests (one separately isolated frame-time test is ignored by the ordinary suite); the isolated 500-object frame-time test also passed. `cargo clippy --all-targets --locked -- -D warnings`, `cargo fmt --all -- --check`, `scripts/check-docs.sh` (59 files), and `git diff --check` passed.
- Coverage receipt correction: the exact PR #22 CI log (run 45972076455, head `a550aac30105652ab76ef5b1a2a7718deb3850ae`) reports 26,350 lines / 8,306 missed = 68.48% lines, 33,192 regions / 9,669 missed = 70.87% regions, and 1,576 functions / 453 missed = 71.26% functions. The earlier 70.79% line entry misread the region column and is superseded.
- The separate exact-head Cobertura artifact (run 45972076373) reports 17,736 / 25,736 instrumented lines = 68.92%. The denominator differs from the CI summary; TASK 38.5/39.6 tracks reconciling those report configurations. Neither receipt approaches the requested 99% line target.
- Remaining eraser work is not claimed done: pixel selection masks are not represented in the current object-selection model; selection clipping design, complete locked/hidden target matrix, opacity/tip controls, and native canvas/export visual review remain open in TASKS 37.2/38.1.

### 2026-11-27 — v0.3.3 TDD, cross-platform release, and browser review

- Added a red/green host test for private browser PNG clipboard payload decoding: it verifies a valid deterministic 2×3 RGBA image's dimensions/pixels and rejects malformed payloads without inserting an object. The shared decoder fix passed focused paste tests and the complete exact-SHA GitHub release quality workflow.
- Published [GitHub Release v0.3.3](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/releases/tag/v0.3.3), source `918ca8ddad5b850ae18f4101327a5e76fcf12966`. All release quality gates passed, along with Windows, universal macOS, Linux tar.gz/AppImage/DEB/Flatpak, WASM, publisher, and GitHub Pages deployment jobs. The release title is exactly `v0.3.3`; all platform assets and source manifests are present.
- Deployed the exact WASM artifact to [Cloudflare Pages preview](https://wasm-review-v0-3-3.firecraft-web.pages.dev/). HTML, build marker, JS, WASM, and favicon returned HTTP 200 with expected MIME types; marker source/version matched the release. Headed Chromium loaded the editor, then pasted a generated 32×24 PNG via the system clipboard; the canvas showed it as a selected bitmap.
- Downloaded and checksum-verified the Windows review ZIP at `output/release-v0.3.3/firecraft-v0.3.3-windows-x86_64.zip`; SHA-256 `2222474b3d79fd12040242321da219a76f140179fb6ecad7a8b2e910978d751a`. Its `BUILD-INFO.txt` identifies v0.3.3 and the release SHA. Windows GUI acceptance remains with the reviewer; see [checklist](testing/windows-11-review.md).
- Current coverage artifact measured **17,708 / 25,709 lines (68.88%)**. This does not meet the requested 99% target; it remains open with focused behavior and high-risk code paths prioritized. Native OS clipboard qualification, browser permission/focus/large-payload tests, and Windows/macOS/Linux GUI acceptance remain open. Cloudflare marketing-site deployment is distinct from the WASM app preview.

### 2026-11-27 — Transparent, live startup splash

- Removed the full-window dark scrim and fake progress bar. The splash now fades the transparent
  brand art over the editor for at most 2.4 seconds, while the workspace continues rendering and
  pointer events remain available. Click or Escape skips the animation.
- Startup now opens directly to the editor; the separate Welcome experience remains available by
  explicit action instead of appearing after the splash.
- Added deterministic fade timing and harness checks for the workspace rendering before, during
  and after the splash. Targeted checks passed; full gates and hosted native/WASM checks remain
  pending.
- A further harness check proves Tab toggles canvas-only mode both directions during the splash,
  without pointer refocus (1 targeted test passed). Because the splash implementation predated this
  resumed TDD request, a failing-before receipt could not be captured without reverting existing
  in-progress work; new fixes must record red/green evidence when a narrow test seam exists.
- Added an asset regression test for a fully transparent splash corner. It passed immediately:
  although the PNG is palette-based and has no explicit alpha channel according to `file`, its
  transparency table decodes to alpha 0 at the corner. No asset change was needed.

### 2026-11-27 — Architecture and repository triage

- Read-only GitHub triage found zero open issues and one open PR (#19), the user's own draft and
  therefore outside the external-PR intake bucket. Existing labels do not include the triage
  workflow's state roles; no remote issue/PR state was changed.
- Wrote the evidence-backed candidate ranking and staged dependency plan in
  [`ARCHITECTURE-REVIEW-2026-11-27.md`](ARCHITECTURE-REVIEW-2026-11-27.md), linked it in the docs
  index, and added Phase 38 tasks for bitmap correctness, canvas/export contracts, byte-codec
  hardening, benchmark reliability, and measured coverage. The visual report is
  `/tmp/architecture-review-20261127.html`.
- Added a headless regression test proving Tab toggles canvas mode in both directions while the
  splash is visible. The test failed at its exact assertion when a temporary prior-style early
  return was injected, then passed after restoring the current source. Full tests now pass: 172
  unit/harness, 12 CLI, 4 hygiene, 10 MCP (198 total),
  including the previously noisy 500-object timing gate on this run. This single pass does not
  resolve its earlier contention-sensitive failures.
- Fresh coverage: 70.84% lines, 71.17% functions, 68.46% regions over 32,920 lines; this exceeds
  the 67.61% ratchet floor but remains below the requested 99% goal. The 2026-11-25 audit snapshot
  is unchanged.
- `cargo deny check`, `cargo audit`, and `cargo machete` exited 0. `cargo audit --json` reports 0
  vulnerabilities, 9 unsoundness advisories for transitive `heapless 0.8.0`, and 4 unmaintained
  crates. The chain is `geo 0.33.1 → rstar 0.12.2 → heapless 0.8.0`; the patched heapless series
  is not compatible with current rstar, so this is tracked in Phase 38.6 without a suppression.
- Atheon single-file scanning flagged many generic Rust false positives. Inspected high/critical
  examples and found no SQL client crate in the application manifest; the flagged source was
  generic identifiers/help strings. The broad scan was stopped before producing output.
  The managed Aegis scan profile refuses this checkout path because it is outside its workspace
  allowlist. Neither tool is represented as a clean security certification.

### 2026-11-27 — WASM fallback contrast triage

- The hook flagged two ignored generated WASM `index.html` artifacts. Their startup-error text is
  already white on `#282828`, but the canvas fallback text inherited the browser's default dark
  foreground, a real low-contrast case if canvas support is absent.
- Fixed the source template `app/index.html` with explicit `#f3f3f3` text and centered fallback
  layout. The contrast is 13.29:1. Impeccable's targeted detector reported no remaining issue in
  the source. The two ignored output snapshots remain historical generated artifacts; no
  suppression was added.

### 2026-11-27 — Canvas tools, cache identity, and validation

- Added Previous Tool (`Shift+Q`) with a state test and keyboard-event harness test.
- Replaced whole-object Eraser deletion with feathered bitmap-alpha editing and Properties size/
  feather controls. Dab, feather, non-square continuous drag, undo and freehand-not-snapped tests
  pass. The drag test found and fixed guide snapping moving input near the page origin; Brush,
  Pencil, Eraser, Freeform and Lasso now retain raw canvas coordinates.
- Made Live Filter parameter/color controls editable and undoable. Their visible renderer effect
  across object types/export is still open and is not claimed as complete.
- Fixed bitmap cache cross-wiring after tracing the index-based key and rejecting an allocation
  address key (the renderer clones the document each frame). Pixel-content hash plus dimensions
  now preserve cache identity across reorder/snapshot clones and invalidate on edit. Hashing cost
  and lifecycle/memory behavior remain to be benchmarked.
- Reviewed Emberframe (`64dcd718dd0308e6d771f55e3d96783574a5731c`, MIT) and BonBon/Libreworks
  (`8e11e2e0526e07abc23452ffc945c3e2716835f7`, GPL-3.0); the clean-room feature/algorithm report
  and phased canvas plan are linked from the documentation index.
- Verification: formatting, diff hygiene, documentation links/claims (`54` files), and strict
  Clippy pass. Tests pass with only the frame-time case excluded: 167 unit/harness, 12 CLI, 4
  hygiene and 10 MCP tests. The 500-object budget test passed alone, but two full-suite attempts
  were 125.2 ms and 120.7 ms against 120 ms under competing host work; this gate is unresolved,
  not waived. A 1360×860 demo screenshot was captured/reviewed at
  `/tmp/firecraft-canvas-audit.png`; interactive Star/Eraser/Filter screenshots and export proof
  remain open. No release or user sign-off is claimed.

## 2026-11-27 — Browser clipboard reproduction and architecture planning

- Reproduced the reported browser image-paste failure against the v0.3.1 WASM app in headed
  Chromium under Xvfb. A browser `ClipboardEvent` contained a real PNG file, but the async callback
  injected `egui::Event::Paste` outside the runner's active input cycle, so the editor reported an
  empty clipboard.
- Replaced that transient event injection with a frame-safe queue consumed by the editor before
  drawing. Rebuilt the locked WASM target with matching `wasm-bindgen` CLI 0.2.130 and repeated the
  real browser clipboard API + Ctrl+V flow. The PNG appeared as a selected 128×128 bitmap, the
  status bar reported a successful paste, and the page reported no JavaScript errors.
- This closes only the browser PNG happy path. Text/internal clipboard, denied permissions, large
  or malformed images, and native Windows/macOS/Linux clipboards still need separate evidence.
- Expanding `PRODUCT_ROADMAP.md` and `TASKS.md` before starting speculative product work to cover
  document history/version snapshots, PDF/page layout, web publishing, provider-based AI media,
  secure terminal/SSH workflows, 3D exploration, mobile/tablet, and the reference-project matrix.
  The external architecture skill's scan/report is being used as planning input; no broad refactor
  is being started ahead of the plan and review build.

## 2026-11-27 — FireCraft v0.3.1 release and Web deployments

- Published GitHub Release [`v0.3.1`](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/releases/tag/v0.3.1)
  with title exactly `v0.3.1`. The tag points to source SHA
  `3efe9c1c9c3bad6537c2038ecb5660946d56e028`; release workflow `45935587390` passed every job.
- Published and locally verified all eight platform packages: Windows x86_64 ZIP, macOS universal
  ZIP and DMG, Linux tar.gz/AppImage/DEB/Flatpak, and WASM ZIP. `SHA256SUMS.txt` passes for all
  eight. Full release manifest and source SHA match the tag.
- GitHub Pages deployment succeeded at
  [aliasfoxkde.github.io/FireCraft-CraftAnything-Experiment](https://aliasfoxkde.github.io/FireCraft-CraftAnything-Experiment/)
  with deployment source SHA `3efe9c1c9c3bad6537c2038ecb5660946d56e028`.
- Deployed the same WASM ZIP to Cloudflare Pages preview
  [wasm-review-v0-3-1.firecraft-web.pages.dev](https://wasm-review-v0-3-1.firecraft-web.pages.dev/).
  `BUILD-INFO.txt` reports version 0.3.1 and the release source SHA; index, JavaScript, and WASM
  return HTTP 200 with expected MIME types. This is a preview deployment; it does not replace the
  separate marketing site.
- Windows review ZIP:
  `output/release-v0.3.1/release-assets/firecraft-v0.3.1-windows-x86_64.zip`, SHA-256
  `aa8ae62835728ac88a493fd03d9d9ca7f0bb7800de31694175d4c621bedd3839`. Hosted Windows CLI smoke
  passed; native Windows 11 interactive acceptance remains for the reviewer.
- Open PR and issue counts were both zero after PRs #13, #14, and #15 merged. Branch hygiene is
  tracked in Phase 33. Final fetch/prune found local `main` plus remote `main` and
  `release/prepare-v0.3.1`; the three merged PR branches were deleted. GitHub's ruleset explicitly
  rejects deleting `release/*`, so that protected release branch is retained.

## 2026-11-27 — consolidation plan merged; v0.3.1 release prepared

- PR #14 merged as `62e004c` after its reconciled head `fc3aaf5` passed hosted Linux gates, WASM,
  Windows build/CLI smoke, coverage, and CodeQL. Experiment issues and open PRs now both count zero;
  the PR #13 feature branch and PR #14 planning branch were removed by squash merge.
- Full local `scripts/ci-local.sh` on the reconciled source passed formatting, strict Clippy, all
  **186 tests**, docs, module size, WASM check, `cargo deny`, `cargo audit`, and coverage. Coverage:
  **67.71% lines, 70.10% functions, 69.94% regions**, against the unchanged 67.61% line floor.
- `cargo audit` succeeded with 13 allowed warnings (unmaintained `ttf-parser`/`zbus` crates and
  unsound `heapless 0.8.0` via `geo → rstar`); `cargo deny` reports configured advisories/bans/
  licenses/sources gates passing. Context and dependency chains are in
  [`research/pattern-scan-triage.md`](research/pattern-scan-triage.md); this is not a security-clean
  claim.
- Prepared package version `0.3.1` and release notes for the editor/UI/WASM changes. The existing
  `v0.3.0` tag/release remains unchanged. Tag-driven multi-platform build and GitHub Pages deployment
  are pending on the clean release commit.
- Current Windows review archive from hosted source `fc3aaf5`:
  `output/windows-review-fc3aaf5/firecraft-v0.3.0-windows-x86_64.zip`, SHA-256
  `45a5b30f045937c416ca8d9d058be64cfa034eb683ccc0f913085df371d1c8ff`. Its CI Windows CLI smoke
  passed; interactive Win11 acceptance remains for the reviewer.
- PR #14 and the merged main now keep UI follow-up in Phase 32 and repository consolidation in
  Phase 33. Phase 34 tracks the new all-platform release separately from Windows acceptance.

## 2026-11-27 — PR #13 merged; roadmap reconciliation underway

- Merged PR #13 as squash commit [`00b1fd0`](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/commit/00b1fd0e12859a2a81ed014d898c5af46bf2ca8e).
  Exact-SHA hosted checks passed: Linux gates, coverage ratchet, WASM, Windows build/CLI smoke and
  ZIP artifact, CodeQL, and Analyze Rust. Coverage upload to GitHub Code Quality returned HTTP 404
  but is configured non-blocking; Cobertura artifact uploaded successfully.
- PR #14 carried the repository consolidation and legacy migration plan. Its `docs/TASKS.md`
  conflict was reconciled by retaining UI follow-up as Phase 32 and moving repository consolidation
  to Phase 33; PR #14 then passed fresh hosted checks and merged as `62e004c`.
- Main advanced through release-prep PR #15 to `3efe9c1`; PR/issue counts are both zero. Windows 11
  interactive acceptance and real browser clipboard-paste proof remain outstanding.

## 2026-11-26 — WASM startup diagnostics, paste tests, and hosted coverage recovery

- Retained the `eframe::WebRunner` for the page lifetime. Its startup handle owns browser
  observers/animation state; dropping it after `start()` could stop the app. Startup failures now
  write to the browser console and an accessible in-page error panel instead of being swallowed.
- Added behavior tests for valid and invalid bitmap paste inputs. Full native suite: **186 passed**
  (160 unit, 12 CLI, 4 hygiene, 10 MCP). Local ratchet coverage is **67.71% lines**, above the
  unchanged 67.61% floor. This remains far below the requested 99% target.
- Optimized WASM release build and target check pass. Browser clipboard write of a real PNG succeeded,
  but Playwright could not verify rendering or app paste: even a standalone ResizeObserver fixture
  failed to receive resize callbacks, screenshots timed out, and the deployed demo showed the same
  300×150 canvas backing size. Keep Phase 32.1 open; this host's browser automation is not valid
  runtime evidence.
- Hosted CI on prior source `0915d40` passed WASM, Windows build/CLI smoke/archive, CodeQL, and the
  separate Cobertura job. The Linux gate failed only because its ratchet read 67.58% against 67.61%;
  the new paste tests raise local ratchet coverage to 67.71%. A new push is required to verify the
  exact changeset in hosted CI and obtain a matching Windows package.
- Windows review archive from `0915d40` is available at
  `output/windows-review-0915/firecraft-v0.3.0-windows-x86_64.zip`, SHA-256
  `98dabd2325d4bdd46d24b4fb80e5aa81301f96ec6deafaf1681f6c5d4bfffbbe`. Hosted Windows smoke passed;
  interactive Windows 11 acceptance is still required.
- GitForge endpoints are responsive, but its local deployment is 273 commits behind upstream, its
  overall health is degraded, and only one runner is online. FireCraft has no GitForge remote, so no
  local GitForge pipeline result is claimed. Aegis is still unavailable because the managed
  `aegis-scan` profile is not installed. Supplemental Atheon source scan and its limits are recorded
  in [`research/pattern-scan-triage.md`](research/pattern-scan-triage.md).

## 2026-11-26 — Clipboard, workspace UI, and file-browser follow-up

- Added a browser `ClipboardEvent` bridge for copied image files. It reads image bytes asynchronously,
  releases each one-shot reader callback, and forwards the bytes into the normal Paste action. Edit
  and canvas context-menu Paste now remain available for external/system clipboard contents.
- Fixed Preview badge overflow by measuring rendered text and eliding it to the canvas width.
- Bound the file browser to the display, reduced its default dimensions, and virtualized the file
  list so only visible rows perform size/date metadata reads and paint. Added a 1,000-row render
  regression check.
- Restored the CS6 tool-family labels, constrained the tool-rail body on short windows, and rendered
  the Swatches panel as a compact, direct 2D grid. Replaced a stale text-outline test that expected
  a fake rectangle conversion with a truthful unsupported-operation assertion.
- Cleaned up the top menu/header arrangement against the checked-in Fireworks menu/layout images.
  Removed crowded MCP/CLI, fullscreen, and theme buttons from the menu row; their actions remain in
  Help/Window. The Palette button stays visible and a Demos dropdown sits immediately to its right,
  with the real Studio Showcase and 1080p starter document actions plus links to all four tutorials.
- Verification: native `cargo check` passed; format check and strict Clippy passed after correcting
  one needless borrow found by Clippy; WASM `cargo check` passed. Full test suite: 158 unit + 12 CLI
  + 4 hygiene + 10 MCP = **184 passed**. Xvfb captures at 1360×860 and 960×640 were visually
  inspected. The 960×640 workspace remains dense and scrolls lower right-dock groups; live browser
  clipboard paste and platform-native picker behavior are still unverified.
- Updated [`AUDIT-2026-11-26.md`](AUDIT-2026-11-26.md) and added Phase 32 to [`TASKS.md`](TASKS.md).
  Aegis scan remains pending: the local command is protected by managed admission, but the shared
  queue currently exposes no `aegis-scan` profile (only the separate Aegis-repository tests profile).

### 2026-11-26 — hosted gate follow-up and extension roadmap

- Audited the latest hosted experiment-repo run. Coverage workflow passed 180 tests (154 unit, 12 CLI, 4 hygiene, 10 MCP) and produced Cobertura at 68.24% lines (16,978/24,880); no branch counters were emitted. This does not meet the requested 99% target.
- Hosted Windows release build, CLI smoke, ZIP packaging, and the WASM release build passed on source `4bd1242`. Neither result proves Windows 11 interactive acceptance or a production deploy.
- Strict Clippy found a duplicated generic bound and two test modules above production items. Corrected those source issues, and made GitHub Code Quality upload failure non-blocking after its endpoint returned HTTP 404; the Cobertura artifact remains the required coverage receipt.
- Added Phase 31 to the task ledger for the extension API, local manager, public catalog, community submission, safe one-click installation, and author/user documentation. Historical desk research and the canonical informational Extensions page are complete; operational marketplace features remain future work.
- Current source-level formatting, workflow lint (with the documented actionlint permission-scope exception), and whitespace checks pass. Full CI still requires the module-size fix and final package/CodeQL results; release and deploy remain gated.
- Follow-up run `45870025403` confirmed strict Clippy, the 180 tests, formatting, docs validation, and WASM build pass. It found two file-size gate failures (`icons.rs` 2,193 lines and `panels/bodies.rs` 2,028 lines); moved only their test modules into adjacent files. Local `module-size.sh` now passes. The new extraction is pushed for another hosted check; Windows and CodeQL were still running.
- Follow-up coverage run `45870025885` reports 68.24% lines (16,979/24,881) and its GitHub Code Quality upload step succeeds. Cobertura has no branch counters, so branch coverage remains unmeasured.
- Full corrected run `45871239608` now passes Linux quality gates, all 180 tests, docs, module-size, Windows CLI/ZIP packaging, and WASM builds. CodeQL was still in progress at the last check. Its CI coverage ratchet measured 67.61% lines / 69.71% functions / 69.82% regions; its separate Cobertura artifact measured 68.05% lines and has no branch counters. Raised `coverage-floor.txt` from 44.40% to 67.61%; this exact new floor needs one hosted confirmation run.


## 2026-11-25 — Tasks 18.3, 18.7, 19.6: layout proof, frame budget, reproducible build

- **18.3**: headless Xvfb captures at 960×640 and 1360×860 with demo scene and
  all docks open; visually verified — nothing clipped, footer fits. Found and
  fixed a real defect: eframe's persisted window state overrode `--size` even
  with `persist_window: false` (that flag only stops *saving*, not *loading*).
  Screenshot mode now points `persistence_path` at a throwaway location
  (`main.rs`), so captures no longer depend on the operator's last interactive
  window size or on moving `~/.local/share/firecraft.editor/app.ron` aside.
- **18.7**: frame-time budget test
  `frame_time_with_500_objects_stays_within_budget_at_1920x1080` in
  `harness.rs`: 500-object frame ≈ 60 ms debug/software raster, hard 120 ms
  ceiling plus a 6× empty-baseline relative bound (the original 3× failed on
  a noisy micro-baseline — noted in the test doc comment). Startup/export/
  memory budgets remain release-receipt items, unverified on this host.
- **19.6**: two release builds with `src/main.rs` touched between them
  produced byte-identical binaries (SHA-256 `41bf162e…`). Caveat recorded in
  TASKS: same target dir, toolchain and host only.
- Gates: fmt clean, clippy `-D warnings` clean, `cargo test` 93+12+4+10 all
  passing; release build succeeded under the host build lease.

## 2026-11-25 — Task 17.5: FileService seam for headless file flows

- Added a `FileService` trait (`open`/`save`) in `app/files.rs` with
  `FormatFileService` as the disk-backed default, stored as
  `FireCraft.file_io`. Open, save, export, import and save-all-for-quit now
  route through the seam instead of calling `formats` directly, so headless
  tests inject a fake service and never touch the filesystem or egui. Two new
  tests: injected open/save round-trip (`Rc` handle asserts the saved path)
  and failing-service error surfacing. 10/10 files tests, 110 total pass.
- Repaired a pre-existing clippy break left by 17.3: the glob re-exports in
  `panels/mod.rs` (`use bodies::*`, `pub use tabs::*`, `use tabs::*`) had
  become dead/unresolvable under `-D warnings`. Replaced with explicit
  re-exports (`pub use bodies::{intelligence_panel, start_panel}` plus a
  named import list) and explicit helper imports in `color.rs`/`tabs.rs`.
- Gates: fmt clean, clippy `-D warnings` clean, `cargo test` 110 passed /
  0 failed, wasm32 check exits 0, module-size budget under the hard cap.

## 2026-11-25 — Task 17.3: panel bodies behind the PANELS registry

- Extracted the remaining panel render bodies from `panels/mod.rs` (3635 ->
  1388 lines) into `panels/bodies.rs` (1912 lines: Optimize, History, Align,
  Path, Styles, Swatches, Info, Pages, Intelligence, Library, Start, and the
  post-registry bodies) and `panels/tabs.rs` (355 lines: Pages/States/Layers
  and related tab wrappers). `mod.rs` now holds the chrome, the `PANELS`
  descriptor table and re-exports; every renderer is reached through the
  registry seam. Pure code motion; fmt, clippy `-D warnings`, tests, wasm
  check and module-size budget pass.

Newest first.

## 2026-11-25 — Task 17.2: extract layout/chrome.rs

- Extracted the document tab strip and canvas footer bar (with their helpers
  `tab_click_without_drag`, `compact_tab_label`, `footer_icon_button`) plus the
  bottom properties dock body from `layout.rs` into `layout/chrome.rs` (815
  lines) behind `pub(super)` visibility. `layout.rs` is 2850 -> 2041 lines.
  Pure code motion; fmt, clippy `-D warnings`, tests and wasm check pass.

Newest first.

## 2026-11-25 — Task 17.1: extract command modules from app.rs

- Extracted `app/src/app/edit.rs` (undo/redo, delete/duplicate/group, clipboard,
  selection sets; 700 lines), `app/src/app/arrange.rs` (align/distribute/space,
  match size, flip/rotate/scale, z-order; 667 lines) and `app/src/app/object.rs`
  (paths/booleans, symbols, styles, auto-shapes, slices, filters, flatten,
  saved commands; 1068 lines) from `app.rs` behind the existing child-module
  pattern (`use super::*;` + `impl FireCraft`). `polygon_boolean`/`polygon_area`
  moved to module scope in `object.rs` with `pub(super)` visibility for the
  property tests. `app.rs` is 7368 -> 4952 lines. Gates: fmt, clippy
  `-D warnings`, tests, wasm check, module-size budget all pass. Behavior is
  unchanged - pure code motion; existing tests cover the seams.

Newest first.

## 2026-11-25 — 0.3.0 quality and release pass

Worked the open task list end to end; decisions D1–D5 recorded.

- **Truth and hygiene:** MIT `LICENSE`; `app/Cargo.toml` is the only version source (0.3.0) and
  `--version`/help/MCP no longer hardcode a version; `rust-toolchain.toml`; crate self-contained
  (`assets/logo.png`, `assets/splash.png`); one WASM host page; `PROJECT_STRUCTURE.md`,
  `build-release.md` and `IMPROVEMENTS.md` rewritten; `scripts/check-docs.sh` gates links, claims,
  trailing whitespace and duplicate headings.
- **Gates:** `deny.toml` (advisory acceptance for `url 2.5.8` with exposure note), `cargo audit`,
  coverage ratchet (`scripts/coverage.sh`, floor 18.70), `[lints]` policy, module-size budget,
  `scripts/ci-local.sh` as the definition of record, Windows build + CLI smoke job, WASM build job,
  Dependabot, issue/PR templates.
- **Tests:** 108 total (was 51). New integration suites `tests/cli.rs` (12), `tests/mcp.rs` (10),
  `tests/hygiene.rs` (4); formats error paths, golden SVG/HTML/PNG files, theme WCAG contrast,
  color round-trip sweep, file-intent matrix, polygon-boolean area identities, quarter-turn bounds.
- **Bugs found and fixed by those tests:** ICO export failed for any page above 256 px; `batch`
  created a missing input directory and reported success; `Escape` was consumed by the pen-path
  check even with no pen path, so it never closed menus/dialogs; `Shift+Tab` could never reach the
  dock toggle because the unmodified `Tab` check consumed it first; light-theme accent contrast was
  4.15:1 (now 5.2:1); polygon booleans silently dropped every result region except the largest.
- **Architecture:** `panels/geometry.rs` (pure snap/stack/height math, 7 tests) with `app.rs`
  delegating; `harness.rs` boots the real frame headless at 1360×860 and 960×640 and drives keys.
- **GUI:** dock header now reads "Panels", Mixer group tall enough for its controls, scrollbars
  always visible on panel bodies, Bridge `Br` text replaced by a folder icon, header cluster spacing
  fixed at 960 px. Evidence: `checks/gui-pass-2026-11-25/g5-*.png`.
- **Release:** `scripts/package-windows.sh` builds `dist/windows/` + versioned zip + SHA-256;
  tag/version mismatch fails the release job; Wine smoke-tested the GNU binary.

## 2026-11-26 — 18.8 defect burn-down and ROADMAP 4.5 closure

- 18.8 disposition: the defects surfaced during Phase 16 verification were
  test-premise bugs, each root-caused and fixed in place (Iconic Mode gate,
  stale replace_color, snap threshold, zoom-clamp status). No silent fixes.
- ROADMAP 4.5 closed: two new harness tests. Styles — preset fill/stroke/
  effects on single and multi-selection, undo restores per-object baselines
  (demo objects carry baseline live effects). Filters — all five LiveEffect
  kinds added, toggled off/on, one removed, all cleared, repeat-last re-adds,
  every step asserted across the whole multi-selection.
- Task-list reconciliation: 16.3 marked done (test landed with the 16.8/G-5
  batch), ROADMAP 2.6 closed by the 16.4 gesture test, 3.3 by the 18.4
  Intelligence tests, 11.5 by the 18.5 disposition. ROADMAP 8.3/9.4 moved to
  `[~]` blocked — no Windows host in this environment (decision D4); the
  full-screen toggle itself is code-verified in harness tests.
- Gates: `cargo test --bin firecraft` 104 passed; fmt and clippy
  `-D warnings` clean.

## 2026-11-26 — Panel suite, screenshot loop, interactive-state capture (16.3/16.8/G-5)

- 16.3: `panel_states_paint_collapse_tab_overflow_iconic_and_inspector_clamp`
  in `app/src/harness.rs` — dock collapse/expand paint delta, last-tab
  overflow, iconic strip vs flyout, bottom-inspector clamp at 5000/0,
  collapsed-bottom paint delta.
- 16.8: `scripts/gui-suite.sh` replaces the manual per-round `gui-shot.sh`
  loop — one invocation captures the six-state evidence set into `checks/`.
- G-5: new `--state <menu|flyout|find-replace>` flag (main.rs) pre-arranges
  the app so interactive states are reachable from the capture path.
  Evidence: `checks/gui-suite-2026-11-26/` (visually verified: File menu
  open, iconic flyout expanded, find-replace dialog). Drag/stack gestures
  stay code-asserted in harness.rs, not pixel-captured.

## 2026-11-26 — Harness interaction tests (16.2/16.4/16.6/16.7)

New tests in `app/src/harness.rs`, all driving the real headless frame:

- `workspace_shell_states_paint_at_the_minimum_size`: startup shell at both
  sizes, splash, Tab key, Alt+F menu open, iconic flyout (Iconic Mode preset),
  footer/status.
- `panel_gestures_float_merge_stack_detach_snap_and_close`: ungroup → float
  (multi-tab source keeps its id, new floating group gets a fresh one), merge,
  stack, detach, edge snap within the 12 px threshold, close-last no-panic.
- `every_menu_row_triggers_and_closes_the_menu`: walks all 10 menus, Enter
  triggers each enabled row, Escape closes; selection set so Filters/Modify/Text
  rows are enabled.
- Find/replace: text mode cycles matches and replaces name+text slots (4
  occurrences across 2 objects); color mode covers the no-match report and the
  real swap (`replace_color` must differ from `find_color` or the run is a
  no-op by design). Dialog toggles frame without panicking.

Gates: `cargo test --bin firecraft` 101 passed; `cargo fmt --check` and
`cargo clippy --all-targets -- -D warnings` clean. Not verified on screen —
headless frame evidence only.

## 2026-11-25 — GUI hill-climb and architecture slice

Four capture-and-fix iterations against real renders (`scripts/gui-shot.sh`, Xvfb + software GL,
1360×860 and 960×640). Evidence: `checks/gui-pass-2026-11-25/`.

- Fixed, each verified by a fresh screenshot: radial gradient bleed and unpainted corners; slice URL
  label covering the CTA label; showcase not fitting at 960 px; selection handles on first launch;
  inspector second row clipped at 118 px; rail hint cut mid-sentence; Matte control overlapping its
  label; CTA label off-centre.
- Architecture slice: extracted `demo::showcase(w, h) -> Vec<Obj>` out of `app.rs` (280 lines) and
  added four layout tests. The tests immediately found a real defect the screenshots could not show:
  multi-line text bounds counted newline glyphs, so feature bodies measured wider than their cards.
  `doc::text_width` and `Text` bounds now use the longest line and the line count.
- Added `--size WxH` (deterministic review at any window size) and one regression test for
  multi-line text bounds.
- Architecture review report (skill `improve-codebase-architecture`) written outside the repo:
  `/tmp/architecture-review-20261125.html`. Top candidate implemented; candidates 2–4 recorded in
  the report and in `docs/PLAN.md` Phase 17.
- Gates: `cargo fmt --check` clean, `cargo clippy --all-targets -- -D warnings` clean,
  `cargo test` = **51 passed** (was 46).

### Unverified

- Interactive states (menus open, tool flyouts, drag/stack gestures) are still not captured; the
  capture path renders the startup frame and exits.
- Right-dock empty header strip, Color Mixer clipping and the 960 px top-right cluster remain open
  and are listed in `docs/TASKS.md`.
- Windows-only rendering not exercised. One entry per work chunk. Record what changed, the gate results, and anything left
unverified. Do not restate the plan; link it.

## 2026-11-25 — Audit and plan baseline

- Recovered state after the lost session: working tree clean at `3133c87`, branch `main` in sync
  with `origin/main`.
- Ran the full gate set and measured rather than assumed: fmt/clippy/test green (46 tests), WASM
  `check` green, `cargo machete` clean.
- Measured coverage for the first time: **18.70 % lines / 22.87 % functions**; the entire UI layer
  (`layout.rs`, `menus.rs`, `dialogs/*`, `panels/color.rs`, `panels/properties.rs`, `icons.rs`)
  reports 0 %.
- `cargo audit`: 1 vulnerability (`url 2.5.8`, RUSTSEC-2026-0592, via `eframe → webbrowser`) and
  13 warnings (9 × `heapless` unsound via `geo → rstar`, 4 unmaintained).
- `cargo clippy -W clippy::pedantic`: 829 warnings, no lint policy in `Cargo.toml`.
- Confirmed the docs drift: stale `app/PROJECT_STRUCTURE.md`, `app/MECHANICS.md` contradicting
  `mechanics.json`, no `LICENSE`, version `0.2.0` in Cargo vs `0.2.1` in CHANGELOG, duplicated WASM
  entry point, splash asset read from outside the crate.
- Wrote [AUDIT.md](AUDIT.md), [PLAN.md](PLAN.md), [TASKS.md](TASKS.md) and `CLAUDE.md`; corrected
  `app/PROJECT_STRUCTURE.md` and `app/MECHANICS.md`; pointed `ROADMAP.md` and the docs index at the
  new task list.
- Not changed: no production code touched in this chunk. Decisions D1–D5 are open and blocking
  Phases 13 and 19.

### Unverified

- All GUI behavior: no display available in this environment. Panel drag/stack/snap, menu
  click-through, 960×640 layout and the Intelligence endpoint still need Phase 16's harness.
- Windows-only behavior (Jump List, app-switch flash) — no Windows shell here.

## 2026-11-25 — UI rework per owner feedback + tutorials

- Tools palette: CS6-style two-column icon grid with tinted category bands, compact
  fill/stroke swatch footer (overlapped chips, swap/default/no-fill/full-screen buttons);
  removed meta counts footer. `panels/tools.rs`.
- Properties/panel-group collapse: chevron click now works on first click — the header's
  `Sense::click_and_drag` was shadowing the chevron `ui.interact`; detection moved to
  header `interact_pointer_pos()` against the chevron rect. `panels/mod.rs`.
- Right dock: expand/collapse-all now two 18 px toggle buttons inside the dock header bar;
  removed the separate full-width button row. `layout.rs`.
- New `docs/tutorials/` (getting-started, keybindings, drawing-and-editing,
  pages-and-states, optimize-and-export) linked from `docs/README.md`. Publishing to the
  GitHub Wiki awaits owner approval.
- Gates: fmt clean; clippy -D warnings exit 0; test suite 110 passed / 0 failed;
  wasm check exit 0. Screenshot /tmp/fc-final.png verified under Xvfb. Interactions
  (click collapse, flyouts) remain unverified — screenshot only.

### 18.1 IMPROVEMENTS triage (same day)

- Verified in source: Alt+letter menu mnemonics dispatch OpenMenu; ArrowUp/Down set
  nav_delta, Left/Right switch menus, Enter/Space activate (menus.rs). Tab (no mods)
  maps to ToggleFullScreenCanvas (layout.rs). Align and Path panels are complete CS6
  icon-row implementations (panels/bodies.rs).
- IMPROVEMENTS.md updated: I-06, I-07 moved to Fixed; I-08 split (collapse part fixed,
  drag/snap gesture stays with 16.3/16.4); I-09 folded into G-1/18.3 polish.
- Unverified: actual keypress/click behavior needs the Phase 16 interactive harness or
  a display; code paths only.

### 18.2 / 18.4 (2026-11-25)

- 18.2 Windows behaviors: this host is Linux with no Windows shell or VM; Jump List,
  app-switch flash and taskbar identity remain **unverified** (code paths exist in
  shell_windows.rs). Recorded as an honest limitation, not closed.
- 18.4 Intelligence panel paths: seven new tests in app.rs (missing key / missing model /
  empty prompt rejected with status and no channel; success sets response and clears the
  channel; error surfaces status with no fabricated response; worker disconnect reported;
  in-flight request blocks a second start; ai_extract_content parses provider JSON and
  rejects error bodies). Env-var dependency documented: tests never mutate the process
  environment (`-D unsafe-code`); the no-key test skips if the ambient key is set.
- 118 tests pass (92 unit + 12 doc + 4 cli + 10 mcp), 0 failed.

### 18.6 Accessibility checklist (same day)

- New docs/A11Y.md: per-area status with evidence. WCAG AA contrast is machine-tested
  in theme.rs for both themes; Alt+mnemonic + arrow menu operation exists in
  layout.rs/menus.rs (code-verified, not screen-verified); WASM focus visuals,
  prefers-reduced-motion and screen readers recorded as limitations with the
  concrete path to close each.
### 2026-11-26 — Documentation and GitHub community organization

- Moved the generated changelog and GitHub community health documents to `.github/`; moved the
  product brief, glossary, improvement history, and completed UI roadmap into `docs/`.
- Kept the existing root `AGENTS.md` and linked it from README/docs navigation rather than creating
  a duplicate. Updated the package assembly scripts so release archives still contain user-facing
  `CHANGELOG.md` and `GLOSSARY.md` files at their established package paths.
- Added GitHub issue-routing links, a Contributor Covenant Code of Conduct, and CODEOWNERS. Existing
  bug, feature, and pull-request templates remain the supported submission forms.
- Updated cross-document links and the documentation checker for the new file locations.
- Verification: `./scripts/ci-local.sh --quick`, `./scripts/check-docs.sh`, `git diff --check`, and
  `bash -n` on the three packaging scripts passed. Before this update, PR #7's current head had
  passing hosted gates, coverage, Windows build/CLI smoke, WASM, and CodeQL; the pushed doc change
  will produce a fresh receipt for the new head. `actionlint` still reports the known
  `code-quality: write` permission scope in the coverage workflow, which GitHub currently accepts.

### 2026-11-27 — v0.3.0 release and version-only release title

- Published [v0.3.0](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/releases/tag/v0.3.0)
  from source `e415a333f6a474bd738f47bb64d56704d711f1ce` with Windows, universal macOS ZIP/DMG,
  Linux tar.gz/AppImage/DEB/Flatpak, and WASM ZIP assets. The release manifest and aggregate SHA-256
  verification cover the exact platform asset set.
- All release workflow jobs passed on run #81, attempt 3. The WASM site is deployed at
  https://aliasfoxkde.github.io/FireCraft-CraftAnything-Experiment/.
- Corrected the published release name to exactly `v0.3.0`; updated the release workflow to use the
  tag as the release name for future releases. Verified the published release assets against
  `SHA256SUMS.txt` and refreshed the local Windows review bundle under `dist/windows-review/v0.3.0/`.

### 2026-11-27 — Bitmap texture lifecycle TDD and coverage refresh

- Added renderer-level regressions in merged PRs [#26](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/pull/26)
  and [#27](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/pull/27). They associate
  rendered bitmap bounds with texture IDs across hotkey reorder, duplicate, undo/redo, delete/add,
  and flattening selected bitmaps into a composite. The existing production cache fix was already on
  `main`; the new tests were not run against the pre-fix revision, so no new historical red run is
  claimed.
- Both PRs passed the full hosted gates, including strict Rust gates, coverage, Windows CLI/build,
  WASM build, and CodeQL. Local validation on PR #27's head passed 178 unit tests, with one isolated
  timing test intentionally ignored in the full-suite command; strict Clippy, formatting, and
  whitespace checks passed. After adding the flatten case, the full suite passed 179 tests with one
  isolated timing test ignored; strict Clippy, formatting, and the 60-file documentation/link check
  passed. The added flatten regression also passed in isolation.
- Retrieved the exact Cobertura artifact from hosted coverage run [45984084090](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/actions/runs/45984084090):
  17,934 / 25,895 lines = **69.26%**. The report has no branch counters (0/0). This is below the
  requested 99% and differs in denominator from the older llvm-cov summary; the discrepancy remains
  open in TASKS 38.5/39.6.
- Aegis is installed locally as version 0.6.6, but this session's command hook requires managed
  admission and the available job manager has no `aegis-scan` profile. Its `aegis-workspace-tests`
  profile only runs Aegis's Rust tests, not a FireCraft scan. No Aegis findings or clean result are
  claimed. The dependency/advisory, cache hashing-cost, broader document/page lifecycle, browser
  WebGL2/clipboard, native Windows GUI, and 99% coverage work remain open.
- Local GitForge health is now checked directly: API, CI/scheduler, and Git HTTP each returned HTTP
  200; the API health response reported a connected database. Authenticated runner listing returned
  401 (`missing_token`), neither `GITFORGE_TOKEN` nor `GITFORGE_API_TOKEN` is set, and FireCraft has
  only its GitHub `origin`, so no local FireCraft pipeline result is claimed. The GitForge checkout
  is itself on `main` ahead of `origin/main` by two commits with a large dirty worktree; it was left
  untouched.
- 2026-11-27 release-candidate follow-up: local GitForge pipeline `dba842d5-0eaf-428d-86ab-b5e1b7041876`
  completed successfully for the offline CI-image branch; job `9d7f2bdd-457f-4f5d-859e-fd7cfd49d071`
  finished at 13:58:31 UTC, including the WASM release build. The exact v0.3.5 lockfile needs a new
  image: image 4 was built as `sha256:d03a844a27237b333ec1b9bc9d1a8989d822227d8d6bc2c57894c8ad755887d8`,
  label `76d7fb62c41c6939bc86debc39b2f7270a3f2e98f744e570db4bee1f89201432`. Offline gates passed
  on the v0.3.5 candidate with 188 unit, 12 CLI, 5 hygiene, 10 MCP and 1 isolated performance test;
  strict Clippy, formatting, 61-file docs, module-size, and release WASM build passed. The docs gate
  had silently accepted an empty file set when Git failed; a red/green hygiene regression now makes
  that condition fail closed and handles Git's safe-directory check inside the container. The exact
  release branch GitForge pipeline and hosted PR checks are still pending.

### 2026-11-27 — v0.3.5 release, audit receipts, and marketing follow-up

- Merged PR #39 and published [GitHub Release `v0.3.5`](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/releases/tag/v0.3.5)
  from `bec0b65f3d05f66d299568f0ea80227e049daee4`. Release workflow
  [46018626991](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/actions/runs/46018626991)
  passed all platform/package, aggregate, release, and GitHub Pages jobs. The Windows review ZIP is
  at `output/release-v0.3.5/firecraft-v0.3.5-windows-x86_64.zip` with SHA-256
  `4cfdad8cc68133fa448d2cd424846a25612447bc25e0b509fe4167b300ed0868`. This is a build and CLI
  smoke receipt; native Windows 11 editing acceptance remains open.
- Downloaded the exact merged-main Cobertura artifact from
  [coverage run 46018564233](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/actions/runs/46018564233):
  18,425 / 26,445 lines = **69.67%**. Branch counters are 0/0; the requested 99% is not met.
- The v0.3.5 Cloudflare WASM preview serves the exact release `BUILD-INFO`, JS, WASM, and favicon
  with matching checksums and MIME types. Chromium loaded the page shell, but WebGL2 canvas startup /
  screenshot capture timed out in this host. Editor interaction and clipboard are not claimed as
  verified.
- Managed Aegis 0.6.6 production scan `4885e72a-64a2-490a-a97b-c27b72ac3b7f` matched the exact
  source tree. It reported 11,315 findings, of which 11,241 came from retained `investigation.cany/`
  evidence/journals. Focused product/script alerts were reviewed as likely scanner false positives
  or intended generated/input behavior; the full evidence set was not exhaustively triaged. The
  scan is **not clean** and task 41.2 stays open. A redacted aggregate is stored at
  [`.audit/aegis-v0.3.5-summary.json`](../.audit/aegis-v0.3.5-summary.json); raw findings were not
  committed.
- Added [the dated audit](AUDIT-2026-11-27.md), corrected the task board release statuses, and linked
  the audit from README and the docs map. The architecture review report
  `/tmp/architecture-review-20261127.html` ranks a canvas interaction policy seam first; its
  implementation awaits maintainer selection.
- In sibling marketing repo PR [#190](https://github.com/aliasfoxkde/FireCraft/pull/190), the
  canonical release links, System/Dark/Light toggle, cursor-following hero light, and planned
  extensions information page are covered by eight Node tests. The marketing validation job and
  `actionlint` passed; Rust-hosted checks were still pending at the time of this entry. Marketing
  Pages deployment remains open until checks, project provisioning, and route verification pass.
