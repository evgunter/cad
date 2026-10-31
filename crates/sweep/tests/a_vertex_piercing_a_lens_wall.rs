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
//! (`Body::ring_vertex`), and the union goes on to the join.
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
        .filter_map(|(f, fd)| match b.get_surface(fd.surface) {
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
/// order, and the union stops at the join's germ frame** for the
/// pyramid's planar face against the wall, which has no section frame
/// (`work/sectarms/a-nurbs-wall-against-a-plane-has-no-germ-frame`). A
/// ring struck by a chord `mev` into the wall refused
/// `Euler(PcurveMint { face: wall, refusal: SplineChart })` here, as on
/// the lily.
#[test]
fn a_vertex_piercing_a_lens_wall_mints_its_ring_and_reaches_the_germ_frame() {
    let tol = Tol::witness();
    let b = blade(tol);
    let (wall, p) = ridge(&b);
    assert!(
        p.z > 0.05,
        "the ridge wall's middle stands above the chord: {p:?}"
    );
    // Counterclockwise seen from the apex, at `−x` of the base.
    let base = [
        [p.x + 0.05, p.y - 0.02, p.z + 0.05],
        [p.x + 0.05, p.y + 0.02, p.z + 0.05],
        [p.x + 0.05, p.y + 0.02, p.z - 0.01],
        [p.x + 0.05, p.y - 0.02, p.z - 0.01],
    ];
    let pose = topo::test_support::meeting::Pose::rest();
    let pyr =
        topo::test_support::meeting::posed_pyramid_at::<f64>(&base, [p.x, p.y, p.z], &pose, tol);
    for (what, x, y, wall_first) in [
        ("pyr ∪ blade", &pyr, &b, false),
        ("blade ∪ pyr", &b, &pyr, true),
    ] {
        let got = topo::union(x, y, tol).map(|_| ());
        let at_frame = match &got {
            Err(BooleanError::GermFrameUnsupported {
                a_face,
                a_kind,
                b_face,
                b_kind,
            }) => {
                let (w, w_kind, other_kind) = if wall_first {
                    (a_face, a_kind, b_kind)
                } else {
                    (b_face, b_kind, a_kind)
                };
                *w == wall
                    && *w_kind == geom::SurfaceKind::Nurbs
                    && *other_kind == geom::SurfaceKind::Plane
            }
            _ => false,
        };
        assert!(
            at_frame,
            "{what}: {got:?}, not the plane × NURBS germ frame at the wall {wall:?}"
        );
    }
}
