#!/usr/bin/env bash
# gate-roster.sh — BOTH halves of CI run every gate in this directory.
# ONE home; ci.yml's "gate roster parity" step runs it, and
# local-scripts/ci-local.sh reaches it through the loop that runs this
# whole directory. It gates itself along with its siblings.
#
# WHAT THIS PROVES: for every gate in `scripts/gates/`,
# `.github/workflows/ci.yml` contains a `--selftest` call and a real
# call and names no `scripts/gates/` path that does not exist; and
# `local-scripts/ci-local.sh` still runs the directory in a loop that
# calls each gate's `--selftest` and then the gate. That is a claim
# about WIRING, not about EXECUTION.
#
# WHAT THIS DOES NOT PROVE: that the gate ever runs on a runner. The
# matcher is textual and cannot see YAML semantics, so a step carrying
# `if: false` (or any condition that evaluates false, including a
# `needs.filter` output) keeps its `run:` line, satisfies this gate, and
# is skipped by Actions. Job-level `if:` on the job that runs the gates
# is the same hole one level up. Deciding whether a step actually
# executes needs the workflow's own semantics, which a grep does not
# have and which is not worth a YAML evaluator here — so the hole is
# named rather than papered over. A gate silenced that way is visible as
# a skipped step in the Actions UI, which is the compensating control.
#
# WHY THIS GATE READS `local-scripts/`, WHICH NOTHING ELSE HOSTED MAY.
# Every other job deletes that tree right after checkout, so hosted CI
# cannot couple its result to a developer's machine. This gate's SUBJECT
# is the agreement between the two halves, so it is the one thing that
# has to see both — and it is sited in the `mirror` job, which is the
# single declared exception to the prune and the single job that runs on
# every tier. `ci-mirror-parity.sh` checks that the exception stays
# exactly one job. An earlier version of this header argued the opposite
# — that the check needed nothing from `local-scripts/` because a
# deletion touching only `local-scripts/ci-local.sh` classifies
# TIER=docs, so the gate job never runs. That is a description of a hole
# offered as the reason not to close it: the local half's loop was
# unverified by construction, and a PR deleting it was invisible.
#
# WHY THE ROSTER IS THE DIRECTORY, not a declared list. A list would be
# a second roster, free to drift from the files on disk; the filesystem
# cannot.
#
# WHY THE ROSTER'S GLOB IS `*.sh` AND NOT `*`. What makes this directory
# a roster is not the path but `lib.sh`'s two-mode contract — `--root
# DIR`, and a `--selftest` that must pass a clean fixture AND fire on a
# planted one — together with the shared plumbing that supplies it. Only
# a bash gate can source that. A python check either reimplements the
# whole contract or meets none of it: `scripts/check-interval-cfg-additive.py`
# is the live case, and it is the near miss — it has `--selftest`, but no
# `--root`, and its self-test runs over in-memory snippets rather than a
# fixture tree. Widening the glob would make "in this directory" and
# "under lib.sh's contract" two different sets, and this roster is worth
# something only while they are one. What keeps a python check from
# going unmirrored instead is `scripts/check-ci-mirror-parity.py`, not
# this glob — which is also where that check itself now lives, for the
# reason its own header gives.
#
# WHY `lib.sh` IS EXCLUDED BY NAME AND NOT BY MODE. `lib.sh` is sourced,
# not run, and it is the only member of this directory that is not a
# gate. Both halves used to derive the roster with `[ -x "$script" ] ||
# continue`, which excludes it — and thereby made the EXECUTABLE BIT the
# registration mechanism: a gate landing mode 0644 was invisible to both
# halves, silently unregistered, reported inside "all N gates". That was
# a filename problem solved with a permission bit. It is a filename
# problem again: `lib.sh` is skipped by name, and a member of this
# directory that is not executable is a FAILURE rather than a skip.
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
LOCAL_HALF=local-scripts/ci-local.sh

# The one member of scripts/gates/ that is sourced rather than run.
NOT_A_GATE=lib.sh

# A half with its comment lines removed: what would actually run.
# MATERIALISED into a variable by every caller, never piped into a check:
# `grep -q` exits on its first match, which SIGPIPEs the upstream
# `grep -v`, which `pipefail` then reports as a failed pipeline. That
# made this check flaky — it fired against a correctly wired ci.yml
# depending on which side won the race. `|| true` for the same reason a
# matcher below carries one: a file of nothing but comments is an empty
# result, not a reason to die before the diagnosis.
#
# One home per file, and this is the third spelling of it in the repo
# (`probe-suite-census.sh` has the fourth). The shared home is
# `scripts/gates/lib.sh`, which lane F-g owns; this note is here so
# whoever takes it can lift all four at once.
non_comment() {
  grep -vE '^[[:space:]]*#' "$1" || true
}

gate() {
  local rc=0 script name esc base loopvar
  if [ ! -f "$HOSTED_HALF" ]; then
    gate_error "$(gate_name): $HOSTED_HALF does not exist under $PWD — the roster cannot be checked against a half it cannot see"
    exit 1
  fi
  if [ ! -f "$LOCAL_HALF" ]; then
    gate_error "$(gate_name): $LOCAL_HALF does not exist under $PWD — the local half is half the claim this gate makes, and a missing one cannot be checked. This gate runs in the one job that does NOT prune local-scripts/; if it is reporting this on a runner, the prune has spread to that job"
    exit 1
  fi

  # THE ROSTER IS THE DIRECTORY. `lib.sh` is excluded by NAME; every
  # other member must be executable, because a gate nobody can run is
  # not registered anywhere and both halves would silently skip it.
  local -a roster=()
  for script in scripts/gates/*.sh; do
    base=$(basename "$script")
    [ "$base" = "$NOT_A_GATE" ] && continue
    if [ ! -x "$script" ]; then
      gate_error "$(gate_name): scripts/gates/$base is mode $(stat -c '%a' "$script" 2>/dev/null || echo '?') and not executable. Both halves derive the roster from this directory, so a non-executable member is a gate that runs NOWHERE while still reading as one of the N — chmod +x it, or move it out of this directory if it is not a gate"
      rc=1
      continue
    fi
    roster+=("$base")
  done
  if [ "${#roster[@]}" -eq 0 ]; then
    gate_error "$(gate_name): no gates in scripts/gates/ under $PWD besides $NOT_A_GATE — the roster scanned nothing, which is not a pass"
    exit 1
  fi
  [ "$rc" -eq 0 ] || exit 1

  local cmds
  cmds=$(non_comment "$HOSTED_HALF")

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
    if [ "$name" = "$NOT_A_GATE" ]; then
      gate_error "$HOSTED_HALF invokes scripts/gates/$NOT_A_GATE, which is shared plumbing to be SOURCED and not a gate to be run"
      rc=1
    elif [ ! -f "scripts/gates/$name" ]; then
      gate_error "$HOSTED_HALF invokes scripts/gates/$name, which is not a gate in this directory — a renamed or deleted gate leaves a step running a stale name"
      rc=1
    fi
  done < <(grep -oE 'scripts/gates/[A-Za-z0-9_-]+\.sh' <<<"$cmds" \
           | sed 's#^scripts/gates/##' | sort -u)

  # THE LOCAL HALF. It has no roster to drift because it runs the
  # directory in a loop — but nothing checked that the loop was still
  # there, which is the whole reason this gate now reads this file. The
  # loop variable is read out of the `for` line rather than assumed, so
  # renaming it is not a way to fail this check.
  # `|| true` ON EVERY MATCHER, and it is load-bearing rather than tidy.
  # Under `set -euo pipefail` a command substitution whose pipeline fails
  # kills the script AT THE ASSIGNMENT — so the first version of this
  # block died before reaching the `gate_error` three lines down, and the
  # gate reported the failure it was written to explain as a bare `exit
  # 1` with no message. No self-test could see it while the harness ran
  # the gate in-process inside an `if` condition, where bash suppresses
  # errexit; lib.sh now runs every case as a real subprocess, so dropping
  # one of these `|| true`s reds this gate's own self-test (S157).
  local body
  body=$(non_comment "$LOCAL_HALF")
  loopvar=$(grep -oE 'for[[:space:]]+[A-Za-z_][A-Za-z0-9_]*[[:space:]]+in[[:space:]]+scripts/gates/\*\.sh' <<<"$body" \
            | head -1 | awk '{print $2}' || true)
  if [ -z "$loopvar" ]; then
    gate_error "$LOCAL_HALF no longer loops \`scripts/gates/*.sh\` — that loop is the ONLY reason the local half has no hand-written roster to drift from this directory, and this gate's header says so. Restore the loop, or give the local half a roster and a check on it"
    rc=1
  else
    grep -qE "\"\\\$$loopvar\"[[:space:]]+--selftest" <<<"$body" ||
      { gate_error "$LOCAL_HALF loops scripts/gates/*.sh as \$$loopvar but never runs \"\$$loopvar\" --selftest — locally too, a guard that has never been shown to fire is not a guard"; rc=1; }
    grep -qE "\"\\\$$loopvar\"([[:space:]]*\||[[:space:]]*\$|[[:space:]]+\|\|)" <<<"$body" ||
      { gate_error "$LOCAL_HALF loops scripts/gates/*.sh as \$$loopvar but never RUNS \"\$$loopvar\" against the tree — self-testing a gate is not running it"; rc=1; }
    grep -qE "$NOT_A_GATE.*continue" <<<"$body" ||
      { gate_error "$LOCAL_HALF's gate loop does not mention $NOT_A_GATE, so it is excluding it by mode rather than by name — that makes the executable bit the registration mechanism again, and a gate landing 0644 invisible locally"; rc=1; }
  fi

  [ "$rc" -eq 0 ] || exit 1
  GATE_SCAN_FILES=${#roster[@]}
  printf '%s OK: ci.yml wires a self-tested step for all %s gates in scripts/gates/, and %s still runs the directory in a self-testing loop (wiring, not execution — see the header on `if:`)\n' \
    "$(gate_name)" "${#roster[@]}" "$LOCAL_HALF"
}

# This gate's subject is ci.yml, the gate directory, and the local
# half's loop, so its fixture is a miniature repo rather than a crate
# tree.
gate_plant_clean() {
  mkdir -p "$1/scripts/gates" "$1/.github/workflows" "$1/local-scripts"
  printf '#!/usr/bin/env bash\n' > "$1/scripts/gates/example-gate.sh"
  chmod +x "$1/scripts/gates/example-gate.sh"
  printf '#!/usr/bin/env bash\n' > "$1/scripts/gates/lib.sh"
  {
    printf '      - name: example gate\n        run: |\n'
    printf '          scripts/gates/example-gate.sh --selftest\n'
    printf '          scripts/gates/example-gate.sh\n'
  } > "$1/.github/workflows/ci.yml"
  {
    printf 'discipline() {\n'
    printf '  local rc=0 g\n'
    printf '  for g in scripts/gates/*.sh; do\n'
    printf '    [ "$(basename "$g")" = lib.sh ] && continue\n'
    printf '    "$g" --selftest || rc=1\n'
    printf '    "$g" || rc=1\n'
    printf '  done\n'
    printf '  return $rc\n'
    printf '}\n'
  } > "$1/local-scripts/ci-local.sh"
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

# EVASION 3, and the one this gate was blind to for its whole first
# life: a gate lands mode 0644. Wired in ci.yml, present on disk, and
# skipped by every `[ -x ]` roster derivation in the repo — reported
# inside "all N gates" while running nowhere.
plant_nonexecutable() {
  printf '#!/usr/bin/env bash\n' > "$1/scripts/gates/nonexec-gate.sh"
  chmod 0644 "$1/scripts/gates/nonexec-gate.sh"
  {
    printf '          scripts/gates/nonexec-gate.sh --selftest\n'
    printf '          scripts/gates/nonexec-gate.sh\n'
  } >> "$1/.github/workflows/ci.yml"
}

# The local half stops running the directory. Invisible to hosted CI by
# construction before this gate read the file: a change touching only
# local-scripts/ classifies TIER=docs.
plant_local_loop_deleted() {
  printf 'discipline() { return 0; }\n' > "$1/local-scripts/ci-local.sh"
}

# The local loop survives but stops self-testing: every gate then runs
# locally without ever being shown to fire.
plant_local_no_selftest() {
  printf 'discipline() {\n  for g in scripts/gates/*.sh; do\n    [ "$(basename "$g")" = lib.sh ] && continue\n    "$g" || rc=1\n  done\n}\n' \
    > "$1/local-scripts/ci-local.sh"
}

# The local loop goes back to excluding lib.sh by MODE — the permission
# bit as registration mechanism, which is what made a 0644 gate
# invisible on this side.
plant_local_mode_exclusion() {
  printf 'discipline() {\n  for g in scripts/gates/*.sh; do\n    [ -x "$g" ] || continue\n    "$g" --selftest || rc=1\n    "$g" || rc=1\n  done\n}\n' \
    > "$1/local-scripts/ci-local.sh"
}

# ci.yml runs the shared plumbing as if it were a gate.
plant_lib_invoked() {
  printf '          scripts/gates/lib.sh\n' >> "$1/.github/workflows/ci.yml"
}

# Nine known evasions, all permanent fixture cases, all run as real
# subprocesses by lib.sh's harness — the property this gate needs most,
# because the `|| true` on every matcher in `gate()` is load-bearing and
# nothing could observe its absence while the harness ran a gate
# in-process (S157).
gate_selftest() {
  gate_selftest_clean
  gate_selftest_case "never RUNS scripts/gates/unwired-gate.sh" plant_unwired
  gate_selftest_case "never RUNS scripts/gates/commented-gate.sh" plant_comment_only
  gate_selftest_case "never RUNS scripts/gates/selftest-only-gate.sh" plant_selftest_only
  gate_selftest_case "which is not a gate in this directory" plant_ghost
  gate_selftest_case "not executable" plant_nonexecutable
  gate_selftest_case "no longer loops" plant_local_loop_deleted
  gate_selftest_case "never runs \"\$g\" --selftest" plant_local_no_selftest
  gate_selftest_case "excluding it by mode rather than by name" plant_local_mode_exclusion
  gate_selftest_case "SOURCED and not a gate" plant_lib_invoked
  printf '%s selftest OK: every case is a REAL subprocess invocation, so a diagnosis lost to errexit fails the self-test. Passes a clean fixture; fires on an unwired gate, a comment-only mention, a selftest-only call, a ghost step, a gate that landed mode 0644, a deleted local loop, a local loop that stopped self-testing, a local loop that went back to excluding lib.sh by mode, and a step running lib.sh\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
