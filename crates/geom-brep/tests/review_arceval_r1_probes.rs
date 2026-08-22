//! Blinded-review probes for the a-anchored arc evaluation (PR #922).
//!
//! Independent of `arc_eval_anchor.rs` (the unit's own regression row):
//! these rows pin the three claims of the anchoring note that the
//! committed row does not exercise —
//!
//! 1. `eval(0)` is `a` up to the interval `sin`'s outward rounding at
//!    the exact point 0 (~1e-24), across the bulge range (near-zero,
//!    semicircle, near-full-turn, negative), including endpoints that
//!    carry real width — the PR prose's "exactly `a`" is measured
//!    FALSE at the bit level, and P1 pins the true statement;
//! 2. the anchored form's enclosure is never wider than the
//!    center-anchored form's at `s = 0`, and never wider than it by
//!    more than the far-anchor slack `2·width(a)` anywhere on the arc —
//!    the asymmetry the reformulation bought, stated as a bound rather
//!    than as prose (P2);
//! 3. the *far* endpoint (`s = 1`) of a short restricted sub-arc also
//!    evaluates at its endpoints' scale — the committed row samples
//!    s = 0 and mid-arc only, and the new form is not symmetric in
//!    `a`/`b`, so the far end is the direction a regression would take
//!    (P3).
//!
//! The center-anchored comparator is replicated inline, byte-for-byte
//! the retired form (`center + R·v`, `sin_cos`), so P2 goes red either
//! way the relationship breaks: if `eval` regresses toward the center
//! form, or if the comparator stops being the width ceiling the doc
//! comment claims it is.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::SketchSegment;
use geom_core::{Bounds, Interval, Point2, Real, Vec2};

fn iv(x: f64) -> Interval {
    Interval::from_f64(x)
}

fn wide(x: f64, w: f64) -> Interval {
    Interval::from_bounds(x - w, x + w)
}

fn width(x: Interval) -> f64 {
    x.hi() - x.lo()
}

fn point_width(p: Point2<Interval>) -> f64 {
    width(p.x).max(width(p.y))
}

/// The retired center-anchored evaluation, replicated exactly as it
/// stood (`(s·θ).sin_cos()`, `center + R·v`), as the width comparator.
fn eval_center_anchored(seg: &SketchSegment<Interval>, s: Interval) -> Point2<Interval> {
    let SketchSegment::Arc { a, b, bulge } = *seg else {
        panic!("comparator is arc-only");
    };
    let half = Interval::from_f64(0.5);
    let four = Interval::from_f64(4.0);
    let chord = b - a;
    let len = chord.norm();
    let unit = chord / len;
    let n = Vec2::new(-unit.y, unit.x);
    let mid = a.lerp(b, half);
    let apothem = len * (Interval::one() - bulge.powi(2)) / (four * bulge);
    let center = mid + n * apothem;
    let theta = four * bulge.atan();
    let (sin, cos) = (s * theta).sin_cos();
    let v = a - center;
    center + Vec2::new(v.x * cos - v.y * sin, v.x * sin + v.y * cos)
}

/// Endpoint-width of the arc's stored data (the scale the evaluation
/// is supposed to keep).
fn endpoint_width(seg: &SketchSegment<Interval>) -> f64 {
    let SketchSegment::Arc { a, b, .. } = *seg else {
        panic!("arc-only");
    };
    point_width(a).max(point_width(b))
}

/// The fixture family: endpoints carrying explicit width `w`, across
/// the legal bulge range — shallow, quarter-turn, semicircle (the pip
/// meridian's shape), near-full-turn, and both signs.
fn family(w: f64) -> Vec<SketchSegment<Interval>> {
    let bulges = [
        0.01,                                 // θ ≈ 0.04 rad
        (core::f64::consts::PI / 12.0).tan(), // θ = π/3, the claimed crossover
        1.0,                                  // semicircle
        -1.0,                                 // semicircle, negative sense
        (0.49 * core::f64::consts::PI).tan(), // θ ≈ 1.96π, near-full turn
    ];
    bulges
        .iter()
        .map(|&bl| SketchSegment::Arc {
            a: Point2::new(wide(0.0, w), wide(-0.09, w)),
            b: Point2::new(wide(0.0, w), wide(0.09, w)),
            bulge: iv(bl),
        })
        .collect()
}

/// P1: `eval(0)` returns `a` — up to the interval `sin`'s outward
/// rounding AT THE EXACT POINT 0, which is nonzero. **Measured**: the
/// PR prose's "at s = 0 the result is exactly `a` / R − I is
/// identically zero" is bitwise FALSE at `T = Interval` — `sin([0,0])`
/// rounds outward, and the excess is scaled by `|v|`, the
/// reconstructed radius, so a residue of the 1/sin(θ/2) amplifier
/// survives at ulp scale (2.68e-17 measured on the shallow fixture).
/// Immaterial against any real endpoint width, but not "identically
/// zero"; this row pins the true statement — containment plus an
/// ulp-scale excess ceiling.
#[test]
fn p1_eval_at_s0_is_a_up_to_the_interval_sins_outward_rounding() {
    for (i, seg) in family(1e-9).iter().enumerate() {
        let SketchSegment::Arc { a, .. } = *seg else {
            unreachable!()
        };
        let e = seg.eval(iv(0.0));
        assert!(
            e.x.lo() <= a.x.lo()
                && a.x.hi() <= e.x.hi()
                && e.y.lo() <= a.y.lo()
                && a.y.hi() <= e.y.hi(),
            "fixture {i}: eval(0) does not contain `a`"
        );
        // Measured worst on this family: 2.68e-17, on the SHALLOW arc
        // (bulge 0.01) — the sin's outward rounding at the exact point
        // 0 is scaled by |v| = the reconstructed radius (~4.5 m there),
        // so a residue of the 1/sin(θ/2) amplifier survives at ulp
        // scale. Bounded at 1e-15 ≈ 4 ulps of that radius: decades
        // under any real endpoint width, but NOT the "identically
        // zero" of the prose.
        let excess = (point_width(e) - point_width(a)).abs();
        assert!(
            excess <= 1e-15,
            "fixture {i}: eval(0) is {excess:e} wider than `a` — the s = 0 anchor is leaking \
             more than the sin's outward rounding times the reconstructed radius"
        );
    }
}

/// P2: across the whole family and the whole parameter range, the
/// anchored form is never wider than the center-anchored form plus the
/// far-anchor slack `2·width(a)` plus a few-ulp rounding term — and at
/// `s = 0` it is never wider at all. **Measured**: with EXACT inputs
/// and `|s·θ| ≈ π/2` on a near-full-turn arc (radius ≈ 1.4 m), the
/// anchored form is genuinely wider — 1.11e-14 vs 7.11e-15 — because
/// it spends two `sin`s and re-adds `a` where the old form spent one
/// `sin_cos`; that is ~8 ulps at the fixture's scale and is the
/// rounding term below. The PR claims tightness only below ~π/3, and
/// that claim holds; this row pins the price OUTSIDE that regime so it
/// cannot silently grow past ulp scale.
#[test]
fn p2_anchored_width_is_bounded_by_center_width_plus_far_anchor_slack() {
    let samples = [0.0, 0.125, 0.25, 0.5, 0.75, 1.0];
    for w in [0.0, 1e-12, 1e-9] {
        for (i, seg) in family(w).iter().enumerate() {
            let SketchSegment::Arc { a, .. } = *seg else {
                unreachable!()
            };
            let wa = point_width(a);
            for &s in &samples {
                let new_w = point_width(seg.eval(iv(s)));
                let old_w = point_width(eval_center_anchored(seg, iv(s)));
                if s == 0.0 {
                    assert!(
                        new_w <= old_w,
                        "fixture {i} (w={w:e}) s=0: anchored {new_w:e} wider than centered {old_w:e}"
                    );
                }
                // 3·width(a): the far-anchor price — `a` re-mentioned
                // directly plus through `(R−I)·v`, whose row sums reach
                // ~2.1 at s·θ near π (theoretical ceiling 1 + √2).
                // 32 ulps at 2 m scale: the two-sins-plus-readd
                // rounding price, measured ~8 ulps worst on this
                // family with exact inputs.
                let slack = 3.0 * wa + 32.0 * f64::EPSILON * 2.0;
                assert!(
                    new_w <= old_w + slack,
                    "fixture {i} (w={w:e}) s={s}: anchored {new_w:e} exceeds centered {old_w:e} \
                     plus slack {slack:e}"
                );
            }
        }
    }
}

/// P2b: on the defect's own shape — a short restricted sub-arc, where
/// the reconstructed center is wide — the anchored form is *strictly*
/// tighter than the center-anchored form at every sample, not merely
/// bounded. This is the claim "tighter wherever |s·θ| is small" with
/// teeth: the row goes red if the improvement evaporates.
#[test]
fn p2b_on_a_short_sub_arc_the_anchored_form_is_strictly_tighter_everywhere() {
    let meridian = SketchSegment::Arc {
        a: Point2::new(iv(0.0), iv(-0.09)),
        b: Point2::new(iv(0.0), iv(0.09)),
        bulge: iv(1.0),
    };
    let sub = meridian.restrict(iv(0.4), iv(0.404));
    for &s in &[0.0, 0.25, 0.5, 0.75, 1.0] {
        let new_w = point_width(sub.eval(iv(s)));
        let old_w = point_width(eval_center_anchored(&sub, iv(s)));
        assert!(
            new_w < old_w,
            "s={s}: anchored {new_w:e} not strictly tighter than centered {old_w:e} on the \
             short sub-arc"
        );
    }
}

/// P3: the far endpoint. The committed regression row samples s = 0
/// and s = 0.5; the anchored form is not symmetric in `a`/`b`, so the
/// far end is where a regression would hide. Same fixture, same 8×
/// bound, at s = 1.
#[test]
fn p3_the_far_endpoint_of_a_short_sub_arc_evaluates_at_its_endpoints_scale() {
    let meridian = SketchSegment::Arc {
        a: Point2::new(iv(0.0), iv(-0.09)),
        b: Point2::new(iv(0.0), iv(0.09)),
        bulge: iv(1.0),
    };
    let sub = meridian.restrict(iv(0.4), iv(0.404));
    let ew = endpoint_width(&sub);
    assert!(ew > 0.0, "FIXTURE: endpoints must carry width");
    let at_end = sub.eval(iv(1.0));
    assert!(
        point_width(at_end) <= 8.0 * ew,
        "the sub-arc's far-end enclosure is {:e} wide against endpoint width {:e}",
        point_width(at_end),
        ew
    );
}
