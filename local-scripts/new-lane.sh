#!/usr/bin/env bash
# new-lane.sh — the standard way to create an agent lane clone.
#   local-scripts/new-lane.sh <lane-name> [branch-to-create]
# Clones into ~/.local/share/cad-work/<lane-name>/cad, activates the
# repo's committed hooks (core.hooksPath — git never auto-activates
# committed hooks, so THIS is the enforcement channel for fresh
# clones), and optionally creates the work branch.
set -euo pipefail
lane=${1:?usage: new-lane.sh <lane-name> [branch]}
case "$lane" in
  */*|~*) echo "new-lane: pass a bare lane NAME (got a path: '$lane') — the clone goes to ~/.local/share/cad-work/<name>/cad" >&2; exit 1;;
esac
dir="$HOME/.local/share/cad-work/$lane/cad"
[ -e "$dir" ] && { echo "new-lane: $dir already exists" >&2; exit 1; }
# The URL comes from the invoking checkout's `origin`, not from a literal.
# A hard-coded `git@github.com:` clone cannot run in an environment with no
# ssh binary and an https `origin` — which is every remote agent container —
# and the failure reads as "the standard way" being unavailable rather than
# as a wrong URL. Falling back keeps a detached invocation working.
url=$(git -C "$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)" \
        remote get-url origin 2>/dev/null || true)
git clone "${url:-git@github.com:evgunter/cad.git}" "$dir"
git -C "$dir" config core.hooksPath local-scripts/hooks
[ -n "${2:-}" ] && git -C "$dir" checkout -b "$2"
echo "lane ready: $dir (hooks active: pre-push fmt-all --check)"
