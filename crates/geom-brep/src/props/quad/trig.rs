//! **Transcendental-free trig enclosures** (module docs of [`super`]):
//! `(cos, sin)` brackets advanced by exact rotation, with polynomial
//! alternating-series bounds on each step — the cylinder lane's and the
//! spiric lane's harmonic channels read them.

use geom_core::Bounds;
use geom_core::interval::Interval;
use geom_core::interval::certification::Certification;

use super::cylinder::limit;
use crate::props::bracket::pt;

/// Enclosure of `cos s` for an EXACT step `s`, |s| ≤ 1.5, by the
/// degree-8 alternating-series pair
/// `1 − s²/2 + s⁴/24 − s⁶/720 ≤ cos s ≤ … + s⁸/40320` (truncations of
/// an alternating series with decreasing terms — decreasing needs
/// s² ≤ 56, ample here; both ends formed in certification arithmetic so their own
/// rounding is outward). Refused outside the domain.
/// **Safe by construction, and that is why the re-mint below needs no
/// refusal**: every operand is `pt` of a finite `f64` and every
/// divisor is a nonzero exact point, so no step can leave a domain
/// and `lo`/`hi` are certified brackets whose endpoints mean what
/// they say. The same argument covers [`sin_step`].
fn cos_step(s: f64) -> Interval {
    if s.is_nan() || s.abs() > 1.5 {
        return Interval::refused();
    }
    let s2 = pt(s).sqr();
    let s4 = s2.sqr();
    let lo = pt(1.0) - s2 / pt(2.0) + s4 / pt(24.0) - s4 * s2 / pt(720.0);
    let hi = lo + s4.sqr() / pt(40_320.0);
    Interval::from_bounds(lo.lo(), hi.hi()).clamped_to(-1.0, 1.0)
}

/// Enclosure of `sin s` for an EXACT step `s`, |s| ≤ 1.5: the degree-9
/// alternating-series pair `s − s³/6 + s⁵/120 − s⁷/5040 ≤ sin s ≤
/// … + s⁹/362880` for s ≥ 0 (decreasing terms need s² ≤ 72), mirrored
/// by oddness for s < 0.
fn sin_step(s: f64) -> Interval {
    if s.is_nan() || s.abs() > 1.5 {
        return Interval::refused();
    }
    let a = s.abs();
    let a1 = pt(a);
    let a2 = a1.sqr();
    let a3 = a1 * a2;
    let a5 = a3 * a2;
    let a7 = a5 * a2;
    let lo = a1 - a3 / pt(6.0) + a5 / pt(120.0) - a7 / pt(5040.0);
    let hi = lo + a7 * a2 / pt(362_880.0);
    let unit = Interval::from_bounds(lo.lo(), hi.hi()).clamped_to(-1.0, 1.0);
    if s >= 0.0 {
        unit
    } else {
        Interval::from_bounds(-unit.hi(), -unit.lo())
    }
}

/// One rotation: `(c, s)` advanced by a step whose `(cos, sin)`
/// enclosures are given. Intersecting with [−1, 1] after every
/// compose keeps the propagated brackets from drifting past the
/// circle (sound: the true values lie in both).
pub(in crate::props) fn rotate(
    c: Interval,
    s: Interval,
    ch: Interval,
    sh: Interval,
) -> (Interval, Interval) {
    let clamp = |x: Interval| x.clamped_to(-1.0, 1.0);
    (clamp(c * ch - s * sh), clamp(s * ch + c * sh))
}

/// `(cos a, sin a)` at the limit `a` a trim edge's bracket `t` is
/// integrated from or to ([`limit`], as [`TrimEdgeQ::limits`](super::cylinder::TrimEdgeQ::limits) takes it),
/// by `trig_at` from the exact `(1, 0)` at `a = 0`: transcendental-free,
/// and its width grows with `|a|` by the double-angle steps, 1.7e-13 at
/// `a = π` and 3.4e-13 at `2π`. Refused for a refused bracket.
#[must_use]
pub fn trig_at_limit(t: Interval) -> (Interval, Interval) {
    trig_at((Interval::one(), Interval::zero()), limit(t))
}

/// `(cos, sin)` at exact offset `off` from a base enclosure:
/// argument-halved series + double-angle squaring. `off` is halved
/// (exactly — powers of two) until ≤ 0.05, the series pair bounds that
/// seed (truncation below interval arithmetic's own ulp there), and the rotation
/// is rebuilt by `k` double-angle steps `(c, s) → (c² − s², 2cs)` —
/// the width amplification is 2^k on a ~1e-16 seed (3.4e-13 for a
/// full-period offset, [`trig_at_limit`]), NOT the exponential-in-|off| compounding a
/// step-march would suffer. The composed rotation is applied to the
/// base exactly once.
pub(in crate::props) fn trig_at(base: (Interval, Interval), off: f64) -> (Interval, Interval) {
    if off == 0.0 {
        return base;
    }
    if !off.is_finite() {
        return (Interval::refused(), Interval::refused());
    }
    let mut seed = off;
    let mut k = 0u32;
    while seed.abs() > 0.05 {
        seed *= 0.5;
        k += 1;
    }
    let (mut c, mut s) = (cos_step(seed), sin_step(seed));
    let clamp = |x: Interval| x.clamped_to(-1.0, 1.0);
    for _ in 0..k {
        // Double angle; `sqr` keeps the squares tight (the
        // interval-square rule).
        let c2 = clamp(c.sqr() - s.sqr());
        let s2 = clamp(pt(2.0) * c * s);
        c = c2;
        s = s2;
    }
    rotate(base.0, base.1, c, s)
}

/// `(cos, sin)` RANGE enclosures over offsets `[off, off + d]` from a
/// base enclosure, `d ≥ 0`: rotate to `off`, then widen by the
/// interval rotation `cos ∈ [1 − d²/2, 1]`, `sin ∈ [0, d]` (sound for
/// d ≤ π; larger spans fall back to the whole circle).
pub(super) fn trig_over(base: (Interval, Interval), off: f64, d: f64) -> (Interval, Interval) {
    if d.is_nan() || d < 0.0 || !off.is_finite() {
        return (Interval::refused(), Interval::refused());
    }
    if d > 3.0 {
        let full = Interval::from_bounds(-1.0, 1.0);
        return (full, full);
    }
    let at = trig_at(base, off);
    // The `.max`/`.min` below are NOT the refusal-swallowing shape
    // `Interval::clamped_to` exists for: `d` is a finite nonnegative
    // f64 by the guard above, so this certification arithmetic cannot produce
    // a refusal and there is no NaN for `f64::max` to absorb. `base` is the
    // operand that can be refused, and it reaches only `trig_at`/`rotate`,
    // which clamp through `clamped_to`.
    let ch = {
        let lo = (pt(1.0) - pt(d).sqr() / pt(2.0)).lo().max(-1.0);
        Interval::from_bounds(lo, 1.0)
    };
    let sh = Interval::from_bounds(0.0, d.min(1.0));
    rotate(at.0, at.1, ch, sh)
}

/// `cos δ` over `|δ| ≤ h`: `[1 − h²/2, 1]`, the whole circle past
/// `h = 1.5`. The `.max` is not the refusal-swallowing shape
/// `Interval::clamped_to` exists for: `h` is a finite f64 by every
/// caller's guard and the operands are `pt` of it with an exact
/// nonzero divisor, so no refusal can arise.
pub(in crate::props) fn spread_cos(h: f64) -> Interval {
    if h <= 1.5 {
        Interval::from_bounds((pt(1.0) - pt(h).sqr() / pt(2.0)).lo().max(-1.0), 1.0)
    } else {
        Interval::from_bounds(-1.0, 1.0)
    }
}

/// `sin δ` over `|δ| ≤ h`: `[−h, h]`, clipped to the unit interval.
pub(in crate::props) fn spread_sin(h: f64) -> Interval {
    let b = h.min(1.0);
    Interval::from_bounds(-b, b)
}
