#!/usr/bin/env bash
# lever-doors.sh — no production code builds a lever from a length it
# wrote down. ONE home; ci.yml's `lint` job runs every gate in this
# directory.
#
# THE RULE (`crates/geom-core/README.md`, "Float rounding"): a verdict's
# lever arm is built only from its sources, and the consumed extent it
# is read off is built "never from a length a caller vouches for". A
# constructor that takes the carrier it reads (a surface, a circle, a
# cover, a carrier over its span) holds that by its signature. The doors
# below take a number or a point the caller computed instead, so each is
# referenced only in its home, and elsewhere at a PIN that only falls.
#
# THE DOORS (`lever_doors.py`'s `DOORS`, each with its home):
#
#  - the bare curvature arms, `Lever::curvature_radius` and
#    `Lever::within_curvature`: home, geom-brep's `curvature_arm.rs`,
#    whose readers take the carrier;
#  - an extent of the caller's own points: `ExtentBall::point`,
#    `ExtentBall::of_box`, `ExtentBall::box_diagonal`,
#    `points_lever_from`, `points_diameter`;
#  - an extent widened, scaled or measured by the caller's number:
#    `ExtentBall::of_cover_within`, `Reach::of_face`, and geom-brep's
#    crate-private `lever_from_within`, `of_crossing_cylinders`,
#    `hinge_lever`, `turn_lever`, `ruled_lever`, `generator_lever`:
#    home, geom-brep's `extent.rs`;
#  - `Lever`'s own tuple literal, which geom-core can write anywhere in
#    its sources: home, geom-core's `predicate.rs`.
#
# `lever-sources.sh` holds the doors that name no source at all
# (`of_consumed_extent`, `unsourced`, the fixtures); this gate holds the
# ones that look geometric.
#
# THE PINS (`lever-doors.counts`, `PATH COUNT`): the sites that still
# hand a door a number or a point, each owed a constructor that reads
# its geometry. A pin never rises above the merge base's (GATE_BASE), so
# a new site is red; a file git moved keeps its pin, and a file split
# from another cites it (`# from OLD_PATH`).
#
# WHAT IT COUNTS: every reference in production code (comments, string
# literals and `#[cfg(test)]` items stripped, test-only mounts dropped)
# under every crate's src, and under demos/ and benches/. A test may
# hand a door its stated scale.
#
# WHAT IT CANNOT SEE, which review holds:
#  - geometry built by hand: a `Surface`, a `Curve3`, a parameter span
#    or a `geom::cover::Cover` constructed from the caller's numbers and
#    handed to a reading constructor;
#  - the pivot: `lever_from(p)` reads the extent's far distance from
#    any point the caller names, so a far pivot is a long lever;
#  - a door called through a macro's expansion, or as `Self::door(…)`
#    in a trait impl for `geom_brep::Reach` in another crate (impls of
#    `Lever` and `ExtentBall`, and renames or aliases of the three
#    types, are red);
#  - a lever's length read back (`Lever::length`) and multiplied by
#    hand outside `Margin::levered`.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"
CENSUS="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/lever_doors.py"

PINS=scripts/gates/lever-doors.counts
RULE="read the lever off the carrier it comes from (geom_brep::curvature_arm's readers, ExtentBall::of_cover of the consumed entity's cover, Reach::of_span), not off a number or a point the call site computed"

gate() {
  gate_require_file "$PINS"
  gate_require_crate_sources
  gate_production_sources
  local files report extra base_note="no base given (GATE_BASE unset), so no pin was compared with one"
  local base_args=() renames
  mapfile -t extra < <(find demos benches -type f -name '*.rs' -not -path '*/target/*' \
    -not -path '*/tests/*' 2>/dev/null | sort)
  files=("${GATE_PRODUCTION_FILES[@]}" ${extra[@]+"${extra[@]}"})
  GATE_SCAN_FILES=${#files[@]}
  renames=$(mktemp)
  if [ -n "${GATE_BASE:-}" ] && git cat-file -e "$GATE_BASE^{tree}" 2>/dev/null \
    && ! git cat-file -e "$GATE_BASE:./$PINS" 2>/dev/null; then
    base_note="$PINS is new since the base $GATE_BASE, so its pins are this change's own"
  elif gate_base_file "$PINS"; then
    if ! git diff -M --name-status "$GATE_BASE" -- crates demos benches > "$renames.raw" 2>/dev/null; then
      rm -f "$renames" "$renames.raw"
      gate_error "$(gate_name): git could not list the renames since the base $GATE_BASE, so a moved file's pin could not be followed — that is not a pass"
      exit 1
    fi
    awk '$1 ~ /^R/ { print $2, $3 }' "$renames.raw" > "$renames"
    rm -f "$renames.raw"
    base_args=(--base "$GATE_BASE_FILE" --renames "$renames")
    base_note="none above the base $GATE_BASE's"
  fi
  if ! report=$(gate_rust_code --skip-cfg-test "${files[@]}" \
    | python3 "$CENSUS" "$PINS" ${LINES_FOR:+--lines "$LINES_FOR"} ${base_args[@]+"${base_args[@]}"}); then
    rm -f "$renames"
    gate_error "$(gate_name): the counter failed under $PWD (its own message is above, if it had one) — the gate could not read its subject, which is not a pass"
    exit 1
  fi
  rm -f "$renames"
  if ! grep -q '^TOTAL [0-9]' <<<"$report"; then
    gate_error "$(gate_name): the counter exited without its TOTAL line — it did not finish its census, which is not a pass"
    exit 1
  fi
  grep '^SITE ' <<<"$report" || true
  local failed=0
  if grep -q '^MALFORMED ' <<<"$report"; then
    grep '^MALFORMED ' <<<"$report"
    gate_error "$PINS has a line it cannot read, or a second line for one file (above): each is \`PATH COUNT\`"
    failed=1
  fi
  if grep -q '^ROSE ' <<<"$report"; then
    grep '^ROSE ' <<<"$report"
    gate_error "a lever door that takes a caller's number or point is referenced past its pin (above; LINES_FOR=FILE lists a file's counted sites) — $RULE"
    failed=1
  fi
  if grep -q '^RAISED ' <<<"$report"; then
    grep '^RAISED ' <<<"$report"
    gate_error "a pin is above the merge base's (above), and a pin only falls. A file moved by git keeps its pin; a file split from another cites it (\`# from OLD_PATH\`). Otherwise — $RULE"
    failed=1
  fi
  if grep -q '^FELL ' <<<"$report"; then
    grep '^FELL ' <<<"$report"
    gate_error "a pinned site is gone (above): drop or lower its line in $PINS"
    failed=1
  fi
  if grep -q '^ALIAS ' <<<"$report"; then
    grep '^ALIAS ' <<<"$report"
    gate_error "Lever, ExtentBall or Reach is renamed or aliased (above), which takes its doors' references out of this count: spell the type by its own name"
    failed=1
  fi
  if grep -q '^IMPL ' <<<"$report"; then
    grep '^IMPL ' <<<"$report"
    gate_error "an impl of Lever, ExtentBall or Reach outside its home (above), where \`Self::door(…)\` is out of this count's sight: the type's constructors live in its home"
    failed=1
  fi
  [ "$failed" -eq 0 ] || exit 1
  gate_ok "the lever doors that take a caller's number or point are referenced only in their homes and at their pins ($(sed -n 's/^TOTAL //p' <<<"$report") pinned sites; $base_note)"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/scripts/gates" "$1/crates/geom-brep/src" "$1/crates/geom-core/src"
  printf '# path count\n' > "$1/$PINS"
  printf 'pub fn circle_curvature_arm(c: &Circle) -> Lever<f64> { Lever::curvature_radius(c.radius) }\n' \
    > "$1/crates/geom-brep/src/curvature_arm.rs"
  {
    printf 'impl<T> ExtentBall<T> {\n    pub fn point(p: P) -> Self { Self::new(p, 0.0) }\n}\n'
    printf 'pub fn points_diameter(p: &[P]) -> Lever<f64> { Lever::of_consumed_extent(far(p)) }\n'
  } > "$1/crates/geom-brep/src/extent.rs"
  {
    printf 'pub struct Lever<T>(pub(crate) T);\n'
    printf 'impl<T> Lever<T> {\n    pub fn curvature_radius(r: T) -> Self { Self(r) }\n}\n'
  } > "$1/crates/geom-core/src/predicate.rs"
}

plant_at_pin() {
  printf 'crates/clean/src/lib.rs 1\n' >> "$1/$PINS"
  printf 'pub fn v(p: P, q: P) -> Lever<f64> { geom_brep::ExtentBall::point(q).lever_from(p) }\n' \
    >> "$1/crates/clean/src/lib.rs"
}

plant_in_a_test() {
  printf '#[cfg(test)]\nmod tests {\n    fn t() { let _ = Lever::curvature_radius(1.0); }\n}\n' \
    >> "$1/crates/clean/src/lib.rs"
}

plant_bare_radius() {
  printf 'pub fn v(len: f64) -> Lever<f64> { Lever::curvature_radius(len) }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_turbofish_point() {
  printf 'pub fn v(p: P, q: P) -> Lever<f64> { ExtentBall::<f64>::point(q).lever_from(p) }\n' \
    >> "$1/crates/clean/src/lib.rs"
}

plant_curvature_in_the_extent_home() {
  printf 'pub fn w(len: f64) -> Lever<f64> { Lever::session_box().within_curvature(1.0 / len) }\n' \
    >> "$1/crates/geom-brep/src/extent.rs"
}

plant_fn_pointer() {
  printf 'pub fn v(ps: &[P]) -> Vec<B> { ps.iter().copied().map(geom_brep::ExtentBall::point).collect() }\n' \
    >> "$1/crates/clean/src/lib.rs"
}

plant_tuple_literal() {
  printf 'pub fn v(len: f64) -> Lever<f64> { Lever(len) }\n' > "$1/crates/geom-core/src/other.rs"
}

plant_alias() {
  printf 'use geom_brep::ExtentBall as Ball;\npub fn v(q: P) -> Ball { Ball::point(q) }\n' \
    >> "$1/crates/clean/src/lib.rs"
}

plant_impl_outside_home() {
  printf 'impl<T> ExtentBall<T> {\n    pub fn sneak(q: P) -> Self { Self::point(q) }\n}\n' \
    > "$1/crates/geom-brep/src/other.rs"
}

plant_stale_pin() {
  printf 'crates/clean/src/lib.rs 1\n' >> "$1/$PINS"
}

plant_malformed_pin() {
  printf 'crates/clean/src/lib.rs one\n' >> "$1/$PINS"
}

gate_selftest() {
  gate_selftest_clean
  gate_selftest_without_tool python3 "the counter failed"
  gate_selftest_passes "a point extent at its pin" plant_at_pin
  gate_selftest_passes "a bare radius in a test" plant_in_a_test
  gate_selftest_case "referenced past its pin" plant_bare_radius
  gate_selftest_case "referenced past its pin" plant_turbofish_point
  gate_selftest_case "referenced past its pin" plant_curvature_in_the_extent_home
  gate_selftest_case "referenced past its pin" plant_fn_pointer
  gate_selftest_case "referenced past its pin" plant_tuple_literal
  gate_selftest_case "renamed or aliased" plant_alias
  gate_selftest_case "outside its home" plant_impl_outside_home
  gate_selftest_case "a pinned site is gone" plant_stale_pin
  gate_selftest_case "it cannot read" plant_malformed_pin
  printf '%s selftest OK: passes a clean fixture, a pinned site and a door in a test; fires on a bare radius, a turbofish, a curvature arm in the extent home, a fn pointer, a tuple literal, an alias, an impl outside the home, a stale pin and a malformed one; diagnoses a counter that cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
