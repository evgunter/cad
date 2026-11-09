#!/usr/bin/env bash
# breakpoints-by-hand.sh — a spline's distinct breakpoints are read
# through `breakpoints()`, never spelled `runs().map(|(v, _)| v)` by hand.
# ONE home; ci.yml's `lint` job runs every gate in this directory.
#
# THE RULE
# (`work/onecopy/a-knot-vectors-breakpoints-are-collected-four-times-with-no-door.md`).
# `geom_core::spline::Knots::breakpoints`
# (`crates/geom-core/src/spline/knot_values.rs`) and
# `KnotVector::breakpoints` (`crates/geom-core/src/spline/knots.rs`) are
# the one statement of where a spline's Bézier pieces begin and end: each
# breakpoint once, ascending, clamps included. A site that drops a run's
# multiplicity by hand is a second statement of it, and the shape a fifth
# site copies from; one that copies every knot out (repeated at a
# multiple knot) and deduplicates it is another. Production code takes
# the doors; the two home files are exempt.
#
# THE LISTS ARE THE ARRAYS BELOW, and only there: `REFUSED_ADAPTERS`,
# `PASSED_ADAPTERS`, `COPY_SOURCES`, `COPY_FORMS` and `DEDUPS`. The
# matcher's alternations and the OK line are built from them, and the
# self-test plants every entry (each refused one red, each passed one
# green), so an entry the matcher drops reds the self-test, and a list
# written out a second time has nowhere to live.
#
# WHAT IS REFUSED, in production code under `crates/*/src` (comments and
# string literals out, `#[cfg(test)]` modules and test-only mounts out,
# and files named `test_support*`, `*_rows.rs` or `*_fuzz.rs` out, as
# `normalize-by-hand.sh` reads them), whitespace and rustfmt's wrap
# across lines allowed throughout:
#   * a closure that destructures a run and drops its multiplicity, in any
#     parameter position, passed to a `REFUSED_ADAPTERS` adapter of
#     `runs(…)` / `knot_runs(…)` (one that takes the closure's value as
#     the stream's or the result's), method or UFCS (`Knots::runs(k)`):
#     `|(v, _)|`, `|(v, _m)|`, `|(v, ..)|`, `&`-borrowed, `move`, typed,
#     whatever its body;
#   * `for (v, _) in …runs(…)` (the same patterns);
#   * `let (v, _) = …runs(…)….unzip()`;
#   * a `COPY_SOURCES` read copied out by a `COPY_FORMS` form, and a
#     `DEDUPS` call on the copy within the next four statements of the
#     same file.
#
# NOT REFUSED. A closure that ignores the multiplicity under a
# `PASSED_ADAPTERS` adapter, one that passes the run on or asks a
# question of it: the `_` drops nothing from the stream there, and
# `runs().filter(|(v, _)| …).map(|(_, m)| m)` needs the multiplicity it
# keeps. A refused adapter that drops it later in the chain is a site.
#
# BLIND SPOTS, each left for its reason:
#   * a `.map(|r| r.0)` that indexes the pair rather than destructuring
#     it (the second pass is
#     `grep -rnE 'runs\([^()]*\)[^;]*\.map\(\|\w+\|[^;]*\.0\b' crates/*/src`);
#   * the runs collected first and projected later; a helper fn taking
#     the runs as an argument;
#   * a copy and its dedup more than four statements apart, or
#     deduplicated by another means (a `BTreeSet`, a hand loop);
#   * `interior_knot_runs()`: interior values only, a different set, and
#     `InteriorKnot`-typed, so never this door's.
# AND WHERE IT CRIES WOLF, read by text and not by type:
#   * `values()` is a name: a map's `values()` copied and deduplicated;
#   * a dedup within four statements of a copy, on an unrelated vec;
#   * a tolerant `dedup_by(|a, b| (a - b).abs() < tol)`, which collects a
#     different set from the breakpoints;
#   * a refused adapter whose closure keeps something in the
#     multiplicity's place, `runs().map(|(v, _)| (v, 1))`.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

HOME_FILE=crates/geom-core/src/spline/knot_values.rs
TWIN_FILE=crates/geom-core/src/spline/knots.rs
HOME_SUBJECT='the home of a spline'"'"'s breakpoints, Knots::breakpoints'

# Each entry is `adapter~~a planted call` on a runs stream; the call drops
# the multiplicity in the closure's pair.
REFUSED_ADAPTERS=(
  'map~~map(|(v, _)| v)'
  'filter_map~~filter_map(|(v, _)| f(v))'
  'find_map~~find_map(|(v, _)| f(v))'
  'flat_map~~flat_map(|(v, _)| g(v))'
  'for_each~~for_each(|(v, _)| use_it(v))'
  'fold~~fold(0.0, |acc, (v, _)| acc + v)'
  'scan~~scan(0.0, |s, (v, _)| Some(v))'
  'map_while~~map_while(|(v, _)| f(v))'
)
PASSED_ADAPTERS=(
  'filter~~filter(|(v, _)| p(v))'
  'find~~find(|(v, _)| p(v))'
  'inspect~~inspect(|(v, _)| log(v))'
  'any~~any(|(v, _)| p(v))'
  'all~~all(|(v, _)| p(v))'
  'position~~position(|(v, _)| p(v))'
  'take_while~~take_while(|(v, _)| p(v))'
  'skip_while~~skip_while(|(v, _)| p(v))'
)
COPY_SOURCES=(knots values)
# Each entry is `name~~ERE~~planted expression`; `@SRC@` stands for a
# source read (`kv.knots()` when planted, the sources' alternation in
# the ERE).
COPY_FORMS=(
  'to_vec~~@SRC@ *\. *to_vec *\(~~@SRC@.to_vec()'
  'to_owned~~@SRC@ *\. *to_owned *\(~~@SRC@.to_owned()'
  'iter().copied().collect()~~@SRC@ *\. *iter *\( *\) *\. *copied *\( *\) *\. *collect~~@SRC@.iter().copied().collect()'
  'iter().cloned().collect()~~@SRC@ *\. *iter *\( *\) *\. *cloned *\( *\) *\. *collect~~@SRC@.iter().cloned().collect()'
  'Vec::from~~(^|[^A-Za-z0-9_])Vec *:: *from *\([^;]*@SRC@~~Vec::from(@SRC@)'
)
DEDUPS=(dedup dedup_by dedup_by_key)

# The first `~~`-field of each entry, one per line.
names_of() {
  local e
  for e in "$@"; do printf '%s\n' "${e%%~~*}"; done
}

# Lines on stdin, comma-joined, for the OK line.
joined() { paste -sd, - | sed 's/,/, /g'; }


copy_re() {
  local src e rest re out=()
  src="(^|[^A-Za-z0-9_])($(gate_ere_alternation "${COPY_SOURCES[@]}")) *\( *\)"
  for e in "${COPY_FORMS[@]}"; do
    rest=${e#*~~}; re=${rest%%~~*}
    out+=("(${re//@SRC@/$src})")
  done
  gate_ere_alternation "${out[@]}" | sed 's/ \*/[[:space:]]*/g'
}

gate() {
  gate_require_crate_sources
  gate_production_sources
  gate_require_homes "$HOME_SUBJECT" "$HOME_FILE" "$TWIN_FILE"
  local files=() f
  for f in "${GATE_PRODUCTION_FILES[@]}"; do
    case "${f##*/}" in
      test_support*|*_rows.rs|*_fuzz.rs) ;;
      *) files+=("$f") ;;
    esac
  done
  GATE_SCAN_FILES=${#files[@]}
  local runs='(^|[^A-Za-z0-9_])(knot_)?runs[[:space:]]*\([^()]*\)'
  local dropped='&?[[:space:]]*\([[:space:]]*&?[[:space:]]*[A-Za-z_][A-Za-z0-9_]*[[:space:]]*,[[:space:]]*(_[A-Za-z0-9_]*|\.\.)[[:space:]]*\)'
  local adapter
  adapter="($(names_of "${REFUSED_ADAPTERS[@]}" | paste -sd'|' -))"
  local closure="$runs[^;]*\.$adapter[[:space:]]*\([^|;]*\|[^|;]*(move[[:space:]]+)?$dropped"
  local for_loop="(^|[^A-Za-z0-9_])for[[:space:]]+$dropped[[:space:]]+in[[:space:]][^;{]*(knot_)?runs[[:space:]]*\("
  local unzipped="(^|[^A-Za-z0-9_])let[[:space:]]+$dropped[^=;]*=[^;]*(knot_)?runs[[:space:]]*\([^;]*\.unzip[[:space:]]*\("
  local stream hits
  stream=$(gate_rust_code --skip-cfg-test --statements "${files[@]}" \
    | gate_grep -vE "$(gate_record_anchor "$HOME_FILE")|$(gate_record_anchor "$TWIN_FILE")" || true)
  hits=$(printf '%s\n' "$stream" | gate_grep -E "$closure|$for_loop|$unzipped" || true)
  hits+=$(printf '%s\n' "$stream" \
    | COPY_RE="$(copy_re)" DEDUP_RE="\\.[[:space:]]*($(gate_ere_alternation "${DEDUPS[@]}"))[[:space:]]*\\(" \
      awk -F: '
    BEGIN { COPY = ENVIRON["COPY_RE"]; DEDUP = ENVIRON["DEDUP_RE"] }
    $0 ~ COPY { held = $0; file = $1; left = 4; next }
    held != "" && $1 == file && left > 0 && $0 ~ DEDUP { print held; held = ""; next }
    held != "" { left--; if (left <= 0 || $1 != file) held = "" }
  ')
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "a spline's breakpoints collected by hand (above): read them as knots.breakpoints() (Knots::breakpoints / KnotVector::breakpoints), the one home"
    exit 1
  fi
  gate_ok "no spline breakpoints collected by hand outside Knots::breakpoints ($GATE_SCAN_FILES files)"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/${HOME_FILE%/*}"
  printf 'pub fn breakpoints(k: &K) -> impl Iterator<Item = f64> + %s_ { k.runs().map(|(v, _)| v) }\n' "'" > "$1/$HOME_FILE"
  printf 'pub fn breakpoints(k: &K) -> impl Iterator<Item = f64> + %s_ { k.knot_runs().map(|(k, _)| k) }\n' "'" > "$1/$TWIN_FILE"
}

# An entry's planted call on a runs stream.
plant_adapter() {
  local call=${1#*~~}
  printf 'pub fn b(k: &K) -> R { k.runs().%s }\n' "$call" >> "$2/crates/clean/src/lib.rs"
}

# A copy by FORM of SOURCE, deduplicated by DEDUP two statements on.
plant_copy() {
  local expr
  expr=${1##*~~}; expr=${expr//@SRC@/kv.$2()}
  printf 'pub fn b(kv: &KV) -> Vec<f64> {\n    let mut b: Vec<f64> = %s;\n    b.sort_by(f64::total_cmp);\n    b.%s(|a, b| a == b);\n    b\n}\n' \
    "$expr" "$3" >> "$4/crates/clean/src/lib.rs"
}

plant_runs() {
  printf 'pub fn b(k: &K) -> Vec<f64> { k.runs().map(|(v, _)| v).collect() }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_knot_runs() {
  printf 'pub fn b(k: &K) -> Vec<f64> { k.knot_runs().map(|(x, _)| x).collect() }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_borrowed() {
  printf 'pub fn b(k: &K) -> Vec<f64> { k.runs().map(|&(v, _)| v).collect() }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_channel() {
  printf 'pub fn b(k: &K) -> Vec<f64> { k.runs().map(|(v, _)| v.value_channel()).collect() }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_wrapped() {
  printf 'pub fn b(k: &K) -> Vec<f64> {\n    k.runs()\n        .map(|(value, _)| value)\n        .collect()\n}\n' >> "$1/crates/clean/src/lib.rs"
}

plant_named_ignore() {
  printf 'pub fn b(k: &K) -> Vec<f64> { k.runs().map(|(v, _m)| v).collect() }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_rest() {
  printf 'pub fn b(k: &K) -> Vec<f64> { k.runs().map(|(v, ..)| v).collect() }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_typed() {
  printf 'pub fn b(k: &K) -> Vec<f64> { k.runs().map(|(v, _): (f64, usize)| v).collect() }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_called() {
  printf 'pub fn b(k: &K) -> Vec<P> { k.runs().map(|(t, _)| eval(t)).collect() }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_block() {
  printf 'pub fn b(k: &K) -> Vec<f64> { k.runs().map(|(t, _)| { t }).collect() }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_ufcs() {
  printf 'pub fn b(k: &K) -> Vec<f64> { Knots::runs(k).map(|(t, _)| t).collect() }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_for_loop() {
  printf 'pub fn b(k: &K) { for (t, _) in k.knot_runs() { use_it(t); } }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_unzip() {
  printf 'pub fn b(k: &K) -> Vec<f64> { let (t, _): (Vec<f64>, Vec<usize>) = k.runs().unzip(); t }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_filter_then_map() {
  printf 'pub fn b(k: &K) -> Vec<f64> { k.runs().filter(|(v, _)| *v > 0.0).map(|(v, _)| v).collect() }\n' >> "$1/crates/clean/src/lib.rs"
}

# A multiplicity read, the other element, each passed adapter, a copy
# never deduplicated, a copy at one file's end and a dedup at the next
# file's start, prose, a test module, a fixture file and the homes
# themselves are not the shape.
plant_not_sites() {
  local e
  {
    printf 'pub fn a(k: &K) -> Vec<usize> { k.runs().map(|(_, m)| m).collect() }\n'
    printf 'pub fn e(k: &K) -> usize { k.runs().map(|(v, m)| m).sum() }\n'
    printf 'pub fn c(k: &K) -> Vec<f64> { k.runs().map(|(v, m)| v * m).collect() }\n'
    printf 'pub fn g(k: &K, x: f64) -> Vec<usize> { k.runs().filter(|(v, _)| *v > x).map(|(_, m)| m).collect() }\n'
    for e in "${PASSED_ADAPTERS[@]}"; do
      printf 'pub fn p(k: &K) -> R { k.runs().%s }\n' "${e#*~~}"
    done
    printf 'pub fn l(kv: &KV) -> Vec<f64> { let b = kv.values().to_vec(); b }\n'
    printf '// k.runs().map(|(v, _)| v) by hand\npub const S: &str = "k.runs().map(|(v, _)| v)";\n'
    printf '#[cfg(test)]\nmod tests {\n    fn u(k: &K) -> Vec<f64> { k.runs().map(|(v, _)| v).collect() }\n}\n'
    # Mounted, so the reader scans them as production sources.
    printf 'mod a_copy;\nmod b_dedup;\nmod test_support;\nmod walk_fuzz;\n'
  } >> "$1/crates/clean/src/lib.rs"
  # Read in this order: a copy closing one file and a dedup as the next
  # file's first record (text, as every fixture here is). The window is a
  # file's own.
  printf 'pub fn last(kv: &KV) -> Vec<f64> {\n    kv.knots().to_vec()\n}\n' > "$1/crates/clean/src/a_copy.rs"
  printf 'b.dedup();\n' > "$1/crates/clean/src/b_dedup.rs"
  printf 'pub fn u(k: &K) -> Vec<f64> { k.runs().map(|(v, _)| v).collect() }\n' > "$1/crates/clean/src/test_support.rs"
  printf 'pub fn u(k: &K) -> Vec<f64> { k.runs().map(|(v, _)| v).collect() }\n' > "$1/crates/clean/src/walk_fuzz.rs"
}

gate_selftest() {
  local want="a spline's breakpoints collected by hand" e src d
  gate_selftest_clean
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case "$want" plant_runs
  gate_selftest_case "$want" plant_knot_runs
  gate_selftest_case "$want" plant_borrowed
  gate_selftest_case "$want" plant_channel
  gate_selftest_case "$want" plant_wrapped
  gate_selftest_case "$want" plant_named_ignore
  gate_selftest_case "$want" plant_rest
  gate_selftest_case "$want" plant_typed
  gate_selftest_case "$want" plant_called
  gate_selftest_case "$want" plant_block
  gate_selftest_case "$want" plant_ufcs
  gate_selftest_case "$want" plant_for_loop
  gate_selftest_case "$want" plant_unzip
  gate_selftest_case "$want" plant_filter_then_map
  for e in "${REFUSED_ADAPTERS[@]}"; do
    gate_selftest_case "$want" plant_adapter "$e"
  done
  # Every form from every source, and every dedup, each once.
  for e in "${COPY_FORMS[@]}"; do
    for src in "${COPY_SOURCES[@]}"; do
      gate_selftest_case "$want" plant_copy "$e" "$src" dedup
    done
  done
  for d in "${DEDUPS[@]}"; do
    gate_selftest_case "$want" plant_copy "${COPY_FORMS[0]}" knots "$d"
  done
  gate_selftest_passes "a multiplicity read, every passed adapter, other elements, a copy never deduplicated, a copy and a dedup in two files, prose, a test module, fixture files and the homes" plant_not_sites
  gate_selftest_homes --narrowed --subject "$HOME_SUBJECT" "$HOME_FILE" "$TWIN_FILE"
  printf '%s selftest OK: fires on runs() and knot_runs() dropping the multiplicity (bare, borrowed, through value_channel, wrapped, a named or rest ignore, typed, a call or block body, UFCS, after a filter), under each refused adapter (%s), in a for loop and an unzip, and on every copy form (%s) of every source (%s) followed by each dedup (%s); passes each passed adapter (%s), a multiplicity read, a copy never deduplicated, a copy and a dedup in two files, prose, a test module, fixture files and both homes; and stays RED when grep cannot run\n' \
    "$(gate_name)" "$(names_of "${REFUSED_ADAPTERS[@]}" | joined)" \
    "$(names_of "${COPY_FORMS[@]}" | joined)" "$(printf '%s\n' "${COPY_SOURCES[@]}" | joined)" \
    "$(printf '%s\n' "${DEDUPS[@]}" | joined)" "$(names_of "${PASSED_ADAPTERS[@]}" | joined)"
}

gate_parse_args "$@"
gate_main
