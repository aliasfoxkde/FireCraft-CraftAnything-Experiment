#!/usr/bin/env bash
# Print the single source of truth for the project version: app/Cargo.toml.
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
sed -n 's/^version = "\(.*\)"$/\1/p' "$root/app/Cargo.toml" | head -1
