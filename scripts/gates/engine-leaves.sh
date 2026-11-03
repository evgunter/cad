#!/usr/bin/env bash
# engine-leaves.sh — `topo`'s operation directories are leaves, pinned
# per file. ONE home; ci.yml's `lint` job runs every gate in this
# directory.
#
# THE RULE is `[topo.homes]` in crates/topo/README.md: `boolean/` and
# `splitting/` are leaves — nothing outside an engine names it, neither
# names the other, and an engine declares nothing crate-wide that is not
# a `pub` door.
#
# So the sites that break it today are PINNED per (kind, file) in
# `engine-leaves.counts`, and the pin only falls: a new site reds, and so
# does a pinned one that is gone, so the change that clears a site lowers
# its line in the same diff. The file is the move list.
#
# WHAT IT COUNTS, in every file of crates/topo/src, test code included
# (comments and literals stripped, `gate_rust_code`): a path through an
# engine's module, and every occurrence of an item the engine DECLARES,
# by name, however it reached scope — so an alias, a hub's re-export,
# `use super::X`, a glob and the root's own `pub use` are all the same
# site. A name also declared outside the engines cannot be told apart by
# name: it is listed as `ambiguous` in the counts file and counted only
# where a path or an import through the engine spells it. The counter,
# and what it can and cannot see, is `engine_leaves.py`.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"
CENSUS="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/engine_leaves.py"

BASELINE=scripts/gates/engine-leaves.counts
MANIFEST=crates/topo/Cargo.toml
RULE='`[topo.homes]` (crates/topo/README.md): code naming a type declared under an engine belongs to that engine; anything else goes to the crate root or below'

gate() {
  gate_require_file "$BASELINE"
  gate_require_file "$MANIFEST"
  local files report
  mapfile -t files < <(find crates/topo/src -type f -name '*.rs' 2>/dev/null | sort)
  GATE_SCAN_FILES=${#files[@]}
  if [ "$GATE_SCAN_FILES" -eq 0 ]; then
    gate_error "$(gate_name): no .rs file under crates/topo/src in $PWD — the gate scanned nothing, which is not a pass"
    exit 1
  fi
  if ! report=$(gate_rust_code "${files[@]}" | python3 "$CENSUS" "$BASELINE" "$MANIFEST"); then
    gate_error "$(gate_name): the counter failed under $PWD (its own message is above, if it had one) — the gate could not read its subject, which is not a pass"
    exit 1
  fi
  local failed=0
  if grep -q '^MALFORMED ' <<<"$report"; then
    grep '^MALFORMED ' <<<"$report"
    gate_error "$BASELINE has a line it cannot read (above)"
    failed=1
  fi
  if grep -qE '^NEW (names|boolean-names|splitting-names)' <<<"$report"; then
    grep -E '^NEW (names|boolean-names|splitting-names)' <<<"$report"
    gate_error "an engine is named past its pin (above) — $RULE. Move the code; do not raise the count"
    failed=1
  fi
  if grep -q '^NEW crate-visible' <<<"$report"; then
    grep '^NEW crate-visible' <<<"$report"
    gate_error "an engine declares an item crate-wide (above) — $RULE. A door is \`pub\`; anything else is private to the engine: \`pub(in crate::<engine>)\` or narrower (in an engine's mod.rs, \`pub(super)\` is crate-wide)"
    failed=1
  fi
  if grep -q '^STALE ' <<<"$report"; then
    grep '^STALE ' <<<"$report"
    gate_error "a pinned site is gone (above): drop or lower its line in $BASELINE"
    failed=1
  fi
  if grep -qE '^AMBIGUOUS-(NEW|GONE) ' <<<"$report"; then
    grep -E '^AMBIGUOUS-(NEW|GONE) ' <<<"$report"
    gate_error "the names an engine shares with code outside it moved (above): add or drop their \`ambiguous NAME\` lines in $BASELINE — an ambiguous name is counted only where a path through the engine spells it"
    failed=1
  fi
  [ "$failed" = 0 ] || exit 1
  gate_ok "the engines' doors at their pinned counts ($(sed -n 's/^TOTAL //p' <<<"$report") sites in all)"
}

# THE FIXTURE: a root naming one engine type through a full path, pinned.
gate_plant_clean() {
  local t="$1/crates/topo"
  mkdir -p "$t/src/boolean" "$t/src/splitting" "$1/scripts/gates"
  printf '[package]\nname = "topo"\n[dependencies]\ngeom-brep = { path = "." }\n' > "$t/Cargo.toml"
  printf 'pub mod boolean;\npub mod splitting;\nmod reader;\npub use boolean::{Op, union};\n' > "$t/src/lib.rs"
  printf 'pub enum Op { A }\npub fn union() {}\npub(super) fn wide() {}\nmod zip;\n' > "$t/src/boolean/mod.rs"
  printf 'pub(super) fn run() {}\npub(in crate::boolean) struct Inner;\n' > "$t/src/boolean/zip.rs"
  printf 'pub struct Plane;\npub fn cut() {}\n' > "$t/src/splitting/mod.rs"
  printf 'pub fn read() { let _ = crate::boolean::Op::A; }\npub struct Side;\nfn local(union: u8) -> u8 { union }\n' \
    > "$t/src/reader.rs"
  printf '# kind count file\ncrate-visible 1 boolean/mod.rs\nnames-boolean 1 reader.rs\n' > "$1/$BASELINE"
}

append() { printf '%s\n' "$2" >> "$1/crates/topo/src/$3"; }

plant_alias_of_crate()   { append "$1" 'use crate as c; fn a() { c::boolean::union(); }' reader.rs; }
plant_self_alias()       { append "$1" 'use crate::{self as r}; fn a() { let _ = r::boolean::Op::A; }' reader.rs; }
plant_nested_hub()       { append "$1" 'mod hub { pub use crate::boolean::Op; }' lib.rs; }
plant_super_import()     { append "$1" 'use super::Op;' reader.rs; }
plant_module_alias()     { append "$1" 'use crate::boolean as b; fn a() { let _ = b::Op::A; b::union(); }' reader.rs; }
plant_root_glob()        { append "$1" 'use crate::*; fn a() { union(); }' reader.rs; }
plant_root_reexport()    { append "$1" 'fn a() -> crate::Op { crate::Op::A }' reader.rs; }
plant_cross_engine()     { append "$1" 'use crate::boolean::Op;' splitting/mod.rs; }
plant_cross_by_name()    { append "$1" 'fn a() { crate::cut(); }' boolean/zip.rs; }
plant_crate_visible()    { append "$1" 'pub(crate) fn leak() {}' boolean/zip.rs; }
plant_super_from_mod()   { append "$1" 'pub(super) fn leak() {}' splitting/mod.rs; }
plant_site_gone()        { sed -i 's/crate::boolean::Op::A/0/' "$1/crates/topo/src/reader.rs"; }
plant_zero_line()        { printf 'names-splitting 0 reader.rs\n' >> "$1/$BASELINE"; }
plant_unknown_kind()     { printf 'names-everything 1 reader.rs\n' >> "$1/$BASELINE"; }
plant_ambiguous()        { append "$1" 'pub struct Plane;' reader.rs; }
plant_ambiguous_listed() {
  append "$1" 'pub struct Plane;' reader.rs
  printf '  ambiguous Plane   # also reader.rs\n' >> "$1/$BASELINE"
}
plant_near_misses() {
  append "$1" 'fn n(x: &S) { x.union(); let union = 1; geom_brep::union(); Op2::union(); let _ = union; }' reader.rs
  append "$1" 'use geom_brep::Op as GOp; fn m() -> GOp { todo!() }' reader.rs
}

gate_selftest() {
  gate_selftest_clean
  gate_selftest_without_tool python3 "the counter failed"
  gate_selftest_passes "a field, a local, a method and other crates' items spelled like engine items" plant_near_misses
  gate_selftest_passes "an ambiguous name listed as such" plant_ambiguous_listed
  local p
  for p in plant_alias_of_crate plant_self_alias plant_nested_hub plant_super_import \
      plant_module_alias plant_root_glob plant_root_reexport; do
    gate_selftest_case --also "NEW names-boolean" "Move the code" "$p"
  done
  gate_selftest_case --also "NEW splitting-names-boolean" "Move the code" plant_cross_engine
  gate_selftest_case --also "NEW boolean-names-splitting" "Move the code" plant_cross_by_name
  gate_selftest_case --also "NEW crate-visible boolean/zip.rs" "private to the engine" plant_crate_visible
  gate_selftest_case --also "NEW crate-visible splitting/mod.rs" "private to the engine" plant_super_from_mod
  gate_selftest_case --also "STALE names-boolean reader.rs" "drop or lower" plant_site_gone
  gate_selftest_case "cannot read" plant_zero_line
  gate_selftest_case "cannot read" plant_unknown_kind
  gate_selftest_case --also "AMBIGUOUS-NEW Plane" "ambiguous NAME" plant_ambiguous
  printf '%s selftest OK: passes a pinned fixture, near misses (a field, a local, a method, another crate'"'"'s item) and a listed ambiguous name; fires on an engine named through an alias of crate, a self alias, a hub nested in lib.rs, use super::X, a module alias, a root glob and a root re-export, across engines by path and by name, on crate-wide visibility (pub(super) in an engine mod.rs included), on a pinned site gone, on a zero or unknown baseline line and on an unlisted ambiguous name, and diagnoses a counter that cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
