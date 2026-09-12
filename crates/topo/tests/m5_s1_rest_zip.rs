//! M5 S1 acceptance — the declared-REST union zip fixtures beyond the
//! crosslap (`crosslap_rest.rs` holds the headline pins):
//!
//! - stacked plates (full-face REST contact): single pair and the
//!   three-plate chain (two declared contacts, sequential ops);
//! - corner-flush REST (the tier-3′ fixture's shape) — chords +
//!   pierce-ring consumption in the pierced face;
//! - the contradiction row: a false REST declaration refuses
//!   `DeclarationContradicted` at the lane, never a silent no-op;
//! - the annular (ringed-patch) contact UNIONS exactly additively
//!   (M9-3's ring-capable zip retired the old sub-frontier refusal);
//! - ∖/∩ disposition rows on the PINNED REST fixtures (crosslap,
//!   corner-flush): classification resolves them structurally
//!   (operand A / typed Empty) without reaching a join door. NOT a
//!   universal claim: a three-wall notch-fill REST ∖ refuses
//!   `Containment(RayExhausted)` — a pre-existing containment-probe
//!   exhaustion, unchanged by S1 (`review_s1_probes.rs` pins the
//!   counterexample);
//! - undeclared doors unchanged (the ladder is law);
//! - re-run bit-identity for the stacked union.
//!
//! Every volume assertion is EXACT dyadic f64 equality — the lane
//! discards nothing (interiors are disjoint), so vol(A∪B) must equal
//! vol(A)+vol(B) to the bit.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{flush_declarations, prism_z};
use geom_core::Decide;
use geom_core::Tol;
use topo::{
    Body, BooleanBody, BooleanError, BooleanResult, BooleanResultKind, mass_properties, subtract,
    subtract_with, union, union_with, validate_geometric, validate_pseudomanifold,
};

fn brick<T: Decide + geom_core::CertifiedBounds + topo::PropsQuadLane>(
    x: (f64, f64),
    y: (f64, f64),
    z: (f64, f64),
) -> Body<T> {
    prism_z::<T>(&[(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)], z.0, z.1).body
}

fn glue<T: Decide + geom_core::CertifiedBounds + topo::PropsQuadLane>(
    a: &Body<T>,
    b: &Body<T>,
) -> BooleanBody<T> {
    match union_with(a, b, &flush_declarations(a, b), Tol::witness())
        .expect("declared REST union builds")
    {
        BooleanResult::Body(body) => body,
        BooleanResult::Empty => panic!("REST union cannot be empty"),
    }
}

/// The glued result's shared gate ladder: seamed, tier 3, 3′ with the
/// surviving records, rest records consumed (3′ ≡ tier 3). Exact
/// volume equality is asserted by the f64 rows (the Interval lane has
/// no scalar equality by design — NaI ≠ NaI).
fn assert_glued<T: Decide + geom_core::CertifiedBounds + topo::PropsQuadLane>(g: &BooleanBody<T>) {
    assert_eq!(g.kind, BooleanResultKind::Seamed);
    assert_eq!(validate_geometric(&g.body, Tol::witness()), Ok(()));
    assert_eq!(
        validate_pseudomanifold(&g.body, &g.contacts, Tol::witness()),
        Ok(())
    );
    assert!(
        g.contacts.vv.is_empty() && g.contacts.a_on_b.is_empty() && g.contacts.b_on_a.is_empty(),
        "REST records are consumed into seam structure: {:?}",
        g.contacts
    );
}

/// Stacked plates, full-face REST contact: the declared union is ONE
/// body with the contact faces gone; the declared same-oriented side
/// planes merge in the output stage, leaving the plain brick's six
/// faces. Undeclared, the coincidence door refuses unchanged.
fn stacked_plates_scenario<T: Decide + geom_core::CertifiedBounds + topo::PropsQuadLane>()
-> BooleanBody<T> {
    let bot = brick::<T>((0.0, 2.0), (0.0, 2.0), (0.0, 1.0));
    let top = brick::<T>((0.0, 2.0), (0.0, 2.0), (1.0, 2.0));
    let err = union(&bot, &top, Tol::witness()).unwrap_err();
    assert!(
        matches!(err, BooleanError::UndeclaredCoincidence { .. }),
        "undeclared stacked union must refuse at the coincidence door: {err:?}"
    );
    let g = glue(&bot, &top);
    assert_glued(&g);
    assert_eq!(
        g.body.faces().count(),
        6,
        "contact faces removed, coplanar declared sides merged: a brick"
    );
    assert_eq!(g.body.shells().count(), 1, "one closed shell");
    g
}

#[test]
fn stacked_plates_full_face_rest() {
    let g = stacked_plates_scenario::<f64>();
    assert_eq!(
        mass_properties(&g.body, Tol::witness()).unwrap().volume,
        8.0,
        "exact dyadic volume additivity"
    );
}

/// The three-plate chain: two declared REST contacts consumed by two
/// sequential ops (the second op's declarations name the FIRST glued
/// result's faces — reuse of a REST result as an operand).
#[test]
fn three_plate_chain() {
    let p1 = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 1.0));
    let p2 = brick::<f64>((0.0, 2.0), (0.0, 2.0), (1.0, 2.0));
    let p3 = brick::<f64>((0.0, 2.0), (0.0, 2.0), (2.0, 3.0));
    let g12 = glue(&p1, &p2);
    assert_glued(&g12);
    assert_eq!(
        mass_properties(&g12.body, Tol::witness()).unwrap().volume,
        8.0
    );
    let g123 = glue(&g12.body, &p3);
    assert_glued(&g123);
    assert_eq!(
        mass_properties(&g123.body, Tol::witness()).unwrap().volume,
        12.0
    );
    assert_eq!(g123.body.faces().count(), 6, "still a plain brick");
}

/// Corner-flush REST (the tier-3′ fixture's shape): the contact square
/// shares two edges with the slab's rim and pierces the top face with
/// one interior corner (a pierce-ring vertex the seam chords consume).
/// Declared, the union BUILDS; undeclared, the coincidence door
/// refuses; ∖ stays operand A (no join door was ever reached).
fn corner_flush_scenario<T: Decide + geom_core::CertifiedBounds + topo::PropsQuadLane>()
-> (BooleanBody<T>, BooleanBody<T>) {
    let slab = brick::<T>((0.0, 4.0), (0.0, 4.0), (0.0, 1.0));
    let corner = brick::<T>((0.0, 1.0), (0.0, 1.0), (1.0, 3.0));
    let err = union(&slab, &corner, Tol::witness()).unwrap_err();
    assert!(
        matches!(err, BooleanError::UndeclaredCoincidence { .. }),
        "undeclared corner-flush union must refuse at the coincidence door: {err:?}"
    );
    let g = glue(&slab, &corner);
    assert_glued(&g);
    // ∖ disposition: pure REST subtract classifies structurally — the
    // whole of A survives (never a join refusal; SPEC §1 note).
    let decls = flush_declarations(&slab, &corner);
    match subtract_with(&slab, &corner, &decls, Tol::witness()).unwrap() {
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
        mass_properties(&g.body, Tol::witness()).unwrap().volume,
        18.0
    );
    assert_eq!(
        mass_properties(&sub.body, Tol::witness()).unwrap().volume,
        16.0
    );
}

/// The contradiction row: a FALSE REST declaration (two definitely
/// distinct planes declared coincident) refuses
/// `DeclarationContradicted` at the lane — the lie is met where the
/// lane consumes the declaration set, never a silent no-op. The pair
/// is chosen so classification itself never meets it (the faces are
/// far apart), isolating the lane door.
#[test]
fn false_rest_declaration_contradicts_at_the_lane() {
    let bot = prism_z::<f64>(&[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)], 0.0, 1.0);
    let top = prism_z::<f64>(&[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)], 1.0, 2.0);
    let mut decls = flush_declarations(&bot.body, &top.body);
    // The lie: bot's bottom cap (z = 0, outward −z) declared
    // coincident with top's top cap (z = 2, outward +z) — opposite
    // orientation, definitely-distinct planes.
    decls.coincident_faces.push(topo::FacePairDeclaration::rest(
        bot.bottom_face,
        top.top_face,
    ));
    let err = union_with(&bot.body, &top.body, &decls, Tol::witness()).unwrap_err();
    assert!(
        matches!(err, BooleanError::ContactContradicted { .. }),
        "false REST declaration must contradict, naming the class and the margin: {err:?}"
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
    let tube = |z0: f64, z1: f64| -> Body<f64> {
        let outer = brick::<f64>((0.0, 3.0), (0.0, 3.0), (z0, z1));
        let hole = brick::<f64>((1.0, 2.0), (1.0, 2.0), (z0 - 1.0, z1 + 1.0));
        match subtract(&outer, &hole, Tol::witness()).expect("tube subtract") {
            BooleanResult::Body(b) => b.body,
            BooleanResult::Empty => panic!("tube is not empty"),
        }
    };
    let bot = tube(0.0, 1.0);
    let top = tube(1.0, 2.0);
    let out = union_with(&bot, &top, &flush_declarations(&bot, &top), Tol::witness())
        .expect("the annular REST union runs");
    let BooleanResult::Body(b) = out else {
        panic!("a stacked-tube union cannot be empty");
    };
    let v = topo::mass_properties(&b.body, Tol::witness())
        .unwrap()
        .volume;
    assert_eq!(v, 16.0, "exactly additive: two 8-volume tube rings");
    if let Err(errs) = topo::validate_geometric(&b.body, Tol::witness()) {
        panic!("the stacked tube must be tier-3 valid: {errs:?}");
    }
}

/// Crosslap ∖/∩ disposition rows (SPEC §1): on THIS fixture (and the
/// corner-flush one above) classification resolves ∖/∩ structurally —
/// no join door is reached, so there is no refusing door to re-text
/// here. The claim is per-fixture, not universal: the three-wall
/// notch-fill REST ∖ refuses `Containment(RayExhausted)` (pre-existing
/// probe exhaustion, unchanged by S1) — `review_s1_probes.rs` pins
/// that counterexample.
#[test]
fn rest_subtract_and_intersect_resolve_structurally() {
    let beam_a = prism_z::<f64>(
        &[(0.0, 1.75), (4.0, 1.75), (4.0, 2.25), (0.0, 2.25)],
        0.0,
        0.5,
    );
    let cut_a = prism_z::<f64>(
        &[(1.75, 1.5), (2.25, 1.5), (2.25, 2.5), (1.75, 2.5)],
        0.25,
        0.75,
    );
    let BooleanResult::Body(a) = subtract(&beam_a.body, &cut_a.body, Tol::witness()).unwrap()
    else {
        panic!("notch A yields a body");
    };
    let beam_b = prism_z::<f64>(
        &[(1.75, 0.0), (2.25, 0.0), (2.25, 4.0), (1.75, 4.0)],
        0.0,
        0.5,
    );
    let cut_b = prism_z::<f64>(
        &[(1.5, 1.75), (2.5, 1.75), (2.5, 2.25), (1.5, 2.25)],
        -0.25,
        0.25,
    );
    let BooleanResult::Body(b) = subtract(&beam_b.body, &cut_b.body, Tol::witness()).unwrap()
    else {
        panic!("notch B yields a body");
    };
    let decls = flush_declarations(&a.body, &b.body);
    match subtract_with(&a.body, &b.body, &decls, Tol::witness()).unwrap() {
        BooleanResult::Body(sub) => {
            assert_eq!(sub.kind, BooleanResultKind::OperandA);
            assert_eq!(
                mass_properties(&sub.body, Tol::witness()).unwrap().volume,
                0.9375
            );
        }
        BooleanResult::Empty => panic!("A ∖ B keeps A's material"),
    }
    assert!(
        matches!(
            topo::intersect_with(&a.body, &b.body, &decls, Tol::witness()).unwrap(),
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
        let bot = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 1.0));
        let top = brick::<f64>((0.0, 2.0), (0.0, 2.0), (1.0, 2.0));
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
// T = Interval, exercised under `--features interval`). ----
#[cfg(feature = "interval")]
mod interval {
    use super::*;

    #[test]
    fn rest_zip_interval() {
        let _ = stacked_plates_scenario::<geom_core::Interval>();
        let _ = corner_flush_scenario::<geom_core::Interval>();
    }
}
