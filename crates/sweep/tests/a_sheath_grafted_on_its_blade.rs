//! **A sheath grafted on its blade at their shared section**: the lily
//! tour's wall 8 in miniature. Two lens lofts stand on one base section,
//! in one plane, running away from it in opposite directions, so each
//! one's base rim arcs ARE the other's, and each arc rises into a NURBS
//! wall of its own body.
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
//! meeting. The union stops one step on, at a seam of A against a NURBS
//! wall of B whose boundary leaves it undecided, where the table's
//! spline-face door has no interior reading
//! (`work/splice/an-analytic-edge-against-a-nurbs-face-has-no-crossing-lane`).
//! This row is that door's tripwire: it reds when the door lands.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::SurfaceKind;
use geom_core::{Band, Point2, Point3, RigidMotion, Tol, Vec3};
use profile::test_support::bulge_loop;
use profile::{ProfileLoop, SketchPlane};
use sweep::loft_body;
use topo::{BooleanError, Operand};

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

/// **Both orders get past the contact and the shared cap**, and stop
/// where an edge of operand A meets B's ridge wall, a NURBS wall its
/// boundary leaves undecided: the spline-face door's refusal. The row
/// pins the operand and the wall; it cannot tell that door from another
/// site raising the same variant on the same wall (the sector walk's, at
/// a vertex on it), since the refusal names no site. Before the
/// boundary pass, each refused at the contact,
/// `CurvedBooleanUnsupported { kind: Nurbs }` on a wall of B; before the
/// ring-composite arm, at the planar lane, `EdgeCarrierUnsupported` on a
/// tip seam of A against the cap of B. The shared rim takes the `Along`
/// reading: read `Clear`, it would be an edge of B bounding the face it
/// is read against, which the twin-edge reading refuses as a
/// contradiction.
#[test]
#[allow(clippy::cast_precision_loss)]
fn a_sheath_on_its_blade_unions_past_the_shared_rim() {
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
        let BooleanError::CurvedBooleanUnsupported {
            operand: Operand::B,
            face,
            kind: SurfaceKind::Nurbs,
        } = err
        else {
            panic!("{what}: past the cap, at a NURBS wall of B: {err:?}");
        };
        let surface = b
            .get_face(face)
            .and_then(|f| b.get_surface(f.surface))
            .map(geom::Surface::kind);
        assert_eq!(surface, Some(SurfaceKind::Nurbs), "{what}: a wall of B");
        // Which wall: B's ridge wall, the higher of its two by the mean
        // height of its control net, in either order.
        let height = |f: &topo::entity::Face| match b.get_surface(f.surface) {
            Some(geom::Surface::Nurbs(n)) => {
                Some(n.control().iter().map(|p| p.z).sum::<f64>() / n.control().len() as f64)
            }
            _ => None,
        };
        let ridge = b
            .faces()
            .filter_map(|(k, f)| height(f).map(|h| (k, h)))
            .max_by(|x, y| x.1.total_cmp(&y.1))
            .map(|(k, _)| k);
        assert_eq!(ridge, Some(face), "{what}: B's ridge wall");
        // The cap read on the way: every spline edge of A, against every
        // plane face of B, through the arm the planar lane reads, crosses
        // nothing inside itself.
        let lane = geom_brep::SplineFoot::<f64>::certified();
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
