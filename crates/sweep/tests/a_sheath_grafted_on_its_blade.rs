//! **A sheath grafted on its blade at their shared section**: the
//! witness that a rim tangent at two points refuses. Two lens lofts
//! stand on one base section, in one plane, running away from it in
//! opposite directions, so each one's base rim arcs ARE the other's, and
//! each arc rises into a NURBS wall of its own body.
//!
//! The boolean reads each rim arc against the other body's spline walls
//! by their boundary first (C5's curve × face clause): along the wall it
//! shares, a run of one circle the curve-pair reading decides; against
//! the lens's other wall, the other body's own rim edge, which meets that
//! wall at the two tips only. Neither asks the surface anything, so the
//! union gets past the contact, in either operand order. A NURBS tip
//! seam rising from the shared plane is read against the other body's
//! base cap there by the curve × face table's ring-composite arm, which
//! finds no crossing inside it: its start on the cap is its end vertex's
//! meeting. Each NURBS seam of one body is read against the other's
//! NURBS walls by the table's spline-face arm, its end at a tip accounted
//! for as the end vertex's meeting. At the result, the shared rim, a
//! seam between the two bodies' NURBS walls, reads transverse: each
//! wall's normal off the rim's image in its chart, levered over each
//! chart's certified curvature. The rim is the `v = 0` row of both walls,
//! a curve known in advance, so it is described by that shared row (D2's
//! structural selector), certified per wall with no uniqueness tube. Its
//! certificate then bounds the walls' crossing over the whole rim, and on
//! one of the lens's two rims the walls are tangent at two points,
//! symmetric about the rim's middle: their crossing reading changes sign
//! across each. The union refuses there. This row is that site's
//! tripwire: it reds when the rim certifies.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point2, Point3, RigidMotion, Tol, Vec3};
use profile::test_support::bulge_loop;
use profile::{ProfileLoop, SketchPlane};
use sweep::loft_body;
use topo::{BooleanError, EdgeDescribeFailure, EulerOpError};

/// The lens on chord `width`, rising `ridge` above it and dropping
/// `keel` below, run counterclockwise from the `+s` margin.
fn lens([width, ridge, keel]: [f64; 3]) -> Vec<ProfileLoop<f64>> {
    let half = 0.5 * width;
    vec![bulge_loop(vec![
        (Point2::new(half, 0.0), -ridge / half),
        (Point2::new(-half, 0.0), -keel / half),
    ])]
}

/// A lens blade from the origin along `d`, its spine an arc of length
/// `len` turning `curl` rad toward `v`, five stations, the section
/// `section(s)` at the fraction `s` of the way.
fn blade(
    d: Vec3<f64>,
    v: Vec3<f64>,
    (len, curl): (f64, f64),
    section: impl Fn(f64) -> [f64; 3],
    tol: Tol,
) -> topo::AtRestBody<f64> {
    const STATIONS: usize = 5;
    let band = Band::linear(tol).expect("the run's tolerance forms a band");
    let u = v.cross(d);
    let r = len / curl;
    let mut sections = Vec::with_capacity(STATIONS);
    let mut places = Vec::with_capacity(STATIONS);
    for k in 0..STATIONS {
        #[allow(clippy::cast_precision_loss)]
        let s = (k as f64) / ((STATIONS - 1) as f64);
        let (sa, ca) = (curl * s).sin_cos();
        let p = Point3::origin() + d * (r * sa) + v * (r * (1.0 - ca));
        let vk = v * ca - d * sa;
        sections.push(lens(section(s)));
        places.push(SketchPlane::from_frame(
            RigidMotion::gram_schmidt(p, u, vk, "sheath_blade", band)
                .expect("the station's two directions span a plane"),
        ));
    }
    let lofted = loft_body::<f64>(&sections, &places, 3, tol)
        .expect("the lens blade lofts")
        .body;
    topo::test_support::finished("a lens blade", lofted, tol)
}

/// **Both orders stop where the walls touch on the rim.** Each rim is
/// each wall's `v = 0` row, recovered as the walls' shared row. On one of
/// them (rim 1) the walls are tangent at two points symmetric about
/// `u = ½`, near `u ≈ 0.3024` and `u ≈ 0.6977` (the PR 5959 re-review's
/// reading off the walls' own derivatives), and the crossing reading
/// changes sign across each. The rim's certificate reads the whole row's
/// cover, where zero-free pieces of opposite signs with the walls' first
/// partials continuous between them hold a tangency, and refuses it
/// definitely: `NotTransverse` with no sample (`NOT_A_SAMPLE`, the
/// whole-row reading's verdict, where a sample's would name its index) and
/// an exact zero. The row runs at `Tol::witness()`, the run's ε, so its
/// three ε come from the CI matrix's `CAD_TOLERANCE_EPS` rows; it refuses
/// alike at 1e-9, 1e-6 and 1e-12. A certificate that read the
/// crossing at its samples alone stepped over both points and passed the
/// rim; before the shared row, the rim's witness refused at limb 3
/// (`TubeNotOneArc`).
/// Before the exact images, limb 2 refused on the fitted image's 9.0e-6 m
/// miss; before the seam's spline entry, the seam's dihedral arm
/// (`Escalated { LeverArm(Seam), "dihedral_arm" }`); before the spline
/// span × spline face arm, `CurvedBooleanUnsupported { operand: B, kind:
/// Nurbs }`; before the boundary pass, the contact on a wall of B; before
/// the ring-composite arm, `EdgeCarrierUnsupported` on a tip seam of A
/// against the cap of B.
#[test]
fn a_sheath_on_its_blade_stops_where_its_walls_touch_on_the_rim() {
    let tol = Tol::witness();
    let base = [0.170, 0.028, 0.020];
    let (d, v) = (Vec3::unit_x(), Vec3::unit_z());
    let leaf = blade(
        d,
        v,
        (1.2, -0.3),
        |s| [0, 1, 2].map(|i| base[i] + ([0.300, 0.034, 0.016][i] - base[i]) * s),
        tol,
    );
    let sheath = blade(-d, v, (0.34, 0.85), |_| base, tol);
    for (what, a, b) in [
        ("blade ∪ sheath", &leaf, &sheath),
        ("sheath ∪ blade", &sheath, &leaf),
    ] {
        let err = topo::union(a, b, tol).expect_err(what);
        assert!(
            matches!(
                &err,
                BooleanError::EdgeUndescribed {
                    failure: EdgeDescribeFailure::Intersection,
                    cause: Some(cause),
                    ..
                } if matches!(
                    **cause,
                    EulerOpError::Certification {
                        error: geom_brep::CertifyError::NotTransverse {
                            sample: geom_brep::certify::NOT_A_SAMPLE,
                            lever: None,
                            verdict: geom_brep::recourse::Refused::Zero(_),
                        },
                    }
                )
            ),
            "{what}: at the rim, where its walls touch: {err:?}"
        );
        // The cap read on the way: every spline edge of A, against every
        // plane face of B, through the arm the planar lane reads, crosses
        // nothing inside itself.
        let lane = geom_brep::NurbsLane::<f64>::certified();
        let band = Band::linear(tol).expect("the run's tolerance forms a band");
        let mut read = 0;
        for (_, edge) in a.edges() {
            let Some(curve) = a
                .get_curve_geom(edge.curve)
                .and_then(topo::CurveGeom::certified)
            else {
                continue;
            };
            let geom::Curve3::Nurbs(net) = curve.carrier() else {
                continue;
            };
            for (_, f) in b.faces() {
                let Some(plane @ geom::Surface::Plane { .. }) = b.get_surface(f.surface) else {
                    continue;
                };
                let roots = lane.surface_crossings(net, curve.params(), plane, band);
                assert!(
                    matches!(roots.as_ref().map(Vec::len), Ok(0)),
                    "{what}: a seam of A crosses a cap of B: {roots:?}"
                );
                read += 1;
            }
        }
        assert!(read > 0, "{what}: A has a spline edge and B a plane face");
    }
}

/// **The interior-loop guard reads the two bodies' walls with their
/// shared rims handed in.** The section certificate over every face
/// pair of blade × sheath, as if the sweep had found no event: each wall
/// of one body against the wall of the other that shares its rim reads
/// that rim, the `v = 0` row of both, as a row handed in
/// (`boolean::spline_known`). On rim 1, where the walls are tangent at
/// two points, the row's cover reads the crossing changing sign along
/// it, the same finding the rim's certificate makes, and the pair refuses
/// as tangent (`ssi_pair_row`). On rim 2 the cover is clear, the section
/// there is the row, a `Side` region on a side of both charts, and the
/// pair clears by W1′. The walls that share no rim meet at the lens's
/// tips and clear; so do the plane caps. Red against no row handed in
/// (rim 2 refuses `ssi_pair_boundary`), against the cover's width
/// dropped (likewise), and against a `Side` region read as a loop (rim 2
/// is placed nowhere: R-undec).
#[test]
fn the_guard_reads_the_walls_with_their_shared_rims_handed_in() {
    let tol = Tol::witness();
    let base = [0.170, 0.028, 0.020];
    let (d, v) = (Vec3::unit_x(), Vec3::unit_z());
    let leaf = blade(
        d,
        v,
        (1.2, -0.3),
        |s| [0, 1, 2].map(|i| base[i] + ([0.300, 0.034, 0.016][i] - base[i]) * s),
        tol,
    );
    let sheath = blade(-d, v, (0.34, 0.85), |_| base, tol);
    let report = topo::test_support::no_event_section_report(&leaf, &sheath, tol)
        .expect("the bodies' boxes build");
    let refused: Vec<&str> = report
        .iter()
        .filter(|(_, _, v)| !v.starts_with("Ok("))
        .map(|(_, _, v)| v.as_str())
        .collect();
    assert_eq!(
        refused,
        ["Err(Tangent(\"ssi_pair_row\"))"],
        "one pair, rim 1's, refuses as tangent along its row: {report:?}"
    );
    let spline = |b: &topo::AtRestBody<f64>, f| {
        b.get_face(f)
            .and_then(|x| b.get_surface(x.surface))
            .is_some_and(|s| s.kind() == geom::SurfaceKind::Nurbs)
    };
    let walls = report
        .iter()
        .filter(|(fa, fb, _)| spline(&leaf, *fa) && spline(&sheath, *fb))
        .count();
    assert_eq!(walls, 4, "every wall pair is read: {report:?}");
}
