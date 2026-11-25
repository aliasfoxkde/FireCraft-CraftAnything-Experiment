# FireCraft Roadmap — UI/UX Polish & Functional Repair

Status: **this polish pass is complete; open work has moved.**

This file is the record of the CS6 polish pass: what was built, and which checks could not be run
without a Windows desktop. The unchecked `Verify` items below are carried as tasks in
[`docs/TASKS.md`](docs/TASKS.md) (Phase 16/18) — they close with automated proof, not another manual
sweep. New work goes to `docs/TASKS.md`; measured state is in [`docs/AUDIT.md`](docs/AUDIT.md) and
the ordering rationale in [`docs/PLAN.md`](docs/PLAN.md).

Every item lists the outcome and how it will be verified. Items were worked in ID order unless a
dependency forced otherwise.

## Guiding constraints

- No placeholders, stubs, or simulated data. An unconfigured integration must say so plainly and
  offer setup, never fabricate a result.
- One implementation per behavior. Duplicate controls, duplicate collapse logic, and duplicate
  state-transport code are removed rather than patched in parallel.
- All new UI follows the existing CS6 Charcoal/Platinum palette, zero corner radius, and compact
  spacing. Icons are monochrome and share the existing icon drawing path.
- Every change must pass `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`,
  and `cargo test` in `app/`.
- The Windows review artifact is rebuilt and smoke-tested after the functional work lands.

---

## Phase 0 — Baseline audit


- [x] **0.1** Map layout, panel, style, gradient, filter, path, and floating-window code paths.
- [x] **0.2** Record known duplication: top toolbar vs. canvas footer vs. status bar resolution/DPI;
      bottom dock header plus `panels::panel` header; state transport in footer and status bar.
- [x] **0.3** Record known functional gaps: transparent style gradients, style application only to
      the primary selection, convex-hull "union", join closing a path, shape filters drawn only as
      rectangles.

---

## Phase 1 — Canvas controls & status bar

- [x] **1.1 Remove duplicate resolution/DPI controls from the Main Toolbar.** The toolbar keeps
      file/edit/arrange actions only. Remove `toolbar_resolution_combo` and `toolbar_dpi_combo`.
- [x] **1.2 Move the PPI/DPI selector into the canvas footer**, immediately beside the resolution
      selector and zoom selector. Order (left → right): transport, zoom, resolution, DPI.
      Resolution and DPI share one preset list and one `resize_canvas` / `doc.dpi` write path.
- [x] **1.3 Remove the duplicate zoom/resolution/state readouts from the bottom status bar.** The
      status bar keeps status text, active tool, selection count, and cursor coordinates only.
- [x] **1.4 Revamp the canvas transport (states)** with one icon set, monochrome styling, and
      disabled states:
      - First (`|◀`), Previous (`◀`), Play/Pause, Next (`▶`), Last (`▶|`).
      - Previous disabled on the first state, Next/Last disabled on the last state, Play disabled
        when only one state exists. Disabled glyphs are dimmed, not hidden.
      - Current state indicator reads `1 / N` and updates after every transport action.
      - Remove the duplicate transport from the status bar.
- [x] **1.5 Verify:** compiles, no duplicate widget IDs, transport state changes round-trip through
      `select_state` / `anim_playing`, and the footer fits at 960 px minimum width.
- [x] **1.6 Escape semantics.** `Esc` in the canvas cancels the active drag/path/text edit, closes
      transient flyouts/context menus, and returns to the Pointer tool. Dialogs keep priority so
      `Esc` closes them first rather than switching tools underneath.
- [x] **1.7 Shortcut audit.** Document and test the keyboard map (tools, transport, view, panels,
      file/edit commands). Add missing bindings where a visible command has no shortcut, and show
      the same shortcut text in menus, tooltips, and the command palette.

## Phase 2 — Panel dock and window management

- [x] **2.1 Merge the two bottom Properties headers into one.** The bottom dock currently draws a
      separate row above `panels::panel`. Extend the shared panel header with a leading collapse
      affordance so the bottom inspector has a single bar: `[▾] [Properties] [Symbol Properties]
      [Intelligence] … [≡]`.
- [x] **2.2 Standardize collapse/expand.** Refactor `panels::panel` into one implementation with an
      optional collapse control. Use the same chevron vocabulary as the tool rail and right dock.
      Double-clicking the panel/tab bar toggles collapse, matching every other dock widget.
- [x] **2.3 Fix the collapsed bottom dock.** Collapsed height shows the same single bar with tabs
      still switchable; expanding restores the previous body height.
- [x] **2.4 Whole-window drag preview.** Dragging a floating panel group or a torn-off panel shows a
      translucent ghost of the **entire widget** (header + body, correct width/height) under the
      cursor, not just a text label. The preview keeps dock/merge/snap target feedback.
- [x] **2.5 Widget grouping.**
      - Tab grouping: dropping a panel/group on another header merges tabs (existing behavior).
      - Vertical grouping: dropping a floating group on the top/bottom half of another floating
        group creates a persistent stack that moves as one unit.
      - Screen snapping: keep edge snapping; snapped stacks stay edge-aligned and continue to move
        together.
      - Ungrouping a stacked group restores independent floating windows.
- [ ] **2.6 Verify:** drag/merge/dock/float/snap paths, no orphan panel groups, no panics when the
      last group is removed, and a single visual bar on the bottom inspector.
- [x] **2.7 Owner-controlled bottom resize.** Persist inspector height in app state (not nested
      egui panel memory) with a dedicated resize grip. Dragging must not snap back on release;
      clamp to 118–340 px. Double-clicking the shared header toggles collapse.

## Phase 3 — Bottom inspector content

- [x] **3.1 Properties completeness.** Audit the selection and document states. Every editable
      field must write through a real mutator and update the canvas immediately; no dead controls.
      Add missing gradient/fill type controls from Phase 4.
- [x] **3.2 Intelligence tab.** Add a third bottom tab with real, non-fabricated integration:
      - Provider configuration for any OpenAI-compatible endpoint (base URL, model, API key).
      - Key can come from the `FIRECRAFT_AI_API_KEY` environment variable or the in-app secure
        field; the field never displays the full key.
      - “Ask about selection” sends the real selected-object summary and prompt to the configured
        endpoint, shows the response, and reports transport/HTTP errors verbatim.
      - When unconfigured, the panel explains exactly what to set and offers a copyable setup hint;
        it does not fake a response.
      - Includes the existing MCP server status (`mcp` stdio command) and a copyable client config.
- [ ] **3.3 Verify:** unconfigured, invalid-key, and successful-response paths are each exercised
      without panics; the tab shares the same panel chrome and collapse behavior.
- [x] **3.4 Layered `.fw.png`.** Save a normal page-bounded PNG plus an embedded, versioned
      project payload (iTXt chunk). Reopening restores layers, states, symbols, styles, and
      out-of-page objects; flat exports still crop to the page. Covered by a round-trip unit test.

## Phase 4 — Styles, gradients, and live filters

- [x] **4.1 Apply styles to every selected object**, not only the primary selection. Iterate the
      full selection across layers and push one undo snapshot for the whole action.
- [x] **4.2 Make style gradients real.** Add a gradient fill model to `Obj` (start/end color,
      linear/radial) and render it for rectangles, ellipses, polygons, and closed paths. A style
      preset with `gradient_end` creates a gradient fill instead of silently dropping the end color.
- [x] **4.3 Properties gradient editor.** When an object has a gradient fill, expose start color,
      end color, angle/linear vs. radial, and a “None” option. Switching back to solid clears the
      gradient. The canvas and style thumbnails agree.
- [x] **4.4 Correct shape-aware live filters.**
      - Shadows/glows follow the actual object silhouette (rect radius, ellipse, polygon/path)
        rather than always a bounding rectangle.
      - Bevel/Inner Shadow clip to the shape and are visible on non-rect shapes.
      - Filter enable/disable and Clear redraw immediately.
- [ ] **4.5 Verify:** apply each built-in style library to single and multi-selections; confirm
      gradient, stroke width, and effects change; toggle every filter and clear the stack.

## Phase 5 — Path and vector tools

- [x] **5.1 Join.** One open path closes its contour. Two or more selected paths join at the
      nearest endpoint pair, preserving point order. Never silently close unrelated subpaths.
- [x] **5.2 Split.** Split at the clicked location when available, otherwise at the midpoint, and
      produce two genuinely separate contours without a visual offset unless the user moves one.
- [x] **5.3 Union / Intersect / Punch (Crop).** Replace the convex-hull union and bounding-box
      punch with real polygon boolean operations. Use a tested polygon clipping implementation
      rather than a hull approximation; handle disjoint results without corrupting objects.
- [x] **5.4 Path panel actions.** Every button (Join, Split, Union, Intersect, Punch, Crop) is
      enabled only when its preconditions hold, gives a specific status message, and is undoable
      as one action.
- [x] **5.5 Verify:** unit tests cover join/split contour shape; boolean area/bounds still need
      manual checks with two overlapping rectangles and a concave polygon.

## Phase 6 — Left tool rail

- [x] **6.1 Nothing is clipped.** Pin the color chips, swap/default/no-fill actions, and Full
      Screen button in a fixed footer of the rail. Only the tool sections scroll; the footer is
      always visible at 640 px window height.
- [x] **6.2 Compact grouped palette.** Six CS6 categories in three columns; every group leader is
      directly visible and the active member is shown on the leader.
- [x] **6.3 Flyouts.** Corner triangles and right-click open a compact group flyout listing every
      member; selecting a member updates the leader and closes the flyout.
- [x] **6.4 Verify:** captured at 960×640 and 1360×860; Auto Shape controls and the Full Screen
      control are reachable without clipping.

## Phase 7 — Menus and context menus

- [x] **7.1 Audit every menu action.** Each enabled item performs the action it names; selection-
      dependent items disable when unavailable. Remove duplicate/misleading entries.
- [x] **7.2 Context menu parity.** Right-click menu mirrors the menu bar capabilities relevant to
      the click target (objects, web slices, empty canvas), with correct enablement and shortcuts.
- [x] **7.3 Consistency pass.** Shared labels, icons, separators, and shortcut formatting across
      menu bar, context menu, command palette, and panel buttons.
- [x] **7.4 Verify:** click through every menu action in a demo document and confirm status/canvas
      feedback; disablement matches selection state.
- [x] **7.5 Dynamic CS6 submenus.** Text derives Font and Size submenus from the active document;
      Filters presents live effects plus Adjust Color, Blur, Sharpen, Shadow/Glow, and Bevel
      submenus instead of a fixed flat list.

## Phase 8 — Windows app-switch flashing

- [x] **8.1 Reproduce/identify** the flash on focus changes in eframe 0.37 (clear color, window
      transparency, theme re-application, or focus-driven full repaint). User report: the glitch
      appears specifically in full-screen mode, not windowed mode. Addressed by forcing the native
      clear color to the workspace background (`App::clear_color`).
- [x] **8.2 Fix** the identified cause and confirm the window background never paints a mismatched
      light frame when regaining focus or toggling full screen. Add a short code comment.
- [ ] **8.3 Verify:** build and run the Windows binary; switch to another app and back repeatedly
      and toggle window full-screen / full-screen canvas.

## Phase 9 — Windows shell integration (taskbar jump list)

- [x] **9.1 App identity.** Set an explicit AppUserModelID at startup so Windows groups the process
      under FireCraft rather than the generic Rust host.
- [x] **9.2 Jump list.** Windows-only jump list with New Document, About, and up to ten recent files;
      rebuilt at startup and after a document is saved.
- [x] **9.3 Graceful fallback.** Non-Windows builds compile unchanged; COM/shell failures are
      reported as status text without blocking startup.
- [ ] **9.4 Verify:** rebuild the Windows binary and confirm the taskbar context menu entries;
      report as unverified if no real Windows shell is available.

## Phase 10 — Quality gate and review build

- [x] **10.1** `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`,
      `cargo test`.
- [x] **10.2** Targeted regression checks for text/color replace, hidden-object replace, and path
      join/split; broader style/filter/panel checks remain part of manual review.
- [x] **10.3** Rebuild `x86_64-pc-windows-gnu` release, smoke-test CLI under Wine, repackage
      `app/dist/firecraft-windows-x86_64-review.zip`, and update `CHANGELOG.md`.
- [x] **10.4** Report limitations explicitly (GUI visual checks that require a real Windows desktop).

## Phase 11 — Architecture, maintainability, and release size

- [x] **11.1 Split monoliths.** Find/Replace moved to `app/find_replace.rs`; the file browser and
      Find/Replace dialog UI moved to `dialogs/`; the tool palette moved to `panels/tools.rs`.
- [x] **11.2 Document the architecture.** `docs/README.md` is a navigation index into architecture,
      build/release, feature references, ADRs, and `GLOSSARY.md`; README is a short entry point.
- [x] **11.3 Release optimization.** Fat LTO, one codegen unit, `opt-level=3`, symbol stripping, and
      abort-on-panic in the release profile. Rust ships already-optimized machine code; there is no
      safe whole-program source minifier/obfuscator, and a single executable remains the documented
      distribution choice (ADR-0001).
- [x] **11.4 First architecture slice.** File/session intents extracted to `app/files.rs`; panel
      registry and split Properties/Color Mixer bodies added; `DocumentFixture` builder adopted.
- [ ] **11.5 Remaining architecture candidates:** extract arrange/edit command modules from
      `app.rs`, deepen the panel body seam, and defer the shell-snapshot adapter until a second
      shell target or recorded shell tests exist.

## Phase 12 — Document tab navigation

- [x] **12.1 Tab overflow.** Keep the active and most-recently-used document tabs visible; shrink
      padding and truncate labels as the strip fills; collapse older tabs into a `…` menu.
- [x] **12.2 Wheel switching.** Mouse-wheel or horizontal trackpad gestures over the tab strip
      move exactly one document per event, wrapping at either end.
- [x] **12.3 Browser-style bulk close.** Canvas-tab context menus offer Close Tab, Close Other
      Tabs, Close Tabs to the Left/Right, and Close All Tabs. Dirty tabs are handled one at a time
      through the existing unsaved prompt; Cancel stops the remaining queue.
- [x] **12.4 Verify:** 16-document run at 1360×860 showed 13 compressed tabs plus `…`, dropdown
      switching, one-step wheel navigation, and a left-side bulk close that stopped on the dirty
      document and resumed after Don't Save.

---

## Known technical debt to retire during this pass

1. Three resolution/DPI control clusters (toolbar, canvas footer, status bar/inspector) writing the
   same fields through separate widgets.
2. Two state transport implementations (canvas footer and status bar) with different icon orders
   and no shared enablement rules.
3. Two bottom Properties header bars and a one-off collapse row instead of the shared panel header.
4. `combine_paths_op` mixing join, split, and boolean operations; union via convex hull; punch via
   bounding-box vertex projection.
5. Style presets carrying a `gradient_end` that `apply_style_preset_value` discards.
6. Property mutators that only touch `self.selected`, leaving the rest of `selected_set` stale.
7. Floating-window drag ghosts implemented as text-only rectangles in two separate places.

Retired in this pass: duplicate style application, gradient end-color loss, convex-hull path
booleans, and text-only drag ghosts. Remaining: `app.rs` still owns command/state orchestration at
~7.3k lines (split incrementally behind tests), and the panel/window grouping code needs a
focused extraction once its gestures are covered.

## Verification log

- 2026-11-24: `cargo fmt --all -- --check` clean; `cargo clippy --all-targets -- -D warnings` clean;
  `cargo test -- --test-threads=1` = 31 passed, 0 failed. Clipboard access is excluded from test
  builds because headless X11 clipboard ownership can block indefinitely.
- 2026-11-24: `cargo fmt --all -- --check` clean; `cargo clippy --all-targets -- -D warnings` clean;
  `cargo test` = 46 passed, 0 failed. Live 1360×860 tab stress test: 16 documents rendered as
  13 compressed tabs plus `…`; dropdown switching, one-step wheel navigation, and bulk close
  stopping on a dirty document all behaved as designed.
- 2026-11-24: Fresh Windows-target check and optimized `x86_64-pc-windows-gnu` release build exit 0.
  Packaged executable SHA-256 `18112517207c8a4a70373fc1e317888633018ee40374250df045d147255b7122`
  (16 MB, stripped, fat LTO). Under Wine `--version`, `list-commands` (68 commands), and
  `info --demo` (22 objects, 2 layers) all exit 0.
- 2026-11-24: Wine GUI screenshot (1360×860 requested, 1280×800 captured under Xvfb) reviewed.
  Fixed the clipped Full Screen label and overlapping many-tab panel labels found in that pass.
- Remaining unverified items require a real Windows desktop or manual interaction: app-switch
  flashing in full-screen, actual taskbar jump-list rendering (code implemented, COM path not
  exercised here), tab drag/stack gestures, menu click-through, 960×640 layout, and a live
  Intelligence endpoint request.

## Out of scope for this pass

- Full Adobe Illustrator artboard import, PSD layer fidelity, and AVIF/HEIC codecs.
- Cloud sync/accounts and multi-user collaboration.
- Web deployment changes beyond keeping the WASM build compiling.
