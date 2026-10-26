//! REST unions beyond the crosslap (`crosslap_rest.rs` holds
//! the headline pins), built by the join:
//!
//! - stacked plates (full-face REST contact): single pair and the
//!   three-plate chain (two contacts, sequential ops);
//! - corner-flush REST (the tier-3′ fixture's shape) — chords +
//!   pierce-ring consumption in the pierced face;
//! - the annular (ringed-patch) contact UNIONS exactly additively;
//! - ∖/∩ disposition rows on the PINNED REST fixtures (crosslap,
//!   corner-flush): classification resolves them structurally
//!   (operand A / typed Empty) without reaching a join door (the
//!   three-wall notch-fill REST ∖ reaches the containment fallback
//!   instead, `review_s1_probes.rs`);
//! - re-run bit-identity for the stacked union.
//!
//! Every volume assertion is EXACT dyadic f64 equality — the
//! interiors are disjoint, so vol(A∪B) must equal vol(A)+vol(B) to the
//! bit.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{brick, finished};
use geom_core::Decide;
use geom_core::Tol;
use topo::{
    AtRestBody, BooleanBody, BooleanResult, BooleanResultKind, mass_properties, subtract, union,
    validate_geometric, validate_pseudomanifold,
};

/// The brick `x × y × z`, finished.
fn finished_brick<T: Decide + topo::AtRestPolicy>(
    x: (f64, f64),
    y: (f64, f64),
    z: (f64, f64),
) -> AtRestBody<T> {
    let tol = Tol::witness();
    finished("brick", brick::<T>(x, y, z, tol), tol)
}

fn glue<T: Decide + geom_core::CertifiedBounds + topo::AtRestPolicy>(
    a: &AtRestBody<T>,
    b: &AtRestBody<T>,
) -> BooleanBody<T> {
    match union(a, b, Tol::witness()).expect("the REST union builds") {
        BooleanResult::Body(body) => body,
        BooleanResult::Empty => panic!("REST union cannot be empty"),
    }
}

/// The glued result's shared gate ladder: seamed, carrying the door's
/// tier-3 verdict (both scalars here certify), tier 3, 3′ with the
/// surviving records, rest records consumed (3′ ≡ tier 3). Exact
/// volume equality is asserted by the f64 rows (the Interval lane has
/// no scalar equality by design — NaI ≠ NaI).
fn assert_glued<T: Decide + geom_core::CertifiedBounds + topo::AtRestPolicy>(g: &BooleanBody<T>) {
    assert_eq!(g.kind, BooleanResultKind::Seamed);
    assert_eq!(
        g.body.outcome(),
        topo::AtRestOutcome::Validated,
        "the door gates its result at tier 3"
    );
    assert_eq!(validate_geometric(&g.body, Tol::witness()), Ok(()));
    assert_eq!(
        validate_pseudomanifold(&g.body, &g.contacts, Tol::witness()),
        Ok(())
    );
    assert!(
        g.contacts.vertex_vertex().next().is_none() && g.contacts.vertex_face().next().is_none(),
        "REST records are consumed into seam structure: {:?}",
        g.contacts
    );
}

/// Stacked plates, full-face REST contact: the union is ONE body with
/// the contact faces gone; the glued same-oriented side planes merge in
/// the output stage, leaving the plain brick's six faces.
fn stacked_plates_scenario<T: Decide + geom_core::CertifiedBounds + topo::AtRestPolicy>()
-> BooleanBody<T> {
    let bot = finished_brick::<T>((0.0, 2.0), (0.0, 2.0), (0.0, 1.0));
    let top = finished_brick::<T>((0.0, 2.0), (0.0, 2.0), (1.0, 2.0));
    let g = glue(&bot, &top);
    assert_glued(&g);
    assert_eq!(
        g.body.faces().count(),
        6,
        "contact faces removed, coplanar sides merged: a brick"
    );
    assert_eq!(g.body.shells().count(), 1, "one closed shell");
    g
}

#[test]
fn stacked_plates_full_face_rest() {
    let g = stacked_plates_scenario::<f64>();
    assert_eq!(
        mass_properties(
            &g.body,
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness())
        )
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume,
        8.0,
        "exact dyadic volume additivity"
    );
}

/// The three-plate chain: two REST contacts consumed by two sequential
/// ops (the second op glues the FIRST glued result's faces — reuse of a
/// REST result as an operand).
#[test]
fn three_plate_chain() {
    let p1 = finished_brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 1.0));
    let p2 = finished_brick::<f64>((0.0, 2.0), (0.0, 2.0), (1.0, 2.0));
    let p3 = finished_brick::<f64>((0.0, 2.0), (0.0, 2.0), (2.0, 3.0));
    let g12 = glue(&p1, &p2);
    assert_glued(&g12);
    assert_eq!(
        mass_properties(
            &g12.body,
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness())
        )
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume,
        8.0
    );
    let g123 = glue(&g12.body, &p3);
    assert_glued(&g123);
    assert_eq!(
        mass_properties(
            &g123.body,
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness())
        )
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume,
        12.0
    );
    assert_eq!(g123.body.faces().count(), 6, "still a plain brick");
}

/// Corner-flush REST (the tier-3′ fixture's shape): the contact square
/// shares two edges with the slab's rim and pierces the top face with
/// one interior corner (a pierce-ring vertex the seam chords consume).
/// The union BUILDS, one body; ∖ stays operand A (no
/// join door was ever reached).
fn corner_flush_scenario<T: Decide + geom_core::CertifiedBounds + topo::AtRestPolicy>()
-> (BooleanBody<T>, BooleanBody<T>) {
    let slab = finished_brick::<T>((0.0, 4.0), (0.0, 4.0), (0.0, 1.0));
    let corner = finished_brick::<T>((0.0, 1.0), (0.0, 1.0), (1.0, 3.0));
    let g = glue(&slab, &corner);
    assert_glued(&g);
    // ∖ disposition: pure REST subtract classifies structurally — the
    // whole of A survives (never a join refusal; SPEC §1 note).
    match subtract(&slab, &corner, Tol::witness()).unwrap() {
        BooleanResult::Body(sub) => {
            assert_eq!(sub.kind, BooleanResultKind::OperandA);
            (g, sub)
        }
        BooleanResult::Empty => panic!("slab ∖ corner is not empty"),
    }
}

#[test]
fn corner_flush_rest_union_builds() {
    let (g, sub) = corner_flush_scenario::<f64>();
    assert_eq!(
        mass_properties(
            &g.body,
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness())
        )
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume,
        18.0
    );
    assert_eq!(
        mass_properties(
            &sub.body,
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness())
        )
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume,
        16.0
    );
}

/// An ANNULAR contact (two square tubes stacked — the contact patch
/// is a ring-carrying face) UNIONS: the glue promotes each interior
/// boundary to its own congruent cycle pair (M9-3's ring-capable
/// zip), the patch pair is removed as interior, and the volume is
/// exactly additive. This row held the lane's old ring sub-frontier
/// refusal; the generalized zip retired it.
#[test]
fn annular_rest_contact_unions_exactly_additively() {
    let tube = |z0: f64, z1: f64| -> AtRestBody<f64> {
        let outer = finished_brick::<f64>((0.0, 3.0), (0.0, 3.0), (z0, z1));
        let hole = finished_brick::<f64>((1.0, 2.0), (1.0, 2.0), (z0 - 1.0, z1 + 1.0));
        match subtract(&outer, &hole, Tol::witness()).expect("tube subtract") {
            BooleanResult::Body(b) => b.body,
            BooleanResult::Empty => panic!("tube is not empty"),
        }
    };
    let bot = tube(0.0, 1.0);
    let top = tube(1.0, 2.0);
    let out = union(&bot, &top, Tol::witness()).expect("the annular REST union runs");
    let BooleanResult::Body(b) = out else {
        panic!("a stacked-tube union cannot be empty");
    };
    let v = topo::test_support::pinned_volume("the measured body", &b.body, Tol::witness()).volume;
    assert_eq!(v, 16.0, "exactly additive: two 8-volume tube rings");
    if let Err(errs) = topo::validate_geometric(&b.body, Tol::witness()) {
        panic!("the stacked tube must be tier-3 valid: {errs:?}");
    }
}

/// Crosslap ∖/∩ disposition rows (SPEC §1): on THIS fixture (and the
/// corner-flush one above) classification resolves ∖/∩ structurally —
/// no join door is reached, so there is no refusing door to re-text
/// here. The claim is per-fixture: the three-wall notch-fill REST ∖
/// reaches the containment fallback instead (`review_s1_probes.rs`).
#[test]
fn rest_subtract_and_intersect_resolve_structurally() {
    let beam_a = finished_brick::<f64>((0.0, 4.0), (1.75, 2.25), (0.0, 0.5));
    let cut_a = finished_brick::<f64>((1.75, 2.25), (1.5, 2.5), (0.25, 0.75));
    let BooleanResult::Body(a) = subtract(&beam_a, &cut_a, Tol::witness()).unwrap() else {
        panic!("notch A yields a body");
    };
    let beam_b = finished_brick::<f64>((1.75, 2.25), (0.0, 4.0), (0.0, 0.5));
    let cut_b = finished_brick::<f64>((1.5, 2.5), (1.75, 2.25), (-0.25, 0.25));
    let BooleanResult::Body(b) = subtract(&beam_b, &cut_b, Tol::witness()).unwrap() else {
        panic!("notch B yields a body");
    };
    match subtract(&a.body, &b.body, Tol::witness()).unwrap() {
        BooleanResult::Body(sub) => {
            assert_eq!(sub.kind, BooleanResultKind::OperandA);
            assert_eq!(
                mass_properties(
                    &sub.body,
                    Tol::witness(),
                    topo::test_support::pinned_resolution(Tol::witness())
                )
                .unwrap()
                .met()
                .expect("the pinned resolution is met")
                .volume,
                0.9375
            );
        }
        BooleanResult::Empty => panic!("A ∖ B keeps A's material"),
    }
    assert!(
        matches!(
            topo::intersect(&a.body, &b.body, Tol::witness()).unwrap(),
            BooleanResult::Empty
        ),
        "disjoint interiors intersect to the typed Empty"
    );
}

/// Stacked-plates re-run bit-identity (naming-key stability beyond
/// the crosslap's own row): naming emission and STL bytes equal
/// across independent runs.
#[test]
fn stacked_rerun_is_bit_identical() {
    let run = || {
        let bot = finished_brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 1.0));
        let top = finished_brick::<f64>((0.0, 2.0), (0.0, 2.0), (1.0, 2.0));
        glue(&bot, &top)
    };
    let (g1, g2) = (run(), run());
    assert_eq!(g1.naming.seam_edges, g2.naming.seam_edges);
    assert_eq!(g1.naming.vertex_merges, g2.naming.vertex_merges);
    assert_eq!(g1.naming.merge_groups, g2.naming.merge_groups);
    let stl_of = |g: &BooleanBody<f64>| {
        let mesh = mesh::tessellate(&g.body, 1e-2, Tol::witness()).expect("tessellate");
        mesh::validate::check_mesh(&mesh).expect("watertight");
        let mut buf = Vec::new();
        stl::write_binary(&mesh, &stl::BinaryOptions::default(), &mut buf).expect("stl");
        buf
    };
    assert_eq!(stl_of(&g1), stl_of(&g2));
}

// ---- Interval lane (Q1 pure replay: the same scenarios at
// T = Interval). ----
mod interval {
    use super::*;

    #[test]
    fn rest_zip_interval() {
        let _ = stacked_plates_scenario::<geom_core::Interval>();
        let _ = corner_flush_scenario::<geom_core::Interval>();
    }
}
