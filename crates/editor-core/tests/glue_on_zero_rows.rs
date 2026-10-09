//! **Booleans glue on Zero** (D10, Booleans; INTENT stage 4 PR E, the
//! spec's §11 rows 15, 16, 17 and 19), at the document door.
//!
//! - **Row 15**: two blocks of separately typed sizes, flush, unioned
//!   with no declaration, are one solid of the summed volume carrying
//!   one `SameOpposite` row the coincidence door leaves unproven.
//! - **Row 16**: the same blocks a sliver apart refuse in band.
//! - **Row 17**: every declared corpus scene builds the same body with
//!   its declarations removed.
//! - **Row 19**: row 15's result passes tier 3′ with its contacts as
//!   the census reads them.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus::{self, body_of, failures};
use crate::docm7_union_declare::{block, failure, run};
use crate::fixture::insert;
use editor_core::{
    BooleanOp, BooleanValue, DocEdit, Node, NodeErrorKind, ProfileDoc, Proof, RecipeNodeId,
    ValuePayload, coincide,
};
use geom_core::Tol;
use topo::{DecisionSite, Relation};

/// A block of `[0, 2]² × [0, 1]` and a plate of `[0.5, 1.5]² × [1 + gap,
/// 1.5 + gap]` above it, each size typed separately, unioned with no
/// declaration: `(doc, block, plate, union)`.
fn plate_on_a_block(gap: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("glue-on-zero", Tol::witness());
    let (doc, base) = block(doc, (0.0, 2.0), (0.0, 2.0), 0.0, 1.0);
    let (doc, plate) = block(doc, (0.5, 1.5), (0.5, 1.5), 1.0 + gap, 0.5);
    let (doc, union) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: base.into(),
            b: plate.into(),
            declare: Vec::new(),
        },
    );
    (doc, base, plate, union)
}

/// **Row 15: an undeclared flush rest glues and records.** One solid,
/// the summed volume, and one `SameOpposite` row the plane ladder
/// decided — unproven, since the two caps are two constructions.
#[test]
fn an_undeclared_flush_rest_glues_and_records_one_unproven_row() {
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
        matches!(coincide::prove(&doc, &rows[0]), Proof::Unproven { .. }),
        "two constructions, no proof"
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

/// **Row 17: declared and undeclared are one body.** For every corpus
/// document, every Boolean and Union that declares contacts builds the
/// same body, bit for bit, with its declarations removed.
#[test]
fn every_declared_corpus_scene_builds_the_same_body_undeclared() {
    let mut scenes = 0;
    for corpus_doc in corpus::documents() {
        let doc = &corpus_doc.doc;
        let declaring: Vec<RecipeNodeId> = doc
            .ids()
            .into_iter()
            .filter(|&id| match doc.node(id) {
                Some(Node::Boolean { declare, .. } | Node::Union { declare, .. }) => {
                    !declare.is_empty()
                }
                _ => false,
            })
            .collect();
        if declaring.is_empty() {
            continue;
        }
        let mut bare = doc.clone();
        for &node in &declaring {
            bare = editor_core::apply(
                &bare,
                &DocEdit::SetDeclare {
                    node,
                    pairs: Vec::new(),
                },
                Tol::witness(),
                &editor_core::RefusingReach,
            )
            .expect("clearing a declaration applies")
            .doc;
        }
        let (declared, undeclared) = (run(doc), run(&bare));
        assert!(failures(&declared).is_empty(), "{:?}", failures(&declared));
        assert!(
            failures(&undeclared).is_empty(),
            "{}: {:?}",
            corpus_doc.name,
            failures(&undeclared)
        );
        for &node in &declaring {
            scenes += 1;
            assert_eq!(
                format!("{:?}", body_of(&declared, node)),
                format!("{:?}", body_of(&undeclared, node)),
                "{}: node {node:?}",
                corpus_doc.name
            );
        }
    }
    assert!(scenes > 0, "the corpus declares a contact somewhere");
}
