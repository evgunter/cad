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
//! ruled (GQ7), the picks live in tool state, everything before the commit
//! is transient, and the document transition is one [`SessionOp`]
//! committing one `DocEdit::InsertNode`.
//!
//! # One target, and why the rule is structural
//!
//! [`crate::session::SessionOp::AddFillet`] and its chamfer twin carry
//! ONE target — a fillet reads one `Edges` selection, which states its
//! body once and resolves through that body's name table and no other. So the accumulator holds one [`BlendTarget`] and a set of
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
//! the picks a commit would author: a drawn circle the body holds as
//! several arcs is one pick and commits every arc
//! ([`BlendTool`], a drawn circle is one pick).
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use std::collections::BTreeSet;

use pncad::document::{
    Doc, Evaluation, Formula, HeldNodes, NodeStanding, ProfileProgram, RecipeNodeId, Said, Say,
    Speaker, SpokenNode,
};
use pncad::prelude::StableName;

use editor_core::resolve::RimNameFault;

use crate::pickcache::NotIndexed;
use crate::pickindex::{EdgeNamesRefused, PickIndex};
use crate::session::{EdgeSelection, FaceSelection, Selection, SessionOp};
use crate::tree::{DrawnBody, PickedName};
use crate::vocab::vocabulary;

/// **What the tool's panel says about the freeze**, so the ratified
/// #217 semantics reach the user at the moment they are committing to
/// a set rather than only in the node's docs.
pub const FREEZE_NOTE: &str = "the picked edges freeze at commit: an upstream edit that adds an edge does not extend this \
     blend, and one that removes a picked edge refuses on the node rather than shrinking it \
     (only a deliberate rebind rewrites a stored selection)";

/// The drawn body a blend's edges belong to: the node whose value it
/// is, and which of that node's output bodies.
///
/// The NODE is the one whose output the fillet's selection reads, and
/// what the selection's names resolve through. The BODY index rides along
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
            Selection::Face(FaceSelection { node, body, .. }) => Some(Self {
                node: *node,
                body: *body,
            }),
            Selection::None | Selection::Node(_) | Selection::Variable(_) => None,
        }
    }
}

/// **The one sentence for this scope is the drawn body's**
/// ([`DrawnBody`]), which a pick's `, on …` says too, so a blend's
/// refusal and a pick's sentence name one body alike. Destructured
/// rather than field-read, so a field added to [`BlendTarget`] is
/// E0027 here: the pair above is the scope an accumulator opens on and
/// the scope a refusal names, and a target that grew a third component
/// while the sentence still named two would name the wrong scope.
impl Say for BlendTarget {
    fn say(&self, f: &mut core::fmt::Formatter<'_>, by: Speaker<'_>) -> core::fmt::Result {
        let Self { node, body } = *self;
        DrawnBody::of(node, body).say(f, by)
    }
}

/// The sentence where no document is at hand: the node by its tag.
impl core::fmt::Display for BlendTarget {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.say(f, Speaker::TAG)
    }
}

vocabulary! {
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
        Fillet = "fillet",
        /// An equal-setback flat chamfer (`Node::Chamfer`).
        Chamfer = "chamfer",
    }

    /// Both kinds with their button labels — the chrome's radio row
    /// and a test that sweeps them.
    pub const ALL;
}

impl BlendKindChoice {
    /// What the one Length field means for this kind, for the field's
    /// own label.
    ///
    /// The QUANTITY and not its unit: the field's unit is the picker
    /// beside it to say ([`crate::widgets::length_picker`]), and a
    /// label that carried one too would be a second place for it to be
    /// stated — free to say metres beside a field written in
    /// millimetres.
    pub fn size_label(self) -> &'static str {
        match self {
            Self::Fillet => "radius",
            Self::Chamfer => "setback",
        }
    }
}

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
        /// The refused pick, said as the chrome says a pick
        /// ([`PickedName`]).
        picked: EdgeSelection,
    },
    /// The target has a value and no edges, so nothing was loaded and
    /// the held set is untouched.
    NoEdgesOnTarget {
        /// The body that was asked.
        target: BlendTarget,
    },
    /// The index draws edges on the target that it cannot name, so
    /// "every edge" is not a set the tool can hold: nothing was loaded
    /// and the held set is untouched.
    EdgesUnnamed {
        /// The index's refusal, which names the body that was asked.
        refused: EdgeNamesRefused,
    },
    /// The target has no value in this evaluation, so there are no
    /// edges to load; nothing was loaded and the held set is
    /// untouched. The standing says why and where the repair is.
    TargetHasNoValue {
        /// The body that was asked.
        target: BlendTarget,
        /// Its node's standing, as the feature tree draws it
        /// ([`crate::tree::standing_as_drawn`]).
        ///
        /// Its `through` may be a mate, which is not the DAG ancestor
        /// `NodeStanding` documents
        /// (`work/wire/kernel-standing-names-a-cluster-refused-node-as-its-own-failure`).
        standing: NodeStanding,
    },
    /// The target node is no longer in the document, so every held
    /// edge is about a body that is gone: the whole set is dropped at
    /// once.
    TargetLost {
        /// The body the set was about.
        target: BlendTarget,
        /// The target's node by the last label the document gave it:
        /// the node is gone, so no later document says it.
        node: SpokenNode,
        /// How many edges went with it.
        edges: usize,
    },
    /// The target survived an upstream edit but some held edges did
    /// not — the strand case, dropped loudly rather than carried to a
    /// commit that would refuse on arrival (module docs: why this is
    /// not the silent shrink #217 forbids).
    ///
    /// **Its sentence counts the lost edges and says none of them.** A
    /// lost name is one the edit removed: the profile steps and nodes
    /// its words cite may be gone with it, and the tool keeps no
    /// labels per edge, so its words would be tags. And a held set can
    /// be a whole body's edges (the all-edges door), which no status
    /// line lists. The names ride the value for a chrome that draws
    /// them and for a row to assert on.
    EdgesLost {
        /// The body the set is about.
        target: BlendTarget,
        /// The names that stopped being edges of it, in canonical
        /// order — carried rather than counted, so a chrome that wants
        /// to say WHICH can, and a row can assert on them.
        names: Vec<StableName>,
        /// How many held picks lost a name: a rim that lost one arc is
        /// one pick, as the panel counted it.
        picks: usize,
        /// How many picks are still held.
        kept: usize,
    },
    /// An edge pick arrived with no pick index to say what drawn edge
    /// it is part of, so the tool cannot tell a whole edge from one
    /// arc of a circle: the pick is refused rather than held as a
    /// possible half, and the held set is untouched.
    Unindexed {
        /// The refused pick, said as the chrome says a pick
        /// ([`PickedName`]).
        picked: EdgeSelection,
        /// Why there is no index: one being built, or none and none
        /// coming ([`NotIndexed::Absent`], whose reason the pick-index
        /// badge carries).
        why: NotIndexed,
    },
    /// The index could not say which drawn edge the pick is part of
    /// ([`PickIndex::whole_edge`]): a naming or pairing bug, so the pick
    /// is refused and the held set is untouched.
    RimUnanswered {
        /// The rim door's refusal.
        fault: RimNameFault,
    },
}

impl Say for BlendEvent {
    fn say(&self, f: &mut core::fmt::Formatter<'_>, by: Speaker<'_>) -> core::fmt::Result {
        match self {
            Self::OtherTarget { held, picked } => write!(
                f,
                "the held edges are on {}, so {} was not taken; cancel to start on another body",
                Said(held, by),
                Said(&PickedName::edge(picked), by)
            ),
            Self::NoEdgesOnTarget { target } => {
                write!(f, "{} has no edges to select", Said(target, by))
            }
            Self::EdgesUnnamed { refused } => {
                write!(f, "the tool loaded no edges: {}", Said(refused, by))
            }
            Self::TargetHasNoValue { target, standing } => write!(
                f,
                "{} has no edges to select: {}",
                Said(target, by),
                Said(standing, by.about(target.node))
            ),
            Self::TargetLost {
                target,
                node,
                edges,
            } => {
                // The node is gone, so it is said as the tool kept it.
                let kept: HeldNodes = [node.clone()].into_iter().collect();
                write!(
                    f,
                    "{} is no longer in the document; the tool dropped all {edges} picked edges",
                    Said(target, Speaker::held(&kept))
                )
            }
            Self::EdgesLost {
                target,
                names: _,
                picks,
                kept,
            } => write!(
                f,
                "an edit removed all or part of {picks} of the picked edges from {}; the tool \
                 dropped what went and still holds {kept}",
                Said(target, by)
            ),
            Self::Unindexed { picked, why } => {
                write!(
                    f,
                    "{} was not taken: without a pick index the tool cannot tell a whole edge \
                     from one arc of a circle; ",
                    Said(&PickedName::edge(picked), by)
                )?;
                match why {
                    NotIndexed::Building => {
                        write!(
                            f,
                            "the picture is still being indexed, so pick it again then"
                        )
                    }
                    NotIndexed::Absent | NotIndexed::AnotherPicture => write!(
                        f,
                        "no index is being built — the pick-index badge says why the last \
                         build refused, and an edit or a new display tolerance is what builds \
                         it again"
                    ),
                }
            }
            Self::RimUnanswered { fault } => {
                write!(f, "the edge was not taken: {}", Said(fault, by))
            }
        }
    }
}

/// The sentence where no document is at hand: each node by its tag.
impl core::fmt::Display for BlendEvent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.say(f, Speaker::TAG)
    }
}

/// **The blend tool**: one target body, an accumulating set of its
/// edges, and one committed blend edit.
///
/// The set is a `BTreeSet` of stable names, which makes two of this
/// tool's rules structural rather than checked: a name cannot be held
/// twice, and what [`BlendTool::selection`] hands the canonicalizing
/// constructor is already in canonical order. Picking a held edge
/// again REMOVES it — the per-pick add/remove the plan asks for, and
/// the only correction a set-valued pick needs.
///
/// # A drawn circle is one pick
///
/// **A pick is the whole drawn edge** ([`PickIndex::whole_edge`]): on
/// a rim the picture draws as one circle but the body holds as several
/// arcs, one click holds every arc. `rims` records which held names
/// came in together, so the live count is the picks the user made
/// rather than the names a commit stores, and un-picking any arc of a
/// rim drops the whole rim. Every rim is a subset of `edges`, the rims
/// are disjoint, and each holds two names or more.
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
/// [`BlendTool::fillet_op`] reads the invariant rather than
/// re-checking both halves: a tool holding nothing has no op.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BlendTool {
    target: Option<HeldTarget>,
    edges: BTreeSet<StableName>,
    rims: BTreeSet<BTreeSet<StableName>>,
}

/// The body the held edges are on, and its node as the document last
/// spoke it ([`BlendTool::respeak`]): what [`BlendEvent::TargetLost`]
/// names it by once the node is gone.
#[derive(Debug, Clone, PartialEq, Eq)]
struct HeldTarget {
    at: BlendTarget,
    node: SpokenNode,
}

impl BlendTool {
    /// A tool holding nothing.
    pub fn new() -> Self {
        Self::default()
    }

    /// The body the held edges are on, once a pick has fixed it.
    pub fn target(&self) -> Option<BlendTarget> {
        self.target.as_ref().map(|held| held.at)
    }

    /// How many edges are held — the panel's live count, a rim
    /// counting once.
    pub fn count(&self) -> usize {
        let in_rims = |name: &&StableName| self.rims.iter().any(|rim| rim.contains(*name));
        self.edges.iter().filter(|name| !in_rims(name)).count() + self.rims.len()
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
    /// This is the value claim; [`BlendTool::held_edges`] is what the
    /// per-frame mark reads, and `a_held_set_marks_exactly_the_edges_it_names`
    /// asserts the two agree so they cannot drift.
    pub fn marks(&self) -> Vec<EdgeSelection> {
        let Some(target) = self.target() else {
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

    /// **The held set as the marks read it** — its body and its
    /// names, `None` while nothing is held (the struct's invariant).
    pub fn held_edges(&self) -> Option<crate::marks::HeldEdges<'_>> {
        self.target().map(|target| crate::marks::HeldEdges {
            node: target.node,
            body: target.body,
            names: &self.edges,
        })
    }

    /// **Feed one edge pick**: add the whole drawn edge it is part of,
    /// or REMOVE it when it is already held.
    ///
    /// `index` says what the pick's whole is ([`PickIndex::whole_edge`]):
    /// every arc of a rim drawn as one circle, or why there is no index.
    /// A new pick with no index is refused ([`BlendEvent::Unindexed`]), and so is one whose rim
    /// the index could not answer ([`BlendEvent::RimUnanswered`]);
    /// un-picking a held edge needs neither.
    ///
    /// The first pick fixes the target; un-picking the last edge
    /// releases it again (the struct's invariant). A pick on another
    /// body is refused ([`BlendEvent::OtherTarget`]) and changes
    /// nothing. `doc` says the target's node when the pick fixes it.
    pub fn pick(
        &mut self,
        doc: &Doc<ProfileProgram>,
        edge: &EdgeSelection,
        index: Result<&PickIndex, NotIndexed>,
    ) -> Option<BlendEvent> {
        let picked = BlendTarget::of(edge);
        match self.target() {
            Some(held) if held != picked => {
                return Some(BlendEvent::OtherTarget {
                    held,
                    picked: edge.clone(),
                });
            }
            Some(_) if self.holds(&edge.name) => {
                self.drop_pick(&edge.name);
                self.release_if_empty();
                return None;
            }
            Some(_) | None => {}
        }
        // A pick from a source other than the viewport (a refusal's
        // named-cell link) can arrive while the index is being built.
        // Held as named it could be half a circle, which is the commit
        // that refuses at the seam vertex, so it is refused instead.
        let index = match index {
            Ok(index) => index,
            Err(why) => {
                return Some(BlendEvent::Unindexed {
                    picked: edge.clone(),
                    why,
                });
            }
        };
        let whole = match index.whole_edge(edge) {
            Ok(whole) => whole,
            Err(fault) => return Some(BlendEvent::RimUnanswered { fault }),
        };
        if self.target.is_none() {
            self.fix(doc, picked);
        }
        self.take(whole);
        None
    }

    /// Hold one pick's names: a rim absorbs any of its arcs already
    /// held, so the rims stay disjoint.
    fn take(&mut self, whole: Vec<StableName>) {
        let whole: BTreeSet<StableName> = whole.into_iter().collect();
        if whole.len() > 1 {
            // An arc held alone joins the rim (a partial revolve turned
            // full under a held arc). A held RIM meeting a new whole is
            // not reachable: the clicked name is not held, and within one
            // index the rims partition the drawn edges.
            debug_assert!(
                self.rims.iter().all(|rim| rim.is_disjoint(&whole)),
                "a held rim meets the new pick {whole:?}: {:?}",
                self.rims
            );
            self.rims.insert(whole.clone());
        }
        self.edges.extend(whole);
    }

    /// Drop the pick holding `name`: its whole rim, or the name alone.
    fn drop_pick(&mut self, name: &StableName) {
        match self.rims.iter().find(|rim| rim.contains(name)).cloned() {
            Some(rim) => {
                self.rims.remove(&rim);
                for arc in &rim {
                    self.edges.remove(arc);
                }
            }
            None => {
                self.edges.remove(name);
            }
        }
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
    /// **A drawn edge the index cannot name refuses the whole load**
    /// ([`BlendEvent::EdgesUnnamed`]), however many others it names:
    /// the set would not be every edge, and a name it lacks is not one
    /// a later click could add either.
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
    /// selection and the held mark (`crate::marks::HeldEdges`) apply — and
    /// the count the panel shows is the set the picture marks, on
    /// every target rather than only on the ones a commit would
    /// accept. This is not a display-tolerance dependency: the mesh
    /// carries one boundary polyline per topological edge, so δ
    /// changes how many POINTS each drawn edge has and never how many
    /// there are.
    ///
    /// # Loading a set is not a promise that it builds
    ///
    /// The kernel's blend assembly refuses what it does not build by
    /// name — mixed convexity, a turn of two edges at a corner,
    /// tangential runs, and a blend of a blend are all typed refusals
    /// on the node's own badge. This door hands over the edges that EXIST,
    /// which is a different claim, and the panel says so beside the
    /// button rather than implying every loaded set is buildable.
    pub fn load_all_edges(
        &mut self,
        target: BlendTarget,
        doc: &Doc<ProfileProgram>,
        eval: &Evaluation<f64>,
        index: &PickIndex,
    ) -> Option<BlendEvent> {
        if let Err(standing) = eval.usable(target.node) {
            return Some(BlendEvent::TargetHasNoValue {
                target,
                standing: crate::tree::standing_as_drawn(standing, eval),
            });
        }
        let drawn = index.edge_names_in(target.node, target.body);
        if let Some(refused) = drawn.refused {
            return Some(BlendEvent::EdgesUnnamed { refused });
        }
        let named: BTreeSet<StableName> = pncad::select::all_edges(eval, target.node)
            .into_iter()
            .collect();
        let edges: BTreeSet<StableName> = drawn
            .named
            .into_iter()
            .map(|(_, name)| name)
            .filter(|name| named.contains(*name))
            .cloned()
            .collect();
        if edges.is_empty() {
            return Some(BlendEvent::NoEdgesOnTarget { target });
        }
        let mut rims = BTreeSet::new();
        for name in &edges {
            let edge = EdgeSelection {
                name: name.clone(),
                node: target.node,
                body: target.body,
            };
            let rim: BTreeSet<StableName> = match index.whole_edge(&edge) {
                Ok(whole) => whole.into_iter().collect(),
                Err(fault) => return Some(BlendEvent::RimUnanswered { fault }),
            };
            if rim.len() > 1 && rim.is_subset(&edges) {
                rims.insert(rim);
            }
        }
        self.fix(doc, target);
        self.rims = rims;
        self.edges = edges;
        None
    }

    /// Fix the target, with its node as `doc` says it.
    fn fix(&mut self, doc: &Doc<ProfileProgram>, target: BlendTarget) {
        self.target = Some(HeldTarget {
            at: target,
            node: doc.spoken(target.node),
        });
    }

    /// **The target's node, spoken again from `doc`**, the shown
    /// document after an operation ([`SpokenNode::respoken`]'s rule), so
    /// a lost target is named by the last label it had.
    pub fn respeak(&mut self, doc: &Doc<ProfileProgram>) {
        if let Some(held) = &mut self.target {
            held.node = held.node.respoken(doc);
        }
    }

    /// Drop every pick — the panel's `Clear picks` button, and what
    /// Cancel's whole-tool replacement amounts to for the picks alone.
    ///
    /// **Destructured rather than field-cleared**, so a field added to
    /// [`BlendTool`] is E0027 here rather than surviving a door whose
    /// whole contract is that the tool holds nothing afterwards — the
    /// state a fresh tool is in, which is what lets the next click
    /// start on any body.
    pub fn clear(&mut self) {
        let Self {
            target,
            edges,
            rims,
        } = self;
        *target = None;
        edges.clear();
        rims.clear();
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
        let Some(HeldTarget { at: target, node }) = self.target.clone() else {
            return Vec::new();
        };
        if doc.node(target.node).is_none() {
            let edges = self.count();
            self.clear();
            return vec![BlendEvent::TargetLost {
                target,
                node,
                edges,
            }];
        }
        let Some((_, eval)) = landed else {
            return Vec::new();
        };
        // A target with no value in this run cannot say which edges it
        // has: dropping the set on a transient failure would cost the
        // picks the moment an upstream slot went momentarily bad. Say
        // nothing and wait for a run that has an answer; the node's
        // badge carries its standing.
        if eval.usable(target.node).is_err() {
            return Vec::new();
        }
        let live: BTreeSet<StableName> = pncad::select::all_edges(eval, target.node)
            .into_iter()
            .collect();
        let names: Vec<StableName> = self
            .edges
            .iter()
            .filter(|name| !live.contains(*name))
            .cloned()
            .collect();
        if names.is_empty() {
            return Vec::new();
        }
        let lost_from_rims = self
            .rims
            .iter()
            .filter(|rim| names.iter().any(|name| rim.contains(name)))
            .count();
        let picks = lost_from_rims
            + names
                .iter()
                .filter(|name| !self.rims.iter().any(|rim| rim.contains(*name)))
                .count();
        for name in &names {
            self.edges.remove(name);
        }
        // A rim that lost an arc is no longer one drawn circle held
        // whole: what is left of it stands as single picks.
        self.rims.retain(|rim| rim.is_subset(&self.edges));
        self.release_if_empty();
        vec![BlendEvent::EdgesLost {
            target,
            names,
            picks,
            kept: self.count(),
        }]
    }

    /// **The one committed edit, as a fillet**: the session op that
    /// inserts `Node::Fillet` through the ordinary commit door, or
    /// `None` until an edge is held, when there is no body to blend and
    /// so no edit to author. The op's refusals are the doors': the
    /// target's kind and the selection's form at the session's insert,
    /// and a stranded name or a radius the geometry cannot take at
    /// evaluation, on the node's own badge.
    pub fn fillet_op(&self, radius: Formula) -> Option<SessionOp> {
        Some(SessionOp::AddFillet {
            target: self.held_target()?,
            radius,
            selection: self.selection(),
        })
    }

    /// **The one committed edit, as a chamfer**: [`BlendTool::fillet_op`]'s
    /// twin, and the size means a setback along both supports rather
    /// than a radius.
    ///
    /// `None` as [`BlendTool::fillet_op`].
    pub fn chamfer_op(&self, distance: Formula) -> Option<SessionOp> {
        Some(SessionOp::AddChamfer {
            target: self.held_target()?,
            distance,
            selection: self.selection(),
        })
    }

    /// The node a commit blends, while an edge is held: the struct's
    /// invariant makes "no target" and "no edges" one state, so reading
    /// the target alone is reading both.
    fn held_target(&self) -> Option<RecipeNodeId> {
        debug_assert_eq!(
            self.target.is_some(),
            !self.edges.is_empty(),
            "a target is held exactly while an edge is"
        );
        self.target().map(|target| target.node)
    }
}

/// **What the all-edges load does with the index's refusals**, at an
/// index a row can put a naming-emission bug into
/// (`PickIndex::unname_edge`) — which is why these rows are here and
/// not with the rest of the tool's in `tests/blend_authoring.rs`.
#[cfg(test)]
mod tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used, clippy::panic)]

    use super::{BlendEvent, BlendTarget, BlendTool, Said, Speaker};
    use crate::pickindex::{EdgeId, EdgeNameFault, EdgeNamesRefused};
    use crate::session::EdgeSelection;
    use crate::test_support::plate_indexed;

    /// **A target whose drawn edges the index cannot all name loads
    /// nothing, and says so in the index's words** — for one such edge
    /// and for every one — and never says the target has no edges. The
    /// held set is untouched.
    #[test]
    fn a_target_whose_edges_the_index_cannot_name_refuses_the_load_in_its_words() {
        let (eval, mut index, copy) = plate_indexed(pncad::geom_core::Tol::witness());
        let (doc, _) = crate::scene::plate_with_hole(pncad::geom_core::Tol::witness())
            .expect("the plate authors");
        let target = BlendTarget {
            node: copy,
            body: 0,
        };
        let drawn = index.edges_in(copy, 0).to_vec();
        let mut tool = BlendTool::new();
        let held = EdgeSelection {
            name: index.edge_name_of(drawn[0]).expect("named").clone(),
            node: copy,
            body: 0,
        };
        assert_eq!(tool.pick(&doc, &held, Ok(&index)), None);
        let before = tool.clone();

        let first = index.unname_edge(drawn[2]);
        let refused = EdgeNamesRefused {
            node: copy,
            body: 0,
            first,
            named: drawn.len() - 1,
            refused: 1,
        };
        let partly = tool
            .load_all_edges(target, &doc, &eval, &index)
            .expect("a refused load says so");
        assert_eq!(
            partly,
            BlendEvent::EdgesUnnamed {
                refused: refused.clone(),
            }
        );
        assert_eq!(tool, before, "nothing was loaded and the held set stands");
        let said = partly.to_string();
        assert!(
            said.contains(&refused.to_string())
                && said.contains(
                    &Said(&EdgeNameFault::Unnamed(first), Speaker::TAG.about(copy)).to_string()
                ),
            "the index's own words, through its Display: {said}"
        );
        assert!(
            !said.contains(&BlendEvent::NoEdgesOnTarget { target }.to_string()),
            "a body whose edges have no names is not a body with no edges: {said}"
        );

        for &id in &drawn[3..] {
            index.unname_edge(id);
        }
        index.unname_edge(drawn[0]);
        index.unname_edge(drawn[1]);
        match tool.load_all_edges(target, &doc, &eval, &index) {
            Some(BlendEvent::EdgesUnnamed { refused }) => {
                assert_eq!(
                    (refused.named, refused.refused),
                    (0, drawn.len()),
                    "every drawn edge refused"
                );
                assert_eq!(refused.first, first);
            }
            other => panic!("every name refusing is still the index's refusal: {other:?}"),
        }
        assert_eq!(tool, before);
    }

    /// **A target the index does not draw is not a naming refusal.**
    /// Its edges are [`EdgeNameFault::NotDrawn`]'s — the ordinary arm —
    /// and the load answers what it answers for a body with no edges.
    #[test]
    fn a_target_the_index_does_not_draw_is_not_a_naming_refusal() {
        let (eval, index, copy) = plate_indexed(pncad::geom_core::Tol::witness());
        let (doc, _) = crate::scene::plate_with_hole(pncad::geom_core::Tol::witness())
            .expect("the plate authors");
        let target = BlendTarget {
            node: copy,
            body: 7,
        };
        assert!(matches!(
            index.edge_name_of(EdgeId {
                node: copy,
                body: 7,
                boundary: 0,
            }),
            Err(EdgeNameFault::NotDrawn { .. })
        ));
        assert_eq!(
            BlendTool::new().load_all_edges(target, &doc, &eval, &index),
            Some(BlendEvent::NoEdgesOnTarget { target })
        );
    }
}
