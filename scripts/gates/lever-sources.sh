#!/usr/bin/env bash
# lever-sources.sh — a verdict's lever arm names its source. ONE home;
# ci.yml's `lint` job runs every gate in this directory.
#
# THE RULE (`crates/geom-core/README.md`, "Float rounding", ratified on
# PR 6329): `Margin::levered` takes a `geom_core::Lever`, built from a
# read of the consumed extent (`geom_brep::Reach` and its ball payload
# `ExtentBall`), a curvature arm, or the session box.
#
# WHAT THIS GATE HOLDS. The type has no constructor that takes a bare
# length without naming a source. Three doors do take one, and this
# gate holds each:
#
#  - `Lever::of_consumed_extent`, the extent's own door, is referenced
#    only in the extent's home (geom-brep's `extent.rs`, the whole file)
#    and at the items EXTENT_ITEMS names below it: its definition in
#    geom-core's `predicate.rs`, and the spiric's torus ball in geom's
#    `curves.rs` (`geom` sits below `geom_brep`, so `ExtentBall::of_torus`
#    is out of reach there), each pinned to its count.
#  - `unsourced` (`Lever::unsourced`, `ExtentBall::unsourced`): an arm
#    that fits no source, each a question for Ev. Its references are
#    PINNED PER FILE (`lever-sources.counts`), so a pinned site swapped
#    for a new one in another file moves two pins where a reviewer reads
#    them, and every one carries a `// Unsourced lever: …` line on its
#    call or above it in the same statement, saying what the arm is and
#    why it fits no source.
#  - the fixture doors (`test_support::fixture_lever`, `Reach::fixture`):
#    a test's length with no geometry behind it, behind test-only cfgs.
#    Their references are pinned per crate (`lever-fixtures.counts`)
#    under that one reason: a test's stated scale.
#
# WHAT REVIEW HOLDS. A geometric constructor takes its geometry's
# numbers (a sphere's radius, a box's corners, a curvature, an edge's
# end points), so no gate can tell a caller's honest radius from a
# length it calls one; nor a rate passed through `Margin::levered` as
# if it were an arm; nor a dimensionless reading levered by hand outside
# the door (`Margin::of(x * arm)`). Review holds those.
#
# WHAT IT COUNTS: every reference in code (comments and string literals
# stripped, `gate_rust_code`) under every crate's src, tests, examples
# and benches, and under demos/ and benches/. The counter is
# `measure_door_census.py`. A `use … as` rename of `Lever` is still
# counted, since the method name is what is matched.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"
CENSUS="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/measure_door_census.py"

BASELINE=scripts/gates/lever-sources.counts
FIXTURES=scripts/gates/lever-fixtures.counts
EXTENT_HOME=crates/geom-brep/src/extent.rs
# file count: the items below the home that read the extent's door.
EXTENT_ITEMS=(
  'crates/geom/src/curves.rs 1'
)

gate() {
  gate_require_file "$BASELINE"
  gate_require_file "$FIXTURES"
  gate_require_crate_sources
  local files report code doors item path want got
  mapfile -t files < <(find crates/*/src crates/*/tests crates/*/examples crates/*/benches demos benches \
    -type f -name '*.rs' -not -path '*/target/*' 2>/dev/null | sort)
  GATE_SCAN_FILES=${#files[@]}
  if ! code=$(gate_rust_code "${files[@]}"); then
    gate_error "$(gate_name): could not read the sources under $PWD — the gate could not read its subject, which is not a pass"
    exit 1
  fi
  # The door's references: its definition is not one.
  doors=$(grep -E '\bof_consumed_extent\b' <<<"$code" | grep -vE '\bfn of_consumed_extent\b' \
    | grep -v "^$EXTENT_HOME:" || true)
  for item in "${EXTENT_ITEMS[@]}"; do
    read -r path want <<<"$item"
    got=$(grep -c "^$path:" <<<"$doors" || true)
    if [[ $got != "$want" ]]; then
      gate_error "Lever::of_consumed_extent is read $got time(s) in $path, pinned at $want: the item below the extent's home moved"
      exit 1
    fi
    doors=$(grep -v "^$path:" <<<"$doors" || true)
  done
  if [[ -n $doors ]]; then
    cut -d: -f1,2 <<<"$doors"
    gate_error "Lever::of_consumed_extent is read outside the extent's home: build the lever from a geom_brep::Reach or ExtentBall read, a curvature arm or the session box"
    exit 1
  fi
  # The escape hatches' own definitions are not references.
  if ! report=$(grep -vE '\bfn (unsourced|fixture_lever|fixture)\b' <<<"$code" \
    | python3 "$CENSUS" "$BASELINE" unsourced --reasons '--marker=// Unsourced lever:' --per-file); then
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
    gate_error "the unsourced levers moved — set $BASELINE to the new per-file counts in the change that moves them, so the move is read beside its reasons"
    exit 1
  fi
  local fixtures
  if ! fixtures=$(grep -vE '\bfn (unsourced|fixture_lever|fixture)\b' <<<"$code" \
    | python3 "$CENSUS" "$FIXTURES" fixture_lever 'Reach::fixture'); then
    gate_error "$(gate_name): the fixture counter failed under $PWD — the gate could not read its subject, which is not a pass"
    exit 1
  fi
  if grep -qE '^(ROSE|FELL) ' <<<"$fixtures"; then
    grep -E '^(ROSE|FELL) ' <<<"$fixtures"
    gate_error "the fixture levers moved — set $FIXTURES to the new per-crate counts in the change that moves them"
    exit 1
  fi
  gate_ok "the extent's door in its home and its pinned items, the unsourced levers at their per-file pins, each with its reason ($(sed -n 's/^TOTAL //p' <<<"$report") sites), and the fixture levers at their pins ($(sed -n 's/^TOTAL //p' <<<"$fixtures"))"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/scripts/gates" "$1/crates/geom/src"
  printf '# file count\n' > "$1/$BASELINE"
  printf '# crate count\n' > "$1/$FIXTURES"
  printf 'pub fn spiric() -> f64 { Lever::of_consumed_extent(1.0).length() }\n' \
    > "$1/crates/geom/src/curves.rs"
}

plant_reasoned_at_pin() {
  printf 'crates/clean/src/lib.rs 1\n' >> "$1/$BASELINE"
  {
    printf 'pub fn v(x: f64) -> f64 {\n'
    printf '    // Unsourced lever: a probe length no geometry names.\n'
    printf '    Margin::levered(x, Lever::unsourced(2.0)).value()\n}\n'
  } >> "$1/crates/clean/src/lib.rs"
}

plant_unreasoned() {
  printf 'crates/clean/src/lib.rs 1\n' >> "$1/$BASELINE"
  printf 'pub fn v(x: f64) -> f64 { Margin::levered(x, Lever::unsourced(2.0)).value() }\n' \
    >> "$1/crates/clean/src/lib.rs"
}

plant_new_site() {
  printf '// Unsourced lever: a probe.\npub fn v(c: P) -> B { ExtentBall::unsourced(c, 1.0) }\n' \
    >> "$1/crates/clean/src/lib.rs"
}

# A pinned site moved to another file of the same crate moves two pins.
plant_swapped_file() {
  printf 'crates/clean/src/lib.rs 1\n' >> "$1/$BASELINE"
  mkdir -p "$1/crates/clean/src/other"
  printf '// Unsourced lever: a probe.\npub fn v(c: P) -> B { ExtentBall::unsourced(c, 1.0) }\n' \
    > "$1/crates/clean/src/other/mod.rs"
}

plant_extent_door_outside_home() {
  printf 'pub fn v(x: f64) -> Lever<f64> { Lever::of_consumed_extent(x) }\n' \
    >> "$1/crates/clean/src/lib.rs"
}

plant_second_door_in_an_item() {
  printf 'pub fn w() -> f64 { Lever::of_consumed_extent(2.0).length() }\n' \
    >> "$1/crates/geom/src/curves.rs"
}

plant_new_fixture() {
  mkdir -p "$1/crates/clean/tests"
  printf 'fn t() { let _ = fixture_lever(1.0); }\n' > "$1/crates/clean/tests/t.rs"
}

gate_selftest() {
  gate_selftest_clean
  gate_selftest_without_tool python3 "the counter failed"
  gate_selftest_passes "a reasoned unsourced lever at its pin" plant_reasoned_at_pin
  gate_selftest_case "with no reason" plant_unreasoned
  gate_selftest_case "unsourced levers moved" plant_new_site
  gate_selftest_case "unsourced levers moved" plant_swapped_file
  gate_selftest_case "outside the extent's home" plant_extent_door_outside_home
  gate_selftest_case "the item below the extent's home moved" plant_second_door_in_an_item
  gate_selftest_case "fixture levers moved" plant_new_fixture
  printf '%s selftest OK: passes a clean fixture and a reasoned unsourced lever at its pin; fires on an unreasoned one, a new one, one swapped into another file, the extent door outside its home or a second time in a pinned item, and a new fixture lever; diagnoses a counter that cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
