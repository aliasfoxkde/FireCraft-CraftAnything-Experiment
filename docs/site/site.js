const catalogPath = new URL("catalog.json", import.meta.url);
const contentRoot = new URL("content/", import.meta.url);
const repository = "https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment";
const sectionOrder = ["Tutorials", "Feature guides", "Product", "Reference", "Engineering",
  "Roadmaps and plans", "Audits and progress", "Project documentation", "Project governance"];

const featuredGuides = [
  {
    source: "docs/tutorials/getting-started.md",
    kind: "First steps",
    description: "Get oriented in the workspace, create a document, and find the references that help you keep going.",
    icon: '<svg viewBox="0 0 72 72" aria-hidden="true"><path d="M12 58h48M17 52l14-34h10l14 34M25 52l11-27 11 27M29 42h16"/><circle cx="36" cy="16" r="3"/></svg>',
  },
  {
    source: "docs/tutorials/drawing-and-editing.md",
    kind: "Make and edit",
    description: "Follow the drawing guide for shapes, bitmap marks, and paths, then keep the tool reference close at hand.",
    icon: '<svg viewBox="0 0 72 72" aria-hidden="true"><path d="m17 53 5-15 28-28 12 12-28 28-15 5ZM22 38l12 12M46 18l12 12M14 60h44"/><circle cx="50" cy="22" r="2"/></svg>',
  },
  {
    source: "docs/features/README.md",
    kind: "Feature reference",
    description: "Browse the feature inventory and its linked pages for canvas behavior, panels, files, tools, and automation.",
    icon: '<svg viewBox="0 0 72 72" aria-hidden="true"><rect x="15" y="13" width="42" height="46"/><path d="M24 25h24M24 34h24M24 43h16M24 52h20"/><path d="M51 48v11"/></svg>',
  },
  {
    source: "docs/tutorials/optimize-and-export.md",
    kind: "Finish and export",
    description: "Learn the current export route and find the file-format and workflow details alongside it.",
    icon: '<svg viewBox="0 0 72 72" aria-hidden="true"><path d="M36 12v32M24 32l12 12 12-12M16 48v11h40V48"/><path d="M23 20h8M41 20h8"/></svg>',
  },
];

const stage = document.querySelector("#slide-stage");
const dots = document.querySelector("#slide-dots");
const count = document.querySelector("#slide-count");
const carousel = document.querySelector("#featured-carousel");
let activeSlide = 0;
let pages = [];
let pageBySource = new Map();
let pageByRoute = new Map();

function renderSlide() {
  const slide = featuredGuides[activeSlide];
  const page = pageBySource.get(slide.source);
  if (!page) return;
  stage.innerHTML = `<section class="slide" aria-roledescription="slide" aria-label="${activeSlide + 1} of ${featuredGuides.length}">
    <div class="slide-symbol">${slide.icon}</div>
    <div class="slide-copy"><p class="slide-kind">${slide.kind}</p><h3>${escapeHtml(page.title)}</h3><p>${escapeHtml(slide.description)}</p></div>
    <a class="slide-link" href="#doc=${encodeURIComponent(page.source)}" data-doc="${escapeHtml(page.source)}">Open this guide <svg class="arrow-icon" viewBox="0 0 20 20" aria-hidden="true"><path d="M3 10h14m-6-6 6 6-6 6"/></svg></a>
  </section>`;
  [...dots.children].forEach((dot, index) => dot.setAttribute("aria-current", String(index === activeSlide)));
  count.textContent = `${String(activeSlide + 1).padStart(2, "0")} / ${String(featuredGuides.length).padStart(2, "0")}`;
}

function createCarouselControls() {
  featuredGuides.forEach((slide, index) => {
    const button = document.createElement("button");
    button.type = "button";
    button.className = "slide-dot";
    button.setAttribute("aria-label", `Show featured guide ${index + 1}: ${slide.kind}`);
    button.setAttribute("aria-current", String(index === activeSlide));
    button.addEventListener("click", () => { activeSlide = index; renderSlide(); });
    dots.append(button);
  });
}

function changeSlide(direction) {
  activeSlide = (activeSlide + direction + featuredGuides.length) % featuredGuides.length;
  renderSlide();
}

function escapeHtml(value) {
  return String(value).replace(/[&<>"']/g, (character) => ({
    "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;",
  })[character]);
}

function renderNavigation(query = "") {
  const normalized = query.trim().toLocaleLowerCase();
  const matches = pages.filter((page) => {
    const haystack = `${page.title} ${page.excerpt} ${page.source} ${page.section} ${page.search}`.toLocaleLowerCase();
    return normalized.split(/\s+/).filter(Boolean).every((token) => haystack.includes(token));
  });
  const groups = new Map();
  for (const page of matches) {
    if (!groups.has(page.section)) groups.set(page.section, []);
    groups.get(page.section).push(page);
  }

  const nav = document.querySelector("#doc-navigation");
  nav.replaceChildren();
  const orderedGroups = [...groups].sort((left, right) => sectionOrder.indexOf(left[0]) - sectionOrder.indexOf(right[0]));
  for (const [section, sectionPages] of orderedGroups) {
    const group = document.createElement("section");
    group.className = "nav-section";
    const heading = document.createElement("h3");
    heading.textContent = section;
    const list = document.createElement("ul");
    for (const page of sectionPages) {
      const item = document.createElement("li");
      const link = document.createElement("a");
      link.href = `#doc=${encodeURIComponent(page.source)}`;
      link.dataset.doc = page.source;
      link.textContent = page.title;
      link.title = page.excerpt;
      item.append(link);
      list.append(item);
    }
    group.append(heading, list);
    nav.append(group);
  }
  document.querySelector("#empty-search").hidden = matches.length > 0;
  document.querySelector("#page-total").textContent = `${matches.length} pages`;
  updateCurrentLink();
}

function updateCurrentLink() {
  const selected = new URLSearchParams(location.hash.slice(1)).get("doc");
  const current = pageBySource.get(selected) ?? pageByRoute.get(selected);
  document.querySelectorAll("[data-doc]").forEach((link) => {
    if (link.closest("#doc-navigation")) {
      if (current && link.dataset.doc === current.source) link.setAttribute("aria-current", "page");
      else link.removeAttribute("aria-current");
    }
  });
}

async function openPage(value, { updateHistory = false, section = null } = {}) {
  const page = pageBySource.get(value) ?? pageByRoute.get(value);
  if (!page) return;
  if (updateHistory) {
    const location = `#doc=${encodeURIComponent(page.source)}${section ? `&section=${encodeURIComponent(section)}` : ""}`;
    history.pushState(null, "", location);
  }
  updateCurrentLink();

  const articleContent = document.querySelector("#article-content");
  const loading = document.querySelector("#article-loading");
  const error = document.querySelector("#article-error");
  const heading = document.querySelector("#article-title");
  const source = document.querySelector("#article-source");
  document.querySelector("#article-section").textContent = page.section;
  heading.textContent = page.title;
  source.textContent = "Source Markdown ↗";
  source.href = `${repository}/blob/main/${page.source}`;
  source.title = `Open ${page.source} on GitHub`;
  articleContent.replaceChildren();
  articleContent.hidden = true;
  error.hidden = true;
  loading.hidden = false;
  try {
    const response = await fetch(new URL(page.route, contentRoot));
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    articleContent.innerHTML = await response.text();
    articleContent.hidden = false;
    if (section) {
      const target = [...articleContent.querySelectorAll("[id]")].find((element) => element.id === section);
      target?.scrollIntoView({ block: "start", behavior: matchMedia("(prefers-reduced-motion: reduce)").matches ? "auto" : "smooth" });
    }
    document.title = `${page.title} · FireCraft Documentation`;
  } catch {
    error.hidden = false;
    const sourceLink = document.createElement("a");
    sourceLink.href = `${repository}/blob/main/${page.source}`;
    sourceLink.target = "_blank";
    sourceLink.rel = "noopener noreferrer";
    sourceLink.textContent = "Open the source page on GitHub.";
    error.append(" ", sourceLink);
  } finally {
    loading.hidden = true;
  }
}

document.querySelector("#previous-slide").addEventListener("click", () => changeSlide(-1));
document.querySelector("#next-slide").addEventListener("click", () => changeSlide(1));
carousel.addEventListener("keydown", (event) => {
  if (event.key === "ArrowLeft") { event.preventDefault(); changeSlide(-1); }
  if (event.key === "ArrowRight") { event.preventDefault(); changeSlide(1); }
});
document.querySelector("#doc-search").addEventListener("input", (event) => renderNavigation(event.currentTarget.value));
document.addEventListener("click", (event) => {
  const link = event.target.closest("a[data-doc], a[href^='#doc=']");
  if (!link) return;
  event.preventDefault();
  const selected = link.dataset.doc ?? new URLSearchParams(link.hash.slice(1)).get("doc");
  const section = new URLSearchParams(link.hash.slice(1)).get("section");
  openPage(selected, { updateHistory: true, section });
});
document.addEventListener("keydown", (event) => {
  if (event.key !== "/" || event.ctrlKey || event.metaKey || event.altKey) return;
  const target = event.target;
  if (target instanceof HTMLElement && (target.isContentEditable || ["INPUT", "TEXTAREA", "SELECT"].includes(target.tagName))) return;
  event.preventDefault();
  document.querySelector("#doc-search").focus();
});
window.addEventListener("hashchange", () => {
  const params = new URLSearchParams(location.hash.slice(1));
  const selected = params.get("doc");
  if (selected) openPage(selected, { section: params.get("section") });
});
window.addEventListener("popstate", () => {
  const params = new URLSearchParams(location.hash.slice(1));
  const selected = params.get("doc");
  if (selected) openPage(selected, { section: params.get("section") });
});

try {
  const response = await fetch(catalogPath);
  if (!response.ok) throw new Error(`HTTP ${response.status}`);
  pages = await response.json();
  pageBySource = new Map(pages.map((page) => [page.source, page]));
  pageByRoute = new Map(pages.map((page) => [page.route, page]));
  createCarouselControls();
  renderNavigation();
  renderSlide();
  const selected = new URLSearchParams(location.hash.slice(1)).get("doc");
  if (selected) openPage(selected, { section: new URLSearchParams(location.hash.slice(1)).get("section") });
} catch (error) {
  document.querySelector("#article-error").hidden = false;
  document.querySelector("#article-error").textContent = `The documentation index could not load (${error.message}). Refresh the page or open the repository source.`;
}
