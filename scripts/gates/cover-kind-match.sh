#!/usr/bin/env bash
# cover-kind-match.sh — the region an entity occupies is read from its
# cover, pinned per file. ONE home; ci.yml's `lint` job runs every gate
# in this directory.
#
# THE RULE is `docs/DESIGN.md` `[one-way.places]`, "The region a
# carrier, an edge or a face can occupy": one cover per entity, and
# every box, support, reach and ball is a method on it, never a second
# reading; `[one-way.readings]` puts the kind matches in the readings
# (`geom`'s `Surface::cover`/`Curve3::cover_over`, `topo`'s
# `edge_cover`/`face_cover`) and says a tighter construction is written
# into the arm, never at a reader.
#
# So the sites that read a region outside the cover modules today are
# PINNED per (kind, file) in `cover-kind-match.counts`, and the pin only
# falls: a new site reds, and so does a pinned one that is gone, so the
# change that clears a site lowers its line in the same diff. A pin
# above the merge base's reds too (`GATE_BASE`). The file is the move
# list. The kinds:
#
#  - `arm`: a `Surface::`/`Curve3::` kind matched in a pattern whose
#    arm names a region (a box, a ball, a reach, an extent, a support, a
#    hull, a cover, a carrier's speed bounds or control net), or inside a
#    fn named for one, its pivot or its floor;
#  - `ball`: a literal torus or sphere ball (`R + r`, `|c − p| + r`);
#  - `whole`: a whole carrier's cover (`.cover()`), where a consumed span
#    or face reads its own;
#  - `reader`: a reference to one of `topo`'s cover doors outside its
#    homes (`boxes.rs`, `cover.rs`), by whole token, so a point-free
#    read (`.map(face_box)`) counts. Each says, in review, which way the
#    door reads looseness (`crates/topo/src/boxes.rs`' module docs).
#
# WHAT IT COUNTS: every production file under crates/*/src (comments,
# literals and test code skipped, `gate_rust_code`); `arm`, `ball` and
# `whole` outside the cover modules (`geom`'s `cover.rs` and `cover/`,
# `topo`'s `cover.rs`). The counter, the vocabulary and what it cannot
# see are `cover_kind_match.py`'s header. A count is a receipt, not a
# verdict: a site the vocabulary over-reads is pinned like the rest.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"
CENSUS="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/cover_kind_match.py"

BASELINE=scripts/gates/cover-kind-match.counts
RULE='`[one-way.places]` (docs/DESIGN.md): the region an entity can occupy is its cover. Read the bound as a method on it (`support`, `box_in`, `range_along`, `distance_range`, `ball`, `aabb`) of `topo::face_cover`, `topo::edge_cover` or `geom`'"'"'s `Curve3::cover_over` over the span consumed; a tighter construction goes into the cover'"'"'s arm (`geom::cover`, `crates/topo/src/cover.rs`), never at the reader. Move the code; do not raise the count'

gate() {
  gate_require_file "$BASELINE"
  gate_require_crate_sources
  gate_production_sources
  local report base_note="no base given (GATE_BASE unset), so no pin was compared with one"
  local base_args=()
  if [ -n "${GATE_BASE:-}" ] && git cat-file -e "$GATE_BASE^{tree}" 2>/dev/null \
    && ! git cat-file -e "$GATE_BASE:./$BASELINE" 2>/dev/null; then
    # The pins are new in this change: there is no base to hold yet.
    base_note="$BASELINE is new since the base $GATE_BASE, so its pins are this change's own"
  elif gate_base_file "$BASELINE"; then
    base_args=(--base "$GATE_BASE_FILE")
    base_note="none above the base $GATE_BASE's"
  fi
  if ! report=$(gate_rust_code --skip-cfg-test "${GATE_PRODUCTION_FILES[@]}" \
    | python3 "$CENSUS" "$BASELINE" ${LINES_FOR:+--lines "$LINES_FOR"} ${base_args[@]+"${base_args[@]}"}); then
    gate_error "$(gate_name): the counter failed under $PWD (its own message is above, if it had one) — the gate could not read its subject, which is not a pass"
    exit 1
  fi
  if ! grep -q '^TOTAL [0-9]' <<<"$report"; then
    gate_error "$(gate_name): the counter exited without its TOTAL line — it did not finish its census, which is not a pass"
    exit 1
  fi
  grep '^LINES ' <<<"$report" || true
  local failed=0
  if grep -q '^MALFORMED ' <<<"$report"; then
    grep '^MALFORMED ' <<<"$report"
    gate_error "$BASELINE has a line it cannot read (above): each is \`KIND COUNT FILE\`, KIND one of arm, ball, whole, reader"
    failed=1
  fi
  if grep -qE '^ROSE (arm|ball|whole) ' <<<"$report"; then
    grep -E '^ROSE (arm|ball|whole) ' <<<"$report"
    gate_error "a region is read outside its cover past its pin (above; LINES_FOR=FILE lists a file's counted sites) — $RULE"
    failed=1
  fi
  if grep -q '^ROSE reader ' <<<"$report"; then
    grep '^ROSE reader ' <<<"$report"
    gate_error "a reader of topo's cover doors arrived past its pin (above). Say in the change which way it reads looseness, pruning or refusing (crates/topo/src/boxes.rs' module docs), and pin it"
    failed=1
  fi
  if grep -q '^RAISED ' <<<"$report"; then
    grep '^RAISED ' <<<"$report"
    gate_error "a pin is above the merge base's (above), and a pin only falls — $RULE"
    failed=1
  fi
  if grep -q '^FELL ' <<<"$report"; then
    grep '^FELL ' <<<"$report"
    gate_error "a pinned site is gone (above): drop or lower its line in $BASELINE"
    failed=1
  fi
  [ "$failed" = 0 ] || exit 1
  gate_ok "the regions read outside their covers, and the cover doors' readers, at their pinned counts ($(sed -n 's/^TOTAL //p' <<<"$report") sites in all); pins: $base_note"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/scripts/gates" "$1/crates/geom/src/cover" "$1/crates/topo/src"
  printf '# kind count file\n' > "$1/$BASELINE"
  # The homes match on kind, build balls and read the doors: none counts.
  printf 'pub fn cover(s: &Surface) -> f64 {\n    match s {\n        Surface::Torus { major_radius, minor_radius, .. } => major_radius + minor_radius,\n        _ => 0.0,\n    }\n}\n' \
    > "$1/crates/geom/src/cover.rs"
  cp "$1/crates/geom/src/cover.rs" "$1/crates/geom/src/cover/readings.rs"
  printf 'pub fn face_cover() {}\npub fn edge_box() { face_cover(); }\n' > "$1/crates/topo/src/cover.rs"
  # Near misses: a kind arm that builds no region, a chart metric
  # `R + r·cos v`, a kind constructed rather than matched, and a cover
  # read over a span.
  {
    printf 'pub fn eval(c: &Curve3, t: f64) -> f64 {\n    match c {\n        Curve3::Circle(k) => k.radius * t.cos(),\n        _ => 0.0,\n    }\n}\n'
    printf 'pub fn metric(major_radius: f64, minor_radius: f64, v: f64) -> f64 { major_radius + minor_radius * v.cos() }\n'
    printf 'pub fn make(l: Line) -> Curve3 { Curve3::Line(l) }\n'
    printf 'pub fn span(c: &Curve3, w: W) -> f64 { c.cover_over(w).unwrap().ball().1 }\n'
    printf 'pub fn unhandled(s: &Surface) { if let Surface::Plane { .. } = s { return Err(Unsupported); } }\n'
  } >> "$1/crates/clean/src/lib.rs"
}

append() { printf '%s\n' "$2" >> "$1/crates/clean/src/lib.rs"; }
pin() { printf '%s\n' "$2" >> "$1/$BASELINE"; }

plant_arm_box() { append "$1" 'pub fn b(c: &Curve3) -> Aabb { match c { Curve3::Circle(k) => Aabb::of(k.radius), _ => Aabb::poison() } }'; }
plant_arm_inline_speed() { append "$1" 'pub fn f(c: &Curve3, q: P) -> f64 { match *c { Curve3::Ellipse(e) => q.d() - e.speed_lo(), _ => 0.0 } }'; }
plant_arm_nested() { append "$1" 'impl R { pub fn foot_on(&self, a: L) -> P { match self { Self::Span { carrier: Curve3::Circle(k), .. } => a.foot(k.c), _ => a.o } } }'; }
plant_arm_let_else() { append "$1" 'pub fn g(c: &Curve3) -> f64 { let Curve3::Circle(k) = c else { return 0.0; }; ExtentBall::new(k.c, k.radius).lever() }'; }
plant_arm_if_let() { append "$1" 'pub fn h(s: &Surface) -> f64 { if let Surface::Sphere { radius, .. } = s { return reach(*radius); } 0.0 }'; }
plant_arm_matches() { append "$1" 'pub fn m(c: &Curve3) -> f64 { if matches!(c, Curve3::Line(_)) { chord_extent(c) } else { 0.0 } }'; }
plant_ball_torus() { append "$1" 'pub fn t(major_radius: f64, minor_radius: f64) -> f64 { major_radius.abs() + minor_radius }'; }
plant_ball_sphere() { append "$1" 'pub fn s(c: P, p: P, radius: f64) -> f64 { (c - p).norm() + radius }'; }
plant_whole() { append "$1" 'pub fn w(c: &Curve3) -> f64 { c.cover().unwrap().ball().1 }'; }
plant_reader_point_free() { append "$1" 'pub fn r(b: &B, fs: &[F]) -> Vec<A> { fs.iter().copied().map(face_box).collect() }'; }
plant_pinned() { plant_arm_box "$1"; plant_whole "$1"; pin "$1" 'arm 1 crates/clean/src/lib.rs'; pin "$1" 'whole 1 crates/clean/src/lib.rs'; }
plant_pin_gone() { pin "$1" 'ball 1 crates/clean/src/lib.rs'; }
plant_malformed() { pin "$1" 'arms 1 crates/clean/src/lib.rs'; }

# A pin above the base's: the base is a commit of the fixture's own repository.
plant_raised() {
  plant_arm_box "$1"
  pin "$1" 'arm 1 crates/clean/src/lib.rs'
  sed -i 's/^arm 1 /arm 0 /' "$1/$BASELINE"
  git -C "$1" init -q
  git -C "$1" add -A
  git -C "$1" -c user.name=gate -c user.email=gate@localhost commit -qm base
  sed -i 's/^arm 0 /arm 1 /' "$1/$BASELINE"
}

gate_selftest() {
  unset GATE_BASE
  gate_selftest_clean
  gate_selftest_without_tool python3 "the counter failed"
  gate_selftest_case "read outside its cover past its pin" plant_arm_box
  gate_selftest_case "read outside its cover past its pin" plant_arm_inline_speed
  gate_selftest_case "read outside its cover past its pin" plant_arm_nested
  gate_selftest_case "read outside its cover past its pin" plant_arm_let_else
  gate_selftest_case "read outside its cover past its pin" plant_arm_if_let
  gate_selftest_case "read outside its cover past its pin" plant_arm_matches
  gate_selftest_case "read outside its cover past its pin" plant_ball_torus
  gate_selftest_case "read outside its cover past its pin" plant_ball_sphere
  gate_selftest_case "read outside its cover past its pin" plant_whole
  gate_selftest_case "a reader of topo's cover doors" plant_reader_point_free
  gate_selftest_case "a pinned site is gone" plant_pin_gone
  gate_selftest_case "a line it cannot read" plant_malformed
  gate_selftest_passes "sites at their pins" plant_pinned
  GATE_BASE=HEAD gate_selftest_case "above the merge base" plant_raised
  gate_selftest_test_module_homes "read outside its cover past its pin" plant_home_breach
  printf '%s selftest OK: passes a clean fixture (the cover homes'"'"' own kind matches, balls and door reads; a kind arm that builds no region, a chart metric, a constructed kind, a span'"'"'s cover) and sites at their pins; fires on a kind arm building a box, an inline speed floor, a nested pattern in a pivot fn, a let-else, an if-let and a matches! governing a region, a literal torus and sphere ball, a whole carrier'"'"'s cover, a point-free door read, a pin gone, a malformed pin and a pin above the base; reads only production code; diagnoses a counter that cannot run\n' "$(gate_name)"
}

plant_home_breach() { printf 'pub fn w(c: &Curve3) -> f64 { c.cover().unwrap().ball().1 }\n' >> "$1"; }

gate_parse_args "$@"
gate_main
