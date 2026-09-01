//! `Doc` — the document as a PLAIN VALUE (spec D2, DESIGN.md D8: the
//! recipe is data): the recipe DAG plus document metadata. Cheap-clone
//! plain Rust (`Vec`/`BTreeMap`; no persistent-structure dependency —
//! document scale does not justify one; revisit only with corpus
//! latency data). All mutation goes through the pure
//! [`crate::edit::apply`]; undo/redo is keeping prior values.

use std::collections::BTreeMap;

use geom_core::Real;

use crate::appearance::{AppearanceMap, AppearanceRecord};
use crate::distribution::Distribution;
use crate::expr::{Dimension, Expr, ExprPath, ParamEnv, ParamValue};
use crate::ident::DocumentId;
use crate::names::StableName;
use crate::node::{Node, RecipeNodeId};
use geom_core::Tol;

/// A document-level parameter name (spec D4's "parameter refs").
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
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
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum DocParam {
    /// A continuous parameter in canonical kernel units.
    Continuous {
        /// Declared dimension (never `Count`; `apply` refuses).
        dim: Dimension,
        /// The value, exact `f64`.
        value: f64,
        /// The display unit this parameter was AUTHORED in, if any —
        /// the same presentation metadata a literal carries
        /// (`Expr::display_unit`), for the same reason: a parameter
        /// authored in millimetres should read back in millimetres.
        ///
        /// `None` means no authored notation, which is NOT the same as
        /// the canonical unit: a reader renders it in whatever it
        /// renders unmarked values in, and for angles that differs
        /// from `Some(rad)`.
        ///
        /// It rides with the DECLARATION, beside `dim` and
        /// `distribution`, and not with the value — which is exactly
        /// why [`crate::DocEdit::SetDocParamValue`] leaves it alone
        /// (see [`DocParamValue`]): how a parameter is written is a
        /// fact about the parameter, not about the number being typed
        /// into it.
        ///
        /// The unit must MEASURE `dim`. Nothing here can enforce that
        /// — the payload is `pub` and the dimension is data — so the
        /// pairing is a document invariant checked by the shared
        /// save/load validator (`persist::check`), like every other
        /// invariant this `pub` payload can be corrupted past. The
        /// authoring doors ([`DocParam::written_length`],
        /// [`DocParam::written_angle`]) cannot produce a mismatched
        /// one at all.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        display_unit: Option<crate::expr::UnitSym>,
        /// Optional uncertainty about this parameter (ERROR-DESIGN
        /// E1/E2), as offsets from `value` in the parameter's own
        /// `dim`. Document metadata read ONLY by
        /// [`crate::analysis`]: it enters no evaluation, no content
        /// key and no predicate, and `None` — the default — means the
        /// parameter is FIXED, not that its uncertainty is unknown.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        distribution: Option<Distribution>,
    },
    /// An integer Count parameter (structural material, spec D3).
    ///
    /// Carries NO distribution, and cannot: structural parameters are
    /// fixed under any error analysis (E11.3), which comes out
    /// UNREPRESENTABLE here rather than as a refusal — there is no
    /// spelling to refuse.
    Count {
        /// The exact value.
        value: i64,
    },
}

/// The VALUE half of a document parameter, with no declaration
/// attached: what a value-only edit
/// ([`crate::DocEdit::SetDocParamValue`]) writes.
///
/// A parameter's declaration — its dimension, and its optional
/// [`Distribution`] — belongs to the parameter, not to the number
/// being typed into it. Carrying only the number is what lets the
/// value door leave both alone; a caller that rebuilds a whole
/// [`DocParam`] from `(dim, value)` deletes the annotation, silently,
/// because [`crate::DocEdit::SetDocParam`] is create-or-replace.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum DocParamValue {
    /// A continuous parameter's nominal, in its ALREADY-DECLARED
    /// dimension (canonical kernel units).
    Continuous(f64),
    /// A `Count` parameter's exact integer.
    Count(i64),
}

impl DocParamValue {
    /// Whether this is the `Count` arm — the kind a value edit must
    /// match against the existing declaration.
    pub fn is_count(&self) -> bool {
        matches!(self, Self::Count(_))
    }
}

impl core::fmt::Display for DocParamValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Continuous(v) => write!(f, "continuous {v}"),
            Self::Count(v) => write!(f, "count {v}"),
        }
    }
}

impl DocParam {
    /// The parameter's dimension.
    pub fn dim(&self) -> Dimension {
        match self {
            Self::Continuous { dim, .. } => *dim,
            Self::Count { .. } => Dimension::Count,
        }
    }

    /// A continuous LENGTH parameter that remembers the notation it
    /// was authored in ([`quantity::WrittenLength`]).
    ///
    /// **Total** — there is no dimension for the unit to disagree
    /// with: a `WrittenLength` holds a `LengthUnit`, which is an index
    /// into a Length row of the table (#669), and this door supplies
    /// `Dimension::Length` itself. The mismatch the validator watches
    /// for is unreachable through here, which is the whole reason to
    /// author through it rather than through the `pub` payload.
    ///
    /// No distribution: the E1/E2 annotation belongs to the parameter
    /// and is added through its own door, exactly as
    /// [`DocParam::continuous`] leaves it alone.
    pub fn written_length(written: quantity::WrittenLength) -> Self {
        Self::Continuous {
            dim: Dimension::Length,
            value: written.meters(),
            display_unit: Some(crate::expr::UnitSym::from_def(&written.unit().def())),
            distribution: None,
        }
    }

    /// A continuous ANGLE parameter that remembers its authored
    /// notation — [`DocParam::written_length`]'s mirror, total for the
    /// same reason.
    pub fn written_angle(written: quantity::WrittenAngle) -> Self {
        Self::Continuous {
            dim: Dimension::Angle,
            value: written.radians_value(),
            display_unit: Some(crate::expr::UnitSym::from_def(&written.unit().def())),
            distribution: None,
        }
    }

    /// A continuous parameter with no distribution and no authored
    /// notation — the plain authoring spelling.
    pub fn continuous(dim: Dimension, value: f64) -> Self {
        Self::Continuous {
            dim,
            value,
            display_unit: None,
            distribution: None,
        }
    }

    /// A continuous parameter carrying `distribution` — the annotated
    /// authoring spelling. Only continuous parameters can be
    /// annotated, so this is a constructor rather than a method: there
    /// is no `Count` case to silently drop the annotation.
    pub fn continuous_with(dim: Dimension, value: f64, distribution: Distribution) -> Self {
        Self::Continuous {
            dim,
            value,
            display_unit: None,
            distribution: Some(distribution),
        }
    }

    /// This parameter's distribution, if it is continuous and carries
    /// one.
    pub fn distribution(&self) -> Option<&Distribution> {
        match self {
            Self::Continuous { distribution, .. } => distribution.as_ref(),
            Self::Count { .. } => None,
        }
    }

    /// This parameter with `value` written into it, keeping the whole
    /// DECLARATION — the dimension, the authored display unit and the
    /// optional distribution — untouched. **The carry-forward, in one place**: every value
    /// door goes through here rather than rebuilding a parameter from
    /// parts, so no door can drop an annotation it never mentioned.
    ///
    /// `None` when the value's arm does not match the declaration's.
    /// Changing a parameter's kind is a REDECLARATION — the
    /// create-or-replace door, where the dimension and the annotation
    /// are stated afresh — and a value edit that quietly performed one
    /// would be the same silent deletion in a different disguise.
    pub fn with_value(&self, value: DocParamValue) -> Option<Self> {
        match (self, value) {
            (
                Self::Continuous {
                    dim,
                    display_unit,
                    distribution,
                    ..
                },
                DocParamValue::Continuous(value),
            ) => Some(Self::Continuous {
                dim: *dim,
                value,
                display_unit: *display_unit,
                distribution: *distribution,
            }),
            (Self::Count { .. }, DocParamValue::Count(value)) => Some(Self::Count { value }),
            // EXHAUSTIVE on purpose, both sides spelled: a new
            // `DocParam` arm or a new value arm must say how a value
            // edit reaches it, or the compile breaks.
            (Self::Continuous { .. }, DocParamValue::Count(_))
            | (Self::Count { .. }, DocParamValue::Continuous(_)) => None,
        }
    }

    /// Bit-semantic equality (spec D7): continuous values compare by
    /// BITS (`0.0` ≠ `-0.0` here), everything else structurally.
    ///
    /// **The display unit is EXCLUDED**, exactly as it is from
    /// `Expr::bit_eq` and for the same reason: it is presentation
    /// metadata, so two parameters differing only in how they are
    /// written are the same parameter. The consequence is the one the
    /// literal already has — a change of notation alone is a document
    /// edit that enters the history and persists, and is invisible to
    /// replay identity and to `diff.rs`. Spelled out rather than
    /// omitted, because the field is named in the pattern below and a
    /// reader is owed the reason it is not in the comparison.
    ///
    /// EXHAUSTIVE on purpose, on BOTH sides of the pair: the mismatched
    /// pairs are spelled out rather than swept up, so a future
    /// `DocParam` variant must say how it compares here or the compile
    /// breaks. A wildcard would have answered `false` for a new variant
    /// against ITSELF — two equal parameters reported as differing,
    /// through [`Doc::bit_eq`] and `diff.rs`, which is D7's replay
    /// identity and the document diff reading the same wrong answer.
    pub fn bit_eq(&self, other: &DocParam) -> bool {
        match (self, other) {
            (
                Self::Continuous {
                    dim: da,
                    value: va,
                    display_unit: _,
                    distribution: ha,
                },
                Self::Continuous {
                    dim: db,
                    value: vb,
                    display_unit: _,
                    distribution: hb,
                },
            ) => {
                da == db
                    && va.to_bits() == vb.to_bits()
                    // Present-vs-present compares BIT-exact on the
                    // offsets; present-vs-absent differs.
                    && match (ha, hb) {
                        (None, None) => true,
                        (Some(a), Some(b)) => a.bit_eq(b),
                        (None, Some(_)) | (Some(_), None) => false,
                    }
            }
            (Self::Count { value: a }, Self::Count { value: b }) => a == b,
            (Self::Continuous { .. }, Self::Count { .. })
            | (Self::Count { .. }, Self::Continuous { .. }) => false,
        }
    }
}

/// The document: recipe DAG (node map + insertion-ordered list) +
/// document metadata (spec D2; ratified F2's substrate). `P` is the
/// opaque profile payload (spec D1/D3 — see [`Node`]).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
// The `with`-routed nodes field hides `P` from serde's bound
// inference; state the bounds explicitly.
#[serde(bound(
    serialize = "P: serde::Serialize",
    deserialize = "P: serde::Deserialize<'de>"
))]
pub struct Doc<P> {
    /// The document's stable identity (ASM-1 D-1): authored data
    /// supplied at construction, never minted from ambient randomness
    /// in this crate. Survives every edit; excluded from the content
    /// pin (the pin answers "which version", the id "which part").
    pub(crate) id: DocumentId,
    /// The monotone id counter: the next [`RecipeNodeId`] to mint.
    /// Never decremented — deletion does not free ids (spec D3).
    pub(crate) next_id: u64,
    /// The nodes, by stable id.
    #[serde(with = "crate::persist::strict::nodes")]
    pub(crate) nodes: BTreeMap<RecipeNodeId, Node<P>>,
    /// Insertion order of the live nodes (the recipe's presentation
    /// order; the DAG's edges are the nodes' input refs, spec D3).
    pub(crate) order: Vec<RecipeNodeId>,
    /// The document's ordered product roots (ASSEMBLY-DESIGN A10,
    /// ASM-ROOTS D-1): document data, never a DAG node. Two invariants
    /// hold at rest and after every edit — *coverage* (every node is
    /// an ancestor of, or is, some root) and *ancestor-freedom* (no
    /// root is a strict ancestor of another) — which together say the
    /// root SET is exactly the DAG's sink set; the list adds the
    /// product's solid ORDER, which is therefore semantic. No
    /// duplicates; every entry is live.
    pub(crate) roots: Vec<RecipeNodeId>,
    /// Cluster placement frames (ASSEMBLY-DESIGN A11, ASM-2A D-2):
    /// document data keyed by the instantiate node whose singleton
    /// cluster the frame places. A MISSING entry is the identity frame
    /// — a legal, complete state — so the registry never needs a row
    /// per instance, and zero-/multi-anchor states cannot be spelled.
    /// Written only by the recorded `SetPlacement` edit; every key
    /// names a live `InstantiatePart` node.
    #[serde(with = "crate::persist::strict::placements")]
    pub(crate) placements: BTreeMap<RecipeNodeId, crate::placement::Frame>,
    /// Document-level named parameters.
    #[serde(with = "crate::persist::strict::params")]
    pub(crate) params: BTreeMap<ParamName, DocParam>,
    /// The recorded modeling tolerance ε (M4 PR 6 spec D4): new
    /// documents record the process's committed ambient ε; loading
    /// reconciles the recorded value against the process (one process
    /// = one ε); `SetTolerance` edits it.
    pub(crate) epsilon: f64,
    /// Per-node witness data (M4 PR 4, SOLVER-DESIGN W1/W4): the
    /// opaque branch-selection datum of each sketch-bearing node,
    /// written ONLY by the recorded `ReWitness` edits (and, at M6, by
    /// committed sketch edits). Document state under GQ3 — undo/redo
    /// and replay need no special cases.
    #[serde(with = "crate::persist::strict::witnesses")]
    pub(crate) witnesses: BTreeMap<RecipeNodeId, crate::witness::WitnessDatum>,
    /// Free-form document metadata (display units etc. — presentation
    /// only, GQ5). Empty in v1 (spec D2).
    #[serde(with = "crate::persist::strict::doc_metadata")]
    pub(crate) metadata: BTreeMap<String, String>,
    /// Appearance attributes keyed by stable name (M4 PR 7;
    /// DESIGN.md's ratified attachment contract). Presentation
    /// metadata: NEVER enters evaluation content keys — see
    /// [`crate::appearance`] for the loss (N3/N5) and wrapper (B11)
    /// semantics.
    #[serde(with = "crate::persist::pairs")]
    pub(crate) appearance: AppearanceMap,
}

impl<P> Doc<P> {
    /// The empty document under the given identity: no nodes, no
    /// params, recorded ε = the process's ambient tolerance (M4 PR 6
    /// spec D4: a new document adopts the process ε, so in-process
    /// documents NEVER disagree with the committed tolerance; the
    /// OnceLock bootstrap commits here on first touch if nothing
    /// committed earlier) — replay's origin (spec D7). The id is
    /// authored data (ASM-1 D-1): there is no id-less document and no
    /// ambient-randomness default.
    pub fn empty(id: DocumentId, tol: Tol) -> Self {
        Self {
            id,
            next_id: 0,
            nodes: BTreeMap::new(),
            order: Vec::new(),
            roots: Vec::new(),
            placements: BTreeMap::new(),
            params: BTreeMap::new(),
            epsilon: tol.eps(),
            witnesses: BTreeMap::new(),
            metadata: BTreeMap::new(),
            appearance: AppearanceMap::new(),
        }
    }

    /// The empty document under a label-derived identity —
    /// [`Self::empty`] ∘ [`DocumentId::derive`], the deterministic
    /// spelling corpus/demos/tests use.
    pub fn empty_derived(label: &str, tol: Tol) -> Self {
        Self::empty(DocumentId::derive(label), tol)
    }

    /// The document's stable identity.
    pub fn id(&self) -> DocumentId {
        self.id
    }

    /// The node with the given id, if live.
    pub fn node(&self, id: RecipeNodeId) -> Option<&Node<P>> {
        self.nodes.get(&id)
    }

    /// Live node ids in insertion order.
    pub fn order(&self) -> &[RecipeNodeId] {
        &self.order
    }

    /// The ordered product roots (A10): the gather order of the
    /// document's product solids.
    pub fn roots(&self) -> &[RecipeNodeId] {
        &self.roots
    }

    /// The placement frame of `node`'s CLUSTER (A11): the frame
    /// recorded against the cluster's gauge, or the identity when
    /// nothing is recorded — the missing entry IS the identity, so
    /// this is total.
    ///
    /// This is the CLUSTER's frame, which places its gauge; an
    /// instance's own world placement is this composed with its solved
    /// relative pose ([`crate::mate::SolvedPoses::placement`]). The
    /// two coincide exactly for a singleton cluster, which is every
    /// cluster in a mate-less document.
    pub fn placement(&self, node: RecipeNodeId) -> crate::placement::Frame {
        self.placements
            .get(&crate::mate::gauge_of(self, node))
            .copied()
            .unwrap_or(crate::placement::Frame::IDENTITY)
    }

    /// Replaces the whole placement registry — the ONE door A11's
    /// cluster-record maintenance writes through
    /// ([`crate::mate::solve::reconcile`]), so re-keying is a single
    /// observable act rather than a scatter of per-row edits.
    pub(crate) fn set_placements(&mut self, rows: BTreeMap<RecipeNodeId, crate::placement::Frame>) {
        self.placements = rows;
    }

    /// The recorded placement rows, in node order. Rows absent here
    /// are identity placements, not missing state.
    pub fn placements(&self) -> &BTreeMap<RecipeNodeId, crate::placement::Frame> {
        &self.placements
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

    /// One name's appearance record (attrs + D7 metadata), if any.
    pub fn appearance_of(&self, name: &StableName) -> Option<&AppearanceRecord> {
        self.appearance.get(name)
    }

    /// The expression subtree an [`ExprPath`] addresses, or `None` if
    /// the node is gone, the slot absent, or the path off the tree
    /// (spec D5).
    pub fn expr_at(&self, path: &ExprPath) -> Option<&Expr>
    where
        P: crate::ProfilePayload,
    {
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
                // The nominal alone crosses into evaluation: a
                // distribution is document metadata the scalar channel
                // never sees (E1).
                let v = match *p {
                    DocParam::Continuous { dim, value, .. } => ParamValue::Continuous {
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

impl<P: PartialEq + crate::ProfilePayload> Doc<P> {
    /// Bit-semantic document equality (spec D7's replay-identity
    /// comparator; M4 PR 1 review non-blocker): every float field —
    /// expression literals, continuous doc params, recorded ε —
    /// compares by BITS; ids, order, structure, metadata compare
    /// structurally. `PartialEq` on `Doc` remains IEEE-semantic
    /// (conflates `±0.0`); use THIS for replay pins and audits.
    pub fn bit_eq(&self, other: &Doc<P>) -> bool {
        self.id == other.id
            && self.next_id == other.next_id
            && self.order == other.order
            && self.roots == other.roots
            && self.epsilon.to_bits() == other.epsilon.to_bits()
            // Placement coordinates are floats: compare BY BITS, like
            // every other float field here.
            && self.placements.len() == other.placements.len()
            && self.placements.iter().all(|(id, frame)| {
                other
                    .placements
                    .get(id)
                    .is_some_and(|theirs| frame.bit_eq(theirs))
            })
            // Witness bytes are exact data (no float semantics to
            // conflate) — structural equality IS bit equality here.
            && self.witnesses == other.witnesses
            && self.metadata == other.metadata
            // Appearance attrs are float-free by construction
            // (integers/bools/strings), and D7 metadata floats compare
            // BY BITS through `MetaValue`'s own `PartialEq` — so
            // structural equality IS bit equality here.
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
