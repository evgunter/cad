//! The swept-point descriptions' anchor and restriction, pinned at the
//! certified scalar.
//!
//! `MappedCurve::RevolvedPoint` evaluates through
//! `Affine3::rotation_about_axis(axis_origin, axis_dir, angles.at(s))`,
//! and `restrict` narrows `angles` while the stored placement stays as
//! built. So the description pays for its rotation once per evaluation
//! and never per split. `ExtrudedPoint` restricts its `stations` the
//! same way.
//!
//! The first fixture's `axis_origin` carries width deliberately. Bodies
//! built in-process hand the constructor exact axis origins; the widths
//! appear when a revolution axis comes out of arithmetic — an imported
//! carrier's circle center (`step-import`'s `RevolvedPoint` mint), a
//! fitted axis, a placement composed through a chain of maps.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::TAU;

use crate::shared::interval::iv;
use geom_brep::{MappedCurve, SweepRange};
use geom_core::{Affine3, Bounds, Interval, Point2, Point3, Vec3};

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
        angles: SweepRange::from_zero(iv(TAU)),
    }
}

/// Where a far fixture sits: a thousand metres out, so a cost charged
/// at the coordinates' scale is three orders above one charged at the
/// radius's.
const FAR: [f64; 3] = [1000.0, -700.0, 300.0];

/// [`rim`]'s exact-axis geometry translated by `at`: the placed point
/// is `at + (2, 2, 0)`, one metre off a `+z` axis through
/// `at + (1, 2, 0)`.
fn rim_at(at: [f64; 3]) -> MappedCurve<Interval> {
    let [x, y, z] = at;
    MappedCurve::RevolvedPoint {
        point: Point2::new(iv(2.0), iv(2.0)),
        place: Affine3::translation(Vec3::new(iv(x), iv(y), iv(z))),
        axis_origin: Point3::new(iv(1.0 + x), iv(2.0 + y), iv(z)),
        axis_dir: Vec3::new(iv(0.0), iv(0.0), iv(1.0)),
        angles: SweepRange::from_zero(iv(TAU)),
    }
}

/// A strut at `at`: the sketch point `(2, 2)` extruded along
/// `(0.5, -1.5, 3)`.
fn strut_at(at: [f64; 3]) -> MappedCurve<Interval> {
    let [x, y, z] = at;
    MappedCurve::ExtrudedPoint {
        point: Point2::new(iv(2.0), iv(2.0)),
        place: Affine3::translation(Vec3::new(iv(x), iv(y), iv(z))),
        vec: Vec3::new(iv(0.5), iv(-1.5), iv(3.0)),
        stations: SweepRange::unit(),
    }
}

/// The widest of the samples `s = 0, ½, 1`.
fn sampled_width(c: &MappedCurve<Interval>) -> f64 {
    [0.0, 0.5, 1.0]
        .into_iter()
        .map(|s| point_width(c.eval(iv(s))))
        .fold(0.0, f64::max)
}

/// `sampled_width` of `c` after each of `0..=64` restrictions to
/// `[s0, s1]`, each applied to the previous one's result.
fn widths_by_split_count(mut c: MappedCurve<Interval>, (s0, s1): (f64, f64)) -> Vec<f64> {
    let mut widths = Vec::with_capacity(65);
    for _ in 0..=64 {
        widths.push(sampled_width(&c));
        c = c.restrict(iv(s0), iv(s1));
    }
    widths
}

/// The described point at `s = 0` is the placed sketch point, so **the
/// axis origin's own enclosure must not reach it** — at any axis width.
///
/// Three axis widths six orders apart, and the start sample must read
/// the same on all three: a spelling that mentioned the anchor twice
/// (`q − R·q`) read `2·width(axis_origin)` here, 4e-9 on the last row.
///
/// What is left is the rotation's diagonal enclosure at the exact angle
/// `0` — `t + c` on the axis component, `8.88e-16` wide — times the
/// placed point's `z = 3`: 2.6645352591003757e-15. Why that floor is
/// the backend's `cos` and not a spelling's is documented at
/// `Mat3::rotation_about`'s width-floor paragraph.
///
/// ε-free: enclosure widths only.
#[test]
fn the_revolved_anchor_contributes_no_width_at_the_start_sample() {
    let mut widths = [0.0f64; 3];
    for (row, half) in [0.0f64, 1.0e-12, 1.0e-9].into_iter().enumerate() {
        let at_start = point_width(rim(half).eval(iv(0.0)));
        println!("axis half-width {half:e}: eval(0) width {at_start:e}");
        widths[row] = at_start;
        assert!(
            at_start <= 1.0e-14,
            "the start sample of a revolved point on an axis of half-width \
             {half:e} is {at_start:e} wide — the described point there is the \
             placed sketch point, exact in this fixture up to the rotation's \
             own floor"
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

/// A restriction stores no anchored map, so the sub-curve's start
/// sample is as tight as the whole curve's — on a wide axis as on an
/// exact one.
///
/// A nonzero `s0` is measured too: there the axis width legitimately
/// reaches the answer (rotating about an uncertain axis genuinely
/// moves the point), so the bound is the honest `≈ 2·width(axis)` of a
/// quarter turn rather than zero (measured 4.000015429994619e-9).
#[test]
fn restriction_does_not_store_the_anchor_round_trip() {
    let half = 1.0e-9;
    let stored = point_width(rim(half).restrict(iv(0.0), iv(0.25)).eval(iv(0.0)));
    let whole = point_width(rim(half).eval(iv(0.0)));
    println!("restrict(0, 0.25).eval(0) width {stored:e} (unrestricted: {whole:e})");
    assert!(
        stored <= whole,
        "restricting from s0 = 0 widened the start sample from {whole:e} to \
         {stored:e} — the s0 = 0 restriction keeps the range's start exactly, so \
         nothing may enter it"
    );

    let real = point_width(rim(half).restrict(iv(0.25), iv(0.5)).eval(iv(0.0)));
    println!("restrict(0.25, 0.5).eval(0) width {real:e}");
    assert!(
        real <= 4.0 * (2.0 * half),
        "a quarter-turn advance about an axis of width {:e} gave {real:e}, \
         over the ≈ 2·width(axis) the geometry itself accounts for",
        2.0 * half,
    );
}

/// **A split costs the parameter's rounding, never a stored motion.**
/// Restrictions that keep an exact end of the range — `(0, ½)` keeps
/// its start and halves its end exactly, `(½, 1)` keeps its end — on a
/// metre-scale rim near the origin and on one a thousand metres out,
/// 64 times over: the stored width stays within 4× of one split's.
/// Composing each split's rotation into the stored placement re-paid
/// the rotation's diagonal enclosure times the coordinate scale per
/// split, growing linearly: 3.55e-15 per `(0, ½)` split near the origin
/// (2.3e-13 at 64 against 1.35e-14 after one), and 6.7e-11 at 64 far
/// out against 7.5e-12 after one.
///
/// Splits with neither end exact still round the composed angles a few
/// ulps per split, and the anchored rotation carries an angle's width
/// to the point times the coordinates' magnitude, so those grow; that
/// is recorded in `work/nurbs/revolved-point-eval-levers-angle-width-by-the-coordinates.md`
/// and not pinned here.
#[test]
fn end_anchored_splits_do_not_grow_the_stored_width() {
    for at in [[0.0, 0.0, 3.0], FAR] {
        for split in [(0.0, 0.5), (0.5, 1.0)] {
            let widths = widths_by_split_count(rim_at(at), split);
            println!(
                "{at:?} {split:?}: widths at 1, 8, 64 splits {:e}, {:e}, {:e}",
                widths[1], widths[8], widths[64]
            );
            for (n, &w) in widths.iter().enumerate().skip(1) {
                assert!(
                    w <= 4.0 * widths[1],
                    "at {at:?}, after {n} splits at {split:?} the stored width is \
                     {w:e}, against {:e} after one — repeated restriction is \
                     accumulating",
                    widths[1],
                );
            }
        }
    }
}

/// **A strut's split costs its stations' rounding, never the
/// coordinates'.** A strut a thousand metres out, split 64 times over
/// at six patterns — end-anchored, dyadic and not: the stored width
/// stays within 4× of the unsplit strut's (or of the coordinates' last
/// ulps, where the unsplit samples are exact). Composing each split's
/// translation into the stored placement rounded it at the coordinates'
/// scale once per split (1.1e-12 after 46 `(½, 1)` splits).
#[test]
fn a_far_strut_split_64_times_stays_at_its_unsplit_width() {
    let patterns = [
        (0.0, 0.5),
        (0.5, 1.0),
        (0.25, 0.75),
        (0.3, 0.7),
        (0.1, 0.9),
        (1.0 / 3.0, 2.0 / 3.0),
    ];
    // A sample a thousand metres out rounds to its coordinates' last
    // ulp, one step each way, whatever the description: the unsplit
    // samples happen to be exact, and a split one is not.
    let floor = 2.0 * (1000f64.next_up() - 1000.0);
    for split in patterns {
        let widths = widths_by_split_count(strut_at(FAR), split);
        let unsplit = widths[0].max(floor);
        println!(
            "strut {split:?}: unsplit {:e}, worst over 64 splits {:e}",
            widths[0],
            widths.iter().copied().fold(0.0, f64::max)
        );
        for (n, &w) in widths.iter().enumerate() {
            assert!(
                w <= 4.0 * unsplit,
                "the strut split {n} times at {split:?} is {w:e} wide against \
                 {unsplit:e} unsplit — a split is charging the coordinates' scale, \
                 which only a composition into the placement does"
            );
        }
    }
}

/// The restriction contract, `restrict(s0, s1).eval(s) = eval(s0 +
/// (s1 − s0)·s)`, read where the exact answer is known: angles at whole
/// quarter and third turns of the metre rim, stations at a half and a
/// third of the strut. At `Interval` each restricted enclosure must
/// CONTAIN the exact point — split parameters that are themselves
/// enclosures (`⅓` as `iv(1)/iv(3)`) included — and at `f64` nested
/// non-dyadic restrictions must sit on the direct evaluation.
#[test]
fn a_restriction_is_the_sub_range_of_the_same_trajectory() {
    let third = iv(1.0) / iv(3.0);
    let holds = |c: Interval, v: f64| c.lo() <= v && v <= c.hi();

    // The rim: placed point (2, 2, 3), axis through (1, 2, 3) along +z.
    let rim = rim_at([0.0, 0.0, 3.0]);
    // [¼, ¾], then its [½, 1]: angles [π, 3π/2].
    let quarters = rim.restrict(iv(0.25), iv(0.75)).restrict(iv(0.5), iv(1.0));
    for (s, x, y) in [(0.0, 0.0, 2.0), (1.0, 1.0, 1.0)] {
        let e = quarters.eval(iv(s));
        assert!(
            holds(e.x, x) && holds(e.y, y) && holds(e.z, 3.0),
            "the rim restricted to [π, 3π/2] at s = {s} misses ({x}, {y}, 3): {e:?}"
        );
    }
    // [⅓, ⅔]: angles [2π/3, 4π/3], where x = 1 + cos θ = ½ at both ends.
    let thirds = rim.restrict(third, iv(2.0) * third);
    for s in [0.0, 1.0] {
        let e = thirds.eval(iv(s));
        assert!(
            holds(e.x, 0.5) && holds(e.z, 3.0),
            "the rim restricted to [2π/3, 4π/3] at s = {s} misses x = ½, z = 3: {e:?}"
        );
    }

    // The strut: (2, 2, 0) + (0.5, -1.5, 3)·t. [⅓, ⅔], then its [½, 1]:
    // t ∈ [½, ⅔], whose ends are (2.25, 1.25, 1.5) and (2⅓, 1, 2).
    let strut = strut_at([0.0, 0.0, 0.0])
        .restrict(third, iv(2.0) * third)
        .restrict(iv(0.5), iv(1.0));
    let start = strut.eval(iv(0.0));
    assert!(
        holds(start.x, 2.25) && holds(start.y, 1.25) && holds(start.z, 1.5),
        "the strut restricted to t ∈ [½, ⅔] starts off (2.25, 1.25, 1.5): {start:?}"
    );
    let end = strut.eval(iv(1.0));
    assert!(
        holds(end.y, 1.0) && holds(end.z, 2.0) && end.x.lo() < 2.34 && 2.33 < end.x.hi(),
        "the strut restricted to t ∈ [½, ⅔] ends off (2⅓, 1, 2): {end:?}"
    );

    let rim64 = MappedCurve::RevolvedPoint {
        point: Point2::new(2.0, 2.0),
        place: Affine3::translation(Vec3::new(0.0, 0.0, 3.0)),
        axis_origin: Point3::new(1.0, 2.0, 3.0),
        axis_dir: Vec3::new(0.0, 0.0, 1.0),
        angles: SweepRange::from_zero(TAU),
    };
    let strut64 = MappedCurve::ExtrudedPoint {
        point: Point2::new(2.0, 2.0),
        place: Affine3::translation(Vec3::new(FAR[0], FAR[1], FAR[2])),
        vec: Vec3::new(0.5, -1.5, 3.0),
        stations: SweepRange::unit(),
    };
    for (name, c) in [("rim", rim64), ("strut", strut64)] {
        // [0.3, 0.7], then its [0.1, 0.6]: [0.34, 0.54].
        let sub = c.restrict(0.3, 0.7).restrict(0.1, 0.6);
        for s in [0.0, 0.25, 0.5, 1.0] {
            let d = (sub.eval(s) - c.eval(0.34 + 0.2 * s)).norm_inf();
            assert!(
                d <= 1.0e-12,
                "the {name} restricted to [0.34, 0.54] at s = {s} is {d:e} off the \
                 direct evaluation"
            );
        }
    }
}

/// The full-period sample, which revolve seams land on: `s = 1` at
/// `angle = 2π` describes the start point again. The anchor operator's
/// half-angle factors (`2·sin²(π)`, `2·sin(π)·cos(π)`) are near zero
/// there, so the sample reads the start enclosure's own floor plus the
/// axis's contribution over a full turn (measured 2.66e-15); a
/// `1 − cos 2π` spelling paid 4.0e-9 on this fixture.
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
        w <= 2.7e-14,
        "the full-period sample is {w:e} wide — the anchor operator holds it \
         to ~2.66e-15 here; a width in the 1e-9 range means the seam angle has \
         gone back to being paid as an ulp-of-1 cancellation"
    );
}
