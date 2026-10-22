#!/usr/bin/env bash
# unmigrated-measure-sites.sh — the transitional measurement alias's
# call sites, per crate, held to a ratchet. ONE home; ci.yml's `lint`
# job runs every gate in this directory.
#
# THE RULE (FLUX unit F, Ev's decision on PR 4822): `mass_properties`
# and `classify_shells` take a finished body (`&AtRestBody`). F1 typed
# the door and migrated topo and the small crates; sweep's and
# editor-core's test sites read through
# `topo::test_support::unmigrated_measure` / `unmigrated_classify` until
# F2 migrates them. The alias means "not yet migrated", never "measures
# an invalid body on purpose" (that is `measure_unvalidated`, with its
# reason at the call).
#
# So its sites may only fall. A rise is a new site on the alias, which
# should be on the typed door or the raw one; a fall must lower the
# baseline (`unmigrated-measure-sites.counts`), so the ratchet holds it
# there. F2 brings every count to 0 and deletes the alias, this gate and
# its baseline (`work/flux/the-measurement-doors-test-sites-still-read-through-the-transitional-alias.md`).
#
# WHAT IT COUNTS: a call `unmigrated_measure(` or `unmigrated_classify(`
# in code (comments and string literals stripped, `gate_rust_code`) under
# `crates/*/src` and `crates/*/tests`, the alias's own `fn` declarations
# excluded. A path call (`topo::test_support::unmigrated_measure(`)
# counts; a `use` of the name alone does not.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

BASELINE=scripts/gates/unmigrated-measure-sites.counts

gate() {
  gate_require_file "$BASELINE"
  gate_require_crate_sources
  local files report
  mapfile -t files < <(find crates/*/src crates/*/tests -type f -name '*.rs' 2>/dev/null | sort)
  if ! report=$(gate_rust_code "${files[@]}" \
    | gate_grep -E '\bunmigrated_(measure|classify)\s*\(' \
    | gate_grep -vE '\bfn\s+unmigrated_(measure|classify)\b' \
    | python3 -c '
import sys
base = {}
with open(sys.argv[1], encoding="utf-8") as fh:
    for line in fh:
        line = line.split("#", 1)[0].strip()
        if line:
            crate, n = line.split()
            base[crate] = int(n)
counts = {}
for hit in sys.stdin:
    parts = hit.split("/")
    if len(parts) > 2 and parts[0] == "crates":
        crate = "crates/" + parts[1]
        counts[crate] = counts.get(crate, 0) + 1
for crate in sorted(set(counts) | set(base)):
    now, was = counts.get(crate, 0), base.get(crate, 0)
    if now > was:
        print(f"ROSE {crate}: {was} -> {now}")
    elif now < was:
        print(f"FELL {crate}: {was} -> {now}")
print(f"TOTAL {sum(counts.values())}")
' "$BASELINE"); then
    gate_error "$(gate_name): the counter failed under $PWD (its own message is above, if it had one) — the gate could not read its subject, which is not a pass"
    exit 1
  fi
  if grep -q '^ROSE ' <<<"$report"; then
    grep '^ROSE ' <<<"$report"
    gate_error "the transitional measurement alias's sites rose — a new call reads through unmigrated_measure / unmigrated_classify. Measure a finished body (finished(...) once per body, or the op's own AtRestBody) or, for a body tier 3 refuses on purpose, topo::test_support::measure_unvalidated with its reason"
    exit 1
  fi
  if grep -q '^FELL ' <<<"$report"; then
    grep '^FELL ' <<<"$report"
    gate_error "the transitional measurement alias's sites fell — lower $BASELINE to the new counts, so the ratchet holds them there"
    exit 1
  fi
  gate_ok "the transitional measurement alias at its baseline ($(sed -n 's/^TOTAL //p' <<<"$report") sites in all)"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/scripts/gates"
  printf '# crate count\n' > "$1/$BASELINE"
}

plant_new_site() {
  printf 'pub fn v(b: &B) -> f64 { topo::test_support::unmigrated_measure(b, t()).volume }\n' \
    >> "$1/crates/clean/src/lib.rs"
}

plant_new_test_site() {
  mkdir -p "$1/crates/clean/tests"
  printf '#[test]\nfn r() { let _ = unmigrated_classify(&b, t()); }\n' \
    > "$1/crates/clean/tests/rows.rs"
}

plant_fell_unrecorded() {
  printf 'crates/clean 2\n' >> "$1/$BASELINE"
  printf 'pub fn v(b: &B) -> f64 { unmigrated_measure(b, t()).volume }\n' \
    >> "$1/crates/clean/src/lib.rs"
}

plant_at_baseline() {
  printf 'crates/clean 1\n' >> "$1/$BASELINE"
  printf 'pub fn v(b: &B) -> f64 { unmigrated_measure(b, t()).volume }\n' \
    >> "$1/crates/clean/src/lib.rs"
  printf 'pub fn unmigrated_measure(b: &B, t: T) -> M { m(b, t) }\n' \
    >> "$1/crates/clean/src/lib.rs"
}

plant_prose_only() {
  {
    printf '//! unmigrated_measure(b, t) is the transitional alias.\n'
    printf 'pub const WHY: &str = "unmigrated_classify(b, t)";\n'
    printf 'use topo::test_support::unmigrated_measure;\n'
  } >> "$1/crates/clean/src/lib.rs"
}

gate_selftest() {
  gate_selftest_clean
  gate_selftest_without_tool python3 "the counter failed"
  gate_selftest_case "sites rose" plant_new_site
  gate_selftest_case "sites rose" plant_new_test_site
  gate_selftest_case "sites fell" plant_fell_unrecorded
  gate_selftest_passes "a site at its baseline beside the alias's declaration" plant_at_baseline
  gate_selftest_passes "prose, a string literal and a use naming the alias" plant_prose_only
  printf '%s selftest OK: passes a clean fixture, one at its baseline beside the declaration, and prose, string and use mentions; fires on a new site in src or tests and on an unrecorded fall, and diagnoses a counter that cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
