#!/usr/bin/env bash
# raw-measure-door-sites.sh — the measurement doors' raw names, per
# crate, pinned, each with its reason. ONE home; ci.yml's `lint` job
# runs every gate in this directory.
#
# THE RULE (FLUX unit F, Ev's decision on PR 4822): every name of the
# measurement door takes a finished body (`&AtRestBody`). A test whose
# subject is the walk on a body tier 3 refuses — an inside-out body, a
# flipped seam region, a corrupt arena — reads it through
# `topo::test_support`'s raw doors (`measure_unvalidated`,
# `classify_unvalidated` and their `_structural` / `_of` twins), and says
# why at the call: a `// Raw door: …` line on the reference or above it
# in the same statement. The walk up from a reference stops where
# another statement ends (`;`, `{`, `}`, a match arm's `,`, a blank
# line, a `fn` or an attribute), so a reason covers the statement it
# heads and no later one.
#
# So two things hold here. The sites are PINNED per crate
# (`raw-measure-door-sites.counts`): a new raw site moves the count in
# the change that adds it, where a reviewer reads its reason, and a site
# that leaves for the typed door moves it the other way. And every
# reference carries its reason.
#
# WHAT IT COUNTS: every REFERENCE to a raw name in code (comments and
# string literals stripped, `gate_rust_code`), under every crate's src,
# tests, examples and benches, and under demos/ and benches/ — a call, a
# turbofish, a fn pointer, a `(path)(…)` call and a `use … as` rename
# each spell the name once. The names' own declarations (topo's `pub use
# … as NAME;`) are not references. The counter is
# `measure_door_census.py`.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"
CENSUS="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/measure_door_census.py"

BASELINE=scripts/gates/raw-measure-door-sites.counts
NAMES=(measure_unvalidated measure_unvalidated_structural classify_unvalidated
  classify_unvalidated_structural classify_unvalidated_of)

gate() {
  gate_require_file "$BASELINE"
  gate_require_crate_sources
  local files report
  mapfile -t files < <(find crates/*/src crates/*/tests crates/*/examples crates/*/benches demos benches \
    -type f -name '*.rs' -not -path '*/target/*' 2>/dev/null | sort)
  GATE_SCAN_FILES=${#files[@]}
  if ! report=$(gate_rust_code "${files[@]}" \
    | python3 "$CENSUS" "$BASELINE" "${NAMES[@]}" --reasons); then
    gate_error "$(gate_name): the counter failed under $PWD (its own message is above, if it had one) — the gate could not read its subject, which is not a pass"
    exit 1
  fi
  if grep -q '^UNREASONED ' <<<"$report"; then
    grep '^UNREASONED ' <<<"$report"
    gate_error "a raw measurement door is read with no reason: put a '// Raw door: …' line on the call or above it in the same statement, saying which refusal of tier 3 the measured body is the subject of"
    exit 1
  fi
  if grep -qE '^(ROSE|FELL) ' <<<"$report"; then
    grep -E '^(ROSE|FELL) ' <<<"$report"
    gate_error "the raw measurement doors' sites moved — set $BASELINE to the new counts in the change that moves them, so the move is read beside its reasons"
    exit 1
  fi
  gate_ok "the raw measurement doors at their pinned counts, each with its reason ($(sed -n 's/^TOTAL //p' <<<"$report") sites in all)"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/scripts/gates"
  printf '# crate count\n' > "$1/$BASELINE"
}

plant_reasoned_at_pin() {
  printf 'crates/clean 2\n' >> "$1/$BASELINE"
  {
    printf 'pub fn v(b: &B) -> f64 {\n'
    printf '    // Raw door: an inside-out body, which tier 3 refuses (NegativeVolume).\n'
    printf '    measure_unvalidated(b, t()).volume\n}\n'
    printf '// Raw door: a corrupt corpus, read as the dump reads it.\n'
    printf 'pub const M: fn(&B, T) -> R = topo::test_support::classify_unvalidated_of;\n'
  } >> "$1/crates/clean/src/lib.rs"
  mkdir -p "$1/crates/topo/src"
  printf 'pub mod test_support {\n    pub use crate::props::raw_door::measure_body as measure_unvalidated;\n}\n' \
    > "$1/crates/topo/src/lib.rs"
}

plant_unreasoned() {
  printf 'crates/clean 1\n' >> "$1/$BASELINE"
  printf 'pub fn v(b: &B) -> f64 { measure_unvalidated(b, t()).volume }\n' \
    >> "$1/crates/clean/src/lib.rs"
}

# A reason in another statement, past a blank line, is not this site's.
plant_reason_too_far() {
  printf 'crates/clean 1\n' >> "$1/$BASELINE"
  {
    printf '// Raw door: an inside-out body, which tier 3 refuses (NegativeVolume).\n'
    printf 'pub const A: u8 = 0;\n\n'
    printf 'pub fn v(b: &B) -> R { topo::test_support::classify_unvalidated(b, t()) }\n'
  } >> "$1/crates/clean/src/lib.rs"
}

# An unreasoned fn after a reasoned one does not borrow its reason.
plant_fn_after_a_reasoned_one() {
  printf 'crates/clean 2\n' >> "$1/$BASELINE"
  {
    printf 'pub fn v(b: &B) -> R {\n'
    printf '    // Raw door: an inside-out body, which tier 3 refuses (NegativeVolume).\n'
    printf '    measure_unvalidated(b, t())\n}\n'
    printf 'pub fn w(b: &B) -> R { measure_unvalidated(b, t()) }\n'
  } >> "$1/crates/clean/src/lib.rs"
}

# A second call, on another body, under the first call's reason.
plant_second_call_under_a_reason() {
  printf 'crates/clean 2\n' >> "$1/$BASELINE"
  {
    printf 'pub fn v(bad: &B, good: &B) {\n'
    printf '    // Raw door: an inside-out body, which tier 3 refuses (NegativeVolume).\n'
    printf '    let a = measure_unvalidated(bad, t());\n'
    printf '    let b = measure_unvalidated(good, t());\n}\n'
  } >> "$1/crates/clean/src/lib.rs"
}

# A reason above a statement covers that statement's continuation lines.
plant_reason_over_a_long_statement() {
  printf 'crates/clean 2\n' >> "$1/$BASELINE"
  {
    printf 'pub fn v(a: &B, b: &B) {\n'
    printf '    // Raw door: a mixed pair, and tier 3 refuses the second (NegativeVolume).\n'
    printf '    assert_eq!(\n'
    printf '        measure_unvalidated(a, t()).is_ok(),\n'
    printf '        measure_unvalidated(b, t()).is_ok(),\n'
    printf '    );\n}\n'
  } >> "$1/crates/clean/src/lib.rs"
}

plant_new_site() {
  {
    printf '// Raw door: an inside-out body, which tier 3 refuses (NegativeVolume).\n'
    printf 'pub fn v(b: &B) -> R { (topo::test_support::measure_unvalidated_structural)(b, t()) }\n'
  } >> "$1/crates/clean/src/lib.rs"
}

plant_use_as() {
  printf '// Raw door: renamed.\nuse topo::test_support::measure_unvalidated as m;\n' \
    >> "$1/crates/clean/src/lib.rs"
}

plant_example_site() {
  mkdir -p "$1/crates/clean/examples"
  printf '// Raw door: a probe of a refused body.\nfn main() { let _ = topo::test_support::classify_unvalidated_structural::<f64>(&b(), t()); }\n' \
    > "$1/crates/clean/examples/probe.rs"
}

plant_fell() {
  printf 'crates/clean 3\n' >> "$1/$BASELINE"
  printf '// Raw door: x.\npub fn v(b: &B) -> R { measure_unvalidated(b, t()) }\n' \
    >> "$1/crates/clean/src/lib.rs"
}

gate_selftest() {
  gate_selftest_clean
  gate_selftest_without_tool python3 "the counter failed"
  gate_selftest_passes "reasoned sites at their pin beside the names' declaration" plant_reasoned_at_pin
  gate_selftest_case "with no reason" plant_unreasoned
  gate_selftest_case "with no reason" plant_reason_too_far
  gate_selftest_case "with no reason" plant_fn_after_a_reasoned_one
  gate_selftest_case "with no reason" plant_second_call_under_a_reason
  gate_selftest_passes "a reason above a statement over its continuation lines" plant_reason_over_a_long_statement
  gate_selftest_case "sites moved" plant_new_site
  gate_selftest_case "sites moved" plant_use_as
  gate_selftest_case "sites moved" plant_example_site
  gate_selftest_case "sites moved" plant_fell
  printf '%s selftest OK: passes a clean fixture and reasoned sites at their pin beside the declaration; fires on a site with no reason, a reason in another statement, an unreasoned fn after a reasoned one or a second call under the reason of another, passes a reason over the continuation lines of its statement; fires on a new site spelled as a (path)(…) call, a use-as rename or an examples/ turbofish, and on a fall, and diagnoses a counter that cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
