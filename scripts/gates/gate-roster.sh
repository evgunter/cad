#!/usr/bin/env bash
# gate-roster.sh — ci.yml WIRES a step for every gate in this directory.
# ONE home; ci.yml's "gate roster parity (ci.yml names every gate)" step
# runs it, and local-scripts/ci-local.sh reaches it through the loop
# that runs this whole directory. It gates itself along with its
# siblings.
#
# WHAT THIS PROVES: for every executable gate in `scripts/gates/`,
# `.github/workflows/ci.yml` contains a `--selftest` call and a real
# call — and it names no `scripts/gates/` path that does not exist. That
# is a claim about WIRING, not about EXECUTION.
#
# WHAT THIS DOES NOT PROVE: that the gate ever runs on a runner. The
# matcher is textual and cannot see YAML semantics, so a step carrying
# `if: false` (or any condition that evaluates false, including a
# `needs.filter` output) keeps its `run:` line, satisfies this gate, and
# is skipped by Actions. Job-level `if:` on `discipline` itself is the
# same hole one level up. Deciding whether a step actually executes
# needs the workflow's own semantics, which a grep does not have and
# which is not worth a YAML evaluator here — so the hole is named rather
# than papered over. A gate silenced that way is visible as a skipped
# step in the Actions UI, which is the compensating control.
#
# WHY ONLY ci.yml. The local half runs this directory in a LOOP
# (`for g in scripts/gates/*.sh`), so it has no roster of its own to
# drift: dropping a file in here is all it takes to be run locally.
# ci.yml cannot loop — the Actions UI reads failures by step name, so
# each gate needs its own named step — which leaves exactly one
# hand-written roster in the repo, and this gate is what checks it.
#
# WHY THE ROSTER IS THE DIRECTORY, not a declared list. A list would be
# a second roster, free to drift from the files on disk; the filesystem
# cannot. This is also why the check needs nothing from
# `local-scripts/`: hosted jobs delete that tree, and a gate that had to
# read it would be buying an exception to a structural rule for
# enforcement it cannot deliver anyway — a deletion touching only
# `local-scripts/ci-local.sh` classifies TIER=docs, so the `discipline`
# job never runs and no hosted gate ever sees it. The loop removes the
# need instead of working around it.
#
# WHY THE LOOP'S GLOB IS `*.sh` AND NOT `*`. What makes this directory a
# roster is not the path but `lib.sh`'s two-mode contract — `--root DIR`,
# and a `--selftest` that must pass a clean fixture AND fire on a planted
# one — together with the shared plumbing that supplies it. Only a bash
# gate can source that. A python check either reimplements the whole
# contract or meets none of it: `scripts/check-interval-cfg-additive.py`
# is the live case, and it is the near miss — it has `--selftest`, but no
# `--root`, and its self-test runs over in-memory snippets rather than a
# fixture tree. Widening the glob would make "in this directory" and
# "under lib.sh's contract" two different sets, and this roster is worth
# something only while they are one.
#
# WHAT COUNTS AS AN INVOCATION. Naming the path in a comment is not
# running it, and self-testing a gate is not running it either: both
# were live evasions of the first version of this check, so the matcher
# requires a real call on a non-comment line AND a `--selftest` call,
# and the fixture below plants both evasions permanently.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

HOSTED_HALF=.github/workflows/ci.yml

# ci.yml with comment lines removed: what the runner would actually
# execute. Materialised into a variable rather than piped into each
# check: `grep -q` exits on its first match, which SIGPIPEs the upstream
# `grep -v`, which `pipefail` then reports as a failed pipeline. That
# made the check flaky — it fired against a correctly wired ci.yml
# depending on which side won the race.
hosted_commands() {
  grep -vE '^[[:space:]]*#' "$HOSTED_HALF"
}

gate() {
  local rc=0 script name esc
  if [ ! -f "$HOSTED_HALF" ]; then
    gate_error "$(gate_name): $HOSTED_HALF does not exist under $PWD — the roster cannot be checked against a half it cannot see"
    exit 1
  fi

  local -a roster=()
  for script in scripts/gates/*.sh; do
    [ -x "$script" ] || continue
    roster+=("$(basename "$script")")
  done
  if [ "${#roster[@]}" -eq 0 ]; then
    gate_error "$(gate_name): no executable gates in scripts/gates/ under $PWD — the roster scanned nothing, which is not a pass"
    exit 1
  fi

  local cmds
  cmds=$(hosted_commands)

  for name in "${roster[@]}"; do
    esc=${name//./\\.}
    if ! grep -qE "(^|[[:space:]])scripts/gates/$esc[[:space:]]*\$" <<<"$cmds"; then
      gate_error "$HOSTED_HALF never RUNS scripts/gates/$name — a gate the hosted half does not invoke is the drift this directory exists to prevent (naming it in a comment, or only self-testing it, does not run it); give it a named step that runs the gate"
      rc=1
    fi
    if ! grep -qE "(^|[[:space:]])scripts/gates/$esc[[:space:]]+--selftest[[:space:]]*\$" <<<"$cmds"; then
      gate_error "$HOSTED_HALF runs scripts/gates/$name without running its --selftest first — a guard that has never been shown to fire is not a guard"
      rc=1
    fi
  done

  while IFS= read -r name; do
    [ -n "$name" ] || continue
    if [ ! -x "scripts/gates/$name" ]; then
      gate_error "$HOSTED_HALF invokes scripts/gates/$name, which is not an executable gate — a renamed or deleted gate leaves a step running a stale name"
      rc=1
    fi
  done < <(grep -oE 'scripts/gates/[A-Za-z0-9_-]+\.sh' <<<"$cmds" \
           | sed 's#^scripts/gates/##' | sort -u)

  [ "$rc" -eq 0 ] || exit 1
  GATE_SCAN_FILES=${#roster[@]}
  printf '%s OK: ci.yml wires a self-tested step for all %s gates in scripts/gates/ (wiring, not execution — see the header on `if:`)\n' \
    "$(gate_name)" "${#roster[@]}"
}

# This gate's subject is ci.yml and the gate directory, so its fixture
# is a miniature repo rather than a crate tree.
gate_plant_clean() {
  mkdir -p "$1/scripts/gates" "$1/.github/workflows"
  printf '#!/usr/bin/env bash\n' > "$1/scripts/gates/example-gate.sh"
  chmod +x "$1/scripts/gates/example-gate.sh"
  printf '#!/usr/bin/env bash\n' > "$1/scripts/gates/lib.sh"
  {
    printf '      - name: example gate\n        run: |\n'
    printf '          scripts/gates/example-gate.sh --selftest\n'
    printf '          scripts/gates/example-gate.sh\n'
  } > "$1/.github/workflows/ci.yml"
}

# A gate on disk that ci.yml does not mention at all.
plant_unwired() {
  printf '#!/usr/bin/env bash\n' > "$1/scripts/gates/unwired-gate.sh"
  chmod +x "$1/scripts/gates/unwired-gate.sh"
}

# EVASION 1: ci.yml only NAMES the gate, in a comment.
plant_comment_only() {
  printf '#!/usr/bin/env bash\n' > "$1/scripts/gates/commented-gate.sh"
  chmod +x "$1/scripts/gates/commented-gate.sh"
  printf '      # runs scripts/gates/commented-gate.sh, honest\n' \
    >> "$1/.github/workflows/ci.yml"
}

# EVASION 2: the real call is gone, the --selftest call remains. The
# gate then only ever runs against its own fixture, never the tree.
plant_selftest_only() {
  printf '#!/usr/bin/env bash\n' > "$1/scripts/gates/selftest-only-gate.sh"
  chmod +x "$1/scripts/gates/selftest-only-gate.sh"
  printf '          scripts/gates/selftest-only-gate.sh --selftest\n' \
    >> "$1/.github/workflows/ci.yml"
}

# A step running a gate that no longer exists.
plant_ghost() {
  printf '          scripts/gates/renamed-away.sh\n' >> "$1/.github/workflows/ci.yml"
}

# Four known evasions, all permanent fixture cases.
gate_selftest() {
  gate_selftest_clean
  gate_selftest_case "never RUNS scripts/gates/unwired-gate.sh" plant_unwired
  gate_selftest_case "never RUNS scripts/gates/commented-gate.sh" plant_comment_only
  gate_selftest_case "never RUNS scripts/gates/selftest-only-gate.sh" plant_selftest_only
  gate_selftest_case "which is not an executable gate" plant_ghost
  printf '%s selftest OK: passes a clean fixture; fires on an unwired gate, a comment-only mention, a selftest-only call, and a ghost step\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
