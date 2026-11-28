# FireCraft Documentation

> **Hosted guide:** [Browse the FireCraft Documentation site](https://aliasfoxkde.github.io/FireCraft-CraftAnything-Experiment/documentation/)
> for searchable navigation, featured tutorials, and rendered articles. The same site is bundled
> with the WebAssembly editor at `/documentation/`; the editor stays at the Pages root.

This page is the documentation map. Start with a category, then follow its index to a focused guide,
tutorial, reference, or engineering detail. The root [README](../README.md) is the product overview.

## Product and releases

- [Product overview and downloads](../README.md) — what FireCraft is, supported surfaces, and the
  current release links.
- [Product roadmap](PRODUCT_ROADMAP.md) — current capabilities, planned work, future scope, and
  phase exit criteria.
- [Changelog](../.github/CHANGELOG.md) — changes grouped by release.
- [Windows 11 review checklist](testing/windows-11-review-v0.3.8.md) — interactive review steps for
  the current v0.3.8 candidate; it is not a substitute for the user's Windows acceptance.

## Tutorials

[Browse tutorials](tutorials/README.md) for guided outcomes, prerequisites, and related references.
The current set covers getting started, drawing/editing, pages/states, exporting, and keybindings.
Screenshots and full end-to-end coverage are being added through the
[workflow evidence process](DOCUMENTATION-AND-WORKFLOW-EVIDENCE.md); existing static UI captures
must not be mistaken for interaction proof.

## Feature guides

[Feature Surface](features/README.md) is the inventory and links to focused behavior pages:

- [Infinite canvas](features/infinite-canvas.md) — page bounds, off-page editing, and god-view.
- [Find & Replace](features/find-replace.md) — matching and replacing names, text, and colors.
- [File browser](features/file-browser.md) — open/save flow, navigation, filters, and current
  limitations.

The inventory distinguishes implemented behavior from items still under verification. Add a focused
feature guide when a feature has no canonical page; do not duplicate the same behavior across the
roadmap, tutorial, and audit.

## Reference

- [Keybindings](tutorials/keybindings.md) — current shortcut lookup table and focus notes.
- [GLOSSARY](GLOSSARY.md) — product terms used by the interface, code, and docs.
- [MCP and CLI feature inventory](features/README.md#automation) — command surfaces and automation
  entry points.

Complete menu, file-format, command-palette, update, and settings references are tracked in the
[documentation plan](DOCUMENTATION-AND-WORKFLOW-EVIDENCE.md#information-architecture); publish each
when its behavior has been checked against the current build.

## Troubleshooting and support

- [Accessibility checklist](A11Y.md) — current checks, limitations, and remaining audit scope.
- [Windows review steps](testing/windows-11-review-v0.3.8.md) — capture exact reproduction details
  and attach screenshots/sample documents when reporting a defect.
- [Bug report](../.github/ISSUE_TEMPLATE/bug_report.yml) and [feature request](../.github/ISSUE_TEMPLATE/feature_request.yml) — choose a focused issue template.
- [Security reports](../.github/SECURITY.md) and [Code of Conduct](../.github/CODE_OF_CONDUCT.md).

## Development and architecture

- [Architecture](architecture.md) — subsystem ownership and dependency direction.
- [Architecture review](ARCHITECTURE-REVIEW-2026-11-27.md) — ranked deepening recommendations.
- [Quality and WebOS roadmap snapshot](QUALITY-AND-WEBOS-ROADMAP-2026-11-27.md) — historical v0.3.2 audit and phased quality plan; current figures are in the dated audit.
- [WebOS host integration contract](WEBOS-HOST-INTEGRATION.md) — a draft future protocol; not implemented or a supported integration.
- [Support and engineering pipeline proposal](SUPPORT-FEEDBACK-PIPELINE-HANDOFF-2026-11-27.md) — future in-app intake and delivery design; not approved or implemented.
- [Build and release](build-release.md) — local development, packaging, and release workflow.
- [CI/CD patterns](research/ci-cd-patterns.md) and [local GitForge workflow](ci/gitforge-image.md).
- [Architecture decisions](adr/README.md) — accepted decisions and their trade-offs.
- [Contributor guide](../.github/CONTRIBUTING.md) and [agent instructions](../AGENTS.md).

## Planning and evidence

- [Active task list](TASKS.md) — the single active implementation checklist.
- [Current dated audit](AUDIT-2026-11-28.md) — candidate status, measured quality, verification receipts, and open limits.
- [Prior audit](AUDIT-2026-11-27.md) — historical v0.3.5 baseline and earlier findings.
- [Progress log](PROGRESS.md) — chronological change and release receipts.
- [Documentation and workflow evidence process](DOCUMENTATION-AND-WORKFLOW-EVIDENCE.md) — the
  information architecture, visual testing loop, screenshot rules, and staged completion plan.
- [Workflow verification catalog](workflows/README.md) — stable scenario IDs, existing proof,
  canonical docs, and the concrete sign-off needed for each priority workflow.
- [Workflow evidence receipt form](testing/workflow-evidence-template.md) — repeatable artifact,
  environment, result, screenshot, and documentation metadata for a real user-flow run.
- [Canvas tools and effects plan](CANVAS-TOOLS-AND-EFFECTS-PLAN.md) — tool correctness and proof order.
- [Reference project research](research/reference-projects.md),
  [Fireworks CS6 behavior audit](research/fireworks-cs6-behavior-audit.md), and
  [extension research](research/fireworks-extensions.md).
- [Legacy migration register](LEGACY-FIRECRAFT-MIGRATION-REGISTER.md) and
  [consolidation plan](PRIMARY-REPO-RESET-AND-CONSOLIDATION-PLAN.md).

## Documentation conventions

- **Feature pages** define behavior, invariants, limitations, and evidence links.
- **Tutorials** teach one tested outcome with screenshots from the actual application.
- **Reference pages** provide complete lookup tables and exact labels.
- **Architecture pages** explain where behavior belongs and why; ADRs record decisions.
- **Audits** state what was measured and what remains unverified. Coverage and release claims must
  point to generated receipts, not memory.
- **Tasks** are the only active checkbox list. Roadmaps describe scope and exit criteria, not daily
  execution.
- Keep each page focused; split by feature or reader question before adding another broad section.
