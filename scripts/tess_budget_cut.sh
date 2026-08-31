#!/usr/bin/env bash
# Stamps a tessellation-budget sweep with the tree it was cut from:
# a `# tess-budget-cut: <commit> <date>` line above the CSV header,
# which `tools/tess-lint` reads and prints beside every verdict.
#
# Usage:
#   scripts/tess_budget_cut.sh <sweep.csv>
#   scripts/tess_budget_cut.sh --selftest
#
# WHY the sweep carries its provenance at all: the gate fails a scene
# the baseline has no rows for, and that finding has two readings —
# a scene the corpus gained in the PR being gated, and a scene the
# baseline was already outgrown by. Without the cut both read as
# "absent" and the second one, which is the decay #1038 named, is
# invisible. With it a reader compares the scene's own age against the
# cut and knows which they are looking at.
#
# The commit is DERIVED, never passed in, by one rule with three arms:
#
#   * The file is tracked, unmodified, and ALREADY STAMPED — REFUSED.
#     Its rows have not moved, so its cut has not either, and by then
#     the commit that last wrote the file is the commit that wrote the
#     STAMP: re-stamping would walk the recorded cut forward past the
#     data it describes, and a scene added in between would read as
#     "arrived after the cut" when the truth is the opposite. That is
#     the exact inversion this record exists to prevent, so the answer
#     is a refusal rather than a fresher number.
#   * The file is tracked and unmodified and carries no valid stamp —
#     its rows are the ones some commit wrote, so the cut is THAT
#     commit. This is the arm that stamps a baseline already in the
#     tree without re-cutting it, and that repairs a stamp the lint
#     would refuse to read.
#   * Otherwise — it was just written (a fresh sweep, or a baseline the
#     sweep has overwritten), so the cut is the tree that produced it:
#     HEAD, marked `-dirty` when the worktree carries uncommitted
#     changes.
#
# `-dirty` IS THE ORDINARY READING ON A CI RUNNER and must not be taken
# for a signal there: every hosted job's first act is a step that
# deletes `local-scripts/` and `.claude/` from the checkout, so the tree
# a hosted sweep runs over genuinely is not its HEAD — and on a pull
# request the SHA it names is the ephemeral merge ref, not a commit on
# any branch. Both are honest, both are restated at the echo below, so
# that a log reader does not spend a minute on them.
#
# Outside a git checkout, and before a repository's first commit, there
# is no commit to record: the stamp is skipped with a warning on stderr,
# and the lint then says the baseline records no cut rather than
# pretending to one.
set -euo pipefail
cd "$(dirname "$0")/.."
root=$(pwd)

# What counts as ALREADY STAMPED. This is the shell's reading of the
# format `tess_lint::split_cut` parses, and the two are pinned by
# nothing: there is no cross-language gate here. A drift shows up as
# this script declining to refuse, never as a wrong cut. `--selftest`
# covers this side and `tools/tess-lint`'s suite covers the other;
# neither reads the other's spelling.
CUT_RE='^# tess-budget-cut: [0-9a-f]{7,40}(-dirty)? [0-9]{4}-[0-9]{2}-[0-9]{2}'

stamp() {
  local csv=$1 commit= date= tmp before
  if ! git -C "$root" rev-parse --git-dir >/dev/null 2>&1; then
    echo "tess_budget_cut: not a git checkout — no cut recorded" >&2
    return 0
  fi

  if git -C "$root" ls-files --error-unmatch "$csv" >/dev/null 2>&1 &&
     git -C "$root" diff --quiet HEAD -- "$csv"; then
    if head -1 "$csv" | grep -Eq "$CUT_RE"; then
      before=$(head -1 "$csv")
      echo "tess_budget_cut: $csv is already stamped ($before) and its rows have" \
           "not moved. The cut moves only when the data moves — re-run the sweep" \
           "to move it. Refusing to walk the record forward past the rows it" \
           "describes." >&2
      return 1
    fi
    # `read` returns 1 on empty input, which under `errexit` would take
    # the script down before it could say what was wrong — so the read
    # is guarded and the EMPTINESS is what gets checked, below.
    read -r commit date < <(git -C "$root" log -1 --format='%h %cI' -- "$csv") || true
  else
    # Same guard, one arm over: before a repository's first commit
    # `rev-parse HEAD` fails, and an unguarded assignment made the
    # documented skip unreachable.
    commit=$(git -C "$root" rev-parse --short=12 HEAD 2>/dev/null) || commit=
    date=$(git -C "$root" show -s --format=%cI HEAD 2>/dev/null) || date=
    if [ -n "$commit" ] && ! git -C "$root" diff --quiet HEAD --; then
      commit="$commit-dirty"
    fi
  fi

  if [ -z "$commit" ] || [ -z "$date" ]; then
    echo "tess_budget_cut: git named no commit for $csv — no cut recorded" >&2
    return 0
  fi

  # Re-stamping replaces the line rather than stacking a second one:
  # the lint reads exactly one, above the header.
  tmp=$(mktemp)
  {
    echo "# tess-budget-cut: $commit $date"
    grep -v '^# tess-budget-cut:' "$csv"
  } > "$tmp"
  mv "$tmp" "$csv"
  case "$commit" in
    *-dirty) echo "cut: $commit $date (the tree carries uncommitted changes —" \
                  "on a CI runner that is the prune step, and is expected)" ;;
    *)       echo "cut: $commit $date" ;;
  esac
}

# --- the selftest ---------------------------------------------------
#
# Every case is a REAL subprocess invocation of a COPY of this script
# inside a scratch repository, for the reason `gate-roster.sh
# --selftest` gives: a diagnosis lost to `errexit` has to FAIL the
# self-test rather than pass it silently, and two of the cases below
# exist only because the unguarded spelling died before it could speak.
selftest() {
  local t u rc=0 out status
  t=$(mktemp -d)
  # A SECOND scratch root, deliberately a sibling of the first rather
  # than a directory inside it: case (5) needs a tree with no repository
  # anywhere above it, and one nested in the scratch repo has one.
  u=$(mktemp -d)
  trap 'rm -rf "$t" "$u"' RETURN
  mkdir -p "$t/scripts"
  cp "$root/scripts/tess_budget_cut.sh" "$t/scripts/"
  git -C "$t" init -q
  git -C "$t" config user.email selftest@example.invalid
  git -C "$t" config user.name selftest

  local subject=$t/scripts/tess_budget_cut.sh
  local csv=$t/b.csv

  run() {  # run <csv>; sets `out` and `status`
    status=0
    out=$("$subject" "$1" 2>&1) || status=$?
  }
  want() {  # want <label> <rc> <substring>
    if [ "$status" != "$2" ]; then
      echo "SELFTEST FAILED: $1: exit $status, wanted $2 — $out" >&2
      rc=1
    elif [ "${out#*"$3"}" = "$out" ]; then
      echo "SELFTEST FAILED: $1: output does not name '$3' — $out" >&2
      rc=1
    fi
  }

  # (1) BEFORE THE FIRST COMMIT there is no HEAD to name, so the
  # documented skip has to actually happen: `rev-parse HEAD` fails here,
  # and the unguarded spelling exited 1 having said nothing.
  printf 'scene,face\n' > "$csv"
  run "$csv"
  want "unborn HEAD" 0 "no cut recorded"
  head -1 "$csv" | grep -q '^scene' ||
    { echo "SELFTEST FAILED: unborn HEAD: the file was stamped anyway" >&2; rc=1; }

  # (2) A FRESH SWEEP (untracked) takes HEAD. The scratch tree is clean,
  # so the dirty marker must be ABSENT — the reading a CI log gets
  # compared against.
  git -C "$t" commit -q --allow-empty -m first
  run "$csv"
  want "fresh sweep" 0 "cut: "
  head -1 "$csv" | grep -Eq "$CUT_RE" ||
    { echo "SELFTEST FAILED: fresh sweep: no valid cut line written" >&2; rc=1; }
  if [ "${out#*-dirty}" != "$out" ]; then
    echo "SELFTEST FAILED: a clean tree stamped -dirty: $out" >&2
    rc=1
  fi

  # (3) A TRACKED, UNMODIFIED, ALREADY-STAMPED file is REFUSED — the
  # case the whole record exists for. By now the commit that last wrote
  # the file is the one that wrote the STAMP, a whole commit newer than
  # the rows, and taking it would date the cut after data it describes.
  git -C "$t" add b.csv
  git -C "$t" commit -q -m stamped
  local before
  before=$(head -1 "$csv")
  run "$csv"
  want "re-stamp refused" 1 "already stamped"
  [ "$before" = "$(head -1 "$csv")" ] ||
    { echo "SELFTEST FAILED: the refusal rewrote the file anyway" >&2; rc=1; }

  # (4) A MALFORMED stamp is REPAIRED rather than refused: the lint
  # reads an unreadable provenance line as harness breakage, so leaving
  # it in place leaves the gate broken. Tracked and unmodified, so the
  # repair takes the commit that wrote the ROWS.
  printf '# tess-budget-cut: nonsense\nscene,face\n' > "$csv"
  git -C "$t" commit -q -am malformed
  run "$csv"
  want "malformed repaired" 0 "cut: "
  head -1 "$csv" | grep -Eq "$CUT_RE" ||
    { echo "SELFTEST FAILED: malformed stamp not repaired: $(head -1 "$csv")" >&2; rc=1; }

  # (5) OUTSIDE A GIT CHECKOUT the skip is documented, so it has to
  # happen rather than crash.
  mkdir -p "$u/scripts"
  cp "$root/scripts/tess_budget_cut.sh" "$u/scripts/"
  printf 'scene,face\n' > "$u/b.csv"
  status=0
  out=$("$u/scripts/tess_budget_cut.sh" "$u/b.csv" 2>&1) || status=$?
  want "outside a checkout" 0 "no cut recorded"
  head -1 "$u/b.csv" | grep -q '^scene' ||
    { echo "SELFTEST FAILED: outside a checkout: the file was stamped anyway" >&2; rc=1; }

  if [ "$rc" = 0 ]; then
    echo "tess_budget_cut selftest OK: stamps a fresh sweep from HEAD and leaves the" \
         "dirty marker off a clean tree; backfills a tracked file, and repairs an" \
         "unreadable stamp, from the commit that wrote its ROWS; REFUSES to re-stamp" \
         "a tracked, unmodified, already-stamped file and leaves it untouched, which" \
         "is what stops the record drifting forward past its data; and says so rather" \
         "than dying where there is no HEAD and where there is no repository"
  fi
  return $rc
}

if [ "${1:-}" = "--selftest" ]; then
  selftest
  exit $?
fi

csv=${1:?usage: tess_budget_cut.sh <sweep.csv> | --selftest}
case "$csv" in
  /*) ;;
  *) csv=$root/$csv ;;
esac
stamp "$csv"
