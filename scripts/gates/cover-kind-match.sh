#!/usr/bin/env bash
# cover-kind-match.sh — the region an entity occupies is read from its
# cover. ONE home; ci.yml's `lint` job runs every gate in this directory.
#
# THE RULE is `docs/DESIGN.md` `[one-way.places]`, "The region a
# carrier, an edge or a face can occupy": one cover per entity, and
# every box, support, reach and ball is a method on it, never a second
# reading; `[one-way.readings]` puts the kind matches in the readings
# (`geom`'s `Surface::cover`/`Curve3::cover_over`, `topo`'s
# `edge_cover`/`face_cover`) and says a tighter construction is written
# into the arm, never at a reader.
#
# TWO LEDGERS, and they move opposite ways.
#
#  - The sites that read a region OUTSIDE the cover modules are PINNED
#    per (kind, file) in `cover-kind-match.counts`, and a pin only falls:
#    a new site reds, and so does a pinned one that is gone, so the change
#    that clears a site lowers its line in the same diff. A pin above the
#    merge base's reds too (`GATE_BASE`). The file is the move list.
#      `arm`   a kind of `Surface`/`Curve3` matched in a pattern whose arm
#              names a region (a box, a ball, a reach, an extent, a
#              support, a hull, a cover, a distance, a carrier's speed
#              bounds or control net), or inside a fn named for one, its
#              pivot or its floor; the kind however the file spells it
#              (an alias, an imported variant, `Self::` in the enum's own
#              impl);
#      `ball`  a literal torus or sphere ball (`R + r`, `|c − p| + r`),
#              read off each sum's flattened terms, whatever else it
#              holds;
#      `whole` a whole carrier's cover (`.cover()`), where a consumed span
#              or face reads its own.
#  - The READERS of `topo`'s cover doors are what the rule asks for, so
#    they are not pinned but REGISTERED in `cover-kind-match.readers`:
#    one line per file, enclosing fn and door, with its count and the
#    DIRECTION it reads looseness in, argued. A new reader lands by adding
#    its line, and so does a swap of one door for another in one fn; a
#    reader that goes drops its line; a moved count is re-read. A fn name
#    a file declares twice is qualified by its impl's self type
#    (`Type::name`), and by its order where that is shared too. The
#    doors are every public free fn of `topo`'s `cover.rs` and `boxes.rs`
#    (less the counter's NOT_DOORS), so a new door is read from the day
#    it lands. The registry is the direction column `boxes.rs`' module
#    docs argue door by door; it recites the direction rather than
#    computing it (`S234`).
#
# A MOVED FILE KEEPS ITS PINS: git's rename detection between the base
# and the tree carries a base pin to the new path. A SPLIT FILE cites its
# origin: each of its pins' lines ends `# from OLD_PATH`, and their sum
# (with OLD_PATH's own line, if it stays) is held to OLD_PATH's base pin.
#
# WHAT IT COUNTS: every production file under crates/*/src (comments,
# literals and test code skipped, `gate_rust_code`); the pinned kinds
# outside the cover modules (`geom`'s `cover.rs` and `cover/`, `topo`'s
# `cover.rs`). The counter, the name resolution, the vocabulary and what
# it cannot see are `cover_kind_match.py`'s header. A count is a receipt,
# not a verdict: a site the vocabulary over-reads is pinned like the
# rest, and the structural exclusions are named there.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"
CENSUS="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/cover_kind_match.py"

BASELINE=scripts/gates/cover-kind-match.counts
READERS=scripts/gates/cover-kind-match.readers
RULE='`[one-way.places]` (docs/DESIGN.md): the region an entity can occupy is its cover. Read the bound as a method on it (`support`, `box_in`, `range_along`, `distance_range`, `ball`, `aabb`) of `topo::face_cover`, `topo::edge_cover` or `geom`'"'"'s `Curve3::cover_over` over the span consumed, and register the read in '"$READERS"' with the direction it reads looseness in; a tighter construction goes into the cover'"'"'s arm (`geom::cover`, `crates/topo/src/cover.rs`), never at the reader. Move the code; do not raise the count'

gate() {
  gate_require_file "$BASELINE"
  gate_require_file "$READERS"
  gate_require_crate_sources
  gate_production_sources
  local report base_note="no base given (GATE_BASE unset), so no pin was compared with one"
  local base_args=() renames
  renames=$(mktemp)
  if [ -n "${GATE_BASE:-}" ] && git cat-file -e "$GATE_BASE^{tree}" 2>/dev/null \
    && ! git cat-file -e "$GATE_BASE:./$BASELINE" 2>/dev/null; then
    # The pins are new in this change: there is no base to hold yet.
    base_note="$BASELINE is new since the base $GATE_BASE, so its pins are this change's own"
  elif gate_base_file "$BASELINE"; then
    if ! git diff -M --name-status "$GATE_BASE" -- crates > "$renames.raw" 2>/dev/null; then
      rm -f "$renames" "$renames.raw"
      gate_error "$(gate_name): git could not list the renames since the base $GATE_BASE, so a moved file's pins could not be followed — that is not a pass"
      exit 1
    fi
    awk '$1 ~ /^R/ { print $2, $3 }' "$renames.raw" > "$renames"
    rm -f "$renames.raw"
    base_args=(--base "$GATE_BASE_FILE" --renames "$renames")
    base_note="none above the base $GATE_BASE's"
  fi
  if ! report=$(gate_rust_code --skip-cfg-test "${GATE_PRODUCTION_FILES[@]}" \
    | python3 "$CENSUS" "$BASELINE" --readers "$READERS" ${LINES_FOR:+--lines "$LINES_FOR"} ${base_args[@]+"${base_args[@]}"}); then
    rm -f "$renames"
    gate_error "$(gate_name): the counter failed under $PWD (its own message is above, if it had one) — the gate could not read its subject, which is not a pass"
    exit 1
  fi
  rm -f "$renames"
  if ! grep -q '^TOTAL [0-9]' <<<"$report"; then
    gate_error "$(gate_name): the counter exited without its TOTAL line — it did not finish its census, which is not a pass"
    exit 1
  fi
  grep '^LINES ' <<<"$report" || true
  local failed=0
  if grep -qE '^(MALFORMED|DUPLICATE) ' <<<"$report"; then
    grep -E '^(MALFORMED|DUPLICATE) ' <<<"$report"
    gate_error "$BASELINE has a line it cannot read or a second line for one (kind, file) (above): each is \`KIND COUNT FILE\`, KIND one of arm, ball, whole, once"
    failed=1
  fi
  if grep -qE '^ROSE (arm|ball|whole) ' <<<"$report"; then
    grep -E '^ROSE (arm|ball|whole) ' <<<"$report"
    gate_error "a region is read outside its cover past its pin (above; LINES_FOR=FILE lists a file's counted sites) — $RULE"
    failed=1
  fi
  if grep -q '^RAISED ' <<<"$report"; then
    grep '^RAISED ' <<<"$report"
    gate_error "a pin is above the merge base's (above), and a pin only falls. A file moved by git keeps its pins; a file split from another cites it (\`# from OLD_PATH\` on each line). Otherwise — $RULE"
    failed=1
  fi
  if grep -q '^FELL ' <<<"$report"; then
    grep '^FELL ' <<<"$report"
    gate_error "a pinned site is gone (above): drop or lower its line in $BASELINE"
    failed=1
  fi
  if grep -q '^BADREASON ' <<<"$report"; then
    grep '^BADREASON ' <<<"$report"
    gate_error "$READERS has a line it cannot read (above): each is FILE, FN, DOOR, COUNT, DIRECTION (prunes, refuses, examines, two-sided or namesake) and the argument for it, tab-separated, once per file, fn and door"
    failed=1
  fi
  if grep -qE '^(UNARGUED|MOVED) ' <<<"$report"; then
    grep -E '^(UNARGUED|MOVED) ' <<<"$report"
    gate_error "a reader of topo's cover doors has no line in $READERS, or its count moved (above). Reading the cover is the rule's answer: add or update its line with the direction it reads looseness in (a looser cover prunes, refuses, examines, or is two-sided) and why (crates/topo/src/boxes.rs' module docs argue each door)"
    failed=1
  fi
  if grep -q '^VANISHED ' <<<"$report"; then
    grep '^VANISHED ' <<<"$report"
    gate_error "a registered reader of topo's cover doors is gone (above): drop its line in $READERS"
    failed=1
  fi
  [ "$failed" = 0 ] || exit 1
  gate_ok "the regions read outside their covers at their pins, and every reader of topo's cover doors registered with its direction ($(sed -n 's/^TOTAL //p' <<<"$report")); pins: $base_note"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/scripts/gates" "$1/crates/geom/src/cover" "$1/crates/topo/src"
  printf '# kind count file\n' > "$1/$BASELINE"
  printf '# file fn door count direction argument\n' > "$1/$READERS"
  # The kinds, read off their declarations.
  printf 'pub enum Surface<T> {\n    Plane { origin: P<T> },\n    Sphere { radius: T },\n    Torus { major_radius: T, minor_radius: T },\n}\n' \
    > "$1/crates/geom/src/surfaces.rs"
  printf 'pub enum Curve3<T> {\n    Line(Line<T>),\n    Circle(Circle<T>),\n    Ellipse(Ellipse<T>),\n}\n' \
    > "$1/crates/geom/src/curves.rs"
  # The homes match on kind, build balls and read the doors: none counts.
  printf 'pub fn cover(s: &Surface) -> f64 {\n    match s {\n        Surface::Torus { major_radius, minor_radius, .. } => major_radius + minor_radius,\n        _ => 0.0,\n    }\n}\n' \
    > "$1/crates/geom/src/cover.rs"
  cp "$1/crates/geom/src/cover.rs" "$1/crates/geom/src/cover/readings.rs"
  printf 'pub fn face_cover() {}\npub fn edge_box() { face_cover(); }\npub fn sweep_pad() {}\nimpl B { pub fn kind(&self) {} }\n' \
    > "$1/crates/topo/src/cover.rs"
  # Near misses: a kind arm that builds no region, `std`'s `Box` in one,
  # a chart metric `R + r·cos v`, a difference of squares and the
  # difference bound it is the simplest case of, the radii's
  # perturbations, a kind constructed rather
  # than matched, a cover read over a span, a refusal by kind, a `use` of
  # a door, a door's namesake declared outside the homes, a method named
  # like a home's method, and a pad.
  {
    printf 'pub fn eval(c: &Curve3, t: f64) -> f64 {\n    match c {\n        Curve3::Circle(k) => k.radius * t.cos(),\n        _ => 0.0,\n    }\n}\n'
    printf 'pub fn boxed(c: &Curve3) -> Box<f64> { match c { Curve3::Line(_) => Box::new(1.0), _ => Box::new(0.0) } }\n'
    printf 'pub fn metric(major_radius: f64, minor_radius: f64, v: f64) -> f64 { major_radius + minor_radius * v.cos() }\n'
    printf 'pub fn squares(major_radius: f64, minor_radius: f64) -> f64 { (major_radius - minor_radius) * (major_radius + minor_radius) }\n'
    printf 'pub fn bound(major: f64, minor: f64, c_major: f64, c_minor: f64) -> f64 { ((major - c_major).abs() + (minor - c_minor).abs()) * (major + c_major + minor + c_minor) }\n'
    printf 'pub fn drift(d_major: f64, d_minor: f64) -> f64 { d_major.abs() + d_minor.abs() }\n'
    printf 'pub fn make(l: Line) -> Curve3 { Curve3::Line(l) }\n'
    printf 'pub fn span(c: &Curve3, w: W) -> f64 { c.cover_over(w).unwrap().ball().1 }\n'
    printf 'pub fn unhandled(s: &Surface) { if let Surface::Plane { .. } = s { return Err(Unsupported); } }\n'
    printf 'use topo::cover::face_cover;\n'
    printf 'pub fn edge_box() -> u8 { 0 }\n'
    printf 'pub fn k(b: &B) { b.kind(); sweep_pad(); }\n'
  } >> "$1/crates/clean/src/lib.rs"
}

append() { printf '%s\n' "$2" >> "$1/crates/clean/src/lib.rs"; }
pin() { printf '%s\n' "$2" >> "$1/$BASELINE"; }
register() { printf '%s\n' "$2" >> "$1/$READERS"; }

plant_arm_box() { append "$1" 'pub fn b(c: &Curve3) -> Aabb { match c { Curve3::Circle(k) => Aabb::of(k.radius), _ => Aabb::poison() } }'; }
plant_arm_inline_speed() { append "$1" 'pub fn f(c: &Curve3, q: P) -> f64 { match *c { Curve3::Ellipse(e) => q.d() - e.speed_lo(), _ => 0.0 } }'; }
plant_arm_nested() { append "$1" 'impl R { pub fn foot_on(&self, a: L) -> P { match self { Self::Span { carrier: Curve3::Circle(k), .. } => a.foot(k.c), _ => a.o } } }'; }
plant_arm_let_else() { append "$1" 'pub fn g(c: &Curve3) -> f64 { let Curve3::Circle(k) = c else { return 0.0; }; ExtentBall::new(k.c, k.radius).lever() }'; }
plant_arm_if_let() { append "$1" 'pub fn h(s: &Surface) -> f64 { if let Surface::Sphere { radius, .. } = s { return reach(*radius); } 0.0 }'; }
plant_arm_matches() { append "$1" 'pub fn m(c: &Curve3) -> f64 { if matches!(c, Curve3::Line(_)) { chord_extent(c) } else { 0.0 } }'; }
plant_arm_alias() { append "$1" 'use geom::Surface as S;'; append "$1" 'pub fn a(s: &S) -> f64 { match s { S::Sphere { radius } => reach(*radius), _ => 0.0 } }'; }
plant_arm_imported() { append "$1" 'pub fn i(s: &Surface) -> f64 { use Surface::{Plane, Sphere}; match s { Sphere { radius } => reach(*radius), Plane { .. } => 0.0, _ => 0.0 } }'; }
plant_arm_self() { append "$1" 'impl<T: Real> Curve3<T> { pub fn ext(&self) -> f64 { match self { Self::Circle(k) => extent(k), _ => 0.0 } } }'; }
plant_arm_distance() { append "$1" 'pub fn n(c: &Curve3, q: P) -> f64 { match c { Curve3::Line(l) => l.distance(q), _ => 0.0 } }'; }
plant_ball_torus() { append "$1" 'pub fn t(major_radius: f64, minor_radius: f64) -> f64 { major_radius.abs() + minor_radius }'; }
plant_ball_receiver() { append "$1" 'pub fn t(t: &T) -> f64 { t.major_radius() + t.minor_radius }'; }
plant_ball_wrapped() { append "$1" 'pub fn t(c: P, p: P, major_radius: f64, minor_radius: f64) -> f64 {'; append "$1" '    (c - p).norm()'; append "$1" '        + major_radius'; append "$1" '        + minor_radius'; append "$1" '}'; }
plant_ball_sphere() { append "$1" 'pub fn s(c: P, p: P, radius: f64) -> f64 { (c - p).norm() + radius }'; }
plant_ball_offset() { append "$1" 'pub fn t(offset: f64, major_radius: f64, minor_radius: f64) -> f64 { offset + major_radius + minor_radius }'; }
plant_ball_offset_abs() { append "$1" 'pub fn t(offset: f64, major_radius: f64, minor_radius: f64) -> f64 { offset.abs() + major_radius + minor_radius }'; }
plant_ball_padded_sphere() { append "$1" 'pub fn s(pad: f64, c: P, q: P, radius: f64) -> f64 { pad + (c - q).norm() + radius }'; }
plant_ball_grouped_torus() { append "$1" 'pub fn s(q: P, t: &T) -> f64 { (q - t.center).norm() + (t.major_radius + t.minor_radius) }'; }
plant_ball_torus_first() { append "$1" 'pub fn s(q: P, c: P, major_radius: f64, minor_radius: f64) -> f64 { (major_radius + minor_radius) + (q - c).norm() }'; }
plant_ball_two_tori() { append "$1" 'pub fn rate(major: f64, minor: f64, r_major: f64, r_minor: f64) -> f64 { (major + minor) + (r_major + r_minor) }'; }
plant_ball_sphere_reversed() { append "$1" 'pub fn s(s: &S, c: P, q: &Q) -> f64 { s.radius + c.distance(q.center()) }'; }
plant_whole() { append "$1" 'pub fn w(c: &Curve3) -> f64 { c.cover().unwrap().ball().1 }'; }
plant_reader_point_free() { append "$1" 'pub fn r(b: &B, fs: &[F]) -> Vec<A> { fs.iter().copied().map(face_cover).collect() }'; }
plant_reader_registered() {
  plant_reader_point_free "$1"
  register "$1" "$(printf 'crates/clean/src/lib.rs\tr\tface_cover\t1\tprunes\ta fixture reader whose looser cover only adds work here')"
}
plant_reader_vanished() { register "$1" "$(printf 'crates/clean/src/lib.rs\tgone\tface_cover\t1\tprunes\ta fixture reader that is no longer in the tree')"; }
plant_reader_bad_direction() {
  plant_reader_point_free "$1"
  register "$1" "$(printf 'crates/clean/src/lib.rs\tr\tface_cover\t1\tloose\ta fixture reader with no direction the gate knows')"
}
# The fn r registered for face_cover now reads edge_box: a swap keeps the
# count and is still a new reader.
plant_reader_swapped() {
  append "$1" 'pub fn r(b: &B, fs: &[F]) -> Vec<A> { fs.iter().copied().map(edge_box).collect() }'
  register "$1" "$(printf 'crates/clean/src/lib.rs\tr\tface_cover\t1\tprunes\ta fixture reader whose looser cover only adds work here')"
}
# Two impls' fns named alike are two readers, each under its type.
twins() {
  append "$1" 'impl A { fn r(&self) { face_cover(); } }'
  append "$1" 'impl<T: Real> Tr for B<T> { fn r(&self) { face_cover(); } }'
}
plant_reader_twins() {
  twins "$1"
  register "$1" "$(printf 'crates/clean/src/lib.rs\tA::r\tface_cover\t1\tprunes\ta fixture reader whose looser cover only adds work here')"
  register "$1" "$(printf 'crates/clean/src/lib.rs\tB::r\tface_cover\t1\tprunes\ta fixture reader whose looser cover only adds work here')"
}
plant_reader_twins_merged() {
  twins "$1"
  register "$1" "$(printf 'crates/clean/src/lib.rs\tr\tface_cover\t2\tprunes\ta fixture reader whose looser cover only adds work here')"
}
plant_pinned() { plant_arm_box "$1"; plant_whole "$1"; pin "$1" 'arm 1 crates/clean/src/lib.rs'; pin "$1" 'whole 1 crates/clean/src/lib.rs'; }
plant_pin_gone() { pin "$1" 'ball 1 crates/clean/src/lib.rs'; }
plant_malformed() { pin "$1" 'arms 1 crates/clean/src/lib.rs'; }
plant_duplicate() { plant_whole "$1"; pin "$1" 'whole 1 crates/clean/src/lib.rs'; pin "$1" 'whole 1 crates/clean/src/lib.rs'; }
plant_no_kinds() { rm "$1/crates/geom/src/curves.rs"; }

# The base is a commit of the fixture's own repository.
commit_base() {
  git -C "$1" init -q
  git -C "$1" add -A
  git -C "$1" -c user.name=gate -c user.email=gate@localhost commit -qm base
}
plant_raised() {
  plant_arm_box "$1"
  pin "$1" 'arm 0 crates/clean/src/lib.rs'
  commit_base "$1"
  sed -i 's/^arm 0 /arm 1 /' "$1/$BASELINE"
}
# A file moved by git keeps its pin under its new path.
plant_moved() {
  mkdir -p "$1/crates/moved/src"
  printf 'pub fn w(c: &Curve3) -> f64 {\n    c.cover().unwrap().ball().1\n}\n' > "$1/crates/moved/src/old.rs"
  pin "$1" 'whole 1 crates/moved/src/old.rs'
  commit_base "$1"
  git -C "$1" mv crates/moved/src/old.rs crates/moved/src/new.rs
  sed -i 's|crates/moved/src/old.rs|crates/moved/src/new.rs|' "$1/$BASELINE"
}
# A file split in two: each half cites its origin, and the halves are
# held to its pin; a half that does not cite it is a raise.
split_base() {
  mkdir -p "$1/crates/split/src"
  printf 'pub fn a(c: &Curve3) -> f64 { c.cover().unwrap().ball().1 }\npub fn b(c: &Curve3) -> f64 { c.cover().unwrap().ball().1 }\n' \
    > "$1/crates/split/src/old.rs"
  pin "$1" 'whole 2 crates/split/src/old.rs'
  commit_base "$1"
  rm "$1/crates/split/src/old.rs"
  printf 'pub fn a(c: &Curve3) -> f64 { c.cover().unwrap().ball().1 }\n' > "$1/crates/split/src/a.rs"
  printf 'pub fn b(c: &Curve3) -> f64 { c.cover().unwrap().ball().1 }\n' > "$1/crates/split/src/b.rs"
  sed -i '/crates\/split\/src\/old.rs/d' "$1/$BASELINE"
}
plant_split_cited() {
  split_base "$1"
  pin "$1" 'whole 1 crates/split/src/a.rs # from crates/split/src/old.rs'
  pin "$1" 'whole 1 crates/split/src/b.rs # from crates/split/src/old.rs'
}
plant_split_uncited() {
  split_base "$1"
  pin "$1" 'whole 1 crates/split/src/a.rs # from crates/split/src/old.rs'
  pin "$1" 'whole 1 crates/split/src/b.rs'
}

gate_selftest() {
  unset GATE_BASE
  gate_selftest_clean
  gate_selftest_without_tool python3 "the counter failed"
  local c
  for c in plant_arm_box plant_arm_inline_speed plant_arm_nested plant_arm_let_else \
    plant_arm_if_let plant_arm_matches plant_arm_alias plant_arm_imported plant_arm_self \
    plant_arm_distance plant_ball_torus plant_ball_receiver plant_ball_wrapped \
    plant_ball_sphere plant_ball_sphere_reversed plant_ball_offset plant_ball_offset_abs \
    plant_ball_padded_sphere plant_ball_grouped_torus plant_ball_torus_first plant_ball_two_tori \
    plant_whole; do
    gate_selftest_case "read outside its cover past its pin" "$c"
  done
  gate_selftest_case "has no line in" plant_reader_point_free
  gate_selftest_passes "a registered reader" plant_reader_registered
  gate_selftest_case "a registered reader of topo's cover doors is gone" plant_reader_vanished
  gate_selftest_case "has a line it cannot read" plant_reader_bad_direction
  gate_selftest_case "has no line in" plant_reader_swapped
  gate_selftest_passes "two impls' like-named readers, each under its type" plant_reader_twins
  gate_selftest_case "has no line in" plant_reader_twins_merged
  gate_selftest_case "a pinned site is gone" plant_pin_gone
  gate_selftest_case "a line it cannot read" plant_malformed
  gate_selftest_case "a second line for one" plant_duplicate
  gate_selftest_case "the counter failed" plant_no_kinds
  gate_selftest_passes "sites at their pins" plant_pinned
  GATE_BASE=HEAD gate_selftest_case "above the merge base" plant_raised
  GATE_BASE=HEAD gate_selftest_passes "a file git moved, its pin under the new path" plant_moved
  GATE_BASE=HEAD gate_selftest_passes "a split file whose halves cite it" plant_split_cited
  GATE_BASE=HEAD gate_selftest_case "above the merge base" plant_split_uncited
  gate_selftest_test_module_homes "read outside its cover past its pin" plant_home_breach
  printf '%s selftest OK: passes a clean fixture (the cover homes'"'"' own kind matches, balls and door reads; a kind arm that builds no region or builds a std Box; a chart metric, a difference of squares and the difference bound, the radii'"'"'s perturbations; a constructed kind; a span'"'"'s cover; a use of a door, a namesake declared outside the homes, a home'"'"'s method name and a pad), sites at their pins, a registered reader, like-named readers of two impls under their types, a moved file and a split file citing its origin; fires on a kind arm building a box, an inline speed floor, a nested pattern in a pivot fn, a let-else, an if-let, a matches!, an alias, an imported variant, Self:: in the enum'"'"'s impl and a distance, on a literal torus ball by name, by receiver, wrapped over lines, after an offset or its abs, grouped beside a distance or first, and two tori summed, and a sphere ball either way round and padded, on a whole carrier'"'"'s cover, an unregistered point-free reader, a vanished one, one with no known direction, a door swapped in a fn and like-named fns of two impls read as one, a pin gone, a malformed or duplicate pin, a pin above the base and an uncited split half; refuses a tree whose kinds it cannot read; reads only production code; diagnoses a counter that cannot run\n' "$(gate_name)"
}

plant_home_breach() { printf 'pub fn w(c: &Curve3) -> f64 { c.cover().unwrap().ball().1 }\n' >> "$1"; }

gate_parse_args "$@"
gate_main
