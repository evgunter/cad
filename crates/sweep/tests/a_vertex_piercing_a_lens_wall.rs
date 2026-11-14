//! **A vertex piercing a lofted lens's NURBS wall**: the shape of the
//! lily's leaf grown out of its stem (the tour's lily wall 8), at a
//! scale the per-PR gate runs.
//!
//! The blade is a straight loft of one lens section, so its ridge wall
//! is a NURBS face minted at rest. A pyramid's apex sits on the ridge
//! at the wall's mid chart point; its base stands across the wall a
//! little way along the blade, so two of the apex's edges dip into the
//! blade and two rise out of it: the apex pierces the wall. The
//! vertex-on-face pass mints the pierce's ring in the complete wall
//! (`Body::ring_vertex`), and the union goes on to the join;
//! `a_vertex_piercing_a_lens_wall_mints_its_ring_and_reaches_the_chords_certificate`
//! holds where it stops at each ε.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point2, Point3, RigidMotion, Tol, Vec3};
use profile::test_support::bulge_loop;
use profile::{ProfileLoop, SketchPlane};
use sweep::loft_body;
use topo::{BooleanError, FaceKey};

/// The lens on chord `width`, rising `ridge` above it and dropping
/// `keel` below.
fn lens([width, ridge, keel]: [f64; 3]) -> Vec<ProfileLoop<f64>> {
    let half = 0.5 * width;
    vec![bulge_loop(vec![
        (Point2::new(half, 0.0), -ridge / half),
        (Point2::new(-half, 0.0), -keel / half),
    ])]
}

/// The lens lofted straight along `+x` from the origin to `x = 1`.
fn blade(tol: Tol) -> topo::AtRestBody<f64> {
    let band = Band::linear(tol).unwrap();
    let mut sections = Vec::new();
    let mut places = Vec::new();
    for k in 0..3 {
        let p = Point3::new(0.5 * f64::from(k), 0.0, 0.0);
        sections.push(lens([0.4, 0.1, 0.08]));
        places.push(SketchPlane::from_frame(
            RigidMotion::gram_schmidt(p, Vec3::unit_y(), Vec3::unit_z(), "lens", band).unwrap(),
        ));
    }
    let lofted = loft_body::<f64>(&sections, &places, 2, tol).unwrap().body;
    topo::test_support::finished("a lens blade", lofted, tol)
}

/// The blade's upper NURBS wall and its point at the middle of its knot
/// domain.
fn ridge(b: &topo::Body<f64>) -> (FaceKey, Point3<f64>) {
    b.faces()
        .filter_map(|(f, _)| match b.face_surface(f) {
            Some(geom::Surface::Nurbs(n)) => {
                let (u0, u1) = n.knot_values_u().domain();
                let (v0, v1) = n.knot_values_v().domain();
                Some((f, n.eval(0.5 * (u0 + u1), 0.5 * (v0 + v1))))
            }
            _ => None,
        })
        .max_by(|(_, p), (_, q)| p.z.total_cmp(&q.z))
        .expect("the lens has NURBS walls")
}

/// **The pierce's ring is minted in the complete wall in either operand
/// order, and the union stops past the join's reading of the carried
/// section** of the pyramid's planar face and the wall: at the at-rest
/// certificate of a chord the join cuts from it, where the plane × NURBS
/// lane's own image of the chord misses it
/// (`work/sectarms/a-planes-carried-arc-across-a-flared-spline-wall-misses-its-own-image-at-rest.md`),
/// or, at `ε = 1e-6`, past the ring the chords close on the wall, which
/// the join winds, at the inside/outside test of a body holding the
/// wall (`work/inside/point-containments-ray-test-has-no-spline-arm.md`).
/// A ring struck by a chord `mev` into the wall refused
/// `Euler(PcurveMint { face: wall, refusal: SplineChart })` here, as on
/// the lily.
#[test]
fn a_vertex_piercing_a_lens_wall_mints_its_ring_and_reaches_the_chords_certificate() {
    let tol = Tol::witness();
    let b = blade(tol);
    let (wall, p) = ridge(&b);
    assert!(
        p.z > 0.05,
        "the ridge wall's middle stands above the chord: {p:?}"
    );
    // Counterclockwise seen from the apex, at `−x` of the base.
    let base = [
        [p.x + 0.15, p.y - 0.06, p.z + 0.15],
        [p.x + 0.15, p.y + 0.06, p.z + 0.15],
        [p.x + 0.15, p.y + 0.06, p.z - 0.05],
        [p.x + 0.15, p.y - 0.06, p.z - 0.05],
    ];
    let pose = topo::test_support::meeting::Pose::rest();
    let pyr =
        topo::test_support::meeting::posed_pyramid_at::<f64>(&base, [p.x, p.y, p.z], &pose, tol);
    // At `ε = 1e-6` the run passes the join and meets the wall at the
    // inside/outside test; at every other row it stops before, at the
    // chord's at-rest certificate.
    let past_the_join = test_utils::kernel_limit::at_row(tol.eps(), &[1e-6]);
    for (what, x, y) in [("pyr ∪ blade", &pyr, &b), ("blade ∪ pyr", &b, &pyr)] {
        let got = topo::union(x, y, tol).map(|_| ());
        if past_the_join {
            assert!(
                matches!(
                    &got,
                    Err(BooleanError::Containment(topo::PointInSolidError::KindUnsupported {
                        face,
                        kind: geom::SurfaceKind::Nurbs,
                    })) if *face == wall
                ),
                "{what}: {got:?}, not the inside/outside test's refusal of the wall {wall:?}"
            );
        } else {
            assert!(
                matches!(
                    &got,
                    Err(BooleanError::Join(join)) if matches!(
                        join.error(),
                        topo::JoinError::Euler(topo::EulerOpError::Certification {
                            error: geom_brep::CertifyError::PlaneNurbs(_),
                        })
                    )
                ),
                "{what}: {got:?}, not the plane × NURBS certificate of a chord the join cut on \
                 the carried section"
            );
        }
    }
}
