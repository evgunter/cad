#!/usr/bin/env bash
# gated-suite-paths.sh — every gated suite's marker resolves against the tree.
#
# THE INVARIANT: for every `test_utils::gated_to!` marker under
# `crates/<crate>/{src,tests}/`, every path it names EXISTS — as a file, or
# as a directory when the entry ends in `/` — and the nextest term that
# selects that suite's tests can be derived. A marker sited where nothing
# reads it, a file marked but holding no test, a second marker in one file,
# and a path that is not repo-relative are the same failure and fail here too.
#
# WHY IT IS A GATE AND NOT A NOTICE. A marker declares that its suite runs on
# a pull request only when one of the named paths, or the suite's own file, is
# in the diff (`scripts/ci-filter.py`, THE PER-FILE TEST GATE). That filter
# FAILS OPEN: a marker it cannot resolve does not skip its suite, so nothing
# about a green run says the marker is broken, and a rename that moves a
# suite's subject out from under it leaves a gate that reads as narrow and is
# in fact "always runs". The tree carries no other record of what a marker was
# supposed to name, so the state has to be made loud where it stops a merge —
# which is here — or it is not made loud at all.
#
# AND IT IS WHERE THAT RENAME GETS CAUGHT IN THE RIGHT LANE. The nightly's
# ungated re-take (`ci-filter.py --gated-set`) REFUSES a tree whose markers do
# not all resolve, because a lane that runs only what it derives cannot
# quietly derive less. Without this row the same rename would red a scheduled
# job hours later, in nobody's pull request.
#
# ONE IMPLEMENTATION, NOT TWO. The marker's spelling, what counts as a path
# and how a term is derived all live in `scripts/ci-filter.py`, which is the
# file that ACTS on them; this gate calls it with `--gated-check` rather than
# re-grepping for a syntax it would then own a second copy of. That is the
# same arrangement `nightly.yml`'s demoted job has with
# `nightly-only-selection.py --markers-present`.
#
# CARGO IS NOT THIS GATE'S SUBJECT. `--gated-check` asks `cargo metadata` for
# the package name behind each crate DIRECTORY, and falls back to the
# directory name — which is the same string for every member of this
# workspace — when cargo cannot answer. A wrong binary id makes a term match
# no test, and a term matching no test EXCLUDES no test, so the failure is a
# run rather than a hole. The subject here is the marker text, and the marker
# text is read without a toolchain.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

GATE_SCAN_NOUN="crate source file"

gate() {
  local out rc=0 count

  gate_require_crate_sources

  # The scan is `crates/*/{src,tests}` plus a walk for markers sited outside
  # it, so the count says what was READ rather than what one guard happened
  # to list. `gate_require_crate_sources` has already proved the tree is not
  # empty; this replaces its narrower count.
  count=$(find crates/*/src crates/*/tests -type f -name '*.rs' 2>/dev/null | wc -l | tr -d ' ')
  [ "$count" -gt 0 ] || count=$GATE_SCAN_FILES
  GATE_SCAN_FILES=$count

  out=$("$GATE_REPO_ROOT/scripts/ci-filter.py" --gated-check "$PWD" 2>&1) || rc=$?
  if [ "$rc" -ne 0 ]; then
    gate_error "$(gate_name): a gated-suite marker does not resolve against this tree"
    printf '%s\n' "$out" >&2
    exit 1
  fi
  printf '%s\n' "$out"
  gate_ok "every gated_to! marker names paths that exist and derives a nextest term"
}

# The fixture is a miniature crate tree carrying ONE healthy marker: a
# `tests/` suite aggregated under a module name that is not its filename,
# which is the shape the real tree has and the shape a derivation that
# guessed the module from the filename would pass without.
gate_plant_clean() {
  local t=$1
  mkdir -p "$t/crates/geom-core/src/interval" "$t/crates/geom-core/tests" \
           "$t/crates/test-utils/src"
  printf 'pub fn ring() {}\n' > "$t/crates/geom-core/src/lib.rs"
  printf 'pub fn r() {}\n' > "$t/crates/geom-core/src/ring.rs"
  printf 'pub fn s() {}\n' > "$t/crates/geom-core/src/interval/scalar.rs"
  printf '// the marker macro lives here\n' > "$t/crates/test-utils/src/lib.rs"
  {
    printf '#[path = "sub/ring_fuzz.rs"]\n'
    printf 'mod sub_ring_fuzz;\n'
  } > "$t/crates/geom-core/tests/all.rs"
  mkdir -p "$t/crates/geom-core/tests/sub"
  {
    printf 'test_utils::gated_to![\n'
    printf '    "crates/geom-core/src/ring.rs",\n'
    printf '    "crates/geom-core/src/interval/",\n'
    printf '];\n'
    printf '#[test]\nfn t() {}\n'
  } > "$t/crates/geom-core/tests/sub/ring_fuzz.rs"
}

# One planter per way a marker can be wrong, each rewriting the healthy
# suite's marker so the clean fixture is the only difference.
remark() {
  local t=$1 body=$2
  {
    printf '%s\n' "$body"
    printf '#[test]\nfn t() {}\n'
  } > "$t/crates/geom-core/tests/sub/ring_fuzz.rs"
}

plant_missing_path() { remark "$1" 'test_utils::gated_to!["crates/geom-core/src/renamed_away.rs"];'; }
# A DIRECTORY WITHOUT ITS TRAILING SLASH. It exists, so a check that only
# asked "is this path in the tree" would clear it — and the filter would then
# match it against changed files as a FILE, which no changed file ever equals,
# leaving the suite gated on nothing it can ever hit.
plant_dir_without_slash() { remark "$1" 'test_utils::gated_to!["crates/geom-core/src/interval"];'; }
plant_absolute_path() { remark "$1" 'test_utils::gated_to!["/crates/geom-core/src/ring.rs"];'; }
plant_escaping_path() { remark "$1" 'test_utils::gated_to!["../crates/geom-core/src/ring.rs"];'; }
plant_no_paths() { remark "$1" 'test_utils::gated_to![];'; }
plant_two_markers() {
  remark "$1" 'test_utils::gated_to!["crates/geom-core/src/ring.rs"];
test_utils::gated_to!["crates/geom-core/src/lib.rs"];'
}

# A suite file that `tests/all.rs` does not aggregate. It compiles nowhere —
# `autotests = false` — so the term would name a module prefix no test id
# carries, and a term that matches nothing excludes nothing.
plant_unaggregated() {
  printf 'test_utils::gated_to!["crates/geom-core/src/ring.rs"];\n#[test]\nfn t() {}\n' \
    > "$1/crates/geom-core/tests/loose_fuzz.rs"
}

# A marker on a file with no test in it: the term selects an empty set, so the
# marker gates nothing while reading exactly like one that does.
plant_marker_without_tests() {
  printf 'test_utils::gated_to!["crates/geom-core/src/ring.rs"];\npub fn f() {}\n' \
    > "$1/crates/geom-core/src/helper.rs"
}

# Sited where no derivation looks: an excluded workspace, and the marker's own
# home. Both read like a gate and are inert.
plant_marker_outside_crates() {
  mkdir -p "$1/demos/tour/src"
  printf 'test_utils::gated_to!["crates/geom-core/src/ring.rs"];\n' \
    > "$1/demos/tour/src/lib.rs"
}
plant_marker_in_test_utils() {
  printf 'test_utils::gated_to!["crates/geom-core/src/ring.rs"];\n' \
    >> "$1/crates/test-utils/src/lib.rs"
}

# THE NEAR MISS, and every widened matcher in this directory owes one: prose
# naming the macro is not a call, and a gate that fired on it would push
# authors to stop writing about the mechanism in the files that use it.
plant_prose_mention() {
  printf '//! see gated_to in crates/test-utils for what this suite is gated to\n' \
    >> "$1/crates/geom-core/src/lib.rs"
}

gate_selftest() {
  gate_selftest_clean
  gate_selftest_passes "prose naming the macro without calling it" plant_prose_mention
  gate_selftest_case 'does not exist in the tree' plant_missing_path
  gate_selftest_case 'it is a DIRECTORY: name it with a trailing' plant_dir_without_slash
  gate_selftest_case 'is not a repo-relative path' plant_absolute_path
  gate_selftest_case 'is not a repo-relative path' plant_escaping_path
  gate_selftest_case 'names no path at all' plant_no_paths
  gate_selftest_case 'one marker per file' plant_two_markers
  gate_selftest_case 'declares no `#[path' plant_unaggregated
  gate_selftest_case 'no `#[test]` or `#[cfg(test)]`' plant_marker_without_tests
  gate_selftest_case 'gates nothing while reading like one that does' plant_marker_outside_crates
  gate_selftest_case "marker's own home" plant_marker_in_test_utils
  printf '%s selftest OK: passes a tree whose one marker resolves (and prose that merely names the macro); fires on a path that is not there, a directory written without its trailing slash, an absolute path, one escaping the repo, an empty path set, two markers in one file, a suite tests/all.rs does not aggregate, a marker on a file with no test, and a marker sited outside crates/<crate>/{src,tests} or inside the macro'"'"'s own crate\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
