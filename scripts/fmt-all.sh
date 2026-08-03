#!/usr/bin/env bash
# fmt-all.sh — rustfmt EVERY workspace in the repo, not just the root.
#
# `cargo fmt --all` formats one workspace; this repo has several
# (root, demos/tour, interval-transcendentals, tools/k-lint — and the
# list is DISCOVERED, not hardcoded, so a new sub-workspace is covered
# the day its Cargo.lock lands). Two 15-minute gate round-trips were
# spent on a sub-workspace fmt miss (2026-08-03, PR #166) before this
# script existed; it runs in ~2 s.
#
#   scripts/fmt-all.sh          format in place
#   scripts/fmt-all.sh --check  fail loudly if anything is unformatted
#                               (the pre-push / CI-mirror mode)
set -euo pipefail
cd "$(dirname "$0")/.."

mode=()
[ "${1:-}" = "--check" ] && mode=(--check)

fail=0
while IFS= read -r lock; do
  ws=$(dirname "$lock")
  if ! (cd "$ws" && cargo fmt --all -- "${mode[@]}"); then
    echo "fmt-all: UNFORMATTED (or fmt error) in workspace: $ws" >&2
    fail=1
  fi
done < <(find . -name Cargo.lock -not -path '*/target/*' -not -path '*/.git/*' | sort)

if [ "$fail" -ne 0 ]; then
  echo "fmt-all: run scripts/fmt-all.sh (no --check) to fix, then re-push" >&2
  exit 1
fi
