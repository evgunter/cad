//! **The solid-pair separation door** (`topo::SolidSeparation`) and the
//! solid-membership map beside it (`topo::SolidOwners`), on real
//! geometry.
//!
//! `Separation` — the placement door — is exercised by the group
//! boolean's own rows. These are the rows for the sibling that answers
//! the question AFTER a graft, over one body's own solids, which is
//! what a product gather needs: `graft_disjoint_all_keyed` asserts
//! nothing about its operands, so somebody has to.
//!
//! Claims pinned here:
//! - two solids the boxes prove apart CERTIFY, and the same two moved
//!   onto each other do not;
//! - the denial is sufficient-not-necessary in the direction the module
//!   promises: solids that merely TOUCH are denied, and that is honest
//!   rather than a defect;
//! - the answer does not depend on argument order, and the pair it
//!   names is in arena order;
//! - a self-pair and an unknown key DENY rather than reading as
//!   vacuous certificates;
//! - `SolidOwners` places every face and vertex of a multi-solid body.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{describe_as_intersections, geometric_cube};
use geom_core::Tol;
use geom_core::{Affine3, Vec3};
use topo::{SolidOwners, SolidSeparation, SolidsMeet};

/// A body holding two unit cubes, the second translated by `dx` along
/// x. `dx = 10` is well clear; `dx = 1` makes them share a face;
/// `dx = 0` puts one exactly on the other.
fn two_cubes(dx: f64) -> topo::Body<f64> {
    let mut src = geometric_cube::<f64>().body;
    describe_as_intersections(&mut src);
    let mut dst = geometric_cube::<f64>().body;
    describe_as_intersections(&mut dst);
    let map = Affine3::translation(Vec3::new(dx, 0.0, 0.0));
    let placed = topo::transform_rigid(&src, &map, Tol::witness()).expect("a rigid map");
    topo::graft_disjoint(&mut dst, &placed, Tol::witness()).expect("a placed graft");
    dst
}

/// Three unit cubes in one body, spread well clear of one another —
/// for the key that a two-solid index cannot resolve.
fn three_cubes() -> topo::Body<f64> {
    let mut dst = two_cubes(10.0);
    let mut src = geometric_cube::<f64>().body;
    describe_as_intersections(&mut src);
    let map = Affine3::translation(Vec3::new(20.0, 0.0, 0.0));
    let placed = topo::transform_rigid(&src, &map, Tol::witness()).expect("a rigid map");
    topo::graft_disjoint(&mut dst, &placed, Tol::witness()).expect("a placed graft");
    dst
}

fn keys(body: &topo::Body<f64>) -> (topo::SolidKey, topo::SolidKey) {
    let mut it = body.solids().map(|(k, _)| k);
    let a = it.next().expect("a first solid");
    let b = it.next().expect("a second solid");
    (a, b)
}

#[test]
fn solids_the_boxes_prove_apart_certify() {
    let body = two_cubes(10.0);
    let (a, b) = keys(&body);
    let sep = SolidSeparation::of(&body, Tol::witness()).expect("boxes over a valid body");
    assert_eq!(sep.certify(a, b), Ok(()));
    // And the certificate does not depend on which way round it is
    // asked.
    assert_eq!(sep.certify(b, a), Ok(()));
}

#[test]
fn coincident_solids_are_denied() {
    // The product-gather bug in miniature: one solid exactly on top of
    // another, which every local battery passes because each solid is
    // individually perfect.
    let body = two_cubes(0.0);
    let (a, b) = keys(&body);
    let sep = SolidSeparation::of(&body, Tol::witness()).expect("boxes");
    assert_eq!(sep.certify(a, b), Err(SolidsMeet { a, b }));
    // Arena order, not argument order: asking backwards names the same
    // pair the same way round (D9 — a report must not depend on how the
    // caller enumerated).
    assert_eq!(sep.certify(b, a), Err(SolidsMeet { a, b }));
}

#[test]
fn touching_solids_are_denied_and_that_is_the_contract() {
    // Face-to-face: disjoint interiors. The boxes are padded and meet,
    // so the certificate is withheld. Sufficient, not necessary — the
    // module says so, and this is the row that would notice if someone
    // "fixed" it into a claim about overlap.
    let body = two_cubes(1.0);
    let (a, b) = keys(&body);
    let sep = SolidSeparation::of(&body, Tol::witness()).expect("boxes");
    assert_eq!(sep.certify(a, b), Err(SolidsMeet { a, b }));
}

#[test]
fn a_self_pair_and_an_unknown_key_deny() {
    let body = two_cubes(10.0);
    let (a, _) = keys(&body);
    let sep = SolidSeparation::of(&body, Tol::witness()).expect("boxes");
    // A solid is not disjoint from itself, and answering `Ok` here
    // would let a caller's self-pair read as a certificate.
    assert_eq!(sep.certify(a, a), Err(SolidsMeet { a, b: a }));

    // A key the index cannot RESOLVE denies. Reaching that arm needs a
    // key past the end of this body's arena — a foreign key from a
    // same-shaped body does NOT reach it, because the index is a
    // `SecondaryMap` keyed by slot and version and the sibling body's
    // first solid occupies the same slot as this one's. That is the
    // door's stated PRECONDITION rather than a defect, and this row
    // pins the distinction so nobody re-derives the reassuring version
    // (the first draft of this row asserted on `stranger` and did not
    // notice that `stranger == a`, which made it the self-pair
    // assertion written twice).
    let sibling = two_cubes(10.0);
    let (sibling_first, _) = keys(&sibling);
    assert_eq!(
        sibling_first, a,
        "a same-shaped body's first solid IS this body's first key, so a \
         foreign key is not detectable here"
    );

    // The genuinely unresolvable case: a body with MORE solids has a
    // key at a slot this two-solid index never filled.
    let bigger = three_cubes();
    let third = bigger
        .solids()
        .map(|(k, _)| k)
        .nth(2)
        .expect("a third solid");
    assert_eq!(sep.certify(a, third), Err(SolidsMeet { a, b: third }));
}

#[test]
fn keys_are_the_bodys_solids_in_arena_order() {
    let body = two_cubes(10.0);
    let sep = SolidSeparation::of(&body, Tol::witness()).expect("boxes");
    let listed: Vec<_> = sep.keys().collect();
    let arena: Vec<_> = body.solids().map(|(k, _)| k).collect();
    assert_eq!(listed, arena);
}

#[test]
fn solid_owners_places_every_face_and_vertex() {
    let body = two_cubes(10.0);
    let (a, b) = keys(&body);
    let owners = SolidOwners::of(&body);
    // Two cubes: 12 faces, 16 vertices, every one of them placed, and
    // split evenly between the two solids.
    let faces: Vec<_> = body
        .faces()
        .map(|(f, _)| owners.face(f).expect("every face is placed"))
        .collect();
    assert_eq!(faces.len(), 12);
    assert_eq!(faces.iter().filter(|&&s| s == a).count(), 6);
    assert_eq!(faces.iter().filter(|&&s| s == b).count(), 6);

    let vertices: Vec<_> = body
        .vertices()
        .map(|(v, _)| owners.vertex(v).expect("every vertex is placed"))
        .collect();
    assert_eq!(vertices.len(), 16);
    assert_eq!(vertices.iter().filter(|&&s| s == a).count(), 8);
    assert_eq!(vertices.iter().filter(|&&s| s == b).count(), 8);
}
