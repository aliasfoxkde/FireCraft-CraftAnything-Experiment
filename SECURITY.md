# Security policy

## Reporting a vulnerability

Report security issues privately to the maintainers (open a private security advisory on the
repository, or contact the maintainer listed on the project page). Do not file a public issue for a
vulnerability. Please include:

- affected version (`firecraft --version`) and platform,
- a minimal reproduction,
- impact assessment (data loss, code execution, information disclosure).

You can expect an acknowledgement within 7 days. Fixes ship in the next patch release.

## Scope

In scope:

- the desktop executable and WASM build,
- the MCP JSON-RPC server and CLI verbs,
- document parsing (PNG, JPEG, GIF, WebP, TIFF, BMP, ICO, SVG/HTML import/export),
- the Windows shell integration (AppUserModelID, Jump List).

Out of scope:

- vulnerabilities in third-party services the user connects to (e.g. an OpenAI-compatible endpoint),
- the Intelligence panel sending user-chosen content to an endpoint the user configured,
- builds produced by an unreviewed fork.

## Dependency advisories

`cargo deny` runs in CI (`deny.toml`). Accepted advisories are documented in that file with an
exposure note and an expiry date; an expired acceptance fails the gate. `cargo audit` runs in the
same pipeline.

## Supported versions

The latest release on `main` receives security fixes. Older versions are unsupported.
