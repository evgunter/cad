#!/usr/bin/env bash
# build-default-env-agrees.sh — every ci.yml job that restores a primed
# rust-cache key (`build-default`, `lint`) declares the same CARGO_* /
# RUST* job env as the job that primes it, as do nightly-attribution.yml's
# `build-default` jobs, and so does every set of nightly.yml jobs that
# names one `shared-key`. fast-set-drift.yml's build-default job is held
# to the same block, and sets the test job's opt-level-0 wrapper and runs
# its check, since it times the binaries the gate runs. ONE home; ci.yml's
# `lint` job runs every gate in this directory.
#
# rust-cache hashes the CARGO_* and RUST* variables into its key, and
# one job on a push to main writes each primed key: cache-prime writes
# `build-default`, cache-prime-lint writes `lint`. A job whose block
# drifts from its primer's (an added variable, a changed value)
# restores nothing, compiles its dependency graph cold, and stays green:
# the waste is silent, so the equality is checked here. nightly.yml's
# keys have no single writer; a key two jobs name (`nightly-opt2`, read
# by the full-suite build and the topo scalpel) is held equal the same
# way, and a key one job names is that job's own.
#
# What it reads: each job's own `env:` block (four-space `env:` under
# the job, its entries at six spaces), the CARGO_* and RUST* entries of
# it, and the `shared-key:` the job names. Blind to a CARGO_* variable
# set at step level or through $GITHUB_ENV before the rust-cache step
# (the test job's opt-level-0 wrapper is set after it, deliberately), to
# the workflow-level `env:`, which every job shares and so cannot make
# two of them differ, and to a key spelled with an expression, which it
# reads only up to the first space.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

WORKFLOW=.github/workflows/ci.yml
NIGHTLY=.github/workflows/nightly.yml
ATTRIBUTION=.github/workflows/nightly-attribution.yml
DRIFT=.github/workflows/fast-set-drift.yml
WRAPPER='RUSTC_WORKSPACE_WRAPPER=$GITHUB_WORKSPACE/scripts/rustc-test-crates-opt0.sh'
WRAPPER_CHECK='name: every integration binary built went through the opt-level-0 wrapper'
GATE_SCAN_NOUN=workflow

# One line per job naming a shared-key: `<key>\t<job>\t<its sorted
# CARGO_/RUST entries, ;-joined>`. awk emits `<key>\t<job>\t<entry>` (and
# a bare `<key>\t<job>\t` so a job with no entries still counts); the
# join and the sort are done outside it, in the POSIX tools every
# runner's awk dialect agrees with.
shared_key_envs() {
  awk '
    function flush(   i) {
      if (job != "" && key != "") {
        printf "%s\t%s\t\n", key, job
        for (i = 1; i <= n; i++) printf "%s\t%s\t%s\n", key, job, entries[i]
      }
      n = 0; key = ""; inenv = 0
    }
    /^jobs:[[:space:]]*$/ { injobs = 1; next }
    !injobs { next }
    /^  [A-Za-z0-9_-]+:[[:space:]]*$/ { flush(); job = $1; sub(/:$/, "", job); next }
    /^    env:[[:space:]]*$/ { inenv = 1; next }
    inenv && /^      [A-Za-z_][A-Za-z0-9_]*:/ {
      k = $1; sub(/:$/, "", k)
      if (k ~ /^(CARGO|RUST)/) {
        val = $0; sub(/^[^:]*:[[:space:]]*/, "", val); gsub(/"/, "", val)
        entries[++n] = k "=" val
      }
      next
    }
    inenv && /^      [[:space:]]*#/ { next }
    inenv && !/^      / { inenv = 0 }
    /shared-key:[[:space:]]*[^[:space:]]/ { key = $0; sub(/^.*shared-key:[[:space:]]*/, "", key); sub(/[[:space:]].*$/, "", key) }
    END { flush() }
  ' "$1" | sort -u | awk -F '\t' '
    $1 "\t" $2 != cur { if (cur != "") print cur "\t" line; cur = $1 "\t" $2; line = "" }
    $3 != "" { line = line (line == "" ? "" : ";") $3 }
    END { if (cur != "") print cur "\t" line }
  '
}

gate() {
  gate_require_file "$WORKFLOW"
  gate_require_file "$NIGHTLY"
  gate_require_file "$ATTRIBUTION"
  gate_require_file "$DRIFT"
  local rows count distinct primed total=0 f
  for primed in build-default lint; do
    rows=$({
      shared_key_envs "$WORKFLOW"
      if [ "$primed" = build-default ]; then shared_key_envs "$ATTRIBUTION"; fi
    } | awk -F '\t' -v k="$primed" '$1 == k')
    count=$(printf '%s\n' "$rows" | grep -c . || true)
    if [ "$count" -lt 2 ]; then
      gate_error "$(gate_name): $WORKFLOW has $count job(s) naming shared-key $primed — the writer and at least one reader are expected, so the gate's reading of the file is wrong or the cache moved; update the gate with it"
      exit 1
    fi
    distinct=$(printf '%s\n' "$rows" | cut -f3 | sort -u | wc -l)
    if [ "$distinct" -ne 1 ]; then
      printf '%s\n' "$rows" | cut -f2- | sed 's/\t/: /'
      gate_error "the jobs restoring $primed disagree on their CARGO_*/RUST* job env, so the ones that differ from its primer's miss the cache and compile cold"
      exit 1
    fi
    total=$((total + count))
  done
  rows=$(shared_key_envs "$DRIFT" | awk -F '\t' '$1 == "build-default"')
  if ! printf '%s\n' "$rows" | grep -q .; then
    gate_error "$(gate_name): $DRIFT names no shared-key build-default job, so the drift check does not time the gate's build or the gate's reading of the file is wrong; update the gate with it"
    exit 1
  fi
  rows=$( { shared_key_envs "$WORKFLOW"; printf '%s\n' "$rows"; } | awk -F '\t' '$1 == "build-default"')
  distinct=$(printf '%s\n' "$rows" | cut -f3 | sort -u | wc -l)
  if [ "$distinct" -ne 1 ]; then
    printf '%s\n' "$rows" | cut -f2- | sed 's/\t/: /'
    gate_error "the $DRIFT job disagrees with the jobs restoring build-default on its CARGO_*/RUST* job env, so it misses the cache and times other binaries than the gate's"
    exit 1
  fi
  for f in "$WORKFLOW" "$DRIFT"; do
    if ! grep -qF "$WRAPPER" "$f" || ! grep -qF "$WRAPPER_CHECK" "$f"; then
      gate_error "$f does not set the opt-level-0 wrapper ($WRAPPER) and run its check, so its build-default binaries are not the ones the test job builds and nothing would say so"
      exit 1
    fi
  done
  total=$((total + 1))
  local nrows key shared=0 bad=""
  nrows=$(shared_key_envs "$NIGHTLY")
  for key in $(printf '%s\n' "$nrows" | cut -f1 | sort | uniq -d); do
    shared=$((shared + 1))
    if [ "$(printf '%s\n' "$nrows" | awk -F '\t' -v k="$key" '$1 == k' | cut -f3 | sort -u | wc -l)" -ne 1 ]; then
      printf '%s\n' "$nrows" | awk -F '\t' -v k="$key" '$1 == k { print $1 " / " $2 ": " $3 }'
      bad="$bad $key"
    fi
  done
  if [ -n "$bad" ]; then
    gate_error "the $NIGHTLY jobs sharing rust-cache key(s)$bad disagree on their CARGO_*/RUST* job env, so all but one of them miss the cache and compile cold"
    exit 1
  fi
  gate_ok "$total jobs on build-default (ci.yml's and the attribution's) and lint declare one CARGO_*/RUST* env per key; $shared shared nightly key(s) each declare one"
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
      - run: echo "$WRAPPER" >> "\$GITHUB_ENV"
      - $WRAPPER_CHECK
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
  lintprime:
    runs-on: ubuntu-latest
    env:
      CARGO_PROFILE_DEV_OPT_LEVEL: "${4:-0}"
    steps:
      - uses: Swatinem/rust-cache@v2
        with:
          shared-key: lint
EOF
}

plant_attribution() {
  # $1 root, $2 its opt-level value
  mkdir -p "$1/.github/workflows"
  printf 'jobs:\n  attribute:\n    env:\n      CARGO_PROFILE_DEV_LTO: "off"\n      CARGO_PROFILE_DEV_OPT_LEVEL: "%s"\n    steps:\n      - with:\n          shared-key: build-default\n' "$2" > "$1/$ATTRIBUTION"
}

plant_nightly() {
  # $1 root, $2 the scalpel job's opt-level value
  mkdir -p "$1/.github/workflows"
  cat > "$1/$NIGHTLY" <<EOF
on: push
jobs:
  build:
    runs-on: ubuntu-latest
    env:
      CARGO_PROFILE_TEST_OPT_LEVEL: "2"
      CARGO_PROFILE_TEST_DEBUG: line-tables-only
    steps:
      - uses: Swatinem/rust-cache@v2
        with:
          shared-key: nightly-opt2
  scalpel:
    runs-on: ubuntu-latest
    env:
      CARGO_PROFILE_TEST_DEBUG: line-tables-only
      CARGO_PROFILE_TEST_OPT_LEVEL: "$2"
    steps:
      - uses: Swatinem/rust-cache@v2
        with:
          shared-key: nightly-opt2
          save-if: false
  alone:
    runs-on: ubuntu-latest
    env:
      CARGO_PROFILE_TEST_OPT_LEVEL: "3"
    steps:
      - uses: Swatinem/rust-cache@v2
        with:
          shared-key: nightly-alone
EOF
}

plant_drift() {
  # $1 root, $2 the drift job's opt-level value, $3 1 to leave out the wrapper check
  mkdir -p "$1/.github/workflows"
  {
    printf 'jobs:\n  drift:\n    env:\n      CARGO_PROFILE_DEV_LTO: "off"\n'
    printf '      CARGO_PROFILE_DEV_OPT_LEVEL: "%s"\n    steps:\n' "$2"
    printf '      - uses: Swatinem/rust-cache@v2\n        with:\n          shared-key: build-default\n'
    printf '      - run: echo "%s" >> "$GITHUB_ENV"\n' "$WRAPPER"
    [ "$3" = 1 ] || printf '      - %s\n' "$WRAPPER_CHECK"
  } > "$1/$DRIFT"
}

gate_plant_clean() { plant_workflow "$1" 1 '      UNRELATED: "x"'; plant_nightly "$1" 2; plant_attribution "$1" 1; plant_drift "$1" 1 0; }
plant_drift_env() { plant_drift "$1" 0 0; }
plant_drift_no_wrapper_check() { plant_drift "$1" 1 1; }
plant_value_drift() { plant_workflow "$1" 2 '      UNRELATED: "x"'; plant_nightly "$1" 2; plant_attribution "$1" 1; }
plant_added_variable() { plant_workflow "$1" 1 '      RUSTFLAGS: "-Cdebug-assertions=off"'; plant_nightly "$1" 2; plant_attribution "$1" 1; }
plant_lint_drift() { plant_workflow "$1" 1 '      UNRELATED: "x"' 1; plant_nightly "$1" 2; plant_attribution "$1" 1; }
plant_one_lint_job() { plant_workflow "$1" 1 '      UNRELATED: "x"'; sed -i '/^  lintprime:/,$d' "$1/$WORKFLOW"; plant_nightly "$1" 2; plant_attribution "$1" 1; }
plant_nightly_drift() { plant_workflow "$1" 1 '      UNRELATED: "x"'; plant_nightly "$1" 1; plant_attribution "$1" 1; }
plant_attribution_drift() { plant_workflow "$1" 1 '      UNRELATED: "x"'; plant_nightly "$1" 2; plant_attribution "$1" 2; }
plant_one_reader() {
  mkdir -p "$1/.github/workflows"
  printf 'jobs:\n  test:\n    env:\n      CARGO_X: "1"\n    steps:\n      - with:\n          shared-key: build-default\n  l:\n    steps:\n      - with:\n          shared-key: lint\n  m:\n    steps:\n      - with:\n          shared-key: lint\n' > "$1/$WORKFLOW"
  plant_nightly "$1" 2
  : > "$1/$ATTRIBUTION"
}

gate_selftest() {
  local want="disagree on their CARGO_*/RUST* job env"
  gate_selftest_clean
  gate_selftest_case "$want" plant_value_drift
  gate_selftest_case "$want" plant_added_variable
  gate_selftest_case "nightly-opt2 disagree" plant_nightly_drift
  gate_selftest_case "restoring lint disagree" plant_lint_drift
  gate_selftest_case "job(s) naming shared-key lint" plant_one_lint_job
  gate_selftest_case "restoring build-default disagree" plant_attribution_drift
  gate_selftest_case "job(s) naming shared-key build-default" plant_one_reader
  gate_selftest_case "job disagrees with the jobs restoring build-default" plant_drift_env
  gate_selftest_case "does not set the opt-level-0 wrapper" plant_drift_no_wrapper_check
  printf '%s selftest OK: passes two build-default jobs whose blocks agree in another order and around a comment, plus an attribution job that agrees with them, beside two lint jobs that agree with each other and differ from them, and two nightly jobs on one key that agree beside a nightly job alone on its key; fires on a changed value, on an added RUST* variable, on a nightly job whose block drifts from its key-mate, on a lint job whose block drifts from its primer'"'"'s, on an attribution job whose block drifts from the build-default writer, on a file where it finds fewer than two build-default or two lint jobs, on a fast-set-drift.yml job whose block drifts from the build-default jobs, and on a drift workflow that leaves out the opt-level-0 wrapper check\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
