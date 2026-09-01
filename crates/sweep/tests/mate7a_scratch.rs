//! SCRATCH measurement (MATE-7a) — deleted before the PR.

use geom_core::{Point3, Tol, Vec3};
use sweep::{TubeWindow, tube_along_arc, tube_along_arc_hollow};
use topo::{Body, BooleanDeclarations, BooleanResult, ContactClass, FaceKey, FacePairDeclaration};

fn center() -> Point3<f64> {
    Point3::new(-5.0, 0.0, 0.0)
}
fn axis() -> Vec3<f64> {
    Vec3::new(0.0, -1.0, 0.0)
}
fn u_ref() -> Vec3<f64> {
    Vec3::new(1.0, 0.0, 0.0)
}
fn window() -> TubeWindow<f64> {
    TubeWindow::Arc {
        t0: 0.0,
        t1: 22.0_f64.to_radians(),
    }
}

/// The toroidal SOCKET: a hollow elbow, outer tube 0.09, wall 0.03 —
/// so its BORE is a torus of minor radius 0.06.
fn socket() -> Body<f64> {
    tube_along_arc_hollow(
        center(),
        axis(),
        u_ref(),
        5.0,
        window(),
        0.09,
        0.03,
        Tol::witness(),
    )
    .expect("socket builds")
    .body
}

/// The exactly-filling toroidal PEG: the same elbow, solid, minor 0.06.
fn peg() -> Body<f64> {
    tube_along_arc(center(), axis(), u_ref(), 5.0, window(), 0.06, Tol::witness())
        .expect("peg builds")
        .body
}

fn torus_faces(body: &Body<f64>, minor: f64) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Torus { minor_radius, .. })
                    if (*minor_radius - minor).abs() < 1e-12
            )
        })
        .map(|(k, _)| k)
        .collect()
}

#[test]
fn mate7a_torus_peg_in_bore() {
    let tol = Tol::witness();
    let (s, p) = (socket(), peg());
    for (name, body) in [("socket", &s), ("peg", &p)] {
        println!("--- {name} ---");
        for (k, f) in body.faces() {
            println!("  {k:?} sense={} {:?}", f.sense, body.get_surface(f.surface));
        }
    }
    let bore = torus_faces(&s, 0.06);
    let wall = torus_faces(&p, 0.06);
    println!("bore {bore:?} wall {wall:?}");
    let mut decls = BooleanDeclarations::none();
    for &fa in &bore {
        for &fb in &wall {
            decls
                .coincident_faces
                .push(FacePairDeclaration::new(fa, fb, ContactClass::Rest));
        }
    }
    println!("undeclared: {:?}", topo::union(&s, &p, tol).err());
    match topo::union_with(&s, &p, &decls, tol) {
        Ok(BooleanResult::Body(bb)) => println!(
            "PEG-IN-BORE UNIONS: kind {:?} faces {} vol {:?}",
            bb.kind,
            bb.body.faces().count(),
            topo::mass_properties(&bb.body, tol).map(|m| m.volume)
        ),
        Ok(BooleanResult::Empty) => println!("PEG-IN-BORE EMPTY"),
        Err(e) => println!("PEG-IN-BORE refuses: {e:?}"),
    }
}
