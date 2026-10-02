//! The split's operand gate, one row per kind the split has no arm
//! for: the unit cube with its top face's surface swapped for a small
//! sphere, torus, spline or approximated spline whose box straddles
//! the level `z = 1.008`, just above the top face. A plane through the cube's mid-height
//! clears that box and reduces. A level plane just above the top face
//! meets the box and none of the cube's edges or vertices, so nothing
//! but the gate can refuse it: each kind's refusal is read through
//! `vertex_sides`, the gate and the vertex sweep alone.
//!
//! The swapped face strands the top face's edge descriptions, so these
//! rows read the gate and never a whole split; a whole split of a body
//! carrying each kind is `sweep`'s `reach_split_gate_per_face`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::common;
use geom::{NurbsSurface, Surface, SurfaceKind};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Tol, Vec3};
use topo::{Body, FaceKey, FaceSurface, SplitPlane, SplitReduceError, split_reduce, vertex_sides};

/// A gently bowed biquadratic patch over the cube's top face, its
/// control net spanning `z ∈ [1, 1.015]` (the bowed patch
/// `rigid_map_near_eps_approx` offsets, lifted to the top face).
fn bowed_patch() -> NurbsSurface<f64> {
    const BOW: f64 = 1.5e-2;
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let mut control = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            let (u, v) = (f64::from(i) * 0.5, f64::from(j) * 0.5);
            let z = 1.0 + BOW * u * (1.0 - u) + (BOW * 2.0 / 3.0) * v * v;
            control.push(Point3::new(u, v, z));
        }
    }
    NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 9]).unwrap()
}

/// Every surface the split has no arm for, each boxed across
/// `z = 1.008`: the offset lies 5 mm off the bowed patch on either side.
fn unarmed_tops() -> Vec<Surface<f64>> {
    let approx = geom_brep::approx_offset_surface(Arc::new(bowed_patch()), 0.005, Tol::witness())
        .expect("the bowed patch's offset mints");
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
        Surface::Nurbs(Arc::new(bowed_patch())),
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

/// The level plane `z = h`.
fn level(h: f64) -> SplitPlane<f64> {
    topo::test_support::split_plane(
        Point3::new(0.5, 0.5, h),
        Vec3::new(0.0, 0.0, 1.0),
        Tol::witness(),
    )
}

#[test]
fn an_unarmed_face_refuses_only_where_the_plane_may_meet_it() {
    for surface in unarmed_tops() {
        let kind = surface.kind();
        let (body, top) = cube_topped(surface);
        let reduced = split_reduce(&body, &level(0.5), Tol::witness())
            .unwrap_or_else(|e| panic!("{kind:?}: a plane clear of the face reduces: {e}"));
        assert_eq!(
            reduced.on_vertices.len(),
            4,
            "{kind:?}: the mid-height plane crosses the four struts"
        );
        match vertex_sides(&body, &level(1.008), Tol::witness()) {
            Err(e @ SplitReduceError::CurvedBooleanUnsupported { face, kind: k }) => {
                assert_eq!(
                    (face, k),
                    (top, kind),
                    "{kind:?}: the refusal names the face"
                );
                let msg = e.to_string();
                assert!(
                    msg.contains("may meet")
                        && msg.contains(
                            "Recourse: move the split plane clear of that face's bounding box"
                        ),
                    "{kind:?}: {msg}"
                );
            }
            other => panic!("{kind:?}: a plane through the face's box refuses, got {other:?}"),
        }
    }
}

/// The box is padded by the boolean sweep's pad, `escalate + 2·zero`. A
/// plane `escalate + 1.5·zero` below the ball is a definite distance
/// from the bare box and inside the padded one, so it refuses only
/// through the pad. One a whole pad further down reduces.
#[test]
fn the_box_is_read_with_its_pad() {
    let band = Band::linear(Tol::witness()).unwrap();
    let (escalate, zero) = (band.escalate(), band.zero());
    let ball = Surface::Sphere {
        center: Point3::new(0.5, 0.5, 1.0),
        radius: 0.1,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
    };
    let (body, _) = cube_topped(ball);
    assert!(
        matches!(
            vertex_sides(&body, &level(0.9 - (escalate + 1.5 * zero)), Tol::witness()),
            Err(SplitReduceError::CurvedBooleanUnsupported {
                kind: SurfaceKind::Sphere,
                ..
            })
        ),
        "a plane inside the pad and definitely clear of the bare box refuses"
    );
    vertex_sides(
        &body,
        &level(0.9 - 2.0 * (escalate + 2.0 * zero)),
        Tol::witness(),
    )
    .expect("a plane clear of the padded ball passes the gate");
}
