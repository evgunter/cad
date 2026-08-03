//! The **Band 4 model corpus** (M4 PR 8a spec D1) — the milestone's
//! proof-of-vocabulary.
//!
//! Every document here is a RECIPE authored through `DocEdit`s (an
//! edit LOG, never a hand-built `Doc` value), so each one is at once:
//!
//! - an evaluation fixture (green at every CI ε row and under the
//!   `interval` feature — `m4_pr8_corpus.rs` / `..._interval.rs`),
//! - a persistence fixture (the PR 6 D6.1 rows run the whole corpus:
//!   `m4_pr6_roundtrip.rs` / `..._interval.rs`),
//! - a latency fixture (D2's full-rebuild + incremental-recompute
//!   reporting row, `m4_pr8_latency.rs`).
//!
//! The corpus is a CRATE-LEVEL TEST ASSET, not demo code: nothing in
//! `demos/` may depend on it, and it carries no rendering, no CLI, no
//! narration.
//!
//! Coverage is asserted, not asserted-by-comment:
//! `m4_pr8_corpus.rs::vocabulary_coverage_is_total` walks the corpus
//! and requires EVERY F4 node kind and EVERY `DocEdit` kind to be
//! exercised somewhere in it (the [`Vocab`] tally below is derived
//! from the documents themselves, so a new node/edit kind fails the
//! test until the corpus covers it).
//!
//! Exact oracles: dimensions are dyadic wherever an exact mass pin is
//! claimed, so `volume`/`surface_area` are asserted with `==` and the
//! derivation is written out at the pin's definition site. Documents
//! whose geometry is not dyadic (arcs) carry no mass pin and are
//! pinned on validity + counts instead.

#![allow(dead_code)] // shared across test binaries; not all use all of it

use std::collections::{BTreeMap, BTreeSet};

use editor_core::{
    BooleanOp, BooleanValue, CancelToken, ContentBits, Datum, DocEdit, EvalOptions, Evaluation,
    Node, NodeResult, PatternKind, ProfileDesc, ProfileDoc, RecipeNodeId, ValuePayload, apply,
    evaluate,
};
use geom_core::Decide;
use topo::Body;

pub mod boss;
pub mod cut_cylinder;
pub mod die;
pub mod die_fillet;
pub mod die_pips;
pub mod heatsink;
pub mod islands;
pub mod sink;
pub mod slots;
pub mod table;
pub mod tangency;

pub use crate::fixture::Recorder;

/// An exact mass-property oracle (dyadic dimensions only — see each
/// document's derivation comment).
#[derive(Debug, Clone, Copy)]
pub struct MassPin {
    /// The exact volume.
    pub volume: f64,
    /// The exact surface area, where it is also dyadic.
    pub area: Option<f64>,
}

/// One corpus document.
pub struct CorpusDoc {
    /// Stable identifier — the latency table's row key and the
    /// baseline JSON's map key. NEVER rename without refreshing the
    /// committed baseline.
    pub name: &'static str,
    /// One-line description (printed in the latency table).
    pub about: &'static str,
    /// The recorded edit log (applied to the empty snapshot).
    pub edits: Vec<DocEdit<ProfileDesc>>,
    /// The replayed current state (empty snapshot + `edits`).
    pub doc: ProfileDoc,
    /// The node carrying the document's headline solid, if it has one
    /// (`None` for documents whose result is not a single body).
    pub result: Option<RecipeNodeId>,
    /// The exact mass oracle for `result`, where dyadic.
    pub pin: Option<MassPin>,
    /// D2's incremental-recompute probe: ONE parameter edit whose
    /// downstream cone is a PROPER subset of the document, so the run
    /// has something to reuse (asserted). It is a mid-DAG edit wherever
    /// the document has interior nodes; `declared_tangency` is two
    /// disjoint leaf chains and has none, so its bump edits a leaf and
    /// the reuse comes from the sibling chain.
    pub bump: DocEdit<ProfileDesc>,
    /// The node the bump edits — the root of the cone that must
    /// recompute (the test derives the cone independently from the
    /// DAG and asserts the counted reuse against it).
    pub bump_root: RecipeNodeId,
}

impl CorpusDoc {
    /// The document's node count.
    pub fn len(&self) -> usize {
        self.doc.len()
    }

    /// Whether the document is empty (never, in this corpus).
    pub fn is_empty(&self) -> bool {
        self.doc.is_empty()
    }

    /// The bumped document (the incremental-recompute probe's input).
    pub fn bumped(&self) -> ProfileDoc {
        apply(&self.doc, &self.bump)
            .expect("corpus bump edit must apply")
            .doc
    }
}

/// The corpus, in a stable order.
pub fn documents() -> Vec<CorpusDoc> {
    vec![
        die::document(),
        table::document(),
        heatsink::document(),
        slots::document(),
        islands::document_105(),
        islands::document_106_depth1(),
        islands::document_106_depth2(),
        tangency::document(),
        sink::document(),
        cut_cylinder::document(),
        boss::document(),
        // `die_fillet` IS registered (above), as of the PR 12 gate fix
        // `5c8540f`. It was held out while the fillet battery's
        // clearance screen seeded a gap with
        // `T::from_f64(f64::INFINITY)` — NaI at the Interval scalar,
        // so the document was green at `f64` and refused under
        // `--features interval`, which registry membership requires.
        // That sentinel is gone; the document runs the Interval lane
        // like every other row. It stays additionally pinned at both
        // scalars by `m5_pr12_fillet_node.rs`.
        die_pips::document(),
    ]
}

/// The transitive downstream cone of `root` (inclusive) over the
/// recipe DAG's input edges — computed independently of the
/// evaluator, so the counted-reuse assertions have a real oracle.
pub fn cone(doc: &ProfileDoc, root: RecipeNodeId) -> BTreeSet<RecipeNodeId> {
    let mut set = BTreeSet::new();
    set.insert(root);
    // `order` is insertion order and inputs must pre-exist, so one
    // forward sweep suffices.
    for &id in doc.order() {
        let node = doc.node(id).expect("ordered node exists");
        if node.inputs().iter().any(|i| set.contains(i)) {
            set.insert(id);
        }
    }
    set
}

/// Evaluates a document at scalar `T` with default options.
pub fn eval<T: Decide + ContentBits + geom_core::Bounds + Send + Sync>(
    doc: &ProfileDoc,
) -> Evaluation<T> {
    evaluate::<T>(doc, None, &CancelToken::new(), &EvalOptions::default())
}

/// The per-node failure report of an evaluation (empty when green).
pub fn failures<T: Decide>(ev: &Evaluation<T>) -> Vec<String> {
    ev.nodes
        .iter()
        .filter_map(|(id, r)| match r {
            NodeResult::Ok(_) => None,
            NodeResult::Failed(e) => Some(format!("{id:?} FAILED: {e:?}")),
            NodeResult::Poisoned { through } => {
                Some(format!("{id:?} poisoned through {through:?}"))
            }
        })
        .collect()
}

/// The single body a node evaluated to (`Body` payload or a boolean's
/// nonempty body).
pub fn body_of<T: Decide>(ev: &Evaluation<T>, id: RecipeNodeId) -> &Body<T> {
    match &ev.value(id).expect("node evaluated to a value").payload {
        ValuePayload::Body(b) => b,
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => body,
        other => panic!("expected a single body, got {}", other.kind_name()),
    }
}

/// The node kinds a document exercises (the coverage tally's domain).
pub const NODE_KINDS: [&str; 12] = [
    "Datum",
    "Profile",
    "Extrude",
    "Revolve",
    // M5 PR 12's constant-radius rolling-ball fillet — COVERED, by
    // the registered `die_fillet` document (the Interval-scalar
    // blocker that once held it out of the registry is gone; see
    // `documents()`). Not an exemption: `Loft`/`Sweep` below still
    // are.
    "Fillet",
    "Split",
    "Boolean",
    "Transform",
    "Pattern",
    // M5 PR 10's definitional feature nodes. They join the tally's
    // DOMAIN here; the Band 4 corpus row that exercises them waits on
    // the NURBS-walled-solid frontier (see
    // `NodeErrorKind::CurvedSolidFrontier`), so the coverage report
    // shows them at zero rather than pretending they are covered.
    "Loft",
    "Sweep",
    "Declare",
];

/// The edit kinds a document exercises (the coverage tally's domain).
pub const EDIT_KINDS: [&str; 14] = [
    "InsertNode",
    "DeleteNode",
    "SetParam",
    "SetStructuralParam",
    "SetExpression",
    "SetDocParam",
    "Rebind",
    "ReWitness",
    "ReWitnessBulk",
    "SetAppearance",
    "ClearAppearance",
    "SetTolerance",
    "SetAppearanceMeta",
    "ClearAppearanceMeta",
];

/// The node SUB-kinds the corpus must also cover in full: every datum
/// flavour, every boolean operator (and the declared boolean), and
/// both pattern kinds.
pub const SUB_KINDS: [&str; 9] = [
    "Datum::Plane",
    "Datum::Axis",
    "Datum::Point",
    "Boolean::Union",
    "Boolean::Intersect",
    "Boolean::Subtract",
    "Boolean+Declare",
    "Pattern::Linear",
    "Pattern::Circular",
];

/// The sub-kind tally names a node contributes (possibly none).
pub fn sub_kinds(node: &Node<ProfileDesc>) -> Vec<&'static str> {
    match node {
        Node::Datum(Datum::Plane { .. }) => vec!["Datum::Plane"],
        Node::Datum(Datum::Axis { .. }) => vec!["Datum::Axis"],
        Node::Datum(Datum::Point { .. }) => vec!["Datum::Point"],
        Node::Boolean { op, declare, .. } => {
            let mut v = vec![match op {
                BooleanOp::Union => "Boolean::Union",
                BooleanOp::Intersect => "Boolean::Intersect",
                BooleanOp::Subtract => "Boolean::Subtract",
            }];
            if declare.is_some() {
                v.push("Boolean+Declare");
            }
            v
        }
        Node::Pattern { kind, .. } => vec![match kind {
            PatternKind::Linear { .. } => "Pattern::Linear",
            PatternKind::Circular { .. } => "Pattern::Circular",
        }],
        // EXHAUSTIVE on purpose (review MIN-2): no wildcard arm, so a
        // new `Node` variant — or a new `Datum`/`BooleanOp`/
        // `PatternKind` flavour above — is a COMPILE error here rather
        // than a silently uncovered sub-kind, matching the
        // compile-time totality `node_kind`/`edit_kind` already have.
        Node::Profile(_)
        | Node::Extrude { .. }
        | Node::Revolve { .. }
        | Node::Fillet { .. }
        | Node::Split { .. }
        | Node::Transform { .. }
        | Node::Loft { .. }
        | Node::Sweep { .. }
        | Node::Declare { .. } => Vec::new(),
    }
}

/// The node kind's tally name.
pub fn node_kind(node: &Node<ProfileDesc>) -> &'static str {
    match node {
        Node::Datum(_) => "Datum",
        Node::Profile(_) => "Profile",
        Node::Extrude { .. } => "Extrude",
        Node::Revolve { .. } => "Revolve",
        Node::Fillet { .. } => "Fillet",
        Node::Split { .. } => "Split",
        Node::Boolean { .. } => "Boolean",
        Node::Transform { .. } => "Transform",
        Node::Pattern { .. } => "Pattern",
        Node::Loft { .. } => "Loft",
        Node::Sweep { .. } => "Sweep",
        Node::Declare { .. } => "Declare",
    }
}

/// The edit kind's tally name.
pub fn edit_kind(edit: &DocEdit<ProfileDesc>) -> &'static str {
    match edit {
        DocEdit::InsertNode { .. } => "InsertNode",
        DocEdit::DeleteNode { .. } => "DeleteNode",
        DocEdit::SetParam { .. } => "SetParam",
        DocEdit::SetStructuralParam { .. } => "SetStructuralParam",
        DocEdit::SetExpression { .. } => "SetExpression",
        DocEdit::SetDocParam { .. } => "SetDocParam",
        DocEdit::Rebind { .. } => "Rebind",
        DocEdit::ReWitness { .. } => "ReWitness",
        DocEdit::ReWitnessBulk { .. } => "ReWitnessBulk",
        DocEdit::SetAppearance { .. } => "SetAppearance",
        DocEdit::ClearAppearance { .. } => "ClearAppearance",
        DocEdit::SetTolerance { .. } => "SetTolerance",
        DocEdit::SetAppearanceMeta { .. } => "SetAppearanceMeta",
        DocEdit::ClearAppearanceMeta { .. } => "ClearAppearanceMeta",
    }
}

/// A kind-name -> covering-document-names tally.
pub type Tally = BTreeMap<&'static str, Vec<&'static str>>;

/// The corpus-wide vocabulary tallies (node kinds, node sub-kinds,
/// edit kinds), derived from the documents themselves.
///
/// Node kinds come from the INSERTED nodes (the edit logs) as well as
/// the replayed DAG, so a node the corpus inserts and later deletes —
/// the `DeleteNode` arm needs one — still counts as exercised.
pub fn vocabulary() -> (Tally, Tally, Tally) {
    let mut nodes: Tally = BTreeMap::new();
    let mut subs: Tally = BTreeMap::new();
    let mut edits: Tally = BTreeMap::new();
    for d in documents() {
        let mut seen_n = BTreeSet::new();
        let mut seen_s = BTreeSet::new();
        let note = |n: &Node<ProfileDesc>,
                    seen_n: &mut BTreeSet<&'static str>,
                    seen_s: &mut BTreeSet<&'static str>| {
            seen_n.insert(node_kind(n));
            seen_s.extend(sub_kinds(n));
        };
        for &id in d.doc.order() {
            note(
                d.doc.node(id).expect("ordered node exists"),
                &mut seen_n,
                &mut seen_s,
            );
        }
        let mut seen_e = BTreeSet::new();
        for e in d.edits.iter().chain(std::iter::once(&d.bump)) {
            seen_e.insert(edit_kind(e));
            if let DocEdit::InsertNode { node } = e {
                note(node, &mut seen_n, &mut seen_s);
            }
        }
        for k in seen_n {
            nodes.entry(k).or_default().push(d.name);
        }
        for k in seen_s {
            subs.entry(k).or_default().push(d.name);
        }
        for k in seen_e {
            edits.entry(k).or_default().push(d.name);
        }
    }
    (nodes, subs, edits)
}
