# ADR 0001 — Ship one portable executable on Windows

- **Status:** Accepted
- **Date:** 2026-11-24

## Context

The request asked whether FireCraft should ship a folder of libraries and split binaries to reduce
the executable size and memory use. The app is a native Rust/egui desktop editor used by designers;
first-run friction and missing-runtime failures are costly.

## Decision

Ship a single `firecraft.exe` inside a distribution bundle that also contains user docs and the
license. Use the release profile to reduce size:

- whole-program LTO;
- one codegen unit;
- `panic = "abort"`;
- strip symbols.

Do not split Rust code into DLLs for this release. If multiple entry points become valuable, split
the crate into a **library** plus thin `bin/` targets first; that improves tests and reuse without
creating a runtime DLL dependency.

## Consequences

- Users can copy one executable and run it.
- The binary is larger than a multi-DLL layout might be, but install/support failures are lower.
- Cross-crate boundaries can still be introduced as Rust library modules/crates; they do not
  require shipping separate dynamic libraries.
- Obfuscation is limited to stripping symbols. Rust has no safe source-level minifier, and an
  external binary obfuscator would make crash reports worse without protecting meaningful secrets.

## Revisit when

- startup memory measurements show a specific library boundary pays off;
- a second shipping executable needs shared code;
- the app adopts a plugin system that requires stable dynamic loading.
