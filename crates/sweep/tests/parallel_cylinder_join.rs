//! **Two cylinder walls with parallel axes crossing**: they meet in two
//! rulings, and the join's germ-pair dispatch splits each wall along its
//! own ruling — the plane × cylinder ruling arm on both sides
//! (`work/join/parallel-cylinder-germ-pair-has-no-join-arm.md`).
//!
//! Every pose runs ∪, ∩ and both differences in both operand orders,
//! and every run builds a body that is SOUND by
//! `common::differential::outcome` (tiers 2 and 3′, the certificate, a
//! legal operand, the volume). The volumes are closed form: the solids
//! meet over the lens of their two discs, and a slab's height over it is
//! constant or linear, whose integral is the lens's area and first
//! moment ([`lens`]).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use crate::common::differential::outcome;
use crate::conic_edge_curved_face::{DRUM_RADIUS, TILT, drum_lower};
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use topo::{Body, BooleanError, BooleanResult};

/// A vertical rod of radius `r` about `(x, y)` over `z ∈ [z0, z1]`.
fn rod(r: f64, (x, y): (f64, f64), (z0, z1): (f64, f64)) -> Body<f64> {
    sweep::test_support::prism_at(
        vec![(Point2::new(x - r, y), 1.0), (Point2::new(x + r, y), 1.0)],
        z0,
        z1 - z0,
        Tol::witness(),
    )
}

/// The lens of the discs `(c1, r1)` and `(c2, r2)`: its area and its
/// first moment `∫ x dA`. Each disc contributes the segment beyond the
/// radical line, at signed distance `h` from its centre toward the
/// other: area `r² acos(h/r) − h √(r² − h²)`, centroid offset
/// `⅔ (r² − h²)^{3/2} / area` along the centre line.
fn lens(c1: (f64, f64), r1: f64, c2: (f64, f64), r2: f64) -> (f64, f64) {
    let (dx, dy) = (c2.0 - c1.0, c2.1 - c1.1);
    let d = dx.hypot(dy);
    let ux = dx / d;
    let h1 = (d * d + r1 * r1 - r2 * r2) / (2.0 * d);
    let h2 = d - h1;
    let seg = |r: f64, h: f64| {
        let s = (r * r - h * h).sqrt();
        (r * r * (h / r).acos() - h * s, 2.0 / 3.0 * s.powi(3))
    };
    let (a1, m1) = seg(r1, h1);
    let (a2, m2) = seg(r2, h2);
    (a1 + a2, a1 * c1.0 + m1 * ux + a2 * c2.0 - m2 * ux)
}

/// One run: the op and order, what it returned, and the volume it must
/// have.
type Run = (&'static str, Result<BooleanResult<f64>, BooleanError>, f64);

/// One pose: the two operands, their volumes and the volume they share.
struct Pose {
    label: String,
    a: Body<f64>,
    b: Body<f64>,
    va: f64,
    vb: f64,
    shared: f64,
}

impl Pose {
    /// The pose's six runs and the volume each must have.
    fn runs(&self) -> [Run; 6] {
        let tol = Tol::witness();
        let (a, b) = (&self.a, &self.b);
        let (va, vb, s) = (self.va, self.vb, self.shared);
        [
            ("U ab", topo::union(a, b, tol), va + vb - s),
            ("U ba", topo::union(b, a, tol), va + vb - s),
            ("I ab", topo::intersect(a, b, tol), s),
            ("I ba", topo::intersect(b, a, tol), s),
            ("S ab", topo::subtract(a, b, tol), va - s),
            ("S ba", topo::subtract(b, a, tol), vb - s),
        ]
    }
}

/// A drum of radius `R` about `z` over `z ∈ [−1, 1]` against a rod
/// `(r, centre, span)`: they share the lens times the spans' overlap.
fn upright(r: f64, c: (f64, f64), span: (f64, f64)) -> Pose {
    const R: f64 = 0.5;
    let (area, _) = lens((0.0, 0.0), R, c, r);
    let overlap = span.1.min(1.0) - span.0.max(-1.0);
    Pose {
        label: format!("drum R {R} × rod r {r} at {c:?}, z ∈ {span:?}"),
        a: rod(R, (0.0, 0.0), (-1.0, 1.0)),
        b: rod(r, c, span),
        va: PI * R * R * 2.0,
        vb: PI * r * r * (span.1 - span.0),
        shared: area * overlap,
    }
}

/// [`upright`] with both operands turned by `angle` about `axis`
/// through the origin: the axes stay parallel, off every coordinate
/// direction.
fn turned(mut pose: Pose, axis: Vec3<f64>, angle: f64) -> Pose {
    let m = Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), axis, angle);
    let turn = |b: &Body<f64>| topo::transform_rigid(b, &m, Tol::witness()).unwrap();
    pose.a = turn(&pose.a);
    pose.b = turn(&pose.b);
    pose.label = format!("{} turned {angle} about {axis:?}", pose.label);
    pose
}

/// The row's poses: the tilted drum cut's lower part against a rod
/// standing across its rim. Over the lens the rod's span is clipped
/// below by `z0` and above by the cut, `0.5 − x tan(TILT)`, which every
/// pose here keeps inside the span; so the shared height is linear in
/// `x` and its integral is the lens's area and moment.
fn across_the_rim(r: f64, c: (f64, f64), (z0, z1): (f64, f64)) -> Pose {
    let (area, moment) = lens((0.0, 0.0), DRUM_RADIUS, c, r);
    let t = TILT.tan();
    Pose {
        label: format!("drum cut × rod r {r} at {c:?}, z ∈ [{z0}, {z1}]"),
        a: drum_lower(),
        b: rod(r, c, (z0, z1)),
        va: PI * DRUM_RADIUS.powi(2) * 0.5,
        vb: PI * r * r * (z1 - z0),
        shared: (0.5 - z0) * area - t * moment,
    }
}

/// The poses that build: a drum against a rod across its wall — rod
/// caps inside it, one through a cap, both through, equal radii, a rod
/// fatter than the drum, a rod axis inside the drum, a rod whose section
/// is an island in the drum's wall face, which a ruling's chord closes
/// as a ring — and one pose turned off the coordinate axes.
fn poses() -> Vec<Pose> {
    vec![
        upright(0.2, (0.5, 0.0), (-0.5, 0.5)),
        upright(0.2, (0.5, 0.0), (-0.5, 1.5)),
        upright(0.3, (0.6, 0.1), (-1.5, 1.5)),
        upright(0.5, (0.7, 0.0), (-0.3, 0.4)),
        upright(0.8, (0.9, 0.0), (-0.5, 0.5)),
        upright(0.2, (0.4, 0.0), (-0.5, 0.5)),
        upright(0.15, (0.0, 0.5), (-0.5, 0.5)),
        turned(
            upright(0.25, (0.5, 0.15), (-0.6, 0.7)),
            Vec3::new(1.0, 2.0, 0.5),
            0.7,
        ),
    ]
}

/// The row's poses (`conic_edge_curved_face`'s
/// `a_rim_crossing_reaches_the_join`).
fn rim_poses() -> Vec<Pose> {
    vec![
        across_the_rim(0.2, (0.5, 0.0), (0.2, 0.45)),
        across_the_rim(0.1, (-0.45, 0.0), (0.5, 0.8)),
        across_the_rim(0.1, (0.0, 0.48), (0.3, 0.7)),
    ]
}

/// The lines of the runs of `poses` that are not SOUND.
fn unsound(poses: &[Pose]) -> Vec<String> {
    let tol = Tol::witness();
    let mut lines = Vec::new();
    for pose in poses {
        for (op, r, want) in pose.runs() {
            let line = outcome(r, want, tol);
            if !line.starts_with("OK SOUND") {
                lines.push(format!("{} | {op} | {line}", pose.label));
            }
        }
    }
    lines
}

/// **Every pose builds SOUND in every op and order**: tiers 2 and 3′,
/// the certificate, a legal operand, and the closed-form volume. Split
/// in two to keep each test short: a rod across the drum's wall, then
/// the wider, nested and turned poses.
#[test]
fn a_rod_across_the_wall_joins_along_its_rulings() {
    let bad = unsound(&poses()[..4]);
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// [`a_rod_across_the_wall_joins_along_its_rulings`]'s other half.
#[test]
fn wide_nested_and_turned_walls_join_along_their_rulings() {
    let bad = unsound(&poses()[4..]);
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// **The row's rods pass the join and stop at the at-infinity probe.**
/// Their walls join along the rulings like the poses above, and every op
/// then refuses `Containment(VolumeUncertified)`: the classification's
/// probe measures the drum's cut wall, trimmed by an ellipse, in closed
/// form only (`work/contact/at-infinity-probe-measures-in-closed-form-only.md`).
/// Each pose's volumes are written down for the day that door opens.
#[test]
fn the_rim_crossing_rods_stop_at_the_volume_probe() {
    for pose in rim_poses() {
        assert!(
            pose.shared > 0.0 && pose.shared < pose.vb,
            "{}: the rod is part in, part out",
            pose.label
        );
        for (op, r, _) in pose.runs() {
            assert!(
                matches!(
                    r,
                    Err(BooleanError::Containment(
                        topo::PointInSolidError::VolumeUncertified
                    ))
                ),
                "{} | {op}: {:?}",
                pose.label,
                r.map(|_| "a body")
            );
        }
    }
}

/// **The lens oracle agrees with a second slicing**: equal discs, whose
/// lens is the symmetric closed form, and every pose's discs against
/// the lens sliced across the centre line, each slice the overlap of
/// two chords, by the midpoint rule.
#[test]
fn the_lens_oracle_agrees_with_a_second_slicing() {
    let (r, d) = (0.5_f64, 0.7_f64);
    let (area, moment) = lens((0.0, 0.0), r, (d, 0.0), r);
    let half = d / 2.0;
    let want = 2.0 * r * r * (half / r).acos() - d * (r * r - half * half).sqrt();
    assert!(
        (area - want).abs() < 1e-14,
        "equal discs: {area} against {want}"
    );
    assert!(
        (moment / area - half).abs() < 1e-14,
        "equal discs: centroid"
    );
    for (r1, c2, r2) in [
        (0.5, (0.5, 0.0), 0.2),
        (0.5, (0.6, 0.1), 0.3),
        (0.5, (0.9, 0.0), 0.8),
        (0.5, (0.4, 0.0), 0.2),
        (0.5, (-0.45, 0.0), 0.1),
        (0.5, (0.0, 0.48), 0.1),
    ] {
        let (area, moment) = lens((0.0, 0.0), r1, c2, r2);
        // Sliced along y: at each y, the two discs' x-chords overlap.
        let n = 400_000;
        let (lo, hi) = (-r1, r1);
        let dy = (hi - lo) / f64::from(n);
        let (mut a, mut m) = (0.0, 0.0);
        for k in 0..n {
            let y = lo + (f64::from(k) + 0.5) * dy;
            let w1 = (r1 * r1 - y * y).max(0.0).sqrt();
            let w2 = (r2 * r2 - (y - c2.1).powi(2)).max(0.0).sqrt();
            let (x0, x1) = ((-w1).max(c2.0 - w2), w1.min(c2.0 + w2));
            if x1 > x0 {
                a += (x1 - x0) * dy;
                m += (x1 * x1 - x0 * x0) / 2.0 * dy;
            }
        }
        assert!((area - a).abs() < 1e-8, "{c2:?}: area {area} against {a}");
        assert!(
            (moment - m).abs() < 1e-8,
            "{c2:?}: moment {moment} against {m}"
        );
    }
}
