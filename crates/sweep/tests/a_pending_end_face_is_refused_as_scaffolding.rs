//! **A blend whose chain ends on a face with no surface yet is refused as
//! construction scaffolding, never as a curved end face.** An Euler cube
//! whose five side and bottom faces are described as planes and whose
//! top face is left pending: the raw battery (`run_battery_for`, public
//! on a raw body) on each vertical edge, whose chain ends at the top
//! face, refuses `BodyNotIntact { breach: Scaffolding }` at that face for
//! fillet and chamfer alike; the bottom edges, which end on planes, read.
#![allow(clippy::unwrap_used, clippy::panic)]
use crate::euler_cube_support as cube;
use geom_core::{Point3, Tol, Vec3};
use sweep::blend::battery::{BlendRequest, run_battery_for};
use sweep::blend::{BlendError, BlendKind, Breach};
use topo::construction::{Construct, FaceSurface};

#[test]
fn a_pending_end_face_is_refused_as_scaffolding() {
    let mut body = topo::Body::<f64>::new();
    let c = cube::build_cube_pub(&mut body);
    let tol = Tol::witness();
    let plane = |o: Point3<f64>, n: Vec3<f64>, u: Vec3<f64>| topo::Surface::Plane {
        frame: geom::Frame::new(o, n, u),
    };
    let z = Point3::new(0.0, 0.0, 0.0);
    let one = Point3::new(1.0, 1.0, 1.0);
    let set = [
        (
            c.mefs[0].face,
            plane(z, Vec3::new(0.0, 0.0, -1.0), Vec3::new(1.0, 0.0, 0.0)),
        ),
        (
            c.mefs[1].face,
            plane(z, Vec3::new(0.0, -1.0, 0.0), Vec3::new(1.0, 0.0, 0.0)),
        ),
        (
            c.mefs[2].face,
            plane(one, Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
        ),
        (
            c.mefs[3].face,
            plane(one, Vec3::new(0.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0)),
        ),
        (
            c.mefs[4].face,
            plane(z, Vec3::new(-1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
        ),
    ];
    for (f, s) in set {
        body.set_face_surface(
            f,
            FaceSurface::New {
                surface: s,
                sense: true,
            },
        )
        .unwrap();
    }
    let top = c.seed.face;
    assert!(
        body.face_surface(top)
            .is_err_and(|m| m == topo::SurfaceMiss::Pending)
    );
    let band = geom_core::Band::linear(tol).unwrap();
    let mut refused = 0;
    for e in c.mevs.iter().map(|m| m.edge) {
        for kind in [BlendKind::Fillet, BlendKind::Chamfer] {
            let req = BlendRequest {
                body: &body,
                edges: vec![e],
                size: 0.1,
            };
            match run_battery_for(&req, band, kind) {
                Ok(_) => {}
                Err(BlendError::BodyNotIntact {
                    at: topo::EntityId::Face(f),
                    breach: Breach::Scaffolding,
                    ..
                }) if f == top => refused += 1,
                Err(other) => {
                    panic!("{e:?} {kind:?}: a pending end face is scaffolding: {other:?}")
                }
            }
        }
    }
    assert_eq!(
        refused, 8,
        "the four chains ending at the top face, fillet and chamfer"
    );
}
