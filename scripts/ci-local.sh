#!/usr/bin/env bash
# Every gate CI runs, in the same order, preserving exit codes.
#
# Usage: scripts/ci-local.sh [--quick]
#   --quick  skip the slower cross-target and security jobs (fmt, clippy, tests, docs only)
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root/app"

quick=0
[ "${1:-}" = "--quick" ] && quick=1

run() {
  echo
  echo "==> $*"
  "$@"
}

run cargo fmt --all -- --check
run cargo clippy --all-targets --quiet -- -D warnings
run cargo test --quiet
run cargo test frame_time_with_500_objects_stays_within_budget_at_1920x1080 -- --ignored --nocapture
( cd "$root" && run ./scripts/check-docs.sh )
( cd "$root" && run ./scripts/module-size.sh )

if [ "$quick" -eq 0 ]; then
  run cargo check --quiet --target wasm32-unknown-unknown
  if command -v cargo-deny >/dev/null; then
    ( cd "$root" && run cargo deny --manifest-path app/Cargo.toml check )
  else
    echo "==> skip cargo deny (not installed)"
  fi
  if command -v cargo-audit >/dev/null; then
    run cargo audit
  else
    echo "==> skip cargo audit (not installed)"
  fi
  if command -v cargo-llvm-cov >/dev/null; then
    run "$root/scripts/coverage.sh"
  else
    echo "==> skip coverage (cargo-llvm-cov not installed)"
  fi
fi

echo
echo "ci-local: all gates passed"
