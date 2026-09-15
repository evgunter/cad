//! R2 review probes for SENSE-FOLD (PR 2668): an end-to-end program a
//! user would write, run at the merge base and at the head, writing
//! every bit it reads to `$R2_OUT` so the two runs can be diffed.
//!
//! Three exercises: a reversed planar face and a reversed cylinder
//! wall through `validate_geometric` (check 6 reads the planar door,
//! check 4 the bit); a convex and a concave fillet whose ball sides
//! differ (`SupportTrace.side`); and a declared kissing-torus union
//! that reaches the contact verifier with same and mixed senses.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use std::fmt::Write as _;

use crate::common::cavity::{cavity_edges, edges_with_corners, rod, vented_cavity};
use geom_core::{Point2, Point3, Tol, Vec3};
use sweep::blend::build::fillet_edges;
use sweep::test_support::cube;
use sweep::{TubeWindow, tube_along_arc};
use topo::query;
use topo::{
    Body, BooleanDeclarations, ContactClass, FaceKey, FacePairDeclaration, validate_geometric,
};

fn out(name: &str, text: &str) {
    let dir = std::env::var("R2_OUT").expect("R2_OUT names the output directory");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(format!("{dir}/{name}.txt"), text).unwrap();
}

fn bits(x: f64) -> String {
    format!("{x:?} [{:016x}]", x.to_bits())
}

/// Every surface, every face bit and the mass properties of a body,
/// as text whose every number is exact.
fn dump_body(label: &str, body: &Body<f64>, s: &mut String) {
    writeln!(s, "## {label}").unwrap();
    let mut faces: Vec<String> = body
        .faces()
        .map(|(_, f)| {
            let surf = body.get_surface(f.surface).unwrap();
            format!("face sense={} surface={surf:?}", f.sense)
        })
        .collect();
    faces.sort();
    for f in faces {
        writeln!(s, "{f}").unwrap();
    }
    match topo::props::mass_properties(body, Tol::witness()) {
        Ok(m) => {
            writeln!(s, "volume {}", bits(m.volume)).unwrap();
            writeln!(s, "area {}", bits(m.surface_area)).unwrap();
        }
        Err(e) => writeln!(s, "mass_properties refused: {e:?}").unwrap(),
    }
    let mut errors: Vec<String> = match validate_geometric(body, Tol::witness()) {
        Ok(()) => vec!["tier3 ok".to_string()],
        Err(errs) => errs.iter().map(|e| format!("{e:?}")).collect(),
    };
    errors.sort();
    for e in errors {
        writeln!(s, "tier3: {e}").unwrap();
    }
}

/// **Probe A.** A user reverses one face of a cube and one wall of a
/// rod, and asks tier 3 which faces are wrong.
#[test]
fn r2_reversed_faces_through_validate_geometric() {
    let mut s = String::new();
    let c = cube(2.0, Tol::witness());
    dump_body("cube honest", &c, &mut s);
    // The +z cap: every boundary vertex at z = 2.
    let top: Vec<FaceKey> = query::all_faces(&c)
        .into_iter()
        .filter(|&f| {
            matches!(
                c.get_face(f).and_then(|fd| c.get_surface(fd.surface)),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if normal.z.abs() > 0.5 && origin.z > 1.0
            )
        })
        .collect();
    assert_eq!(top.len(), 1, "one +z cap");
    let flipped = c.flipped_face_sense_for_tests(top[0]).unwrap();
    dump_body("cube with the +z cap reversed", &flipped, &mut s);

    let r = rod(Point2::new(0.0, 0.0), 0.5, 0.0, 2.0);
    dump_body("rod honest", &r, &mut s);
    let walls: Vec<FaceKey> = query::all_faces(&r)
        .into_iter()
        .filter(|&f| {
            matches!(
                r.get_face(f).and_then(|fd| r.get_surface(fd.surface)),
                Some(geom::Surface::Cylinder { .. })
            )
        })
        .collect();
    assert!(!walls.is_empty(), "the rod has a cylinder wall");
    let flipped = r.flipped_face_sense_for_tests(walls[0]).unwrap();
    dump_body("rod with one cylinder wall reversed", &flipped, &mut s);
    out("probe_a_validate", &s);
}

/// **Probe B.** A convex fillet (every edge of a cube) and a concave
/// one (every cavity edge of the vented cavity): the ball sides differ,
/// and every minted surface's bits are recorded.
#[test]
fn r2_convex_and_concave_fillets_bits() {
    let mut s = String::new();
    let c = cube(2.0, Tol::witness());
    let edges = query::all_edges(&c);
    let convex = fillet_edges(&c, &edges, 0.25, Tol::witness())
        .unwrap_or_else(|e| panic!("the cube fillets whole: {e:?}"));
    dump_body("cube, all twelve edges filleted r=0.25", &convex.body, &mut s);
    // One convex edge alone, too: the top +x edge.
    let one = edges_with_corners(&c, |p: Point3<f64>| {
        (p.x - 2.0).abs() < 1e-12 && (p.z - 2.0).abs() < 1e-12
    });
    assert_eq!(one.len(), 1);
    match fillet_edges(&c, &one, 0.3, Tol::witness()) {
        Ok(f) => dump_body("cube, one edge filleted r=0.3", &f.body, &mut s),
        Err(e) => writeln!(s, "## cube, one edge: refused {e:?}").unwrap(),
    }

    let v = vented_cavity();
    let ce = cavity_edges(&v);
    let concave = fillet_edges(&v, &ce, 0.25, Tol::witness())
        .unwrap_or_else(|e| panic!("the cavity fillets whole: {e:?}"));
    dump_body("vented cavity, all cavity edges filleted r=0.25", &concave.body, &mut s);
    out("probe_b_fillets", &s);
}

const TUBE: f64 = 0.06;
const RING: f64 = 5.0;

fn full_torus(major: f64) -> Body<f64> {
    tube_along_arc(
        Point3::origin(),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
        major,
        TubeWindow::Full,
        TUBE,
        Tol::witness(),
    )
    .expect("the full torus builds")
    .body
}

fn torus_faces(body: &Body<f64>) -> Vec<FaceKey> {
    query::all_faces(body)
        .into_iter()
        .filter(|&f| {
            matches!(
                body.get_face(f).and_then(|fd| body.get_surface(fd.surface)),
                Some(geom::Surface::Torus { minor_radius, .. })
                    if (*minor_radius - TUBE).abs() < 1e-12
            )
        })
        .collect()
}

/// **Probe C.** A declared kissing-torus union that reaches the
/// contact verifier, with the two bodies' walls at the same sense and
/// at opposite senses.
#[test]
fn r2_declared_kissing_tori_reach_the_contact_verifier() {
    let mut s = String::new();
    let a = full_torus(RING);
    let mut b = full_torus(RING + 2.0 * TUBE);
    for pass in ["same senses", "second body's walls reversed"] {
        let mut decls = BooleanDeclarations::none();
        for &fa in &torus_faces(&a) {
            for &fb in &torus_faces(&b) {
                decls
                    .coincident_faces
                    .push(FacePairDeclaration::new(fa, fb, ContactClass::Tangent));
            }
        }
        let senses: Vec<bool> = torus_faces(&b)
            .iter()
            .map(|&f| b.get_face(f).unwrap().sense)
            .collect();
        writeln!(s, "## {pass}: b wall senses {senses:?}").unwrap();
        match topo::union_with(&a, &b, &decls, Tol::witness()) {
            Ok(r) => match r.body() {
                Some(bb) => dump_body("union built", &bb.body, &mut s),
                None => writeln!(s, "union empty").unwrap(),
            },
            Err(e) => writeln!(s, "union refused: {e:?}").unwrap(),
        }
        for f in torus_faces(&b) {
            let cur = b.get_face(f).unwrap().sense;
            b.set_face_sense(f, !cur).unwrap();
        }
    }
    out("probe_c_contact", &s);
}
