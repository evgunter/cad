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
# ITS form, so moving a site onto its home moves no `f64` or `Sym`
# value; at `Interval` an enclosure can only narrow, by one ulp under the
# 2Prod floor where the halving was a division, and never widens (the
# product by ½ is exact at every normal magnitude). Where the halving
# was a division by 2 its `Sym` form differs (its digest, and which
# denominators `Form::add` shares). The home's own file is exempt.
#
# WHAT IS REFUSED, in production code under `crates/*/src` (comments and
# string literals out, `#[cfg(test)]` modules and test-only mounts out,
# and files named `test_support*`, `*_rows.rs` or `*_fuzz.rs` out, as
# `atan2-sites.sh` reads them), whitespace between tokens allowed and
# rustfmt's wrap across lines read as one statement. HALF below is
# `0.5` (`0.5_f64` too) or any path call on it, a lift
# (`T::from_f64(0.5)`, `Interval::point(0.5)`, a local `pt(0.5)`), on
# either side of its product, or a division by `2.0` or a lift of it
# (a bare `/ 2` is an integer's and is not read), and a number that only
# starts like one (`0.55`, `0.5e-3`, `2.05`) is not; a halving behind a
# `*` or `/`, or followed by one, is a different term (a grid step
# `lo + 0.5 * (hi - lo) * i`, a halved product `l * (m0 + m1) * 0.5`, a
# trapezoid `(f[0] + f[1]) * 0.5 * h`) and is not read.
#   * `E + HALF(F - E)`, E the same name, field path or call chain twice
#     (`lo`, `x.lo()`, `self.b.0`), whatever the names;
#   * `HALF(P)`, where the pair P is, in either order: `Xlo + Xhi`, X the
#     same prefix twice, with the same call parens and index
#     (`lo + hi`, `z_lo + z_hi`, `x.lo() + x.hi()`, `lo[k] + hi[k]`),
#     bare or each inside one lift (`pt(lo) + pt(hi)`);
#     `E.0 + E.1` or `E[0] + E[1]`, E the same chain twice; or
#     `S0 + S1`, S the same stem twice (`t0 + t1`, `e.t0 + e.t1`); each
#     of these bare or each end inside one lift (`pt(t0) + pt(t1)`,
#     `pt(x.0) + pt(x.1)`);
#   * `HALF·A + HALF·B`, the sum of two halves on either side of each
#     product (`0.5 * lo + 0.5 * hi`, `lo * 0.5 + hi * 0.5`,
#     `a / 2.0 + b / 2.0`), where A and B are a PAIR of ends as P is
#     above, bare or each inside one lift (`pt(0.5) * pt(lo) + pt(0.5) *
#     pt(hi)`); two unrelated halves (`c + 0.5 * a + 0.5 * b`, a
#     vector's) are a weighted sum and pass; and `a.midpoint(b)` or
#     `f64::midpoint(a, b)`, std's form. Both read as `mid_param`; why
#     neither is a second home is `mid_param`'s doc
#     (`crates/geom-core/src/param.rs`), and the ruling is
#     `work/onecopy/a-bracket-middle-is-also-spelled-half-lo-plus-half-hi-and-f64-midpoint.md`'s
#     PR 6681. A zero-argument `.midpoint()` (a type's own door,
#     `BoxAxis::midpoint`) is not read;
#   * `p.lerp(q, ½)`, the half `0.5` or `T::from_f64(0.5)`: `lerp`'s body
#     at `t = ½` is `p.mid(q)`'s arithmetic, and `lerp` stays the door for
#     a general `t`. The half is read as `lerp`'s last argument, so
#     `.lerp(a, f(x, 0.5))` is refused too (a false positive; none is in
#     the tree).
#
# BLIND SPOTS, each one a shape that passes:
#   * `½(a + b)` under OTHER NAMES (`a`/`b`, `s`/`t`, `low`/`high`,
#     `va`/`vb`, `w[a]`/`w[b]`, `pi`/`qi`): those are as often the
#     middle of two points, two directions or two readings as of a
#     bracket. The second pass lists them, the half and the operands
#     read as the first pass reads them (a lift on either, one level of
#     call parens in an operand), for a reader to sort by hand:
#     H='(0\.5|[A-Za-z_][A-Za-z0-9_:]*\( *0\.5 *\))'; O='([^()]|\([^()]*\))+'
#     grep -rnE "$H \* \($O \+ $O\)|\($O \+ $O\) (\* $H|/ (2\.0|[A-Za-z_][A-Za-z0-9_:]*\( *2\.0 *\)))" crates/*/src
#   * A lifted pair whose two ends sit under two DIFFERENT lifts, or a
#     lift that takes more than the end (`Interval::new(lo, lo)`).
#   * The sum of two halves with a parenthesised operand
#     (`0.5 * (a - b) + 0.5 * c`), or behind a `*`/`/`; a division by
#     `2_f64` written without its point; and `midpoint` reached through a
#     path other than `f64::` or a method call (`<f64>::midpoint`, a
#     `use`d alias).
#   * `hi - HALF(hi - lo)`, the middle read from the upper end.
#   * A width or a half bound by `let` first (`let w = hi - lo; … lo +
#     0.5 * w`), and a half held under a name (`HALF`, `let h = 0.5`),
#     `p.lerp(q, half)` with the half in a variable included.
#   * A PARENTHESISED OR INDEXED OPERAND in the first form
#     (`(a - b) + 0.5 * (c - (a - b))`, `x[0] + 0.5 * (x[1] - x[0])`),
#     since E is a chain of names, `()` calls and fields.
#   * A middle taken through a local helper, a closure or a macro.
#
# FALSE POSITIVES it can raise, none in the tree; one that appears is
# renamed or routed, whichever it is:
#   * the stem rule reads ANY pair `S0 + S1` halved as a bracket's ends,
#     so two coefficients `c0 + c1` halved would be refused;
#   * a HALF is any path call on `0.5`, so `scale(0.5) * (lo + hi)` is
#     read as a halving;
#   * the method arm cannot type its receiver, so an integer's
#     overflow-safe `lo.midpoint(hi)` is refused too: spell it
#     `usize::midpoint(lo, hi)`, which the path arm (`f64::` only) passes.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

HOMES=(crates/geom-core/src/param.rs)
HOME_SUBJECT='the home of a bracket'"'"'s middle, mid_param and bracket_mid'

# THE SUM OF HALVES, `HALF·A + HALF·B` (either side of each product, or
# `A/2 + B/2`), read on the statement view, fires only where A and B are
# a PAIR of ends in the sum form's sense: `Xlo`/`Xhi` with one prefix,
# call and index, `E.0`/`E.1`, `E[0]`/`E[1]` or `S0`/`S1`, either order.
# Two unrelated halves (`c + 0.5 * a + 0.5 * b`, a vector's halves) are a
# weighted sum, not a middle.
# shellcheck disable=SC2016
HALVES_MATCHER='
import re, sys
LIFT = r"[A-Za-z_][A-Za-z0-9_:]*"
NUM = r"(?![0-9A-Za-z_])"
HALF = r"(?:0\.5(?:_?f64)?" + NUM + r"|" + LIFT + r"\(\s*0\.5(?:_?f64)?\s*\))"
DIV = r"(?:2\.0?(?:_?f64)?" + NUM + r"|" + LIFT + r"\(\s*2\.0?\s*\))"
CH = r"[A-Za-z_][A-Za-z0-9_]*(?:\(\))?(?:\.[A-Za-z0-9_]+(?:\(\))?|\[[A-Za-z0-9_]+\])*"
# An operand is a chain, bare or inside one lift (`pt(lo)`): the lift is
# captured beside it, and a pair pairs only under one lift.
def op(n):
    return r"(?:(?P<" + n + r"l>" + LIFT + r")\(\s*)?(?P<" + n + r">" + CH + r")(?(" + n + r"l)\s*\))"
def half(n):
    return (r"(?:" + HALF + r"\s*\*\s*" + op(n + "a") + r"|" + op(n + "b")
            + r"\s*(?:\*\s*" + HALF + r"|/\s*" + DIV + r"))")
SUM = re.compile(r"(?:^|[^A-Za-z0-9_.*/\s]|return)\s*" + half("x") + r"\s*\+\s*" + half("y")
                 + r"(?=\s*(?:[^*/\s]|$))")
END = re.compile(r"^(.*?)(lo|hi)((?:\(\))?(?:\[[A-Za-z0-9_]+\])?)$")
def paired(a, b):
    for x, y in ((a, b), (b, a)):
        m, n = END.match(x), END.match(y)
        if m and n and m.group(2) == "lo" and n.group(2) == "hi" and (m.group(1), m.group(3)) == (n.group(1), n.group(3)):
            return True
        for one, two in ((".0", ".1"), ("[0]", "[1]"), ("0", "1")):
            if x.endswith(one) and y.endswith(two) and x[: -len(one)] == y[: -len(two)] and x[: -len(one)]:
                return True
    return False
for record in sys.stdin:
    text = record.rstrip("\n")
    body = text.split(":", 2)[-1]
    for m in SUM.finditer(body):
        xa, ya = ("xa" if m.group("xa") else "xb"), ("ya" if m.group("ya") else "yb")
        if m.group(xa + "l") != m.group(ya + "l"):
            continue
        if paired(m.group(xa), m.group(ya)):
            print(text)
            break
'

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
  # A HALF is `0.5` (`0.5_f64`, `0.5f64` too) or any path call on it, a
  # lift (`T::from_f64(0.5)`, `Interval::point(0.5)`, a local `pt(0.5)`);
  # a division is by `2.` or `2.0` or a lift of it (a bare `/ 2` is an
  # integer's: a float divides only by a float). ONE group each, so the
  # backreference numbers below stay fixed.
  local lift='[A-Za-z_][A-Za-z0-9_:]*'
  local times="(0\\.5[_f64]{0,4}|$lift\\(${s}0\\.5[_f64]{0,4}$s\\))"
  local by_half="(\\*$s$times|/$s(2\\.0?[_f64]{0,4}|$lift\\(${s}2\\.0?$s\\)))"
  local edge='(^|[^A-Za-z0-9_.*/[:space:]]|return)[[:space:]]*'
  # The tail refuses a digit, an exponent or a suffix after the half, so
  # `* 0.55`, `* 0.5e-3` and `/ 2.05` are not a halving.
  local tail="$s([^*/[:space:]0-9eE_]|\$)"
  # THE DIFFERENCE FORM. Group 1 is the edge and group 2 is E.
  local diff="\\($s$chain$s-$s\\2$s\\)"
  local lower="$edge($chain)$s\\+$s($times$s\\*$s$diff|$diff$s$by_half)$tail"
  # THE SUM FORM: a pair of ends, halved. A backreference's number is
  # global to its regex, so each shape is its own grep. In a shape, @K
  # stands for the group its first end lands in (3 behind a leading
  # halving: edge, halving; 2 before a trailing one), @L for K + 1 and
  # @M for K + 3; in a lifted pair (`pt(lo) + pt(hi)`, one lift on both
  # ends) K is the lift, L the prefix, @P (K + 2) the call and @Q
  # (K + 4) the index.
  local pfx='[A-Za-z0-9_.]*' call='((\(\))?)' idx='((\[[A-Za-z0-9_]+\])?)'
  local stem='[A-Za-z_][A-Za-z0-9_]*(\.[A-Za-z_][A-Za-z0-9_]*)*'
  local -a pairs=(
    "($pfx)lo$call$idx$s\\+$s\\@Khi\\@L\\@M"
    "($pfx)hi$call$idx$s\\+$s\\@Klo\\@L\\@M"
    "($lift)\\(${s}($pfx)lo$call$idx$s\\)$s\\+$s\\@K\\($s\\@Lhi\\@P\\@Q$s\\)"
    "($lift)\\(${s}($pfx)hi$call$idx$s\\)$s\\+$s\\@K\\($s\\@Llo\\@P\\@Q$s\\)"
    "($chain)\\.0$s\\+$s\\@K\\.1"
    "($chain)\\.1$s\\+$s\\@K\\.0"
    "($chain)\\[0\\]$s\\+$s\\@K\\[1\\]"
    "($chain)\\[1\\]$s\\+$s\\@K\\[0\\]"
    "($stem)0$s\\+$s\\@K1"
    "($stem)1$s\\+$s\\@K0"
    "($lift)\\(${s}($chain)\\.0$s\\)$s\\+$s\\@K\\($s\\@L\\.1$s\\)"
    "($lift)\\(${s}($chain)\\.1$s\\)$s\\+$s\\@K\\($s\\@L\\.0$s\\)"
    "($lift)\\(${s}($chain)\\[0\\]$s\\)$s\\+$s\\@K\\($s\\@L\\[1\\]$s\\)"
    "($lift)\\(${s}($chain)\\[1\\]$s\\)$s\\+$s\\@K\\($s\\@L\\[0\\]$s\\)"
    "($lift)\\(${s}($stem)0$s\\)$s\\+$s\\@K\\($s\\@L1$s\\)"
    "($lift)\\(${s}($stem)1$s\\)$s\\+$s\\@K\\($s\\@L0$s\\)"
  )
  local lerp_re="\\.lerp$s\\(.*,$s$times$s\\)"
  local std_re="(\\.midpoint$s\\($s[^)[:space:]]|(^|[^A-Za-z0-9_])f64$s::${s}midpoint$s\\()"
  local view lower_hits lerp_hits halves_hits std_hits sum_hits='' pair re k
  view=$(gate_rust_code --skip-cfg-test --statements "${files[@]}" \
    | gate_grep -vE "$(gate_record_anchor_any "${HOMES[@]}")")
  lower_hits=$(printf '%s\n' "$view" | gate_grep -E "$lower")
  lerp_hits=$(printf '%s\n' "$view" | gate_grep -E "$lerp_re")
  if ! halves_hits=$(printf '%s\n' "$view" | python3 -c "$HALVES_MATCHER"); then
    gate_error "$(gate_name): the halves matcher failed under $PWD (its own message is above, if it had one) — the gate could not read its subject, which is not a pass"
    exit 1
  fi
  std_hits=$(printf '%s\n' "$view" | gate_grep -E "$std_re")
  for pair in "${pairs[@]}"; do
    for k in 3 2; do
      re=${pair//@K/$k}
      re=${re//@L/$((k + 1))}
      re=${re//@M/$((k + 3))}
      re=${re//@P/$((k + 2))}
      re=${re//@Q/$((k + 4))}
      if [ "$k" = 3 ]; then
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
  if [ -n "$halves_hits" ]; then
    printf '%s\n' "$halves_hits"
    gate_error "a bracket's middle spelled ½lo + ½hi by hand (above): read it as mid_param(lo, hi) (geom_core::mid_param, re-exported as geom::mid_param), its home"
  fi
  if [ -n "$std_hits" ]; then
    printf '%s\n' "$std_hits"
    gate_error "a bracket's middle spelled with std's f64::midpoint (above): read it as mid_param(lo, hi) (geom_core::mid_param, re-exported as geom::mid_param), its home"
  fi
  if [ -n "$sum_hits$lower_hits$lerp_hits$halves_hits$std_hits" ]; then
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
plant_halves() { plant "$1" 'pub fn m(lo: f64, hi: f64) -> f64 { 0.5 * lo + 0.5 * hi }'; }
plant_halves_trailing() { plant "$1" 'pub fn m(x: I) -> f64 { x.lo() * 0.5 + x.hi() * 0.5 }'; }
plant_halves_divided() { plant "$1" 'pub fn m(lo: f64, hi: f64) -> f64 { lo / 2.0 + hi / 2.0 }'; }
plant_halves_scalar() { plant "$1" 'pub fn m<T: Real>(t0: T, t1: T) -> T { T::from_f64(0.5) * t0 + t1 * T::from_f64(0.5) }'; }
plant_halves_indexed() { plant "$1" 'pub fn m(x: [f64; 2]) -> f64 { 0.5 * x[0] + 0.5 * x[1] }'; }
plant_halves_tuple() { plant "$1" 'pub fn m(x: (f64, f64)) -> f64 { x.1 * 0.5 + x.0 * 0.5 }'; }
plant_sum_lift() { plant "$1" 'pub fn m(lo: f64, hi: f64) -> I { (pt(lo) + pt(hi)) * pt(0.5) }'; }
plant_sum_lift_stem() { plant "$1" 'pub fn m(t0: f64, t1: f64) -> I { (pt(t0) + pt(t1)) * pt(0.5) }'; }
plant_sum_lift_tuple() { plant "$1" 'pub fn m(x: (f64, f64)) -> I { pt(0.5) * (pt(x.1) + pt(x.0)) }'; }
plant_halves_lift() { plant "$1" 'pub fn m(lo: f64, hi: f64) -> I { pt(0.5) * pt(lo) + pt(0.5) * pt(hi) }'; }
plant_sum_point() { plant "$1" 'pub fn m(x: I) -> I { Interval::point(0.5) * (x.lo() + x.hi()) }'; }
plant_sum_suffixed() { plant "$1" 'pub fn m(lo: f64, hi: f64) -> f64 { 0.5_f64 * (lo + hi) }'; }
plant_lower_lift() { plant "$1" 'pub fn m(a: I, b: I) -> I { a + (b - a) * pt(0.5) }'; }
plant_halves_wrapped() { plant "$1" $'pub fn m(lo: f64, hi: f64) -> f64 {\n    0.5 * lo\n        + 0.5 * hi\n}'; }
plant_std_method() { plant "$1" 'pub fn m(lo: f64, hi: f64) -> f64 { lo.midpoint(hi) }'; }
plant_std_path() { plant "$1" 'pub fn m(lo: f64, hi: f64) -> f64 { f64::midpoint(lo, hi) }'; }
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
    printf 'pub fn o(a: B) -> f64 { a.midpoint() }\n'
    printf 'pub fn s(c: f64, a: f64, b: f64, u: V, v: V) -> f64 { c + 0.5 * a + 0.5 * b + (0.5 * u + 0.5 * v).x }\n'
    printf 'pub fn t(cleared: u64, stop: u64) -> u64 { cleared + (stop - cleared) / 2 }\n'
    printf 'pub fn q(a: f64, b: f64, c: f64) -> f64 { 0.5 * a + 0.25 * b + c * 2.0 }\n'
    printf 'pub fn r(a: f64, b: f64, h: f64) -> f64 { h * 0.5 * a + 0.5 * b * h }\n'
    printf 'pub fn u(lo: f64, hi: f64) -> f64 { (lo + hi) * 0.55 + (lo + hi) * 0.5e-3 + (lo + hi) / 2.05 }\n'
    printf 'pub fn v(lo: f64, hi: f64) -> f64 { 0.5 * lo + 0.55 * hi + lo / 2.05 + hi / 2.05 }\n'
    printf 'pub fn w(lo: f64, hi: f64) -> I { pt(lo) * 0.5 + hi * 0.5 }\n'
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
    plant_lower_scalar plant_lower_scalar_leading plant_lower_scalar_halved plant_lower_lift; do
    gate_selftest_case "read it as bracket_mid((lo, hi))" "$p"
  done
  for p in plant_sum_half plant_sum_trailing plant_sum_halved plant_sum_swapped plant_sum_indexed \
    plant_sum_tuple plant_sum_tuple_swapped plant_sum_pair plant_sum_pair_swapped plant_sum_stem \
    plant_sum_stem_swapped plant_sum_scalar plant_sum_scalar_leading plant_sum_scalar_halved \
    plant_sum_lift plant_sum_lift_stem plant_sum_lift_tuple plant_sum_point plant_sum_suffixed; do
    gate_selftest_case "read it as mid_param(lo, hi)" "$p"
  done
  for p in plant_halves plant_halves_trailing plant_halves_divided plant_halves_scalar plant_halves_wrapped \
    plant_halves_indexed plant_halves_tuple plant_halves_lift; do
    gate_selftest_case "spelled ½lo + ½hi by hand" "$p"
  done
  for p in plant_std_method plant_std_path; do
    gate_selftest_case "spelled with std's f64::midpoint" "$p"
  done
  for p in plant_lerp_half plant_lerp_scalar; do
    gate_selftest_case "read it as p.mid(q)" "$p"
  done
  gate_selftest_passes "a grid step, an integer middle, other names, an unhalved sum, another operand, a halved product, a trapezoid, ends of two different pairs or brackets, prose, a test module, fixture files and the home" plant_not_sites
  gate_selftest_homes --narrowed --subject "$HOME_SUBJECT" "${HOMES[@]}"
  printf '%s selftest OK: passes a clean fixture carrying the home, and a zero-argument midpoint door, other weighted sums, unpaired halves (a scalar and a vector), halves under two different lifts, a factor or divisor that only starts like a half (0.55, 0.5e-3, 2.05), an integer halving, halves behind a product, a lerp at a general t, a grid step, an integer middle, other names, an unhalved sum, another operand, a halved product, a trapezoid, ends of two different pairs or brackets, prose, a test module and fixture files; fires naming bracket_mid on lo + ½(hi − lo) with the half as 0.5 or T::from_f64(0.5) on either side or a division by 2.0 or T::from_f64(2.0), after return and wrapped across lines; fires naming mid_param on ½lo + ½hi over a paired lo/hi, t0/t1, x[0]/x[1] and x.1/x.0 (HALF on either side, /2.0, T::from_f64, wrapped, each end inside one lift) and on the std a.midpoint(b) and f64::midpoint(a, b); fires naming p.mid(q) on p.lerp(q, ½) with the half as 0.5 or T::from_f64(0.5); fires naming mid_param on ½(lo + hi) halved the same ways and through a lift (pt(0.5), Interval::point(0.5)) and 0.5_f64, in either order, as Xlo + Xhi (indexed and called), E.0 + E.1, E[0] + E[1] and S0 + S1, bare or each end inside one lift; and stays RED when grep cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
