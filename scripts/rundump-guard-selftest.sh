#!/usr/bin/env bash
# rundump-guard-selftest.sh — proves k_probe_sweep.sh's `run_dump` guards
# actually fire. Written by the D17 style review (#739); the guards are
# the only thing standing between "the harness ran" and "the sweep
# recorded nothing and exited 0", and nothing else in the tree exercises
# them.
#
# NO CARGO. `run_dump` is sourced out of the real script and driven
# against a STUB `cargo` on PATH, so each case costs milliseconds and the
# row can gate on every merge. The stub is what lets the third case exist
# at all: "the harness passed and wrote no file" cannot be staged with
# the real harness without editing it.
set -euo pipefail
cd "$(dirname "$0")/.."
root=$(pwd)

# `run_dump` verbatim from the script under test — extracted rather than
# copied so a change to it cannot leave this selftest testing a fossil.
extract_run_dump() {
  sed -n '/^run_dump() {/,/^}/p' "$root/scripts/k_probe_sweep.sh"
}
if [ -z "$(extract_run_dump)" ]; then
  echo "SELFTEST FAILED: no run_dump() found in scripts/k_probe_sweep.sh" >&2
  exit 1
fi

# stub cargo: $STUB_PASSED rows "passed"; writes $CAD_K_REPORT_OUT only
# if $STUB_ROWS is set.
make_stub() {
  local d=$1
  mkdir -p "$d"
  cat > "$d/cargo" <<'STUB'
#!/usr/bin/env bash
echo "running ${STUB_PASSED:-1} tests"
if [ -n "${STUB_ROWS:-}" ] && [ -n "${CAD_K_REPORT_OUT:-}" ]; then
  { echo "shape,predicate,margin,band_zero,band_escalate,outcome"
    for _ in $(seq 1 "$STUB_ROWS"); do echo "s,p,1,0,0,ok"; done
  } > "$CAD_K_REPORT_OUT"
fi
echo "test result: ok. ${STUB_PASSED:-1} passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s"
STUB
  chmod +x "$d/cargo"
}

# Runs run_dump in a subshell with the stub on PATH; echoes its exit code.
drive() {
  local out=$1 t
  t=$(mktemp -d); make_stub "$t/bin"
  set +e
  ( set -euo pipefail
    export PATH="$t/bin:$PATH"
    eval "$(extract_run_dump)"
    run_dump 1e-9 selftest somepkg some_module:: "$out"
  ) >/dev/null 2>&1
  local rc=$?
  set -e
  rm -rf "$t"
  echo "$rc"
}

expect() {
  local want=$1 got=$2 what=$3
  if [ "$got" != "$want" ]; then
    echo "SELFTEST FAILED ($what): run_dump exited $got, expected $want" >&2
    exit 1
  fi
}

tmp=$(mktemp -d); trap 'rm -rf "$tmp"' EXIT

# 1. NEGATIVE CONTROL. A guard that refused everything would pass the
#    plants below, so the healthy case must pass first.
rm -f "$tmp/ok.csv"
expect 0 "$(STUB_PASSED=1 STUB_ROWS=3 drive "$tmp/ok.csv")" 'clean: one row passed, CSV written'

# 2. THE SELECTION MATCHED NOTHING. Cargo exits 0 over zero tests; the
#    passed-count guard is what turns that into a failure.
rm -f "$tmp/none.csv"
expect 1 "$(STUB_PASSED=0 drive "$tmp/none.csv")" 'zero tests selected'

# 3. THE HARNESS RAN AND RECORDED NOTHING — the case run_dump's own error
#    message names ("the harness ran but recorded no margins"). The file
#    is ABSENT rather than short, which is what happens when the dump's
#    env var is renamed or its write path is dropped. `wc -l < missing`
#    prints nothing, so the comparison is `[ "" -lt 2 ]`, which errors
#    inside an `if` condition — exempt from `set -e` — and the guard
#    takes the false branch.
rm -f "$tmp/absent.csv"
expect 1 "$(STUB_PASSED=1 drive "$tmp/absent.csv")" 'harness passed, wrote no CSV at all'

# 4. A CSV WITH A HEADER AND NO ROWS.
printf 'shape,predicate,margin,band_zero,band_escalate,outcome\n' > "$tmp/hdr.csv"
expect 1 "$(STUB_PASSED=1 drive "$tmp/hdr.csv")" 'header only, no sample rows'

echo 'rundump-guard selftest OK: run_dump passes a clean dump and fires on an empty selection, an absent CSV, and a header-only CSV' >&2
