//! **Booleans glue on Zero** (D10, Booleans; INTENT stage 4 PR E, the
//! spec's §11 rows 15, 16 and 19), at the document door.
//!
//! - **Row 15**: two blocks of separately typed sizes, flush, unioned
//!   are one solid of the summed volume carrying
//!   one `SameOpposite` row the coincidence door leaves unproven.
//! - **Row 16**: the same blocks a sliver apart refuse in band.
//! - **Row 19**: row 15's result passes tier 3′ with its contacts as
//!   the census reads them.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, failure, run};
use crate::corpus::{body_of, failures};
use crate::fixture::insert;
use editor_core::{
    BooleanValue, Node, NodeErrorKind, ProfileDoc, Proof, RecipeNodeId, ValuePayload, coincide,
};
use geom_core::Tol;
use topo::{DecisionSite, Relation};

/// A block of `[0, 2]² × [0, 1]` and a plate of `[0.5, 1.5]² × [1 + gap,
/// 1.5 + gap]` above it, each size typed separately, unioned:
/// `(doc, block, plate, union)`.
fn plate_on_a_block(gap: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("glue-on-zero", Tol::witness());
    let (doc, base) = block(doc, (0.0, 2.0), (0.0, 2.0), 0.0, 1.0);
    let (doc, plate) = block(doc, (0.5, 1.5), (0.5, 1.5), 1.0 + gap, 0.5);
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
/// decided — unproven, since the two caps are two constructions.
#[test]
fn a_flush_rest_glues_and_records_one_unproven_row() {
    let (doc, _, _, union) = plate_on_a_block(0.0);
    let ev = run(&doc);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
    let body = body_of(&ev, union);
    assert_eq!(body.solids().count(), 1, "one solid");
    let volume = topo::mass_properties(body, Tol::witness())
        .expect("the union measures")
        .volume;
    assert!((volume - 4.5).abs() < 1e-12, "the summed volume: {volume}");
    let rows = &ev.value(union).expect("the union evaluated").coincidences;
    assert_eq!(rows.len(), 1, "one row for the one rest: {rows:#?}");
    assert_eq!(
        (rows[0].relation, rows[0].site),
        (Relation::SameOpposite, DecisionSite::PlaneLadder)
    );
    assert!(
        matches!(
            coincide::prove(
                &doc,
                &[(union, &rows[..])],
                Tol::witness(),
                &editor_core::CancelToken::new()
            )
            .expect("an uncanceled door")[0][0],
            Proof::Unproven {
                recourse: coincide::Recourse::OneConstruction,
                ..
            }
        ),
        "two constructions, no proof; the offset's form freezes at the door's budget \
         (declared-rest-rows-are-two-values-or-too-big-to-close), which is no value \
         computed apart from the variables, so the recourse is to build one construction"
    );
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
    let ValuePayload::Boolean(BooleanValue::Body { body, contacts, .. }) =
        &ev.value(union).expect("the union evaluated").payload
    else {
        panic!("the union is a body");
    };
    assert_eq!(
        topo::validate_pseudomanifold(body, contacts, Tol::witness()),
        Ok(()),
        "tier 3′"
    );
}
