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
# WHAT IS READ: production sources (`crates/*/src`, less test-only
# mounts and `#[cfg(test)]` items) outside `crates/geom-brep/`, in the
# code view (comments and literals blanked, so a doc link is not
# counted), for each `geom_brep::ladder` path.
#
# WHAT IT CANNOT SEE: a `use geom_brep::{ladder, ..}` group (it names the
# module apart from its crate) and a test module's spelling. A
# reviewer's, those.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

REGISTER=scripts/gates/ladder-re-export-callers.counts
PATH_RE='geom_brep::ladder\b'

gate() {
  gate_require_crate_sources
  gate_require_file "$REGISTER" "the ladder re-export's register"
  gate_production_sources
  local -a files=()
  local f
  for f in "${GATE_PRODUCTION_FILES[@]}"; do
    case "$f" in crates/geom-brep/*) ;; *) files+=("$f") ;; esac
  done
  local live=""
  if [ "${#files[@]}" -gt 0 ]; then
    # Each line is `file:line:code`; a line may spell the path twice.
    live=$(gate_rust_code --skip-cfg-test "${files[@]}" | gate_grep -E "$PATH_RE" \
      | awk '{ split($0, at, ":"); split(at[1], p, "/"); c[p[1] "/" p[2]] += gsub(/geom_brep::ladder/, "&") }
             END { for (k in c) print k, c[k] }')
  fi
  local want
  want=$(awk '!/^[[:space:]]*(#|$)/ { print $1, $2 }' "$REGISTER" | sort)
  local diff
  diff=$(diff <(printf '%s\n' "$live" | sed '/^$/d' | sort) <(printf '%s\n' "$want" | sed '/^$/d') || true)
  if [ -n "$diff" ]; then
    printf '%s\n' "$diff"
    gate_error "geom_brep::ladder spellings differ from $REGISTER ('<' is the tree, '>' the register): import geom_core::ladder, or lower the count where callers moved"
    exit 1
  fi
  gate_ok "every crate outside geom-brep spells the transitional geom_brep::ladder exactly its registered count"
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

gate_selftest() {
  local want="spellings differ"
  gate_selftest_clean
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case "$want" plant_new_caller
  gate_selftest_case "$want" plant_stale_entry
  gate_selftest_passes "a registered caller" plant_registered
  gate_selftest_passes "geom-brep's own spelling" plant_in_geom_brep
  gate_selftest_passes "a doc link" plant_in_a_doc_link
  printf '%s selftest OK: passes a clean tree, a registered caller, geom-brep'"'"'s own spelling and a doc link; fires on a new caller and on a register entry the tree no longer carries, and stays RED when grep cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
