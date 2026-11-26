# Agent skill installation record

**Installed:** 2026-11-26  
**Target:** `/home/mkinney/.codex/skills/` and Codex's local plugin cache  
**Purpose:** record the requested upstreams, immutable source revisions, overlap handling, and declared runtime requirements. This is a user-level tool setup record; the skills are not bundled with FireCraft.

## Installations

| Source | Revision reviewed/installed | Method | Result |
|---|---|---|---|
| [mattpocock/skills](https://github.com/mattpocock/skills) | `6f7f7831a625f040a8e06a0e7954826b87ce775a` | Codex plugin marketplace `mattpocock`, plugin `mattpocock-skills` | Installed and enabled, plugin version `1.3.2`. This is the upstream-recommended auto-updating plugin path and includes the engineering, productivity, and PR skills. |
| [t3dotgg/tstack](https://github.com/t3dotgg/tstack) | `11e37a74d0a6868c121cd0910ab2b3bd9873b92a` | Codex skill installer | Installed all 9 skills: `full-send`, `full-stop`, `council`, `sol-loop`, `file-pr`, `takeover`, `babysit-pr`, `modernize`, `leaving-pr-comments`. |
| [backnotprop/pstack](https://github.com/backnotprop/pstack) | `c81f1bf680803f9e3f0cade6adb62593f43fb61f` | Codex skill installer | Installed all 59 `skills/*` skills, plus three `automations/benny/skills/*` skills (`reproduce-and-fix-issues`, `setup-benny`, `triage-issue-reports`). |
| [cursor/plugins `pstack`](https://github.com/cursor/plugins/tree/main/pstack/skills) | `b7b93c36be9bcc36a74db438e0fd385d660bc559` | Codex skill installer, with alternate names | Its `.cursor-plugin` package was not accepted as a Codex marketplace manifest. Git tree comparison showed the 59 skills share names with the standalone mirror, but 15 `SKILL.md` blobs differ. Those 15 Cursor revisions were installed side-by-side as `cursor-pstack-*`; identical skills use the standalone Pstack copies. |

## Runtime notes

- The tstack README requires authenticated `gh` for its PR-related workflows and Python for `babysit-pr`'s helper. Its `council` and `sol-loop` skills need different-provider subagents or another compatible harness; those model routes are not guaranteed in every session.
- Pstack standalone skills are independent except for the dependencies declared in its README (for example `teach` uses `how` and `why`, and `architect` uses `arena`). Pstack's setup and support skills can guide later configuration; no configuration was changed as a side effect of installation.
- The Cursor Pstack repository was handled as skill source, not as a Codex plugin, because the Codex installer rejected its Cursor-only plugin manifest. The source comparison was by Git blob SHA for every `SKILL.md` path.
- Skills are instructions, not authorization. Repository instructions, the current user request, tool availability, and permission policies still govern whether a skill action is appropriate. In particular, an installed auto-merge workflow does not itself authorize merging or publishing.
- The Codex skill installer aborts on destination collisions. Existing skills were preserved; Cursor-only revisions received a `cursor-pstack-` prefix instead of replacing standalone versions.

## Local changes

No upstream skill content was edited after installation. The only local adaptation was renaming the 15 overlapping Cursor-snapshot variants to `cursor-pstack-*` to keep both revisions addressable. All other copied skill directories retain their upstream names and source instructions.
