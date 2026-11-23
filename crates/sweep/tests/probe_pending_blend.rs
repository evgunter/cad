//! Reviewer probe: the raw battery on a cube whose top face is pending.
#[path = "probe_support/cube.rs"]
mod cube;
use geom_core::{Point3, Tol, Vec3};
use sweep::blend::BlendKind;
use sweep::blend::battery::{BlendRequest, run_battery_for};
use topo::construction::{Construct, FaceSurface};

#[test]
fn probe_pending_blend() {
    let mut body = topo::Body::<f64>::new();
    let c = cube::build_cube_pub(&mut body);
    let tol = Tol::witness();
    let plane = |o: Point3<f64>, n: Vec3<f64>, u: Vec3<f64>| topo::Surface::Plane { frame: geom::Frame::new(o, n, u) };
    let z = Point3::new(0.0, 0.0, 0.0);
    let one = Point3::new(1.0, 1.0, 1.0);
    let set = [
        (c.mefs[0].face, plane(z, Vec3::new(0.0, 0.0, -1.0), Vec3::new(1.0, 0.0, 0.0))),
        (c.mefs[1].face, plane(z, Vec3::new(0.0, -1.0, 0.0), Vec3::new(1.0, 0.0, 0.0))),
        (c.mefs[2].face, plane(one, Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0))),
        (c.mefs[3].face, plane(one, Vec3::new(0.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0))),
        (c.mefs[4].face, plane(z, Vec3::new(-1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0))),
    ];
    for (f, s) in set {
        body.set_face_surface(f, FaceSurface::New { surface: s, sense: true }).unwrap();
    }
    println!("DOOR geometric: {:?}", topo::validate::validate_geometric(&body, tol));
    let band = geom_core::Band::linear(tol).unwrap();
    // every vertical edge (struts) and the bottom edges
    let edges: Vec<_> = c.mevs.iter().map(|m| m.edge).collect();
    for e in edges {
        for kind in [BlendKind::Fillet, BlendKind::Chamfer] {
            let req = BlendRequest { body: &body, edges: vec![e], size: 0.1 };
            let r = run_battery_for(&req, band, kind);
            println!("DOOR battery {e:?} {kind:?}: {:?}", r.as_ref().map(|_| ()).map_err(|e| format!("{e:?}")));
        }
    }
}
