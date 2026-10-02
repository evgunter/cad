//! The split's operand gate, one row per kind the split has no arm
//! for: the unit cube with its top face's surface swapped for a small
//! sphere, torus, spline or approximated spline sitting at the top
//! face. A plane through the cube's mid-height clears that surface's
//! box and reduces; a tilted plane through the top face may meet it
//! and refuses naming the face, its kind and the recourse.
//!
//! The swapped face strands the top face's edge descriptions, so these
//! rows read the gate through `split_reduce` only; a whole split of a
//! body carrying each kind is `sweep`'s `reach_split_gate_per_face`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::common;
use geom::{NurbsSurface, Surface, SurfaceKind};
use geom_core::spline::KnotVector;
use geom_core::{Point3, Tol, Vec3};
use topo::{Body, FaceKey, FaceSurface, SplitPlane, SplitReduceError, split_reduce};

/// A bilinear patch over the cube's top face, lifted to `z = 1 + lift`.
fn top_patch(lift: f64) -> NurbsSurface<f64> {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)]
        .map(|(x, y)| Point3::new(x, y, 1.0 + lift))
        .to_vec();
    NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 4]).unwrap()
}

/// Every surface the split has no arm for, each boxed near `z = 1`.
fn unarmed_tops() -> Vec<Surface<f64>> {
    let approx = geom_brep::approx_offset_surface(Arc::new(top_patch(0.0)), 0.01, Tol::witness())
        .expect("a flat patch's offset mints");
    vec![
        Surface::Sphere {
            center: Point3::new(0.5, 0.5, 1.0),
            radius: 0.1,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
        },
        Surface::Torus {
            center: Point3::new(0.5, 0.5, 1.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            major_radius: 0.2,
            minor_radius: 0.05,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        },
        Surface::Nurbs(Arc::new(top_patch(0.0))),
        Surface::Approx(approx),
    ]
}

/// The described unit cube with its top face on `surface`.
fn cube_topped(surface: Surface<f64>) -> (Body<f64>, FaceKey) {
    let cube = common::geometric_cube::<f64>(Tol::witness());
    let mut body = cube.body;
    common::describe_as_intersections(&mut body, Tol::witness());
    body.set_face_surface_stranding_for_tests(
        cube.seed.face,
        FaceSurface::New {
            surface,
            sense: true,
        },
    )
    .unwrap();
    (body, cube.seed.face)
}

fn plane(origin: Point3<f64>, normal: Vec3<f64>) -> SplitPlane<f64> {
    topo::test_support::split_plane(origin, normal, Tol::witness())
}

#[test]
fn an_unarmed_face_refuses_only_where_the_plane_may_meet_it() {
    let clear = plane(Point3::new(0.5, 0.5, 0.5), Vec3::new(0.0, 0.0, 1.0));
    let through = plane(Point3::new(0.5, 0.5, 1.0), Vec3::new(0.3, 0.0, 1.0));
    for surface in unarmed_tops() {
        let kind = surface.kind();
        let (body, top) = cube_topped(surface);
        let reduced = split_reduce(&body, &clear, Tol::witness())
            .unwrap_or_else(|e| panic!("{kind:?}: a plane clear of the face reduces: {e}"));
        assert_eq!(
            reduced.on_vertices.len(),
            4,
            "{kind:?}: the mid-height plane crosses the four struts"
        );
        match split_reduce(&body, &through, Tol::witness()) {
            Err(e @ SplitReduceError::CurvedBooleanUnsupported { face, kind: k }) => {
                assert_eq!(
                    (face, k),
                    (top, kind),
                    "{kind:?}: the refusal names the face"
                );
                let msg = e.to_string();
                assert!(
                    msg.contains("may meet") && msg.contains("Recourse: move the split plane"),
                    "{kind:?}: {msg}"
                );
            }
            other => panic!("{kind:?}: a plane through the face refuses, got {other:?}"),
        }
    }
}

/// The box is padded, so a plane just clear of the sphere's ball — inside
/// the pad — still refuses, and one clear of the pad reduces. The pad is
/// the boolean sweep's, `escalate + 2·zero`, a few ε.
#[test]
fn the_box_is_read_with_its_pad() {
    let eps = Tol::witness().get().eps;
    let ball = Surface::Sphere {
        center: Point3::new(0.5, 0.5, 1.0),
        radius: 0.1,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
    };
    let (body, _) = cube_topped(ball);
    let at = |z: f64| plane(Point3::new(0.5, 0.5, z), Vec3::new(0.0, 0.0, 1.0));
    assert!(
        matches!(
            split_reduce(&body, &at(0.9 - eps), Tol::witness()),
            Err(SplitReduceError::CurvedBooleanUnsupported {
                kind: SurfaceKind::Sphere,
                ..
            })
        ),
        "a plane an ε below the ball is inside the pad"
    );
    split_reduce(&body, &at(0.9 - 1e4 * eps), Tol::witness())
        .expect("a plane clear of the padded ball reduces");
}
