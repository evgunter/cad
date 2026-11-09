#!/usr/bin/env bash
# bracket-mid-by-hand.sh — the middle of a bracket `(lo, hi)` is read
# through its home, never spelled by hand. ci.yml's `lint` job runs
# every gate in this directory.
#
# THE RULE (`work/onecopy/inline-bracket-midpoints-are-spelled-in-two-forms.md`).
# A bracket's middle has two spellings, each with its home:
#   * `½(lo + hi)` is `geom_core::mid_param` (`crates/geom-core/src/param.rs`,
#     re-exported as `geom::mid_param`), at
#     every `T: Real`; at `Interval` each end enters once, so its
#     enclosure is tight.
#   * `lo + ½(hi − lo)` is `geom::bracket_mid`
#     (`crates/geom/src/projection_policy.rs`), the `f64` tuple door.
# A hand spelling of either form is refused and named with the home of
# ITS form, so moving a site onto its home changes no bit. The two homes'
# own files are exempt.
#
# WHAT IS REFUSED, in production code under `crates/*/src` (comments and
# string literals out, `#[cfg(test)]` modules and test-only mounts out,
# and files named `test_support*`, `*_rows.rs` or `*_fuzz.rs` out, as
# `atan2-sites.sh` reads them), whitespace between tokens allowed and
# rustfmt's wrap across lines read as one statement:
#   * `E + 0.5 * (F - E)`, `E + (F - E) * 0.5` and `E + (F - E) / 2.0`,
#     E the same name, field path or call chain twice (`lo`, `x.lo()`,
#     `self.b.0`), whatever the names, and not followed by a further
#     `*` or `/` (`lo + 0.5 * (hi - lo) * i` is a grid, not a middle);
#   * `0.5 * (P)`, `(P) * 0.5` and `(P) / 2.0`, and the same halved by
#     `T::from_f64(0.5)` on either side, not behind a `*` or `/` (a
#     halved product, `l * (m0 + m1) * 0.5`, is a different term), where
#     the pair P is `L + H` in either order, L's last name ending in `lo`
#     and H's in `hi`, either indexed (`lo`, `z_lo`, `x.lo()`,
#     `self.lo`, `lo[k]`); or `E.0 + E.1` or `E[0] + E[1]`, E the same
#     chain twice (`span.0 + span.1`, `w[0] + w[1]`); or `S0 + S1`, S
#     the same stem twice (`t0 + t1`, `e.t0 + e.t1`).
#
# BLIND SPOTS, each one a shape that passes:
#   * `½(a + b)` under OTHER NAMES (`a`/`b`, `s`/`t`, `low`/`high`,
#     `va`/`vb`, `w[a]`/`w[b]`): those are as often the middle of two
#     points, two directions or two readings as of a bracket. The
#     second pass,
#     `grep -rnE '0\.5 \* \([^()]+ \+ [^()]+\)|\([^()]+ \+ [^()]+\) (\* 0\.5|/ 2\.0)' crates/*/src`,
#     lists them for a reader to sort by hand.
#   * `0.5 * lo + 0.5 * hi` and `lo * 0.5 + hi * 0.5`, which differ
#     from `mid_param` only where a half is subnormal.
#   * `f64::midpoint` (`lo.midpoint(hi)`), std's correctly rounded form:
#     the same bits as `mid_param` wherever both ends lie within
#     `f64::MAX / 2`, and finite past it.
#   * A PARENTHESISED OR INDEXED OPERAND in the first form
#     (`(a - b) + 0.5 * (c - (a - b))`, `x[0] + 0.5 * (x[1] - x[0])`),
#     since E is a chain of names, `()` calls and fields.
#   * A middle taken through a local helper, a closure or a macro.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

HOMES=(crates/geom-core/src/param.rs crates/geom/src/projection_policy.rs)
HOME_SUBJECT='the homes of a bracket'"'"'s middle, geom::mid_param and geom::bracket_mid'

gate() {
  gate_require_crate_sources
  gate_production_sources
  gate_require_homes "$HOME_SUBJECT" "${HOMES[@]}"
  local files=() f
  for f in "${GATE_PRODUCTION_FILES[@]}"; do
    case "${f##*/}" in
      test_support*|*_rows.rs|*_fuzz.rs) ;;
      *) files+=("$f") ;;
    esac
  done
  GATE_SCAN_FILES=${#files[@]}
  local s='[[:space:]]*'
  local chain='[A-Za-z_][A-Za-z0-9_]*(\(\))?(\.[A-Za-z0-9_]+(\(\))?)*'
  local half="(0\\.5$s\\*$s\\(${s}${chain}$s-$s\\2$s\\)|\\(${s}${chain}$s-$s\\2$s\\)$s(\\*${s}0\\.5|/${s}2\\.0?))"
  local lower="(^|[^A-Za-z0-9_.*/+[:space:]-]|return)$s($chain)$s\\+$s$half$s([^*/[:space:]]|\$)"
  # THE SUM FORM: a pair of ends, halved. A backreference's number is
  # global to its regex, so each shape is its own grep, and `@` in a
  # shape stands for the group its first end lands in: 3 behind a
  # leading halving (boundary, halving, end), 2 before a trailing one.
  local idx='(\[[A-Za-z0-9_]+\])?'
  local lo="[A-Za-z0-9_.]*lo(\(\))?$idx" hi="[A-Za-z0-9_.]*hi(\(\))?$idx"
  local stem='[A-Za-z_][A-Za-z0-9_]*(\.[A-Za-z_][A-Za-z0-9_]*)*'
  local -a pairs=(
    "($lo$s\\+$s$hi|$hi$s\\+$s$lo)"
    "($chain)\\.0$s\\+$s\\@\\.1"
    "($chain)\\[0\\]$s\\+$s\\@\\[1\\]"
    "($stem)0$s\\+$s\\@1"
  )
  local by_half="(0\\.5|T::from_f64\\(${s}0\\.5$s\\))"
  local edge='(^|[^A-Za-z0-9_.*/[:space:]]|return)[[:space:]]*'
  local view lower_hits sum_hits='' pair re
  view=$(gate_rust_code --skip-cfg-test --statements "${files[@]}" \
    | gate_grep -vE "$(gate_record_anchor_any "${HOMES[@]}")")
  lower_hits=$(printf '%s\n' "$view" | gate_grep -E "$lower")
  for pair in "${pairs[@]}"; do
    for re in "$edge$by_half$s\\*$s\\($s${pair//@/3}$s\\)" \
      "$edge\\($s${pair//@/2}$s\\)$s(\\*$s$by_half|/${s}2\\.0?)"; do
      sum_hits+=$(printf '%s\n' "$view" | gate_grep -E "$re")$'\n'
    done
  done
  sum_hits=$(printf '%s' "$sum_hits" | sort -u | sed '/^$/d')
  if [ -n "$sum_hits" ]; then
    printf '%s\n' "$sum_hits"
    gate_error "a bracket's middle spelled ½(lo + hi) by hand (above): read it as geom::mid_param(lo, hi), its home"
  fi
  if [ -n "$lower_hits" ]; then
    printf '%s\n' "$lower_hits"
    gate_error "a bracket's middle spelled lo + ½(hi − lo) by hand (above): read it as geom::bracket_mid((lo, hi)), its home"
  fi
  if [ -n "$sum_hits$lower_hits" ]; then
    exit 1
  fi
  gate_ok "no bracket's middle spelled by hand outside geom::mid_param and geom::bracket_mid ($GATE_SCAN_FILES files)"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/crates/geom/src" "$1/crates/geom-core/src"
  printf 'pub fn mid_param(lo: f64, hi: f64) -> f64 { 0.5 * (lo + hi) }\n' > "$1/${HOMES[0]}"
  printf 'pub fn bracket_mid((lo, hi): (f64, f64)) -> f64 { lo + 0.5 * (hi - lo) }\n' > "$1/${HOMES[1]}"
}

plant() {
  printf '%s\n' "$2" >> "$1/crates/clean/src/lib.rs"
}

plant_lower_half() { plant "$1" 'pub fn m(lo: f64, hi: f64) -> f64 { lo + 0.5 * (hi - lo) }'; }
plant_lower_trailing() { plant "$1" 'pub fn m(a: f64, b: f64) -> f64 { a + (b - a) * 0.5 }'; }
plant_lower_halved() { plant "$1" 'pub fn m(x: I) -> f64 { x.lo() + (x.hi() - x.lo()) / 2.0 }'; }
plant_lower_return() { plant "$1" 'pub fn m(lo: f64, hi: f64) -> f64 { return lo + 0.5 * (hi - lo); }'; }
plant_lower_wrapped() { plant "$1" $'pub fn m(lo: f64, hi: f64) -> f64 {\n    lo\n        + (hi - lo) * 0.5\n}'; }
plant_sum_half() { plant "$1" 'pub fn m(lo: f64, hi: f64) -> f64 { 0.5 * (lo + hi) }'; }
plant_sum_trailing() { plant "$1" 'pub fn m(x: I) -> f64 { (x.lo() + x.hi()) * 0.5 }'; }
plant_sum_halved() { plant "$1" 'pub fn m(z_lo: f64, z_hi: f64) -> f64 { (z_lo + z_hi) / 2.0 }'; }
plant_sum_swapped() { plant "$1" 'pub fn m(s: S) -> f64 { 0.5 * (s.hi + s.lo) }'; }
plant_sum_indexed() { plant "$1" 'pub fn m(lo: [f64; 3], hi: [f64; 3]) -> f64 { 0.5 * (lo[1] + hi[1]) }'; }
plant_sum_tuple() { plant "$1" 'pub fn m(c: C) -> f64 { 0.5 * (c.span.0 + c.span.1) }'; }
plant_sum_pair() { plant "$1" 'pub fn m(w: &[f64]) -> f64 { (w[0] + w[1]) * 0.5 }'; }
plant_sum_stem() { plant "$1" 'pub fn m(e: E) -> f64 { (e.t0 + e.t1) / 2.0 }'; }
plant_sum_scalar() { plant "$1" 'pub fn m<T: Real>(t0: T, t1: T) -> T { (t0 + t1) * T::from_f64(0.5) }'; }
plant_sum_scalar_leading() { plant "$1" 'pub fn m<T: Real>(d0: T, d1: T) -> T { T::from_f64(0.5) * (d0 + d1) }'; }

# A grid step, an integer middle, other names, a sum that is not halved,
# a halved product, ends of two different pairs,
# prose, a test module, fixture files and the home are not the shape.
plant_not_sites() {
  {
    printf 'pub fn a(lo: f64, hi: f64, i: f64) -> f64 { lo + 0.5 * (hi - lo) * i }\n'
    printf 'pub fn b(lo: usize, hi: usize) -> usize { (lo + hi) / 2 }\n'
    printf 'pub fn c(a: f64, b: f64) -> f64 { 0.5 * (a + b) }\n'
    printf 'pub fn d(lo: f64, hi: f64) -> f64 { lo + hi }\n'
    printf 'pub fn e(a: f64, lo: f64, hi: f64) -> f64 { a * lo + 0.5 * (hi - lo) }\n'
    printf 'pub fn f(lo: f64, hi: f64, c: f64) -> f64 { lo + 0.5 * (hi - c) }\n'
    printf 'pub fn g(l: f64, m0: f64, m1: f64) -> f64 { l * (m0 + m1) * 0.5 }\n'
    printf 'pub fn h(w: &[f64], a: usize, b: usize) -> f64 { 0.5 * (w[a] + w[b]) }\n'
    printf 'pub fn i(s: (f64, f64), r: (f64, f64)) -> f64 { 0.5 * (s.0 + r.1) + 0.5 * (t0 + u1) }\n'
    printf '// 0.5 * (lo + hi) by hand\npub const S: &str = "lo + 0.5 * (hi - lo)";\n'
    printf '#[cfg(test)]\nmod tests {\n    fn m(lo: f64, hi: f64) -> f64 { 0.5 * (lo + hi) }\n}\n'
  } >> "$1/crates/clean/src/lib.rs"
  printf 'pub fn m(lo: f64, hi: f64) -> f64 { 0.5 * (lo + hi) }\n' > "$1/crates/clean/src/test_support.rs"
  printf 'pub fn m(lo: f64, hi: f64) -> f64 { 0.5 * (lo + hi) }\n' > "$1/crates/clean/src/walk_rows.rs"
}

gate_selftest() {
  gate_selftest_clean
  gate_selftest_without_tool grep "it is grep saying it could not search"
  local p
  for p in plant_lower_half plant_lower_trailing plant_lower_halved plant_lower_return plant_lower_wrapped; do
    gate_selftest_case "read it as geom::bracket_mid((lo, hi))" "$p"
  done
  for p in plant_sum_half plant_sum_trailing plant_sum_halved plant_sum_swapped plant_sum_indexed \
    plant_sum_tuple plant_sum_pair plant_sum_stem plant_sum_scalar plant_sum_scalar_leading; do
    gate_selftest_case "read it as geom::mid_param(lo, hi)" "$p"
  done
  gate_selftest_passes "a grid step, an integer middle, other names, an unhalved sum, another operand, a halved product, ends of two different pairs, prose, a test module, fixture files and the home" plant_not_sites
  gate_selftest_homes --narrowed --subject "$HOME_SUBJECT" "${HOMES[@]}"
  printf '%s selftest OK: passes a clean fixture carrying both homes, and a grid step, an integer middle, other names, an unhalved sum, another operand, a halved product, ends of two different pairs, prose, a test module and fixture files; fires naming geom::bracket_mid on lo + ½(hi − lo) as 0.5 * (…), (…) * 0.5 and (…) / 2.0, after return and wrapped across lines, and naming geom::mid_param on ½(lo + hi) as 0.5 * (…), (…) * 0.5 and (…) / 2.0 in either order, indexed, as E.0 + E.1, E[0] + E[1] and t0 + t1, and halved by T::from_f64(0.5) on either side; and stays RED when grep cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
