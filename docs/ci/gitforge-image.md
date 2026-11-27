# GitForge CI image

FireCraft's GitForge pipeline runs offline. It must not install Rust components,
targets, operating-system packages, or crates during a job. The image contains
Rust 1.100.0, `rustfmt`, Clippy, the `wasm32-unknown-unknown` target, native
Linux development libraries, and the exact registry packages in
`app/Cargo.lock`.

## Build and refresh

Build the image on the Docker host used by the GitForge runner:

```sh
./scripts/build-gitforge-ci-image.sh
```

The script builds `firecraft-ci-rust:3` by default. It uses a pinned
linux/amd64 Rust base image and a minimal context containing the app manifest,
lockfile, the real `src/main.rs` Cargo target marker, and Dockerfile. The source
file is only needed while Cargo resolves/fetches dependencies; the job mounts
the checkout over `/workspace`. The Cargo lockfile SHA is recorded as an image
label. After changing `app/Cargo.lock`, Rust toolchain, native build
dependencies, or WASM target, rebuild and validate the image before changing
the tag referenced by `.gitforge.yml`.

The runner must use the same Docker image store as the build command. If the
runner is moved to a different Docker host, publish the image to a registry
reachable from that runner and change the `.gitforge.yml` image reference to
that registry-qualified tag; a local tag on another host is not sufficient.

## Offline validation

From the repository root, validate the same gates the pipeline runs:

```sh
docker run --rm --network none \
  -v "$PWD:/workspace" -w /workspace \
  firecraft-ci-rust:3 ./scripts/ci-local.sh --quick
docker run --rm --network none \
  -v "$PWD:/workspace" -w /workspace/app \
  firecraft-ci-rust:3 cargo build --release --target wasm32-unknown-unknown
```

`CARGO_NET_OFFLINE=true` is set in the image. A new dependency missing from
the baked Cargo registry fails immediately; rebuild the image from the updated
lockfile instead of allowing a job to download dependencies at runtime.
