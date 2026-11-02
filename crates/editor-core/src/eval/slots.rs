//! Slot evaluation: every variable a node's slots read, looked up in the
//! node's deterministic slot order through ONE door — the single place
//! expression failures acquire their (node, slot) context (spec D2)
//! and the single source of values for BOTH the content key and the op
//! wiring (they must never disagree).
//!
//! The door is the one SPELLING of the read, not a count of reads.
//! Its askers:
//! - `eval_node`, twice per node: once at the evaluation scalar, whose
//!   values the op runs on and the key's lane half holds, and once at
//!   the document's nominal, whose values the key's other half holds
//!   (`super::tag::slot`);
//! - a pose read of a revolve's axis output (`super::wire::pose`),
//!   which reads the revolve's slots at the lane.
//!
//! A slot's variable is read in exactly one loop
//! ([`eval_rows`]), and a refusal at any of them arrives in one shape.

use geom_core::Decide;

use crate::expr::{EvalError, VarEnv, eval_var, eval_var_count};
use crate::node::{Node, SlotId};
use crate::var::VarId;

/// An evaluated slot: continuous scalar or exact count.
#[derive(Debug, Clone, Copy)]
pub(crate) enum SlotVal<T> {
    /// A continuous value in kernel units, and the fraction of a turn
    /// an angle's definition is, where it is one ([`VarEnv::turns`]).
    Scalar(T, Option<crate::expr::Ratio>),
    /// An exact structural count.
    Count(i64),
}

/// All of a node's evaluated slots, in `Node::slots()` order.
pub(crate) type SlotValues<T> = Vec<(SlotId, SlotVal<T>)>;

/// Evaluates every slot; the first failure returns with its slot.
///
/// Profile nodes are EXEMPT (empty result): their program slots
/// resolve at f64 in `eval_node`'s dedicated stage (LIB-SWITCH §4b —
/// the C6 f64 pin), never at the lane scalar, and their key
/// contribution is the resolved program stream, not slot values
/// (`super::tag::slot` carries what that means for the nominal half).
pub(crate) fn eval_slots<T: Decide, P: crate::ProfilePayload>(
    node: &Node<P>,
    env: &VarEnv<T>,
) -> Result<SlotValues<T>, (SlotId, EvalError)> {
    if matches!(node, Node::Profile(_)) {
        return Ok(Vec::new());
    }
    eval_rows(node.rows(), env)
}

/// **The one loop that evaluates slot rows**, in the order given: a
/// structural slot as an exact count, every other as a scalar. The
/// first failure returns with its slot.
pub(crate) fn eval_rows<'e, T: Decide>(
    rows: impl IntoIterator<Item = (SlotId, &'e VarId)>,
    env: &VarEnv<T>,
) -> Result<SlotValues<T>, (SlotId, EvalError)> {
    rows.into_iter()
        .map(|(slot, &var)| {
            let val = if slot.is_structural() {
                SlotVal::Count(eval_var_count(var, env).map_err(|e| (slot, e))?)
            } else {
                let dim = slot.expr_dimension();
                let value = eval_var(var, dim, env).map_err(|e| (slot, e))?;
                SlotVal::Scalar(value, env.turns.get(&var).copied())
            };
            Ok((slot, val))
        })
        .collect()
}

/// The scalar in a named slot (wiring helper; `None` if absent or a
/// count). Callers state the slot they mean, so a miss is a wiring bug:
/// `wire`'s `declared` panics on it, and the D10-held readers still
/// refuse it as `NodeErrorKind::MissingSlot`.
pub(crate) fn scalar<T: Copy>(values: &SlotValues<T>, slot: SlotId) -> Option<T> {
    values.iter().find_map(|(s, v)| match v {
        SlotVal::Scalar(x, _) if *s == slot => Some(*x),
        _ => None,
    })
}

/// The angle in a named slot, with the form it was written in
/// ([`crate::expr::eval_angle`]'s, for a slot).
pub(crate) fn angle<T: geom_core::Real>(
    values: &SlotValues<T>,
    slot: SlotId,
) -> Option<geom_core::Angle<T>> {
    values.iter().find_map(|(s, v)| match v {
        SlotVal::Scalar(x, turns) if *s == slot => Some(crate::expr::angle_of(*x, *turns)),
        _ => None,
    })
}

/// The count in a named slot.
pub(crate) fn count<T>(values: &SlotValues<T>, slot: SlotId) -> Option<i64> {
    values.iter().find_map(|(s, v)| match v {
        SlotVal::Count(n) if *s == slot => Some(*n),
        _ => None,
    })
}
