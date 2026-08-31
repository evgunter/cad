//! **Fillet and chamfer authoring**: the modal layer-3 tool that turns
//! a SET of edge picks into exactly one committed blend edit.
//!
//! # Why this tool is not seated
//!
//! [`crate::seats`] is two role-typed seats filled in order, and every
//! tool built on it holds a fixed, small number of picks that mean
//! different things. A blend holds one body and an OPEN-ENDED set of
//! that body's edges, all meaning the same thing — so the seat value's
//! whole rule (fill the first empty, replace the last, drop a seat on
//! its own) says nothing about it. What it shares with the seated
//! tools is the shape rather than the state: single-select stays
//! ruled, the picks live in tool state, everything before the commit
//! is transient, and the document transition is one [`SessionOp`]
//! committing one `DocEdit::InsertNode`.
//!
//! # One target, and why the rule is structural
//!
//! [`crate::session::SessionOp::AddFillet`] and its chamfer twin carry
//! ONE target — `Node::Fillet { target, .. }` blends edges of one
//! body, and a selection resolves through that body's name table and
//! no other. So the accumulator holds one [`BlendTarget`] and a set of
//! names under it: the first pick fixes the target, and a pick on
//! another drawn body has nowhere to land. It is refused as a typed
//! event ([`BlendEvent::OtherTarget`]) rather than silently ignored,
//! and rather than being allowed to clear the picks the user already
//! made — a mis-aimed click must not cost eleven good ones.
//!
//! The alternative rule (a cross-target pick STARTS OVER on the new
//! body) was not taken: a click that discards held picks with no
//! confirmation is the more expensive mistake of the two, and Cancel
//! is the door that means "drop these".
//!
//! # The selection freezes (#217), and this tool is where it starts
//!
//! `Node::Fillet`'s ratified semantics: the stored set is a
//! commitment, an upstream edit that adds an edge does NOT extend it,
//! and an upstream edit that strands one is a typed refusal on the
//! node's badge rather than a silent shrink. That is a property of the
//! COMMITTED node, so the tool's job is to say so before the commit
//! ([`FREEZE_NOTE`], which the panel shows) and to hand the
//! canonicalizing constructor a set the user actually chose.
//!
//! # What the survival step does, and why it is not a #217 breach
//!
//! **#217 governs the COMMITTED node, not the accumulator.** A stored
//! selection must never shrink silently, because it is a commitment
//! the document records and the user can no longer see being edited.
//! Tool state is the opposite: it is what the user is looking at right
//! now, and the panel is showing a count. So a held name whose edge no
//! longer exists must not be carried on as if it did — committing it
//! would author a node that refuses on arrival, which is the worst of
//! both rules.
//!
//! [`BlendTool::reconcile`] therefore re-reads the held names against
//! the target's CURRENT edge names on every document change and drops
//! the strands, loudly ([`BlendEvent::EdgesLost`]), alongside the
//! whole-set drop when the target node itself goes
//! ([`BlendEvent::TargetLost`]). Nothing here is silent: every drop is
//! a typed event the chrome shows, and the count the panel reads is
//! the count a commit would author.

use std::collections::BTreeSet;

use pncad::document::{Doc, Evaluation, ProfileProgram, RecipeNodeId};
use pncad::prelude::StableName;

use crate::session::{EdgeSelection, Selection, SessionOp};

/// **What the tool's panel says about the freeze**, so the ratified
/// #217 semantics reach the user at the moment they are committing to
/// a set rather than only in the node's docs.
pub const FREEZE_NOTE: &str = "the picked edges freeze at commit: an upstream edit that adds an edge does not extend this \
     blend, and one that removes a picked edge refuses on the node rather than shrinking it \
     (only a deliberate rebind rewrites a stored selection)";

/// The drawn body a blend's edges belong to: the node whose value it
/// is, and which of that node's output bodies.
///
/// The NODE is what `Node::Fillet` stores as its target and what the
/// selection's names resolve through. The BODY index rides along
/// because an edge pick carries one and a set drawn from two bodies of
/// one node would be as wrong as a set drawn from two nodes — the
/// accumulator scopes to the pair it was opened on, and the refusal
/// names the pair.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlendTarget {
    /// The node whose body carries the edges.
    pub node: RecipeNodeId,
    /// The output body index within that node's value.
    pub body: u32,
}

impl BlendTarget {
    /// The target an edge pick is about.
    pub fn of(edge: &EdgeSelection) -> Self {
        Self {
            node: edge.node,
            body: edge.body,
        }
    }

    /// **The drawn body a selection is a pick on**, when it is one.
    ///
    /// A face pick and an edge pick both carry `(node, body)` — they
    /// name something DRAWN, which is the only kind of selection that
    /// can say which body. `Selection::Node` is a feature picked in
    /// the tree and names no body at all; answering `body: 0` for it
    /// would be a guess, and precisely on the multi-body nodes where
    /// the narrowing matters it would be the wrong one. So it answers
    /// `None` and the affordance that needs a body says what it wants.
    pub fn of_selection(selection: &Selection) -> Option<Self> {
        match selection {
            Selection::Edge(edge) => Some(Self::of(edge)),
            Selection::Face(face) => Some(Self {
                node: face.node,
                body: face.body,
            }),
            Selection::None | Selection::Node(_) | Selection::Param(_) => None,
        }
    }
}

impl core::fmt::Display for BlendTarget {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "feature {} body {}", self.node.0, self.body)
    }
}

/// Which blend is being authored — the tool's kind choice, and the
/// discrimination that picks which commit door the panel calls.
///
/// Two variants rather than a flag on one op, because the two nodes
/// are two nodes: a fillet's size is a rolling-ball RADIUS and a
/// chamfer's is a SETBACK, they live in different slots, and
/// `Node::Chamfer`'s docs give the argument for why a recipe must not
/// have a boolean deciding which one a number means.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BlendKindChoice {
    /// A constant-radius rolling-ball fillet (`Node::Fillet`).
    #[default]
    Fillet,
    /// An equal-setback flat chamfer (`Node::Chamfer`).
    Chamfer,
}

impl BlendKindChoice {
    /// Both kinds with their button labels — the chrome's radio row
    /// and a test that sweeps them.
    pub const ALL: [(Self, &'static str); 2] =
        [(Self::Fillet, "fillet"), (Self::Chamfer, "chamfer")];

    /// What the one Length field means for this kind, for the field's
    /// own label.
    pub fn size_label(self) -> &'static str {
        match self {
            Self::Fillet => "radius (m)",
            Self::Chamfer => "setback (m)",
        }
    }
}

/// A typed blend-tool refusal at a commit door (closed enum, D4 ¶3).
#[derive(Debug, PartialEq, Eq)]
pub enum BlendError {
    /// No edges are held, so there is nothing to blend.
    ///
    /// The commit door refuses here rather than emitting an op with an
    /// empty set, for the reason a seated tool refuses an empty seat:
    /// a button that authors an unfinished node is a button that costs
    /// an undo. The DOCUMENT-level rule is unchanged and lives where
    /// it always did — `NodeErrorKind::BlendSelectionEmpty` refuses an
    /// empty selection at evaluation, so a hand-written recipe gets
    /// the same answer as an authored one.
    NoEdges,
}

impl core::fmt::Display for BlendError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NoEdges => f.write_str("no edges picked yet"),
        }
    }
}

impl core::error::Error for BlendError {}

/// A typed tool event the chrome renders — every state change, or
/// refused change, that was not the direct echo of a pick the tool
/// took.
#[derive(Debug, PartialEq, Eq)]
pub enum BlendEvent {
    /// A pick landed on a body other than the one the held edges
    /// belong to. The pick is refused and the held set is untouched
    /// (module docs: one target, and why a mis-aim does not cost the
    /// picks).
    OtherTarget {
        /// The body the held edges are on.
        held: BlendTarget,
        /// The body the refused pick was on.
        picked: BlendTarget,
    },
    /// The all-edges door found no edges on the target, so nothing was
    /// loaded and the held set is untouched. A node with no value, no
    /// name table, or no edges answers the same way — the door cannot
    /// tell them apart and does not pretend to.
    NoEdgesOnTarget {
        /// The body that was asked.
        target: BlendTarget,
    },
    /// The target node is no longer in the document, so every held
    /// edge is about a body that is gone: the whole set is dropped at
    /// once.
    TargetLost {
        /// The body the set was about.
        target: BlendTarget,
        /// How many edges went with it.
        edges: usize,
    },
    /// The target survived an upstream edit but some held edges did
    /// not — the strand case, dropped loudly rather than carried to a
    /// commit that would refuse on arrival (module docs: why this is
    /// not the silent shrink #217 forbids).
    EdgesLost {
        /// The body the set is about.
        target: BlendTarget,
        /// The names that stopped being edges of it, in canonical
        /// order — carried rather than counted, so a chrome that wants
        /// to say WHICH can, and a row can assert on them.
        names: Vec<StableName>,
        /// How many are still held.
        kept: usize,
    },
}

impl core::fmt::Display for BlendEvent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OtherTarget { held, picked } => write!(
                f,
                "the held edges are on {held}, so the edge on {picked} was not taken; \
                 cancel to start on another body"
            ),
            Self::NoEdgesOnTarget { target } => {
                write!(f, "{target} has no edges to select")
            }
            Self::TargetLost { target, edges } => write!(
                f,
                "{target} is no longer in the document; the tool dropped all {edges} picked edges"
            ),
            Self::EdgesLost {
                target,
                names,
                kept,
            } => write!(
                f,
                "an edit removed {} of the picked edges from {target}; the tool dropped them and \
                 still holds {kept}",
                names.len()
            ),
        }
    }
}

/// **The blend tool**: one target body, an accumulating set of its
/// edges, and one committed blend edit.
///
/// The set is a `BTreeSet` of stable names, which makes three of this
/// tool's rules structural rather than checked: a name cannot be held
/// twice, the live count is the set's size, and what
/// [`BlendTool::selection`] hands the canonicalizing constructor is
/// already in canonical order. Picking a held edge again REMOVES it —
/// the per-pick add/remove the plan asks for, and the only correction
/// a set-valued pick needs.
///
/// # The invariant the two fields keep together
///
/// **A target is held exactly while an edge is** — `target.is_some()`
/// iff `!edges.is_empty()`, maintained by every door that writes
/// either. A tool whose last edge was un-picked releases its target
/// and is indistinguishable from a fresh one, so the next click may
/// start on any body; without that, un-picking down to zero left the
/// tool latched to a body it was no longer holding anything on, and
/// the cross-target refusal would have fired on a set of nothing.
/// [`BlendTool::require_target`] reads the invariant rather than
/// re-checking both halves, which is what makes
/// [`BlendError::NoEdges`]'s one sentence true of the one state.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BlendTool {
    target: Option<BlendTarget>,
    edges: BTreeSet<StableName>,
}

impl BlendTool {
    /// A tool holding nothing.
    pub fn new() -> Self {
        Self::default()
    }

    /// The body the held edges are on, once a pick has fixed it.
    pub fn target(&self) -> Option<BlendTarget> {
        self.target
    }

    /// How many edges are held — the panel's live count.
    pub fn count(&self) -> usize {
        self.edges.len()
    }

    /// Whether this edge is one of the held ones.
    pub fn holds(&self, name: &StableName) -> bool {
        self.edges.contains(name)
    }

    /// The held set in canonical order — what a commit hands
    /// `Node::fillet` / `Node::chamfer`, which canonicalize again
    /// because a recipe's bits must not depend on who built it.
    pub fn selection(&self) -> Vec<StableName> {
        self.edges.iter().cloned().collect()
    }

    /// **The held edges as pick selections** — which edges are held,
    /// as the value vocabulary the rest of the crate speaks. Each is a
    /// held name on the tool's own target, so a consumer that resolves
    /// one gets the same (node, body) narrowing a single selection
    /// gets.
    ///
    /// This is the value claim; [`BlendTool::mark_segments`] is the
    /// per-frame path, and `a_held_set_marks_exactly_the_edges_it_names`
    /// asserts the two agree so they cannot drift.
    pub fn marks(&self) -> Vec<EdgeSelection> {
        let Some(target) = self.target else {
            return Vec::new();
        };
        self.edges
            .iter()
            .map(|name| EdgeSelection {
                name: name.clone(),
                node: target.node,
                body: target.body,
            })
            .collect()
    }

    /// **The drawn segments of the whole held set**, as the line-list
    /// pairs a renderer consumes — what the viewport marks while this
    /// tool is open.
    ///
    /// **One pass over the target's drawn edges**, testing set
    /// membership per drawn edge, rather than one
    /// `crate::pick::edge_segments` search per held name: the search
    /// scans the body's whole edge run for each name, so the obvious
    /// spelling costs `O(E²)` name comparisons every frame on a body
    /// with `E` edges — fine for a cube, not for a real part. This is
    /// `O(E log E)` and allocates nothing but the output.
    ///
    /// The narrowing is exactly what a single selection gets: scoped
    /// to the tool's own (node, body), empty for a target this index
    /// does not draw, and silent about a held name the index has no
    /// edge for — so the picture shows the live members and nothing
    /// else.
    pub fn mark_segments(
        &self,
        index: &crate::pick::PickIndex,
        display: &crate::display::DisplayView,
    ) -> Vec<[f32; 3]> {
        let Some(target) = self.target else {
            return Vec::new();
        };
        let mut out = Vec::new();
        for &id in index.edges_in(target.node, target.body) {
            if index
                .edge_name_of(id)
                .is_ok_and(|name| self.edges.contains(name))
            {
                out.extend(crate::pick::edge_id_segments(index, display, id));
            }
        }
        out
    }

    /// **Feed one edge pick**: add it, or REMOVE it when it is already
    /// held.
    ///
    /// The first pick fixes the target; un-picking the last edge
    /// releases it again (the struct's invariant). A pick on another
    /// body is refused ([`BlendEvent::OtherTarget`]) and changes
    /// nothing.
    pub fn pick(&mut self, edge: &EdgeSelection) -> Option<BlendEvent> {
        let picked = BlendTarget::of(edge);
        match self.target {
            Some(held) if held != picked => {
                return Some(BlendEvent::OtherTarget { held, picked });
            }
            Some(_) => {}
            None => self.target = Some(picked),
        }
        if !self.edges.remove(&edge.name) {
            self.edges.insert(edge.name.clone());
        }
        self.release_if_empty();
        None
    }

    /// **The all-edges affordance**: load every edge of `target` as it
    /// stands in this evaluation, as an ordinary frozen set.
    ///
    /// The door is `editor_core`'s [`pncad::select::all_edges`], which
    /// exists precisely so that "all edges" is materialized once and
    /// STORED rather than left as a live query — `Node::Fillet` has no
    /// every-edge variant on purpose. What lands here is therefore
    /// indistinguishable from twelve clicks, which is the point.
    ///
    /// It REPLACES whatever was held, target included: "all edges of
    /// this body" is an answer about one body, and merging it into
    /// picks from another would be the cross-target rule broken from
    /// the inside. A target with no edges loads nothing and says so,
    /// rather than emptying the set on the way to a refusal.
    ///
    /// # The door answers per NODE, so the answer is NARROWED here
    ///
    /// `all_edges` reads one node's name table, and a node whose value
    /// is several bodies has one table covering all of them: asked
    /// about a split it answers both halves, about a pattern every
    /// instance. Loading that unnarrowed broke this tool's own
    /// one-target rule from the inside — the panel counted 24 edges
    /// where the picture drew 12, and no `denotes_body` gate applies
    /// here because that gate is the COMMIT door's.
    ///
    /// So the door's answer is intersected with the names the index
    /// draws for `(node, body)` — the same narrowing a single
    /// selection and [`BlendTool::mark_segments`] already apply — and
    /// the count the panel shows is the set the picture marks, on
    /// every target rather than only on the ones a commit would
    /// accept. This is not a display-tolerance dependency: the mesh
    /// carries one boundary polyline per topological edge, so δ
    /// changes how many POINTS each drawn edge has and never how many
    /// there are.
    ///
    /// # Loading a set is not a promise that it builds
    ///
    /// The kernel's blend assembly admits a fully-requested chain set
    /// and refuses the rest by name — mixed convexity, tangential
    /// runs, and a blend of a blend are all typed refusals on the
    /// node's own badge. This door hands over the edges that EXIST,
    /// which is a different claim, and the panel says so beside the
    /// button rather than implying every loaded set is buildable.
    pub fn load_all_edges(
        &mut self,
        target: BlendTarget,
        eval: &Evaluation<f64>,
        index: &crate::pick::PickIndex,
    ) -> Option<BlendEvent> {
        let named: BTreeSet<StableName> = pncad::select::all_edges(eval, target.node)
            .into_iter()
            .collect();
        let edges: BTreeSet<StableName> = index
            .edges_in(target.node, target.body)
            .iter()
            .filter_map(|&id| index.edge_name_of(id).ok())
            .filter(|name| named.contains(*name))
            .cloned()
            .collect();
        if edges.is_empty() {
            return Some(BlendEvent::NoEdgesOnTarget { target });
        }
        self.target = Some(target);
        self.edges = edges;
        None
    }

    /// Drop every pick — the panel's `Clear picks` button, and what
    /// Cancel's whole-tool replacement amounts to for the picks alone.
    pub fn clear(&mut self) {
        self.target = None;
        self.edges.clear();
    }

    /// Release the target when the last edge leaves, keeping the
    /// struct's bilateral invariant in the one place both writers
    /// reach.
    fn release_if_empty(&mut self) {
        if self.edges.is_empty() {
            self.target = None;
        }
    }

    /// **The survival step**, once per frame — and it is TWO
    /// questions, because a held set can be wrong in two ways.
    ///
    /// 1. The target node left the document: every held edge is about
    ///    a body that is gone, so the whole set goes at once
    ///    ([`BlendEvent::TargetLost`]). Answered from the document,
    ///    which is the only thing that can be asked with nothing
    ///    landed.
    /// 2. The target survived an upstream edit that changed which
    ///    edges it has — moving a boolean's operand, re-sizing a
    ///    profile — and some held names are no longer edges of it.
    ///    Those are dropped and named ([`BlendEvent::EdgesLost`]).
    ///    Answered from the LANDED evaluation, because "which edges
    ///    does this body have" is an evaluation's question.
    ///
    /// Without (2) the panel went on counting edges that no longer
    /// existed and the commit authored a node that refused on arrival
    /// — the freeze rule inverted, since #217 governs the stored
    /// selection and not the accumulator (module docs).
    ///
    /// **With nothing landed, (2) is not asked**, which is the honest
    /// answer: "we cannot tell" is not "it is gone", the same rule the
    /// mate tool's survival step takes.
    ///
    /// The membership test is the target NODE's edge names
    /// (`all_edges`), not the drawn body's: a name that survived on
    /// another body of a multi-body node is therefore not flagged
    /// here. That gap cannot reach a document — the commit door admits
    /// only single-body targets (`crate::combine::denotes_body`) — and
    /// closing it would need the pick index, which the survival step
    /// runs before this frame's is built.
    ///
    /// The id-reuse hazard it does not cover is the one
    /// [`crate::seats`] states for every tool holding a
    /// `RecipeNodeId` across a history rewind (issue #1384) — not
    /// restated here, and no narrower for holding a set.
    pub fn reconcile(
        &mut self,
        doc: &Doc<ProfileProgram>,
        landed: Option<(&Doc<ProfileProgram>, &Evaluation<f64>)>,
    ) -> Vec<BlendEvent> {
        let Some(target) = self.target else {
            return Vec::new();
        };
        if doc.node(target.node).is_none() {
            let edges = self.edges.len();
            self.clear();
            return vec![BlendEvent::TargetLost { target, edges }];
        }
        let Some((_, eval)) = landed else {
            return Vec::new();
        };
        let live: BTreeSet<StableName> = pncad::select::all_edges(eval, target.node)
            .into_iter()
            .collect();
        // A target that has no edges AT ALL in this evaluation is a
        // node that failed or was never run, not a body that lost
        // every edge: dropping the whole set on a transient failure
        // would cost the picks the moment an upstream slot went
        // momentarily bad. Say nothing and wait for a run that has an
        // answer.
        if live.is_empty() {
            return Vec::new();
        }
        let names: Vec<StableName> = self
            .edges
            .iter()
            .filter(|name| !live.contains(*name))
            .cloned()
            .collect();
        if names.is_empty() {
            return Vec::new();
        }
        for name in &names {
            self.edges.remove(name);
        }
        self.release_if_empty();
        vec![BlendEvent::EdgesLost {
            target,
            names,
            kept: self.edges.len(),
        }]
    }

    /// **The one committed edit, as a fillet**: the session op that
    /// inserts `Node::Fillet` through the ordinary commit door.
    ///
    /// # Errors
    ///
    /// [`BlendError::NoEdges`] until an edge is held. The target's
    /// kind refuses at the session door, and everything about the
    /// selection's meaning — a stranded name, a mis-kinded one, a
    /// radius the geometry cannot take — refuses typed at evaluation
    /// on the node's own badge.
    pub fn fillet_op(&self, radius: f64) -> Result<SessionOp, BlendError> {
        Ok(SessionOp::AddFillet {
            target: self.require_target()?,
            radius,
            selection: self.selection(),
        })
    }

    /// **The one committed edit, as a chamfer**: [`BlendTool::fillet_op`]'s
    /// twin, and the size means a setback along both supports rather
    /// than a radius.
    ///
    /// # Errors
    ///
    /// As [`BlendTool::fillet_op`].
    pub fn chamfer_op(&self, distance: f64) -> Result<SessionOp, BlendError> {
        Ok(SessionOp::AddChamfer {
            target: self.require_target()?,
            distance,
            selection: self.selection(),
        })
    }

    /// The target a commit needs, or the typed "nothing picked yet".
    ///
    /// One check for both doors, and one check for both halves: the
    /// struct's invariant makes "no target" and "no edges" the same
    /// state, so reading the target alone is reading both, and
    /// [`BlendError::NoEdges`]'s single sentence is true of it.
    fn require_target(&self) -> Result<RecipeNodeId, BlendError> {
        debug_assert_eq!(
            self.target.is_some(),
            !self.edges.is_empty(),
            "a target is held exactly while an edge is"
        );
        match self.target {
            Some(target) => Ok(target.node),
            None => Err(BlendError::NoEdges),
        }
    }
}
