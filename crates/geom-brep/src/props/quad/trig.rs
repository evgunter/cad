//! **Transcendental-free trig enclosures** (module docs of [`super::cylinder`]):
//! `(cos, sin)` brackets advanced by exact rotation, with polynomial
//! alternating-series bounds on each step — the cylinder lane's and the
//! spiric lane's harmonic channels read them.

use geom_core::Bounds;
use geom_core::interval::Interval;
use geom_core::interval::certification::Certification;

use crate::props::bracket::{mid, pt};

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

/// The limit a trim edge is integrated from or to, off the bracket `t`
/// of its parameter: the bracket's midpoint. The cylinder lane's
/// `TrimEdgeQ::limits` and [`trig_at_limit`] both read it here, so the
/// trig a caller hands the lane is at the limit the lane integrates
/// from.
pub(super) fn limit(t: Interval) -> f64 {
    mid(t)
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::props::bracket::mid;
    use geom_core::Bounds;

    /// Trig propagation soundness: dense grid of bases and offsets —
    /// the true (std) cos/sin at base+offset must lie inside the
    /// propagated brackets, and the brackets stay tight (< 1e-9 after
    /// a 6-rad walk).
    #[test]
    fn trig_propagation_contains_truth_and_stays_tight() {
        for i in 0..40 {
            let t0 = -6.0 + 0.31 * f64::from(i);
            let base = (pt(t0.cos()), pt(t0.sin()));
            for j in 0..30 {
                let off = -6.0 + 0.41 * f64::from(j);
                let (c, s) = trig_at(base, off);
                let (tc, ts) = ((t0 + off).cos(), (t0 + off).sin());
                assert!(
                    c.lo() <= tc && tc <= c.hi() && s.lo() <= ts && ts <= s.hi(),
                    "trig_at unsound at t0={t0}, off={off}: cos {tc} vs {c:?}, sin {ts} vs {s:?}"
                );
                assert!(
                    c.width() < 1e-9 && s.width() < 1e-9,
                    "width blowup at {off}"
                );
                // The RANGE form contains a sample fan of the interval.
                let d = 0.7;
                let (cr, sr) = trig_over(base, off, d);
                for k in 0..=8 {
                    let t = t0 + off + d * f64::from(k) / 8.0;
                    assert!(
                        cr.lo() <= t.cos() && t.cos() <= cr.hi(),
                        "trig_over cos unsound at {t}"
                    );
                    assert!(
                        sr.lo() <= t.sin() && t.sin() <= sr.hi(),
                        "trig_over sin unsound at {t}"
                    );
                }
            }
        }
    }

    /// **The trig at a trim edge's limit**, [`trig_at_limit`]: it holds
    /// the std `(cos, sin)` of the bracket's midpoint over a period each
    /// way, and its width is what its docs say, 1.7e-13 at `π` and
    /// 3.4e-13 at `2π`, each pinned within 1.1×.
    #[test]
    fn the_trig_at_a_limit_holds_its_value_at_the_width_its_docs_state() {
        for i in -64..=64 {
            let a = core::f64::consts::TAU * f64::from(i) / 64.0;
            let t = Interval::from_bounds(a - 1e-9, a + 1e-9);
            let (c, s) = trig_at_limit(t);
            let m = mid(t);
            assert!(
                c.lo() <= m.cos() && m.cos() <= c.hi() && s.lo() <= m.sin() && m.sin() <= s.hi(),
                "trig_at_limit misses the trig at {m}: {c:?}, {s:?}"
            );
        }
        let width = |a: f64| {
            let (c, s) = trig_at_limit(pt(a));
            c.width().max(s.width())
        };
        for (a, pinned) in [
            (core::f64::consts::PI, 1.7e-13),
            (core::f64::consts::TAU, 3.4e-13),
        ] {
            let w = width(a);
            assert!(
                w <= 1.1 * pinned && w >= pinned / 1.1,
                "trig_at_limit at {a} is {w:e} wide, pinned at {pinned:e}"
            );
        }
    }
}
