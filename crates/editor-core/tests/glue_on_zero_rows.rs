//! **Booleans glue on Zero** (D10, Booleans; INTENT stage 4 PR E, the
//! spec's §11 rows 15, 16 and 19), at the document door.
//!
//! - **Row 15**: two blocks of separately typed sizes, flush, unioned
//!   are one solid of the summed volume carrying
//!   one `SameOpposite` row the coincidence door leaves unproven, and a
//!   row per plate corner decided on the block's top.
//! - **Row 16**: the same blocks a sliver apart refuse in band.
//! - **Row 19**: row 15's result passes tier 3′ with its contacts as
//!   the census reads them.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, block_on, failure, run};
use crate::corpus::{body_of, failures};
use crate::fixture::insert;
use editor_core::{
    BooleanValue, Node, NodeErrorKind, ProfileDoc, Proof, RecipeNodeId, ValuePayload, coincide,
};
use geom_core::Tol;
use topo::{DecisionSite, Relation};

/// A block of `[0, 2]² × [0, 1]` and a plate of `[0.5, 1.5]² × [1 + gap,
/// 1.5 + gap]` placed above it, each size typed separately, unioned:
/// `(doc, block, plate, union)`, `plate` the placement.
fn plate_on_a_block(gap: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("glue-on-zero", Tol::witness());
    let (doc, base) = block(doc, (0.0, 2.0), (0.0, 2.0), 1.0);
    let (doc, plate) = block_on(doc, base, (0.5, 1.5), (0.5, 1.5), 1.0 + gap, 0.5);
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![base.into(), plate.into()]),
        },
    );
    (doc, base, plate, union)
}

/// **Row 15: a flush rest glues and records.** One solid,
/// the summed volume, and one `SameOpposite` row the plane ladder
/// decided — unproven, since the two caps are two constructions, the
/// plate a copy placed on the block by a slide off the block's top (a
/// standoff), which states no seat
/// — beside the plate's four corners, each decided on the block's top.
#[test]
fn a_flush_rest_glues_and_records_its_unproven_rows() {
    let (doc, _, _, union) = plate_on_a_block(0.0);
    let ev = run(&doc);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
    let body = topo::test_support::finished(
        "body",
        topo::Body::clone(&(body_of(&ev, union))),
        Tol::witness(),
    );
    assert_eq!(body.solids().count(), 1, "one solid");
    let volume =
        topo::test_support::pinned_volume("the union measures", &body, Tol::witness()).volume;
    assert!((volume - 4.5).abs() < 1e-12, "the summed volume: {volume}");
    let rows = &ev.value(union).expect("the union evaluated").coincidences;
    assert_eq!(rows.len(), 5, "the rest and its four corners: {rows:#?}");
    assert_eq!(
        (rows[0].relation, rows[0].site),
        (Relation::SameOpposite, DecisionSite::PlaneLadder)
    );
    assert!(
        rows[1..]
            .iter()
            .all(|r| (r.relation, r.site) == (Relation::OnCarrier, DecisionSite::VertexFusion)),
        "the corners: {rows:#?}"
    );
    let proofs = coincide::prove(
        &doc,
        &[(union, &rows[..])],
        None,
        Tol::witness(),
        &editor_core::CancelToken::new(),
    )
    .expect("an uncanceled door")
    .remove(0);
    // Every row, the rest and each corner alike, is two constructions
    // the replay finds no identity between.
    for (row, proof) in rows.iter().zip(&proofs) {
        assert!(
            matches!(
                proof,
                Proof::Unproven {
                    residual,
                    recourse: coincide::Recourse::Budget,
                } if residual.replay == coincide::Unreplayed::Numeric
            ),
            "{row:?}: two constructions, no proof; the offset's form freezes at the door's \
             budget (declared-rest-rows-are-two-values-or-too-big-to-close), which is no \
             value computed apart from the variables, so the recourse names the budget: \
             {proof:?}"
        );
    }
    assert_eq!(proofs.len(), rows.len(), "the door answers every row");
}

/// **Row 16: a sliver still refuses.** The same plate `2ε` above the
/// block: the offset lies in band, which neither glues nor parts.
#[test]
fn the_same_blocks_a_sliver_apart_refuse_in_band() {
    let eps = Tol::witness().get().eps;
    let (doc, _, _, union) = plate_on_a_block(2.0 * eps);
    let ev = run(&doc);
    let Some(NodeErrorKind::Boolean(e)) = failure(&ev, union) else {
        panic!("the sliver refuses: {:?}", failure(&ev, union));
    };
    assert!(
        matches!(e, topo::BooleanError::Escalated { .. }),
        "an in-band refusal: {e:?}"
    );
}

/// **Row 19: the census backs Zero glue.** Row 15's result passes
/// tier 3′ against the contacts it ships.
#[test]
fn a_zero_glued_union_passes_tier_three_prime() {
    let (doc, _, _, union) = plate_on_a_block(0.0);
    let ev = run(&doc);
    let value = ev.value(union).expect("the union evaluated");
    let ValuePayload::Boolean(BooleanValue::Body { body, .. }) = &value.payload else {
        panic!("the union is a body");
    };
    let contacts = &value.carry.contacts;
    assert_eq!(
        topo::validate_pseudomanifold(body.body(), contacts, Tol::witness()),
        Ok(()),
        "tier 3′"
    );
}
