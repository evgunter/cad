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
///
/// Profile nodes are EXEMPT (empty result): their program slots
/// resolve at f64 in `eval_node`'s dedicated stage (LIB-SWITCH §4b —
/// the C6 f64 pin), never at the lane scalar, and their key
/// contribution is the resolved program stream, not slot values.
pub(crate) fn eval_slots<T: Decide, P: crate::ProfilePayload>(
    node: &Node<P>,
    env: &ParamEnv<T>,
) -> Result<SlotValues<T>, (SlotId, EvalError)> {
    if matches!(node, Node::Profile(_)) {
        return Ok(Vec::new());
    }
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

/// A named `[Expr; 2]` slot pair as a sketch-plane vector — the X and
/// Y components only, for a node authored in a frame's own
/// coordinates. `Z` is not read, because such a node carries no `Z`
/// slot to read.
pub(crate) fn vec2<T: geom_core::Real>(
    values: &SlotValues<T>,
    f: fn(crate::node::Axis3) -> SlotId,
) -> Option<geom_core::Vec2<T>> {
    Some(geom_core::Vec2::new(
        scalar(values, f(crate::node::Axis3::X))?,
        scalar(values, f(crate::node::Axis3::Y))?,
    ))
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
