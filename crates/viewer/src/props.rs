//! The property panel's model: a node's slots, their current values,
//! and what an edit to each one is allowed to be.
//!
//! # Canonical units, and nothing else
//!
//! Every value here is in kernel-canonical metres and radians. The
//! ruling for v1 is explicit that the units/display layer is not a
//! dependency of the panels, so a value is displayed as it is stored;
//! the dimension is reported beside it ([`SlotRow::dimension`]) so a
//! reader knows which canonical unit they are looking at.
//!
//! # Structural is not continuous
//!
//! The recipe's structural/continuous divide is typed, not emergent:
//! a Count slot is edited by `SetStructuralParam` and everything else
//! by `SetParam`, and [`SlotRow::structural`] carries which. The panel
//! never picks the edit arm by inspecting the value.
//!
//! # The expression-driven refusal
//!
//! Setting a number into a slot that is DRIVEN — by a document
//! parameter or by arithmetic — is refused, with an affordance
//! (the ratified micro-decision). [`SlotDriver`] is how a slot says
//! which it is, using only public expression API: a bare literal is a
//! leaf with no parameter references, and anything else is driven.
//!
//! **What the affordance can and cannot offer, measured against this
//! substrate.** The typed expression API has a text door INWARD —
//! `parse_expr` — so the affordance offers a text field whose contents
//! become a new expression through it, with no parser written here.
//! It has no door OUTWARD: `Expr` exposes its dimension, its literal
//! value, its children and its parameter references, but no operator
//! identity, so neither this crate nor any other consumer can render
//! an existing expression back to its source text. The field therefore
//! cannot be pre-filled with what the slot says today. What the panel
//! shows instead is what the substrate does expose and what a user
//! actually needs in order to act: the slot's CURRENT VALUE under the
//! document's parameters, and the names of the parameters driving it —
//! each of which the panel can navigate to and edit as a document
//! parameter. An unparser in `editor-core` would close the gap and is
//! scheduled as issue #1103, which names this affordance as the
//! consumer that wants it; it is not this unit's work.

use pncad::document::{
    Dimension, Doc, DocEdit, DocParam, EvalError, Expr, Node, ParamName, ProfileProgram,
    RecipeNodeId, SlotId, eval, eval_count,
};

/// What is in a slot right now.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SlotValue {
    /// A continuous value, canonical metres/radians (or a bare
    /// scalar).
    Continuous(f64),
    /// An exact integer, from a Count slot.
    Count(i64),
}

impl SlotValue {
    /// **The one rule for which arm a widget's `f64` becomes.**
    ///
    /// The DIMENSION decides, and nothing else: a `Count` dimension
    /// takes the value truncated toward zero, everything else takes it
    /// as-is. Not the widget, not the arm the value currently has, and
    /// not the slot's structurality read separately — `SlotId::
    /// is_structural` is itself defined as "the dimension is Count", so
    /// deciding on the dimension is deciding on the same thing in one
    /// place instead of three.
    ///
    /// Three call sites wanted this and two of them had spelled it
    /// differently, which is why it is a function.
    pub fn of(dimension: Dimension, value: f64) -> Self {
        if dimension == Dimension::Count {
            Self::Count(value as i64)
        } else {
            Self::Continuous(value)
        }
    }

    /// The value as an `f64`, for a widget that has only one kind of
    /// number. Lossless for every count a recipe can carry.
    pub fn as_f64(self) -> f64 {
        match self {
            Self::Continuous(v) => v,
            Self::Count(v) => v as f64,
        }
    }
}

/// What decides a slot's value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SlotDriver {
    /// A bare literal: editable in place.
    Literal,
    /// An expression. Direct numeric editing is refused; `params` are
    /// the document parameters it references, in first-seen order and
    /// deduplicated — the affordance's navigation targets.
    Expression {
        /// The parameters this expression reads.
        params: Vec<ParamName>,
    },
}

impl SlotDriver {
    /// Classify an expression using public expression API only.
    ///
    /// A bare literal is a LEAF (no child at index 0) that references
    /// no parameter. Everything else — a parameter reference, or any
    /// arithmetic, however constant — is driven, which is the
    /// conservative direction: refusing to overwrite a computed slot
    /// is recoverable, silently flattening one to a number is not.
    pub fn of(expr: &Expr) -> Self {
        let mut refs = Vec::new();
        expr.param_refs(&mut refs);
        if refs.is_empty() && expr.child(0).is_none() {
            return Self::Literal;
        }
        let mut params: Vec<ParamName> = Vec::new();
        for (name, _) in refs {
            if !params.contains(&name) {
                params.push(name);
            }
        }
        Self::Expression { params }
    }

    /// Whether this slot refuses a direct numeric edit.
    pub fn is_driven(&self) -> bool {
        matches!(self, Self::Expression { .. })
    }
}

/// Why a slot has no value to show.
#[derive(Clone, Debug, PartialEq)]
pub enum SlotFault {
    /// The expression did not evaluate — the kernel's own typed reason
    /// (an unbound parameter, a non-finite result).
    Eval(EvalError),
    /// The node LISTED this slot and then carried no expression for it.
    /// A broken `Node::slots`/`Node::expr` postcondition, reported
    /// rather than skipped; see [`slot_row`].
    NoExpression,
}

impl core::fmt::Display for SlotFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Eval(error) => write!(f, "{error}"),
            Self::NoExpression => write!(
                f,
                "the node lists this slot but carries no expression for it"
            ),
        }
    }
}

impl core::error::Error for SlotFault {}

/// One editable row of the property panel.
#[derive(Clone, Debug, PartialEq)]
pub struct SlotRow {
    /// The named slot (never an index).
    pub slot: SlotId,
    /// The dimension the slot's expression must have.
    pub dimension: Dimension,
    /// Whether the slot is structural — the `SetStructuralParam`
    /// half of the edit vocabulary.
    pub structural: bool,
    /// Literal, or driven by an expression.
    pub driver: SlotDriver,
    /// The value the slot has under the document's parameters, or the
    /// typed reason it has none.
    pub value: Result<SlotValue, SlotFault>,
}

/// The rows for one node, in the node vocabulary's own slot order.
///
/// Empty for a node that carries no expressions (a boolean, a mate, an
/// instance) — which is a true statement about that node, not a
/// failure.
pub fn slot_rows(doc: &Doc<ProfileProgram>, id: RecipeNodeId) -> Vec<SlotRow> {
    let Some(node) = doc.node(id) else {
        return Vec::new();
    };
    node.slots()
        .into_iter()
        .map(|slot| slot_row(doc, node, slot))
        .collect()
}

/// One row for a slot the node lists.
///
/// **`Node::slots()` is authoritative**: it is documented as "the
/// domain of `Node::expr`", so a slot it lists and `expr` denies is a
/// broken postcondition in the node vocabulary, not a slot this panel
/// should quietly skip. Dropping it silently is what a `filter_map`
/// here used to do — a fail-loud codebase's panel showing a node with
/// one fewer row than it has and no way to notice.
///
/// The row is still a value, not a panic: the panel says the slot is
/// there and that its value could not be read, which is the same shape
/// every other unreadable value takes here.
fn slot_row(doc: &Doc<ProfileProgram>, node: &Node<ProfileProgram>, slot: SlotId) -> SlotRow {
    let Some(expr) = node.expr(slot) else {
        return SlotRow {
            slot,
            dimension: slot.dimension(),
            structural: slot.is_structural(),
            // No expression to classify. Reported as driven, which is
            // the refusing direction: nothing here should be
            // overwritten with a number on the strength of an
            // invariant that just failed.
            driver: SlotDriver::Expression { params: Vec::new() },
            value: Err(SlotFault::NoExpression),
        };
    };
    let env = doc.param_env::<f64>();
    let value = if slot.dimension() == Dimension::Count {
        eval_count(expr, &env)
            .map(SlotValue::Count)
            .map_err(SlotFault::Eval)
    } else {
        eval::<f64>(expr, &env)
            .map(SlotValue::Continuous)
            .map_err(SlotFault::Eval)
    };
    SlotRow {
        slot,
        dimension: slot.dimension(),
        structural: slot.is_structural(),
        driver: SlotDriver::of(expr),
        value,
    }
}

/// One document-level parameter, as the panel shows it.
#[derive(Clone, Debug, PartialEq)]
pub struct ParamRow {
    /// The parameter's name.
    pub name: ParamName,
    /// Its declared dimension.
    pub dimension: Dimension,
    /// Its exact stored value.
    pub value: SlotValue,
}

/// Every document parameter, name order.
pub fn param_rows(doc: &Doc<ProfileProgram>) -> Vec<ParamRow> {
    doc.params()
        .iter()
        .map(|(name, param)| ParamRow {
            name: name.clone(),
            dimension: param.dim(),
            value: match param {
                DocParam::Continuous { value, .. } => SlotValue::Continuous(*value),
                DocParam::Count { value } => SlotValue::Count(*value),
            },
        })
        .collect()
}

/// The edit that writes `value` into `slot` on `node`.
///
/// The structural/continuous divide is decided by the SLOT, which is
/// the only thing that knows: a Count slot takes
/// `SetStructuralParam`, everything else `SetParam`. Returning the
/// edit rather than applying it keeps this a pure function of the
/// request, which is what lets a test assert on the emitted edit.
///
/// # Errors
///
/// The dimension refusal `Expr::literal` raises for a value that is
/// not finite.
pub fn slot_edit(
    node: RecipeNodeId,
    slot: SlotId,
    value: SlotValue,
) -> Result<DocEdit<ProfileProgram>, pncad::document::DimensionError> {
    let expr = match value {
        SlotValue::Count(count) => Expr::count(count),
        SlotValue::Continuous(v) => Expr::literal(v, slot.dimension())?,
    };
    Ok(if slot.is_structural() {
        DocEdit::SetStructuralParam { node, slot, expr }
    } else {
        DocEdit::SetParam { node, slot, expr }
    })
}

/// The edit that replaces a document parameter's value, keeping its
/// declared dimension.
pub fn param_edit(
    name: ParamName,
    dimension: Dimension,
    value: SlotValue,
) -> DocEdit<ProfileProgram> {
    let param = match value {
        SlotValue::Count(value) => DocParam::Count { value },
        SlotValue::Continuous(value) => DocParam::Continuous {
            dim: dimension,
            value,
        },
    };
    DocEdit::SetDocParam { name, value: param }
}
