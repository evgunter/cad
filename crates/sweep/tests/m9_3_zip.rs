//! M9-3 PR-B — the zip and the marks: the two-peg kernel path
//! (CONTACT-DESIGN's considered-not-built demo, now built): plate P
//! with two pegs, plate Q with two through-bores, mated on one plane;
//! three declared `Rest` contact groups (one planar + two
//! cylindrical); the union removes all three patches as interior,
//! the bore walls vanish (full engagement), and the volume is exactly
//! additive (the C7-lane statement).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{
    Body, BooleanDeclarations, BooleanResult, ContactClass, FacePairDeclaration, mass_properties,
};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// A rectangular plate [0,6]×[0,4], z ∈ [z0, z0 + 1].
fn plate(z0: f64) -> Body<f64> {
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(6.0, 0.0), p2(6.0, 4.0), p2(0.0, 4.0)]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(1.0), Tol::witness())
        .unwrap()
        .body
}

/// A radius-0.5 three-arc cylinder at (cx, 2), z ∈ [z0, z0 + h].
fn cyl(cx: f64, z0: f64, h: f64) -> Body<f64> {
    let b120 = (core::f64::consts::PI / 6.0).tan();
    let at = |deg: f64| {
        let th = deg.to_radians();
        p2(cx + 0.5 * th.cos(), 2.0 + 0.5 * th.sin())
    };
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(at(0.0), b120),
        ProfileVertex::new(at(120.0), b120),
        ProfileVertex::new(at(240.0), b120),
    ]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(h), Tol::witness())
        .unwrap()
        .body
}

fn body_of(r: BooleanResult<f64>) -> Body<f64> {
    match r {
        BooleanResult::Body(b) => b.body,
        BooleanResult::Empty => panic!("a two-peg operand cannot be empty"),
    }
}

/// Plate P: base [0,1] with two pegs rising to z = 2 (embedded boss
/// unions — the shipped transverse lane).
fn plate_with_pegs() -> Body<f64> {
    let p0 = plate(0.0);
    let p1 = body_of(topo::union(&p0, &cyl(2.0, 0.4, 1.6), Tol::witness()).unwrap());
    body_of(topo::union(&p1, &cyl(4.0, 0.4, 1.6), Tol::witness()).unwrap())
}

/// Plate Q: z ∈ [1, 2] with two through-bores (the shipped transverse
/// subtracts).
fn plate_with_bores() -> Body<f64> {
    let q0 = plate(1.0);
    let q1 = body_of(topo::subtract(&q0, &cyl(2.0, 0.8, 1.4), Tol::witness()).unwrap());
    body_of(topo::subtract(&q1, &cyl(4.0, 0.8, 1.4), Tol::witness()).unwrap())
}

/// The cylinder faces of a body whose axis x is near `cx`.
fn walls_at(body: &Body<f64>, cx: f64) -> Vec<topo::FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Cylinder { origin, .. }) if (origin.x - cx).abs() < 0.5
            )
        })
        .map(|(k, _)| k)
        .collect()
}

/// The planar face at height `z` facing `up`.
fn plane_face(body: &Body<f64>, z: f64, up: bool) -> topo::FaceKey {
    let hits: Vec<_> = body
        .faces()
        .filter(|(_, f)| match body.get_surface(f.surface) {
            Some(geom::Surface::Plane { origin, normal, .. }) => {
                (origin.z - z).abs() < 1e-12 && (normal.z > 0.5) == up
            }
            _ => false,
        })
        .map(|(k, _)| k)
        .collect();
    let [f] = hits[..] else {
        panic!("expected exactly one z = {z} face (up = {up}), got {hits:?}");
    };
    f
}

/// The three declared contact groups: the mating plane (P top × Q
/// bottom) and the two cylinder bands, each peg declared against its
/// own bore's walls only (cross-peg pairs are DISTINCT carriers and
/// would be contradicted — correctly).
fn declarations(p: &Body<f64>, q: &Body<f64>) -> BooleanDeclarations {
    let mut decls = BooleanDeclarations::none();
    decls.coincident_faces.push(FacePairDeclaration::new(
        plane_face(p, 1.0, true),
        plane_face(q, 1.0, false),
        ContactClass::Rest,
    ));
    for cx in [2.0, 4.0] {
        for &fa in &walls_at(p, cx) {
            for &fb in &walls_at(q, cx) {
                decls
                    .coincident_faces
                    .push(FacePairDeclaration::new(fa, fb, ContactClass::Rest));
            }
        }
    }
    decls
}

/// Acceptance (i): the two-peg kernel path — three declared contacts,
/// union succeeds, volume EXACTLY additive (vol(P∪Q) = vol(P)+vol(Q):
/// interiors are disjoint, nothing is discarded — and the π terms of
/// the pegs and bores cancel against 48 exactly).
#[test]
fn two_peg_plate_union_is_exactly_additive() {
    let p = plate_with_pegs();
    let q = plate_with_bores();
    let vp = mass_properties(&p, Tol::witness()).unwrap().volume;
    let vq = mass_properties(&q, Tol::witness()).unwrap().volume;
    let decls = declarations(&p, &q);
    let out =
        topo::union_with(&p, &q, &decls, Tol::witness()).expect("the two-peg kernel path unions");
    let body = body_of(out);
    let v = mass_properties(&body, Tol::witness()).unwrap().volume;
    assert_eq!(v, vp + vq, "exactly additive (the C7-lane statement)");
    // The bore walls vanished with full engagement: no cylinder
    // surface survives anywhere in the result.
    assert!(
        body.faces().all(|(_, f)| !matches!(
            body.get_surface(f.surface),
            Some(geom::Surface::Cylinder { .. })
        )),
        "full-engagement patch removal deletes every wall face"
    );
    if let Err(errs) = topo::validate_geometric(&body, Tol::witness()) {
        panic!("the mated pair must be tier-3 valid: {errs:?}");
    }
}
