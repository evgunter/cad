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
git clone git@github.com:evgunter/cad.git "$dir"
git -C "$dir" config core.hooksPath local-scripts/hooks
[ -n "${2:-}" ] && git -C "$dir" checkout -b "$2"
echo "lane ready: $dir (hooks active: pre-push fmt-all --check)"
