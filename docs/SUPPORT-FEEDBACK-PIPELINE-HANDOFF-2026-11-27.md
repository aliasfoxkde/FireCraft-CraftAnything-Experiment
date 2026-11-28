# FireCraft Autonomous Support, Engineering & Release Pipeline

**Status:** Proposed future architecture; not approved or implemented
**Prepared:** 2026-11-27
**Repository:** `aliasfoxkde/FireCraft-CraftAnything-Experiment`
**Inspected main SHA:** `284e00a53001da9bcb93d6de9e2c7f46abcdf2b3`
**Handoff branch:** `docs/support-feedback-pipeline-handoff-2026-11-27`
**Product baseline:** `app/Cargo.toml` version `0.3.2` at inspected SHA
**Purpose:** Explore a policy-governed support and engineering pipeline. This document is a proposal, not an implementation handoff or accepted operating policy; current code, audit, and task tracking remain authoritative.

> **Current-state note:** This proposal was inspected against `284e00a` / FireCraft `v0.3.2`. Its
> implementation status and metrics are historical. See the current [audit](AUDIT-2026-11-27.md),
> [progress](PROGRESS.md), and [active tasks](TASKS.md). It does not authorize autonomous PR merges,
> release publication, production deployment, or changes to repository protections.

---

## 1. Proposed operating model (decision pending)

A future support system could operate a continuous improvement loop:

1. A user submits a Bug, Feature Request, or Change Request from Help.
2. The system validates, classifies, deduplicates, and prioritizes it.
3. Reproducible bugs enter an automated reproduce → regression-test → fix cycle.
4. Suitable feature/change requests can be implemented behind an Experimental flag that is disabled by default.
5. An engineering agent creates a scoped branch and pull request automatically.
6. CI, static analysis, security checks, automated UI tests, and packaging gates run against the exact commit.
7. A passing change becomes eligible for staging only under an explicitly approved merge policy and protected checks; absent that decision, a maintainer merges the PR.
8. Staging is deployed to a protected Cloudflare Pages environment; tagged immutable nightly artifacts are published for opted-in developer/test cohorts.
9. Automated telemetry and explicit feedback evaluate the candidate against predeclared criteria.
10. A passing candidate may be promoted in measured cohorts after an approved release policy; stable production promotion remains maintainer-controlled unless a separate policy is approved. A failed candidate is stopped or rolled back automatically where that capability has been tested.
11. Ticket state and reporter notifications are updated from verified deployment, validation, and rollback events.

**Automation authority is unresolved.** AI may eventually prepare implementation PRs, but automatic merge to staging or production deployment is not authorized by this proposal. Maintainers retain PR merge and release authority until they explicitly accept a threat model, repository policy, rollback plan, and operational ownership. User consent is still required for pre-release participation, experimental features, telemetry, or screenshot sharing.

The system must still stop automatically when it cannot establish a safe result. Security reports, suspected data corruption, ambiguous destructive migrations, credential exposure, unsafe code execution, unclear licensing, missing build provenance, failed gates, or contradictory runtime signals are quarantined, disabled, or held. A hard automated stop is preferable to shipping an unverified change. It must not quietly convert uncertainty into success.

### Core design principle

**Bounded automation; deterministic authorization; progressive exposure; reversible changes.**

An AI tool may draft or implement work in an isolated branch. A deterministic policy engine can enforce limits, while maintainers retain merge and release authority until a separate decision grants narrower automation. Tests and runtime evidence must decide whether an artifact is eligible to advance. Feature flags and release channels can limit exposure and make rollback practical.

## 2. Target architecture

```text
FireCraft native / WASM
  Help -> Submit Support Ticket
       |
       v
Cloudflare Worker: support-api
  validation / consent / abuse controls / idempotency
       |
       +----> D1: canonical tickets, events, demand, decisions
       |
       +----> R2: private optional screenshots
       |
       v
Deterministic intake + policy engine
  classify -> deduplicate -> score -> reproduce / decide
       |
       +----> request for information / duplicate / defer / close
       |
       v
Provider-neutral AI + coding-agent adapter
  isolated task context / bounded tools / budget / retry limit
       |
       v
GitHub: issue mirror + ticket-specific branch + implementation PR
       |
       v
Required exact-SHA CI + merge queue
  fmt / strict Clippy / tests / coverage ratchet / docs / security
  WASM / native platform builds / browser E2E / artifact checks
       |
       v
staging branch
       |
       +----> Cloudflare Pages protected staging deployment
       |
       +----> immutable nightly GitHub Release artifacts
       |         |
       |         +----> opted-in developer/test cohort
       |
       v
Automated candidate evaluation
  crash/error / task success / performance / compatibility / feedback
       |
       +----> disable flag / revert staging / mark failed / notify users
       |
       v
Canary -> 5% -> 25% -> 50% -> 100% stable rollout
       |
       v
main + stable GitHub Release + stable channel manifest
       |
       +----> WebOS downloads the WASM application package from GitHub
       +----> Desktop updater downloads the correct platform package
       |
       v
Immutable prior releases + cached last-known-good + rollback controller
```

### System responsibilities

| Component | Responsibility | Must not do |
|---|---|---|
| FireCraft client | User report, consent, ticket receipt, channel choice, feature opt-in, update UX | Store GitHub/Cloudflare/model credentials or make release policy decisions |
| Worker | Validate requests, enforce consent and limits, coordinate services | Execute repository code or blindly trust model output |
| D1 | Canonical ticket state, immutable event history, demand signals, deployment mapping | Store binaries or public secrets |
| R2 | Private screenshot objects and expiry | Publicly expose attachments or retain them indefinitely |
| Policy engine | Apply deterministic acceptance, risk, and promotion rules | Treat confidence scores as proof |
| Model adapter | Categorize, summarize, identify duplicates, generate test plans and code changes | Grant itself permissions or override release policy |
| GitHub | Source of truth for code, PRs, checks, commits, tags, releases, artifact provenance | Serve as the only private reporter database |
| CI and merge queue | Validate exact commit; serialize protected integration | Accept skipped or stale required checks |
| Cloudflare Pages | Staging/review web build | Receive production secrets or real user ticket content |
| Release controller | Promote channel pointers and rollback to immutable known-good builds | Rebuild different bytes under an existing release identity |
| WebOS | Resolve channel manifest, fetch, verify, install, pin, and roll back app packages | Trust an unverified `latest` URL or overwrite user data |
| Desktop updater | Update/downgrade the matching native installation safely | Replace a package-manager-owned install behind the package manager |

Use Cloudflare Worker + D1 + private R2 for support intake. GitHub remains the engineering execution plane. Keep the app usable when the support API or model provider is unavailable.

## 3. User-facing support form and data contract

### 3.1 Entry point

Add Help → Submit Support Ticket with three types:

- **Bug:** FireCraft is broken, crashes, loses work, or behaves incorrectly.
- **Feature Request:** a capability or workflow does not exist.
- **Change Request:** an existing interaction, default, visual treatment, or workflow should be different.

The user must not need a GitHub account or understand branches, labels, PRs, models, or deployment environments.

### 3.2 Minimum form

Required: type, title, description, and agreement to submit the entered content. Acknowledgement should explain that automated systems and AI agents may process the report and may create/test changes.

For bugs, show optional expected result, actual result, steps, frequency, impact, and whether a reliable reproduction is available.

For feature/change requests, ask for the user goal, desired outcome, current workaround, and optional context. Avoid asking the user to prescribe architecture.

Email is optional. Provide a receipt identifier and a revocable reporter token if status retrieval is supported. Keep public posting separate from private support submission.

### 3.3 Metadata and screenshot consent

Before submit, show a clear preview of metadata. Native and WASM collection must differ accurately:

| Field | Native | WASM |
|---|---|---|
| FireCraft version/build/source | Yes | Yes |
| OS family and architecture | Best-effort known platform | Browser/platform hint only |
| Device category | Coarse desktop/tablet/phone | Coarse hint only |
| Browser family/version | Not applicable | Yes, normalized |
| Viewport bucket | Not applicable | Yes |
| Local file paths, user name, serials | Never | Never |
| Project/document contents or clipboard | Never | Never |
| Precise location or covert fingerprint | Never | Never |

Do not collect a full raw user-agent if normalized fields suffice. Do not enumerate installed fonts, fingerprint Canvas/WebGL, infer identity, or keep raw IPs in application logs. A metadata toggle must permit a minimum-content report.

Screenshots are a separate explicit action and consent:
1. User chooses a file or deliberately invokes a supported visible capture flow.
2. Show preview, file type, dimensions, and size.
3. Explain that screenshots may contain private or work information.
4. User must explicitly approve upload.
5. Re-encode to an allowlisted raster format, strip EXIF, enforce byte/dimension limits, and scan where practical.
6. Store with random object key in private R2; signed access is short-lived and maintainer-scoped.
7. Never attach a FireCraft project by default. Do not upload document contents, local paths, or clipboard data.

### 3.4 Submission reliability

Use versioned API routes, strict schemas, request size limits, idempotency keys, rate limiting, and transactional ticket + event writes. Preserve the form after a transient failure. Provide save/copy fallback and clear receipt status. Never claim submission succeeded before the Worker confirms durable storage.

Suggested routes:
- `POST /v1/tickets`
- `GET /v1/tickets/{public_id}`
- `POST /v1/tickets/{public_id}/attachments/presign`
- `POST /v1/tickets/{public_id}/attachments/complete`
- `POST /v1/tickets/{public_id}/votes`
- `POST /v1/tickets/{public_id}/feedback`
- `DELETE /v1/tickets/{public_id}/contact`

Use D1 tables for `tickets`, `ticket_events`, `ticket_attachments`, `ticket_votes`, `ticket_relations`, `ticket_triage`, `ticket_jobs`, `ticket_deployments`, `ticket_feedback`, and `release_observations`. Add migrations, constraints, indexes, append-only event history, retention timestamps, and deletion workflows. Protect contact data separately from engineering summaries.

## 4. Ticket state machine and automated disposition

States are deterministic and auditable. A model may recommend a transition; only the policy service applies it after checking required evidence.

| State | Meaning | Allowed next states |
|---|---|---|
| `received` | Durable intake receipt created | `normalizing`, `rejected` |
| `normalizing` | Validate schema, consent, abuse, secrets, metadata | `triaging`, `needs-info`, `rejected` |
| `triaging` | Classify, deduplicate, score, detect risk | `duplicate`, `needs-info`, `reproducing`, `candidate`, `deferred`, `wontfix`, `quarantined` |
| `reproducing` | Build deterministic reproduction or fixture | `verified`, `needs-info`, `unable-to-reproduce`, `quarantined` |
| `verified` | Bug reproduced or request accepted by policy | `queued-for-agent` |
| `candidate` | Feature/change passes experimental eligibility | `queued-for-agent`, `deferred`, `wontfix` |
| `queued-for-agent` | Bounded brief and branch job created | `implementing`, `deferred`, `quarantined` |
| `implementing` | Agent is producing code/tests | `pr-open`, `blocked`, `failed` |
| `pr-open` | PR exists with ticket/source traceability | `checks-running`, `blocked` |
| `checks-running` | Exact-SHA gates and previews executing | `staging`, `fixing`, `blocked`, `failed` |
| `staging` | Merged into staging and deployed | `cohort-testing`, `rolled-back` |
| `cohort-testing` | Opted-in test group evaluating build | `promoting`, `fixing`, `rolled-back`, `deferred` |
| `promoting` | Automated staged rollout | `resolved`, `rolled-back` |
| `resolved` | Verified fix/feature is stable and available | terminal (may reopen on regression) |
| `duplicate`, `deferred`, `wontfix`, `rejected`, `unable-to-reproduce`, `failed` | Explicit disposition with reason | Reopen only on new evidence or reporter response |

Every transition records actor type, timestamp, policy version, source evidence, old/new state, and a human-readable reason. AI-generated rationale is clearly marked as generated. Reporter notifications are based on actual state changes, not on model claims.

### 4.1 Bug acceptance and reproduction

A bug should be fixed when it is sufficiently reproducible and the fix can be validated without creating unacceptable regression risk.

1. Normalize the report and check for duplicates.
2. Extract environment, observed result, expected result, frequency, and suspected subsystem.
3. Build a minimal fixture, deterministic test, scripted UI journey, fuzz input, or reproducer.
4. Run the reproducer against the unmodified baseline. Save test output and source SHA.
5. If it fails on baseline in the described way, mark the bug verified and require a regression test.
6. If it cannot reproduce, automatically request only the missing details. After a configurable waiting window, close as unable-to-reproduce with the saved rationale; allow reopening when new evidence arrives.
7. Run the generated fix against the new regression test and all applicable existing suites.
8. Reject a patch that merely weakens/removes the test or suppresses the symptom without satisfying the expected behavior.
9. Proceed only through the same exact-SHA gates and staged validation as all changes.

Security vulnerabilities, data-loss/corruption reports, and possible privacy exposures go to a private quarantine path, not public issue mirroring. Do not run attacker-supplied files or scripts on privileged workers.

### 4.2 Feature/change request acceptance

Do not treat every feature suggestion as a commitment. Evaluate it from:
- Unique reporters, votes, and independent installations.
- Recency and sustained demand, not just one viral spike.
- User impact and workflow frequency.
- Strategic product fit and accessibility benefit.
- Implementation complexity, architecture impact, maintenance cost, and testability.
- Reversibility and ability to isolate it behind an off-by-default flag.
- Data compatibility, security, privacy, performance, and license constraints.

Initial demand bands (configurable, not promises):
- **Signal:** one request is stored and deduplicated; no automatic build obligation.
- **Emerging:** at least 2 independent reporters or 3 distinct votes; cluster and score it.
- **Candidate:** at least 3 independent reporters in 30 days or 5 distinct votes, with a supported user scenario and an implementable bounded scope.
- **High demand:** repeated activity across multiple release windows; eligible for earlier roadmap priority.
- **Strategic exception:** accessibility, critical compatibility, or reliability improvements may qualify without vote thresholds.

Count one vote per pseudonymous installation/reporter principal, with abuse controls. Multiple submissions by the same reporter must not simulate demand. Never use raw vote count alone as the product decision. Heavy usage of an experimental feature is one signal; task success, user feedback, crash rate, performance, support burden, and opt-out rates also matter.

A feature can be marked `deferred` or `wontfix` automatically with a specific reason: duplicate, unsupported use case, insufficient evidence, conflicts with product invariants, excessive risk/cost, or no safe implementation path. It must not disappear silently.

### 4.3 Experimental feature contract

The system may implement and ship a new feature without waiting for broad demand when it is safe to experiment.

Every Experimental feature must:
- Be **disabled by default** in every channel, including nightly, until the user explicitly opts in.
- Have an individual flag, stable identifier, owner subsystem, version, and kill switch.
- Be isolated so disabling it restores the prior behavior.
- Avoid modifying existing document data unless a reversible, versioned, backward-compatible migration exists.
- Include tests for on/off behavior, persistence, restart, import/export, and downgrade.
- Explain experimental status, known limits, and how to disable it.
- Emit only consented, privacy-minimized exposure/usage/error signals.
- Never be used to bypass security, accessibility, privacy, or correctness requirements.

Feature flags are not a substitute for safe schema changes. Use expand/contract migration: new code first reads old and new forms, new writes remain backwards-compatible, and old format removal happens only after the rollback window. If downgrade cannot read the new data, the release is not automatically reversible and must remain blocked or require an explicit export/backup path.

## 5. Policy engine and automation permissions

Separate AI inference from authority. The policy engine is code with versioned rules and tests.

### 5.1 Autonomous actions

Permitted when policy checks pass:
- classify, cluster, deduplicate, prioritize, request missing details, and close spam/duplicates;
- create GitHub issue mirrors from scrubbed summaries;
- create a feature/bug brief and a ticket-specific branch;
- run test generation, implementation attempts, static analysis, builds, test suites, and local sandboxes;
- open/update PRs, push commits to the task branch, and request automated CI;
- fix failed checks within a bounded retry/compute budget;
- merge to `staging` through protected required checks/merge queue;
- deploy protected staging builds and publish nightly pre-releases;
- promote channel manifests and stable releases after automated cohort gates;
- disable a feature flag, roll back a deployment/channel pointer, revert a staging change, and notify affected users;
- update ticket disposition from verified pipeline events.

### 5.2 Actions forbidden to an agent/model

The agent cannot:
- write directly to `main` or move protected release tags;
- modify branch protection, rulesets, GitHub Actions secrets, Cloudflare production secrets, or signing keys;
- access the support database's private reporter contact fields or raw screenshots unless a narrowly scoped, audited tool is required;
- read arbitrary production data, user documents, local machine credentials, or environment secrets;
- disable required checks, rewrite CI results, remove security gates, or approve its own exception;
- change the policy thresholds or feature-flag defaults for its own task;
- claim deployment/test success without machine-verifiable receipts.

Use GitHub App installation tokens with minimum repository permissions and short-lived credentials. Separate the agent token, CI token, staging deploy token, production release token, and signing identity. Production promotion must be available only to the release controller after policy evaluation, not to the coding model. The coding agent's tools must not expose a generic arbitrary command runner with production credentials.

### 5.3 Risk tiers

| Tier | Examples | Automated behavior |
|---|---|---|
| R0 — isolated/reversible | Documentation, cosmetic UI, independent opt-in feature | Agent PR; standard CI; maintainer merge by default; optional staging auto-merge only after explicit policy approval |
| R1 — normal product change | New command, tool, panel, format support behind flags | Agent PR; full regression/E2E; staging; opt-in/nightly; progressive promotion |
| R2 — sensitive compatibility | Document serialization, import/export, core model, dependency graph, performance-sensitive canvas | Expanded golden/property/fuzz/compatibility tests; larger cohort; longer observation; auto-hold on uncertainty |
| R3 — critical/irreversible | Security boundary, authentication, encryption, destructive migration, data-loss risk, release-signing trust, arbitrary code execution | Quarantine or fail closed; no autonomous production release until a specific safe automated proof path is defined |

R3 is not a manual approval queue. It is a hard policy boundary that prevents the pipeline from claiming it can safely automate an unproven class of change. Where safe isolation and validation are later engineered, the policy can be revised separately with tests and version history.

### 5.4 Agent execution loop

1. Create an immutable task record containing sanitized ticket data, repo SHA, task policy version, expected result, reproduction, constraints, and resource budget.
2. Search current code, tests, architecture docs, prior fixes, open PRs, and existing issues for related implementation.
3. Produce a plan with likely impacted modules, invariants, risk tier, test matrix, and rollback plan.
4. Create an isolated worktree/container and ticket-specific branch from current staging.
5. Implement a small, coherent patch with regression tests.
6. Run focused tests before the full suite. Do not repeatedly run expensive CI for unchanged code.
7. Run format, strict lint, complete tests, docs, dependency/security, WASM, native builds, and relevant browser/UI tests.
8. On failure, classify the failure. Retry only fixable implementation errors within a bounded attempt/compute budget (initial default: 3 repair cycles). Do not retry flaky tests indefinitely.
9. If checks pass, open/update the PR with complete evidence, source SHA, generated patch summary, known limitations, and ticket links.
10. Let required checks and a merge queue gate integration into staging. If checks fail after queueing, return the ticket to fixing or blocked with machine-readable cause.
11. Release controller deploys staging and continues cohort validation.
12. The ticket is resolved only when the declared rollout/availability condition is satisfied, not when the agent opens a PR.

Keep each task on its own branch. Do not use one mutable staging branch as the agent worktree. Ensure cancellation cleans up worktrees, jobs, and temporary credentials.

## 6. GitHub workflow, CI and merge policy

### 6.1 Branches

| Branch/channel | Purpose | Who writes |
|---|---|---|
| `main` | Stable production source | Protected promotion PR/merge queue only |
| `staging` | Next integrated release | Protected PR/merge queue only |
| `agent/FC-...` / `fix/FC-...` | One isolated ticket implementation | Ticket-scoped agent |
| `release/*` or immutable tags | Release preparation and immutable versions | Release controller |

If `staging` does not exist, create it from the current production release source and reconcile any unmerged work first. Never make staging drift a silent second source of truth.

### 6.2 Required checks

Build on the existing `.github/workflows/ci.yml` and `scripts/ci-local.sh`. Every PR to staging and main must have:
- pinned toolchain and SHA-pinned third-party Actions;
- `cargo fmt --all -- --check`;
- `cargo clippy --all-targets -- -D warnings`;
- full Rust unit/integration/doc tests;
- documentation link/claim gate and module-size gate;
- coverage floor / regression ratchet (measured, not a fixed 99% fiction);
- `cargo deny`, `cargo audit`, license/provenance policy;
- WASM compile and production-mode bundle build;
- Windows, macOS and Linux build/package smoke tests appropriate to the change;
- WASM browser boot, Playwright/browser tests for critical journeys, console/error checks, and app startup/health checks;
- focused golden/property/fuzz/compatibility tests for formats and document mutations;
- source-SHA manifest, checksums, and artifact inventory;
- required status checks bound to the expected GitHub Actions identity.

Add a `merge_group` trigger to required-check workflows before enabling GitHub merge queues. Without this event, the checks that run on pull requests may not report successfully for merge-queue candidates. Required checks must not be skipped when unrelated documentation or workflow conditions evaluate incorrectly.

CI should run cheap/static/focused tests first and shard expensive OS/browser suites. Keep a cached exact-SHA evidence ledger to avoid redundant work; never reuse a pass result for a different commit or dependency lockfile.

### 6.3 Optional staging auto-merge (not approved)

If maintainers later approve staging auto-merge, a PR should be eligible only when:
- the ticket is policy-accepted and correctly risk-tiered;
- patch scope matches the task brief;
- mandatory tests exist, including a regression test for verified bugs;
- every required exact-SHA gate passes;
- no unresolved critical security/license/provenance alert exists;
- change is isolated or has a tested migration and rollback path;
- preview deployment health check passes where relevant.

A merge queue should serialize candidates and test the merge result. Agent-generated PR comments should state they are AI-authored and include evidence. Until an explicit authorization policy is adopted, required maintainer approval remains in force.

## 7. Staging and rollout policy

### 7.1 Staging

Every candidate merged to `staging` is deployed automatically to a dedicated Cloudflare Pages staging project or branch alias. Restrict non-public review environments with Cloudflare Access. Previews are public by default unless access is configured; verify the configured access policy instead of assuming random preview URLs are private.

Each deployment record stores:
- ticket ID(s), PR URL, branch, commit SHA, tree SHA;
- workflow run and required check results;
- WASM bundle SHA-256 and build manifest;
- Pages deployment ID and URL;
- target channel and flag manifest;
- health status, start/end timestamps, and rollback link.

Use synthetic data and staging API resources. Do not pass the support D1/R2 data or production credentials to preview builds. Staging should run automated browser smoke tests after deployment: WASM initializes, canvas paints, user can create/edit/save/export a sample document, no fatal JS errors, and expected feature-flag state is reported.

### 7.2 Nightly/test channel

After staging gates pass:
- publish immutable GitHub pre-release assets named with date and source SHA, e.g. `nightly-YYYYMMDD-<short-sha>`;
- attach platform artifacts as available, beginning with WASM ZIP and desktop packages produced from the same source SHA;
- include build-info, source SHA, full manifest, SHA-256 checksums, dependency/SBOM metadata, and provenance attestation;
- update the `nightly` channel manifest only after every required artifact is uploaded and verified;
- invite users to opt into nightly from WebOS or desktop app settings;
- keep new experimental features separately disabled until the user enables each one.

Nightly opt-in is not implied by installing stable. Provide clear channel warnings and one-click switch back to stable. A failed nightly must not overwrite a user's stable installation or their document files.

### 7.3 Cohort validation and automatic sign-off

The system signs off a candidate only from a declared observation policy, not a single thumbs-up or one passing test.

Track opt-in, privacy-minimized signals:
- install/launch success, app startup time, WASM initialization failure;
- crash and fatal-error rate, hangs, save/export failures;
- user task completion for supported scripted or voluntarily measured flows;
- performance against per-platform budgets;
- feature flag exposure, enable/disable, successful task completion, and opt-out;
- explicit “works”, “problem”, and “disable” feedback;
- upgrade/downgrade and rollback success.

Do not collect document content, image pixels, file names, clipboard data, or arbitrary keystrokes. Usage telemetry must be opt-in, explain the data and retention, allow opt-out, and use pseudonymous rotation rather than a permanent cross-product identity.

Suggested initial rollout stages (tune only through a policy change):
1. **Staging:** automated maintainer-like tests and scripted sample documents.
2. **Nightly:** voluntary developer/test cohort; features still off by default.
3. **Canary:** 1% or a small allowlisted cohort on stable only after nightly gates.
4. **5%:** at least 24 hours and minimum observation sample.
5. **25%:** at least 48 hours and stable health.
6. **50%:** at least 48 hours and stable health.
7. **100%:** stable channel pointer is promoted.
8. **Soak:** continue crash/performance/error monitoring after full rollout.

A sample target is at least 20 distinct opted-in installations and 100 successful launches/sessions for ordinary reversible changes. High-risk changes need a larger and longer cohort. If the sample is insufficient, the system must say “insufficient evidence” and hold the candidate; it must not assume that no reported bug means success.

Initial automatic stop conditions should include:
- crash/fatal error rate materially worse than the previous stable baseline (e.g. greater than 1.5× and at least 5 affected sessions);
- any confirmed data-loss/corruption report;
- save/export failure rate increase above the configured threshold;
- startup/wasm initialization regression above 1 percentage point;
- hard performance-budget regression (e.g. p95 critical interaction latency >20% worse);
- security, signature, provenance, or artifact mismatch;
- explicit rapid increase in opt-outs/disable feedback;
- missing monitoring, stale source identity, failed health check, or telemetry outage where the signal is required for that tier.

Set numeric budgets from a measured stable baseline by platform and task, not arbitrary global numbers. Keep thresholds versioned and test them with synthetic failure injection. An alert is not automatically proof that a code change caused the event; however, uncertain attribution must stop promotion, not continue it.

A candidate is promoted automatically when the minimum cohort/time is met, every guardrail remains within tolerance, there are no unresolved critical safety reports, and all platform-specific required signals pass. No human sign-off is required for ordinary R0/R1 changes.

### 7.4 Feature-level rollout

A release can be stable while an individual new feature remains Experimental and opt-in. Promotion of a feature flag from Experimental to Preview/Stable must be separately evaluated from promotion of the binary.

Suggested feature flag states:
- `experimental_off`: compiled/distributed, disabled for everyone unless manually enabled;
- `experimental_opt_in`: user enables it; no automatic user enrollment;
- `cohort_preview`: explicit opt-in cohort; can be disabled remotely;
- `stable_on`: default on after measured success and compatibility plan;
- `disabled_kill_switch`: forced off due to regression;
- `retired`: removed after migration and deprecation windows.

Never auto-enable a feature just because the user previously opened an Experimental setting. Ask for explicit enablement of each feature, with a clear description.

## 8. Rollback and recovery

Rollback must be a first-class pipeline operation, tested before promotion.

### 8.1 Fast rollback controls

Maintain:
- immutable release assets, tags, manifests, and source SHAs;
- last-known-good stable and nightly channel manifests;
- channel pointer history with actor/policy version/time;
- server-side kill switches for independent feature flags;
- an automated release controller capable of reverting the channel pointer without rebuilding;
- cached current and previous known-good WASM app bundles in WebOS;
- desktop updater backup/restore state for supported installation types;
- automatic health checks after rollback.

Rollback action sequence:
1. Freeze further promotion immediately.
2. Turn off the specific feature flag where the failure is isolated.
3. If app-wide regression exists, repoint channel manifest to previous immutable release.
4. WebOS/Desktop verifies and restores the last-known-good artifact.
5. Run smoke/health tests against rolled-back build.
6. Reopen or link affected tickets, record failed source SHA and diagnostic reason, and notify users who opted into release notices.
7. Keep bad artifact and evidence for debugging; never overwrite or delete it to hide the failure.
8. Agent may prepare a fix PR with the failure evidence attached.

### 8.2 Data and document safety

The editor's user documents must not be stored in a backend migration or rewritten automatically by the support pipeline. Keep documents local unless a separate explicit cloud feature exists.

Binary rollback does not reverse a document schema migration. Any persisted format evolution needs:
- versioned document schema;
- old-reader/new-reader compatibility tests;
- backup before migration;
- migration idempotency and corruption tests;
- read-only/restore path if migration fails;
- a downgrade policy proven to work with the old binary.

Prefer additive metadata and backwards-compatible readers. If an experimental feature writes a format that previous releases cannot read, it is not eligible for broad rollout until a safe compatibility/backup story exists.

## 9. WebOS application distribution from GitHub Releases

The WebOS host should execute FireCraft's web/WASM package without requiring FireCraft to host and serve a separate application website per install. WebOS itself is the host/runtime; GitHub Releases provide immutable versioned package downloads.

### 9.1 Package what the WASM app actually needs

Do not fetch only `firecraft_bg.wasm`. The current release workflow produces a browser package containing the WASM binary, wasm-bindgen JavaScript glue, HTML entrypoint, and favicon/assets. WebOS should download the complete application package/ZIP (or a future equivalent versioned package format) and validate all referenced files.

Each release includes a manifest with:
- application ID and package schema version;
- FireCraft application version and channel;
- source commit SHA and release tag;
- build workflow/run identity and build timestamp;
- WASM target/runtime ABI and required WebOS host API version;
- entrypoint, JS glue, WASM filename, static asset inventory;
- file sizes and SHA-256 hashes;
- required permissions/capabilities;
- feature-flag schema version and defaults;
- minimum compatible WebOS version;
- provenance/attestation identity.

### 9.2 Channel manifests

Publish small signed channel documents:
- `stable.json`
- `nightly.json`
- later `beta.json`

Each document points to an immutable GitHub Release tag and exact package asset, with version, commit SHA, hash, manifest URL, minimum runtime, and a monotonically increasing channel generation. The manifest is a pointer; it is not the binary itself.

WebOS resolution sequence:
1. Read user's selected channel (stable by default).
2. Fetch the channel manifest from a trusted FireCraft-owned endpoint/source.
3. Validate schema, signature/provenance, allowed repository, tag format, and version policy.
4. Compare installed and available versions using semantic-version rules, not string equality.
5. Download the exact immutable GitHub Release asset.
6. Verify package digest and attestation before extraction/installation.
7. Install into a versioned app directory; do not overwrite the active version in place.
8. Launch a smoke/health probe and verify the declared entrypoint and WASM startup.
9. Atomically switch the app's active-version pointer.
10. Keep the previous known-good package and support explicit version pin/rollback.
11. Cache the active and previous versions for offline recovery; garbage-collect older versions only under a documented retention policy.

Use GitHub Release APIs/assets for binaries, not a moving `latest/download` alias. The WebOS runtime may use an application catalog and local registry, but should not proxy every package through a FireCraft server. Cache manifests and artifacts according to version/hash while revalidating channel pointers. Respect GitHub API/asset rate limits and provide a fallback mirror only if it preserves exact content hashes and provenance.

### 9.3 WebOS host contract

Define a stable versioned contract between WebOS and hosted WASM applications:
- lifecycle: init, ready, suspend, resume, shutdown;
- storage: app-scoped persistent store, quota, backup/export/restore;
- file open/save/export capabilities mediated by WebOS;
- clipboard, notifications, menus/windowing only through declared host capabilities;
- update/rollback interface;
- versioned capability negotiation and optional features.

If FireCraft is using a standalone browser HTML page, it must continue to work independently. WebOS integration must not fork or replace the public WASM build; it supplies host capabilities through a compatibility layer. Test both standalone browser and WebOS host modes on each artifact.

## 10. Desktop stable/nightly updates and downgrades

The existing updater is a “check and open release page” implementation, not an updater. Replace it with a channel-aware update subsystem.

### 10.1 Channels

- **Stable:** default, tagged `vX.Y.Z` release, intended for general users.
- **Beta:** optional cohort builds with a documented risk level and reversible package.
- **Nightly:** frequent immutable date/source-SHA pre-releases, intended for opted-in test/developer users.

A user can choose channel, pause automatic downloads, pin a version, check updates, inspect release notes, install/delay restart, downgrade, and switch back to stable. Never mix stable and nightly version comparison based on plain string equality.

### 10.2 Installation-specific updater

Detect install ownership and use the right mechanism:
- Windows portable ZIP: install helper downloads/verifies/extracts to a versioned directory, updates a launcher pointer, preserves old binary until post-update health check. Do not overwrite a running executable.
- Windows installer: use signed installer/updater and verify Authenticode when signing is configured.
- macOS app bundle/DMG: stage a new bundle, verify signature/notarization status, replace safely after app exit, and preserve last-known-good bundle.
- Linux AppImage: download to versioned path, verify, atomically switch launcher/symlink.
- Linux DEB/Flatpak: prefer the system package manager/Flatpak update path; do not bypass package ownership with an application-level self-updater.
- macOS/Windows unsigned builds must be labeled accurately and must not claim platform-trusted signing.

Keep user data, settings, document recovery, and plugin/user extensions outside the replaceable application directory. Updater must stage, validate, switch, health-check, and revert. Support explicit downgrade to a known prior version with compatibility warnings where the app has changed persisted state.

### 10.3 Update protocol

The update subsystem should:
1. Fetch a channel manifest asynchronously so the UI never blocks on network I/O.
2. Validate channel, semver, platform, architecture, minimum OS/runtime, and release state.
3. Display release notes and artifact size before download.
4. Download with resumable/timeout-aware behavior to a temporary path.
5. Verify checksum and provenance/signature before install.
6. Stage into a versioned path; preserve old version.
7. Switch atomically (or via helper on Windows/macOS).
8. Launch the updated process and wait for a small health signal.
9. If health check times out or process exits/crashes repeatedly, roll back automatically.
10. Report result and preserve diagnostic evidence without leaking credentials or document contents.

A release must retain prior stable and nightly versions and provide a clear downgrade target. Do not make the updater auto-downgrade user documents or silently migrate app data to an incompatible schema.

## 11. OpenCodifier and model-provider reliability

OpenCodifier is a candidate worker, not a trust boundary. Build a provider-neutral interface for both triage and coding agents. Keep model selection/configuration replaceable.

### Required evaluation before autonomous dispatch

Build a benchmark from:
- synthetic bugs with known reproductions and expected fixes;
- historical FireCraft bugs and regression tests;
- small feature requests with explicit acceptance criteria;
- duplicate/ambiguous requests;
- adversarial prompt injection in issue text, Markdown, image metadata, and comments;
- dependency/security/licensing cases that should be refused or quarantined;
- intentionally flaky tests and unavailable provider scenarios.

Measure:
- correct category and duplicate grouping;
- reproduction success rate;
- patch acceptance rate on hidden tests;
- regression introduction rate;
- strict-lint/full-suite pass rate;
- first-pass and repaired-pass success;
- false “fixed” claims;
- unsafe tool/permission attempts;
- cost, latency, context truncation, and retry rate;
- rollback frequency after staged exposure.

Use shadow-mode only to gather a baseline if needed; shadow mode is a calibration step, not a permanent human approval workflow. Promote the model adapter to autonomous dispatch when it meets declared thresholds on a held-out evaluation set. If OpenCodifier fails or times out, fall back to another approved provider or queue the task; do not relax test gates.

The Worker passes the model only sanitized ticket content and explicitly selected repository context. All ticket text, comments, screenshots, external documentation, and repository content are untrusted. Separate instructions from data, isolate tool outputs, and never treat content as permission to execute commands.

### Runtime reliability controls

- JSON-schema validation for every plan/triage result.
- Separate read-only triage permissions from write-enabled ticket branch permissions.
- Ticket-scoped GitHub token; no main/release write.
- Sandboxed builds with network restricted unless dependency download is explicitly needed.
- Tool allowlist and path allowlist.
- Prompt, model, adapter, policy, schema, and source SHA recorded per run.
- Cost/token/time/attempt budgets and cancellation.
- Circuit breaker and rate-limit handling.
- Deterministic pre/post checks; AI cannot mark its own work successful.
- Retry only fixable failures; detect flaky infrastructure separately.
- Store patch diff and test receipts for reproducibility.

## 12. Notifications and feedback loop

Notify users when meaningful ticket/build events occur:
- ticket received, needs more information, duplicate/closed/deferred with reason;
- fix or Experimental feature deployed to staging/nightly;
- a new nightly/stable version is available;
- an opted-in feature is ready to test;
- a rollout is paused or rolled back;
- ticket resolved in stable with release/version information.

Use an in-app inbox/notification center first. Email is optional and consented. Notifications must not claim a fix is released until the relevant channel contains the verified artifact.

After a user tests an experimental feature or fix, offer direct structured feedback:
- worked / partially worked / did not work;
- optional short explanation;
- optional screenshot with separate consent;
- disable feature / revert to stable.

Connect feedback to exact version, source SHA, flag state, platform, and ticket IDs. Don't ask the user to reproduce bugs that already have verified automated repros; instead ask whether their issue is resolved in the candidate version.

## 13. Observability and system health

Build a dashboard/API for:
- tickets by type/state/risk/age and duplicate clusters;
- accepted/closed/deferred dispositions and reasons;
- agent jobs, model provider/latency/cost, retries, failures;
- PRs, exact source SHA, required checks, merge-queue status;
- staging deployment health and per-platform artifact health;
- nightly cohort size, launches, task success, crashes, opt-outs, and feedback;
- channel promotion history and active version;
- rollback events, time-to-detect, time-to-disable, and recovery success;
- update adoption and failed update/downgrade attempts.

Avoid a dashboard that only counts tickets or shows model confidence. The system must explain which evidence permitted a change to advance and which metric caused a hold or rollback.

Operational targets to define and measure:
- median time from ticket to classified disposition;
- median time from verified bug to fix PR;
- median time to staging;
- percentage of patches passing regression + full gates;
- false resolution rate;
- staging-to-stable rollback rate;
- time to global kill switch;
- updater rollback success;
- user opt-in and opt-out rates.

## 14. Repository-specific audit and known issues

This was a read-only source/document review at main SHA `284e00a53001da9bcb93d6de9e2c7f46abcdf2b3`, plus prior dated audit reports. No build, test, or runtime command was executed during this inspection. Findings below are either directly visible in the checked-in code or explicitly labeled as unverified follow-up.

| Priority | Finding | Evidence and impact | Required action |
|---|---|---|---|
| P1 | No real channel-aware updater or rollback | `app/src/app.rs` `check_for_updates()` calls GitHub Releases `/releases/latest`; it only displays version status and opens the release page. There is no install, nightly/beta selection, pinning, artifact verification, health check, or downgrade flow. | Implement the channel/update subsystem in §10; make updater a separate testable module. |
| P1 | Update check blocks the UI thread | `check_for_updates()` calls `Command::output()` synchronously with a six-second curl timeout and is invoked from the Help menu/dialog. A slow network/API can freeze the editor for up to that timeout. | Move network work to async/background worker and add timeout/cancellation/retry UI tests. |
| P2 | macOS release-page launch likely uses the wrong launcher | `open_update_release_page()` uses `xdg-open` for every non-Windows, non-WASM target; standard macOS uses `open`. This is a likely macOS-specific failure unless `xdg-open` happens to be installed. | Use platform-specific opener or a portable opener library; test macOS and Linux separately. |
| P2 | AI API key is passed as a curl command-line argument | `start_ai_request()` constructs `Authorization: Bearer {key}` and passes it via `Command::args`. Process arguments can be observable to other local processes/users depending on OS and permissions. | Prefer an in-process HTTP client; otherwise pass sensitive headers over a protected stdin/pipe and ensure logs/errors never echo them. Add secret-redaction tests. |
| P2 | Unknown file extensions silently default to PNG | `Format::from_path()` returns `Format::Png` for any unrecognized extension. `save()` then rasterizes and writes PNG bytes under the supplied unknown filename, which can mislead users and downstream tools. | Return an explicit unknown/unsupported format result; require confirmation or reject unsupported extensions. Add tests for no extension and unknown extension. |
| P1 | Release and PR CI do not yet form the requested autonomous pipeline | Checked workflows run CI on PRs to main and publish on version tags; no branch integration queue, staging deployment gate, nightly channel manifest, cohort controller, or automatic rollback is wired into these two workflows. | Implement staged branch topology and separate release controller; preserve current tagged release flow until replacement is proven. |
| P1 | Browser/native acceptance remains incomplete | Dated audit/task ledger still records missing coverage for browser paste permission denial, malformed/large data, focus, native clipboard, native Windows GUI, macOS GUI and update behavior. Existing WASM compile/build is not equivalent to browser E2E. | Expand browser automation and platform acceptance receipts before claiming cross-platform release confidence. |
| P2 | Dependency advisories remain in the dated audit | `docs/AUDIT-2026-11-26.md` records `cargo audit` warnings for unmaintained `ttf-parser`, `zbus_names`, `zvariant*`, and unsound `heapless` 0.8 pulled by `rstar` via `geo`. They were not described as fixed. | Re-run current `cargo audit`, assess exploitability and update paths, pin policy/exception expiry, and block new critical/high advisories. Do not copy dated counts as current. |
| P2 | Coverage is below the requested 99% and UI evidence is incomplete | Dated audit snapshots contain several different historical measurements; the latest reviewed source does not prove 99% coverage or full criterion-level WCAG conformance. | Rerun exact-SHA coverage and UI accessibility checks. Ratchet behavior-focused coverage by module; do not optimize for percentage alone. |

### 14.1 Audit cautions

- The checked-in main tree contains no obvious `TODO`, `FIXME`, `todo!`, or `unimplemented!` source placeholders according to the dated audit and source search. That does not prove feature completeness.
- The `ci-local.sh` script conditionally skips `cargo deny`, `cargo audit`, and coverage when the tools are absent. Hosted CI installs them, but local “all gates passed” can be weaker depending on installed tools. Make required release tooling fail closed rather than silently skip.
- Existing release workflow has useful exact-SHA, asset inventory, package, and checksum validation. Preserve these invariants and extend them, rather than replacing the release flow wholesale.
- `docs/AUDIT-2026-11-26.md` is historical evidence; inspect the latest commit and rerun gates before making current claims.
- The source inspection did not establish every file-format, UI, or platform bug. Treat this list as concrete leads and add a focused regression test for each confirmed defect.

## 15. Implementation phases and exit criteria

### Phase 0 — Rebaseline and policy design

- Re-read `AGENTS.md`, `CLAUDE.md`, current `docs/TASKS.md`, `docs/PRODUCT_ROADMAP.md`, architecture docs, CI/release workflows, and current release metadata.
- Re-run baseline gates on exact main SHA. Record test counts, coverage, advisories, Windows/macOS/Linux/WASM build status, and browser test capability.
- Create ADRs for autonomous permissions/risk policy, channel manifests, feature flags/data compatibility, and rollback/updater.
- Define D1 schema, state transition table, API contracts, retention, metadata consent, and abuse controls.
- Create staging branch only after reconciling existing PRs, tags, and release source.

**Exit:** policies and exact baseline documented; no production secrets exposed to planned agent context.

### Phase 1 — Durable private intake MVP

- Implement Help menu + Bug/Feature/Change form on native and WASM.
- Implement consent/metadata preview, screenshot opt-in, validation, receipt, retry, offline fallback.
- Deploy Worker/D1; add private R2 upload path and deletion/retention.
- Add automated schema, rate limit, idempotency, privacy, and persistence tests.
- Keep AI and code generation off until intake is reliable.

**Exit:** ticket survives app restart/network failure; duplicate submissions are idempotent; consent behavior is proven; reporter data is never mirrored publicly by default.

### Phase 2 — Deterministic triage and issue synchronization

- Add state machine, event log, duplicate clusters, vote/principal controls, demand metrics, automatic dispositions.
- Add bug reproduction runner and safe fixture harness; no execution of arbitrary reporter-provided code on trusted infrastructure.
- Add scrubbed GitHub issue mirror and ticket-to-issue mapping.
- Add reporter status/notifications and needs-info expiry/reopen path.

**Exit:** all tickets end in a documented state with a reason; reproducible bugs have baseline proof and regression test before agent dispatch.

### Phase 3 — Coding-agent adapter and autonomous PRs

- Add provider-neutral agent adapter and benchmark; evaluate OpenCodifier plus fallback provider.
- Run model shadow-mode evaluation to establish reliability, then enable autonomous dispatch for eligible R0/R1 tasks.
- Implement short-lived scoped permissions, isolated worktrees/containers, tool/path allowlists, budgets, cancellation, and repair-loop limits.
- Agent creates ticket-specific PRs; no direct push to main or staging.
- Add hidden regression/acceptance tests where possible so an agent cannot trivially game visible tests.

**Exit:** benchmark thresholds met; agent-generated PRs pass strict required checks on exact SHA; unsafe permission attempts and prompt injection are contained; failures leave ticket traceability.

### Phase 4 — Protected staging and automatic integration

- Add protected `staging` branch and branch rules.
- Add required checks for staging and main; add `merge_group` event before enabling merge queue.
- Consider staging auto-merge only after maintainers explicitly approve the policy, branch protections, merge queue, rollback path, and all exact-SHA gates; otherwise retain required maintainer approval.
- Deploy staging to Cloudflare Pages with Access; isolate it from production APIs/data.
- Run browser smoke tests and verify source SHA and bundle identity after deployment.
- Publish immutable nightly assets and channel manifest only after the complete required platform artifact set has passed.

**Exit:** one ticket can move from accepted to isolated PR, automated merge, staging preview, and nightly artifact without maintainer intervention; every transition has source-linked receipts.

### Phase 5 — Test cohort, flags and demand-driven promotion

- Add opt-in nightly/dev channel choice and per-feature Experimental settings.
- Add feature flag manifest, disabled-by-default policy, persistent user choice, kill switches, compatibility guard.
- Add opt-in telemetry/feedback events without document content or covert identity.
- Add staged cohort controller, minimum sample/time rules, platform budgets and automatic hold/rollback thresholds.
- Validate with synthetic crash/error/telemetry failure injection.

**Exit:** opt-in cohort can install/test a candidate; success promotes automatically; regression disables feature or rolls back; stable users are not silently enrolled.

### Phase 6 — Stable rollout and ticket closure

- Promote the verified build through canary percentages and stable channel manifest.
- Record exact release assets, SHA-256, build manifest, attestations, platform and source SHA.
- Update ticket status to resolved only after availability reaches the required target.
- Send release/rollback notifications to opted-in reporters and test users.
- Retain immutable prior releases and a tested rollback pointer.

**Exit:** end-to-end automated cycle is demonstrated with a simulated bug fix and an opt-in experimental feature, including a deliberately injected failure and successful rollback.

### Phase 7 — WebOS package catalog and desktop updater

- Integrate WebOS host contract and versioned manifest/package resolution.
- Download complete WASM ZIP directly from GitHub Release, validate hashes/provenance, install versioned copy, atomic pointer switch, preserve prior known-good.
- Add desktop channels, platform package selection, staged install helper, health check, rollback, downgrade and version pin.
- Handle AppImage/portable ZIP/app bundles vs package-manager-managed installs separately.
- Test interrupted download, bad hash, wrong architecture, old WebOS ABI, offline, failed startup, corrupted previous version, downgrade and recovery.

**Exit:** WebOS can switch between two immutable release versions and restore previous version offline; desktop can safely update/downgrade per install type without touching documents.

### Phase 8 — Production hardening and operations

- Create support/automation dashboard and SLOs.
- Add abuse, prompt-injection, model-provider outage, GitHub outage, Worker/D1/R2 outage and credential rotation exercises.
- Validate accessibility and support flow at native and WASM sizes.
- Add incident timeline, automatic freeze, rollback and postmortem evidence.
- Revisit risk tiers and model/provider evaluation using actual operational evidence.

**Exit:** system can identify failure, stop rollout, recover to known-good, update ticket/reporters, and preserve diagnostic evidence without human intervention for standard incidents.

## 16. End-to-end acceptance scenarios

The implementation is not complete until automated end-to-end tests prove the following:

1. **Verified bug:** submit a reproducible bug; D1 creates exactly one ticket; baseline repro fails; agent adds regression test and fix; PR gates pass; merge queue merges to staging; Pages smoke passes; opted-in nightly validates; staged rollout reaches stable; ticket becomes resolved with release ID.
2. **False report / cannot reproduce:** ticket requests missing details; no agent receives an unsafe task; expiry closes it as unable-to-reproduce with a reason; new user evidence reopens the ticket.
3. **Duplicate:** multiple reports collapse to one engineering issue while each reporter keeps a private ticket receipt and receives appropriate notifications.
4. **Feature request:** demand scoring identifies candidate; agent builds isolated opt-in feature; Experimental is off for all users until enabled; usage/feedback is measured only from consenting users.
5. **Feature regression:** synthetic failure causes kill switch and/or channel rollback; the app restarts on last-known-good without losing user documents; tickets and cohort are notified.
6. **OpenCodifier outage or poor output:** adapter circuit breaker activates; no malformed patch or unvalidated action advances; budget and ticket state remain correct.
7. **GitHub/Cloudflare outage:** intake remains durable; queued work resumes idempotently; no duplicate PR, release, or deployment is created.
8. **WebOS install:** resolves stable manifest, downloads exact release asset, verifies hash/provenance, launches WASM through host contract, upgrades, pins older version, and rolls back while offline.
9. **Desktop install:** stable-to-nightly switch requires explicit choice; updates correct platform artifact; verifies; health-checks; rolls back after injected startup failure; works for portable and package-managed distributions.
10. **Privacy:** metadata opt-out, screenshot consent, deletion, short-lived attachment access, log redaction, and absence of document/clipboard capture are verified.
11. **Release identity:** every staged/nightly/stable deployment maps to the exact source SHA and immutable artifact hashes; no rebuild is silently substituted under an existing release identity.
12. **Policy bypass:** agent tries to edit its own policy, remove a required test, change feature flag default, reach main, exfiltrate a secret, or use a ticket prompt injection as instructions; all attempts are denied and audited.

## 17. First-run checklist for implementation agent

- [ ] Check out this branch and preserve the existing planning commits.
- [ ] Read `AGENTS.md`, `CLAUDE.md`, `docs/TASKS.md`, `docs/PRODUCT_ROADMAP.md`, architecture docs, current release/tag state, `.github/workflows/ci.yml`, `.github/workflows/release.yml`, and `scripts/ci-local.sh`.
- [ ] Reconcile current main/staging/PRs/releases; do not overwrite or reset unrelated work.
- [ ] Re-run exact-SHA baseline gates and update the audit findings with verified status and links.
- [ ] Implement Phase 0–2 before enabling coding-agent write access.
- [ ] Build and evaluate the provider-neutral agent adapter; OpenCodifier is not assumed reliable without evidence.
- [ ] Implement protected branch/ruleset/merge queue and exact-SHA gates before auto-merge.
- [ ] Keep stable deployment and production secrets inaccessible to implementation agents.
- [ ] Implement staging, opt-in nightly, cohort policy, feature flags, kill switch, and rollback before any automatic stable promotion.
- [ ] Implement complete WASM package manifest for WebOS and platform-specific desktop updater paths.
- [ ] Add failure injection and end-to-end scenarios before marking phases complete.
- [ ] Update `docs/TASKS.md`, roadmap, architecture/ADR, privacy policy, updater docs, channel docs, release receipts, and progress/audit evidence.
- [ ] Open a PR to `main` from this handoff branch only when the plan/docs are complete. The eventual implementation work should be separately traceable and should not be conflated with this documentation change.

## 18. Reference material

### Repository evidence
- `docs/TASKS.md` — canonical active work ledger.
- `docs/PRODUCT_ROADMAP.md` — product direction and user-request traceability.
- `docs/AUDIT-2026-11-26.md` — historical audit and CI receipts; rerun before treating metrics as current.
- `docs/research/quality-gap-analysis.md` — test/architecture gap analysis.
- `.github/workflows/ci.yml` and `.github/workflows/release.yml` — existing build/test/release baseline.
- `app/src/app.rs` — current update check and AI provider call.
- `app/src/formats.rs` — current file format dispatch and save path.

### External design references
- Matt Pocock triage skill: https://github.com/mattpocock/skills/tree/main/skills/engineering/triage
- Cloudflare Pages preview deployments: https://developers.cloudflare.com/pages/configuration/preview-deployments/
- Cloudflare Pages branch controls: https://developers.cloudflare.com/pages/configuration/branch-build-controls/
- Cloudflare R2 presigned URLs: https://developers.cloudflare.com/r2/api/s3/presigned-urls/
- GitHub merge queues: https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/configuring-pull-request-merges/managing-a-merge-queue
- GitHub artifact attestations: https://docs.github.com/en/actions/concepts/security/artifact-attestations

---

## 19. Definition of done for this plan

This plan is complete when implementation work can be divided into bounded, testable tasks and the autonomous policy is unambiguous. The actual system is complete only when the end-to-end scenarios in §16 pass and every production rollout, channel pointer, feature flag, and rollback is tied to machine-verifiable evidence.

**Do not mark support intake complete when only the form exists. Do not mark autonomous delivery complete when only an AI-generated PR exists. Do not mark a release safe when it only compiles.**
