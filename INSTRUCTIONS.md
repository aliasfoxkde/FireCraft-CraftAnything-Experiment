\# INSTRUCTIONS



Working directory: "C:\\Repos\\FireCraft-CraftAnything-Experiment" (everything relative to it)

Source application: "C:\\Program Files (x86)\\Adobe\\Adobe Fireworks CS6"



I want the same app as a baseline with feature parity but also with support for higher resolutions, better memory management, modern features like image formats (AVIF, HEIC, TIFF, WebP, and so on), better integration support for Adobe Bridge, Illustrator (real vector support including but not limited to SVG, AI, exporting/importing into other programs, etc.), Photoshop (PSD), and so on. Support for Adobe Fireworks CS6 was dropped nearly a decade ago in 2013 and even installing and using it is difficult because the servers for auth and setup no longer exist, the old software is buggy on newer platforms, there are not UI ways to modify the config and extend functionality and so on. So I want a new envisioned version of Adobe Fireworks CS6 called "FireCraft" with all modern features, layout/design parity but with a dark theme and support for auto-detect of system theme (dark/light), and so on.



FireCraft Expanded notes:

\- Make sure branding is FireCraft and link help/docs to the FireCraft Wiki and repo:

https://github.com/aliasfoxkde/FireCraft/wiki

https://github.com/aliasfoxkde/FireCraft/blob/main/README.md

\- And the prior attempts were a failure as this version seems to be going much better, but there may be information worth extracting from it, such as MCP server, backend, CLI features, and so on? See: C:\\Repos\\FireCraft-RAE and C:\\Repos\\Fireworks and C:\\Repos\\FireCraft

\- Strengthen the branding and audit the current state of the application, and so on...

\- Cleanup the code base, audit with Aegis, and improve the code quality.

\- Add/extract Adobe Animate (formerly Adobe Flash Professional and Macromedia Flash, which Adobe also dropped support for) Timeline, Motion Tweening, Symbol System, Vector Art/support and manipulation (as well as dynamic features, including animation, etc.), and other useful features to be included with FireCraft.

\- Allow support for publishing/exporting to HTML5, WebGL, SVG, video, animated GIFs, and other formats, including but not limited to legacy formats and ensureg compatibility across desktop, mobile, and web environments... as well as other Adobe Creative Cloud applications like Photoshop and Illustrator, making it a versatile choice for animators, designers, and developers. 

\- The software should support a wide range of creation types, including (but not limited to):

&#x20; - Frame-by-frame and tweened animation for cartoons, ads, and motion graphics. 

&#x20; - Interactive content for websites, eLearning modules, and browser-based games using JavaScript or ActionScript. 

&#x20; - Character rigging and puppet design for efficient, reusable asset animation.

\- Audit the screenshots under `./screenshots` for visual references, menus, layout, windows, popups include "New Document", and so on.



Audit:

\- Validate that all functionality works and make notes to hand off to AI for refinement, etc.

\- I tested the build and light theme looks "okay" (3 out of 10 maybe), but not great and nothing seems to be functional yet.

&#x20; - I can't create a new document, history/path tabs don't do anything or change,

\- I would like there to be a transparent splash page/graphic, more like the original Adobe Fireworks CS6 has.



Update and expand the task list to be comprehensive, detailed and feature complete for transparency and to keep track of progress, changes, and so that you do not forget or lose track of tasks.



Make sure to commit and push changes up to the repository periodically to keep track of changes, history, and have the ability to restore, audit, and reference previous progress, etc.



Complete all tasks and recommended steps, if you have any, and fix all issues regardless of if they are pre-existing or not. Then determine gaps, areas of improvement, and next steps to create a comprehensive and detailed plan, broken down by phases, to work through systematically, step by step. Break down complex tasks into automatic tasks which are smaller, more actionable ones and keep mindful of the bigger picture and architecture.



Do deep research, analysis, critical audit, an honest assessment, and expand the plan. Review other projects for best practices for CI/CD/Actions, structure, documentation and so on, to ensure the highest code, documentation, and architectural structure. Create a task list for transparency, to keep track of progress, changes, and so on. Also, work through cleaning up this repository and reach 99% test, code, and documentation coverage as well as audit for best practices, strict linting rules, WGCA 2.1 AAA standards (for web app/front-ends, etc.), use the strictest linting guidelines, audit for code smells, validate all e2e tests pass, validate against CI/CD/Actions using GitForge locally, and reach the highest code quality. And utilize pattern scanning for immediate feedback using Aegis, to catch issues early and save time/effort later having to do complex/compounding debugging and troubleshooting steps. 



Do not use placeholders, stubs, simulated, or otherwise fake data unless asked explicitly to do so; planning documentation is where you would put any example code, ideas and so on related to development. Fix any and all blockers, pre-existing issues, and so on that you might run into; do not leave any work undone. Make sure to start/end by committing and pushing all changes to keep track of progress, history, and as backup. And create a GitHub release and deploy if applicable.


Git protocol (do this every time you finish a work chunk, not just at the end):

\- Commit AND push. A local commit alone is not enough: the backup only exists once it is on the remote. Run exactly this, from the repository root:

```
git add -A
git commit -m "type(scope): short description"
git push origin main
```

\- If the push fails, do not give up and do not skip it. Read the error, fix it, and push again in the same turn. A rejected push means the remote moved; pull with `git pull --rebase origin main` and push once more.

\- Never commit build output or anything under `target/`, `app/target/`, `app/scratch/`, `app/shots/`, or the Craft Anything runtime state (`build-activity.log`, `craft.json`, `feed.json`, `transcript.json`, `usage.json`). `.gitignore` already covers all of these; if `git status` ever shows one of them, `.gitignore` has regressed and fixing it is part of the task.

\- `app/mechanics.json` must keep the schema Craft Anything reads: `{"version": 1, "values": [{"name": ..., "group": ..., "value": <number|text|bool>, "unit": ..., "source": "function 0x..."}]}`. `version` is a number, `values` is an array, and every value carries a `source` or `"estimated": true`. `app/MECHANICS.md` and `app/mechanics.json` must agree.

\- The source application path is "C:\\Program Files (x86)\\Adobe\\Adobe Fireworks CS6\\Fireworks.exe" (SHA-256 ab9a267ffe6a9fd23f8f3f09446997c770effceeaae66babcb3f10d15433053f). Every value written into `app/MECHANICS.md` must cite the Fireworks.exe function address it was observed at.



