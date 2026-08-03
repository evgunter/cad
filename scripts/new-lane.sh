#!/usr/bin/env bash
# new-lane.sh — the standard way to create an agent lane clone.
#   scripts/new-lane.sh <lane-name> [branch-to-create]
# Clones into ~/.local/share/cad-work/<lane-name>/cad, activates the
# repo's committed hooks (core.hooksPath — git never auto-activates
# committed hooks, so THIS is the enforcement channel for fresh
# clones), and optionally creates the work branch.
set -euo pipefail
lane=${1:?usage: new-lane.sh <lane-name> [branch]}
dir="$HOME/.local/share/cad-work/$lane/cad"
[ -e "$dir" ] && { echo "new-lane: $dir already exists" >&2; exit 1; }
git clone git@github.com:evgunter/cad.git "$dir"
git -C "$dir" config core.hooksPath scripts/hooks
[ -n "${2:-}" ] && git -C "$dir" checkout -b "$2"
echo "lane ready: $dir (hooks active: pre-push fmt-all --check)"
