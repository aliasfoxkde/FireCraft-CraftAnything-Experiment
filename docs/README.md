# FireCraft Documentation

Start with the level of detail you need. The root `README.md` is the product overview; this page
is the map for deeper material.

## I want to…

| Goal | Read |
|------|------|
| Review the current v0.3.7 Windows build on Windows 11 | [Windows 11 review checklist](testing/windows-11-review-v0.3.7.md) |
| Review the v0.3.5 Windows build | [Archived Windows 11 checklist](testing/windows-11-review-v0.3.5.md) |
| Review the v0.3.3 Windows build | [Archived Windows 11 checklist](testing/windows-11-review.md) |
| See what is measured right now | [Audit](AUDIT.md) |
| Read the post-release audit, exact v0.3.5 receipts, coverage and gaps | [2026-11-27 audit](AUDIT-2026-11-27.md) |
| Read the 2026-11-26 repository audit | [Audit snapshot](AUDIT-2026-11-26.md) |
| Review UI/menu/dialog follow-up findings | [UI follow-up audit](AUDIT-2026-11-26.md#ui-and-workflow-follow-up--2026-11-26) |
| Review code quality and verification gaps | [Quality gap analysis](research/quality-gap-analysis.md) |
| Review scanner findings and triage | [Pattern scan triage](research/pattern-scan-triage.md) |
| Investigate immediate GitHub Actions failures | [GitHub Actions triage](research/github-actions-triage.md) |
| Understand the experiment-first repository reset | [Consolidation Plan](PRIMARY-REPO-RESET-AND-CONSOLIDATION-PLAN.md) · [Legacy Migration Register](LEGACY-FIRECRAFT-MIGRATION-REGISTER.md) · [Agent Handoff](AGENT-HANDOFF-2026-11-26.md) |
| Know what to work on next | [Tasks](TASKS.md) |
| Understand the phased plan and its exit criteria | [Plan](PLAN.md) |
| Read the public product roadmap, current vs planned | [Product Roadmap](PRODUCT_ROADMAP.md) |
| Review reference projects and feature research | [Reference project study](research/reference-projects.md) |
| Review Emberframe and BonBon tools, implementation seams and adoption plan | [Emberframe / BonBon audit](research/emberframe-bonbon-audit.md) |
| Track canvas tools, image rendering, gradients, filters and their phased proof plan | [Canvas tools and effects plan](CANVAS-TOOLS-AND-EFFECTS-PLAN.md) |
| Compare current canvas behavior with documented Fireworks CS6 workflows | [Fireworks CS6 behavior audit](research/fireworks-cs6-behavior-audit.md) |
| Review Fireworks extensions and the community marketplace plan | [Fireworks extensions research](research/fireworks-extensions.md) |
| Review CI/CD patterns and adoption steps | [CI/CD patterns](research/ci-cd-patterns.md) |
| See requested Codex skill installations | [Agent skill installation record](research/agent-skills-installation.md) |
| Read the repository working agreement | [AGENTS.md](../AGENTS.md) · [Contributor guide](../.github/CONTRIBUTING.md) |
| See what changed and when | [Progress log](PROGRESS.md) |
| Build, test, or package a release | [Build & Release](build-release.md) |
| Understand the module layout and design seams | [Architecture](architecture.md) |
| Read the dated architecture review and extraction recommendations | [Architecture review](architecture-review-2026-11-26.md) |
| Work on the infinite / god-view canvas | [Infinite Canvas](features/infinite-canvas.md) |
| Work on Find & Replace (including colors) | [Find & Replace](features/find-replace.md) |
| Work on Open / Save / Export | [File Browser](features/file-browser.md) |
| See the full feature inventory | [Feature Surface](features/README.md) |
| Learn to use FireCraft (tutorials, keybindings) | [Tutorials](tutorials/README.md) |
| Accessibility checklist and limitations | [A11Y](A11Y.md) |
| Read the completed UI polish history | [Polish history](ROADMAP.md) |
| See changes by release | [Changelog](../.github/CHANGELOG.md) |
| Read the vocabulary and original product brief | [Glossary](GLOSSARY.md) · [Instructions](INSTRUCTIONS.md) |
| Review completed and planned improvements | [Improvements](IMPROVEMENTS.md) · [Roadmap](ROADMAP.md) |
| Review architecture opportunities and the implementation order | [Architecture review (2026-11-27)](ARCHITECTURE-REVIEW-2026-11-27.md) · [Architecture](architecture.md) · [Plan](PLAN.md) |
| Contribute or report a security issue | [Contributing](../.github/CONTRIBUTING.md) · [Security](../.github/SECURITY.md) · [Code of Conduct](../.github/CODE_OF_CONDUCT.md) |

## Conventions

- **Domain terms** are defined in [GLOSSARY.md](GLOSSARY.md). Use those names in code, UI copy,
  issues, and docs.
- **Architecture decisions** live in [docs/adr/](adr/). Do not silently reverse an ADR; add a new
  one that explains the change.
- **Feature notes** describe behavior and invariants, not implementation diaries.
- **`docs/TASKS.md` is the single active checklist.** `PRODUCT_ROADMAP.md` is public-facing scope
  and exit criteria; `ROADMAP.md` keeps completed polish history and its remaining manual notes.
  Do not start another checklist in a feature or research document.

## Documentation rules

1. The README answers “what is this and how do I start?”
2. This index answers “where do I read next?”
3. Feature pages answer “how does this behave, and what must not regress?”
4. Architecture pages answer “where does this live, and why is the seam here?”
5. ADRs answer “why did we choose this and what would reversing it cost?”
6. The audit answers “what is true today, and what is only claimed?” — numbers must come from a
   command, never from memory.
7. The plan answers “in what order, and what proves each phase done?”; tasks are the checkboxes.

When a page grows past roughly 300 lines, split it by user goal rather than appending another
section.
