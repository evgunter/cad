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

# The guarded functions verbatim from the script under test — extracted
# rather than copied so a change to one cannot leave this selftest
# testing a fossil. `run_dump` selects `--ignored` tests and dumps CSV;
# `run_plain` selects the default set; both end in `finish_run`, which
# reads the runner's own counts, writes the tally row the executed-set
# floor reads, and refuses a selection that neither ran nor skipped
# anything. `feats_for` rides the same list because both callers now ask
# it which feature set a module needs — extracted rather than stubbed,
# so a suite added to its table is exercised here and not only in the
# sweep.
extract_fn() {
  sed -n "/^$1() {/,/^}/p" "$root/scripts/k_probe_sweep.sh"
}
extract_guards() {
  local f
  for f in feats_for result_field record_ran finish_run run_dump run_plain; do
    if [ -z "$(extract_fn "$f")" ]; then
      echo "SELFTEST FAILED: no $f() found in scripts/k_probe_sweep.sh" >&2
      exit 1
    fi
    extract_fn "$f"
  done
}
# The stated ε `run_plain` pins, extracted for the same reason: a
# constant copied here would let the two drift.
extract_const() {
  grep -E "^$1=" "$root/scripts/k_probe_sweep.sh" | head -1
}
if [ -z "$(extract_const PLAIN_EPS)" ]; then
  echo "SELFTEST FAILED: no PLAIN_EPS in scripts/k_probe_sweep.sh" >&2
  exit 1
fi
extract_guards >/dev/null

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
echo "test result: ok. ${STUB_PASSED:-1} passed; 0 failed; ${STUB_IGNORED:-0} ignored; 0 measured; 0 filtered out; finished in 0.00s"
STUB
  chmod +x "$d/cargo"
}

# Runs run_dump in a subshell with the stub on PATH; echoes its exit code.
drive() {
  local out=$1 t
  t=$(mktemp -d); make_stub "$t/bin"
  set +e
  ( set -euo pipefail
    export PATH="$t/bin:$PATH" RAN_TALLY="$t/ran.tsv"
    eval "$(extract_guards)"
    run_dump 1e-9 selftest somepkg some_module:: "$out"
  ) >/dev/null 2>&1
  local rc=$?
  set -e
  rm -rf "$t"
  echo "$rc"
}

# The same for `run_plain`, plus the row it wrote — because the row is
# what the executed-set floor reads, and a guard that exits non-zero
# while recording nothing would leave the floor with no evidence either
# way. Echoes `<rc> <tally row>`.
drive_plain() {
  local t rc row
  t=$(mktemp -d); make_stub "$t/bin"
  set +e
  ( set -euo pipefail
    export PATH="$t/bin:$PATH" RAN_TALLY="$t/ran.tsv"
    eval "$(extract_const PLAIN_EPS)"
    eval "$(extract_guards)"
    run_plain somepkg some_module
  ) >/dev/null 2>&1
  rc=$?
  set -e
  row=$(tr '\t' ' ' < "$t/ran.tsv" 2>/dev/null | head -1)
  rm -rf "$t"
  echo "$rc|$row"
}

expect() {
  local want=$1 got=$2 what=$3
  if [ "$got" != "$want" ]; then
    echo "SELFTEST FAILED ($what): got \`$got\`, expected \`$want\`" >&2
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

# 5. THE DEFAULT-SELECTION GUARD, both directions. `run_plain` exists
#    because `run_dump`'s `--ignored` runs ONLY `#[ignore]`d tests, so a
#    filter naming a suite of plain `#[test]`s selects the module and
#    executes none of it. That is the case a reachability floor scores as
#    covered. The row `run_plain` writes carries BOTH counts, because the
#    floor reconciles the skipped half against what `--ignored` ran; the
#    tally is that floor's only evidence, so it is asserted here too.
expect '0|plain somepkg some_module 1 0' "$(STUB_PASSED=1 drive_plain)" 'plain: one test ran, tally records it'
# THE COMPLEMENT the executed-set floor reads: the default selection ran
# nothing and SKIPPED two, which is a legitimate all-`#[ignore]`d suite —
# it must pass here and be reconciled against the `--ignored` run there.
expect '0|plain somepkg some_module 0 2' "$(STUB_PASSED=0 STUB_IGNORED=2 drive_plain)" 'plain: all-ignored suite, complement recorded'
# Neither ran nor skipped: the module matched nothing at all.
expect '1|plain somepkg some_module 0 0' "$(STUB_PASSED=0 drive_plain)" 'plain: module matched no test at all'

echo 'rundump-guard selftest OK: run_dump passes a clean dump and fires on an empty selection, an absent CSV, and a header-only CSV; run_plain passes a live selection and an all-ignored suite, fires on a module that matched no test at all, and records the executed and skipped counts in every case' >&2
