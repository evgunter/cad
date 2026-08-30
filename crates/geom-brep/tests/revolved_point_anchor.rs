//! The revolved-point description's anchor, pinned at the certified
//! scalar.
//!
//! `MappedCurve::RevolvedPoint` evaluates through
//! `Affine3::rotation_about_axis(axis_origin, axis_dir, s·angle)`, and
//! `restrict` composes that same map — at `s0` — into the STORED
//! placement. So whatever the anchored rotation charges for mentioning
//! its anchor is charged to this description twice over: once per
//! evaluation, and once permanently at every split.
//!
//! The constructor used to spell its translation `q − R·q`, which is
//! zero over the reals but `2·width(axis_origin)` wide at
//! `T = Interval` — `x − x` does not cancel in an enclosure. The rows
//! here are the consumer-side measurement of that: at `s = 0` the
//! described point is the placed sketch point and nothing else, so any
//! width beyond the placed point's own is the anchor artifact and
//! nothing else.
//!
//! The fixture's `axis_origin` carries width deliberately. Bodies built
//! in-process hand this constructor exact axis origins, where the
//! artifact is zero for the trivial reason that `x − x` is `[0, 0]` on
//! a degenerate interval; the widths appear when a revolution axis
//! comes out of arithmetic — an imported carrier's circle center
//! (`step-import`'s `RevolvedPoint` mint), a fitted axis, a placement
//! composed through a chain of maps. That is the population this
//! measures.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::TAU;

use geom_brep::MappedCurve;
use geom_core::{Affine3, Bounds, Interval, Point2, Point3, Real, Vec3};

fn iv(x: f64) -> Interval {
    Interval::from_f64(x)
}

fn width(e: Interval) -> f64 {
    e.hi() - e.lo()
}

fn point_width(p: Point3<Interval>) -> f64 {
    width(p.x).max(width(p.y)).max(width(p.z))
}

/// A revolve rim: a sketch point at radius 1 from the axis, revolved a
/// full turn about a `+z` axis whose origin carries `half` of enclosure
/// half-width per component.
fn rim(half: f64) -> MappedCurve<Interval> {
    let w = |c: f64| Interval::from_bounds(c - half, c + half);
    MappedCurve::RevolvedPoint {
        point: Point2::new(iv(2.0), iv(2.0)),
        place: Affine3::translation(Vec3::new(iv(0.0), iv(0.0), iv(3.0))),
        axis_origin: Point3::new(w(1.0), w(2.0), w(3.0)),
        axis_dir: Vec3::new(iv(0.0), iv(0.0), iv(1.0)),
        angle: iv(TAU),
    }
}

/// The described point at `s = 0` is the placed sketch point, so **the
/// axis origin's own enclosure must not reach it** — at any axis width.
///
/// The sweep is the assertion: three axis widths six orders apart, and
/// the start sample must read the same on all three. Under `q − R·q` it
/// read `2·width(axis_origin)` — 4e-9 on the last row, six orders above
/// the other two — so the spread is exactly the artifact, measured.
///
/// **What is left is not this site.** The residue the three rows share
/// (2.7e-15 here) is `Mat3::rotation_about`'s own: at `angle = 0` the
/// interval `cos` encloses `[1 − 4.44e-16, 1]`, so `R` is `I` only to
/// within that, and `R·p` spreads it across the placed point's
/// coordinates (|p| ≈ 3 ⇒ ≈ 2.7e-15). It is a sibling of the same class
/// at a different site, and it is recorded as such rather than fixed
/// here — retiring it re-spells the factor every rotation in the kernel
/// is built from, which is a bit-movement pass of its own.
///
/// ε-free: enclosure widths only, no tolerance is consulted, so the row
/// reads identically at every tolerance row.
#[test]
fn the_revolved_anchor_contributes_no_width_at_the_start_sample() {
    let mut widths = [0.0f64; 3];
    for (row, half) in [0.0f64, 1.0e-12, 1.0e-9].into_iter().enumerate() {
        let curve = rim(half);
        let at_start = point_width(curve.eval(iv(0.0)));
        println!("axis half-width {half:e}: eval(0) width {at_start:e}");
        widths[row] = at_start;
        assert!(
            at_start <= 1.0e-14,
            "the start sample of a revolved point on an axis of half-width \
             {half:e} is {at_start:e} wide — the described point there is the \
             placed sketch point, which is exact in this fixture"
        );
    }
    assert!(
        widths[2] <= 2.0 * widths[0] && widths[1] <= 2.0 * widths[0],
        "the start sample tracks the axis origin's width ({:e} / {:e} / {:e} at \
         half-widths 0 / 1e-12 / 1e-9) — the anchor is reaching a sample that \
         does not depend on it",
        widths[0],
        widths[1],
        widths[2],
    );
}

/// `restrict` STORES the anchored map, so the same artifact would be
/// stored rather than merely transient: the sub-curve's own start sample
/// carries whatever the composed placement picked up.
///
/// `s0 = 0` is the case the issue named — the composed rotation is the
/// identity, so every bit of width in the stored placement came from the
/// anchor round trip. A nonzero `s0` is measured too: there the axis
/// width legitimately reaches the answer (rotating about an uncertain
/// axis genuinely moves the point), so the bound is the honest
/// `≈ angle·width(axis)` rather than zero.
#[test]
fn restriction_does_not_store_the_anchor_round_trip() {
    let half = 1.0e-9;
    let curve = rim(half);

    let from_start = curve.restrict(iv(0.0), iv(0.25));
    let stored = point_width(from_start.eval(iv(0.0)));
    let exact_axis = point_width(rim(0.0).restrict(iv(0.0), iv(0.25)).eval(iv(0.0)));
    println!("restrict(0, 0.25).eval(0) width {stored:e} (exact axis: {exact_axis:e})");
    assert!(
        stored <= 2.0 * exact_axis,
        "restricting from s0 = 0 stored {stored:e} of width into the placement \
         against {exact_axis:e} on an exact axis — the s0 = 0 composition is the \
         identity, so the axis origin's width must not enter it"
    );

    // A real advance: the axis uncertainty is genuinely in the answer,
    // bounded by the swept angle against the axis width (2·π·2e-9 ≈
    // 1.3e-8), not by a round trip.
    let advanced = curve.restrict(iv(0.25), iv(0.5));
    let real = point_width(advanced.eval(iv(0.0)));
    println!("restrict(0.25, 0.5).eval(0) width {real:e}");
    assert!(
        real <= 4.0 * TAU * (2.0 * half),
        "a quarter-turn advance about an axis of width {:e} gave {real:e}",
        2.0 * half,
    );
}

/// The full-period sample, which revolve seams land on: `s = 1` at
/// `angle = 2π` describes the start point again.
///
/// This is the other place the anchored translation shows up, and the
/// half-angle factoring reaches it too: the operator's factors are
/// `2·sin²(π)` and `2·sin(π)·cos(π)` rather than `1 − cos 2π` and
/// `sin 2π`, and the first of those is the one that used to enclose
/// `[0, 4.44e-16]` at an angle where its true value is zero.
#[test]
fn the_full_period_sample_returns_to_the_start_enclosure() {
    let curve = rim(1.0e-9);
    let start = curve.eval(iv(0.0));
    let end = curve.eval(iv(1.0));
    let w = point_width(end);
    println!("eval(1) at a full period: width {w:e}");
    for (a, b, which) in [
        (start.x, end.x, "x"),
        (start.y, end.y, "y"),
        (start.z, end.z, "z"),
    ] {
        // The two enclosures describe the same point, so they overlap;
        // the full-period one is wider by the axis's own contribution
        // over a full turn, not by an anchor round trip.
        assert!(
            b.lo() <= a.hi() && a.lo() <= b.hi(),
            "the full-period sample {which} = [{:e}, {:e}] misses the start \
             enclosure [{:e}, {:e}]",
            b.lo(),
            b.hi(),
            a.lo(),
            a.hi(),
        );
    }
    assert!(
        w <= 4.0 * TAU * 2.0e-9,
        "the full-period sample is {w:e} wide against an axis width of 2e-9"
    );
}
