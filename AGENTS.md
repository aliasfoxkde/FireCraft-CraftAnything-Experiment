# Repository Agent Guide

## Instruction precedence

This file is the provider-neutral repository baseline. Read it together with
[CLAUDE.md](CLAUDE.md), which supplies FireCraft-specific invariants and commands.
When both cover a topic, follow the stricter applicable rule; project-specific behavior
in `CLAUDE.md` is authoritative for FireCraft implementation details.

## Repository facts

- FireCraft is a clean-room Rust/egui raster and vector graphics editor.
- The application crate and Cargo commands live in `app/`.
- Active work and checkboxes belong in [`docs/TASKS.md`](docs/TASKS.md); measured state
  belongs in [`docs/AUDIT.md`](docs/AUDIT.md); architecture belongs in
  [`docs/architecture.md`](docs/architecture.md) and [`docs/adr/`](docs/adr/).
- Keep the user-facing product roadmap in [`docs/PRODUCT_ROADMAP.md`](docs/PRODUCT_ROADMAP.md).
- Use [`docs/README.md`](docs/README.md) as the documentation map; preserve the directory layout:
  project/architecture/usage material in `docs/`, GitHub community health files and the generated
  changelog in `.github/`.
- GitHub contribution entry points live in `.github/CONTRIBUTING.md`, `.github/SECURITY.md`,
  `.github/CODE_OF_CONDUCT.md`, `.github/ISSUE_TEMPLATE/`, `.github/PULL_REQUEST_TEMPLATE.md`, and
  `.github/CODEOWNERS`.
- Do not copy Adobe code, artwork, proprietary assets, or third-party project code into this
  clean-room repository. Record third-party assets and their licenses in
  [`THIRD-PARTY-NOTICES.md`](THIRD-PARTY-NOTICES.md).

## Working rules

1. Inspect `git status`, the current branch, and relevant task/ADR docs before editing. Preserve
   all pre-existing and concurrent changes; do not reset, clean, or overwrite another worker's
   edits.
2. Use the repository's graph route for symbol/relationship questions when the graph server is
   available. Use `rg`/`rg --files` for docs, literals, configuration, or gaps in graph results.
3. Make changes at the narrowest behavioral seam. Preserve document, undo, export, accessibility,
   and platform invariants. Record a new ADR when a material architectural decision changes.
4. Do not introduce placeholders, stub implementations, simulated integrations, or fabricated
   results. In plans, label examples as examples and distinguish planned work from implemented work.
5. Add or update behavior tests when the user asks for verification or when the change requires
   regression coverage. Run formatting, lint, tests, and target checks appropriate to the change;
   report exact commands and any host/platform limitations.
6. Update the active task ledger and affected user/developer documentation in the same change.
   Keep historical audit snapshots dated; do not rewrite old measurements as current.
7. Do not claim coverage, WCAG conformance, format parity, platform support, release readiness, or
   successful deployment without current machine-readable or reproducible evidence. Define the
   measurement and record the tool, command, source revision, and result.
8. Keep CI fail-closed for release gates. Pin actions/dependencies where the project's policy
   requires it; validate artifact identity, checksums, platform matrix, and release metadata before
   publication.
9. Commit and push only when the user explicitly asks. A request to build or audit does not by
   itself authorize publishing binaries or deploying a public site.

## Finish each task with

- A concise summary of code, docs, tests, and artifacts changed.
- Exact verification results and known gaps.
- Updated task status and a concrete next action for anything still open.
