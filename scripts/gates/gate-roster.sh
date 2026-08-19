#!/usr/bin/env bash
# gate-roster.sh — both halves of CI run the SAME set of gates.
# ONE home; ci.yml's "gate roster parity (both halves run the same set)"
# step and local-scripts/ci-local.sh's discipline row both call this
# file. It gates itself along with its siblings.
#
# THE INVARIANT: for every executable gate in `scripts/gates/`, both
# `.github/workflows/ci.yml` and `local-scripts/ci-local.sh` invoke it —
# and neither invokes a `scripts/gates/` path that does not exist.
#
# WHY THIS EXISTS. Extracting the gate BODIES to one home each killed
# the drift class one level down, not at the top: each half still names
# which gates to run, so adding a ninth gate to one half and forgetting
# the other reproduces exactly the defect the extraction was for — and
# that is not hypothetical, it is what happened to `EvalScalar` and the
# interval-square gate, which ran hosted-only for months. Sharing the
# bodies would have left the same hole one level up. This closes it:
# neither the gate logic nor the gate list is maintained twice.
#
# WHY THE ROSTER IS THE DIRECTORY, not a list in `lib.sh`. A declared
# list would itself be a third hand-maintained roster, free to drift
# from the files on disk; the filesystem cannot. And ci.yml must spell
# out one named step per gate regardless — the Actions UI step names are
# how failures get read — so the check has to be a grep of ci.yml
# either way. Deriving from the directory makes both halves checked by
# one mechanism, with no list anywhere that nobody verifies.
#
# READING THE PRUNED HALF. Every workflow job runs `rm -rf local-scripts`
# right after checkout, so on a hosted runner `local-scripts/ci-local.sh`
# is not on disk. It is still in the commit, so this gate falls back to
# `git show HEAD:<path>`. That does not weaken the prune's rule: the
# prune exists so hosted CI never EXECUTES or DEPENDS ON local-only
# tooling, and reading the committed text of the mirror to assert the
# two halves agree is neither — it is the whole point of the mirror.
# Enforcing this invariant only on the local half, where nobody is
# watching, would leave it unenforced.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

HOSTED_HALF=.github/workflows/ci.yml
LOCAL_HALF=local-scripts/ci-local.sh

# Text of one half: the working tree when present, else the commit.
half_text() {
  if [ -f "$1" ]; then
    cat "$1"
  elif git rev-parse --git-dir >/dev/null 2>&1 && git cat-file -e "HEAD:$1" 2>/dev/null; then
    git show "HEAD:$1"
  else
    return 1
  fi
}

gate() {
  local half rc=0 script name missing extra
  for half in "$HOSTED_HALF" "$LOCAL_HALF"; do
    if ! half_text "$half" >/dev/null; then
      gate_error "$(gate_name): cannot read $half from the working tree or from HEAD — the roster cannot be checked against a half it cannot see"
      exit 1
    fi
  done

  local -a roster=()
  for script in scripts/gates/*.sh; do
    name=$(basename "$script")
    [ "$name" = lib.sh ] && continue
    roster+=("$name")
  done
  if [ "${#roster[@]}" -eq 0 ]; then
    gate_error "$(gate_name): no gates found in scripts/gates/ under $PWD — the roster scanned nothing, which is not a pass"
    exit 1
  fi

  # Every gate on disk is invoked by BOTH halves.
  for name in "${roster[@]}"; do
    missing=
    for half in "$HOSTED_HALF" "$LOCAL_HALF"; do
      half_text "$half" | grep -qF "scripts/gates/$name" || missing="$missing $half"
    done
    if [ -n "$missing" ]; then
      gate_error "scripts/gates/$name is not invoked by:$missing — a gate that only one half runs is the drift this directory exists to prevent; wire it into both, keeping ci.yml's one named step per gate"
      rc=1
    fi
  done

  # Neither half invokes a gate that does not exist.
  for half in "$HOSTED_HALF" "$LOCAL_HALF"; do
    while IFS= read -r extra; do
      [ -n "$extra" ] || continue
      if [ ! -f "scripts/gates/$extra" ]; then
        gate_error "$half invokes scripts/gates/$extra, which does not exist — a renamed or deleted gate leaves the other half running a stale name"
        rc=1
      fi
    done < <(half_text "$half" | grep -oE 'scripts/gates/[A-Za-z0-9_-]+\.sh' \
             | sed 's#^scripts/gates/##' | grep -v '^lib\.sh$' | sort -u)
  done

  [ "$rc" -eq 0 ] || exit 1
  GATE_SCAN_FILES=${#roster[@]}
  printf '%s OK: both halves run all %s gates in scripts/gates/\n' "$(gate_name)" "${#roster[@]}"
}

# This gate's subject is the two halves and the gate directory, so its
# fixture is a miniature repo rather than a crate tree.
gate_plant_clean() {
  mkdir -p "$1/scripts/gates" "$1/.github/workflows" "$1/local-scripts"
  printf '#!/usr/bin/env bash\n' > "$1/scripts/gates/example-gate.sh"
  printf '        run: scripts/gates/example-gate.sh\n' > "$1/.github/workflows/ci.yml"
  printf '  scripts/gates/example-gate.sh || rc=1\n' > "$1/local-scripts/ci-local.sh"
}

# The live defect: a gate wired into the hosted half only.
plant() {
  printf '#!/usr/bin/env bash\n' > "$1/scripts/gates/hosted-only-gate.sh"
  printf '        run: scripts/gates/hosted-only-gate.sh\n' >> "$1/.github/workflows/ci.yml"
}

gate_parse_args "$@"
gate_main "is not invoked by: local-scripts/ci-local.sh" plant
