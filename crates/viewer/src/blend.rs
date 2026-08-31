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
//! It is also why the survival step here is all-or-nothing. A seated
//! tool drops the one pick whose node left the document; this one
//! cannot, because a set that quietly shrank while the panel showed a
//! count would be exactly the silent shrink #217 forbids, one layer
//! early. The only thing that can happen to the set is the loss of the
//! body it is about, and then the whole set goes at once and says so
//! ([`BlendEvent::TargetLost`]).

use std::collections::BTreeSet;

use pncad::document::{Doc, Evaluation, ProfileProgram, RecipeNodeId};
use pncad::prelude::StableName;

use crate::session::{EdgeSelection, SessionOp};

/// **What the tool's panel says about the freeze**, so the ratified
/// #217 semantics reach the user at the moment they are committing to
/// a set rather than only in the node's docs.
pub const FREEZE_NOTE: &str =
    "the picked edges freeze at commit: a later edit that adds an edge does not extend this \
     blend, and one that removes a picked edge refuses on the node rather than shrinking it";

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
    pub const ALL: [(Self, &'static str); 2] = [(Self::Fillet, "fillet"), (Self::Chamfer, "chamfer")];

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
    /// once (module docs: why this is all-or-nothing).
    TargetLost {
        /// The body the set was about.
        target: BlendTarget,
        /// How many edges went with it.
        edges: usize,
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

    /// **The held edges as pick selections**, for the marks the
    /// picture draws: each is the held name on the tool's own target,
    /// which is the (node, body) narrowing `crate::pick::edge_segments`
    /// applies — so an edge whose name this evaluation does not draw
    /// there marks nothing, and the set marks exactly its live
    /// members.
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

    /// **Feed one edge pick**: add it, or REMOVE it when it is already
    /// held.
    ///
    /// The first pick fixes the target. A pick on another body is
    /// refused ([`BlendEvent::OtherTarget`]) and changes nothing.
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
    pub fn load_all_edges(
        &mut self,
        target: BlendTarget,
        eval: &Evaluation<f64>,
    ) -> Option<BlendEvent> {
        let edges: BTreeSet<StableName> =
            pncad::select::all_edges(eval, target.node).into_iter().collect();
        if edges.is_empty() {
            return Some(BlendEvent::NoEdgesOnTarget { target });
        }
        self.target = Some(target);
        self.edges = edges;
        None
    }

    /// Drop every pick — the chrome's "start over" door, and Cancel's
    /// effect by another route.
    pub fn clear(&mut self) {
        self.target = None;
        self.edges.clear();
    }

    /// **The survival step**, once per frame (module docs: why it is
    /// all-or-nothing). The target node leaving the document voids the
    /// whole set; nothing else here drops anything.
    pub fn reconcile(&mut self, doc: &Doc<ProfileProgram>) -> Option<BlendEvent> {
        let target = self.target?;
        if doc.node(target.node).is_some() {
            return None;
        }
        let edges = self.edges.len();
        self.clear();
        Some(BlendEvent::TargetLost { target, edges })
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
    /// One check for both doors: the target is `Some` exactly when a
    /// pick landed, and a pick that landed put a name in the set, so
    /// "no target" and "no edges" are the same state and get the one
    /// sentence.
    fn require_target(&self) -> Result<RecipeNodeId, BlendError> {
        match self.target {
            Some(target) if !self.edges.is_empty() => Ok(target.node),
            _ => Err(BlendError::NoEdges),
        }
    }
}
