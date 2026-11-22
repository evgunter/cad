#!/usr/bin/env bash
# parallel-maps-carry-the-role.sh — every rayon map in the workspace's
# sources goes through `geom_core::k_stats::par_map`. ONE home; ci.yml's
# `lint` job runs every gate in this directory.
#
# THE RULE (NAMING-DESIGN N5's roles, PR 7114): a verdict's role (a
# choice or a check, `k_stats::Role`) is the scope its caller opened,
# held in a thread-local. A rayon worker blocked inside a check-scoped
# `collect` steals other work from the pool, so a job can run on a
# thread in any role. `par_map` captures the role of the thread that
# maps and SETS it in every job; a bare `par_iter` runs the job in the
# stealing thread's role, which turns choices into checks on some
# schedules (`geom-core/tests/k_stats_roles.rs`,
# `a_stolen_job_runs_in_its_spawners_role`). So no source outside the
# home spells a rayon map, a join, a scope or a spawn.
#
# KNOWN GAPS: a map spelled through a renamed import (`use rayon::join
# as j`) or a helper of another crate that wraps rayon is not seen. A
# `ThreadPoolBuilder` is not a map and is not matched; a job installed
# into a pool runs in the installing thread's role.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

# THE HOME'S PATH, held once: the filter's exemption, `gate_require_homes`'s
# subject check and the clean fixture all read this name.
HOME_FILE=crates/geom-core/src/k_stats.rs
HOME_SUBJECT='the one home of a recorded parallel map, `k_stats::par_map`'

PATTERN='\.(into_)?par_iter(_mut)?\(|\.par_bridge\(|\.par_chunks|\.par_extend\(|\brayon::(join|scope|spawn)\b'

gate() {
  gate_require_crate_sources
  gate_require_homes "$HOME_SUBJECT" "$HOME_FILE"
  local hits
  hits=$(gate_rust_code "${GATE_SOURCE_FILES[@]}" \
    | gate_grep -E "$PATTERN" \
    | gate_grep -vE "$(gate_record_anchor "$HOME_FILE")")
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "a rayon map outside geom_core::k_stats::par_map: its jobs would run in the role of whatever thread steals them, not their spawner's. Map through k_stats::par_map (or k_stats::map_detached)"
    exit 1
  fi
  gate_ok "no rayon map outside geom_core::k_stats::par_map"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/${HOME_FILE%/*}"
  printf 'pub fn par_map(v: Vec<u32>) -> Vec<u32> { v.into_par_iter().map(|x| x).collect() }\n' \
    > "$1/$HOME_FILE"
}

plant_colon_after_the_home_that_is_not_a_line_number() {
  printf 'pub fn m(v: &[u32]) -> Vec<u32> { v.par_iter().copied().collect() }\n' \
    > "$1/$HOME_FILE:x.rs"
}

plant() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn m(v: &[u32]) -> Vec<u32> { v.par_iter().copied().collect() }\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_into() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn m(n: usize) -> Vec<usize> { (0..n).into_par_iter().collect() }\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_join() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn m() -> (u8, u8) { rayon::join(|| 1, || 2) }\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_prose_only() {
  mkdir -p "$1/crates/planted/src"
  {
    printf '//! A map goes through par_map, never a bare .par_iter( call.\n'
    printf '/*\n * rayon::join is named in this block comment only.\n */\n'
    printf 'pub const WHY: &str = ".into_par_iter()";\n'
    printf 'pub fn ok(a: f64) -> f64 { a } // and .par_bridge( in a trailing one\n'
  } > "$1/crates/planted/src/lib.rs"
}

gate_selftest() {
  local want="a rayon map outside geom_core::k_stats::par_map"
  gate_selftest_clean
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case "$want" plant
  gate_selftest_case "$want" plant_into
  gate_selftest_case "$want" plant_join
  gate_selftest_case "$want" plant_colon_after_the_home_that_is_not_a_line_number
  gate_selftest_passes "prose, doc comments and a string literal naming a rayon map" plant_prose_only
  gate_selftest_homes --subject "$HOME_SUBJECT" "$HOME_FILE"
  printf '%s selftest OK: passes a clean fixture carrying the home itself, and prose/doc/string mentions of a rayon map; fires on a par_iter, an into_par_iter and a rayon::join outside the home, and at the colon-carrying path a home skip that ends at `:` exempts; and it stays RED, with a diagnosis, when `grep` itself cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
