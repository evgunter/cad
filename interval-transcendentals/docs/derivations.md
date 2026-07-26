# Pad derivations: why every enclosure is conservative

Every bound this crate emits is `(libm or IEEE op) ± k neighbor steps`.
This note proves the `k`s. Notation: `RN(t)` = round-to-nearest of real
`t`; `next_down/next_up` = adjacent representables; `ulp(t)` = gap between
the two representables straddling `t` (for `t` beyond `MAX`, the gap of
`MAX`).

## §1 Rounding lemmas

**Lemma P1 (1 step covers correct rounding).** If `c = RN(t)` then
`next_down(c) <= t <= next_up(c)`.
*Proof.* Suppose `next_down(c) > t`. Then `next_down(c)` is representable
and strictly between `t` and `c`, so `|next_down(c) − t| < |c − t|`,
contradicting `c` nearest. Symmetric for `next_up`. Overflow: `c = +inf`
implies `t > MAX` (RN overflows only beyond the midpoint above `MAX`), and
`next_down(+inf) = MAX <= t`. Underflow/subnormals need no special case:
RN onto the subnormal grid is still nearest-point. ∎

Applies to: `+ − × ÷ sqrt fma` (IEEE 754 correctly rounded), hence every
padded endpoint in `round.rs` and `sqrt`.

**Lemma P2 (2k steps cover a k-ulp error).** For `k <= 2^50`: if
`|c − t| <= k·ulp(t)`, then `step_down(c, 2k) <= t <= step_up(c, 2k)`.
*Proof (lower side; upper is mirrored).* If `c <= t`, stepping down only
helps. So assume `t < c`. It suffices that **at most `2k` representables
lie strictly between `t` and `c`**: then `2k` steps down from `c` land
at or below the largest representable `<= t`.

`[t, c]` cannot cross 0: that would need `t < 0 <= c`, hence
`|c − t| > |t| >= 2^52·ulp(t) > k·ulp(t)` for normal `t` (for subnormal
`t`, `ulp(t)` is already the minimum positive gap and every gap in
`[t, c]` is `>= ulp(t)`; the count below then even gives `k`).
So `t` and `c` share a sign. Since `|c − t| <= k·2^−51·|t|`, every point
of `[t, c]` has magnitude `>= |t|·(1 − k·2^−51) >= |t|/2`, i.e. the
segment reaches at most ONE binade below `t`'s: **every gap inside
`[t, c]` is `>= ulp(t)/2`**. If `2k + 1` or more representables sat
strictly between `t` and `c`, the `>= 2k + 1` gaps separating `t` from
`c` would sum to `> 2k·(ulp(t)/2) = k·ulp(t)` — contradiction. ∎

The factor 2 is not paranoia — it is exactly the binade-boundary case
(gaps halve one binade toward zero), and a 1-ulp error straddling a
boundary genuinely needs 2 steps. The lemma is also empirically
enforced by the containment harness on millions of samples.

## §2 libm accuracy assumption (per function)

Rust `libm` 0.2.16 is the musl-derived pure-Rust port; rust-lang/libm's
CI checks every f64 function against MPFR with a per-function allowed
error, **1 ulp** for `sin cos tan asin acos atan atan2` (musl provenance:
the double-precision trig kernels with full Payne–Hanek argument
reduction; accuracy holds for ALL finite arguments, including huge ones).
The pad chain, every factor accounted for:

1. CI bound: error `<= 1` ulp, measured in "ulp of the expected value"
   (rust-lang/libm's precision checks divide by the ulp of the
   MPFR-computed reference).
2. Unit conversion: `ulp(expected) <= 2·ulp(t)` in the worst
   binade-boundary case, so error `<= 2·ulp(t)` (straddle definition of
   §1). This step takes the CI bound at face value — no extra safety
   multiplier; the factor 2 is the exact worst-case conversion.
3. Lemma P2 with `k = 2`: `2k = 4` outward steps enclose the true value.

Hence `PAD_ULPS = 4` for `sin cos tan asin acos atan atan2`. Typical
real-world libm error is < 0.7 ulp, so the pads are ~5× looser than
observed reality — that is what "proven, not measured" costs at these
magnitudes (≈ 9·10^−16 relative on O(1) values; three orders below the
kernel's tolerance scales). The differential harness then verifies
containment against inari+MPFR on millions of cases per function,
including adversarial edge sweeps; a single violation fails the build.

## §3 sqrt exactness witness

`f64::sqrt` is IEEE-correctly-rounded: 1 step (Lemma P1), except when
`s = sqrt(a)` is *provably exact*, witnessed by `fma(s, s, −a) == 0` —
valid because the rounding error of a product is exactly representable
when the product does not underflow (2Prod validity condition,
Ogita–Rump–Oishi, *Accurate Sum and Dot Product*, SIAM J. Sci. Comput.
2005; floor ≈ `2^−969` for binary64). We gate BOTH the sqrt witness and
`round.rs::mul_exact` at `2^−960` — nine spare binades over the
literature floor — so the residual can never be flushed to an
untruthful zero. Below the gate: always pad.

**Harness catch (kept as a war story because it is the whole point of
the oracle):** the first implementation gated `mul_exact` on
`r.is_normal()`. The differential harness found a barely-*normal*
product of a subnormal factor whose residual underflowed: the witness
returned "exact", the unpadded corner was 1 ulp short of the oracle's
hull — a real containment violation at case 997 of the arithmetic
sweep. `is_normal()` of the ROUNDED product is not the 2Prod validity
condition; the magnitude gate is.

## §4 Extremum / pole localization (trig) and atan2 corners

Grid test (`consts.rs`): "might `{c + k·p}` meet `[a, b]`?" is decided by
whether the OUTWARD-rounded interval `K = ([a,b] − C)/P` contains an
integer, with `C ⊇ c`, `P ⊇ p` one-ulp enclosures. Outward rounding only
widens `K`; a wider `K` can only turn "no integer" into "integer", so
`false` is a PROOF of absence and `true` is merely "possibly". Uses:

- `sin`: maxima exactly on `π/2 + 2πk`, minima on `−π/2 + 2πk`; interior
  extrema of `sin` on `[a,b]` occur only at those grid points, so if the
  min-grid is provably absent, `inf sin = min(sin a, sin b)` (padded),
  and a possible max-grid hit pins `sup <= 1` with the exactly
  representable `1.0`. Clipping to `[-1, 1]` is sound (true image is a
  subset) and makes extremum bounds exact. `cos`: same with grids `2πk`
  and `π + 2πk`.
- `tan`: poles exactly on `π/2 + πk`; provable absence makes `[a,b]` a
  subset of one open monotone branch (endpoint values bound the image);
  possible presence returns the whole line with `Trv` (1788 permits any
  decoration weaker than the tightest correct one — over-poisoning is
  sound, and it is the documented huge-argument refusal).
- `atan2` corner sufficiency (case 3 of `invtrig.rs`): on a box avoiding
  the origin and not meeting the open negative-x ray from both y-signs,
  atan2 is continuous and every box edge is monotone (horizontal edges:
  monotone in `x` for fixed `y ≠ 0`, constant `π` on a `y = 0` edge with
  `x < 0`; vertical edges: monotone in `y` on each side), and the
  gradient never vanishes — so extrema lie at corners. Zero `y`
  endpoints are normalized to `+0.0` before corner evaluation because
  the real number 0 against `x < 0` must evaluate on the `+π` branch
  (the `−0.0` corner would UNSOUNDLY miss `π`; regression-tested).
  IEEE atan2 infinity conventions supply correct one-sided limits for
  unbounded boxes; double-infinite corners (`±π/4`-type values) lie
  between the single-infinite limits and cannot shrink the hull.

## §5 powi

Directed binary exponentiation: products of same-sign directed bounds,
each padded by Lemma P1 (or exact via the §3 witness), stay directed
bounds inductively; underflow saturates the lower bound at `0` (sound:
true powers of nonnegative reals are `>= 0`), overflow saturates the
upper bound at `+inf`. Even powers of zero-straddling intervals return
lower bound EXACTLY `0.0` (the infimum is attained at `0 ∈ x`) — the
tight-square contract that keeps `sqrt(x² + …)` from seeing spurious
negatives (memories/interval-square-poison.md). Negative exponents are
the reciprocal of the positive power; division supplies pole semantics
(`Trv`, unbounded) when the base encloses 0.

## §6 Constants

`f64::consts::PI/TAU/FRAC_PI_2` are each the round-to-nearest of the
true constant and are known to land BELOW it (the 54th bit of π's binary
expansion is 1, and scaling by 2 is exact), so `[const, next_up(const)]`
encloses. The harness cross-checks against inari's constants at startup
of the certification suite.
