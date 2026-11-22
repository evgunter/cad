#!/usr/bin/env bash
# build-default-env-agrees.sh — every ci.yml job that restores the
# `build-default` rust-cache key declares the same CARGO_* / RUST* job
# env. ONE home; ci.yml's `lint` job runs every gate in this directory.
#
# rust-cache hashes the CARGO_* and RUST* variables into its key, and
# cache-prime is the one job that writes `build-default`. A job whose
# block drifts from cache-prime's (an added variable, a changed value)
# restores nothing, compiles its dependency graph cold, and stays green:
# the waste is silent, so the equality is checked here.
#
# What it reads: each job's own `env:` block (four-space `env:` under
# the job, its entries at six spaces), the CARGO_* and RUST* entries of
# it, and whether the job names `shared-key: build-default`. Blind to a
# CARGO_* variable set at step level or through $GITHUB_ENV before the
# rust-cache step (the test job's opt-level-0 wrapper is set after it,
# deliberately), and to the workflow-level `env:`, which every job
# shares and so cannot make two of them differ.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

WORKFLOW=.github/workflows/ci.yml
GATE_SCAN_NOUN=workflow

# One line per build-default job: `<job>\t<its sorted CARGO_/RUST entries, ;-joined>`.
# awk emits `<job>\t<entry>` (and a bare `<job>\t` so a job with no
# entries still counts); the join and the sort are done outside it, in
# the POSIX tools every runner's awk dialect agrees with.
build_default_envs() {
  awk '
    function flush(   i) {
      if (job != "" && uses) {
        printf "%s\t\n", job
        for (i = 1; i <= n; i++) printf "%s\t%s\n", job, entries[i]
      }
      n = 0; uses = 0; inenv = 0
    }
    /^jobs:[[:space:]]*$/ { injobs = 1; next }
    !injobs { next }
    /^  [A-Za-z0-9_-]+:[[:space:]]*$/ { flush(); job = $1; sub(/:$/, "", job); next }
    /^    env:[[:space:]]*$/ { inenv = 1; next }
    inenv && /^      [A-Za-z_][A-Za-z0-9_]*:/ {
      key = $1; sub(/:$/, "", key)
      if (key ~ /^(CARGO|RUST)/) {
        val = $0; sub(/^[^:]*:[[:space:]]*/, "", val); gsub(/"/, "", val)
        entries[++n] = key "=" val
      }
      next
    }
    inenv && /^      [[:space:]]*#/ { next }
    inenv && !/^      / { inenv = 0 }
    /shared-key:[[:space:]]*build-default[[:space:]]*$/ { uses = 1 }
    END { flush() }
  ' "$WORKFLOW" | sort -u | awk -F '\t' '
    $1 != cur { if (cur != "") print cur "\t" line; cur = $1; line = "" }
    $2 != "" { line = line (line == "" ? "" : ";") $2 }
    END { if (cur != "") print cur "\t" line }
  '
}

gate() {
  gate_require_file "$WORKFLOW"
  local rows
  rows=$(build_default_envs)
  local count
  count=$(printf '%s\n' "$rows" | grep -c . || true)
  if [ "$count" -lt 2 ]; then
    gate_error "$(gate_name): $WORKFLOW has $count job(s) naming shared-key build-default — the writer and at least one reader are expected, so the gate's reading of the file is wrong or the cache moved; update the gate with it"
    exit 1
  fi
  local distinct
  distinct=$(printf '%s\n' "$rows" | cut -f2 | sort -u | wc -l)
  if [ "$distinct" -ne 1 ]; then
    printf '%s\n' "$rows" | sed 's/\t/: /'
    gate_error "the jobs restoring build-default disagree on their CARGO_*/RUST* job env, so the ones that differ from cache-prime's miss the cache and compile cold"
    exit 1
  fi
  gate_ok "$count jobs on build-default declare one CARGO_*/RUST* env"
}

plant_workflow() {
  # $1 root, $2 the reader job's opt-level value, $3 an extra reader line
  mkdir -p "$1/.github/workflows"
  cat > "$1/$WORKFLOW" <<EOF
on: push
jobs:
  test:
    runs-on: ubuntu-latest
    env:
      CARGO_PROFILE_DEV_OPT_LEVEL: "1"
      # a comment inside the block
      CARGO_PROFILE_DEV_LTO: "off"
    steps:
      - uses: Swatinem/rust-cache@v2
        with:
          shared-key: build-default
  reader:
    runs-on: ubuntu-latest
    env:
      CARGO_PROFILE_DEV_LTO: "off"
      CARGO_PROFILE_DEV_OPT_LEVEL: "$2"
$3
    steps:
      - uses: Swatinem/rust-cache@v2
        with:
          shared-key: build-default
  other:
    runs-on: ubuntu-latest
    env:
      CARGO_PROFILE_DEV_OPT_LEVEL: "0"
    steps:
      - uses: Swatinem/rust-cache@v2
        with:
          shared-key: lint
EOF
}

gate_plant_clean() { plant_workflow "$1" 1 '      UNRELATED: "x"'; }
plant_value_drift() { plant_workflow "$1" 2 '      UNRELATED: "x"'; }
plant_added_variable() { plant_workflow "$1" 1 '      RUSTFLAGS: "-Cdebug-assertions=off"'; }
plant_one_reader() {
  mkdir -p "$1/.github/workflows"
  printf 'jobs:\n  test:\n    env:\n      CARGO_X: "1"\n    steps:\n      - with:\n          shared-key: build-default\n' > "$1/$WORKFLOW"
}

gate_selftest() {
  local want="disagree on their CARGO_*/RUST* job env"
  gate_selftest_clean
  gate_selftest_case "$want" plant_value_drift
  gate_selftest_case "$want" plant_added_variable
  gate_selftest_case "job(s) naming shared-key build-default" plant_one_reader
  printf '%s selftest OK: passes two build-default jobs whose blocks agree in another order and around a comment, beside a job on another key that differs; fires on a changed value, on an added RUST* variable, and on a file where it finds fewer than two build-default jobs\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
