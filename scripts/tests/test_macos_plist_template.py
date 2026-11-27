#!/usr/bin/env python3
"""Validate the macOS bundle plist template without requiring macOS tools."""

from __future__ import annotations

import plistlib
import re
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SCRIPT = ROOT / "scripts" / "package-macos.sh"


class MacOSPlistTemplateTests(unittest.TestCase):
    def test_generated_info_plist_is_valid_and_has_no_external_entities(self) -> None:
        script = SCRIPT.read_text(encoding="utf-8")
        match = re.search(
            r"cat > \"\$app/Contents/Info\.plist\" <<PLIST\n(.*?)\nPLIST\n",
            script,
            re.DOTALL,
        )
        self.assertIsNotNone(match, "Info.plist heredoc not found")
        template = match.group(1)
        self.assertNotRegex(template, r"(?is)<!\s*(DOCTYPE|ENTITY)\b")

        rendered = template.replace("${version}", "0.0.0-test").replace(
            "${bundle_version}", "0.0.0"
        )
        info = plistlib.loads(rendered.encode("utf-8"))
        self.assertEqual(info["CFBundleIdentifier"], "io.github.aliasfoxkde.FireCraft")
        self.assertEqual(info["CFBundleShortVersionString"], "0.0.0-test")
        self.assertEqual(info["CFBundleVersion"], "0.0.0")


if __name__ == "__main__":
    unittest.main()
