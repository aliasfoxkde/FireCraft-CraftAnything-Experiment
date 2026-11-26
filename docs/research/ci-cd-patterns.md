# CI/CD patterns and recommended adoption plan

> **Status:** audit and recommendations, not a claim that these gates already run here.
> **Reviewed:** 2026-11-26
> **Scope:** `FireCraft-CraftAnything-Experiment` workflow files and the adjacent `/nas/Temp/repos/FireCraft` repository as a local reference.

## Summary

The experiment repository has a useful single local gate script (`scripts/ci-local.sh`), a Linux quality job, Windows release build/CLI smoke, a WASM build, and a tag-triggered release/deploy workflow. Its main risks are that GitHub Actions use mutable version tags, native Windows is built in both CI and the release workflow without a single canonical artifact path, the release workflow publishes after only checksum/version checks, and the local gate silently skips some optional tools when absent. There is no workflow-level change classifier, Aegis gate, test/coverage artifact upload, or dedicated browser E2E/accessibility job visible in the current workflow files.

The sibling `/nas/Temp/repos/FireCraft` has useful patterns to adapt: SHA-pinned Actions checked by a self-test; least-privilege job permissions; explicit change classification that fails closed; Aegis on documentation-only changes; tests of release helper scripts; artifact allowlists and aggregate validation; manifests, checksums, SBOM and provenance; independent deploy workflow with narrow path triggers; and explicit GitForge pipeline validation. Its design is more extensive and has operational caveats of its own. Copy the principles and evidence model, not the full pipeline or assumptions about a different GitForge runner.

## Current experiment-repository inventory

Reviewed files: `.github/workflows/ci.yml`, `.github/workflows/release.yml`, `.gitforge.yml`, `scripts/ci-local.sh`, `scripts/coverage.sh`, `scripts/check-docs.sh`, `scripts/module-size.sh`, `scripts/package-windows.sh`, and `docs/build-release.md`.

| Area | Current implementation | Gap / risk |
|---|---|---|
| Triggers | CI runs on pushes and PRs targeting `main`; release workflow runs on `main` pushes, `v*` tags, and manual dispatch. | Release/deploy work is also invoked on every main push, increasing duplicate packaging and requiring strong guards around publish/deploy steps. |
| Actions/toolchains | `actions/checkout@v4`, `rust-toolchain@1.100.0`, `rust-cache@v2`, `install-action@v2`, upload-artifact `@v4`; toolchain has `rust-toolchain.toml`. | Action tags move; no pin validator is present. Toolchain itself is pinned. Installed helper tool versions should be pinned explicitly, not assumed from action defaults. |
| Quality | `ci-local.sh`: fmt, clippy `-D warnings`, unit/integration tests, docs checks, module-size check, WASM target check, `cargo deny`, `cargo audit`, coverage ratchet. | `cargo deny`, `cargo audit`, and coverage are conditionally skipped if missing. CI currently installs them, but the same script can report success outside that provisioned environment without those gates. |
| Coverage | `coverage.sh` checks aggregate line coverage against `coverage-floor.txt`; the recorded current floor is low relative to the aspirational 99% target. | No report upload or per-module trend/evidence appears in the workflow. Aggregate line coverage does not measure docs completeness, branch coverage, UI/e2e breadth, or product behavior. |
| Platform builds | Separate Windows release build plus CLI smoke and ZIP artifact; separate WASM release build and wasm-bindgen output. Release workflow also rebuilds Windows and WASM. | Duplicated build paths may diverge; CI Windows packaging and release packaging should consume the same verified build recipe. No real Win11 desktop GUI smoke is established by a compile/CLI check. |
| Release | Tag-matched version, Windows and WASM artifacts, SHA-256 list/check, then `gh release upload/create`. | No fail-closed exact artifact inventory, SBOM, provenance/attestation, signing, draft-review boundary, or external artifact re-hash is apparent in this file. `||` fallback around create/upload should have tests for partial/failed states. |
| Deploy | `actions/deploy-pages@v4` deploys WASM after build on the `main` push path. | No separate static marketing-site workflow is present in this repository. Pages deployment should be isolated from release permissions and guarded by content validation/smoke. |
| GitForge | `.gitforge.yml` uses GitForge's versioned `name` / `version` / `trigger_on` / `environment` / `jobs` schema and runs `ci-local.sh --quick` plus a WASM build. | GitForge does not accept GitLab-style `stages` / `before_script` syntax. The pipeline is a single gate job and skips slower coverage/security checks; a local runner receipt is still required before claiming it ran successfully. |
| Diagnostics | GitHub workflows are the only visible Actions definitions in this repository. | No explicit accessibility/E2E browser workflow, Aegis/pattern scan, workflow pin self-test, or PR artifact review workflow is present. |

This is a source review of workflow definitions, not evidence that a current GitHub or GitForge run succeeds. Markdown relative links are checked by `scripts/check_docs.py`; actual GitForge execution still requires a current runner receipt. Record run IDs and failed-step logs in the task ledger separately.

## Transferable patterns from the sibling FireCraft repository

The sibling's current files include `.github/workflows/ci.yml`, `release.yml`, `deploy-site.yml`, `.gitforge.yml`, `scripts/validate_workflow_pins.py`, `scripts/validate_gitforge_pipeline.py`, `scripts/validate_release_aggregate.sh`, `scripts/classify_release_mode.sh`, `scripts/install-aegis.sh`, and self-tests around those scripts. Its records are more elaborate; the strongest directly transferable practices are:

1. **Pin third-party actions to full commit SHAs and validate the pins in CI.** Keep the human-readable version comment, and make an automated checker reject tag-only `uses:` references. Update pins through a reviewed, tested dependency PR.
2. **Least privilege by default.** Set read-only workflow permissions, then widen only the release or deployment job that needs write/OIDC permission. Keep secrets out of PR code paths.
3. **Fail closed on unknown change classification.** A docs-only fast path must preserve required security/doc checks; missing base SHA or empty diff must run the full path. Never path-filter away a required check and leave branch protection permanently pending.
4. **Scan documentation too.** The sibling explains that Aegis covers docs and therefore runs for prose-only changes as well. For this repo, add Aegis/pattern scanning to the common required check and make “scanner missing” fail CI rather than silently skip.
5. **Exercise gate scripts.** Add self-tests for release mode classification, package allowlists, checksum verification, version/tag mismatch, and failure cleanup. A test of the happy path alone misses the dangerous partial-release paths.
6. **Build once, promote the exact bytes.** Release aggregation should validate exact filenames, versions, target triples, source SHA, non-empty artifacts, hashes, and expected count before draft publication. The publish job should not rebuild or accept stale files from a reused directory.
7. **Give releases a verifiable inventory.** Produce `SHA256SUMS`, machine-readable manifest, SBOM, and build provenance; verify each against the bytes before upload. Sign/attest only after setting the necessary permissions and verifying the provider trust model.
8. **Keep website deploys independent.** Trigger on deployed content/config changes or an explicit redeploy; validate required files, metadata, reduced-motion behavior and links before deployment. Serialize production deployments and use minimal token scope.
9. **Validate GitForge syntax and operational assumptions offline.** The sibling has a parser/check script and documents that its GitForge runner has constraints distinct from GitHub Actions. This repo should validate the exact local pipeline schema before relying on a server run, then record server health/build drift and run receipt.
10. **Treat caches as an optimization, never a correctness dependency.** Cache keys must include OS/toolchain/lockfile where appropriate; stale or shared build trees must not let one branch reuse artifacts from another tree.

The sibling uses different workspace structure, toolchain versions, package matrix, and CI mechanics. Its coverage/toolchain history includes compiler-specific differences, and its GitForge workflow assumes a particular host cache/runner implementation. Do not import its numbers, deployment configuration, `xtask` commands, or runner image into this repository without a local measurement and compatibility check.

## Ordered adoption plan

### Phase A — Establish trustworthy green checks

1. Capture current `main` SHA, required branch-protection checks, latest GitHub run IDs, open PR status, and GitForge health/runner build identity in `docs/AUDIT.md` or a dated report.
2. Make `scripts/ci-local.sh` strict in CI: require `cargo deny`, `cargo audit`, coverage tools and Aegis/pattern scan when the CI profile says they are mandatory; preserve a clearly labeled developer `--quick` profile.
3. Add a lightweight workflow static-check job that validates Action SHA pins, shell syntax, workflow YAML, `.gitforge.yml` structure, and unresolved conflict markers.
4. Add Aegis/pattern scan to CI for code and documentation changes. Baseline findings by content and document reviewed exceptions; do not disable rule classes to get green.
5. Make the required-check set stable: one always-present aggregate check that depends on all applicable platform, quality, and security jobs. Missing/unknown classifier results take the full-check path.

**Exit evidence:** clean rerun on the current head, all required checks present and successful, explicit logs/artifacts, no silently skipped mandatory gate, GitForge pipeline parsed and run on a current healthy worker.

### Phase B — Increase test and accessibility evidence

1. Split fast static/test checks from slower native/WASM/browser lanes, with job boundaries that produce actionable failures.
2. Add coverage report output (LCOV or Cobertura plus summary) as a CI artifact; track line and branch coverage separately where instrumentation supports it.
3. Keep coverage thresholds per meaningful crate/module and raise them incrementally from measured current values. Add tests for file parsing, persistence/recovery, command dispatch, UI interactions, and failures before raising the threshold.
4. Add browser/WASM end-to-end smoke covering launch, document create/open/save/export, keyboard focus, clipboard, dialogs, and errors. Add keyboard-only and screen-reader checks where the UI stack can expose semantics.
5. Establish WCAG 2.1 AAA checks only where criteria are applicable and measurable: contrast tokens, text resizing/zoom, keyboard focus and operation, reduced motion, labels, error identification and browser accessibility inspection. Record exceptions and native desktop limitations; do not call a theme contrast unit test “AAA compliance.”
6. Add platform-native smoke receipts: Windows 11 launch/input/clipboard/save, supported Linux launch and browser engines. A cross-compiled exe or Wine CLI run is not a Windows GUI receipt.

**Exit evidence:** uploaded coverage artifact tied to commit SHA, documented coverage denominator/toolchain, repeatable E2E reports, accessibility findings closed or explicitly tracked, real platform receipt per claimed target.

### Phase C — Make release promotion deterministic and reviewable

1. Separate PR CI, release artifact production, GitHub publication, Pages deploy, and marketing-site deploy permissions/jobs.
2. Have release builds consume an immutable tag SHA and build each target once. Ensure `main` pushes do not publish release assets.
3. Build the Windows package and WASM/site output in clean workspaces; validate package contents against an explicit allowlist and reject missing, extra, stale, empty, wrong-version or wrong-SHA artifacts.
4. Generate SHA-256 checksums and a JSON manifest containing version, source SHA, target/platform, byte size, digest and verification status. Verify all values before upload and again after download/assembly.
5. Generate a CycloneDX SBOM from the lockfile/dependency graph. Add SLSA/in-toto provenance or GitHub artifact attestations once the permissions and identity verification are configured.
6. Publish a draft release first; require a successful aggregate gate and human review before public release. Make rehearsal/test releases explicit and impossible to confuse with production.
7. Add release self-tests for failure and retry behavior, including pre-existing draft, partially uploaded assets, checksum mismatch, malformed version/tag, stale `dist`, absent SBOM/manifest, and cancellation.
8. Add post-deploy verification: fetch public website/WASM URLs, compare served hashes to the approved artifact manifest (account for known platform HTML transformations), and store a deployment receipt.

**Exit evidence:** same-tag end-to-end rehearsal from clean checkout, test artifacts install/launch on claimed targets, all manifest/checksum/SBOM/provenance validators pass, draft visible with exact inventory, post-deploy byte/health report.

### Phase D — Reduce CI cost without weakening proof

1. Measure job wall-clock and queue time before introducing matrices or caches.
2. Use lockfile/toolchain/OS-scoped caches; never share mutable Cargo target outputs between concurrent branches unless the cache system provides isolation.
3. Split platform jobs only where that improves failure localization or parallelism; keep one stable aggregate required check.
4. Cache installable tools by pinned version and verify checksum/signature on installation.
5. Keep path-based skips explicit and conservative. Any skipped job must have a deterministic classification result and must not bypass Aegis, workflow validation, or release ledger checks.
6. Add timeouts from measured p95/p99 run durations, not arbitrary low values; distinguish timeouts from gate failures in the report.

**Exit evidence:** before/after CI duration and cache-hit figures on comparable runs; no missing required status; cold-cache run still succeeds.

## Ongoing task ledger rows to add

Use these as small atomic tasks in `docs/TASKS.md` or GitHub issues. They are recommendations, not completed work.

| ID | Task | Dependency | Done when |
|---|---|---|---|
| CI-01 | Capture current workflow/run/branch-protection baseline | none | Dated evidence links to SHA, workflow run IDs, required-check names and server state. |
| CI-02 | Require mandatory tools in CI profile | CI-01 | Missing Aegis/audit/deny/coverage tool makes CI fail; developer quick mode stays explicit. |
| CI-03 | Pin and validate all Actions | CI-01 | Workflow checker rejects non-SHA refs and passes self-tests. |
| CI-04 | Add fail-closed change classification + stable aggregate check | CI-03 | Empty/unknown diff runs full checks; branch protection has one always-present aggregate. |
| CI-05 | Add Aegis to docs and code lanes | CI-02 | Both docs-only and code changes receive scan receipt. |
| CI-06 | Emit coverage report artifact and module trend | CI-02 | Report is commit-stamped and names metric, toolchain, denominator and floor. |
| CI-07 | Add browser E2E and accessibility smoke | CI-04 | Real browser test results attach to check; WCAG claims map to tested criteria. |
| REL-01 | Create exact artifact allowlist and release manifest | CI-03 | Validator rejects missing/extra/stale/wrong-version artifacts. |
| REL-02 | Add SBOM and provenance validation | REL-01 | SBOM validates against claimed schema and provenance digests match every artifact. |
| REL-03 | Gate Windows review and release evidence | REL-01 | Windows build is launched on a Windows host/VM for GUI, input, clipboard and persistence flows. |
| REL-04 | Rehearse draft release and publish gate | REL-01, REL-02 | Full rehearsal uploads exact assets to a draft; production remains explicit and reviewable. |
| DEP-01 | Add standalone marketing-site validation/deploy | CI-04 | Static site checks pass, deploy scopes are least privilege, served-site verification receipt exists. |
| GF-01 | Validate GitForge YAML and worker health/identity | CI-01 | Definition parser passes and run receipt shows current image/build, status, duration and logs. |

## Metrics and honest targets

- **Test coverage:** report measured line and branch coverage, plus E2E scenario coverage; define exclusions and toolchain. The user’s 99% goal is a long-term target, not a current result or a useful sole proxy for quality.
- **Documentation coverage:** track feature-to-doc links, public claim-to-test/evidence mapping, broken links, tutorial task coverage, and freshness. Do not describe prose as “99% covered” without defining the denominator.
- **Accessibility:** report each applicable WCAG criterion, test method, result, affected surface, and exception. AAA is not inferred from a contrast palette test.
- **Release readiness:** target 100% of required artifact inventory validated with provenance before publication; each platform is labeled “built,” “smoke-tested,” “user-reviewed,” or “certified” based on actual evidence.
- **CI health:** record pass rate, median/p95 duration, queue time, flaky reruns, skipped jobs, stale runner/image status, and artifact retention.

## References

- Experiment workflows: [CI](../../.github/workflows/ci.yml), [release](../../.github/workflows/release.yml), [GitForge](../../.gitforge.yml).
- Experiment local gate: [`scripts/ci-local.sh`](../../scripts/ci-local.sh), [`scripts/coverage.sh`](../../scripts/coverage.sh), [Build & Release](../build-release.md).
- Local sibling FireCraft reference: `/nas/Temp/repos/FireCraft/.github/workflows/ci.yml`, `release.yml`, `deploy-site.yml`, `.gitforge.yml`, `scripts/validate_workflow_pins.py`, `scripts/validate_release_aggregate.sh`, and `docs/decisions/0099-workflow-actions-are-pinned.md`. This is a local reference branch/state, not proof that this experiment repository inherits its gates.
