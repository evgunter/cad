//! Issue 1032 — a declared cylindrical `Rest` that is the mate's ONLY
//! contact: a shaft in a bore with no shoulder, flange or seat.
//!
//! The rows are the issue's own four spellings, held at the two-peg
//! acceptance fixture's face structure (3 declared wall faces against
//! 3, both rims `circle_split`-style 120° arcs):
//!
//!   (1) PARTIAL engagement — the peg proud of the bore at both ends;
//!   (2) FULL engagement — the peg exactly spanning the collar;
//!   (3) the same mate at fixture (i)'s exact face structure;
//!   (4) the control — the same collar/peg WITH a planar `Rest` beside
//!       the cylindrical ones.
//!
//! **ε posture.** Every row runs at `Tol::witness()`, and the arm they
//! exercise turns on a DEFINITE `Out` from the cylinder chart's trim:
//! a wider band turns that verdict into the door's no-verdict
//! remainder and the old frontier stands, a narrower one certifies it
//! in more configurations. So the adversarial row for this change is
//! the TIGHTEST band, where the widened path runs most often — not the
//! loosest, where it runs least. The margins these fixtures put on the
//! trim are angular separations of a third of a turn at r = 0.5, four
//! to five orders above every band in the matrix, so the rows
//! themselves are band-insensitive; what the tight row exercises is
//! the rest of the tree taking the widened path.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use topo::{
    Body, BooleanDeclarations, BooleanResult, ContactClass, FacePairDeclaration, mass_properties,
};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// A circle at the origin as three 120° arcs, first joint at `deg0`.
fn three_arc(radius: f64, deg0: f64) -> ProfileLoop<f64> {
    let b120 = (core::f64::consts::PI / 6.0).tan();
    let at = |deg: f64| {
        let th: f64 = deg.to_radians();
        p2(radius * th.cos(), radius * th.sin())
    };
    ProfileLoop::new(vec![
        ProfileVertex::new(at(deg0), b120),
        ProfileVertex::new(at(deg0 + 120.0), b120),
        ProfileVertex::new(at(deg0 + 240.0), b120),
    ])
}

fn extruded(loops: Vec<ProfileLoop<f64>>, z0: f64, h: f64) -> Body<f64> {
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, loops).validate(Tol::witness()).unwrap();
    sweep::extrude(&profile, sweep::Extrusion::Distance(h), Tol::witness())
        .unwrap()
        .body
}

/// The collar: an annulus (outer r = 1.5, bore r = 0.5), z ∈ [1, 2],
/// both rims three 120° arcs — so the bore wall is 3 faces.
fn collar() -> Body<f64> {
    extruded(vec![three_arc(1.5, 0.0), three_arc(0.5, 0.0)], 1.0, 1.0)
}

/// The peg: a three-arc cylinder of radius 0.5, z ∈ [z0, z0 + h].
fn peg(z0: f64, h: f64) -> Body<f64> {
    extruded(vec![three_arc(0.5, 0.0)], z0, h)
}

/// The cylinder faces of `body` at radius ≈ `r`.
fn walls_at(body: &Body<f64>, r: f64) -> Vec<topo::FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Cylinder { radius, .. }) if (radius - r).abs() < 1e-9
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

/// Every (bore wall × peg wall) pair declared `Rest` — the mate's only
/// contact unless a caller adds one.
fn wall_decls(a: &Body<f64>, b: &Body<f64>) -> BooleanDeclarations {
    let mut decls = BooleanDeclarations::none();
    for &fa in &walls_at(a, 0.5) {
        for &fb in &walls_at(b, 0.5) {
            decls
                .coincident_faces
                .push(FacePairDeclaration::new(fa, fb, ContactClass::Rest));
        }
    }
    decls
}

fn volume(b: &Body<f64>) -> f64 {
    mass_properties(b, Tol::witness()).unwrap().volume
}

/// Exact additivity, to the arithmetic these volumes are computed in:
/// the operand interiors are disjoint, so the union's volume is their
/// sum. The comparison is relative rather than bitwise because both
/// sides carry irrational (π) terms that no rearrangement cancels —
/// fixture (i)'s bitwise oracle exists only because its peg and bore
/// π-terms cancel against an integer.
fn assert_additive(v: f64, vp: f64, vq: f64) {
    let sum = vp + vq;
    assert!(
        (v - sum).abs() <= 8.0 * f64::EPSILON * sum.abs(),
        "exactly additive: {v} vs {vp} + {vq} = {sum}"
    );
}

fn body_of(r: BooleanResult<f64>) -> Body<f64> {
    boolean_body(r).body
}

fn boolean_body(r: BooleanResult<f64>) -> topo::BooleanBody<f64> {
    match r {
        BooleanResult::Body(b) => b,
        BooleanResult::Empty => panic!("a threaded mate cannot be empty"),
    }
}

/// Spelling (3)/(1): the shaft-in-a-bore mate at fixture (i)'s face
/// structure, PARTIAL engagement, cylindrical `Rest` the only contact.
#[test]
fn threaded_collar_partial_engagement_unions() {
    let c = collar();
    let p = peg(0.5, 2.0);
    let decls = wall_decls(&c, &p);
    assert_eq!(decls.coincident_faces.len(), 9, "3 bore faces against 3");
    let out = topo::union_with(&c, &p, &decls, Tol::witness());
    println!("partial engagement: {:?}", out.as_ref().err());
    let bb = boolean_body(out.expect("the cylindrical Rest reaches the rest lane"));
    let body = bb.body;
    // Additive: the interiors are disjoint (the peg fills the bore
    // over z ∈ [1,2] and stands proud of it at both ends).
    assert_additive(volume(&body), volume(&c), volume(&p));
    if let Err(errs) = topo::validate_geometric(&body, Tol::witness()) {
        panic!("the threaded mate must be tier-3 valid: {errs:?}");
    }
    if let Err(errs) = topo::validate_pseudomanifold(&body, &bb.contacts, Tol::witness()) {
        panic!("the threaded mate must be pseudomanifold-clean: {errs:?}");
    }
    // ONE shell, genus 0: the bore's handle is closed by the peg.
    assert_eq!(body.shells().count(), 1, "one shell");
    // The mating patches are gone over the engaged span, and what
    // survives on the shared carrier is exactly the peg's proud ends.
    assert!(
        !walls_at(&body, 0.5).is_empty(),
        "the peg stands proud of the bore, so its wall survives outside it"
    );
}

/// Spelling (2): the same pair at FULL engagement — the peg exactly
/// spanning the collar, its caps flush with the collar's annuli.
#[test]
fn threaded_collar_full_engagement_unions() {
    let c = collar();
    let p = peg(1.0, 1.0);
    let decls = wall_decls(&c, &p);
    let out = topo::union_with(&c, &p, &decls, Tol::witness());
    println!("full engagement: {:?}", out.as_ref().err());
    let body = body_of(out.expect("full engagement reaches the rest lane"));
    assert_additive(volume(&body), volume(&c), volume(&p));
    if let Err(errs) = topo::validate_geometric(&body, Tol::witness()) {
        panic!("the fully engaged mate must be tier-3 valid: {errs:?}");
    }
    // Full engagement removes both mating wall patches: no face on the
    // shared r = 0.5 carrier survives. (The collar's own outer wall,
    // r = 1.5, is boundary and does survive — three faces of it.)
    assert!(
        walls_at(&body, 0.5).is_empty(),
        "full-engagement patch removal deletes every face on the mating carrier"
    );
    assert_eq!(walls_at(&body, 1.5).len(), 3, "the outer wall is boundary");
}

/// Spelling (4)'s SHAPE with a planar `Rest` beside the cylindrical
/// ones — a seated collar whose shaft still stands proud at the top.
///
/// The issue's own control is fixture (i)
/// (`m9_3_zip::two_peg_plate_union_is_exactly_additive`), which is
/// pinned by staying green. This row is the sharper statement the
/// measurement affords: a planar `Rest` carries only the incidences at
/// its OWN plane, so the seat rescues the bottom rim and the proud top
/// rim refuses on its own account. It is therefore red on main too —
/// at the top rim, the mirror image of the bore row's refusal.
#[test]
fn seated_collar_with_a_planar_rest_unions() {
    // A seated peg, built the way fixture (i) builds its plate-and-peg
    // operand: a shaft EMBEDDED in a PLATE (an undeclared boss union on
    // the shipped transverse lane), so the collar rests on the plate's
    // top face at z = 1 while the shaft fills its bore. The plate is
    // square and oversized so the mate's ONLY curved coincidence is the
    // declared bore pair — a plate rim flush with the collar's outer
    // wall would be an undeclared cosurface touch and refuse on its
    // own account, which is a different question.
    let plate = ProfileLoop::polygon([p2(-2.0, -2.0), p2(2.0, -2.0), p2(2.0, 2.0), p2(-2.0, 2.0)]);
    let flange = extruded(vec![plate], 0.0, 1.0);
    let shaft = peg(0.6, 1.9);
    let seated = body_of(topo::union(&flange, &shaft, Tol::witness()).unwrap());
    let c = collar();
    let mut decls = wall_decls(&c, &seated);
    decls.coincident_faces.push(FacePairDeclaration::new(
        plane_face(&c, 1.0, false),
        plane_face(&seated, 1.0, true),
        ContactClass::Rest,
    ));
    let out = topo::union_with(&c, &seated, &decls, Tol::witness());
    println!("control (planar Rest present): {:?}", out.as_ref().err());
    let body = body_of(out.expect("the control mate unions"));
    assert_additive(volume(&body), volume(&c), volume(&seated));
}
