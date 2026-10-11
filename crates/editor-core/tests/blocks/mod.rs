//! **Axis-aligned blocks and the unions over them** — the evaluation,
//! refusal and census helpers the boolean suites share.
#![allow(clippy::expect_used, clippy::panic)]

use crate::fixture::{insert, len, on_frame};
use editor_core::{
    BooleanValue, CancelToken, EvalOptions, Evaluation, ExtrudeSide, Node, NodeErrorKind,
    NodeResult, ProfileDoc, RecipeNodeId, ValuePayload, evaluate,
};
use geom_core::Tol;

/// Evaluates, and holds every table the run produced to the N3
/// flatness rule on the way out. A tripwire over the merged rows these
/// suites reach, not the guard: the mint refuses a nested constituent
/// before a table is published, and the rows that carry the rule are
/// `docm8_flat_merged`'s (the corpus walk and the mint-site rows).
pub(crate) fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    let ev = evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    crate::fixture::assert_no_nested_merged(&ev);
    ev
}

/// The refusal `id` evaluated to, if it refused.
pub(crate) fn failure(ev: &Evaluation<f64>, id: RecipeNodeId) -> Option<&NodeErrorKind> {
    match ev.nodes.get(&id) {
        Some(NodeResult::Failed(e)) => Some(&e.kind),
        _ => None,
    }
}

/// An axis-aligned block on the xy plane at height `z0`, extruded `dz`.
pub(crate) fn block(
    doc: ProfileDoc,
    (x0, x1): (f64, f64),
    (y0, y1): (f64, f64),
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, z0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(dz),
            side: ExtrudeSide::Along,
        },
    )
}

/// The tier-3′ verdict on a boolean node's value, over its own records.
pub(crate) fn census_of(
    ev: &Evaluation<f64>,
    id: RecipeNodeId,
) -> Result<(), Vec<topo::ValidationError>> {
    match &ev.value(id).expect("the node evaluated").payload {
        ValuePayload::Boolean(BooleanValue::Body { body, contacts, .. }) => {
            topo::validate_pseudomanifold(body, contacts, Tol::witness()).map(|_| ())
        }
        other => panic!("expected a boolean body, got {other:?}"),
    }
}
