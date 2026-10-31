//! **Axis-aligned blocks and the unions over them** — the evaluation,
//! refusal and census helpers the boolean suites share.
#![allow(clippy::expect_used, clippy::panic)]

use crate::fixture::{insert, len, slid};
use editor_core::{
    BooleanValue, CancelToken, EvalOptions, Evaluation, Node, NodeErrorKind, NodeResult,
    ProfileDoc, RecipeNodeId, ValuePayload, evaluate,
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

/// A block's x span, y span, base z and height, as [`block_of`] and
/// [`block_on_of`] take them.
pub(crate) type Spans = ((f64, f64), (f64, f64), f64, f64);

/// [`block`] over `spans`: the root a later block stands on, so its
/// base is its own xy.
pub(crate) fn block_of(doc: ProfileDoc, (x, y, z0, dz): Spans) -> (ProfileDoc, RecipeNodeId) {
    assert_eq!(z0, 0.0, "a root block's base is its own xy");
    block(doc, x, y, dz)
}

/// [`block_on`] over `spans`, against `target`.
pub(crate) fn block_on_of(
    doc: ProfileDoc,
    target: RecipeNodeId,
    (x, y, z0, dz): Spans,
) -> (ProfileDoc, RecipeNodeId) {
    block_on(doc, target, x, y, z0, dz)
}

/// An axis-aligned block on its own xy plane, extruded `dz`: a
/// construction, the root of its own space.
pub(crate) fn block(
    doc: ProfileDoc,
    (x0, x1): (f64, f64),
    (y0, y1): (f64, f64),
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = insert(
        doc,
        Node::Profile(crate::fixture::desc(vec![vec![
            (x0, y0),
            (x1, y0),
            (x1, y1),
            (x0, y1),
        ]])),
    );
    insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(dz),
        },
    )
}

/// **[`block`] at height `z0` in `target`'s space**: the block placed
/// against `target`'s geometry, slid `z0` up its own `z`
/// ([`crate::fixture::slid`]). Returns the placement, whose output is
/// the block's copy, as the operand a combine with `target` reads.
pub(crate) fn block_on(
    doc: ProfileDoc,
    target: RecipeNodeId,
    xs: (f64, f64),
    ys: (f64, f64),
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, b) = block(doc, xs, ys, dz);
    slid(doc, target, b, [0.0, 0.0, z0])
}

/// The tier-3′ verdict on a boolean node's value, over its own records.
pub(crate) fn census_of(
    ev: &Evaluation<f64>,
    id: RecipeNodeId,
) -> Result<(), Vec<topo::ValidationError>> {
    let value = ev.value(id).expect("the node evaluated");
    match &value.payload {
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => {
            topo::validate_pseudomanifold(body.body(), &value.carry.contacts, Tol::witness())
                .map(|_| ())
        }
        other => panic!("expected a boolean body, got {other:?}"),
    }
}
