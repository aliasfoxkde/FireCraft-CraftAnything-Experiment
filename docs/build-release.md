# Build & Release

## Local development

```sh
cd app
cargo check
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

The full gate set — the same one CI runs — is one command from the repository root:

```sh
./scripts/ci-local.sh          # all gates
./scripts/ci-local.sh --quick  # fmt, clippy, tests, docs only
```

`ci-local.sh` is the definition of record; `.github/workflows/ci.yml` and `.gitforge.yml` only
invoke it. Tests may run in parallel; `--test-threads=1` is no longer required (it is still
accepted).

## Toolchain

`rust-toolchain.toml` pins the compiler (`1.100.0`), `rustfmt`, `clippy` and the
`wasm32-unknown-unknown` target, so a local run and CI use the same toolchain.

## Release profile

`app/Cargo.toml` ships a size- and memory-conscious release profile:

```toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
panic = "abort"
strip = "symbols"
```

What this does:

- **LTO + one codegen unit** removes cross-crate dead code and enables whole-program inlining.
- **`opt-level = 3`** keeps interactive rendering and bitmap filters fast; lowering to `s` saves a
  little disk space but costs CPU in filters and path operations.
- **`panic = "abort"`** removes unwinding tables/landing pads, reducing both binary size and
  per-thread memory.
- **`strip = "symbols"`** removes symbol names and debuginfo from the shipped executable.

Rust does not have a separate runtime source-minification or identifier-obfuscation stage. The
compiler already emits optimized machine code, and stripped symbols remove the useful names. Do
not add a JavaScript-style obfuscator; it would break backtraces without protecting anything
meaningful.

## Version

`app/Cargo.toml` is the only version source. `scripts/version.sh` prints it; bundle names,
`--version`, the About window and CI artifact names all derive from it. The release job refuses to
publish a tag that does not match `Cargo.toml`.

## Windows cross-build (Linux)

```sh
rustup target add x86_64-pc-windows-gnu
cd app
cargo build --release --target x86_64-pc-windows-gnu
cd ..
./scripts/package-windows.sh
```

The GNU cross-build is a developer convenience. CI and releases build the MSVC target on
`windows-latest`.

## Distribution layout

`scripts/package-windows.sh [exe]` assembles the bundle deterministically from a clean tree:

```text
dist/windows/
  firecraft.exe
  firecraft.exe.sha256
  VERSION
  BUILD-INFO.txt
  README.md
  LICENSE
  CHANGELOG.md
  GLOSSARY.md
  MECHANICS.md
  docs/
```

It writes `app/dist/firecraft-v<version>-windows-x86_64.zip` plus a `.sha256` file for the archive.
Internal planning documents (`AUDIT`, `PLAN`, `TASKS`, `PROGRESS`) are excluded. A single
executable is intentional: a creative desktop app should not require users to install a private
DLL/runtime directory.

## Multi-platform release artifacts

The tag workflow builds Windows x86_64 (MSVC), universal macOS (arm64 + x86_64), Linux x86_64,
and a WebAssembly web bundle from the same version-matched tag. Linux packaging is handled by
`scripts/package-linux.sh` and currently produces a portable tarball, AppImage, DEB, and Flatpak.
The DEB declares `libc6 >= 2.35` (the Ubuntu 22.04 runner baseline); older distributions are not
claimed as supported without native install/launch testing. The Flatpak currently requests
home-directory and network access because file access uses direct paths and the optional
Intelligence feature can contact its configured provider. Review these permissions before
installation and expect the sandbox to be broader than a portal-based package.
The Flatpak uses the Freedesktop 26.08 runtime/SDK branch.

`scripts/package-macos.sh` produces a universal `.app`, ZIP, and DMG. Packages remain unsigned and
unnotarized until Developer ID credentials and notarization are configured; macOS may block them
under the default Gatekeeper policy. The package includes a distribution note and SHA-256 values.
This limitation is not equivalent to native macOS runtime acceptance.

The WASM ZIP is built from the same tag and the workflow deploys that directory to GitHub Pages.
Cloudflare marketing-site deployment is a separate task and is not implied by the Pages deploy.
Artifact hashes and source-SHA metadata are verified before the GitHub Release is published.

## Command-line surface

| Flag / verb | Meaning |
|-------------|---------|
| `--version` | Print the version and exit. |
| `--help` | Print CLI help and exit. |
| `list-commands` | Print the command catalog (shared with the in-app palette). |
| `info <file>` \| `info --demo` | Print document metadata as JSON. |
| `export <file> --format png\|jpg\|svg\|html\|webp …` | Export a document. |
| `convert <in> <out>` | Convert between supported formats. |
| `batch <dir> --format … --out <dir>` | Convert a directory. |
| `apply <file> <command>` | Run one catalog command against a document. |
| `mcp [--demo]` | Serve MCP JSON-RPC 2.0 over stdio. |
| `--demo` | Open the showcase document. |
| `--new` | Start with an empty document. |
| `--splash` | Show the splash sequence. |
| `--about` | Open the About window. |
| `--tab <0-3>` | Select the top-level tab (Original, Preview, 2-Up, 4-Up). |
| `--size <WxH>` | Initial window size; clamped to the 960×640 minimum. |
| `--screenshot <path>` | Render one frame, save a PNG and exit (review capture). |
| `<path>` | Open a document at startup. |

## Smoke tests

`cargo test` covers the CLI verbs. To smoke-test a real binary, including the Windows one under
Wine:

```sh
WINEARCH=win64 WINEPREFIX=/tmp/firecraft-wine \
  wine app/target/x86_64-pc-windows-gnu/release/firecraft.exe --version
wine app/target/.../firecraft.exe list-commands
wine app/target/.../firecraft.exe info --demo
```

GUI smoke tests may need a software OpenGL backend under Xvfb. Taskbar Jump List behavior requires
a real Windows Explorer shell and is reported as unverified when it cannot be exercised.
