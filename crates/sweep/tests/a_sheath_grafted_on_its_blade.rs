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
//! union gets past the contact, in either operand order, and stops one
//! step on: a NURBS tip seam rising from the shared plane, read against
//! the other body's base cap there, where the planar lane has no
//! crossing reading for a spline edge
//! (`work/splice/nurbs-edge-crossing-rung-is-the-ring-composite`). This
//! row is that rung's tripwire too: it reds when the rung lands.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{CurveKind, SurfaceKind};
use geom_core::{Band, Point2, Point3, RigidMotion, Tol, Vec3};
use profile::test_support::bulge_loop;
use profile::{ProfileLoop, SketchPlane};
use sweep::loft_body;
use topo::{BooleanError, EdgeCarrierSite, Operand};

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

/// **Both orders get past the contact**, and stop where a tip seam of
/// operand A meets operand B's base cap: a spline edge of A, against a
/// plane face of B, in the planar lane. Before the boundary pass, each
/// refused at the contact, `CurvedBooleanUnsupported { kind: Nurbs }`
/// on a wall of B.
#[test]
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
        let BooleanError::EdgeCarrierUnsupported {
            operand: Operand::A,
            edge,
            face,
            face_of: Operand::B,
            site: EdgeCarrierSite::PlanarCrossing,
        } = err
        else {
            panic!("{what}: past the contact, at the planar lane: {err:?}");
        };
        let kind = a
            .get_edge(edge)
            .and_then(|e| a.get_curve_geom(e.curve))
            .and_then(topo::CurveGeom::certified)
            .map(|c| c.carrier().kind());
        assert_eq!(kind, Some(CurveKind::Nurbs), "{what}: a seam of A");
        let surface = b
            .get_face(face)
            .and_then(|f| b.get_surface(f.surface))
            .map(geom::Surface::kind);
        assert_eq!(surface, Some(SurfaceKind::Plane), "{what}: a cap of B");
    }
}
