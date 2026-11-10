#!/usr/bin/env bash
# ladder-re-export-callers.sh — code outside geom-brep that still spells
# the ladder by its transitional path `geom_brep::ladder`, counted per
# crate: a ratchet that may only fall. ONE home; ci.yml's `lint` job
# runs every gate in this directory.
#
# THE RULE (work/onecopy/geom-brep-ladder-re-export-awaits-its-callers.md).
# The ladder lives at `geom_core::ladder`. `geom_brep::ladder` is a
# `#[doc(hidden)]` re-export kept so that the lanes in flight at the move
# did not conflict. New code outside geom-brep imports
# `geom_core::ladder`; geom-brep's own code says `crate::ladder`. A crate
# above its count here fails (a new caller of the old path), and a crate
# below it fails too, asking for the count to be lowered. When every
# count is gone, the re-export, this gate and its register are deleted
# together, and `rung-holds-one-home.sh`'s `LADDER` drops `geom_brep::`.
#
# WHAT IS READ: every `.rs` file under `crates/`, less geom-brep's own
# `src/` (its spelling is `crate::ladder`, which the re-export also
# serves), RAW: code, `#[cfg(test)]` modules, test-only mounts, `tests/`,
# doc links, prose and literals alike. So a count of none means nothing
# outside geom-brep's library names the old path, and the re-export can
# be deleted. Each `geom_brep::ladder` is one spelling, two on a line
# counting two.
#
# REFUSED OUTRIGHT: an import of the module itself (`use
# geom_brep::ladder;`, `use geom_brep::ladder as …`, or `ladder` in a
# `geom_brep::{…}` group on one line). A file could drop its count with
# one, spelling `ladder::` after it, and port nothing.
#
# WHAT IT CANNOT SEE: a `geom_brep::{…}` group split over lines that
# names `ladder` on a line of its own, and a macro that builds the path.
# A reviewer's, those.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

REGISTER=scripts/gates/ladder-re-export-callers.counts
PATH_RE='geom_brep::ladder\b'
ALIAS_RE='use[[:space:]]+geom_brep::ladder[[:space:]]*(;|as[[:space:]])|geom_brep::\{[^}]*\bladder\b'

gate() {
  gate_require_crate_sources
  gate_require_file "$REGISTER" "the ladder re-export's register"
  local -a files=()
  mapfile -t files < <(find crates -name '*.rs' -not -path 'crates/geom-brep/src/*' -not -path '*/target/*' | sort)
  local alias=""
  if [ "${#files[@]}" -gt 0 ]; then
    alias=$(gate_grep -nHE "$ALIAS_RE" "${files[@]}")
  fi
  if [ -n "$alias" ]; then
    printf '%s\n' "$alias"
    gate_error "a module alias of the transitional geom_brep::ladder (above): import geom_core::ladder or its items, never the old module"
    exit 1
  fi
  local live=""
  if [ "${#files[@]}" -gt 0 ]; then
    # `file:line:text`; a line may spell the path twice.
    live=$(gate_grep -nHE "$PATH_RE" "${files[@]}" \
      | awk '{ split($0, at, ":"); split(at[1], p, "/"); c[p[1] "/" p[2]] += gsub(/geom_brep::ladder/, "&") }
             END { for (k in c) print k, c[k] }')
  fi
  local want
  want=$(awk '!/^[[:space:]]*(#|$)/ { print $1, $2 }' "$REGISTER" | sort)
  local diff
  diff=$(diff <(printf '%s\n' "$live" | sed '/^$/d' | sort) <(printf '%s\n' "$want" | sed '/^$/d') || true)
  if [ -n "$diff" ]; then
    printf '%s\n' "$diff"
    gate_error "geom_brep::ladder spellings differ from $REGISTER ('<' is the tree, '>' the register): import geom_core::ladder, or lower the count where spellings moved"
    exit 1
  fi
  GATE_SCAN_FILES=${#files[@]}
  gate_ok "every crate outside geom-brep's library spells the transitional geom_brep::ladder exactly its registered count"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/scripts/gates"
  printf '# none\n' > "$1/$REGISTER"
}

plant_new_caller() {
  mkdir -p "$1/crates/planted/src"
  printf 'use geom_brep::ladder::climb;\npub fn f() {}\n' > "$1/crates/planted/src/lib.rs"
}

plant_stale_entry() {
  printf 'crates/clean 1\n' >> "$1/$REGISTER"
}

plant_registered() {
  plant_new_caller "$1"
  printf 'crates/planted 1\n' >> "$1/$REGISTER"
}

plant_in_geom_brep() {
  mkdir -p "$1/crates/geom-brep/src"
  printf 'pub use geom_brep::ladder::climb;\n' > "$1/crates/geom-brep/src/lib.rs"
}

plant_in_a_doc_link() {
  mkdir -p "$1/crates/planted/src"
  printf '/// [`geom_brep::ladder::climb`]\npub fn f() {}\n' > "$1/crates/planted/src/lib.rs"
}

plant_in_a_test_module() {
  mkdir -p "$1/crates/planted/src"
  printf '#[cfg(test)]\nmod rows { use geom_brep::ladder::climb; }\n' > "$1/crates/planted/src/lib.rs"
}

plant_in_an_integration_test() {
  mkdir -p "$1/crates/geom-brep/tests"
  printf 'use geom_brep::ladder::climb;\n' > "$1/crates/geom-brep/tests/rows.rs"
}

plant_module_alias() {
  mkdir -p "$1/crates/planted/src"
  printf 'use geom_brep::ladder;\npub fn f() { ladder::climb(); }\n' > "$1/crates/planted/src/lib.rs"
}

plant_module_alias_renamed() {
  mkdir -p "$1/crates/planted/src"
  printf 'use geom_brep::ladder as l;\n' > "$1/crates/planted/src/lib.rs"
}

plant_module_in_a_group() {
  mkdir -p "$1/crates/planted/src"
  printf 'use geom_brep::{Thing, ladder};\n' > "$1/crates/planted/src/lib.rs"
}

gate_selftest() {
  local want="spellings differ" alias="a module alias"
  gate_selftest_clean
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case "$want" plant_new_caller
  gate_selftest_case "$want" plant_stale_entry
  gate_selftest_case "$want" plant_in_a_doc_link
  gate_selftest_case "$want" plant_in_a_test_module
  gate_selftest_case "$want" plant_in_an_integration_test
  gate_selftest_case "$alias" plant_module_alias
  gate_selftest_case "$alias" plant_module_alias_renamed
  gate_selftest_case "$alias" plant_module_in_a_group
  gate_selftest_passes "a registered caller" plant_registered
  gate_selftest_passes "geom-brep's own library spelling" plant_in_geom_brep
  printf '%s selftest OK: passes a clean tree, a registered caller and geom-brep'"'"'s own library spelling; fires on a new spelling in code, a doc link, a test module and an integration test, on a register entry the tree no longer carries, and on a module alias (bare, renamed, or in a group), and stays RED when grep cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
