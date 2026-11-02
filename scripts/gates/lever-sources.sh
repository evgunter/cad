#!/usr/bin/env bash
# lever-sources.sh — a verdict's lever arm is built only from its
# sources. ONE home; ci.yml's `lint` job runs every gate in this
# directory.
#
# THE RULE (`crates/geom-core/README.md`, "Float rounding", ratified on
# PR 6329): `Margin::levered` takes a `geom_core::Lever`, built only
# from a read of the consumed extent (`geom_brep::Reach` and its ball
# payload `ExtentBall`), a curvature arm, or the session box. The type
# closes every door but two, and this gate holds those two:
#
#  - `Lever::of_consumed_extent`, the extent's own door, is referenced
#    only in its definition (geom-core's predicate.rs), in the extent's
#    home (geom-brep's extent.rs), and at the one bounded carrier minted
#    below that home (geom's curves.rs, the spiric's torus ball). A
#    reference anywhere else is a lever minted from a length no geometry
#    names.
#  - `unsourced` (`Lever::unsourced`, `ExtentBall::unsourced`) is an arm
#    no source is named for yet, each a question for Ev (a fourth
#    source is his call). Its references are PINNED per crate
#    (`lever-sources.counts`) and every one carries a
#    `// Unsourced lever: …` line on its call or above it in the same
#    statement, saying what the arm is and why it fits no source. The
#    pin only falls: a site that finds its source moves it down in the
#    change that finds it.
#
# WHAT IT COUNTS: every reference in code (comments and string literals
# stripped, `gate_rust_code`) under every crate's src, tests, examples
# and benches, and under demos/ and benches/. The counter is
# `measure_door_census.py`. WHAT IT CANNOT SEE: a `use … as` rename of
# `Lever` itself (`L::unsourced` is still counted, since the method name
# is what is matched); a rate passed through `Margin::levered` as if it
# were an arm, which only review catches.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"
CENSUS="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/measure_door_census.py"

BASELINE=scripts/gates/lever-sources.counts
EXTENT_HOMES=(
  crates/geom-core/src/predicate.rs
  crates/geom-brep/src/extent.rs
  crates/geom/src/curves.rs
)

gate() {
  gate_require_file "$BASELINE"
  gate_require_crate_sources
  local files report code stray
  mapfile -t files < <(find crates/*/src crates/*/tests crates/*/examples crates/*/benches demos benches \
    -type f -name '*.rs' -not -path '*/target/*' 2>/dev/null | sort)
  GATE_SCAN_FILES=${#files[@]}
  if ! code=$(gate_rust_code "${files[@]}"); then
    gate_error "$(gate_name): could not read the sources under $PWD — the gate could not read its subject, which is not a pass"
    exit 1
  fi
  stray=$(grep -E '\bof_consumed_extent\b' <<<"$code" \
    | grep -vE "^($(IFS='|'; echo "${EXTENT_HOMES[*]}")):" || true)
  if [[ -n $stray ]]; then
    cut -d: -f1,2 <<<"$stray"
    gate_error "Lever::of_consumed_extent is read outside the extent's home: build the lever from a geom_brep::Reach or ExtentBall read, a curvature arm or the session box"
    exit 1
  fi
  # The two doors' own definitions are not references.
  if ! report=$(grep -vE '\bfn unsourced\b' <<<"$code" \
    | python3 "$CENSUS" "$BASELINE" unsourced --reasons '--marker=// Unsourced lever:'); then
    gate_error "$(gate_name): the counter failed under $PWD (its own message is above, if it had one) — the gate could not read its subject, which is not a pass"
    exit 1
  fi
  if grep -q '^UNREASONED ' <<<"$report"; then
    grep '^UNREASONED ' <<<"$report"
    gate_error "an unsourced lever with no reason: put a '// Unsourced lever: …' line on the call or above it in the same statement, saying what the arm is and why it fits no source"
    exit 1
  fi
  if grep -qE '^(ROSE|FELL) ' <<<"$report"; then
    grep -E '^(ROSE|FELL) ' <<<"$report"
    gate_error "the unsourced levers moved — set $BASELINE to the new counts in the change that moves them, so the move is read beside its reasons"
    exit 1
  fi
  gate_ok "the extent's door in its home, and the unsourced levers at their pinned counts, each with its reason ($(sed -n 's/^TOTAL //p' <<<"$report") sites in all)"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/scripts/gates"
  printf '# crate count\n' > "$1/$BASELINE"
}

plant_reasoned_at_pin() {
  printf 'crates/clean 1\n' >> "$1/$BASELINE"
  {
    printf 'pub fn v(x: f64) -> f64 {\n'
    printf '    // Unsourced lever: a probe length no geometry names.\n'
    printf '    Margin::levered(x, Lever::unsourced(2.0)).value()\n}\n'
  } >> "$1/crates/clean/src/lib.rs"
}

plant_unreasoned() {
  printf 'crates/clean 1\n' >> "$1/$BASELINE"
  printf 'pub fn v(x: f64) -> f64 { Margin::levered(x, Lever::unsourced(2.0)).value() }\n' \
    >> "$1/crates/clean/src/lib.rs"
}

plant_new_site() {
  printf '// Unsourced lever: a probe.\npub fn v(c: P) -> B { ExtentBall::unsourced(c, 1.0) }\n' \
    >> "$1/crates/clean/src/lib.rs"
}

plant_extent_door_outside_home() {
  printf 'pub fn v(x: f64) -> Lever<f64> { Lever::of_consumed_extent(x) }\n' \
    >> "$1/crates/clean/src/lib.rs"
}

gate_selftest() {
  gate_selftest_clean
  gate_selftest_without_tool python3 "the counter failed"
  gate_selftest_passes "a reasoned unsourced lever at its pin" plant_reasoned_at_pin
  gate_selftest_case "with no reason" plant_unreasoned
  gate_selftest_case "unsourced levers moved" plant_new_site
  gate_selftest_case "outside the extent's home" plant_extent_door_outside_home
  printf '%s selftest OK: passes a clean fixture and a reasoned unsourced lever at its pin; fires on an unreasoned one, on a new one, and on the extent door read outside its home, and diagnoses a counter that cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
