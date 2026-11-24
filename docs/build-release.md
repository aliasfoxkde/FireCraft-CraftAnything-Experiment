# Build & Release

## Local development

```sh
cd app
cargo check
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test -- --test-threads=1
```

Use `--test-threads=1` because the editor tests construct a `FireCraft` context and some
clipboard/platform interactions are deliberately excluded from test builds.

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

## Windows cross-build

```sh
rustup target add x86_64-pc-windows-gnu
cd app
cargo build --release --target x86_64-pc-windows-gnu
```

The GNU cross-build is convenient on Linux. CI/release builds should use the MSVC target for the
most natural Windows runtime.

## Distribution layout

Keep build intermediates out of git. Assemble a user-facing bundle:

```text
dist/windows/
  firecraft.exe
  README.md
  LICENSE
  docs/
    README.md
    build-release.md
    features/...
```

Create the zip from that directory, not from `target/`. Verify the archive and record SHA-256
checksums for both the executable and the archive. A single executable is intentional: a creative
desktop app should not require users to install a private DLL/runtime directory. If the codebase
later grows into multiple binaries, split the Rust crate into a library plus thin `bin/` entry
points before shipping extra executables, and keep shared assets in a resource directory.

## Smoke tests

On Linux with Wine:

```sh
WINEARCH=win64 WINEPREFIX=/tmp/firecraft-wine \
  wine app/target/x86_64-pc-windows-gnu/release/firecraft.exe --version
wine app/target/.../firecraft.exe list-commands
wine app/target/.../firecraft.exe info --demo
```

GUI smoke tests may need a software OpenGL backend under Xvfb. Taskbar Jump List behavior requires
a real Windows Explorer shell and should be reported as unverified when it cannot be exercised.
