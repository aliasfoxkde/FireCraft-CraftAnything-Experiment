# Brief

**Question:** What are the real numeric constants, formulas and rules behind Fireworks CS6's canvas defaults, zoom levels, tool defaults, undo limit, swatch palette and export defaults — and where in Fireworks.exe do they live?

**Purpose:** Recover Fireworks CS6's real defaults and rules from the original program and build FireCraft to match them.

**Targets:**
- C:\Program Files (x86)\Adobe\Adobe Fireworks CS6\Fireworks.exe
- C:\Repos\FireCraft-CraftAnything-Experiment\app

**Constraints:**
- Clean room is off; Deep mode is on and the original is on this machine.
- Only app/ may be changed; capture/ and reference/ are read-only.
- Every value written into MECHANICS.md must cite a function address from Fireworks.exe.

**Done when:** MECHANICS.md and mechanics.json hold values traced to Fireworks.exe function addresses, the build uses them, and the app builds and passes rebuild_check.
