//! **Two cylinder walls with parallel axes crossing**: they meet in two
//! rulings, and the join's germ-pair dispatch splits each wall along its
//! own ruling — the plane × cylinder ruling arm on both sides
//! (`parallel-cylinder-germ-pair-has-no-join-arm`, JOIN, closed by PR 4031).
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
use topo::{AtRestBody, BooleanError, BooleanErrorKind, BooleanResult};

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
            topo::transform_rigid(
                b,
                &geom_core::test_support::rigid(&m, Tol::witness()),
                Tol::witness(),
            )
            .unwrap(),
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
/// along the rulings like the poses above. Each build passes tiers 2 and
/// 3′, the certificate and its closed-form volume, and is a legal
/// operand: its union with a brick far away adds the brick. The
/// closed-form volume cannot side the drum's cut wall, trimmed by an
/// ellipse, for a ray from the far brick that meets nothing; a ray aimed
/// at a planar face does.
#[test]
fn the_rim_crossing_rods_build_in_every_op() {
    let tol = Tol::witness();
    let far = finished(
        "a brick far away",
        sweep::test_support::brick((50.0, 51.0), (50.0, 51.0), (50.0, 51.0), tol),
        tol,
    );
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
                    let m = topo::mass_properties(
                        &bb.body,
                        tol,
                        topo::test_support::pinned_resolution(tol),
                    )
                    .unwrap_or_else(|e| panic!("{label}: measures, got {e:?}"));
                    assert!(
                        (m.enclosure.volume - want).abs() <= m.enclosure.volume_pad + 1e-12,
                        "{label}: volume {} ± {} vs {want}",
                        m.enclosure.volume,
                        m.enclosure.volume_pad
                    );
                    let joined = topo::union(&bb.body, &far, tol)
                        .unwrap_or_else(|e| panic!("{label}: ∪ a far brick builds, got {e:?}"));
                    let j = topo::mass_properties(
                        &joined.body().expect("not empty").body,
                        tol,
                        topo::test_support::pinned_resolution(tol),
                    )
                    .unwrap_or_else(|e| panic!("{label}: ∪ a far brick measures, got {e:?}"));
                    assert!(
                        (j.enclosure.volume - (want + 1.0)).abs() <= j.enclosure.volume_pad + 1e-12,
                        "{label}: ∪ a far brick: volume {} ± {} vs {}",
                        j.enclosure.volume,
                        j.enclosure.volume_pad,
                        want + 1.0
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
            topo::transform_rigid(&rod_b, &geom_core::test_support::rigid(&m, tol), tol).unwrap(),
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

/// A rod of radius `r` about `(x, y)`, extruded over `z ∈ [−far, far]`
/// so its wall's origin is stored on the profile at `z = −far`, cut to
/// `[z0, z1]` by a box (the wall keeps that origin, now `far` off its
/// faces), then tipped `theta` about `x` through its own centre.
fn cut_and_tipped(
    r: f64,
    (x, y): (f64, f64),
    (z0, z1): (f64, f64),
    far: f64,
    theta: f64,
) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let cutter = finished(
        "the cutter",
        sweep::test_support::brick((x - 1.0, x + 1.0), (y - 1.0, y + 1.0), (z0, z1), tol),
        tol,
    );
    let cut = topo::intersect(&rod(r, (x, y), (-far, far)), &cutter, tol).expect("the cut rod");
    let cut = cut.body().expect("the cut is a body");
    let m = Affine3::rotation_about_axis(
        Point3::new(x, y, 0.5 * (z0 + z1)),
        Vec3::new(1.0, 0.0, 0.0),
        theta,
    );
    finished(
        "the tipped rod",
        topo::transform_rigid(&cut.body, &geom_core::test_support::rigid(&m, tol), tol).unwrap(),
        tol,
    )
}

/// **The rulings are read at the germ sites, wherever a wall's origin is
/// stored.** Pose 0's rod cut from a long one, so its wall's origin is
/// stored up to 1e5 m off its faces, and tipped a fifth of the zero band:
/// every op builds SOUND at the closed-form volume, in both orders.
///
/// A pose runs only inside D4 ¶4's size range at the run's ε: the long
/// rod reaches `|z| = far`, where f64 spaces coordinates
/// `ulp(far) ≈ far·2⁻⁵²`, so a residual between two points evaluated
/// there can read a whole ulp of rounding. Past `far ≈ 2⁵²·ε` no stored
/// datum certifies within ε. At `far = 1e5` and ε = 1e-12,
/// `carrier_matches_mapped_source` reads one ulp at `|z| ≈ 6.2e4` on the
/// long rod's strut and escalates.
#[test]
fn a_tipped_rod_whose_origin_is_stored_far_joins_along_its_rulings() {
    const R: f64 = 0.5;
    let zero = geom_core::Band::linear(Tol::witness()).unwrap().zero();
    let (r, c, span) = (0.2, (0.5, 0.0), (-0.5, 0.5));
    let (area, _) = lens((0.0, 0.0), R, c, r);
    let poses: Vec<Pose> = [2.0, 1000.0, 1.0e5]
        .into_iter()
        .filter(|&far| {
            let in_range = far * f64::EPSILON <= zero;
            if !in_range {
                test_utils::vacuity::stood_down(
                    &format!("rod cut from ±{far} m"),
                    &format!(
                        "f64 spaces coordinates {:e} m apart at |z| = {far}, past the zero band \
                         {zero:e}: no pose this far certifies at this ε",
                        far * f64::EPSILON
                    ),
                );
            }
            in_range
        })
        .map(|far| Pose {
            label: format!("rod cut from ±{far} m, tipped 0.2·zero"),
            a: rod(R, (0.0, 0.0), (-1.0, 1.0)),
            b: cut_and_tipped(r, c, span, far, 0.2 * zero),
            va: PI * R * R * 2.0,
            vb: PI * r * r * (span.1 - span.0),
            shared: area * (span.1 - span.0),
        })
        .collect();
    assert!(
        !poses.is_empty(),
        "every pose stood down at zero band {zero:e}"
    );
    let bad = unsound(&poses);
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// The lever of an in-band reading of a cylinder pair's axes.
const AXES_LEVER: &str = "Recourse: turn one part so the two cylinders' axes are clearly parallel";

/// The unit rod about `(1, 1)` over `z ∈ [−0.5, 0.5]` tipped `theta`
/// about `axis` through its centre, against the unit rod about the
/// origin over `z ∈ [−1, 1]`. Their walls cross along the rulings at
/// `(1, 0)` and `(0, 1)`, which are a seam ruling of each rod
/// ([`rod`]'s seams stand at `x ± r`): each germ runs along one
/// solid's edge inside the other's wall face, so no germ's frame reads
/// the axes, and the join reads them first.
fn seams_on_the_walls(axis: Vec3<f64>, theta: f64) -> Pose {
    let tol = Tol::witness();
    let m = Affine3::rotation_about_axis(Point3::new(1.0, 1.0, 0.0), axis, theta);
    let b = rod(1.0, (1.0, 1.0), (-0.5, 0.5));
    Pose {
        label: format!("seams on the walls, tipped {theta:e} about {axis:?}"),
        a: rod(1.0, (0.0, 0.0), (-1.0, 1.0)),
        b: finished(
            "the tipped rod",
            topo::transform_rigid(&b, &geom_core::test_support::rigid(&m, tol), tol).unwrap(),
            tol,
        ),
        va: 2.0 * PI,
        vb: PI,
        shared: lens((0.0, 0.0), 1.0, (1.0, 1.0), 1.0).0,
    }
}

/// **The join reads the axes of a pair no frame read, at the frame's
/// lever, and refuses an in-band reading as their angle.**
/// [`seams_on_the_walls`] upright builds SOUND in every op and order.
/// Tipped `1.5·zero / 3.2` about `x` or about `y`, the tip reads about
/// `1.7·zero` over the walls' span (about 3.7, the diameter of their
/// faces' vertices), in the band, and every op escalates
/// `cc_axes_parallel` as the axes' own decision, whose lever is a turn;
/// at the radius it would read `0.47·zero`, parallel, and take the
/// rulings arm with the axes apart. The lever names the parallel side
/// alone, the one that passes: turned back to `0.5·zero / 3.2` every op
/// builds SOUND. Turned clearly apart, a fifth of a radian, every op
/// ends at a frontier, the same at the three gated tolerances (1e-9,
/// 1e-6, 1e-12; at 1e-5, 3e-5 and 1e-4 the tilt about `x` escalates
/// first, on `PierceCurvature` or `ArcCylinderRoots`): about `x` the germ
/// frame has no arm for the skew walls (`GermFrameUnsupported`), about
/// `y` an edge pierces a curved face the Boolean cannot yet read
/// (`CurvedPierceUnsupported`).
#[test]
fn seams_lying_on_the_walls_read_the_axes_at_the_walls_span() {
    let tol = Tol::witness();
    let zero = geom_core::Band::linear(tol).unwrap().zero();
    let axes = [Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)];
    let mut building = vec![seams_on_the_walls(axes[0], 0.0)];
    building.extend(axes.map(|axis| seams_on_the_walls(axis, 0.5 * zero / 3.2)));
    let bad = unsound(&building);
    assert!(bad.is_empty(), "{}", bad.join("\n"));
    for (axis, frontier) in axes.into_iter().zip([
        BooleanErrorKind::GermFrameUnsupported,
        BooleanErrorKind::CurvedPierceUnsupported,
    ]) {
        let tipped = seams_on_the_walls(axis, 1.5 * zero / 3.2);
        for (op, r, _) in tipped.runs() {
            assert!(
                matches!(
                    &r,
                    Err(e @ BooleanError::Escalated {
                        decision: topo::BooleanDecision::CylinderAxes,
                        diag,
                    }) if diag.predicate == Some("cc_axes_parallel")
                        && e.to_string().ends_with(AXES_LEVER)
                ),
                "{} | {op}: {:?}",
                tipped.label,
                r.map(|_| "a body").map_err(|e| e.to_string())
            );
        }
        let turned = seams_on_the_walls(axis, 0.2);
        for (op, r, _) in turned.runs() {
            assert!(
                matches!(&r, Err(e) if e.kind() == frontier),
                "{} | {op}: want {frontier:?}, got {:?}",
                turned.label,
                r.map(|_| "a body").map_err(|e| e.to_string())
            );
        }
    }
}

/// A one-face wall: the circle of radius `r` about `c` with its one
/// vertex at azimuth `az`, extruded over `z ∈ [z0, z1]`, its seam the
/// wall's wrap edge.
fn one_face_rod(c: (f64, f64), r: f64, az: f64, (z0, z1): (f64, f64)) -> AtRestBody<f64> {
    use profile::{Profile, RawLoop, Segment, SketchPlane};
    let tol = Tol::witness();
    let circle = RawLoop::new([(
        Point2::new(c.0 + r * az.cos(), c.1 + r * az.sin()),
        Segment::Arc(geom_core::Arc2 {
            centre: Point2::new(c.0, c.1),
            radius: r,
            sweep: 2.0 * PI,
        }),
    )]);
    let profile = Profile::new(SketchPlane::<f64>::xy(), vec![circle])
        .validate(tol)
        .unwrap();
    let depth = sweep::Extrusion::Distance {
        depth: z1 - z0,
        side: sweep::ExtrudeSide::Along,
    };
    let body = sweep::extrude(&profile, depth, tol).unwrap().body;
    let up = Affine3::translation(Vec3::new(0.0, 0.0, z0));
    finished(
        "the one-face rod",
        topo::transform_rigid(&body, &geom_core::test_support::rigid(&up, tol), tol).unwrap(),
        tol,
    )
}

/// The one-face unit rod about `(1.2, 0)` over `z ∈ [−0.3, 0.3]`,
/// its seam at azimuth `az`, tipped `theta` about `x` through its
/// centre, against the one-face unit rod about the origin over
/// `z ∈ [−1, 1]` with its seam at the same azimuth. Every germ lies
/// inside a wall face of both, so the frame reads the axes as the
/// segments are matched.
fn one_face_walls(az: f64, theta: f64) -> Pose {
    let tol = Tol::witness();
    let m =
        Affine3::rotation_about_axis(Point3::new(1.2, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), theta);
    let b = topo::transform_rigid(
        &one_face_rod((1.2, 0.0), 1.0, az, (-0.3, 0.3)),
        &geom_core::test_support::rigid(&m, tol),
        tol,
    )
    .unwrap();
    Pose {
        label: format!("one-face walls, seams at {az}, tipped {theta:e}"),
        a: one_face_rod((0.0, 0.0), 1.0, az, (-1.0, 1.0)),
        b: finished("the tipped rod", b, tol),
        va: 2.0 * PI,
        vb: 0.6 * PI,
        shared: 0.6 * lens((0.0, 0.0), 1.0, (1.2, 0.0), 1.0).0,
    }
}

/// **The frame reads one-face walls' axes as an angle too.**
/// [`one_face_walls`] tipped `0.2·zero` builds SOUND in every op and
/// order; tipped `0.6·zero`, read over the walls' span, the tip is in
/// the band and every op escalates `cc_axes_parallel` as the axes' own
/// decision, its lever a turn.
#[test]
fn one_face_walls_read_their_axes_as_an_angle() {
    let tol = Tol::witness();
    let zero = geom_core::Band::linear(tol).unwrap().zero();
    let seams = [0.0, PI];
    let bad = unsound(&seams.map(|az| one_face_walls(az, 0.2 * zero)));
    assert!(bad.is_empty(), "{}", bad.join("\n"));
    for az in seams {
        let tipped = one_face_walls(az, 0.6 * zero);
        for (op, r, _) in tipped.runs() {
            assert!(
                matches!(
                    &r,
                    Err(e @ BooleanError::Escalated {
                        decision: topo::BooleanDecision::CylinderAxes,
                        diag,
                    }) if diag.predicate == Some("cc_axes_parallel")
                        && e.to_string().ends_with(AXES_LEVER)
                ),
                "{} | {op}: {:?}",
                tipped.label,
                r.map(|_| "a body").map_err(|e| e.to_string())
            );
        }
    }
}

/// **A tipped rod's refusal does not hang on the operand order.** At
/// the half-length 50 and the larger tip, the drum's seam vertex lies on
/// the rod's tipped cap with its ruling a tip off the cap's plane, an
/// honest in-band reading, while the rod's own rim vertex on the drum's
/// wall reads the pair's definite skew frame. Whichever operand's
/// vertices are read first, the definite refusal is the answer: every
/// op of the pose, in both orders, refuses alike.
#[test]
fn a_tipped_rod_refuses_alike_in_both_operand_orders() {
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    for half in [50.0, 200.0] {
        for drift in [2.0 * band.escalate(), 20.0 * band.escalate()] {
            for (r, c) in [(0.2, 0.5), (0.3, 0.6)] {
                let pose = tipped(r, c, half, drift / (2.0 * half));
                let kinds: Vec<_> = pose
                    .runs()
                    .into_iter()
                    .map(|(op, res, _)| match res {
                        Err(e) => (op, e.kind()),
                        Ok(_) => panic!("{} | {op}: refuses", pose.label),
                    })
                    .collect();
                assert!(
                    kinds.iter().all(|(_, k)| *k == kinds[0].1),
                    "{}: every op in both orders refuses alike: {kinds:?}",
                    pose.label
                );
            }
        }
    }
}
