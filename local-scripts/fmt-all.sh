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
#   local-scripts/fmt-all.sh          format in place
#   local-scripts/fmt-all.sh --check  fail loudly if anything is unformatted
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
# THE WORKSPACE LIST COMES FROM THE REPOSITORY, NOT THE FILESYSTEM.
# `find` answers "what is on disk", which in a working checkout is not
# the same question: this repo is worked on through agent worktrees
# under `.claude/worktrees/`, each a full checkout with its own
# Cargo.lock, and `find` handed every one of them to `cargo fmt --all`
# — reformatting four other lanes' trees in place, from a script whose
# subject is this one. `git ls-files` cannot pick up an untracked tree,
# and it stays derived, which is the property the header claims.
done < <(git ls-files -- '*Cargo.lock' | sort)

if [ "$fail" -ne 0 ]; then
  echo "fmt-all: run local-scripts/fmt-all.sh (no --check) to fix, then re-push" >&2
  exit 1
fi
