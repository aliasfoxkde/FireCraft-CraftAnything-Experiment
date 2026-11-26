#!/usr/bin/env python3
"""Check relative Markdown links without requiring network access."""

from __future__ import annotations

import re
import sys
from pathlib import Path
from urllib.parse import unquote, urlsplit

ROOT = Path(__file__).resolve().parent.parent
SKIP_DIRS = {".git", ".worktrees", "target", "node_modules", "dist"}
LINK = re.compile(r"!?\[[^\]]*\]\((?:<([^>]+)>|([^\s)]+))")


def markdown_files() -> list[Path]:
    return sorted(
        path
        for path in ROOT.rglob("*.md")
        if not any(part in SKIP_DIRS for part in path.relative_to(ROOT).parts)
    )


def main() -> int:
    problems: list[str] = []
    for source in markdown_files():
        text = source.read_text(encoding="utf-8")
        for match in LINK.finditer(text):
            raw = match.group(1) or match.group(2)
            target = urlsplit(raw)
            if target.scheme or target.netloc or not target.path:
                continue
            local = (source.parent / unquote(target.path)).resolve()
            if not local.exists():
                line = text.count("\n", 0, match.start()) + 1
                problems.append(f"{source.relative_to(ROOT)}:{line}: missing link target {raw}")

    if problems:
        print("Markdown link check failed:", file=sys.stderr)
        print("\n".join(problems), file=sys.stderr)
        return 1
    print(f"Markdown link check passed ({len(markdown_files())} files)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
