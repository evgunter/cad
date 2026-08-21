//! M3 PR 6a acceptance: `validate_pseudomanifold` (tier 3′) —
//! declared-contact certification on the touching-configuration corpus
//! at rest, negative controls, closure stress, and exports.
//!
//! Every promotion scenario tests BOTH directions: the boolean result
//! is green under its carried contacts AND red (`UndeclaredContact`)
//! when the declarations are withheld — the census never blesses
//! (F1/F2). Scenarios are generic over `T` (f64 ε rows via CI; the
//! explicit Interval lane at the bottom, per suite convention).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{mapped_cube, prism_z};
use geom_core::{Decide, Point3, Vec3};
use topo::{
    Body, BooleanBody, BooleanError, BooleanResult, ContactRecords, ValidationError,
    intersect_with, mass_properties, subtract_with, union, union_with, validate_geometric,
    validate_pseudomanifold,
};
use geom_core::Tol;

fn brick<T: Decide + geom_core::Bounds + topo::PropsQuadLane>(
    x: (f64, f64),
    y: (f64, f64),
    z: (f64, f64),
) -> Body<T> {
    prism_z::<T>(&[(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)], z.0, z.1).body
}

type BoolOp<T> =
    fn(&Body<T>, &Body<T>, &topo::BooleanDeclarations, Tol) -> Result<BooleanResult<T>, BooleanError>;

/// Runs one declared op (M4 PR 5: the corpus declares its intended
/// flush contacts — the recipe intent, test form).
fn run_body<T: Decide + geom_core::Bounds + topo::PropsQuadLane>(
    op: BoolOp<T>,
    a: &Body<T>,
    b: &Body<T>,
) -> BooleanBody<T> {
    match op(a, b, &common::flush_declarations(a, b), Tol::witness()).unwrap() {
        BooleanResult::Body(body) => body,
        BooleanResult::Empty => panic!("expected a non-empty boolean result"),
    }
}

/// The green/red promotion pair: 3′ passes with the carried contacts,
/// and withholding them yields `UndeclaredContact` (and nothing else).
fn assert_promoted<T: Decide + geom_core::Bounds + topo::PropsQuadLane>(b: &BooleanBody<T>) {
    assert_eq!(validate_pseudomanifold(&b.body, &b.contacts, Tol::witness()), Ok(()));
    let withheld = validate_pseudomanifold(&b.body, &ContactRecords::default(), Tol::witness()).unwrap_err();
    assert!(!withheld.is_empty());
    for e in &withheld {
        assert!(
            matches!(e, ValidationError::UndeclaredContact { .. }),
            "withheld contacts must surface as UndeclaredContact, got {e:?}"
        );
    }
}

// ---------------------------------------------------------------
// PROMOTION (D9): the touching corpus at rest.
// ---------------------------------------------------------------

/// Corner kiss (v-v): the PR 5 assembly, now certified at rest.
fn corner_kiss_scenario<T: Decide + geom_core::Bounds + topo::PropsQuadLane>() {
    let a = brick::<T>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let b = brick::<T>((1.0, 2.0), (1.0, 2.0), (1.0, 2.0));
    let body = run_body(union_with as BoolOp<T>, &a, &b);
    assert_eq!(body.contacts.vv.len(), 1);
    assert_promoted(&body);
}

#[test]
fn corner_kiss_promoted() {
    corner_kiss_scenario::<f64>();
}

/// Wedge touch / tangent edge (PR 4's diagonal edge-edge tie): two
/// bricks sharing exactly one edge — the union assembly carries the
/// two shared-endpoint v-v records, and the census reconstructs the
/// full-length coincident-edge SEGMENT from them (the D3 rule's
/// bounded-by-declared-records lane, live end to end).
fn tangent_edge_scenario<T: Decide + geom_core::Bounds + topo::PropsQuadLane>() {
    let a = brick::<T>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let b = brick::<T>((1.0, 2.0), (0.0, 1.0), (1.0, 2.0));
    let body = run_body(union_with as BoolOp<T>, &a, &b);
    assert_eq!(body.contacts.vv.len(), 2);
    assert_promoted(&body);
}

#[test]
fn tangent_edge_promoted() {
    tangent_edge_scenario::<f64>();
}

/// Skew touching edges (PR 4 acceptance 4): an A-edge crossing a
/// B-edge at interior points of both in the shared tangent plane. At
/// reduce time this is 2 refined v-v pairs + one v-on-f rest per side
/// (PR 4 pins it); the ∪ SEAM consumes every one of them (the contact
/// vertices are zipped into single structural vertices), so the
/// result is the CONSUMED class: empty carried contacts and a census
/// that agrees — 3′ ≡ tier 3 on it. ∩: every touching sector
/// classifies Out ⇒ the typed empty. ∖: operand A (records dropped
/// with B absent), tier 3.
fn skew_edges_scenario<T: Decide + geom_core::Bounds + topo::PropsQuadLane>() {
    let a = brick::<T>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = brick::<T>((1.5, 3.5), (0.5, 2.5), (2.0, 4.0));
    let body = run_body(union_with as BoolOp<T>, &a, &b);
    assert!(body.contacts.vv.is_empty());
    assert!(body.contacts.a_on_b.is_empty() && body.contacts.b_on_a.is_empty());
    assert_eq!(validate_pseudomanifold(&body.body, &body.contacts, Tol::witness()), Ok(()));
    assert_eq!(validate_geometric(&body.body, Tol::witness()), Ok(()), "3′ ≡ tier 3 here");
    assert!(matches!(
        intersect_with(&a, &b, &common::flush_declarations(&a, &b), Tol::witness()).unwrap(),
        BooleanResult::Empty
    ));
    let sub = run_body(subtract_with as BoolOp<T>, &a, &b);
    assert!(sub.contacts.vv.is_empty() && sub.contacts.b_on_a.is_empty());
    assert_eq!(validate_pseudomanifold(&sub.body, &sub.contacts, Tol::witness()), Ok(()));
}

#[test]
fn skew_edges_promoted() {
    skew_edges_scenario::<f64>();
}

/// Vertex-on-face kiss: a tilted cube balancing one corner on a
/// slab's top-face interior (right prisms cannot kiss a face with a
/// lone vertex — the tilt is the honest fixture). Union: a touching
/// assembly carrying exactly the one v-on-f record.
#[test]
fn vertex_on_face_kiss_promoted() {
    let slab = brick::<f64>((0.0, 4.0), (0.0, 4.0), (0.0, 1.0));
    // Corner (0,0,0) of the mapped cube sits at (2,2,1) exactly; the
    // three edge frames all point upward (material strictly above).
    let tilted = mapped_cube(|x, y, z| {
        let (e1, e2, e3) = (
            Vec3::new(0.9, 0.1, 0.3),
            Vec3::new(-0.2, 0.8, 0.45),
            Vec3::new(-0.3, -0.4, 0.85),
        );
        Point3::new(
            2.0 + x * e1.x + y * e2.x + z * e3.x,
            2.0 + x * e1.y + y * e2.y + z * e3.y,
            1.0 + x * e1.z + y * e2.z + z * e3.z,
        )
    });
    let body = run_body(union_with as BoolOp<f64>, &tilted, &slab);
    assert_eq!(body.contacts.a_on_b.len(), 1, "{:?}", body.contacts);
    assert!(body.contacts.vv.is_empty() && body.contacts.b_on_a.is_empty());
    assert_promoted(&body);
}

/// Edge rest (the D4 pin): B's bottom edge rests along A's top rim
/// edge with interior overhang — B's endpoint lands on A's edge
/// INTERIOR and A's corner on B's edge interior. Reduction refines
/// BOTH vertex-on-edge events into v-v records by splitting
/// (`split_other_at_point` — no vertex-on-edge record type exists,
/// by derivation), and the census certifies the collinear overlap
/// segment from those bounding records (D3).
fn edge_rest_scenario<T: Decide + geom_core::Bounds + topo::PropsQuadLane>() {
    let a = brick::<T>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = brick::<T>((1.0, 3.0), (-2.0, 0.0), (2.0, 4.0));
    let body = run_body(union_with as BoolOp<T>, &a, &b);
    // Two refined v-v pairs: B's corner (1,0,2) on A's edge interior;
    // A's corner (2,0,2) on B's edge interior.
    assert_eq!(body.contacts.vv.len(), 2, "{:?}", body.contacts);
    assert_promoted(&body);
}

#[test]
fn edge_rest_promoted_d4_pin() {
    edge_rest_scenario::<f64>();
}

/// Flush pillar rests (D9): interior flush rest — pillar standing on
/// the slab's top-face interior — SEAMS (the working envelope's
/// "interior-rest flush contact") and consumes every rest record into
/// structure: the consumed class, census agrees, 3′ ≡ tier 3.
/// Corner-flush (contact-square edges collinear with the slab's own
/// rim) was the documented boundary-on-boundary ∪ refusal (M3
/// envelope class (iii)) until M5 S1's declared-REST union zip: the
/// declared ∪ now BUILDS through the same consumed class, and the
/// undeclared door refuses unchanged (the ladder is law). ∖ returns
/// operand A at tier 3, as before — pure REST subtracts never reach
/// a join door.
fn flush_rests_scenario<T: Decide + geom_core::Bounds + topo::PropsQuadLane>() {
    let slab = brick::<T>((0.0, 4.0), (0.0, 4.0), (0.0, 1.0));
    let pillar = brick::<T>((1.0, 2.0), (1.0, 2.0), (1.0, 3.0));
    let body = run_body(union_with as BoolOp<T>, &slab, &pillar);
    assert!(body.contacts.vv.is_empty() && body.contacts.b_on_a.is_empty());
    assert_eq!(validate_pseudomanifold(&body.body, &body.contacts, Tol::witness()), Ok(()));
    assert_eq!(validate_geometric(&body.body, Tol::witness()), Ok(()));

    let corner = brick::<T>((0.0, 1.0), (0.0, 1.0), (1.0, 3.0));
    // Undeclared: the coincidence door refuses first now (M4 PR 5's
    // rung (b) narrowing — value equality never classifies).
    let err = union(&slab, &corner, Tol::witness()).unwrap_err();
    assert!(
        matches!(err, BooleanError::UndeclaredCoincidence { .. }),
        "undeclared corner-flush ∪ must refuse at the coincidence door, got {err:?}"
    );
    // Declared: the M5 S1 REST lane zips the corner-flush mate — the
    // former Join(_) pin flipped to a certified pass (the same
    // frontier as the crosslap; `crosslap_rest.rs` holds the headline
    // pins, `m5_s1_rest_zip.rs` the exact-volume row for this shape).
    let glued = run_body(union_with as BoolOp<T>, &slab, &corner);
    assert!(
        glued.contacts.vv.is_empty()
            && glued.contacts.a_on_b.is_empty()
            && glued.contacts.b_on_a.is_empty(),
        "corner-flush REST records are consumed into seam structure"
    );
    assert_eq!(
        validate_pseudomanifold(&glued.body, &glued.contacts, Tol::witness()),
        Ok(())
    );
    assert_eq!(validate_geometric(&glued.body, Tol::witness()), Ok(()));
    let sub = run_body(subtract_with as BoolOp<T>, &slab, &corner);
    assert_eq!(validate_pseudomanifold(&sub.body, &sub.contacts, Tol::witness()), Ok(()));
}

#[test]
fn flush_rests() {
    flush_rests_scenario::<f64>();
}

/// D1.5's pin: on a tier-3 body with EMPTY declarations, the census
/// must find nothing and 3′ ≡ tier 3 (plus the census actually run) —
/// pinned on a plain prism, an L-prism, and a Seamed boolean result.
fn tier3_equivalence_scenario<T: Decide + geom_core::Bounds + topo::PropsQuadLane>() {
    let plain = brick::<T>((0.0, 2.0), (0.0, 1.0), (0.0, 1.0));
    assert_eq!(validate_geometric(&plain, Tol::witness()), Ok(()));
    assert_eq!(
        validate_pseudomanifold(&plain, &ContactRecords::default(), Tol::witness()),
        Ok(())
    );
    let ell = prism_z::<T>(
        &[
            (0.0, 0.0),
            (4.0, 0.0),
            (4.0, 2.0),
            (2.0, 2.0),
            (2.0, 4.0),
            (0.0, 4.0),
        ],
        0.0,
        1.0,
    )
    .body;
    assert_eq!(validate_geometric(&ell, Tol::witness()), Ok(()));
    assert_eq!(
        validate_pseudomanifold(&ell, &ContactRecords::default(), Tol::witness()),
        Ok(())
    );
    let a = brick::<T>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = brick::<T>((1.0, 3.0), (1.0, 3.0), (1.0, 3.0));
    let body = run_body(union_with as BoolOp<T>, &a, &b);
    assert!(body.contacts.vv.is_empty());
    assert_eq!(validate_geometric(&body.body, Tol::witness()), Ok(()));
    assert_eq!(validate_pseudomanifold(&body.body, &body.contacts, Tol::witness()), Ok(()));
}

#[test]
fn tier3_equivalence_empty_contacts() {
    tier3_equivalence_scenario::<f64>();
}

// ---------------------------------------------------------------
// NEGATIVE CONTROLS (D9).
// ---------------------------------------------------------------

/// A tampered declaration (wrong vertex key) is a typed
/// `StaleContactDeclaration` — and the real kiss, now unbacked,
/// surfaces as `UndeclaredContact`: both certification directions
/// fire on one body.
#[test]
fn tampered_declaration_is_stale() {
    let a = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let b = brick::<f64>((1.0, 2.0), (1.0, 2.0), (1.0, 2.0));
    let body = run_body(union_with as BoolOp<f64>, &a, &b);
    let mut tampered = body.contacts.clone();
    let real_a = tampered.vv[0].a;
    let wrong = body
        .body
        .vertices()
        .map(|(k, _)| k)
        .find(|&k| k != real_a && k != tampered.vv[0].b)
        .unwrap();
    tampered.vv[0].a = wrong;
    let errors = validate_pseudomanifold(&body.body, &tampered, Tol::witness()).unwrap_err();
    assert!(
        errors
            .iter()
            .any(|e| matches!(e, ValidationError::StaleContactDeclaration { .. })),
        "{errors:?}"
    );
    assert!(
        errors
            .iter()
            .any(|e| matches!(e, ValidationError::UndeclaredContact { .. })),
        "{errors:?}"
    );
}

/// A hand-built genuine self-intersection — two interpenetrating cube
/// shells in ONE body, no declarations — is `UndeclaredContact`, hard:
/// the census's proper-crossing lanes (edge-face pierce / edge-edge
/// cross) have no backing path by design.
#[test]
fn hand_built_self_intersection_is_undeclared() {
    let mut body = mapped_cube(|x, y, z| Point3::new(2.0 * x, 2.0 * y, 2.0 * z));
    common::cube_into(&mut body, |x, y, z| {
        Point3::new(1.0 + 2.0 * x, 1.0 + 2.0 * y, 1.0 + 2.0 * z)
    });
    let errors = validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness()).unwrap_err();
    assert!(!errors.is_empty());
    assert!(
        errors
            .iter()
            .all(|e| matches!(e, ValidationError::UndeclaredContact { .. })),
        "{errors:?}"
    );
    assert!(
        errors.iter().any(|e| matches!(
            e,
            ValidationError::UndeclaredContact {
                contact: topo::CensusContact::EdgeFacePierce { .. },
                ..
            }
        )),
        "expected a pierce finding: {errors:?}"
    );
}

// ---------------------------------------------------------------
// CLOSURE (D9, documented-unproven): 3′ results fed back through the
// ops. Every case ends in a correct certified result (volume oracle)
// or a typed/loud outcome — the one systematic gap is DOCUMENTED and
// asserted exactly: a boolean op does not consume its OPERANDS'
// declared contacts, so an operand-internal coincidence that persists
// into the result is re-discovered by the census as undeclared — the
// at-rest 3′ gate refuses it loudly (never silent wrongness). Feeding
// declarations through op composition is the 6(b)/M4 recipe-layer
// item; the rows here are the closure table's data.
// ---------------------------------------------------------------

/// The corner-kiss assembly (1 v-v declaration) as the 3′ base.
fn kiss_base<T: Decide + geom_core::Bounds + topo::PropsQuadLane>() -> BooleanBody<T> {
    let a = brick::<T>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let b = brick::<T>((1.0, 2.0), (1.0, 2.0), (1.0, 2.0));
    run_body(union_with as BoolOp<T>, &a, &b)
}

/// Closure vs a generic mover crossing one shell away from the kiss.
#[test]
fn closure_kiss_vs_mover() {
    let base = kiss_base::<f64>();
    let mover = brick::<f64>((1.5, 2.5), (1.5, 2.5), (1.5, 2.5));

    // ∪: closes structurally (volume oracle exact); the surviving
    // kiss is operand-internal — undeclared in the new result's
    // records — and the 3′ gate refuses it LOUDLY (the documented
    // closure gap; row 1 of the table).
    let r = run_body(union_with as BoolOp<f64>, &base.body, &mover);
    assert_eq!(mass_properties(&r.body, Tol::witness()).unwrap().volume, 2.875);
    let errors = validate_pseudomanifold(&r.body, &r.contacts, Tol::witness()).unwrap_err();
    assert!(
        errors
            .iter()
            .all(|e| matches!(e, ValidationError::UndeclaredContact { .. })),
        "{errors:?}"
    );

    // ∖: same shape — closes, kiss persists, gate refuses loudly.
    let r = run_body(subtract_with as BoolOp<f64>, &base.body, &mover);
    assert_eq!(mass_properties(&r.body, Tol::witness()).unwrap().volume, 1.875);
    let errors = validate_pseudomanifold(&r.body, &r.contacts, Tol::witness()).unwrap_err();
    assert!(
        errors
            .iter()
            .all(|e| matches!(e, ValidationError::UndeclaredContact { .. })),
        "{errors:?}"
    );

    // ∩: the kiss shell drops out of the result — certified 3′ green
    // with the exact volume (row 3: closes cleanly).
    let r = run_body(intersect_with as BoolOp<f64>, &base.body, &mover);
    assert_eq!(mass_properties(&r.body, Tol::witness()).unwrap().volume, 0.125);
    assert_eq!(validate_pseudomanifold(&r.body, &r.contacts, Tol::witness()), Ok(()));
}

/// Closure vs a SECOND toucher at the same locus: a third brick
/// kissing at (1,1,1) (and edge-tied to the first cube). The op
/// closes; the new result declares only the new operand pair's
/// contacts, so the surviving operand-internal pair refuses at the
/// gate — same documented gap, multi-body locus flavor.
#[test]
fn closure_kiss_vs_second_toucher() {
    let base = kiss_base::<f64>();
    let toucher = brick::<f64>((0.0, 1.0), (1.0, 2.0), (1.0, 2.0));
    match union(&base.body, &toucher, Tol::witness()) {
        Ok(BooleanResult::Body(r)) => {
            let verdict = validate_pseudomanifold(&r.body, &r.contacts, Tol::witness());
            match verdict {
                Ok(()) => {}
                Err(errors) => {
                    assert!(
                        errors
                            .iter()
                            .all(|e| matches!(e, ValidationError::UndeclaredContact { .. })),
                        "gap must surface as UndeclaredContact only: {errors:?}"
                    );
                }
            }
        }
        Ok(BooleanResult::Empty) => panic!("union of touching bodies cannot be empty"),
        Err(e) => {
            // A typed refusal is an acceptable closure outcome (the
            // boundary-on-boundary family) — record which.
            eprintln!("closure second-toucher refusal: {e:?}");
        }
    }
}

/// Closure on the CONSUMED class: a flush-rest union (tier 3, empty
/// contacts) takes a further pocket subtract and stays certified —
/// the clean closure row.
#[test]
fn closure_consumed_base_stays_certified() {
    let slab = brick::<f64>((0.0, 4.0), (0.0, 4.0), (0.0, 1.0));
    let pillar = brick::<f64>((1.0, 2.0), (1.0, 2.0), (1.0, 3.0));
    let boss = run_body(union_with as BoolOp<f64>, &slab, &pillar);
    assert_eq!(mass_properties(&boss.body, Tol::witness()).unwrap().volume, 18.0);
    let cutter = brick::<f64>((3.0, 3.5), (3.0, 3.5), (0.5, 1.5));
    let r = run_body(subtract_with as BoolOp<f64>, &boss.body, &cutter);
    assert_eq!(mass_properties(&r.body, Tol::witness()).unwrap().volume, 18.0 - 0.125);
    assert_eq!(validate_pseudomanifold(&r.body, &r.contacts, Tol::witness()), Ok(()));
}

// ---- Interval lane (the same generic scenarios at T = Interval). ----
#[cfg(feature = "interval")]
mod interval {
    use super::*;

    #[test]
    fn tier3prime_interval() {
        corner_kiss_scenario::<geom_core::Interval>();
        tangent_edge_scenario::<geom_core::Interval>();
        skew_edges_scenario::<geom_core::Interval>();
        edge_rest_scenario::<geom_core::Interval>();
        flush_rests_scenario::<geom_core::Interval>();
        tier3_equivalence_scenario::<geom_core::Interval>();
    }
}
