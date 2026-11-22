#!/usr/bin/env bash
# fast-set-times-wiring.sh — the fast-set bar is wired the way its scripts
# assume. ONE home; ci.yml's `lint` job runs every gate in this directory.
#
# check-fast-set-times.py judges a test on its cost alone, which it can
# read only by re-running the test on the step's own binaries: so ci.yml's
# default-eps step must hand it the step's cargo scope (`"$SCOPE"`, set
# from `matrix.scope`) and call it before nextest-failures.py deletes the
# junit report. The drift check (fast-set-drift.py) reads the PR runs'
# `fast-set-times-<shard>` artifacts, so ci.yml must upload them, and the
# drift workflow must be scheduled and run its judge.
#
# What it reads: the default-eps step (from its `- name: test (default eps`
# line to the next step), the upload step's artifact name anywhere in
# ci.yml, and the drift workflow's text. Blind to a step that is present
# and skipped by its `if:` (the selftests of the two scripts and the runs
# themselves are the instrument for that).
# shellcheck disable=SC2016 # the workflow text it reads and plants is unexpanded
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

WORKFLOW=.github/workflows/ci.yml
DRIFT=.github/workflows/fast-set-drift.yml
GATE_SCAN_NOUN=workflow

default_eps_step() {
  awk '/^      - name: test \(default eps/ { on = 1; print; next }
       on && /^      - (name|uses):/ { exit }
       on { print }' "$1"
}

gate() {
  gate_require_file "$WORKFLOW"
  gate_require_file "$DRIFT"
  local step bad=0 checker failures
  step=$(default_eps_step "$WORKFLOW")
  if [ -z "$step" ]; then
    gate_error "$(gate_name): $WORKFLOW has no \`- name: test (default eps\` step; the gate's reading of the file is wrong or the step moved — update the gate with it"
    exit 1
  fi
  checker=$(printf '%s\n' "$step" | grep -n 'scripts/check-fast-set-times.py "default eps"' | head -1 || true)
  failures=$(printf '%s\n' "$step" | grep -n 'scripts/nextest-failures.py "default eps"' | head -1 || true)
  if [ -z "$checker" ]; then
    gate_error "the default-eps step does not run check-fast-set-times.py, so no fast-set test is timed"
    bad=1
  elif ! printf '%s\n' "$checker" | grep -qF '"$SCOPE"' \
    || ! printf '%s\n' "$step" | grep -qE '^ +SCOPE: \$\{\{ matrix\.scope \}\}'; then
    gate_error "the default-eps step does not hand check-fast-set-times.py its own cargo scope (\"\$SCOPE\" from matrix.scope), so its re-runs alone are not of the binaries the step built"
    bad=1
  elif [ -n "$failures" ] && [ "${checker%%:*}" -gt "${failures%%:*}" ]; then
    gate_error "the default-eps step runs check-fast-set-times.py after nextest-failures.py, which deletes the junit report it reads"
    bad=1
  fi
  if ! grep -q 'uses: actions/upload-artifact@' "$WORKFLOW" \
    || ! grep -qF 'name: fast-set-times-${{ matrix.shard }}' "$WORKFLOW"; then
    gate_error "$WORKFLOW does not upload the fast-set-times-<shard> artifact the drift check reads"
    bad=1
  fi
  if ! grep -qE '^  schedule:' "$DRIFT" || ! grep -q 'fast-set-drift.py judge' "$DRIFT"; then
    gate_error "$DRIFT is not scheduled or does not run fast-set-drift.py judge, so nothing owns a test drifting past the bar"
    bad=1
  fi
  [ "$bad" = 0 ] || exit 1
  gate_ok "the default-eps step re-times on its own scope before the report is deleted, the junit is uploaded, and the drift check is scheduled"
}

plant_ci() {
  # $1 root, $2 the checker's scope argument, $3 1 to put the checker last,
  # $4 the artifact name
  mkdir -p "$1/.github/workflows"
  local checker="          scripts/check-fast-set-times.py \"default eps\" \"\$rc\" tokens $2"
  local failures='          scripts/nextest-failures.py "default eps" "$rc"'
  {
    printf 'jobs:\n  test:\n    steps:\n'
    printf '      - name: test (default eps; the bar)\n        env:\n'
    printf '          SCOPE: ${{ matrix.scope }}\n        run: |\n'
    if [ "$3" = 1 ]; then printf '%s\n%s\n' "$failures" "$checker"; else printf '%s\n%s\n' "$checker" "$failures"; fi
    printf '      - name: upload\n        uses: actions/upload-artifact@v4\n        with:\n'
    printf '          name: %s\n' "$4"
    printf '      - name: later\n        run: scripts/check-fast-set-times.py "default eps" x y "$SCOPE"\n'
  } > "$1/$WORKFLOW"
}

plant_drift() {
  # $1 root, $2 the trigger key
  mkdir -p "$1/.github/workflows"
  printf 'on:\n  %s:\n    - cron: "1 1 * * *"\njobs:\n  d:\n    steps:\n      - run: python3 scripts/fast-set-drift.py judge kernel x\n' "$2" > "$1/$DRIFT"
}


gate_plant_clean() { plant_ci "$1" '"$SCOPE"' 0 'fast-set-times-${{ matrix.shard }}'; plant_drift "$1" schedule; }
plant_no_scope() { plant_ci "$1" '' 0 'fast-set-times-${{ matrix.shard }}'; }
plant_after_delete() { plant_ci "$1" '"$SCOPE"' 1 'fast-set-times-${{ matrix.shard }}'; }
plant_no_upload() { plant_ci "$1" '"$SCOPE"' 0 'something-else'; }
plant_unscheduled() { plant_drift "$1" push; }

gate_selftest() {
  gate_selftest_clean
  gate_selftest_case "its own cargo scope" plant_no_scope
  gate_selftest_case "after nextest-failures.py" plant_after_delete
  gate_selftest_case "does not upload the fast-set-times" plant_no_upload
  gate_selftest_case "is not scheduled" plant_unscheduled
  printf '%s selftest OK: passes a default-eps step that hands the checker its scope before the report is deleted, beside an upload and a scheduled drift judge; fires on a checker without the scope (a later step that has it does not count), on one called after nextest-failures.py, on no fast-set-times upload, and on an unscheduled drift workflow\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
