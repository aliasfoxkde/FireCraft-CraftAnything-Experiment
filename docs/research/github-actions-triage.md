# GitHub Actions immediate-failure triage

**Inspected:** 2026-11-26 (GitHub API queried at 15:56 UTC)
**Repository:** [`aliasfoxkde/FireCraft-CraftAnything-Experiment`](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment)
**Branch / revision:** `main` / `dab457c233779a97d9b5b80f85642d16ed6dc663`

## Findings

| Observation | Evidence | What it establishes |
|---|---|---|
| The repository is public and not archived. | `GET /repos/aliasfoxkde/FireCraft-CraftAnything-Experiment` returned `visibility: public`, `archived: false`, and default branch `main`. The legacy `public` field was null; `visibility` is the meaningful field here. | Current visibility is public. This does not by itself prove an individual job can be provisioned. |
| GitHub Actions is enabled and all actions are allowed. | `GET /actions/permissions` returned `enabled: true`, `allowed_actions: all`. | Repository-level Actions enablement and allow policy are not blocking the workflow. |
| The latest CI run failed before any recorded step. | [Run 45792648147](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/actions/runs/45792648147), event `push`, created 2026-11-26 09:30:47Z, revision above. It contains three failed jobs, each with `steps: []`; each completed by 09:30:50Z. | No checkout, toolchain setup, build, test, or package step is recorded as having run. This is not evidence of a source-code or test failure. |
| The jobs have no assigned runner in the API record. | `GET /actions/runs/45792648147/attempts/1/jobs`: two `ubuntu-latest` jobs and one `windows-latest` job; each has `runner_id: 0`, empty runner name and runner group, and empty steps. | GitHub's job record contains no runner assignment. This is consistent with a failure in scheduling/provisioning or job setup, but does not identify which one. |
| The log archive is unavailable. | `GET /actions/jobs/139815287758/logs` returned HTTP 404 with Azure `BlobNotFound`; `gh run view --log` and `--log-failed` yielded no step output. | There is no retained job log to identify a more specific cause. A missing archive is not itself the cause. |
| The workflow requests GitHub-hosted runners. | Current `.github/workflows/ci.yml` uses `ubuntu-latest` for Linux gates and WASM, and `windows-latest` for Windows. | The workflow does not request a self-hosted runner. |
| No self-hosted runners are registered. | `GET /actions/runners` returned `total_count: 0`. | This is only the self-hosted runner inventory. It says nothing about GitHub-hosted runner availability. |

GitHub documents standard hosted runners as free and unlimited for public repositories, including `ubuntu-latest`, `windows-latest`, and supported macOS labels. This supports the expectation that public-repository workflows can use those hosted labels; it does **not** establish that a specific run was successfully queued or that all account, organization, policy, and service conditions were satisfied. See [GitHub-hosted runners reference](https://docs.github.com/en/actions/reference/runners/github-hosted-runners) and [Actions billing and usage](https://docs.github.com/en/actions/concepts/billing-and-usage).

## Verified facts versus hypotheses

**Verified:** current visibility is `public`; Actions is enabled; the latest recorded CI run and its three jobs failed almost immediately; jobs have no steps and no runner identity in the API; logs are unavailable; workflows request GitHub-hosted runner labels; no self-hosted runners are registered.

**Not established:** the exact time when visibility changed relative to each failed run; whether the jobs were rejected before assignment, failed in GitHub's runner provisioning service, or hit an account/organization policy or service incident; whether another later run succeeded. The metadata does not include an explanatory job message, and the log archive cannot be fetched. Treat scheduling/provisioning or an account/service restriction as hypotheses, not a confirmed diagnosis. The three-second duration and `runner_id: 0` make an in-workflow command failure unlikely, but are not enough to name the cause.

The latest runs listed during inspection were the paired CI and Build, Release & Deploy WASM runs at revisions `dab457c…` (09:30:47Z) and `ec1cafc…` (09:29:59Z); both workflows failed. This chronology does not show whether visibility had already changed at those times. Historical failures should not be represented as proof that public-repository hosted runners are unavailable now.

## Minimal diagnostic and remediation sequence

1. **Capture a fresh, non-release CI run after confirming public visibility.** A normal push or pull request is sufficient; do not create a release tag or grant write permissions just to diagnose runner startup. Record run URL, revision, and UTC time.
2. **Inspect job assignment before changing source.** Use `gh run view RUN_ID --json jobs,url,headSha` and `gh api repos/aliasfoxkde/FireCraft-CraftAnything-Experiment/actions/runs/RUN_ID/attempts/1/jobs`. Check whether jobs acquire a nonzero `runner_id`/runner name and nonempty steps. If they do, inspect the first failing step and logs, then fix the actual workflow or code failure.
3. **If the fresh run again has empty steps and no runner identity, inspect GitHub's run/check details and account/organization Actions policy and usage limits, then check GitHub Status for an Actions incident.** Confirm that Actions is enabled at applicable organization/account levels and that public-repository fork/push policy is not the relevant trigger. Preserve run IDs and exact API output for GitHub Support if metadata still gives no reason.
4. **Retry only after finding and addressing a concrete setting or service issue, or after GitHub confirms recovery.** Then verify the normal CI gates pass on the target revision before attempting a release workflow. Do not infer readiness from a queued job or from the repository being public alone.

No workflow, repository setting, run, or release was changed as part of this investigation.

## Follow-up run on the release-preparation PR

After opening PR [#7](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/pull/7), run [45856428176](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/actions/runs/45856428176) did obtain GitHub-hosted runners. Its WASM job passed. The quality job ran tests successfully (105 unit, 12 CLI, 4 hygiene, 10 MCP) and then failed at `scripts/check-docs.sh`, which found trailing Markdown hard-break spaces in seven newly tracked docs. These were removed in the working tree; the local docs gate now passes over 49 tracked documents. The Windows release-smoke job was still running when this follow-up was recorded. The next PR-head CI run is required to validate the whitespace fix and the complete job matrix.
