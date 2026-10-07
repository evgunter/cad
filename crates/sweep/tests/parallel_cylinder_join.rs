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
use sweep::test_support::finished;
use topo::{AtRestBody, BooleanError, BooleanResult};

/// A vertical rod of radius `r` about `(x, y)` over `z ∈ [z0, z1]`.
fn rod(r: f64, (x, y): (f64, f64), (z0, z1): (f64, f64)) -> AtRestBody<f64> {
    let rod = sweep::test_support::prism_at(
        vec![(Point2::new(x - r, y), 1.0), (Point2::new(x + r, y), 1.0)],
        z0,
        z1 - z0,
        Tol::witness(),
    );
    finished("the rod", rod, Tol::witness())
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
    a: AtRestBody<f64>,
    b: AtRestBody<f64>,
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
    let turn = |b: &AtRestBody<f64>| {
        finished(
            "the turned operand",
            topo::transform_rigid(b, &m, Tol::witness()).unwrap(),
            Tol::witness(),
        )
    };
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

/// The `k`th of the eight poses that build: a drum against a rod across
/// its wall — rod caps inside it, one through a cap, both through, equal
/// radii, a rod fatter than the drum, a rod axis inside the drum, a rod
/// whose section is an island in the drum's wall face, which a ruling's
/// chord closes as a ring — and one pose turned off the coordinate axes.
fn pose(k: usize) -> Pose {
    match k {
        0 => upright(0.2, (0.5, 0.0), (-0.5, 0.5)),
        1 => upright(0.2, (0.5, 0.0), (-0.5, 1.5)),
        2 => upright(0.3, (0.6, 0.1), (-1.5, 1.5)),
        3 => upright(0.5, (0.7, 0.0), (-0.3, 0.4)),
        4 => upright(0.8, (0.9, 0.0), (-0.5, 0.5)),
        5 => upright(0.2, (0.4, 0.0), (-0.5, 0.5)),
        6 => upright(0.15, (0.0, 0.5), (-0.5, 0.5)),
        7 => turned(
            upright(0.25, (0.5, 0.15), (-0.6, 0.7)),
            Vec3::new(1.0, 2.0, 0.5),
            0.7,
        ),
        _ => panic!("there are eight poses"),
    }
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
/// in three to keep each test short.
#[test]
fn rods_across_the_wall_join_along_their_rulings() {
    let bad = unsound(&(0..3).map(pose).collect::<Vec<_>>());
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// [`rods_across_the_wall_join_along_their_rulings`] for equal radii, a
/// rod fatter than the drum, and a rod axis inside the drum.
#[test]
fn equal_wider_and_nested_rods_join_along_their_rulings() {
    let bad = unsound(&(3..6).map(pose).collect::<Vec<_>>());
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// [`rods_across_the_wall_join_along_their_rulings`] for the island a
/// ruling closes as a ring, and the turned pose.
#[test]
fn an_island_and_a_turned_pose_join_along_their_rulings() {
    let bad = unsound(&(6..8).map(pose).collect::<Vec<_>>());
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// **The row's rods build in every op and order.** Their walls join
/// along the rulings like the poses above. The classification's probe
/// measures the drum's cut wall, trimmed by an ellipse, in closed form
/// only (`work/contact/at-infinity-probe-measures-in-closed-form-only.md`),
/// so a probe ray that meets nothing cannot side its point; that ray is
/// set aside, and the query refuses `Containment(VolumeUncertified)` only
/// where no ray settles, which none of these poses reaches. Each build
/// passes tiers 2 and 3′, the certificate and its closed-form volume.
/// They are not yet legal operands (their union with a far brick
/// refuses; why is not measured here).
#[test]
fn the_rim_crossing_rods_build_in_every_op() {
    let tol = Tol::witness();
    for pose in rim_poses() {
        assert!(
            pose.shared > 0.0 && pose.shared < pose.vb,
            "{}: the rod is part in, part out",
            pose.label
        );
        for (op, r, want) in pose.runs() {
            let label = format!("{} | {op}", pose.label);
            match r {
                Ok(r) => {
                    let bb = r
                        .body()
                        .unwrap_or_else(|| panic!("{label}: a body is owed"));
                    assert!(topo::validate_closed(&bb.body).is_ok(), "{label}: tier 2");
                    assert!(
                        topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol).is_ok(),
                        "{label}: tier 3′"
                    );
                    assert!(
                        topo::validate_geometric_certificate(&bb.body, tol).is_ok(),
                        "{label}: the certificate"
                    );
                    // The cut wall's flux is a certified quadrature: the
                    // slack is its enclosure's own half-width.
                    let m = topo::mass_properties(&bb.body, tol)
                        .unwrap_or_else(|e| panic!("{label}: measures, got {e:?}"));
                    assert!(
                        (m.volume - want).abs() <= m.volume_pad + 1e-12,
                        "{label}: volume {} ± {} vs {want}",
                        m.volume,
                        m.volume_pad
                    );
                }
                other => panic!("{label}: {:?}", other.map(|_| "a body")),
            }
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

/// A drum of radius 0.5 over `z ∈ [−half − 1, half + 1]` against a rod
/// of radius `r` about `(c, 0)` over `z ∈ [−half, half]`, tipped `theta`
/// about the `x` axis through its centre.
fn tipped(r: f64, c: f64, half: f64, theta: f64) -> Pose {
    let tol = Tol::witness();
    let m = Affine3::rotation_about_axis(Point3::new(c, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), theta);
    let rod_b = rod(r, (c, 0.0), (-half, half));
    Pose {
        label: format!("drum × rod r {r} at {c}, half-length {half}, tipped {theta:e}"),
        a: rod(0.5, (0.0, 0.0), (-half - 1.0, half + 1.0)),
        b: finished(
            "the tipped rod",
            topo::transform_rigid(&rod_b, &m, tol).unwrap(),
            tol,
        ),
        va: PI * 0.25 * (2.0 * half + 2.0),
        vb: PI * r * r * 2.0 * half,
        shared: lens((0.0, 0.0), 0.5, (c, 0.0), r).0 * 2.0 * half,
    }
}

/// **A rod tipped off parallel over a long wall takes a decided door.**
/// The frame levers the axes' parallelism by the walls' reach, so a tip
/// whose drift over the walls is past the band reads as a skew pair (at
/// the witness tolerance, `GermFrameUnsupported`; at a finer one the
/// crossing layer meets the drift first) rather than reaching the
/// rulings arm and failing at the result's pcurve pass, as it did with
/// the parallelism levered by the radius alone. A tip whose drift stays
/// inside the zero band is parallel, and builds. The tips are set from
/// the band, so the row asks the same question at every tolerance.
#[test]
fn a_rod_tipped_off_parallel_over_a_long_wall_takes_a_decided_door() {
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    for half in [50.0, 200.0] {
        for drift in [2.0 * band.escalate(), 20.0 * band.escalate()] {
            for (r, c) in [(0.2, 0.5), (0.3, 0.6)] {
                let pose = tipped(r, c, half, drift / (2.0 * half));
                for (op, res, _) in pose.runs() {
                    assert!(
                        matches!(&res, Err(e) if !matches!(e, BooleanError::Pcurves { .. })),
                        "{} | {op}: {:?}",
                        pose.label,
                        res.map(|_| "a body")
                    );
                }
            }
        }
    }
    let parallel = tipped(0.3, 0.6, 5.0, band.zero() / 10.0 / 10.0);
    let bad = unsound(&[parallel]);
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// REVIEW PROBE (PR 4231, mutant R3): a rod of radius `r` about
/// `(x, y)` extruded from `z = −far` to `far`, so its wall's origin is
/// stored on the profile at `z = −far`; cut to `[z0, z1]` by a box (the
/// wall keeps its stored origin, now `far` metres off its faces), then
/// turned `k·zero` about `x` through its own centre.
fn tilted_rod(r: f64, (x, y): (f64, f64), (z0, z1): (f64, f64), far: f64, k: f64) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let zero = geom_core::Band::linear(tol).unwrap().zero();
    let long = rod(r, (x, y), (-far, far));
    let cutter = finished(
        "the cutter",
        sweep::test_support::brick((x - 1.0, x + 1.0), (y - 1.0, y + 1.0), (z0, z1), tol),
        tol,
    );
    let cut = topo::intersect(&long, &cutter, tol).expect("the cut rod");
    let cut = cut.body().expect("a body").clone();
    for (_, s) in cut.body.surfaces() {
        if let geom::Surface::Cylinder { origin, .. } = s {
            eprintln!("PROBE far {far}: the cut wall's stored origin {origin:?}");
        }
    }
    let cut = cut.body.clone();
    let m = Affine3::rotation_about_axis(
        Point3::new(x, y, 0.5 * (z0 + z1)),
        if std::env::var("PROBE_AX").is_ok() { Vec3::new(0.0, 1.0, 0.0) } else { Vec3::new(1.0, 0.0, 0.0) },
        k * zero,
    );
    finished(
        "the tilted rod",
        topo::transform_rigid(&cut, &m, tol).unwrap(),
        tol,
    )
}

/// **REVIEW PROBE (PR 4231)**: pose 0 with the rod's wall tilted in the
/// band and its origin stored 1000 m along: the join reads its radical
/// plane at the germ sites, so every op builds sound wherever the
/// origin is stored.
#[test]
fn review_4231_a_tilted_rod_stored_far_joins_along_its_rulings() {
    const R: f64 = 0.5;
    let (r, c, span) = (0.2, (0.5, 0.0), (-0.5, 0.5));
    let (area, _) = lens((0.0, 0.0), R, c, r);
    let mut bad = Vec::new();
    let k: f64 = std::env::var("PROBE_K").map(|v| v.parse().unwrap()).unwrap_or(0.2);
    for far in [2.0, 1000.0, 1.0e5] {
        let pose = Pose {
            label: format!("rod cut from ±{far} m, tilted {k}·zero"),
            a: rod(R, (0.0, 0.0), (-1.0, 1.0)),
            b: tilted_rod(r, c, span, far, k),
            va: PI * R * R * 2.0,
            vb: PI * r * r * (span.1 - span.0),
            shared: area * (span.1 - span.0),
        };
        bad.extend(unsound(&[pose]));
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}
