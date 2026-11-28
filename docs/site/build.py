#!/usr/bin/env python3
"""Render the repository's Markdown into the self-contained Pages documentation site."""

from __future__ import annotations

import argparse
import html
import json
import re
import shutil
from pathlib import Path
from urllib.parse import unquote

import bleach
import markdown


ROOT = Path(__file__).resolve().parents[2]
SITE = Path(__file__).resolve().parent
DEFAULT_OUTPUT = ROOT / "app" / "dist" / "wasm" / "documentation"

INCLUDE = ["README.md", "docs/**/*.md", ".github/CHANGELOG.md", ".github/CONTRIBUTING.md",
           ".github/SECURITY.md", ".github/CODE_OF_CONDUCT.md"]
ALLOWED_TAGS = set(bleach.sanitizer.ALLOWED_TAGS) | {
    "a", "blockquote", "br", "caption", "dd", "del", "details", "div", "dl", "dt",
    "h1", "h2", "h3", "h4", "h5", "h6", "hr", "img", "input", "li", "ol", "p",
    "pre", "span", "sup", "sub", "table", "tbody", "td", "th", "thead", "tr", "ul",
}
ALLOWED_ATTRIBUTES = {
    "*": ["id", "class"],
    "a": ["href", "title"],
    "img": ["src", "alt", "title", "width", "height"],
    "input": ["type", "checked", "disabled"],
    "td": ["align"],
    "th": ["align"],
}


def route_for(source: str) -> str:
    if source.startswith(".github/"):
        return f"project/{Path(source).stem.lower().replace('_', '-')}.html"
    return source[:-3] + ".html"


def title_for(source: str, body: str) -> str:
    for line in body.splitlines():
        if line.startswith("# "):
            return line[2:].strip()
    return Path(source).stem.replace("-", " ").replace("_", " ").title()


def excerpt_for(body: str) -> str:
    for line in body.splitlines():
        line = re.sub(r"^\s*(?:[#>*-]+\s*|\[[^]]+\]\([^)]*\))", "", line).strip()
        if line and not line.startswith("---"):
            return (line[:210].rsplit(" ", 1)[0] + "…") if len(line) > 210 else line
    return "Open this page for the complete guide and its related references."


def search_text(body: str) -> str:
    text = re.sub(r"!\[([^]]*)\]\([^)]*\)", r"\1", body)
    text = re.sub(r"\[([^]]+)\]\([^)]*\)", r"\1", text)
    text = re.sub(r"<[^>]*>", " ", text)
    text = re.sub(r"[`*_~>#|]", " ", text)
    return re.sub(r"\s+", " ", text).strip().lower()


def section_for(source: str) -> str:
    path = Path(source)
    parts = path.parts
    if source == "README.md":
        return "Product"
    if parts[0] == ".github":
        return "Project governance"
    if len(parts) > 1 and parts[1] == "tutorials":
        return "Tutorials"
    if len(parts) > 1 and parts[1] == "features":
        return "Feature guides"
    if len(parts) > 1 and parts[1] in {"adr", "ci", "research", "integration"}:
        return "Engineering"
    name = path.name.lower()
    if any(word in name for word in ("audit", "quality", "progress")):
        return "Audits and progress"
    if any(word in name for word in ("roadmap", "plan", "task", "improvement", "migration")):
        return "Roadmaps and plans"
    if name in {"glossary.md", "a11y.md", "instructions.md", "architecture.md"}:
        return "Reference"
    return "Project documentation"


def markdown_links_to_routes(markup: str, source: str, source_to_route: dict[str, str]) -> str:
    """Keep internal Markdown navigation inside the docs reader and preserve safe external links."""
    source_dir = Path(source).parent

    def replace(match: re.Match[str]) -> str:
        prefix, href, suffix = match.group(1), html.unescape(match.group(2)), match.group(3)
        path, marker, fragment = href.partition("#")
        if not path or re.match(r"^[a-zA-Z][a-zA-Z0-9+.-]*:", path) or path.startswith("/"):
            return match.group(0)
        if path.lower().endswith(".md"):
            candidate_path = (ROOT / source_dir / unquote(path)).resolve()
            try:
                candidate = candidate_path.relative_to(ROOT).as_posix()
            except ValueError:
                candidate = ""
            route = source_to_route.get(candidate)
            if route:
                href = f"#doc={route}"
                if marker and fragment:
                    href += f"&section={fragment}"
                return f'{prefix}{html.escape(href, quote=True)}{suffix}'
        return match.group(0)

    return re.sub(r'(<a\b[^>]*?\bhref=")([^"]*)("[^>]*>)', replace, markup, flags=re.IGNORECASE)


def discover_sources() -> list[Path]:
    found: set[Path] = set()
    for pattern in INCLUDE:
        if "**" in pattern:
            found.update(ROOT.glob(pattern))
        else:
            candidate = ROOT / pattern
            if candidate.is_file():
                found.add(candidate)
    return sorted(path for path in found if path.is_file() and not path.relative_to(ROOT).as_posix().startswith("docs/site/"))


def build(output: Path) -> int:
    sources = discover_sources()
    output.mkdir(parents=True, exist_ok=True)
    content_dir = output / "content"
    content_dir.mkdir(parents=True, exist_ok=True)

    records = []
    for path in sources:
        source = path.relative_to(ROOT).as_posix()
        route = route_for(source)
        body = path.read_text(encoding="utf-8")
        records.append({"source": source, "route": route, "title": title_for(source, body),
                        "excerpt": excerpt_for(body), "section": section_for(source),
                        "search": search_text(body)})

    route_map = {item["source"]: item["route"] for item in records}
    for item, path in zip(records, sources, strict=True):
        text = path.read_text(encoding="utf-8")
        parser = markdown.Markdown(extensions=["tables", "fenced_code", "toc", "sane_lists"])
        rendered = parser.convert(text)
        rendered = markdown_links_to_routes(rendered, item["source"], route_map)
        safe = bleach.clean(rendered, tags=ALLOWED_TAGS, attributes=ALLOWED_ATTRIBUTES,
                            protocols={"http", "https", "mailto"}, strip=True)
        destination = content_dir / item["route"]
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_text(safe + "\n", encoding="utf-8")

    (output / "catalog.json").write_text(json.dumps(records, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    for filename in ("index.html", "site.css", "site.js"):
        shutil.copy2(SITE / filename, output / filename)
    print(f"Rendered {len(records)} Markdown pages to {output}")
    return len(records)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", type=Path, default=DEFAULT_OUTPUT)
    args = parser.parse_args()
    print(f"Building docs from {ROOT}")
    build(args.output.resolve())


if __name__ == "__main__":
    main()
