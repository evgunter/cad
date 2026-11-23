#!/usr/bin/env bash
# conic-centre-reads.sh — a ratchet on production reads of a conic's
# centre frame outside the conic's own module. ONE home; ci.yml's `lint`
# job runs every gate in this directory.
#
# THE RULE it serves (`crates/geom-core/README.md`, "Float rounding"):
# "Every reading of where a conic is goes through the conic." An ellipse
# held on its arc derives its centre on each read, at its semi-axes'
# rounding, so a reader that takes the centre as a datum states another
# curve than the one the evaluator draws.
#
# WHO DECIDES WHAT A READ IS: clippy, by type. `clippy.toml`'s
# `disallowed-methods` names the doors that hand out the centre
# (`Conic::frame`, `Conic::center`, `Circle::frame`, `Ellipse::frame`,
# `Curve3::frame`), the one that reads a point about it
# (`Conic::unit_in_plane`), and the two that take a conic restated on a
# caller's frame (`Curve3::circle_at`, `Cover::arc`); the workspace
# allows the lint and every library and binary root denies it outside
# `cfg(test)`, so a test may read them and production code may not. A production site that still reads one carries
# `#[expect(clippy::disallowed_methods, reason = "...")]` on its
# function, the reason citing the row that owes its conversion; clippy
# reds an expectation whose read has gone, so the list only holds live
# sites.
#
# WHAT THIS GATE HOLDS, which clippy does not:
#  - the arming: the doors in `clippy.toml`, the workspace `allow`, and
#    the `deny` line in every `crates/*/src/lib.rs`, `src/main.rs` and
#    `src/bin/*.rs`;
#  - the shape of a listed site: `#[expect(` on one line, the lint on
#    the next, then a `reason` that cites a `work/` row; no `allow` of
#    the lint under any crate's src but the conic's module's own;
#  - no mention under any crate's src of a lint GROUP that holds the
#    lint (`clippy::style`, `clippy::all`, `warnings` in an `allow` or
#    an `expect`): a group allowed on an item overrides the root's deny
#    and names the lint nowhere;
#  - THE PINS (`conic-centre-reads.counts`): the count of listed
#    functions per file, equal to the pin. A new listed function is a
#    raised pin in the diff that adds it, held by review as
#    `bare-rounding-sites.tsv`'s counts are.
#
# WHAT IT DOES NOT HOLD:
#  - a second read added inside a function already listed (the
#    expectation is the function's);
#  - a centre restated without these doors: `Curve3::data()` walks the
#    stored point by name, and a reader may subtract the semi-axis from
#    a point by hand;
#  - a read through a `Frame` another function was handed: the list
#    holds the function that took it from the conic;
#  - the closures of `Circle::placed`, `Ellipse::mapped` and the two
#    `map_scalar`s, which are handed the STORED frame (for an ellipse
#    the centre or the anchor, with nothing saying which): a closure
#    that reads its `origin()` reads a placement no door names;
#  - a conic rebuilt from a struct another function filled with a
#    centre (`crates/topo/src/boolean/mod.rs`'s `Rim` round trip, two
#    `Curve3::circle(rim.center, …)` sites): no door fires there;
#  - a centre summed from two readings (`point_at(θ) +
#    second_derivative_at(θ)`);
#  - `examples/` and `benches/` targets of a workspace crate, which are
#    development targets as tests are and carry no deny line;
#  - demos/, benches/ and tools/, which sit outside the workspace's
#    lint table and allow the lint in their own manifests.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

PINS=scripts/gates/conic-centre-reads.counts
HOME_FILE=crates/geom/src/curves/conic.rs
ARMING='#![cfg_attr(not(test), deny(clippy::disallowed_methods))]'
DOORS=(geom::Conic::frame geom::Conic::center geom::Circle::frame geom::Ellipse::frame geom::Curve3::frame geom::Conic::unit_in_plane geom::Curve3::circle_at geom::cover::Cover::arc)

# Per file: the count of well-formed listed sites, and a line per
# malformed mention of the lint (`BAD file:line`).
CENSUS='
FNR == 1 { prev = ""; want_reason = 0 }
{
  line = $0
  sub(/^[ \t]+/, "", line)
  if (want_reason) {
    if (line !~ /^reason = ".*work\/[a-z0-9-]+\/[A-Za-z0-9_.-]+\.md/) print "BAD " FILENAME ":" FNR
    want_reason = 0
  }
  if (line !~ /^\/\// && (line ~ /clippy::(style|all)([^A-Za-z0-9_]|$)/ || line ~ /(allow|expect)\(.*[^A-Za-z0-9_]?warnings([^A-Za-z0-9_]|$)/ || line ~ /^warnings,?$/)) print "GROUP " FILENAME ":" FNR
  if (line !~ /^\/\// && index(line, "clippy::disallowed_methods") > 0 && line != arming) {
    if (line == "clippy::disallowed_methods," && prev == "#[expect(") { n[FILENAME]++; want_reason = 1 }
    else print "BAD " FILENAME ":" FNR
  }
  prev = line
}
END { for (f in n) print f, n[f] }
'

gate() {
  gate_require_file "$HOME_FILE"
  gate_require_file "$PINS"
  gate_require_file clippy.toml
  gate_require_file Cargo.toml
  local bad=0 door root
  for door in "${DOORS[@]}"; do
    if ! grep -qF "path = \"$door\"" clippy.toml; then
      printf 'clippy.toml: disallowed-methods does not name %s\n' "$door"
      bad=1
    fi
  done
  if ! grep -qxF 'disallowed_methods = "allow"' Cargo.toml; then
    printf 'Cargo.toml: [workspace.lints.clippy] does not allow disallowed_methods (the library roots arm it)\n'
    bad=1
  fi
  local roots=()
  mapfile -t roots < <(find crates \( -path '*/src/lib.rs' -o -path '*/src/main.rs' -o -path '*/src/bin/*.rs' \) -type f -print 2>/dev/null | sort)
  if [ "${#roots[@]}" -eq 0 ]; then
    gate_error "$(gate_name): no crates/*/src/lib.rs in $PWD, so the scan read nothing, which is not a pass"
    exit 1
  fi
  for root in "${roots[@]}"; do
    if ! grep -qxF "$ARMING" "$root"; then
      printf '%s: does not carry %s\n' "$root" "$ARMING"
      bad=1
    fi
  done
  local files=() counts
  mapfile -t files < <(find crates -path '*/src/*' -name '*.rs' -type f ! -path "$HOME_FILE" -print 2>/dev/null | sort)
  GATE_SCAN_FILES=${#files[@]}
  if ! counts=$(awk -v arming="$ARMING" "$CENSUS" "${files[@]}"); then
    gate_error "$(gate_name): awk exited, so what it did not count is unknown, which is not a pass"
    exit 1
  fi
  local file got want
  declare -A pinned=()
  while read -r file want; do
    case "$file" in ''|'#'*) continue ;; esac
    pinned[$file]=$want
  done < "$PINS"
  while read -r file got; do
    [ -n "$file" ] || continue
    if [ "$file" = GROUP ]; then
      printf '%s: names a lint group that holds clippy::disallowed_methods (clippy::style, clippy::all, warnings): allow the lint you mean by its own name\n' "$got"
      bad=1
      continue
    fi
    if [ "$file" = BAD ]; then
      printf '%s: names clippy::disallowed_methods outside a listed site (#[expect( / the lint / a reason citing a work/ row)\n' "$got"
      bad=1
      continue
    fi
    want=${pinned[$file]:-0}
    if [ "$got" != "$want" ]; then
      printf '%s: lists %s function(s) that read a conic'"'"'s centre frame, pinned at %s\n' "$file" "$got" "$want"
      bad=1
    fi
  done <<< "$counts"
  for file in "${!pinned[@]}"; do
    if ! grep -q "^$file " <<< "$counts"; then
      printf '%s: pinned at %s, lists none\n' "$file" "${pinned[$file]}"
      bad=1
    fi
  done
  if [ "$bad" -ne 0 ]; then
    gate_error "a conic's centre frame is read where $PINS does not pin it, or the lint that finds the reads is not armed (above): every reading of where a conic is goes through the conic (crates/geom-core/README.md, Float rounding); read its point, extent, cover, azimuth or directions, and lower the pin of a site that has gone"
    exit 1
  fi
  gate_ok "a conic's centre frame is read in production only at the pinned, listed sites, and the lint that finds them is armed in every library and binary root"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/${HOME_FILE%/*}" "$1/scripts/gates"
  printf 'pub fn frame() {}\n' > "$1/$HOME_FILE"
  {
    printf 'disallowed-methods = [\n'
    local door
    for door in "${DOORS[@]}"; do printf '    { path = "%s", reason = "r" },\n' "$door"; done
    printf ']\n'
  } > "$1/clippy.toml"
  printf '[workspace.lints.clippy]\ndisallowed_methods = "allow"\n' > "$1/Cargo.toml"
  local root
  while read -r root; do
    printf '%s\n' "$ARMING" >> "$root"
  done < <(find "$1/crates" -path '*/src/lib.rs' -type f)
  printf '%s\n' "$ARMING" > "$1/crates/geom/src/lib.rs"
  {
    printf '#[expect(\n    clippy::disallowed_methods,\n'
    printf '    reason = "a centre (work/oneround/a-row.md)"\n)]\n'
    printf 'pub fn reader() {}\n'
  } > "$1/crates/geom/src/reader.rs"
  printf 'crates/geom/src/reader.rs 1\n' > "$1/$PINS"
}

plant_new_site() {
  {
    printf '#[expect(\n    clippy::disallowed_methods,\n'
    printf '    reason = "another (work/oneround/a-row.md)"\n)]\n'
    printf 'pub fn second() {}\n'
  } >> "$1/crates/geom/src/reader.rs"
}

plant_allow() {
  printf '#[allow(clippy::disallowed_methods)]\npub fn quiet() {}\n' >> "$1/crates/geom/src/reader.rs"
}

plant_no_row() {
  {
    printf '#[expect(\n    clippy::disallowed_methods,\n'
    printf '    reason = "because"\n)]\n'
    printf 'pub fn reader() {}\n'
  } > "$1/crates/geom/src/reader.rs"
}

plant_site_gone() {
  printf 'pub fn reader() {}\n' > "$1/crates/geom/src/reader.rs"
}

plant_unarmed_root() {
  printf 'pub fn f() {}\n' > "$1/crates/geom/src/lib.rs"
}

plant_door_dropped() {
  grep -v 'geom::Ellipse::frame' "$1/clippy.toml" > "$1/clippy.toml.new"
  mv "$1/clippy.toml.new" "$1/clippy.toml"
}

plant_group_allow() {
  printf '#[allow(clippy::style)]\npub fn quiet() {}\n' >> "$1/crates/geom/src/reader.rs"
}

plant_group_expect() {
  printf '#[expect(clippy::all, reason = "r")]\npub fn quiet() {}\n' >> "$1/crates/geom/src/reader.rs"
}

plant_warnings_allow() {
  printf '#[allow(\n    warnings,\n)]\npub fn quiet() {}\n' >> "$1/crates/geom/src/reader.rs"
}

plant_unarmed_bin() {
  mkdir -p "$1/crates/geom/src/bin"
  printf 'fn main() {}\n' > "$1/crates/geom/src/bin/tool.rs"
}

plant_prose_only() {
  printf '// clippy::disallowed_methods is armed by the root.\npub fn g() {}\n' >> "$1/crates/geom/src/reader.rs"
}

gate_selftest() {
  local want="a conic's centre frame is read where"
  gate_selftest_clean
  gate_selftest_case "$want" plant_new_site
  gate_selftest_case "$want" plant_allow
  gate_selftest_case "$want" plant_no_row
  gate_selftest_case "$want" plant_site_gone
  gate_selftest_case "$want" plant_unarmed_root
  gate_selftest_case "$want" plant_door_dropped
  gate_selftest_case "$want" plant_group_allow
  gate_selftest_case "$want" plant_group_expect
  gate_selftest_case "$want" plant_warnings_allow
  gate_selftest_case "$want" plant_unarmed_bin
  gate_selftest_passes "a comment that names the lint" plant_prose_only
  printf '%s selftest OK: passes a clean fixture and a comment naming the lint; fires on a listed site past its pin, an allow of the lint, a reason citing no row, a pinned site that has gone, a library root without the deny line, a door dropped from clippy.toml, an allow or an expect of a lint group that holds the lint (clippy::style, clippy::all, warnings) and a binary root without the deny line\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
