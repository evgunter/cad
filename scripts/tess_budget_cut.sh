#!/usr/bin/env bash
# Stamps a tessellation-budget sweep with the tree it was cut from:
# a `# tess-budget-cut: <commit> <date>` line above the CSV header,
# which `tools/tess-lint` reads and prints beside every verdict.
#
# Usage:
#   scripts/tess_budget_cut.sh <sweep.csv>
#
# WHY the sweep carries its provenance at all: the gate fails a scene
# the baseline has no rows for, and that finding has two readings —
# a scene the corpus gained in the PR being gated, and a scene the
# baseline was already outgrown by. Without the cut both read as
# "absent" and the second one, which is the decay #1038 named, is
# invisible. With it a reader compares the scene's own age against the
# cut and knows which they are looking at.
#
# The commit is DERIVED, never passed in, by one rule with two arms:
#
#   * The file is tracked and unmodified — its rows are the ones some
#     commit wrote, so the cut is THAT commit. This is the arm that
#     stamps a baseline already in the tree without re-cutting it.
#   * Otherwise — it was just written (a fresh sweep, or a baseline
#     the sweep has overwritten), so the cut is the tree that produced
#     it: HEAD, marked `-dirty` when the worktree carries uncommitted
#     changes, because that is exactly when the pair is least
#     trustworthy and most worth printing.
#
# Outside a git checkout there is no commit to record: the stamp is
# skipped with a warning on stderr, and the lint then says the
# baseline records no cut rather than pretending to one.
set -euo pipefail
cd "$(dirname "$0")/.."
root=$(pwd)

csv=${1:?usage: tess_budget_cut.sh <sweep.csv>}
case "$csv" in
  /*) ;;
  *) csv=$root/$csv ;;
esac

if ! git -C "$root" rev-parse --git-dir >/dev/null 2>&1; then
  echo "tess_budget_cut: not a git checkout — no cut recorded" >&2
  exit 0
fi

if git -C "$root" ls-files --error-unmatch "$csv" >/dev/null 2>&1 &&
   git -C "$root" diff --quiet HEAD -- "$csv"; then
  read -r commit date < <(git -C "$root" log -1 --format='%h %cI' -- "$csv")
else
  commit=$(git -C "$root" rev-parse --short=12 HEAD)
  date=$(git -C "$root" show -s --format=%cI HEAD)
  git -C "$root" diff --quiet HEAD -- || commit="$commit-dirty"
fi

if [ -z "${commit:-}" ] || [ -z "${date:-}" ]; then
  echo "tess_budget_cut: git named no commit for $csv — no cut recorded" >&2
  exit 0
fi

# Re-stamping replaces the line rather than stacking a second one:
# the lint reads exactly one, above the header.
tmp=$(mktemp)
{
  echo "# tess-budget-cut: $commit $date"
  grep -v '^# tess-budget-cut:' "$csv"
} > "$tmp"
mv "$tmp" "$csv"
echo "cut: $commit $date"
