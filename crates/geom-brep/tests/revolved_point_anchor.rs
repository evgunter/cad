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
/// **What is left is not this site, and it is not the `1 − cos` floor
/// either.** The residue the three rows share is 2.6645352591003757e-15,
/// and it decomposes — measured, on this fixture — as
/// `width(R.c2.z) × p.z`:
///
/// - `R = rotation_about(+z, [0, 0])`. Its diagonal entry on the axis
///   component is `t·nᵢ² + c` with `nᵢ = 1`, i.e. `t + c`. Both halves
///   carry an ulp-of-1 enclosure — `t = 1 − cos` is `[0, 4.44e-16]` and
///   `c = cos` is `[1 − 4.44e-16, 1]` — and they **add**: the entry is
///   **8.88e-16** wide, not 4.44e-16.
/// - `p` is the placed sketch point `(2, 2, 3)`, and the widest
///   coordinate of `R·p` is the z one, `8.88e-16 × 3 = 2.66e-15`. The
///   multiplier is the **z coordinate, 3** — not `|p|`, which is
///   `√17 = 4.12`.
///
/// So `1 − cos` is at most HALF of it, and retiring `1 − cos` alone
/// does not remove even that half: `t` respelled to the half angle is
/// `[0, 2.5e-323]`, but adding subnormal dust to a near-1 quantity
/// still rounds the sum outward by an ulp of 1, leaving the entry
/// 6.66e-16 wide — which, multiplied by 3 and rounded outward again,
/// gives back the **same 2.66e-15**. Measured recovery from respelling
/// `t` alone: **0%**. From respelling `t` and `c` both (`c = 1 − 2sin²`,
/// the full half-angle form): **17%** at this sample, and **0%** at the
/// full-period sample, where the shipped and fully-respelled forms are
/// both 2.66e-15.
///
/// The irreducible part is `cos`'s own enclosure at an exact angle,
/// which is a backend property, not a spelling. `Mat3::rotation_about`
/// is still recorded as an audit member of the same class — its `t`
/// does contribute the other half — but it is not "the entire residue",
/// and retiring it would buy at most a sixth of this number.
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
    // CALIBRATED TO EXACTLY ONE SPLIT. Both sides here are one
    // composition deep (6.217248937900877e-15 measured, on both the
    // wide and the exact axis), so the comparison says the AXIS WIDTH
    // does not enter — which is this row's claim — and says nothing
    // about what a second composition costs. It is not a bound on
    // repeated restriction: that grows, and its law is pinned by
    // `stored_restriction_width_grows_linearly_in_the_split_count`.
    assert!(
        stored <= 2.0 * exact_axis,
        "restricting from s0 = 0 stored {stored:e} of width into the placement \
         against {exact_axis:e} on an exact axis — the s0 = 0 composition is the \
         identity, so the axis origin's width must not enter it"
    );

    // A real advance: the axis uncertainty is genuinely in the answer.
    // The law is `2·width(axis)` for a quarter turn (the displacement of
    // an uncertain origin under a π/2 rotation), measured
    // 4.000015429994619e-9 against `2·width(axis)` = 4e-9; bounded at 2×
    // that. The old bound carried a spurious `4·2π` and was 12× loose.
    let advanced = curve.restrict(iv(0.25), iv(0.5));
    let real = point_width(advanced.eval(iv(0.0)));
    println!("restrict(0.25, 0.5).eval(0) width {real:e}");
    assert!(
        real <= 4.0 * (2.0 * half),
        "a quarter-turn advance about an axis of width {:e} gave {real:e}, \
         over the ≈ 2·width(axis) the geometry itself accounts for",
        2.0 * half,
    );
}

/// **Repeated `restrict` accumulates, linearly, and nothing else here
/// says so.** `split_edge` calls `restrict`, and every call composes the
/// anchored rotation into the STORED placement through `Affine3::Mul` —
/// which re-applies `Mat3::rotation_about`'s diagonal enclosure to the
/// translation each time. The cost is therefore paid per split, not
/// once, and it does not converge.
///
/// Measured on an **exact-axis** fixture, so nothing here comes from the
/// axis origin's width — this is the residue of the rotation itself,
/// re-added:
///
/// | splits | `eval(0)` width |
/// |---|---|
/// | 0 | 2.6645352591003757e-15 |
/// | 1 | 6.217248937900877e-15 |
/// | 2 | 9.769962616701378e-15 |
/// | 4 | 1.687538997430238e-14 |
/// | 8 | 3.108624468950438e-14 |
///
/// The increment is 3.552713678800501e-15 per split, the same to the
/// bit at every step. **The law is what is pinned, not the digits**:
/// linearity, and a slope of the order of the diagonal enclosure
/// (8.88e-16) times the coordinate scale. A quadratic or compounding
/// growth would mean the composition had started multiplying rather
/// than adding its slop, and a slope orders off would mean the
/// per-composition cost had changed character. Both red here; a
/// last-ulp move in the increment does not, and should not.
///
/// This is the caller/callee-split shape the unit's sweep declares as
/// its blind spot — a constructor that stores an anchor and a method
/// that composes onto it later — now with a measured instance.
#[test]
fn stored_restriction_width_grows_linearly_in_the_split_count() {
    let mut curve = rim(0.0);
    let mut widths = Vec::new();
    for _ in 0..=8 {
        widths.push(point_width(curve.eval(iv(0.0))));
        curve = curve.restrict(iv(0.0), iv(0.5));
    }
    println!("widths by split count: {widths:?}");

    let base = widths[0];
    let slope = widths[1] - widths[0];
    // The slope is the diagonal enclosure (8.88e-16) times the placed
    // point's coordinate scale (3), to within a small factor. Pin the
    // ORDER, not the digits.
    assert!(
        slope >= 8.0e-16 && slope <= 1.6e-14,
        "the per-split increment is {slope:e}; expected the order of \
         `rotation_about`'s diagonal enclosure (8.88e-16) times the placed \
         point's coordinate scale — a slope outside this band means the \
         per-composition cost has changed character"
    );
    for (n, &w) in widths.iter().enumerate() {
        #[allow(clippy::cast_precision_loss)]
        let predicted = base + slope * n as f64;
        assert!(
            (w - predicted).abs() <= 0.25 * slope,
            "after {n} splits the stored width is {w:e}, off the linear law \
             `{base:e} + {slope:e}·n` (predicted {predicted:e}) — repeated \
             restriction is no longer accumulating linearly"
        );
    }
    // And it really does grow: a row that passed on a constant would be
    // pinning nothing.
    assert!(
        widths[8] >= 8.0 * widths[0],
        "eight splits took the stored width from {:e} only to {:e} — if the \
         accumulation has stopped, this row's law is stale and the stored-row \
         comment above it is wrong",
        widths[0],
        widths[8],
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
///
/// **The ceiling is calibrated to what this row commemorates.** The
/// improvement is six orders — 4.0e-9 before the constructor changed,
/// 2.6645352591003757e-15 after — and a ceiling the PRE-FIX tree also
/// passes guards none of it. This row's bound was `4·2π·width(axis)`
/// ≈ 5e-8, which the pre-fix 4.0e-9 clears by an order: it could not
/// have gone red for the thing it exists to record. The bound is now
/// ~10× the measured width. That reds the pre-fix spelling by six
/// orders and still leaves room for the last-ulp movement a libm bump
/// can cause.
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
    // 2.6645352591003757e-15 measured; ~10x that. NOT angle*width(axis),
    // which the defect this row records also satisfies.
    assert!(
        w <= 2.7e-14,
        "the full-period sample is {w:e} wide — the half-angle factoring \
         holds it to ~2.66e-15 here; the retired `1 − cos 2π` spelling \
         paid 4.0e-9, so a width in that range means the seam angle has \
         gone back to being paid as an ulp-of-1 cancellation"
    );
}
