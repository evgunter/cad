//! Slot evaluation: every expression a node carries, evaluated in the
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
//!   (`super::tag::slot`). An authored frame's f64 placement is minted
//!   from the nominal list (`super::wire::mint_frame_placement`) and
//!   rides the frame's value, so a profile drawn on that frame READS it
//!   rather than asking this door for the frame's nine slots again;
//! - the mate solve, for a placer's slots at the nominal
//!   (`crate::mate::member`'s `node_slots`);
//! - [`crate::Placement::eval`], over a placement's own rows
//!   ([`eval_rows`]), for a caller holding a placement and no node.
//!
//! A slot's expression is evaluated in exactly one loop
//! ([`eval_rows`]), and a refusal at any of them arrives in one shape.

use geom_core::Decide;

use crate::expr::{EvalError, Expr, ParamEnv, eval, eval_count};
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
/// contribution is the resolved program stream, not slot values
/// (`super::tag::slot` carries what that means for the nominal half).
pub(crate) fn eval_slots<T: Decide, P: crate::ProfilePayload>(
    node: &Node<P>,
    env: &ParamEnv<T>,
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
    rows: impl IntoIterator<Item = (SlotId, &'e Expr)>,
    env: &ParamEnv<T>,
) -> Result<SlotValues<T>, (SlotId, EvalError)> {
    rows.into_iter()
        .map(|(slot, expr)| {
            let val = if slot.is_structural() {
                SlotVal::Count(eval_count(expr, env).map_err(|e| (slot, e))?)
            } else {
                SlotVal::Scalar(eval(expr, env).map_err(|e| (slot, e))?)
            };
            Ok((slot, val))
        })
        .collect()
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
    f: impl Fn(crate::node::Axis3) -> SlotId,
) -> Option<geom_core::Vec3<T>> {
    Some(geom_core::Vec3::new(
        scalar(values, f(crate::node::Axis3::X))?,
        scalar(values, f(crate::node::Axis3::Y))?,
        scalar(values, f(crate::node::Axis3::Z))?,
    ))
}
