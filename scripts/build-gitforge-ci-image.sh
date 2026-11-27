#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd -P)"
image_tag="${FIRECRAFT_CI_IMAGE_TAG:-firecraft-ci-rust:3}"
lock_sha256="$(sha256sum "$repo_root/app/Cargo.lock" | cut -d ' ' -f 1)"
build_context="$(mktemp -d "${TMPDIR:-/tmp}/firecraft-ci-image.XXXXXX")"
trap 'rm -rf -- "$build_context"' EXIT

mkdir -p "$build_context/app/src"
cp "$repo_root/ci/firecraft-ci.Dockerfile" "$build_context/Dockerfile"
cp "$repo_root/app/Cargo.toml" "$repo_root/app/Cargo.lock" "$build_context/app/"
cp "$repo_root/app/src/main.rs" "$build_context/app/src/"

docker build \
  --pull \
  --platform linux/amd64 \
  --build-arg "CARGO_LOCK_SHA256=$lock_sha256" \
  --tag "$image_tag" \
  "$build_context"

docker image inspect "$image_tag" \
  --format 'image={{.Id}} tag={{index .RepoTags 0}} cargo-lock-sha256={{index .Config.Labels "org.firecraft.cargo-lock-sha256"}}'
