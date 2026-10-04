//! **A cylinder wall crossing a sphere off the cylinder's axis**: the
//! germ pair's section is a space quartic, one loop or two, and the
//! join's matcher reads it through the transverse frame
//! (`topo::boolean::join`'s `cs_transverse_frame`): a centre and an axis
//! each loop turns about once.
//!
//! Two families of poses, each under ∪, ∩ and both differences in both
//! operand orders:
//!
//! - a ball against a radius-0.5 drum `z ∈ [−1, 1]`, clear of its caps,
//!   crossing the wall in one loop (its reach short of the far side) or
//!   two (past it), with the ball's chart turned off the axes on some;
//! - the row's own poses (`work/join/cylinder-sphere-germ-pair-has-no-section-frame.md`):
//!   the tilted drum cut's lower part against a ball straddling the rim.
//!
//! Each run builds a body that is SOUND by
//! `common::differential::outcome` (tiers 2 and 3′, the certificate, a
//! legal operand, the volume) or refuses at the join's lane door for the
//! cylinder × sphere pair, `CurvedBooleanUnsupported` naming the wall or
//! the sphere (`work/join/cylinder-sphere-germ-pair-has-no-join-lane.md`).
//! The matched segments are checked against the analytic loops, where
//! every segment joins two sites that are neighbours along their loop.
//!
//! The volumes are not elementary — a ball against an off-axis cylinder
//! is an elliptic integral — so [`oracle`] integrates the closed-form area
//! of each slice, a disc against a convex polygon
//! (`common::differential::disc_clip_area`), by adaptive Gauss–Kronrod,
//! and [`the_oracle_agrees_with_a_second_slicing`] checks it against the
//! slicing along the cylinder's axis, whose slices are lenses.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use crate::common::differential::{clip_convex, disc_clip_area, outcome};
use crate::common::germ_pair::cyl;
use crate::conic_edge_curved_face::{DRUM_RADIUS, TILT, ball, drum_lower};
use geom_core::{Affine3, Point3, Tol, Vec3};
use topo::{Body, BooleanError, BooleanOp};

const R: f64 = 0.5;

/// The solid the ball is cut against: the cylinder of radius [`R`] about
/// `z` over `z ∈ [z0, z1]`, below the plane through `(0, 0, cut.0)` with
/// normal `(sin cut.1, 0, cos cut.1)` where there is a cut.
#[derive(Clone, Copy)]
struct Drum {
    z0: f64,
    z1: f64,
    cut: Option<(f64, f64)>,
}

const FULL: Drum = Drum {
    z0: -1.0,
    z1: 1.0,
    cut: None,
};

const LOWER: Drum = Drum {
    z0: 0.0,
    z1: 1.0,
    cut: Some((0.5, TILT)),
};

/// `∫ f` over `[a, b]` to `tol`, by adaptive 7–15 Gauss–Kronrod.
fn gk(f: &dyn Fn(f64) -> f64, a: f64, b: f64, tol: f64, depth: u32) -> f64 {
    const XK: [f64; 8] = [
        0.991_455_371_120_812_6,
        0.949_107_912_342_758_5,
        0.864_864_423_359_769_1,
        0.741_531_185_599_394_4,
        0.586_087_235_467_691_1,
        0.405_845_151_377_397_2,
        0.207_784_955_007_898_5,
        0.0,
    ];
    const WK: [f64; 8] = [
        0.022_935_322_010_529_2,
        0.063_092_092_629_979_0,
        0.104_790_010_322_250_2,
        0.140_653_259_715_525_9,
        0.169_004_726_639_267_9,
        0.190_350_578_064_785_4,
        0.204_432_940_075_298_9,
        0.209_482_141_084_728,
    ];
    const WG: [f64; 4] = [
        0.129_484_966_168_869_7,
        0.279_705_391_489_276_7,
        0.381_830_050_505_118_9,
        0.417_959_183_673_469_4,
    ];
    let (c, h) = ((a + b) / 2.0, (b - a) / 2.0);
    let fc = f(c);
    let (mut k, mut g) = (WK[7] * fc, WG[3] * fc);
    for i in 0..7 {
        let s = f(c - h * XK[i]) + f(c + h * XK[i]);
        k += WK[i] * s;
        if i % 2 == 1 {
            g += WG[i / 2] * s;
        }
    }
    let (k, g) = (k * h, g * h);
    if (k - g).abs() <= tol || depth == 0 {
        k
    } else {
        gk(f, a, c, tol / 2.0, depth - 1) + gk(f, c, b, tol / 2.0, depth - 1)
    }
}

/// **The oracle**: the volume of the ball of radius `big` about `c`
/// inside `drum`. Sliced at `y`, the ball is a disc about `(c.x, c.z)` of
/// radius `√(big² − (y − c.y)²)` and the drum the convex polygon
/// `|x| ≤ √(R² − y²)`, `z0 ≤ z ≤ z1`, below the cut's line.
fn oracle(big: f64, c: [f64; 3], drum: Drum) -> f64 {
    let slice = |y: f64| {
        let w2 = R * R - y * y;
        let rho2 = big * big - (y - c[1]) * (y - c[1]);
        if w2 <= 0.0 || rho2 <= 0.0 {
            return 0.0;
        }
        let w = w2.sqrt();
        let mut poly = vec![(-w, drum.z0), (w, drum.z0), (w, drum.z1), (-w, drum.z1)];
        if let Some((h, t)) = drum.cut {
            // Below the cut: `x·sin t + (z − h)·cos t ≤ 0`, a CCW half plane.
            let (s, co) = (t.sin(), t.cos());
            let big_l = 10.0;
            let (dx, dz) = (co, -s);
            let half = vec![
                (-dx * big_l, h - dz * big_l),
                (dx * big_l, h + dz * big_l),
                (dx * big_l - s * big_l, h + dz * big_l - co * big_l),
                (-dx * big_l - s * big_l, h - dz * big_l - co * big_l),
            ];
            poly = clip_convex(&poly, &crate::common::differential::ccw(half));
        }
        if poly.len() < 3 {
            return 0.0;
        }
        let moved: Vec<(f64, f64)> = poly.iter().map(|&(x, z)| (x - c[0], z - c[2])).collect();
        disc_clip_area(rho2.sqrt(), &moved).abs()
    };
    let lo = (-R).max(c[1] - big);
    let hi = R.min(c[1] + big);
    if lo >= hi {
        return 0.0;
    }
    let n = 64;
    (0..n)
        .map(|k| {
            let a = lo + (hi - lo) * f64::from(k) / f64::from(n);
            let b = lo + (hi - lo) * f64::from(k + 1) / f64::from(n);
            gk(&slice, a, b, 1e-15, 40)
        })
        .sum()
}

/// The area of the lens of two discs of radii `a` and `b` whose centres
/// are `d` apart.
fn lens(a: f64, b: f64, d: f64) -> f64 {
    if d >= a + b {
        return 0.0;
    }
    if d <= (a - b).abs() {
        return PI * a.min(b).powi(2);
    }
    let ca = ((d * d + a * a - b * b) / (2.0 * d * a)).clamp(-1.0, 1.0);
    let cb = ((d * d + b * b - a * a) / (2.0 * d * b)).clamp(-1.0, 1.0);
    let k = ((-d + a + b) * (d + a - b) * (d - a + b) * (d + a + b)).max(0.0);
    a * a * ca.acos() + b * b * cb.acos() - k.sqrt() / 2.0
}

/// A ball about `c` against an uncut drum, sliced along the cylinder's
/// axis: each slice a lens of the ball's disc and the cylinder's.
fn oracle_along_the_axis(big: f64, c: [f64; 3], drum: Drum) -> f64 {
    let d = c[0].hypot(c[1]);
    let slice = |z: f64| {
        let rho2 = big * big - (z - c[2]) * (z - c[2]);
        if rho2 <= 0.0 {
            0.0
        } else {
            lens(rho2.sqrt(), R, d)
        }
    };
    let lo = drum.z0.max(c[2] - big);
    let hi = drum.z1.min(c[2] + big);
    let n = 64;
    (0..n)
        .map(|k| {
            let a = lo + (hi - lo) * f64::from(k) / f64::from(n);
            let b = lo + (hi - lo) * f64::from(k + 1) / f64::from(n);
            gk(&slice, a, b, 1e-15, 40)
        })
        .sum()
}

/// One pose: a ball of radius `big` about `c`, its chart turned `spin`
/// radians about `(1, 2, 3)` through its centre, against `drum`.
struct Pose {
    label: &'static str,
    big: f64,
    c: [f64; 3],
    spin: f64,
    drum: Drum,
}

fn poses() -> Vec<Pose> {
    let full = |label, big, c, spin| Pose {
        label,
        big,
        c,
        spin,
        drum: FULL,
    };
    vec![
        full("one loop, centre on the wall", 0.3, [0.5, 0.0, 0.0], 0.0),
        full("one loop, centre inside", 0.3, [0.35, 0.0, 0.1], 0.0),
        full("one loop, centre outside", 0.3, [0.7, 0.1, -0.2], 0.0),
        full("one loop, turned chart", 0.25, [0.3, -0.35, 0.4], 0.9),
        // Wider than the wall, short of its far side: `r < R < r + d`.
        full(
            "one loop, ball wider than the wall",
            0.699,
            [0.2, 0.0, 0.0],
            0.0,
        ),
        full("two loops", 0.7, [0.1, 0.0, 0.0], 0.0),
        full("two loops, turned chart", 0.8, [0.2, 0.1, 0.05], 0.6),
        Pose {
            label: "the row's ball r 0.2 across the rim",
            big: 0.2,
            c: [0.5, 0.0, 0.35],
            spin: 0.0,
            drum: LOWER,
        },
        Pose {
            label: "the row's ball r 0.1 across the rim",
            big: 0.1,
            c: [0.45, 0.0, 0.3],
            spin: 0.0,
            drum: LOWER,
        },
    ]
}

impl Pose {
    fn operands(&self) -> (Body<f64>, Body<f64>) {
        let a = match self.drum.cut {
            None => cyl(R, self.drum.z1),
            Some(_) => drum_lower(),
        };
        let mut b = ball(self.big, self.c);
        if self.spin != 0.0 {
            let turn = Affine3::rotation_about_axis(
                Point3::from_array(self.c),
                Vec3::new(1.0, 2.0, 3.0).normalize(),
                self.spin,
            );
            b = topo::transform_rigid(&b, &turn, Tol::witness()).unwrap();
        }
        (a, b)
    }

    /// The drum's volume, the ball's, and their common part's.
    fn volumes(&self) -> (f64, f64, f64) {
        let va = match self.drum.cut {
            None => PI * R * R * (self.drum.z1 - self.drum.z0),
            Some(_) => PI * DRUM_RADIUS.powi(2) * 0.5,
        };
        (
            va,
            4.0 / 3.0 * PI * self.big.powi(3),
            oracle(self.big, self.c, self.drum),
        )
    }
}

/// The six runs of a pose: label, op, the two operands in order, and the
/// closed-form volume of the result.
fn runs(va: f64, vb: f64, vi: f64) -> [(&'static str, BooleanOp, bool, f64); 6] {
    [
        ("A ∪ B", BooleanOp::Union, false, va + vb - vi),
        ("B ∪ A", BooleanOp::Union, true, va + vb - vi),
        ("A ∩ B", BooleanOp::Intersect, false, vi),
        ("B ∩ A", BooleanOp::Intersect, true, vi),
        ("A ∖ B", BooleanOp::Subtract, false, va - vi),
        ("B ∖ A", BooleanOp::Subtract, true, vb - vi),
    ]
}

fn run(
    op: BooleanOp,
    x: &Body<f64>,
    y: &Body<f64>,
) -> Result<topo::BooleanResult<f64>, BooleanError> {
    let tol = Tol::witness();
    match op {
        BooleanOp::Union => topo::union(x, y, tol),
        BooleanOp::Intersect => topo::intersect(x, y, tol),
        BooleanOp::Subtract => topo::subtract(x, y, tol),
    }
}

/// The join's lane door for the cylinder × sphere germ pair: no chord lane
/// for its section, naming the wall or the sphere.
fn at_the_lane_door(e: &BooleanError) -> bool {
    matches!(
        e,
        BooleanError::CurvedBooleanUnsupported {
            kind: geom::SurfaceKind::Cylinder | geom::SurfaceKind::Sphere,
            ..
        }
    )
}

/// **Every run builds SOUND or stops at the lane door**, never at the
/// germ frame, in the join's own kernel-bug words, or on a wrong body.
/// Each run's `outcome` line is printed, so the rows double as a
/// battery.
#[test]
fn a_ball_across_a_cylinder_wall_builds_sound_or_stops_at_the_lane() {
    let tol = Tol::witness();
    for pose in poses() {
        let (a, b) = pose.operands();
        let (va, vb, vi) = pose.volumes();
        for (op_label, op, swapped, want) in runs(va, vb, vi) {
            let label = format!("{}, {op_label}", pose.label);
            let (x, y) = if swapped { (&b, &a) } else { (&a, &b) };
            let got = run(op, x, y);
            if let Err(e) = &got {
                assert!(at_the_lane_door(e), "{label}: refused {e:?}");
            }
            let line = outcome(got, want, tol);
            println!("{label}: {line}");
            assert!(
                line.starts_with("ERR") || line.contains("SOUND") || line == "EMPTY ok",
                "{label}: {line}"
            );
        }
    }
}

/// A site's place on its loop of the analytic section, read off the
/// cylinder's chart without the frame: which loop, where along it, and
/// the loop's period. On one loop, `τ` runs up the side `z > c.z` over
/// `θ ∈ (−θ₀, θ₀)` and back down the other; on two, each loop is one
/// side and `τ = θ`.
fn place(pose: &Pose, p: Point3<f64>) -> (i8, f64, f64) {
    let d = pose.c[0].hypot(pose.c[1]);
    let theta = {
        let t = p.y.atan2(p.x) - pose.c[1].atan2(pose.c[0]);
        (t + PI).rem_euclid(2.0 * PI) - PI
    };
    let up = p.z >= pose.c[2];
    if pose.big > R + d {
        (
            if up { 1 } else { -1 },
            theta.rem_euclid(2.0 * PI),
            2.0 * PI,
        )
    } else {
        let t0 = ((R * R + d * d - pose.big * pose.big) / (2.0 * R * d)).acos();
        let tau = if up { theta + t0 } else { 3.0 * t0 - theta };
        (0, tau.rem_euclid(4.0 * t0), 4.0 * t0)
    }
}

/// **Every segment joins two neighbouring sites along their loop**, and
/// every arc between neighbours is one segment: the matcher's pairing,
/// read through the frame, against the loops' own order. Every germ is
/// consumed (one segment per pair record).
#[test]
fn every_segment_joins_neighbouring_sites_along_its_loop() {
    let tol = Tol::witness();
    for pose in poses().into_iter().filter(|p| p.drum.cut.is_none()) {
        let (a, b) = pose.operands();
        for (op_label, op, swapped, _) in runs(0.0, 0.0, 0.0) {
            let label = format!("{}, {op_label}", pose.label);
            let (x, y) = if swapped { (&b, &a) } else { (&a, &b) };
            let (records, segments) = topo::test_support::boolean_segment_sites(op, x, y, tol)
                .unwrap_or_else(|e| panic!("{label}: the matcher refused {e:?}"))
                .unwrap_or_else(|| panic!("{label}: the walls cross"));
            assert_eq!(segments.len(), records, "{label}: every germ is consumed");
            let mut sites: Vec<(i8, f64, f64)> = Vec::new();
            for s in &segments {
                for &p in s {
                    let at = place(&pose, p);
                    if !sites
                        .iter()
                        .any(|q| q.0 == at.0 && cyclic_gap(q.1, at.1, at.2) < 1e-9)
                    {
                        sites.push(at);
                    }
                }
            }
            for s in &segments {
                let (p, q) = (place(&pose, s[0]), place(&pose, s[1]));
                assert_eq!(p.0, q.0, "{label}: a segment joins two loops");
                let mut along: Vec<f64> =
                    sites.iter().filter(|t| t.0 == p.0).map(|t| t.1).collect();
                along.sort_by(f64::total_cmp);
                let index = |x: f64| {
                    along
                        .iter()
                        .position(|&t| cyclic_gap(t, x, p.2) < 1e-9)
                        .unwrap()
                };
                let (i, j, n) = (index(p.1), index(q.1), along.len());
                assert!(
                    (i + 1) % n == j || (j + 1) % n == i,
                    "{label}: a segment skips a site on its loop ({i} → {j} of {n})"
                );
            }
            assert_eq!(
                segments.len(),
                sites.len(),
                "{label}: one segment per arc between neighbouring sites"
            );
        }
    }
}

fn cyclic_gap(a: f64, b: f64, period: f64) -> f64 {
    let g = (a - b).rem_euclid(period);
    g.min(period - g)
}

/// **The oracle agrees with a second slicing**: on the uncut drum the
/// slices along the cylinder's axis are lenses, a closed form of its
/// own, and the two integrals of the same volume meet.
#[test]
fn the_oracle_agrees_with_a_second_slicing() {
    for pose in poses().into_iter().filter(|p| p.drum.cut.is_none()) {
        let (x, y) = (
            oracle(pose.big, pose.c, pose.drum),
            oracle_along_the_axis(pose.big, pose.c, pose.drum),
        );
        assert!(
            (x - y).abs() < 1e-11,
            "{}: {x} sliced across against {y} along the axis",
            pose.label
        );
    }
    // And the cut: a ball wholly below it and inside the wall is whole.
    let inside = oracle(0.1, [0.0, 0.0, 0.2], LOWER);
    assert!((inside - 4.0 / 3.0 * PI * 0.001).abs() < 1e-13, "{inside}");
}
