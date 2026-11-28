# FireCraft documentation site brief

## Direction contract

**THESIS:** Make the documentation feel like a well-indexed workshop manual: readers can enter through a task, a feature, or a reference page and keep their place while they follow the detail. The default documentation portal that is only a link list is refused.

**OWN-WORLD:** Carry FireCraft's charcoal workspace and ember-orange action color into a quiet reading surface. Use crisp borders, restrained orange accents, legible system typography, and generous reading measure; content remains the visual focus.

**STORY:** A new user sees a small set of practical guides, chooses a route through the documentation, and opens complete articles without losing the navigation. Contributors can reach architecture, roadmap, workflow, and policy material from the same index.

**FIRST VIEWPORT:** A compact header links to the editor and source repository. Beneath it, the featured-guide carousel spans the reading width; the searchable documentation index and selected article begin directly below. The footer offers support, source, wiki, and release links.

**SIGNATURE INTERACTION:** Full-text search filters the page index as the reader types; choosing a page loads its rendered content in place, preserves a shareable URL fragment, and keeps the reader's navigation context.

**MOTION GRAMMAR:** Use only short, ease-out transitions for carousel movement and navigation state. No automatic carousel rotation; honor reduced-motion settings and keep every action keyboard accessible.

## Content and implementation constraints

- Source of truth is the Markdown in the repository; do not duplicate article text in the site.
- Render Markdown at release build time into static, sanitized HTML so the documentation works without client-side libraries or external CDNs.
- Publish the site under `/documentation/` in the same Pages artifact as the WASM editor. Preserve the editor at the Pages root.
- Label unverified workflows and historical audit snapshots as their source documents do.
- The featured carousel must include previous/next controls, direct navigation, keyboard operation, and a visible pause control if autoplay is ever introduced. Autoplay is disabled.
- Target WCAG 2.1 AAA where practical, meet WCAG AA text contrast at minimum, and verify responsive layouts and focus visibility. The chosen body, secondary, navigation, and carousel text colors are measured at 7:1 or higher in their paired surfaces; the large orange display heading measures above the 4.5:1 large-text threshold.
