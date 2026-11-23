#!/usr/bin/env bash
# conic-centre-reads.sh — holds the wiring of the census of production
# reads of a conic's centre doors. ONE home; ci.yml's `lint` job runs
# every gate in this directory.
#
# THE RULE it serves (`crates/geom-core/README.md`, "Float rounding"):
# "A conic's position is the point it stores plus two vectors times two
# functions of its parameter, and every reading of where the conic is
# (at a parameter, along it, or of a point against it) is read from
# that position." An ellipse held on its arc derives its centre on each
# read, at its semi-axes' rounding, so a reader that takes the centre as
# a datum states another curve than the one the evaluator draws.
#
# WHO COUNTS THE READS: clippy, by type, and not this script.
# `clippy.toml`'s `disallowed-methods` names the doors that hand out the
# centre (`Conic::frame`, `Conic::center`, `Circle::frame`,
# `Ellipse::frame`, `Curve3::frame`), the one that reads a point about
# it (`Conic::unit_in_plane`), and the two that take a conic restated on
# a caller's frame (`Curve3::circle_at`, `Cover::arc`). The lint job's
# clippy runs with `--force-warn clippy::disallowed_methods`, which
# reports every use whatever attribute sits over it, and
# `scripts/conic_centre_census.py` reads those diagnostics and holds the
# production reads per file at THE PINS (`conic-centre-reads.counts`).
# No attribute's text is read anywhere, so no spelling of one hides a
# read. Run it by hand as the census script's header gives it; with the
# lint job's arguments it is a replay of clippy's cache.
#
# Beside the census, for a reader at the site and for a plain
# `cargo clippy`: the workspace allows the lint, every library and
# binary root denies it outside `cfg(test)`, and a production function
# that still reads a door carries `#[expect(clippy::disallowed_methods,
# reason = "...")]`, which clippy reds once its read has gone.
#
# WHAT THIS GATE HOLDS (it runs before any toolchain is installed, so it
# cannot run clippy): that the wiring is there. The doors in
# `clippy.toml`; `--force-warn clippy::disallowed_methods` on the lint
# job's clippy in `.github/workflows/ci.yml`, and the census step after
# it; the workspace `allow` and each root's `deny` line; the pins file.
#
# WHAT THE CENSUS DOES NOT SEE:
#  - a centre restated without the doors: `Curve3::data()` walks the
#    stored point by name; `point_at(θ) + second_derivative_at(θ)` sums
#    it; a whole circle's cover's ball stands at it
#    (`c.cover_over(full turn).ball().0`);
#  - the closures of `Circle::placed`, `Ellipse::mapped` and the two
#    `map_scalar`s, which are handed the STORED frame (for an ellipse
#    the centre or the anchor, with nothing saying which);
#  - a conic rebuilt from a struct another function filled with a
#    centre (`crates/topo/src/boolean/mod.rs`'s `Rim` round trip);
#  - the trait's default `extent_along` and `cover_over`, which read
#    `self.frame()` inside the conic's own module: a kind added that can
#    be held off its centre and does not state its own reads a derived
#    centre there, and nothing reds;
#  - a read through a `Frame` another function was handed;
#  - `demos/`, `benches/` and `tools/`, roots outside the workspace,
#    which allow the lint in their manifests and are not in the census;
#  - a scoped run's unchecked crates: a PR's lint job checks the crates
#    its diff reaches, and the census holds those; the unscoped run
#    holds all.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

PINS=scripts/gates/conic-centre-reads.counts
CENSUS=scripts/conic_centre_census.py
WORKFLOW=.github/workflows/ci.yml
ARMING='#![cfg_attr(not(test), deny(clippy::disallowed_methods))]'
FORCE='--force-warn clippy::disallowed_methods'
DOORS=(geom::Conic::frame geom::Conic::center geom::Circle::frame geom::Ellipse::frame geom::Curve3::frame geom::Conic::unit_in_plane geom::Curve3::circle_at geom::cover::Cover::arc)

gate() {
  local f
  for f in "$PINS" "$CENSUS" "$WORKFLOW" clippy.toml Cargo.toml; do gate_require_file "$f"; done
  local bad=0 door root
  for door in "${DOORS[@]}"; do
    if ! grep -qF "path = \"$door\"" clippy.toml; then
      printf 'clippy.toml: disallowed-methods does not name %s\n' "$door"
      bad=1
    fi
  done
  # Every clippy invocation of the lint job's step carries the flag (a
  # run without it reports only what no attribute covers), and the
  # census reads one of them.
  local plain forced census
  plain=$(grep -cE '^\s*cargo clippy \$SCOPE ' "$WORKFLOW" || true)
  forced=$(grep -E '^\s*cargo clippy \$SCOPE ' "$WORKFLOW" | grep -cF -- "$FORCE" || true)
  census=$(grep -cF "python3 $CENSUS --check" "$WORKFLOW" || true)
  if [ "$plain" -lt 2 ] || [ "$plain" != "$forced" ] || [ "$census" -lt 1 ]; then
    printf '%s: the lint job does not run clippy twice on $SCOPE with %s and pipe the second to %s --check (%s invocation(s), %s with the flag, %s census)\n' \
      "$WORKFLOW" "$FORCE" "$CENSUS" "$plain" "$forced" "$census"
    bad=1
  fi
  if ! grep -qxF 'disallowed_methods = "allow"' Cargo.toml; then
    printf 'Cargo.toml: [workspace.lints.clippy] does not allow disallowed_methods (the roots arm it)\n'
    bad=1
  fi
  local roots=()
  mapfile -t roots < <(find crates \( -path '*/src/lib.rs' -o -path '*/src/main.rs' -o -path '*/src/bin/*.rs' \) -type f -print 2>/dev/null | sort)
  if [ "${#roots[@]}" -eq 0 ]; then
    gate_error "$(gate_name): no library or binary root under crates/ in $PWD, so the scan read nothing, which is not a pass"
    exit 1
  fi
  GATE_SCAN_FILES=${#roots[@]}
  for root in "${roots[@]}"; do
    if ! grep -qxF "$ARMING" "$root"; then
      printf '%s: does not carry %s\n' "$root" "$ARMING"
      bad=1
    fi
  done
  if grep -vE '^(#.*|crates/[^ ]+\.rs [1-9][0-9]*)$' "$PINS" | grep -q .; then
    printf '%s: a line that is neither a comment nor "<file> <count>"\n' "$PINS"
    bad=1
  fi
  if [ "$bad" -ne 0 ]; then
    gate_error "the census of a conic's centre reads is not wired (above): clippy counts the reads under $FORCE and $CENSUS holds them at $PINS; without the doors, the flag, the census step or a root's deny line a new read of a derived centre lands unseen"
    exit 1
  fi
  gate_ok "the census of a conic's centre reads is wired: the doors, the forced lint on the lint job's clippy, the census step, and the deny line in every library and binary root"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/crates/geom/src" "$1/scripts/gates" "$1/.github/workflows"
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
  printf '# pins\ncrates/geom/src/reader.rs 1\n' > "$1/$PINS"
  printf '# census\n' > "$1/$CENSUS"
  {
    printf '          cargo clippy $SCOPE --all-targets -- -D warnings %s\n' "$FORCE"
    printf '          cargo clippy $SCOPE --all-targets --message-format=json -- -D warnings %s \\\n' "$FORCE"
    printf '            | python3 %s --check\n' "$CENSUS"
  } > "$1/$WORKFLOW"
}

plant_flag_dropped() {
  printf '          cargo clippy $SCOPE --all-targets -- -D warnings\n' >> "$1/$WORKFLOW"
}

plant_census_dropped() {
  grep -vF "$CENSUS" "$1/$WORKFLOW" > "$1/$WORKFLOW.new"
  mv "$1/$WORKFLOW.new" "$1/$WORKFLOW"
}

plant_unarmed_root() {
  printf 'pub fn f() {}\n' > "$1/crates/geom/src/lib.rs"
}

plant_unarmed_bin() {
  mkdir -p "$1/crates/geom/src/bin"
  printf 'fn main() {}\n' > "$1/crates/geom/src/bin/tool.rs"
}

plant_door_dropped() {
  grep -v 'geom::Ellipse::frame' "$1/clippy.toml" > "$1/clippy.toml.new"
  mv "$1/clippy.toml.new" "$1/clippy.toml"
}

plant_pin_malformed() {
  printf 'crates/geom/src/reader.rs one\n' >> "$1/$PINS"
}

gate_selftest() {
  local want="the census of a conic's centre reads is not wired"
  gate_selftest_clean
  gate_selftest_case "$want" plant_flag_dropped
  gate_selftest_case "$want" plant_census_dropped
  gate_selftest_case "$want" plant_unarmed_root
  gate_selftest_case "$want" plant_unarmed_bin
  gate_selftest_case "$want" plant_door_dropped
  gate_selftest_case "$want" plant_pin_malformed
  python3 "$GATE_REPO_ROOT/$CENSUS" --selftest
  printf '%s selftest OK: passes a clean fixture; fires on a clippy invocation without the forced lint, a missing census step, a library and a binary root without the deny line, a door dropped from clippy.toml and a malformed pin; the census script passed its own\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
