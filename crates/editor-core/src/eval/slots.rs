//! Slot evaluation: every expression a node carries, evaluated once
//! per node in the node's deterministic slot order — the single place
//! expression failures acquire their (node, slot) context (spec D2;
//! PR 1's banked `NonFiniteResult` obligation) and the single source
//! of values for BOTH the content key and the op wiring (they must
//! never disagree).

use geom_core::Decide;

use crate::expr::{EvalError, ParamEnv, eval, eval_count};
use crate::node::{Node, SlotId};

/// An evaluated slot: continuous scalar or exact count.
#[derive(Debug, Clone, Copy)]
pub(crate) enum SlotVal<T> {
    /// A continuous value in kernel units.
    Scalar(T),
    /// An exact structural count.
    Count(i64),
}

/// All of a node's evaluated slots, in `Node::slots()` order.
pub(crate) type SlotValues<T> = Vec<(SlotId, SlotVal<T>)>;

/// Evaluates every slot; the first failure returns with its slot.
pub(crate) fn eval_slots<T: Decide, P>(
    node: &Node<P>,
    env: &ParamEnv<T>,
) -> Result<SlotValues<T>, (SlotId, EvalError)> {
    let mut out = Vec::new();
    for slot in node.slots() {
        let Some(expr) = node.expr(slot) else {
            // Unreachable: slots() is the domain of expr().
            continue;
        };
        let val = if slot.is_structural() {
            SlotVal::Count(eval_count(expr, env).map_err(|e| (slot, e))?)
        } else {
            SlotVal::Scalar(eval(expr, env).map_err(|e| (slot, e))?)
        };
        out.push((slot, val));
    }
    Ok(out)
}

/// The scalar in a named slot (wiring helper; `None` if absent or a
/// count — callers state the slot they mean, so a miss is a wiring
/// bug surfaced as a typed operand error upstream).
pub(crate) fn scalar<T: Copy>(values: &SlotValues<T>, slot: SlotId) -> Option<T> {
    values.iter().find_map(|(s, v)| match v {
        SlotVal::Scalar(x) if *s == slot => Some(*x),
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

/// A named `[Expr; 3]` slot triple as a vector (wiring helper).
pub(crate) fn vec3<T: geom_core::Real>(
    values: &SlotValues<T>,
    f: fn(crate::node::Axis3) -> SlotId,
) -> Option<geom_core::Vec3<T>> {
    Some(geom_core::Vec3::new(
        scalar(values, f(crate::node::Axis3::X))?,
        scalar(values, f(crate::node::Axis3::Y))?,
        scalar(values, f(crate::node::Axis3::Z))?,
    ))
}
