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

mod mate2_common;

use geom_core::Tol;
use mate2_common::*;
use profile::{ProfileLoop, RawLoop};
use topo::{ContactClass, FacePairDeclaration};

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
