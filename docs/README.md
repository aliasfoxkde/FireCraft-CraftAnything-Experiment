# FireCraft Documentation

Start with the level of detail you need. The root `README.md` is the product overview; this page
is the map for deeper material.

## I want to…

| Goal | Read |
|------|------|
| Build, test, or package a release | [Build & Release](build-release.md) |
| Understand the module layout and design seams | [Architecture](architecture.md) |
| Work on the infinite / god-view canvas | [Infinite Canvas](features/infinite-canvas.md) |
| Work on Find & Replace (including colors) | [Find & Replace](features/find-replace.md) |
| Work on Open / Save / Export | [File Browser](features/file-browser.md) |
| See the full feature inventory | [Feature Surface](features/README.md) |
| Track remaining work | [Roadmap](../ROADMAP.md) |
| See changes by release | [Changelog](../CHANGELOG.md) |

## Conventions

- **Domain terms** are defined in [GLOSSARY.md](../GLOSSARY.md). Use those names in code, UI copy,
  issues, and docs.
- **Architecture decisions** live in [docs/adr/](adr/). Do not silently reverse an ADR; add a new
  one that explains the change.
- **Feature notes** describe behavior and invariants, not implementation diaries.
- **Roadmap** is the single checklist. Do not duplicate long task lists in feature docs.

## Documentation rules

1. The README answers “what is this and how do I start?”
2. This index answers “where do I read next?”
3. Feature pages answer “how does this behave, and what must not regress?”
4. Architecture pages answer “where does this live, and why is the seam here?”
5. ADRs answer “why did we choose this and what would reversing it cost?”

When a page grows past roughly 300 lines, split it by user goal rather than appending another
section.
