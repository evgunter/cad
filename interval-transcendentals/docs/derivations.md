# Pad derivations: why every enclosure is conservative

Every bound this crate emits is `(libm or IEEE op) ± k neighbor steps`.
This note proves the `k`s. Notation: `RN(t)` = round-to-nearest of real
`t`; `next_down/next_up` = adjacent representables; `ulp(t)` = gap between
the two representables straddling `t` (for `t` beyond `MAX`, the gap of
`MAX`).

## §1 Rounding lemmas

**Lemma P0 (TwoSum is exact, with no underflow proviso).** For finite
doubles `a`, `b` with `s = RN(a + b)` finite, the rounding error
`e = (a + b) − s` is **itself exactly representable**, and Knuth's
six-operation TwoSum computes it exactly:
`bp = s − a; ap = s − bp; e = (a − ap) + (b − bp)`.
*Proof sketch (Knuth, *TAOCP* vol. 2 §4.2.2 Thm. B; Shewchuk 1997 Thm.
7).* `s` lies in the same binade as the larger operand or one above, so
`e` has magnitude `<= ulp(s)/2` and its trailing bits are a suffix of
the exact sum's bits, which the exponent range of binary64 accommodates
— **including when `a`, `b` or `e` are subnormal**, because addition on
the subnormal grid is exact (the grid is uniform and closed under
differences). The six operations are each exact by the same argument
applied to Sterbenz-representable differences. ∎

Consequences used by the code:
- `round.rs::two_sum_err` decides *exactly* whether `a + b` was exact,
  so `add_lo`/`add_hi`/`sub_lo`/`sub_hi` pad only when they must;
- **there is no validity floor for the addition witness.** This is the
  contrast with 2Prod (§3), whose residual can itself underflow below
  `2^-960` and make the witness LIE — which is why the multiplication,
  division and sqrt witnesses are magnitude-gated and addition's is not,
  and why `tests/review_fuzz_exact.rs` fuzzes those three and not
  `+ −`. Non-finite `s` yields a NaN error term, which compares
  `!= 0.0` and therefore takes the padded path.

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
boundary genuinely needs 2 steps. The libm pads in §2 rest on the
sharper P3 below, which counts in the metric libm's CI measures.

**Lemma P3 (bit-distance from a correctly rounded reference).** Let
`expected = RN(t)` and let the returned value `c` satisfy
`bitdist(c, expected) <= k`, where `bitdist(x, y) =
|(bits(x) as i64) − (bits(y) as i64)|` (libm-test's metric). Within
each sign class this encoding is a unit-step isometry: one
`next_up`/`next_down` step changes the encoded integer by exactly 1
(subnormals and binade boundaries included), order-preserving on
positives and order-REVERSING on negatives — either way, absolute
encoded distance = neighbor-step count. Then
`step_down(c, k + 1) <= t <= step_up(c, k + 1)`.
*Proof.* For small `k` the hypothesis forces `c` and `expected` to
share a sign bit: opposite-sign bit patterns differ by ≥ 2^63 under the
signed encoding (e.g. `+0.0 ↦ 0`, `−0.0 ↦ i64::MIN`), so a passing
small bound implies same sign, where `bitdist` counts neighbor steps
exactly. Hence `c` is within `k` neighbor steps of
`expected` in the total order: `step_down(c, k) <= expected <=
step_up(c, k)`. Lemma P1 gives `next_down(expected) <= t <=
next_up(expected)`. Composing (both `next_down` and `next_up` are
monotone): `step_down(c, k + 1) = next_down(step_down(c, k)) <=
next_down(expected) <= t`, mirrored above. Infinities: a finite/
infinite mismatch is rejected by the CI harness outright, and if both
are `+inf` the lower chain still holds through `next_down(+inf) = MAX
<= t` (P1's overflow case). ∎

Note what P3 buys over the P2 route: no ulp-unit conversion is needed
at all, because the CI metric already counts representables — the
binade factor 2 never enters.

## §2 libm accuracy assumption (per function)

Rust `libm` 0.2.16 is the musl-derived pure-Rust port. Source facts,
verified at tag `libm-v0.2.16` (rust-lang/compiler-builtins), which the
pads rest on:

1. **The CI metric is integer bit-distance, not an ulp quotient**:
   `libm-test/src/test_traits.rs` computes
   `act_bits.checked_sub(exp_bits).unwrap().abs()` on the sign-extended
   bit patterns — exactly `bitdist` of Lemma P3.
2. **The reference is the correctly rounded value**: the MPFR oracle
   runs at 53-bit precision, `Round::Nearest`, with
   `subnormalize_ieee_round` (`libm-test/src/mpfloat.rs`) — a single
   rounding, so `expected = RN(t)` exactly (no double-rounding gap).
3. **Per-function allowed distance** (`libm-test/src/precision.rs`):
   `sin cos tan asin acos atan` ⇒ **1**; **`atan2` ⇒ 2**. (Musl provenance; trig uses full Payne–Hanek
   argument reduction, so the bound is enforced across the sampled
   sweeps at all magnitudes, huge arguments included.)

Assumption A (the one non-derived ingredient, stated plainly): the
shipped binaries meet their CI distance bounds on ALL inputs, not just
the CI's sampled ones. This is trust in musl's documented accuracy +
libm's enforcement, hedged by this crate's own differential sweeps
(millions of cases incl. subnormal/huge windows; one violation fails
the build).

The proof is then Lemma P3, per function:

| function | CI bitdist k | steps needed (k+1) | PAD_ULPS | margin |
|---|---|---|---|---|
| sin, cos, tan | 1 | 2 | 4 | 2 |
| asin, acos, atan | 1 | 2 | 4 | 2 |
| atan2 | **2** | **3** | 4 | **1** |

`PAD_ULPS = 4` therefore covers every inventoried transcendental, with
margin 2 everywhere except atan2 (margin 1 — adequate, but any future
function added with a CI bound above 3 must raise its own pad).

Typical real-world libm error is < 0.7 ulp, so the 4-step pads are
~5× looser than observed reality — the price of "proven, not
measured" (≈ 9·10^−16 relative on O(1) values; three orders below the
kernel's tolerance scales).

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

**The gate is on magnitude, not on `is_normal()`.** `is_normal()` of the
ROUNDED product is not the 2Prod validity condition: a barely-normal
product of a subnormal factor can have a residual that underflows, so
the witness returns "exact" for an inexact product and the unpadded
corner falls outside the true hull.

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
negatives. Negative exponents are
the reciprocal of the positive power; division supplies pole semantics
(`Trv`, unbounded) when the base encloses 0.

## §6 Constants

`f64::consts::PI/TAU/FRAC_PI_2` are each the round-to-nearest of the
true constant and are known to land BELOW it (the 54th bit of π's binary
expansion is 1, and scaling by 2 is exact), so `[const, next_up(const)]`
encloses. The harness cross-checks against inari's constants at startup
of the certification suite.
