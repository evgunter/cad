//! The swept-point descriptions' anchor and restriction, pinned at the
//! certified scalar.
//!
//! `MappedCurve::RevolvedPoint` evaluates through
//! `Affine3::rotation_about_axis(axis_origin, axis_dir, range.at(s)·angle)`,
//! and `restrict` narrows `range`, a sub-range of the whole sweep's
//! normalized parameter, while the stored placement stays as built. So
//! the description pays for its rotation once per evaluation and never
//! per split. `ExtrudedPoint` restricts its `range` the same way.
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
use geom_core::{Affine3, Bounds, Interval, Point2, Point3, Real, Vec3};

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
        range: SweepRange::whole(),
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
        angle: iv(TAU),
        range: SweepRange::whole(),
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
        range: SweepRange::whole(),
    }
}

/// The widest of the samples `s = 0, ½, 1`.
fn sampled_width(c: &MappedCurve<Interval>) -> f64 {
    [0.0, 0.5, 1.0]
        .into_iter()
        .map(|s| point_width(c.eval(iv(s))))
        .fold(0.0, f64::max)
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

/// One split of a chain, as `restrict`'s two parameters.
type Split = (Interval, Interval);

/// A split parameter an intersection hands `restrict`, non-dyadic and
/// varying along the chain: `0.37 … 0.414`.
fn crossing(k: usize) -> f64 {
    #[allow(clippy::cast_precision_loss)]
    let wobble = ((k * 7) % 5) as f64;
    0.37 + 0.011 * wobble
}

/// The split chains the width rows drive, by name. The enclosure
/// chains hand `restrict` what a caller derives from a carrier
/// parameter `t` — `(t − t0)/span`, an enclosure of its own.
fn chain(name: &str, k: usize) -> Split {
    let a = crossing(k);
    let fuzzy = Interval::from_bounds(a - 1e-13, a + 1e-13);
    let quotient = iv(a * 3.0) / iv(3.0);
    let alternate = |x: Interval| {
        if k.is_multiple_of(2) {
            (x, iv(1.0))
        } else {
            (iv(0.0), x)
        }
    };
    match name {
        "(0, 1/2)" => (iv(0.0), iv(0.5)),
        "(1/2, 1)" => (iv(0.5), iv(1.0)),
        "(0, a)" => (iv(0.0), iv(a)),
        "(a, 1)" => (iv(a), iv(1.0)),
        "(1/4, 3/4)" => (iv(0.25), iv(0.75)),
        "(0, a ± 1e-13)" => (iv(0.0), fuzzy),
        "(0.3, 0.7)" => (iv(0.3), iv(0.7)),
        "(0.3, 0.7) as quotients" => (iv(0.9) / iv(3.0), iv(2.1) / iv(3.0)),
        "alternate (a, 1) / (0, a)" => alternate(iv(a)),
        "alternate, a = t/span" => alternate(quotient),
        "alternate, a ± 1e-13" => alternate(fuzzy),
        "(a ± 1e-13, 1)" => (fuzzy, iv(1.0)),
        _ => unreachable!("an unnamed chain"),
    }
}

/// `sampled_width` of `c` before any split and after each of 64, the
/// `k`-th split `split(k)` applied to the previous one's result.
fn widths_along(mut c: MappedCurve<Interval>, split: impl Fn(usize) -> Split) -> Vec<f64> {
    let mut widths = Vec::with_capacity(65);
    widths.push(sampled_width(&c));
    for k in 0..64 {
        let (s0, s1) = split(k);
        c = c.restrict(s0, s1);
        widths.push(sampled_width(&c));
    }
    widths
}

/// The near rim, a metre off an axis through `(1, 2, 3)`.
const NEAR: [f64; 3] = [0.0, 0.0, 3.0];

/// **A split chain's stored width stays under a ceiling 2.5× what this
/// form measures**, for every chain at both placements, at every count
/// up to 64: each ceiling is 2.5× the worst width over its chain,
/// rounded up. A range stored as its two ends and interpolated breaks
/// one; an eval that rotates by the range's start and then by its span
/// stays under them and breaks
/// [`restriction_is_no_wider_than_composing_into_the_placement`].
#[test]
fn restricted_widths_stay_under_their_ceilings() {
    let rows: [([f64; 3], &str, f64); 24] = [
        (NEAR, "(0, 1/2)", 2.7e-14),
        (NEAR, "(1/2, 1)", 2.8e-13),
        (NEAR, "(0, a)", 4.9e-14),
        (NEAR, "(a, 1)", 9.4e-13),
        (NEAR, "(1/4, 3/4)", 1.5e-13),
        (NEAR, "(0.3, 0.7)", 5.7e-13),
        (NEAR, "(0.3, 0.7) as quotients", 6.1e-13),
        (NEAR, "alternate (a, 1) / (0, a)", 4.7e-13),
        (NEAR, "alternate, a = t/span", 4.7e-13),
        (NEAR, "alternate, a ± 1e-13", 4.4e-11),
        (NEAR, "(a ± 1e-13, 1)", 6.8e-11),
        (NEAR, "(0, a ± 1e-13)", 1.8e-11),
        (FAR, "(0, 1/2)", 1.6e-11),
        (FAR, "(1/2, 1)", 1.4e-10),
        (FAR, "(0, a)", 2.6e-11),
        (FAR, "(a, 1)", 4.7e-10),
        (FAR, "(1/4, 3/4)", 7.4e-11),
        (FAR, "(0.3, 0.7)", 2.9e-10),
        (FAR, "(0.3, 0.7) as quotients", 3.1e-10),
        (FAR, "alternate (a, 1) / (0, a)", 2.4e-10),
        (FAR, "alternate, a = t/span", 2.4e-10),
        (FAR, "alternate, a ± 1e-13", 2.2e-8),
        (FAR, "(a ± 1e-13, 1)", 3.4e-8),
        (FAR, "(0, a ± 1e-13)", 8.6e-9),
    ];
    for (at, name, ceiling) in rows {
        let widths = widths_along(rim_at(at), |k| chain(name, k));
        let worst = widths.iter().copied().fold(0.0, f64::max);
        println!("{at:?} {name}: worst over 64 splits {worst:e} (ceiling {ceiling:e})");
        for (n, &w) in widths.iter().enumerate() {
            assert!(
                w <= ceiling,
                "at {at:?}, after {n} splits of {name} the stored width is {w:e}, over \
                 {ceiling:e}"
            );
        }
    }
}

/// **A chain anchored at either end stays flat.** In the normalized
/// parameter a dyadic split is exact, so `(0, ½)` and `(½, 1)` store no
/// rounding at all, and `(0, a)` only scales the span: every count up to
/// 52 stays within 1.5× of the widest of the unsplit rim and its first
/// split. Past 52 halvings `(½, 1)`'s start `1 − 2⁻ⁿ` is no longer an
/// `f64`, and the range — `2π·2⁻⁵³` of turn — rounds.
#[test]
fn end_anchored_chains_stay_flat() {
    for at in [NEAR, FAR] {
        for name in ["(0, 1/2)", "(1/2, 1)", "(0, a)"] {
            let widths = widths_along(rim_at(at), |k| chain(name, k));
            let base = widths[0].max(widths[1]);
            let worst = widths[..=52].iter().copied().fold(0.0, f64::max);
            println!("{at:?} {name}: unsplit/first {base:e}, worst to 52 {worst:e}");
            for (n, &w) in widths[..=52].iter().enumerate() {
                assert!(
                    w <= 1.5 * base,
                    "at {at:?}, after {n} splits of {name} the stored width is {w:e} \
                     against {base:e} unsplit or split once — an end-anchored chain grew"
                );
            }
        }
    }
}

/// Columns of a 3×3 linear map and a translation.
type Pose = ([[Interval; 3]; 3], [Interval; 3]);

/// `cols · v`, as `(c0·x + c1·y) + c2·z` per component.
fn apply(cols: [[Interval; 3]; 3], v: [Interval; 3]) -> [Interval; 3] {
    core::array::from_fn(|i| cols[0][i] * v[0] + cols[1][i] * v[1] + cols[2][i] * v[2])
}

/// The rotation by `theta` about `+z` through `q`, written out: the
/// linear part from `cos`/`sin` with the axis entry `(1 − c) + c`, and
/// the translation `(I − R)·q` from the half-angle factors `2 sin²(θ/2)`
/// and `2 sin(θ/2) cos(θ/2)`.
fn turn_z(theta: Interval, q: [Interval; 3]) -> Pose {
    let zero = iv(0.0);
    let (s, c) = theta.sin_cos();
    let (hs, hc) = (theta * iv(0.5)).sin_cos();
    let (s2, t2) = (iv(2.0) * hs * hc, iv(2.0) * hs.powi(2));
    (
        [[c, s, zero], [-s, c, zero], [zero, zero, (iv(1.0) - c) + c]],
        [t2 * q[0] + s2 * q[1], -(s2 * q[0]) + t2 * q[1], zero],
    )
}

/// `a` after `b`.
fn compose((al, at): Pose, (bl, bt): Pose) -> Pose {
    let t = apply(al, bt);
    (
        core::array::from_fn(|j| apply(al, bl[j])),
        core::array::from_fn(|i| t[i] + at[i]),
    )
}

/// `rim_at(at)` restricted by composing each split's start rotation
/// into the stored placement and scaling the angle — the restriction
/// this form replaced — spelled out by hand for the `+z` axis, so it
/// shares no code with `MappedCurve`, `SweepRange` or
/// `Affine3::rotation_about_axis`.
#[derive(Clone, Copy)]
struct Composed {
    place: Pose,
    q: [Interval; 3],
    angle: Interval,
}

impl Composed {
    fn at(at: [f64; 3]) -> Self {
        let (zero, one) = (iv(0.0), iv(1.0));
        Composed {
            place: (
                [[one, zero, zero], [zero, one, zero], [zero, zero, one]],
                [iv(at[0]), iv(at[1]), iv(at[2])],
            ),
            q: [iv(1.0 + at[0]), iv(2.0 + at[1]), iv(at[2])],
            angle: iv(TAU),
        }
    }

    fn restrict(self, (s0, s1): Split) -> Self {
        Composed {
            place: compose(turn_z(s0 * self.angle, self.q), self.place),
            angle: (s1 - s0) * self.angle,
            ..self
        }
    }

    fn sampled_width(self) -> f64 {
        let (lin, t) = self.place;
        let p = apply(lin, [iv(2.0), iv(2.0), iv(0.0)]);
        let placed: [Interval; 3] = core::array::from_fn(|i| p[i] + t[i]);
        [0.0, 0.5, 1.0]
            .into_iter()
            .map(|s| {
                let (rl, rt) = turn_z(iv(s) * self.angle, self.q);
                let r = apply(rl, placed);
                (0..3).map(|i| width(r[i] + rt[i])).fold(0.0, f64::max)
            })
            .fold(0.0, f64::max)
    }
}

/// **No chain is wider than composing its splits into the placement**,
/// at either placement and every count up to 64, against the composed
/// restriction spelled out by hand ([`Composed`]). End-anchored and
/// dyadic chains stay at or under it; chains whose start moves by an
/// inexact amount each split stay within 1.25×. Those pay one outward
/// rounding of the start per split, at the start's own ulp; the
/// anchored rotation carries that to the point at the coordinates'
/// scale, as it carries the composed placement's per-split rotation
/// (`work/nurbs/revolved-point-eval-levers-angle-width-by-the-coordinates.md`):
/// `(0.3, 0.7)` far reaches 1.06× and its quotient form 1.03×.
///
/// Each row's allowance is the lesser of that bar and 1.2× the ratio
/// this form measures, so a chain that sits well under the composed
/// rim must stay there: an eval that rotated by the range's start and
/// then by its span pays the composed rim's width again, and reads
/// 0.97–1.0 on chains this form holds at 0.5–0.7.
#[test]
fn restriction_is_no_wider_than_composing_into_the_placement() {
    let rows: [([f64; 3], &str, f64); 24] = [
        (NEAR, "(0, 1/2)", 0.95),
        (NEAR, "(1/2, 1)", 0.71),
        (NEAR, "(0, a)", 1.00),
        (NEAR, "(a, 1)", 0.94),
        (NEAR, "(1/4, 3/4)", 0.58),
        (NEAR, "(0.3, 0.7)", 0.69),
        (NEAR, "(0.3, 0.7) as quotients", 0.87),
        (NEAR, "alternate (a, 1) / (0, a)", 0.93),
        (NEAR, "alternate, a = t/span", 0.87),
        (NEAR, "alternate, a ± 1e-13", 1.16),
        (NEAR, "(a ± 1e-13, 1)", 0.80),
        (NEAR, "(0, a ± 1e-13)", 1.00),
        (FAR, "(0, 1/2)", 1.00),
        (FAR, "(1/2, 1)", 0.74),
        (FAR, "(0, a)", 1.00),
        (FAR, "(a, 1)", 1.12),
        (FAR, "(1/4, 3/4)", 0.60),
        (FAR, "(0.3, 0.7)", 1.25),
        (FAR, "(0.3, 0.7) as quotients", 1.24),
        (FAR, "alternate (a, 1) / (0, a)", 1.20),
        (FAR, "alternate, a = t/span", 1.14),
        (FAR, "alternate, a ± 1e-13", 1.18),
        (FAR, "(a ± 1e-13, 1)", 0.83),
        (FAR, "(0, a ± 1e-13)", 1.00),
    ];
    for (at, name, allowed) in rows {
        let ours = widths_along(rim_at(at), |k| chain(name, k));
        let mut theirs = Composed::at(at);
        let mut worst: f64 = 0.0;
        for (k, &w) in ours.iter().enumerate().skip(1) {
            theirs = theirs.restrict(chain(name, k - 1));
            worst = worst.max(w / theirs.sampled_width());
        }
        println!("{at:?} {name}: worst ratio to the composed rim {worst} (allowed {allowed})");
        assert!(
            worst <= allowed,
            "at {at:?}, {name} reaches {worst}× the composed rim, over {allowed}×"
        );
    }
}

/// **A whole range evaluates as the unrestricted rotation, bit for
/// bit**, at `f64` and at `Interval`, at every sample and placement:
/// `SweepRange::whole().at(s)` is `s` itself, so the angle is `s·angle`
/// as it always was and nothing a body builds unrestricted moves.
/// Struts likewise read `vec·s`.
#[test]
fn a_whole_range_evaluates_as_the_unrestricted_motion_bit_for_bit() {
    let mut mismatches = Vec::new();
    for angle in [TAU, -TAU, 1.0, -1.0, 1e-8, -0.3, core::f64::consts::PI] {
        for at in [[0.0, 0.0, 0.0], [1000.0, -700.0, 300.0], [-3.0, 0.0, 0.0]] {
            let place = Affine3::translation(Vec3::new(at[0], at[1], at[2]));
            let q = Point3::new(at[0] + 1.0, at[1] + 2.0, at[2]);
            let n = Vec3::new(0.3, -0.2, 1.0);
            let pt = Point2::new(2.0, 2.0);
            let c = MappedCurve::RevolvedPoint {
                point: pt,
                place,
                axis_origin: q,
                axis_dir: n,
                angle,
                range: SweepRange::whole(),
            };
            let ci = MappedCurve::RevolvedPoint {
                point: Point2::new(iv(2.0), iv(2.0)),
                place: Affine3::translation(Vec3::new(iv(at[0]), iv(at[1]), iv(at[2]))),
                axis_origin: Point3::new(iv(q.x), iv(q.y), iv(q.z)),
                axis_dir: Vec3::new(iv(n.x), iv(n.y), iv(n.z)),
                angle: iv(angle),
                range: SweepRange::whole(),
            };
            for i in 0..=64 {
                let s = if i == 7 {
                    1.0 / 3.0
                } else {
                    f64::from(i) / 64.0
                };
                let p = place.transform_point(Point3::new(pt.x, pt.y, 0.0));
                let want = Affine3::rotation_about_axis(q, n, s * angle).transform_point(p);
                let got = c.eval(s);
                for (a, b) in [(got.x, want.x), (got.y, want.y), (got.z, want.z)] {
                    if a.to_bits() != b.to_bits() {
                        mismatches
                            .push(format!("f64 angle {angle} at {at:?} s {s}: {a:e} vs {b:e}"));
                    }
                }
                let si = if i == 7 { iv(1.0) / iv(3.0) } else { iv(s) };
                let pi = Affine3::translation(Vec3::new(iv(at[0]), iv(at[1]), iv(at[2])))
                    .transform_point(Point3::new(iv(2.0), iv(2.0), iv(0.0)));
                let want = Affine3::rotation_about_axis(
                    Point3::new(iv(q.x), iv(q.y), iv(q.z)),
                    Vec3::new(iv(n.x), iv(n.y), iv(n.z)),
                    si * iv(angle),
                )
                .transform_point(pi);
                let got = ci.eval(si);
                for (a, b) in [(got.x, want.x), (got.y, want.y), (got.z, want.z)] {
                    if a.lo().to_bits() != b.lo().to_bits() || a.hi().to_bits() != b.hi().to_bits()
                    {
                        mismatches.push(format!("Interval angle {angle} at {at:?} s {s}"));
                    }
                }
            }
            let v = Vec3::new(0.5, -1.5, angle);
            let strut = MappedCurve::ExtrudedPoint {
                point: pt,
                place,
                vec: v,
                range: SweepRange::whole(),
            };
            for i in 0..=16 {
                let s = f64::from(i) / 16.0;
                let got = strut.eval(s);
                let want = place.transform_point(Point3::new(pt.x, pt.y, 0.0)) + v * s;
                for (a, b) in [(got.x, want.x), (got.y, want.y), (got.z, want.z)] {
                    if a.to_bits() != b.to_bits() {
                        mismatches.push(format!("strut {v:?} at {at:?} s {s}: {a:e} vs {b:e}"));
                    }
                }
            }
        }
    }
    assert!(
        mismatches.is_empty(),
        "{} mismatches: {:?}",
        mismatches.len(),
        &mismatches[..mismatches.len().min(10)]
    );
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
    for (s0, s1) in patterns {
        let widths = widths_along(strut_at(FAR), |_| (iv(s0), iv(s1)));
        let unsplit = widths[0].max(floor);
        println!(
            "strut ({s0}, {s1}): unsplit {:e}, worst over 64 splits {:e}",
            widths[0],
            widths.iter().copied().fold(0.0, f64::max)
        );
        for (n, &w) in widths.iter().enumerate() {
            assert!(
                w <= 4.0 * unsplit,
                "the strut split {n} times at ({s0}, {s1}) is {w:e} wide against \
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
        angle: TAU,
        range: SweepRange::whole(),
    };
    let strut64 = MappedCurve::ExtrudedPoint {
        point: Point2::new(2.0, 2.0),
        place: Affine3::translation(Vec3::new(FAR[0], FAR[1], FAR[2])),
        vec: Vec3::new(0.5, -1.5, 3.0),
        range: SweepRange::whole(),
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
