#!/usr/bin/env bash
# bracket-mid-by-hand.sh — the middle of a bracket `(lo, hi)` is read
# through its home, never spelled by hand. ci.yml's `lint` job runs
# every gate in this directory.
#
# THE RULE (`work/onecopy/inline-bracket-midpoints-are-spelled-in-two-forms.md`).
# A bracket's middle has two spellings, both homed in
# `crates/geom-core/src/param.rs` (re-exported by `geom` under the same
# names), each at every `T: Real` and each with its reason stated there:
#   * `½(lo + hi)` is `mid_param(lo, hi)`: at `Interval` each end enters
#     once, so its enclosure is tight.
#   * `lo + ½(hi − lo)` is `bracket_mid((lo, hi))`: at an infinite point
#     it is NaN where `½(lo + hi)` is `∞`, which projection's poison exit
#     reads. A point pair's middle is `p.mid(q)` (`Point2`/`Point3`),
#     `bracket_mid` per coordinate.
# A hand spelling of either form is refused and named with the home of
# ITS form, so moving a site onto its home changes no `f64` or
# `Interval` bit. A `Sym` form whose halving was a division by 2 becomes
# the home's multiplication by ½: the same value, a different form (its
# digest, and which denominators `Form::add` shares). The home's own file
# is exempt.
#
# WHAT IS REFUSED, in production code under `crates/*/src` (comments and
# string literals out, `#[cfg(test)]` modules and test-only mounts out,
# and files named `test_support*`, `*_rows.rs` or `*_fuzz.rs` out, as
# `atan2-sites.sh` reads them), whitespace between tokens allowed and
# rustfmt's wrap across lines read as one statement. HALF below is
# `0.5` or `T::from_f64(0.5)`, on either side of its product, or a
# division by `2.0` or `T::from_f64(2.0)`; a halving behind a `*` or `/`,
# or followed by one, is a different term (a grid step
# `lo + 0.5 * (hi - lo) * i`, a halved product `l * (m0 + m1) * 0.5`, a
# trapezoid `(f[0] + f[1]) * 0.5 * h`) and is not read.
#   * `E + HALF(F - E)`, E the same name, field path or call chain twice
#     (`lo`, `x.lo()`, `self.b.0`), whatever the names;
#   * `HALF(P)`, where the pair P is, in either order: `Xlo + Xhi`, X the
#     same prefix twice, with the same call parens and index
#     (`lo + hi`, `z_lo + z_hi`, `x.lo() + x.hi()`, `lo[k] + hi[k]`);
#     `E.0 + E.1` or `E[0] + E[1]`, E the same chain twice; or
#     `S0 + S1`, S the same stem twice (`t0 + t1`, `e.t0 + e.t1`);
#   * `p.lerp(q, ½)`, the half `0.5` or `T::from_f64(0.5)`: `lerp`'s body
#     at `t = ½` is `p.mid(q)`'s arithmetic, and `lerp` stays the door for
#     a general `t`. The half is read as `lerp`'s last argument, so
#     `.lerp(a, f(x, 0.5))` is refused too (a false positive; none is in
#     the tree).
#
# BLIND SPOTS, each one a shape that passes:
#   * `½(a + b)` under OTHER NAMES (`a`/`b`, `s`/`t`, `low`/`high`,
#     `va`/`vb`, `w[a]`/`w[b]`): those are as often the middle of two
#     points, two directions or two readings as of a bracket. The
#     second pass,
#     `grep -rnE '0\.5 \* \([^()]+ \+ [^()]+\)|\([^()]+ \+ [^()]+\) (\* 0\.5|/ 2\.0)' crates/*/src`,
#     lists them for a reader to sort by hand.
#   * `0.5 * lo + 0.5 * hi` and `lo * 0.5 + hi * 0.5`, on one line or
#     across lines, which differ from `mid_param` only where a half is
#     subnormal, and `f64::midpoint`, std's correctly rounded form, the
#     same bits as `mid_param` wherever both ends lie within
#     `f64::MAX / 2`. Both are filed:
#     `work/onecopy/a-bracket-middle-is-also-spelled-half-lo-plus-half-hi-and-f64-midpoint.md`.
#   * `hi - HALF(hi - lo)`, the middle read from the upper end.
#   * A width or a half bound by `let` first (`let w = hi - lo; … lo +
#     0.5 * w`), and a half held under a name (`HALF`, `let h = 0.5`).
#   * A PARENTHESISED OR INDEXED OPERAND in the first form
#     (`(a - b) + 0.5 * (c - (a - b))`, `x[0] + 0.5 * (x[1] - x[0])`),
#     since E is a chain of names, `()` calls and fields.
#   * A middle taken through a local helper, a closure or a macro.
#
# FALSE POSITIVES it can raise: the stem rule reads ANY pair `S0 + S1`
# halved as a bracket's ends, so two coefficients `c0 + c1` halved would
# be refused; none is in the tree, and one that appears is renamed or
# routed, whichever it is.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

HOMES=(crates/geom-core/src/param.rs)
HOME_SUBJECT='the home of a bracket'"'"'s middle, mid_param and bracket_mid'

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
  local times="(0\\.5|T::from_f64\\($s(0\\.5)$s\\))"
  local by_half="(\\*$s$times|/$s(2\\.0?|T::from_f64\\($s(2\\.0?)$s\\)))"
  local edge='(^|[^A-Za-z0-9_.*/[:space:]]|return)[[:space:]]*'
  local tail="$s([^*/[:space:]]|\$)"
  # THE DIFFERENCE FORM. Group 1 is the edge and group 2 is E.
  local diff="\\($s$chain$s-$s\\2$s\\)"
  local lower="$edge($chain)$s\\+$s($times$s\\*$s$diff|$diff$s$by_half)$tail"
  # THE SUM FORM: a pair of ends, halved. A backreference's number is
  # global to its regex, so each shape is its own grep. In a shape, @K
  # stands for the group its first end lands in (4 behind a leading
  # halving: edge, halving, its inner 0.5; 2 before a trailing one), @L
  # for K + 1 and @M for K + 3.
  local pfx='[A-Za-z0-9_.]*' call='((\(\))?)' idx='((\[[A-Za-z0-9_]+\])?)'
  local stem='[A-Za-z_][A-Za-z0-9_]*(\.[A-Za-z_][A-Za-z0-9_]*)*'
  local -a pairs=(
    "($pfx)lo$call$idx$s\\+$s\\@Khi\\@L\\@M"
    "($pfx)hi$call$idx$s\\+$s\\@Klo\\@L\\@M"
    "($chain)\\.0$s\\+$s\\@K\\.1"
    "($chain)\\.1$s\\+$s\\@K\\.0"
    "($chain)\\[0\\]$s\\+$s\\@K\\[1\\]"
    "($chain)\\[1\\]$s\\+$s\\@K\\[0\\]"
    "($stem)0$s\\+$s\\@K1"
    "($stem)1$s\\+$s\\@K0"
  )
  local lerp_re="\\.lerp$s\\(.*,$s$times$s\\)"
  local view lower_hits lerp_hits sum_hits='' pair re k
  view=$(gate_rust_code --skip-cfg-test --statements "${files[@]}" \
    | gate_grep -vE "$(gate_record_anchor_any "${HOMES[@]}")")
  lower_hits=$(printf '%s\n' "$view" | gate_grep -E "$lower")
  lerp_hits=$(printf '%s\n' "$view" | gate_grep -E "$lerp_re")
  for pair in "${pairs[@]}"; do
    for k in 4 2; do
      re=${pair//@K/$k}
      re=${re//@L/$((k + 1))}
      re=${re//@M/$((k + 3))}
      if [ "$k" = 4 ]; then
        re="$edge$times$s\\*$s\\($s$re$s\\)$tail"
      else
        re="$edge\\($s$re$s\\)$s$by_half$tail"
      fi
      sum_hits+=$(printf '%s\n' "$view" | gate_grep -E "$re")$'\n'
    done
  done
  sum_hits=$(printf '%s' "$sum_hits" | sort -u | sed '/^$/d')
  if [ -n "$sum_hits" ]; then
    printf '%s\n' "$sum_hits"
    gate_error "a bracket's middle spelled ½(lo + hi) by hand (above): read it as mid_param(lo, hi) (geom_core::mid_param, re-exported as geom::mid_param), its home"
  fi
  if [ -n "$lower_hits" ]; then
    printf '%s\n' "$lower_hits"
    gate_error "a bracket's middle spelled lo + ½(hi − lo) by hand (above): read it as bracket_mid((lo, hi)) (geom_core::bracket_mid, re-exported as geom::bracket_mid), or p.mid(q) for a pair of points, its home"
  fi
  if [ -n "$lerp_hits" ]; then
    printf '%s\n' "$lerp_hits"
    gate_error "a point pair's middle spelled p.lerp(q, ½) by hand (above): read it as p.mid(q) (Point2::mid / Point3::mid), its home"
  fi
  if [ -n "$sum_hits$lower_hits$lerp_hits" ]; then
    exit 1
  fi
  gate_ok "no bracket's middle spelled by hand outside mid_param and bracket_mid ($GATE_SCAN_FILES files)"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/crates/geom-core/src"
  printf 'pub fn mid_param(lo: f64, hi: f64) -> f64 { 0.5 * (lo + hi) }\npub fn bracket_mid((lo, hi): (f64, f64)) -> f64 { lo + 0.5 * (hi - lo) }\n' > "$1/${HOMES[0]}"
}

plant() {
  printf '%s\n' "$2" >> "$1/crates/clean/src/lib.rs"
}

plant_lower_half() { plant "$1" 'pub fn m(lo: f64, hi: f64) -> f64 { lo + 0.5 * (hi - lo) }'; }
plant_lower_trailing() { plant "$1" 'pub fn m(a: f64, b: f64) -> f64 { a + (b - a) * 0.5 }'; }
plant_lower_halved() { plant "$1" 'pub fn m(x: I) -> f64 { x.lo() + (x.hi() - x.lo()) / 2.0 }'; }
plant_lower_return() { plant "$1" 'pub fn m(lo: f64, hi: f64) -> f64 { return lo + 0.5 * (hi - lo); }'; }
plant_lower_wrapped() { plant "$1" $'pub fn m(lo: f64, hi: f64) -> f64 {\n    lo\n        + (hi - lo) * 0.5\n}'; }
plant_lower_scalar() { plant "$1" 'pub fn m<T: Real>(lo: P<T>, hi: P<T>) -> P<T> { lo + (hi - lo) * T::from_f64(0.5) }'; }
plant_lower_scalar_leading() { plant "$1" 'pub fn m<T: Real>(a: T, b: T) -> T { a + T::from_f64(0.5) * (b - a) }'; }
plant_lower_scalar_halved() { plant "$1" 'pub fn m<T: Real>(p: T, q: T) -> T { p + (q - p) / T::from_f64(2.0) }'; }
plant_lerp_half() { plant "$1" 'pub fn m(p: P, q: P) -> P { p.lerp(q, 0.5) }'; }
plant_lerp_scalar() { plant "$1" 'pub fn m<T: Real>(p: P<T>, q: P<T>) -> P<T> { p.lerp(q, T::from_f64(0.5)) }'; }
plant_sum_half() { plant "$1" 'pub fn m(lo: f64, hi: f64) -> f64 { 0.5 * (lo + hi) }'; }
plant_sum_trailing() { plant "$1" 'pub fn m(x: I) -> f64 { (x.lo() + x.hi()) * 0.5 }'; }
plant_sum_halved() { plant "$1" 'pub fn m(z_lo: f64, z_hi: f64) -> f64 { (z_lo + z_hi) / 2.0 }'; }
plant_sum_swapped() { plant "$1" 'pub fn m(s: S) -> f64 { 0.5 * (s.hi + s.lo) }'; }
plant_sum_indexed() { plant "$1" 'pub fn m(lo: [f64; 3], hi: [f64; 3]) -> f64 { 0.5 * (lo[1] + hi[1]) }'; }
plant_sum_tuple() { plant "$1" 'pub fn m(c: C) -> f64 { 0.5 * (c.span.0 + c.span.1) }'; }
plant_sum_tuple_swapped() { plant "$1" 'pub fn m(c: C) -> f64 { 0.5 * (c.span.1 + c.span.0) }'; }
plant_sum_pair() { plant "$1" 'pub fn m(w: &[f64]) -> f64 { (w[0] + w[1]) * 0.5 }'; }
plant_sum_pair_swapped() { plant "$1" 'pub fn m(w: &[f64]) -> f64 { (w[1] + w[0]) * 0.5 }'; }
plant_sum_stem() { plant "$1" 'pub fn m(e: E) -> f64 { (e.t0 + e.t1) / 2.0 }'; }
plant_sum_stem_swapped() { plant "$1" 'pub fn m(t0: f64, t1: f64) -> f64 { 0.5 * (t1 + t0) }'; }
plant_sum_scalar() { plant "$1" 'pub fn m<T: Real>(t0: T, t1: T) -> T { (t0 + t1) * T::from_f64(0.5) }'; }
plant_sum_scalar_leading() { plant "$1" 'pub fn m<T: Real>(d0: T, d1: T) -> T { T::from_f64(0.5) * (d0 + d1) }'; }
plant_sum_scalar_halved() { plant "$1" 'pub fn m<T: Real>(s: S<T>) -> T { (s.lo + s.hi) / T::from_f64(2.0) }'; }

# A grid step, an integer middle, other names, a sum that is not halved,
# another operand, a halved product, a trapezoid, ends of two different
# pairs or brackets, prose, a test module, fixture files and the home
# are not the shape.
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
    printf 'pub fn j(a: I, b: I) -> f64 { 0.5 * (a.lo() + b.hi()) + (z_lo + y_hi) * 0.5 }\n'
    printf 'pub fn k(f: [f64; 2], h: f64) -> f64 { (f[0] + f[1]) * 0.5 * h }\n'
    printf 'pub fn l<T: Real>(lo: T, hi: T, h: T) -> T { lo + (hi - lo) * T::from_f64(0.5) * h }\n'
    printf 'pub fn n(p: P, q: P, t: f64) -> P { p.lerp(q, t).lerp(q, 0.25) }\n'
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
  for p in plant_lower_half plant_lower_trailing plant_lower_halved plant_lower_return plant_lower_wrapped \
    plant_lower_scalar plant_lower_scalar_leading plant_lower_scalar_halved; do
    gate_selftest_case "read it as bracket_mid((lo, hi))" "$p"
  done
  for p in plant_sum_half plant_sum_trailing plant_sum_halved plant_sum_swapped plant_sum_indexed \
    plant_sum_tuple plant_sum_tuple_swapped plant_sum_pair plant_sum_pair_swapped plant_sum_stem \
    plant_sum_stem_swapped plant_sum_scalar plant_sum_scalar_leading plant_sum_scalar_halved; do
    gate_selftest_case "read it as mid_param(lo, hi)" "$p"
  done
  for p in plant_lerp_half plant_lerp_scalar; do
    gate_selftest_case "read it as p.mid(q)" "$p"
  done
  gate_selftest_passes "a grid step, an integer middle, other names, an unhalved sum, another operand, a halved product, a trapezoid, ends of two different pairs or brackets, prose, a test module, fixture files and the home" plant_not_sites
  gate_selftest_homes --narrowed --subject "$HOME_SUBJECT" "${HOMES[@]}"
  printf '%s selftest OK: passes a clean fixture carrying the home, and a lerp at a general t, a grid step, an integer middle, other names, an unhalved sum, another operand, a halved product, a trapezoid, ends of two different pairs or brackets, prose, a test module and fixture files; fires naming bracket_mid on lo + ½(hi − lo) with the half as 0.5 or T::from_f64(0.5) on either side or a division by 2.0 or T::from_f64(2.0), after return and wrapped across lines; fires naming p.mid(q) on p.lerp(q, ½) with the half as 0.5 or T::from_f64(0.5); fires naming mid_param on ½(lo + hi) halved the same ways, in either order, as Xlo + Xhi (indexed and called), E.0 + E.1, E[0] + E[1] and S0 + S1; and stays RED when grep cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
