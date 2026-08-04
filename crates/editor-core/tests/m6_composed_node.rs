//! **M6 unit 1 at the recipe layer** — the one blocker keeping the
//! composed die's document (`corpus/die_composed.rs`) out of the
//! corpus registry, pinned and executed.
//!
//! `Node::Fillet` is every-edge BY DESIGN (no stable edge names to go
//! stale under a bump). A pipped cube's edge set includes the cap's
//! MERIDIAN seam edges — two half-cap faces on ONE sphere surface —
//! and a co-surface seam has no dihedral wedge, so the battery
//! honestly refuses `TangentialEdge` at zero margin, at any radius.
//! The composed die therefore needs an edge-SELECTION vocabulary the
//! recipe layer does not have (the N4 fillet-naming emitter, banked
//! in `eval/wire.rs::wire_fillet`'s docs). This row is what keeps
//! that blocker from going stale: the day `Node::Fillet` can name a
//! subset, this refusal pin flips and `die_composed` joins
//! `documents()`.
//!
//! The surgery itself is fully exercised elsewhere — the 21-pip
//! composed die's ladder in `sweep/tests/m6_surgery.rs`, the interval
//! lane in `m6_surgery_interval.rs`, and the demo tour's
//! `diecomposed` stop.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use corpus::die_composed;
use editor_core::{CancelToken, EvalOptions, NodeErrorKind, NodeResult, evaluate};

/// **The blocker, executed**: evaluating the composed-die recipe
/// refuses TYPED at the fillet node — the every-edge request meets
/// the cap meridians, which have no wedge — and the refusal is the
/// battery's `TangentialEdge` with a zero margin, not a crash and
/// not a silent skip.
#[test]
fn every_edge_fillet_on_a_pipped_body_refuses_at_the_cap_meridians() {
    let doc = die_composed::document();
    let ev = evaluate::<f64>(&doc.doc, None, &CancelToken::new(), &EvalOptions::default());
    let fillet_node = doc.result.expect("the document names its result");
    let err = match ev.nodes.get(&fillet_node) {
        Some(NodeResult::Failed(e)) => e,
        other => panic!(
            "the every-edge fillet must refuse typed at the meridians, got {other:?}"
        ),
    };
    match &err.kind {
        NodeErrorKind::Fillet(sweep::fillet::FilletError::TangentialEdge { margin, .. }) => {
            assert_eq!(*margin, 0.0, "a co-surface seam has exactly no wedge");
        }
        other => panic!("expected the battery's TangentialEdge refusal, got {other:?}"),
    }
}
