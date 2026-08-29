//! `Doc` — the document as a PLAIN VALUE (spec D2, DESIGN.md D8: the
//! recipe is data): the recipe DAG plus document metadata. Cheap-clone
//! plain Rust (`Vec`/`BTreeMap`; no persistent-structure dependency —
//! document scale does not justify one; revisit only with corpus
//! latency data). All mutation goes through the pure
//! [`crate::edit::apply`]; undo/redo is keeping prior values.

use std::collections::BTreeMap;

use geom_core::Real;
use geom_core::tolerance::DEFAULT_EPS;

use crate::appearance::{AppearanceMap, AttrSet};
use crate::expr::{Dimension, Expr, ExprPath, ParamEnv, ParamValue};
use crate::names::StableName;
use crate::node::{Node, RecipeNodeId};

/// A document-level parameter name (spec D4's "parameter refs").
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ParamName(pub String);

impl ParamName {
    /// Convenience constructor.
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

/// A document-level named parameter's declared dimension and exact
/// stored value (spec D2/D4: `f64` bit-exact for continuous, `i64`
/// for Count — bit-identical replay is trivial by representation).
#[derive(Debug, Clone, PartialEq)]
pub enum DocParam {
    /// A continuous parameter in canonical kernel units.
    Continuous {
        /// Declared dimension (never `Count`; `apply` refuses).
        dim: Dimension,
        /// The value, exact `f64`.
        value: f64,
    },
    /// An integer Count parameter (structural material, spec D3).
    Count {
        /// The exact value.
        value: i64,
    },
}

impl DocParam {
    /// The parameter's dimension.
    pub fn dim(&self) -> Dimension {
        match self {
            Self::Continuous { dim, .. } => *dim,
            Self::Count { .. } => Dimension::Count,
        }
    }

    /// Bit-semantic equality (spec D7): continuous values compare by
    /// BITS (`0.0` ≠ `-0.0` here), everything else structurally.
    pub fn bit_eq(&self, other: &DocParam) -> bool {
        match (self, other) {
            (Self::Continuous { dim: da, value: va }, Self::Continuous { dim: db, value: vb }) => {
                da == db && va.to_bits() == vb.to_bits()
            }
            (Self::Count { value: a }, Self::Count { value: b }) => a == b,
            _ => false,
        }
    }
}

/// The document: recipe DAG (node map + insertion-ordered list) +
/// document metadata (spec D2; ratified F2's substrate). `P` is the
/// opaque profile payload (spec D1/D3 — see [`Node`]).
#[derive(Debug, Clone, PartialEq)]
pub struct Doc<P> {
    /// The monotone id counter: the next [`RecipeNodeId`] to mint.
    /// Never decremented — deletion does not free ids (spec D3).
    pub(crate) next_id: u64,
    /// The nodes, by stable id.
    pub(crate) nodes: BTreeMap<RecipeNodeId, Node<P>>,
    /// Insertion order of the live nodes (the recipe's presentation
    /// order; the DAG's edges are the nodes' input refs, spec D3).
    pub(crate) order: Vec<RecipeNodeId>,
    /// Document-level named parameters.
    pub(crate) params: BTreeMap<ParamName, DocParam>,
    /// The recorded modeling tolerance ε (H4's future landing:
    /// `SetTolerance` arrives in PR 6; until then the ratified
    /// compiled default, `geom_core::tolerance::DEFAULT_EPS`).
    pub(crate) epsilon: f64,
    /// Per-node witness data (M4 PR 4, SOLVER-DESIGN W1/W4): the
    /// opaque branch-selection datum of each sketch-bearing node,
    /// written ONLY by the recorded `ReWitness` edits (and, at M6, by
    /// committed sketch edits). Document state under GQ3 — undo/redo
    /// and replay need no special cases.
    pub(crate) witnesses: BTreeMap<RecipeNodeId, crate::witness::WitnessDatum>,
    /// Free-form document metadata (display units etc. — presentation
    /// only, GQ5). Empty in v1 (spec D2).
    pub(crate) metadata: BTreeMap<String, String>,
    /// Appearance attributes keyed by stable name (M4 PR 7;
    /// DESIGN.md's ratified attachment contract). Presentation
    /// metadata: NEVER enters evaluation content keys — see
    /// [`crate::appearance`] for the loss (N3/N5) and wrapper (B11)
    /// semantics.
    pub(crate) appearance: AppearanceMap,
}

impl<P> Default for Doc<P> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<P> Doc<P> {
    /// The empty document: no nodes, no params, recorded ε at the
    /// ratified compiled default (D4 ¶1) — replay's origin (spec D7).
    pub fn empty() -> Self {
        Self {
            next_id: 0,
            nodes: BTreeMap::new(),
            order: Vec::new(),
            params: BTreeMap::new(),
            epsilon: DEFAULT_EPS,
            witnesses: BTreeMap::new(),
            metadata: BTreeMap::new(),
            appearance: AppearanceMap::new(),
        }
    }

    /// The node with the given id, if live.
    pub fn node(&self, id: RecipeNodeId) -> Option<&Node<P>> {
        self.nodes.get(&id)
    }

    /// Live node ids in insertion order.
    pub fn order(&self) -> &[RecipeNodeId] {
        &self.order
    }

    /// Number of live nodes.
    pub fn len(&self) -> usize {
        self.order.len()
    }

    /// Whether the document has no live nodes.
    pub fn is_empty(&self) -> bool {
        self.order.is_empty()
    }

    /// The document-level named parameters.
    pub fn params(&self) -> &BTreeMap<ParamName, DocParam> {
        &self.params
    }

    /// The recorded modeling tolerance ε (spec D2; edited by PR 6's
    /// `SetTolerance`).
    pub fn epsilon(&self) -> f64 {
        self.epsilon
    }

    /// The document metadata map (empty in v1, spec D2).
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// The recorded witness datum of a sketch-bearing node, if any
    /// (SOLVER-DESIGN W1; written only by `ReWitness` edits).
    pub fn witness(&self, id: RecipeNodeId) -> Option<&crate::witness::WitnessDatum> {
        self.witnesses.get(&id)
    }

    /// Every recorded witness, by node.
    pub fn witnesses(&self) -> &BTreeMap<RecipeNodeId, crate::witness::WitnessDatum> {
        &self.witnesses
    }

    /// The appearance store: attributes by stable name (M4 PR 7;
    /// edited through `SetAppearance`/`ClearAppearance`; `Rebind`
    /// rewrites keys — the attribute rides the name).
    pub fn appearance(&self) -> &AppearanceMap {
        &self.appearance
    }

    /// One name's attributes, if any are attached.
    pub fn appearance_of(&self, name: &StableName) -> Option<&AttrSet> {
        self.appearance.get(name)
    }

    /// The expression subtree an [`ExprPath`] addresses, or `None` if
    /// the node is gone, the slot absent, or the path off the tree
    /// (spec D5).
    pub fn expr_at(&self, path: &ExprPath) -> Option<&Expr> {
        self.nodes
            .get(&path.node)?
            .expr(path.slot)?
            .descend(&path.path)
    }

    /// The evaluation environment for this document's parameters,
    /// embedding stored exact values into any [`Real`] `T` (spec D4:
    /// the evaluator is scalar-generic; units erase here, GQ5).
    pub fn param_env<T: Real>(&self) -> ParamEnv<T> {
        let bindings = self
            .params
            .iter()
            .map(|(name, p)| {
                let v = match *p {
                    DocParam::Continuous { dim, value } => ParamValue::Continuous {
                        dim,
                        value: T::from_f64(value),
                    },
                    DocParam::Count { value } => ParamValue::Count(value),
                };
                (name.clone(), v)
            })
            .collect();
        ParamEnv { bindings }
    }
}

impl<P: PartialEq> Doc<P> {
    /// Bit-semantic document equality (spec D7's replay-identity
    /// comparator; M4 PR 1 review non-blocker): every float field —
    /// expression literals, continuous doc params, recorded ε —
    /// compares by BITS; ids, order, structure, metadata compare
    /// structurally. `PartialEq` on `Doc` remains IEEE-semantic
    /// (conflates `±0.0`); use THIS for replay pins and audits.
    pub fn bit_eq(&self, other: &Doc<P>) -> bool {
        self.next_id == other.next_id
            && self.order == other.order
            && self.epsilon.to_bits() == other.epsilon.to_bits()
            // Witness bytes are exact data (no float semantics to
            // conflate) — structural equality IS bit equality here.
            && self.witnesses == other.witnesses
            && self.metadata == other.metadata
            // Appearance values are float-free by construction
            // (integers/bools/strings), so structural equality IS bit
            // equality here.
            && self.appearance == other.appearance
            && self.nodes.len() == other.nodes.len()
            && self.nodes.iter().all(|(id, node)| {
                other
                    .nodes
                    .get(id)
                    .is_some_and(|theirs| node.bit_eq(theirs))
            })
            && self.params.len() == other.params.len()
            && self.params.iter().all(|(name, p)| {
                other
                    .params
                    .get(name)
                    .is_some_and(|theirs| p.bit_eq(theirs))
            })
    }
}
