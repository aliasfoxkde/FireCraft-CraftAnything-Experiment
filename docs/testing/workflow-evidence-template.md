# Workflow evidence receipt

Copy this form for a real workflow run and save it as
`docs/testing/<workflow-id>-<date>.md`. Use the stable ID from the
[workflow catalog](../workflows/README.md). Keep one receipt per build and platform so a later
release cannot silently inherit an older pass.

## Identity

- Workflow ID and scenario revision:
- FireCraft version and Git commit:
- Artifact path and SHA-256:
- Run date/time and reviewer:
- OS, browser/runtime, GPU/render backend:
- Window or viewport size, display scale, theme:
- Test command or manual checklist revision:

## Scenario

- User outcome:
- Starting document/state and its provenance/license:
- Actions, including keyboard-only route:
- Expected result at each important transition:
- Undo/recovery path:
- Related requirements, tasks, issues, and pull requests:

## Result

- Outcome: `pass`, `fail`, or `partial`:
- Assertions completed and result:
- Console/runtime errors:
- Known limitations or platform differences:
- Reviewer notes for focus, contrast, clipping, scaling, and error recovery:

## Evidence

- Trace/log artifact and checksum:
- Screenshot paths, checksums, and alt text:
- Before/after defect evidence, if applicable:
- CI run or manual-session reference:

## Documentation follow-through

- Canonical feature guide updated:
- Tutorial updated from the passing run:
- Reference, keybindings, menu, or release notes updated:
- Open defects recorded in `docs/TASKS.md` or GitHub:
- Next acceptance step:

## Status decision

State why the workflow catalog status is unchanged or what evidence justifies changing it. A static
screenshot proves appearance only. A code test proves only the exercised code seam. Mark a workflow
fully accepted only after all required user actions on the named surface passed and the evidence
matches the artifact identity above.
