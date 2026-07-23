//! The DocEdit vocabulary v1 and the pure `apply` (spec D2 + D6).
//!
//! `apply` returns a NEW document value; the input is untouched.
//! Undo/redo is keeping prior values — no edit destroys history at
//! this layer (spec D2).

use crate::doc::{Doc, DocParam, ParamName};
use crate::expr::{Dimension, DimensionError, Expr, ExprPath};
use crate::node::{Node, RecipeNodeId, SlotId, StableName};

/// The v1 edit vocabulary (spec D6).
///
/// # Reserved arms (planned evolution, spec D6 — NOT ad hoc)
///
/// - `Rebind` — explicit name re-binding (NAMING-DESIGN N5/N6); PR 4.
/// - `ReWitness` — explicit witness adoption after a certified solve
///   (SOLVER-DESIGN W4: the repair is a recorded, replayable DocEdit —
///   never silent write-back); types land in PR 4 alongside naming's
///   Rebind, semantics with the M6 solver.
/// - `SetTolerance` — the recorded-ε edit with its flipped-predicate
///   audit (H4); PR 6.
/// - Appearance edits — presentation attachment by StableName; PR 7.
#[derive(Debug, Clone, PartialEq)]
pub enum DocEdit<P> {
    /// Insert a node; the new [`RecipeNodeId`] is minted from the
    /// document's monotone counter and returned in the
    /// [`EditRecord`]. Input refs must resolve to EXISTING nodes —
    /// which is also why insertion can never create a cycle.
    InsertNode {
        /// The node payload (data only, spec D3).
        node: Node<P>,
    },
    /// Delete a node. Refused while any live node references it
    /// (typed, spec D3/D6); the id is never reused afterwards.
    DeleteNode {
        /// The node to delete.
        id: RecipeNodeId,
    },
    /// Replace a CONTINUOUS slot's expression (Length/Angle/Scalar
    /// slots; spec D3's continuous parameters).
    SetParam {
        /// The node owning the slot.
        node: RecipeNodeId,
        /// The named slot (spec D5: never an index).
        slot: SlotId,
        /// The replacement expression (dimension re-checked).
        expr: Expr,
    },
    /// Replace a STRUCTURAL (Count-typed) slot's expression — a
    /// DISTINCT arm from [`DocEdit::SetParam`] so the structural/
    /// continuous divide is unlosable in the edit stream (spec D3,
    /// DESIGN.md "stated, not emergent").
    SetStructuralParam {
        /// The node owning the slot.
        node: RecipeNodeId,
        /// The named structural slot.
        slot: SlotId,
        /// The replacement Count expression.
        expr: Expr,
    },
    /// Replace the expression SUBTREE at an [`ExprPath`] (empty path =
    /// the whole slot), re-running dimension checks on rebuilt
    /// ancestors (spec D6).
    SetExpression {
        /// The subtree address.
        path: ExprPath,
        /// The replacement subtree.
        expr: Expr,
    },
    /// Create or replace a document-level named parameter (spec D6).
    /// A dimension change re-validates every referencing expression.
    SetDocParam {
        /// The parameter name.
        name: ParamName,
        /// The declared dimension and exact value.
        value: DocParam,
    },
}

/// Typed, specific edit refusal (spec D6: no stringly errors).
#[derive(Debug, Clone, PartialEq)]
pub enum EditError {
    /// The edit targets a node id that is not live.
    UnknownNode {
        /// The missing id.
        id: RecipeNodeId,
    },
    /// An inserted node's input ref does not resolve to a live node
    /// (spec D3: `apply` rejects unresolvable refs).
    UnresolvedInput {
        /// The dangling upstream reference.
        input: RecipeNodeId,
    },
    /// The recipe graph would contain a cycle (defensive: insertion
    /// referencing only pre-existing nodes cannot cycle, but the
    /// invariant is CHECKED, not assumed — spec D3).
    WouldCycle {
        /// A node on the detected cycle.
        at: RecipeNodeId,
    },
    /// Deleting this node would dangle a live reference to it.
    DeleteWouldDangle {
        /// The deletion target.
        id: RecipeNodeId,
        /// A live node still referencing it.
        referenced_by: RecipeNodeId,
    },
    /// The node does not carry the named slot.
    UnknownSlot {
        /// The node.
        id: RecipeNodeId,
        /// The slot it lacks.
        slot: SlotId,
    },
    /// The expression's dimension does not match the slot's required
    /// dimension (checks re-run on every touched expression, spec D6).
    SlotDimensionMismatch {
        /// The slot.
        slot: SlotId,
        /// The slot's required dimension.
        expected: Dimension,
        /// The offered expression's dimension.
        found: Dimension,
    },
    /// `SetParam` aimed at a STRUCTURAL slot — structural edits go
    /// through the distinct `SetStructuralParam` arm (spec D3).
    StructuralSlotNeedsStructuralEdit {
        /// The structural slot.
        slot: SlotId,
    },
    /// `SetStructuralParam` aimed at a continuous slot.
    NotStructuralSlot {
        /// The continuous slot.
        slot: SlotId,
    },
    /// An expression references a document parameter that does not
    /// exist.
    UnknownDocParam {
        /// The missing parameter.
        name: ParamName,
        /// The referencing node.
        node: RecipeNodeId,
        /// The referencing slot.
        slot: SlotId,
    },
    /// An expression's recorded ref dimension disagrees with the
    /// document parameter's declared dimension.
    DocParamDimensionMismatch {
        /// The parameter.
        name: ParamName,
        /// The referencing node.
        node: RecipeNodeId,
        /// The referencing slot.
        slot: SlotId,
        /// The document table's declared dimension.
        declared: Dimension,
        /// The dimension the expression's ref recorded.
        referenced: Dimension,
    },
    /// A `Continuous` doc param declared with `Dimension::Count` —
    /// Count parameters use [`DocParam::Count`] (exact integers).
    ContinuousParamCannotBeCount {
        /// The parameter.
        name: ParamName,
    },
    /// A `SetExpression` path runs off the expression tree (spec D5).
    PathOffTree {
        /// The offending address.
        path: ExprPath,
    },
    /// Replacing the subtree broke an ancestor's dimension check.
    Dimension(DimensionError),
    /// A `Declare` payload names a node that does not exist at edit
    /// time (spec D3 carve-out, ruled): a never-existed id is a TYPO,
    /// refused at the best-diagnostics door. (A later `DeleteNode`
    /// stranding a name is ALLOWED — N5 dangling semantics; see
    /// [`Node::Declare`].)
    DeclareNamesMissingNode {
        /// The name whose node is not live.
        name: StableName,
    },
    /// A non-finite (NaN/inf) continuous doc-param value — refused at
    /// the edit door (ruled door 1 of the non-finite policy; F3's
    /// persist-time refusal then has nothing to catch).
    NonFiniteDocParam {
        /// The parameter.
        name: ParamName,
    },
}

/// What an accepted edit did (spec D6: structural edits are FLAGGED
/// in the returned record; the record also returns the minted id —
/// without it a caller could never reference an inserted node).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditRecord {
    /// The id minted by an `InsertNode`, `None` otherwise.
    pub minted: Option<RecipeNodeId>,
    /// Whether the edit was STRUCTURAL (spec D3/D6): it can change
    /// the result's combinatorial shape — insert/delete, a
    /// Count-slot expression edit, or a Count doc-param set.
    /// Continuous edits (`SetParam`, continuous-slot `SetExpression`,
    /// continuous `SetDocParam`) leave recipe structure fixed.
    pub structural: bool,
}

/// An accepted edit: the NEW document (the input untouched, spec D2)
/// plus the [`EditRecord`].
#[derive(Debug, Clone, PartialEq)]
pub struct Applied<P> {
    /// The new document value.
    pub doc: Doc<P>,
    /// What the edit did.
    pub record: EditRecord,
}

/// Validate one expression's document-parameter refs against the
/// param table (spec D6: dimension checks re-run on touched
/// expressions; `node`/`slot` locate the expression for the error).
fn check_param_refs<P>(
    doc: &Doc<P>,
    node: RecipeNodeId,
    slot: SlotId,
    expr: &Expr,
) -> Result<(), EditError> {
    let mut refs = Vec::new();
    expr.param_refs(&mut refs);
    for (name, referenced) in refs {
        match doc.params().get(&name) {
            None => return Err(EditError::UnknownDocParam { name, node, slot }),
            Some(p) if p.dim() != referenced => {
                return Err(EditError::DocParamDimensionMismatch {
                    name,
                    node,
                    slot,
                    declared: p.dim(),
                    referenced,
                });
            }
            Some(_) => {}
        }
    }
    Ok(())
}

/// Validate every slot of a node payload against slot dimensions and
/// the param table, keyed as `id` for error reporting.
fn check_node_slots<P>(doc: &Doc<P>, id: RecipeNodeId, node: &Node<P>) -> Result<(), EditError> {
    for slot in node.slots() {
        // slots() and expr() agree by construction; a miss here is a
        // vocabulary bug, surfaced as UnknownSlot rather than hidden.
        let Some(expr) = node.expr(slot) else {
            return Err(EditError::UnknownSlot { id, slot });
        };
        if expr.dim() != slot.dimension() {
            return Err(EditError::SlotDimensionMismatch {
                slot,
                expected: slot.dimension(),
                found: expr.dim(),
            });
        }
        check_param_refs(doc, id, slot, expr)?;
    }
    Ok(())
}

/// Reject cycles in the recipe DAG (spec D3/D6). Defensive: insertion
/// referencing only existing nodes cannot cycle, but the invariant is
/// checked. Iterative DFS, three-color, deterministic order.
fn check_acyclic<P>(doc: &Doc<P>) -> Result<(), EditError> {
    use std::collections::BTreeMap;
    #[derive(Clone, Copy, PartialEq)]
    enum Color {
        White,
        Grey,
        Black,
    }
    let mut color: BTreeMap<RecipeNodeId, Color> =
        doc.order().iter().map(|&id| (id, Color::White)).collect();
    for &root in doc.order() {
        if color.get(&root) != Some(&Color::White) {
            continue;
        }
        // Stack of (node, next-input-index) frames.
        let mut stack = vec![(root, 0usize)];
        color.insert(root, Color::Grey);
        while let Some(&mut (id, ref mut next)) = stack.last_mut() {
            let inputs = doc.node(id).map(|n| n.inputs()).unwrap_or_default();
            if *next >= inputs.len() {
                color.insert(id, Color::Black);
                stack.pop();
                continue;
            }
            let input = inputs[*next];
            *next += 1;
            match color.get(&input) {
                Some(Color::Grey) => return Err(EditError::WouldCycle { at: input }),
                Some(Color::White) => {
                    color.insert(input, Color::Grey);
                    stack.push((input, 0));
                }
                // Black (done) or a ref outside the map (dangling —
                // reported by ref checks, not the cycle walk).
                _ => {}
            }
        }
    }
    Ok(())
}

/// Apply one edit to a document, PURELY (spec D2): the input is
/// untouched; on acceptance a new value comes back with the
/// [`EditRecord`]. All validation is here — refs resolve, no cycles,
/// dimension checks re-run on touched expressions (spec D6).
#[allow(clippy::too_many_lines)] // one arm per DocEdit variant, each short
pub fn apply<P: Clone>(doc: &Doc<P>, edit: &DocEdit<P>) -> Result<Applied<P>, EditError> {
    let mut new = doc.clone();
    let record = match edit {
        DocEdit::InsertNode { node } => {
            for input in node.inputs() {
                if !new.nodes.contains_key(&input) {
                    return Err(EditError::UnresolvedInput { input });
                }
            }
            // Spec D3 carve-out (ruled): name refs (Declare pairs)
            // must point at LIVE nodes at edit time — a never-existed
            // id is a typo. They are not DAG edges: later deletes may
            // strand them (N5), so this is the ONLY door that checks.
            if let Node::Declare { pairs } = node {
                for name in pairs.iter().flat_map(|(a, b)| [a, b]) {
                    if !new.nodes.contains_key(&name.node) {
                        return Err(EditError::DeclareNamesMissingNode { name: name.clone() });
                    }
                }
            }
            let id = RecipeNodeId(new.next_id);
            check_node_slots(&new, id, node)?;
            new.next_id += 1;
            new.nodes.insert(id, node.clone());
            new.order.push(id);
            check_acyclic(&new)?;
            EditRecord {
                minted: Some(id),
                structural: true,
            }
        }
        DocEdit::DeleteNode { id } => {
            if !new.nodes.contains_key(id) {
                return Err(EditError::UnknownNode { id: *id });
            }
            for (&other, node) in &new.nodes {
                if other != *id && node.inputs().contains(id) {
                    return Err(EditError::DeleteWouldDangle {
                        id: *id,
                        referenced_by: other,
                    });
                }
            }
            new.nodes.remove(id);
            new.order.retain(|&n| n != *id);
            // next_id is NOT decremented: ids are never reused (D3).
            EditRecord {
                minted: None,
                structural: true,
            }
        }
        DocEdit::SetParam { node, slot, expr } => {
            if slot.is_structural() {
                return Err(EditError::StructuralSlotNeedsStructuralEdit { slot: *slot });
            }
            set_slot(&mut new, *node, *slot, expr)?;
            EditRecord {
                minted: None,
                structural: false,
            }
        }
        DocEdit::SetStructuralParam { node, slot, expr } => {
            if !slot.is_structural() {
                return Err(EditError::NotStructuralSlot { slot: *slot });
            }
            set_slot(&mut new, *node, *slot, expr)?;
            EditRecord {
                minted: None,
                structural: true,
            }
        }
        DocEdit::SetExpression { path, expr } => {
            let Some(node) = new.nodes.get(&path.node) else {
                return Err(EditError::UnknownNode { id: path.node });
            };
            let Some(root) = node.expr(path.slot) else {
                return Err(EditError::UnknownSlot {
                    id: path.node,
                    slot: path.slot,
                });
            };
            let rebuilt = root
                .with_replaced(&path.path, expr.clone())
                .ok_or_else(|| EditError::PathOffTree { path: path.clone() })?
                .map_err(EditError::Dimension)?;
            let structural = path.slot.is_structural();
            set_slot(&mut new, path.node, path.slot, &rebuilt)?;
            EditRecord {
                minted: None,
                structural,
            }
        }
        DocEdit::SetDocParam { name, value } => {
            if let DocParam::Continuous {
                dim: Dimension::Count,
                ..
            } = value
            {
                return Err(EditError::ContinuousParamCannotBeCount { name: name.clone() });
            }
            // Ruled door 1 (non-finite policy): recipe data never
            // carries NaN/inf.
            if let DocParam::Continuous { value: v, .. } = value
                && !v.is_finite()
            {
                return Err(EditError::NonFiniteDocParam { name: name.clone() });
            }
            let structural = matches!(value, DocParam::Count { .. });
            new.params.insert(name.clone(), value.clone());
            // A (re)declaration can change the dimension out from
            // under referencing expressions: re-validate every slot
            // (documents are small; spec D6's re-run requirement).
            for &id in &new.order {
                if let Some(node) = new.nodes.get(&id) {
                    check_node_slots(&new, id, node)?;
                }
            }
            EditRecord {
                minted: None,
                structural,
            }
        }
    };
    Ok(Applied { doc: new, record })
}

/// Shared slot-write path: node exists, slot exists, dimension
/// matches, param refs valid — then write.
fn set_slot<P: Clone>(
    new: &mut Doc<P>,
    id: RecipeNodeId,
    slot: SlotId,
    expr: &Expr,
) -> Result<(), EditError> {
    let Some(node) = new.nodes.get(&id) else {
        return Err(EditError::UnknownNode { id });
    };
    if node.expr(slot).is_none() {
        return Err(EditError::UnknownSlot { id, slot });
    }
    if expr.dim() != slot.dimension() {
        return Err(EditError::SlotDimensionMismatch {
            slot,
            expected: slot.dimension(),
            found: expr.dim(),
        });
    }
    check_param_refs(new, id, slot, expr)?;
    if let Some(target) = new.nodes.get_mut(&id).and_then(|n| n.expr_mut(slot)) {
        *target = expr.clone();
        Ok(())
    } else {
        Err(EditError::UnknownSlot { id, slot })
    }
}

impl<P: Clone> Doc<P> {
    /// Method form of [`apply`] (spec D2's pure edit entry point).
    pub fn apply(&self, edit: &DocEdit<P>) -> Result<Applied<P>, EditError> {
        apply(self, edit)
    }

    /// Replay an edit list from the EMPTY document (spec D7): the
    /// result reproduces the edits' document BIT-IDENTICALLY (floats
    /// are stored exactly; ids re-mint deterministically).
    pub fn replay(edits: &[DocEdit<P>]) -> Result<Doc<P>, EditError> {
        let mut doc = Doc::empty();
        for edit in edits {
            doc = apply(&doc, edit)?.doc;
        }
        Ok(doc)
    }
}
