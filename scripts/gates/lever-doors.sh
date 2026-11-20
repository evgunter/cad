#!/usr/bin/env bash
# lever-doors.sh — a ratchet on the lever doors that take a caller's
# number or point. ONE home; ci.yml's `lint` job runs every gate in this
# directory.
#
# THE RULE it serves (`crates/geom-core/README.md`, "Float rounding"): a
# verdict's lever arm is built only from its sources, and the consumed
# extent it is read off "never from a length a caller vouches for".
#
# WHAT THIS GATE HOLDS, and it is less than the rule: the named doors
# below are referenced by name only in their homes, and elsewhere at a
# PIN per file and door that only falls. So a new reference to a named
# door is red, and so is one door's site swapped for another's in a
# file. It does not hold that no length a caller vouches for becomes a
# lever: see WHAT IT DOES NOT HOLD.
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
#    `ExtentBall::of_cover_within`, `lever_from_point_and_plane` (its
#    normal is the caller's, unit or not), `Reach::of_face`, a
#    `Reach::Measured` or `Reach::Face` literal, and geom-brep's
#    crate-private `lever_from_within`, `of_crossing_cylinders`,
#    `hinge_lever`, `turn_lever`, `ruled_lever`, `generator_lever`:
#    home, geom-brep's `extent.rs`.
#
# `Lever`'s own field is private to its module, so no gate holds it.
# `lever-sources.sh` holds the doors that name no source at all
# (`of_consumed_extent`, `unsourced`, the fixtures).
#
# THE PINS (`lever-doors.counts`, `DOOR COUNT FILE`, read by
# `pin_ledger.py` as `cover-kind-match`'s are): the sites that still
# hand a door a number or a point. A pin never rises above the merge
# base's (GATE_BASE); a file git moved keeps its pins, and a file split
# from another cites it (`# from OLD_PATH`), its pins summed with the
# origin's and held to the origin's base pin.
#
# WHAT IT COUNTS: every reference in production code (comments, string
# literals and `#[cfg(test)]` items stripped, test-only mounts dropped,
# read a statement at a time so a call split over lines is one text)
# under every crate's src, and under demos/ and benches/. A turbofish on
# one of the three types and a qualified path to one are read as the
# type. A test may hand a door its stated scale.
#
# WHAT IT DOES NOT HOLD, which review and the residue row
# (`work/oneround/lever-doors-pinned-sites-await-a-geometry-read.md`)
# carry:
#  - a WRAPPER: a `pub fn` in a pinned file that takes a number or a
#    point and hands it to its pinned site is callable from any file,
#    unpinned (the row lists the ones that exist);
#  - geometry built by hand: `Surface`, `Circle`, `Frame` and `Curve3`
#    have public fields and `geom::cover::Cover` public constructors, so
#    a caller's length reaches every reading constructor through a
#    carrier, a span or a cover it made;
#  - a caller's point: `lever_from(p)` reads from any pivot, and the
#    curvature home's `curvature_lever_arm(cone, p)` and
#    `parallel_curvature_arm(s, p)` read `p`'s distance from an axis;
#  - a door called through a macro's expansion, or as `Self::door(…)`
#    in a trait impl for `geom_brep::Reach` in another crate (impls of
#    `Lever` and `ExtentBall`, and renames or aliases of the three
#    types and of the held free fns, are red);
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
  if ! report=$(gate_rust_code --skip-cfg-test --statements "${files[@]}" \
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
  if grep -qE '^(MALFORMED|DUPLICATE) ' <<<"$report"; then
    grep -E '^(MALFORMED|DUPLICATE) ' <<<"$report"
    gate_error "$PINS has a line it cannot read, or a second line for one door and file (above): each is \`DOOR COUNT FILE\`, DOOR one of lever_doors.py's"
    failed=1
  fi
  if grep -q '^ROSE ' <<<"$report"; then
    grep '^ROSE ' <<<"$report"
    gate_error "a lever door that takes a caller's number or point is referenced past its pin (above; LINES_FOR=FILE lists a file's counted sites) — $RULE"
    failed=1
  fi
  if grep -q '^RAISED ' <<<"$report"; then
    grep '^RAISED ' <<<"$report"
    gate_error "a pin is above the merge base's (above), and a pin only falls. A file moved by git keeps its pins; a file split from another cites it (\`# from OLD_PATH\`), and its pins are summed with the origin's. Otherwise — $RULE"
    failed=1
  fi
  if grep -q '^FELL ' <<<"$report"; then
    grep '^FELL ' <<<"$report"
    gate_error "a pinned site is gone (above): drop or lower its line in $PINS"
    failed=1
  fi
  if grep -q '^ALIAS ' <<<"$report"; then
    grep '^ALIAS ' <<<"$report"
    gate_error "Lever, ExtentBall, Reach or a held free fn is renamed or aliased (above), which takes a door's references out of this count: spell it by its own name"
    failed=1
  fi
  if grep -q '^IMPL ' <<<"$report"; then
    grep '^IMPL ' <<<"$report"
    gate_error "an impl of Lever, ExtentBall or Reach outside its home (above), where \`Self::door(…)\` is out of this count's sight: the type's constructors live in its home"
    failed=1
  fi
  [ "$failed" -eq 0 ] || exit 1
  gate_ok "the named lever doors that take a caller's number or point are referenced only in their homes and at their pins ($(sed -n 's/^TOTAL //p' <<<"$report") pinned sites; $base_note)"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/scripts/gates" "$1/crates/geom-brep/src" "$1/crates/geom-core/src"
  printf '# door count file\n' > "$1/$PINS"
  printf 'pub fn circle_curvature_arm(c: &Circle) -> Lever<f64> { Lever::curvature_radius(c.radius) }\n' \
    > "$1/crates/geom-brep/src/curvature_arm.rs"
  {
    printf 'impl<T> ExtentBall<T> {\n    pub fn point(p: P) -> Self { Self::new(p, 0.0) }\n}\n'
    printf 'pub fn points_diameter(p: &[P]) -> Lever<f64> { Lever::of_consumed_extent(far(p)) }\n'
  } > "$1/crates/geom-brep/src/extent.rs"
  {
    printf 'pub struct Lever<T>(T);\n'
    printf 'impl<T> Lever<T> {\n    pub fn curvature_radius(r: T) -> Self { Self(r) }\n}\n'
  } > "$1/crates/geom-core/src/predicate.rs"
}

POINT_SITE='pub fn v(p: P, q: P) -> Lever<f64> { geom_brep::ExtentBall::point(q).lever_from(p) }\n'

plant_at_pin() {
  printf 'ExtentBall::point 1 crates/clean/src/lib.rs\n' >> "$1/$PINS"
  # shellcheck disable=SC2059
  printf "$POINT_SITE" >> "$1/crates/clean/src/lib.rs"
}

plant_in_a_test() {
  printf '#[cfg(test)]\nmod tests {\n    fn t() { let _ = Lever::curvature_radius(1.0); }\n}\n' \
    >> "$1/crates/clean/src/lib.rs"
}

plant_alias_of_a_holder() {
  printf 'type Unmated<T> = Vec<(K, geom_core::Lever<T>)>;\n' >> "$1/crates/clean/src/lib.rs"
}

plant_bare_radius() {
  printf 'pub fn v(len: f64) -> Lever<f64> { Lever::curvature_radius(len) }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_turbofish_over_lines() {
  printf 'pub fn v(q: P) -> B {\n    geom_brep::ExtentBall::<\n        f64,\n    >::point(q)\n}\n' \
    >> "$1/crates/clean/src/lib.rs"
}

plant_qualified_path() {
  printf 'pub fn v(p: P, q: P) -> Lever<f64> { <geom_brep::ExtentBall<f64>>::point(q).lever_from(p) }\n' \
    >> "$1/crates/clean/src/lib.rs"
}

plant_scaled_normal() {
  printf 'pub fn v(c: &C, p: P, n: V) -> Lever<f64> { ExtentBall::of_cover(c).lever_from_point_and_plane(p, (p, n)) }\n' \
    >> "$1/crates/clean/src/lib.rs"
}

plant_measured_literal() {
  mkdir -p "$1/crates/geom-brep/src"
  printf 'pub fn v(p: P, len: f64) -> Reach<f64> { Reach::Measured { at: p, lever: len } }\n' \
    > "$1/crates/geom-brep/src/other.rs"
}

plant_curvature_in_the_extent_home() {
  printf 'pub fn w(len: f64) -> Lever<f64> { Lever::session_box().within_curvature(1.0 / len) }\n' \
    >> "$1/crates/geom-brep/src/extent.rs"
}

plant_fn_pointer() {
  printf 'pub fn v(ps: &[P]) -> Vec<B> { ps.iter().copied().map(geom_brep::ExtentBall::point).collect() }\n' \
    >> "$1/crates/clean/src/lib.rs"
}

# One door's pinned site swapped for another door's: the file's total is
# unmoved, and two pins are.
plant_swapped_door() {
  printf 'ExtentBall::point 1 crates/clean/src/lib.rs\n' >> "$1/$PINS"
  printf 'pub fn v(len: f64) -> Lever<f64> { Lever::curvature_radius(len) }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_alias() {
  printf 'use geom_brep::ExtentBall as Ball;\npub fn v(q: P) -> Ball { Ball::point(q) }\n' \
    >> "$1/crates/clean/src/lib.rs"
}

plant_renamed_free_fn() {
  printf 'use geom_brep::points_diameter as pd;\npub fn v(a: &[P]) -> Lever<f64> { pd(a) }\npub fn w(a: &[P]) -> Lever<f64> { pd(a) }\n' \
    >> "$1/crates/clean/src/lib.rs"
}

plant_impl_outside_home() {
  printf 'impl<T> ExtentBall<T> {\n    pub fn sneak(q: P) -> Self { Self::point(q) }\n}\n' \
    > "$1/crates/geom-brep/src/other.rs"
}

plant_stale_pin() {
  printf 'ExtentBall::point 1 crates/clean/src/lib.rs\n' >> "$1/$PINS"
}

plant_malformed_pin() {
  printf 'crates/clean/src/lib.rs 1\n' >> "$1/$PINS"
}

# The fixture as a repository whose one commit pins a site in `held.rs`.
plant_base() {
  mkdir -p "$1/crates/held/src"
  # shellcheck disable=SC2059
  printf "$POINT_SITE" > "$1/crates/held/src/held.rs"
  printf 'ExtentBall::point 1 crates/held/src/held.rs\n' >> "$1/$PINS"
  git -C "$1" init -q
  git -C "$1" add -A
  git -C "$1" -c user.name=gate -c user.email=gate@localhost commit -qm base
}

plant_raised() {
  plant_base "$1"
  # shellcheck disable=SC2059
  printf "$POINT_SITE" | sed 's/fn v/fn w/' >> "$1/crates/held/src/held.rs"
  sed -i 's|^ExtentBall::point 1 crates/held/src/held.rs|ExtentBall::point 2 crates/held/src/held.rs|' "$1/$PINS"
}

plant_moved() {
  plant_base "$1"
  git -C "$1" mv crates/held/src/held.rs crates/held/src/new.rs
  sed -i 's|crates/held/src/held.rs|crates/held/src/new.rs|' "$1/$PINS"
}

# A site moved out of `held.rs` into a new file that cites it.
plant_split_cited() {
  plant_base "$1"
  git -C "$1" mv crates/held/src/held.rs crates/held/src/half.rs
  printf 'pub fn identity(x: f64) -> f64 { x }\n' > "$1/crates/held/src/held.rs"
  sed -i 's|^ExtentBall::point 1 crates/held/src/held.rs|ExtentBall::point 1 crates/held/src/half.rs # from crates/held/src/held.rs|' "$1/$PINS"
  # Not a rename git reports: the origin still exists.
  git -C "$1" add -A
}

# A NEW site in a new file, citing a pinned file whose own pin is unmoved.
plant_new_site_citing() {
  plant_base "$1"
  # shellcheck disable=SC2059
  printf "$POINT_SITE" > "$1/crates/held/src/extra.rs"
  printf 'ExtentBall::point 1 crates/held/src/extra.rs # from crates/held/src/held.rs\n' >> "$1/$PINS"
}

# An existing pin raised, citing another pinned file.
plant_raised_citing() {
  plant_base "$1"
  mkdir -p "$1/crates/held/src"
  # shellcheck disable=SC2059
  printf "$POINT_SITE" > "$1/crates/held/src/other.rs"
  printf 'ExtentBall::point 1 crates/held/src/other.rs\n' >> "$1/$PINS"
  git -C "$1" add -A
  git -C "$1" -c user.name=gate -c user.email=gate@localhost commit -qm second
  # shellcheck disable=SC2059
  printf "$POINT_SITE" | sed 's/fn v/fn w/' >> "$1/crates/held/src/other.rs"
  sed -i 's|^ExtentBall::point 1 crates/held/src/other.rs|ExtentBall::point 2 crates/held/src/other.rs # from crates/held/src/held.rs|' "$1/$PINS"
}

gate_selftest() {
  unset GATE_BASE
  gate_selftest_clean
  gate_selftest_without_tool python3 "the counter failed"
  gate_selftest_passes "a point extent at its pin" plant_at_pin
  gate_selftest_passes "a bare radius in a test" plant_in_a_test
  gate_selftest_passes "a type alias that only holds a Lever" plant_alias_of_a_holder
  gate_selftest_case "referenced past its pin" plant_bare_radius
  gate_selftest_case "referenced past its pin" plant_turbofish_over_lines
  gate_selftest_case "referenced past its pin" plant_qualified_path
  gate_selftest_case "referenced past its pin" plant_scaled_normal
  gate_selftest_case "referenced past its pin" plant_measured_literal
  gate_selftest_case "referenced past its pin" plant_curvature_in_the_extent_home
  gate_selftest_case "referenced past its pin" plant_fn_pointer
  gate_selftest_case --also "a pinned site is gone" "referenced past its pin" plant_swapped_door
  gate_selftest_case "renamed or aliased" plant_alias
  gate_selftest_case "renamed or aliased" plant_renamed_free_fn
  gate_selftest_case "outside its home" plant_impl_outside_home
  gate_selftest_case "a pinned site is gone" plant_stale_pin
  gate_selftest_case "it cannot read" plant_malformed_pin
  GATE_BASE=HEAD gate_selftest_case "above the merge base" plant_raised
  GATE_BASE=HEAD gate_selftest_passes "a file git moved, its pin under the new path" plant_moved
  GATE_BASE=HEAD gate_selftest_passes "a split file that cites its origin" plant_split_cited
  GATE_BASE=HEAD gate_selftest_case "above the merge base" plant_new_site_citing
  GATE_BASE=HEAD gate_selftest_case "above the merge base" plant_raised_citing
  printf '%s selftest OK: passes a clean fixture, a pinned site, a door in a test, a type alias that only holds a Lever, a file git moved and a split file citing its origin; fires on a bare radius, a turbofish split over lines, a qualified path, a caller'"'"'s plane normal, a Measured literal, a curvature arm in the extent home, a fn pointer, one door swapped for another in a file, an alias of a type and of a free fn, an impl outside the home, a stale pin, a malformed one, a raised pin, a new site citing an unchanged file and a raised pin citing another; diagnoses a counter that cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
