# ADR-0003: Defer AVIF/HEIC, PSD/AI interop, Bridge process integration and animation

- Status: accepted
- Date: 2026-11-25
- Decision owner: project owner (recorded here so the scope does not drift by accident)

## Context

`INSTRUCTIONS.md` lists CS6 features the product does not implement: AVIF/HEIC import, PSD/AI
interchange, a Bridge application, an Animate timeline with tweening and symbols, and video or
animated-GIF authoring. Each is a large surface with its own decoder, UI, and file formats.

The 0.3.0 pass is a quality, evidence and release-engineering pass. Adding codecs and timelines
would dilute that work and ship untested surface.

## Decision

1. **Not in this milestone.** AVIF/HEIC writing stays unsupported (opening explains why), PSD/AI
   stay a documented bridge-only import (export from the source app to PNG/SVG), Bridge remains an
   in-app file browser, and animation states stay frame-based without a timeline.
2. **No placeholder UI.** Every unsupported path returns an explicit message; nothing fabricates
   output.
3. **Revisit with an ADR per feature.** Each accepted feature needs its own ADR, roadmap section and
   exit criteria before implementation (TASKS 20.2/20.3).

## Consequences

- The feature matrix in `docs/features/README.md` must say "not supported", not "coming soon".
- A request to add one of these features starts with an ADR, not a pull request.
- `Format::writeable()` is the single gate: anything unsupported is rejected at save time with the
  list of working formats.
