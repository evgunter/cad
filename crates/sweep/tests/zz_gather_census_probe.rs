#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use geom_core::{Affine3, Point2, Tol, Vec2, Vec3};
use profile::test_support::bulge_loop;
use profile::{Open, Profile, ProfileLoop, SketchPlane, Start, ValidatedProfile};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, loft_body, revolve};
use topo::{Body, ContactRecords, CurveContact, DeclaredContact, EdgeKey, FaceKey};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}
fn lune() -> ProfileLoop<f64> {
    let tol = Tol::witness();
    Open.at(p2(0.0, 4.0))
        .angle(-std::f64::consts::FRAC_PI_2, tol)
        .unwrap()
        .line(2.0, tol)
        .unwrap()
        .turn(std::f64::consts::FRAC_PI_2, tol)
        .unwrap()
        .tangent_arc_to(p2(0.0, 0.0), tol)
        .unwrap()
        .cusp()
        .tangent_arc_to(Start, tol)
        .unwrap()
        .into()
}
fn crescent() -> ProfileLoop<f64> {
    let tol = Tol::witness();
    let h = std::f64::consts::FRAC_1_SQRT_2;
    Open.at(p2(1.0, 0.0))
        .angle(std::f64::consts::FRAC_PI_2, tol)
        .unwrap()
        .tangent_arc_to(p2(h, h), tol)
        .unwrap()
        .cusp()
        .line(1.0, tol)
        .unwrap()
        .line_to(Start, tol)
        .unwrap()
        .into()
}
fn on_axis() -> ProfileLoop<f64> {
    let tol = Tol::witness();
    Open.at(p2(0.0, 1.0))
        .angle(std::f64::consts::FRAC_PI_2, tol)
        .unwrap()
        .line(std::f64::consts::SQRT_2 - 1.0, tol)
        .unwrap()
        .turn(-3.0 * std::f64::consts::FRAC_PI_4, tol)
        .unwrap()
        .line(1.0, tol)
        .unwrap()
        .cusp()
        .tangent_arc_to(Start, tol)
        .unwrap()
        .into()
}
fn validated(loops: Vec<ProfileLoop<f64>>) -> ValidatedProfile<f64> {
    Profile::new(SketchPlane::xy(), loops)
        .validate(Tol::witness())
        .unwrap()
}
fn faces(body: &Body<f64>, edge: EdgeKey) -> [FaceKey; 2] {
    let e = body.get_edge(edge).unwrap();
    let f = |he| {
        body.get_loop(body.get_half_edge(he).unwrap().parent_loop)
            .unwrap()
            .face
    };
    let mut p = [f(e.he_plus), f(e.he_minus)];
    p.sort();
    p
}
fn records(body: &Body<f64>, d: &[DeclaredContact], candidates: &[EdgeKey]) -> ContactRecords {
    let mut r = ContactRecords::default();
    for c in d {
        let mut p = [c.a, c.b];
        p.sort();
        let w: Vec<_> = candidates
            .iter()
            .copied()
            .filter(|&e| faces(body, e) == p)
            .collect();
        eprintln!("  pair {p:?}: {} witness edges", w.len());
        for e in w {
            r.curves.push(CurveContact {
                face_a: c.a,
                face_b: c.b,
                witness: e,
            });
        }
    }
    r
}
fn probe(label: &str, body: &Body<f64>, d: &[DeclaredContact], cands: &[EdgeKey]) {
    let tol = Tol::witness();
    eprintln!("== {label}: declared {}", d.len());
    let r = records(body, d, cands);
    eprintln!(
        "  geometric: {:?}",
        topo::validate_geometric(body, tol).err()
    );
    eprintln!(
        "  geometric_declared: {:?}",
        topo::validate_geometric_declared(body, d, tol)
    );
    eprintln!(
        "  pseudomanifold(empty): {:?}",
        topo::validate_pseudomanifold(body, &ContactRecords::default(), tol)
    );
    eprintln!(
        "  pseudomanifold(curves): {:?}",
        topo::validate_pseudomanifold(body, &r, tol)
    );
}

#[test]
fn gather_census_probe() {
    let tol = Tol::witness();
    for d in [1.0, -1.0] {
        let b = extrude(&validated(vec![lune()]), Extrusion::Distance(d), tol).unwrap();
        let s: Vec<_> = b.strut_edges.concat();
        probe(
            &format!("extrude lune {d}"),
            &b.body,
            &b.declared_contacts,
            &s,
        );
    }
    let plate = bulge_loop(vec![
        (p2(-1.0, -1.0), 0.0),
        (p2(3.0, -1.0), 0.0),
        (p2(3.0, 5.0), 0.0),
        (p2(-1.0, 5.0), 0.0),
    ]);
    let b = extrude(
        &validated(vec![plate, lune()]),
        Extrusion::Distance(1.0),
        tol,
    )
    .unwrap();
    probe(
        "extrude hole lune",
        &b.body,
        &b.declared_contacts,
        &b.strut_edges.concat(),
    );
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    for rev in [Revolution::Partial(1.0), Revolution::Full] {
        let b = revolve(&validated(vec![crescent()]), axis, rev, tol).unwrap();
        let all: Vec<EdgeKey> = b.body.edges().map(|(k, _)| k).collect();
        probe(
            &format!("revolve crescent {rev:?}"),
            &b.body,
            &b.declared_contacts,
            &all,
        );
    }
    let b = revolve(&validated(vec![on_axis()]), axis, Revolution::Full, tol).unwrap();
    let all: Vec<EdgeKey> = b.body.edges().map(|(k, _)| k).collect();
    probe("revolve on-axis full", &b.body, &b.declared_contacts, &all);
    let places: Vec<Affine3<f64>> = [0.0, 1.0]
        .iter()
        .map(|z| Affine3::translation(Vec3::new(0.0, 0.0, *z)))
        .collect();
    let b = loft_body::<f64>(&[vec![lune()], vec![lune()]], &places, 1, tol).unwrap();
    probe(
        "loft lune",
        &b.body,
        &b.declared_contacts,
        &b.seam_edges.concat(),
    );
}
