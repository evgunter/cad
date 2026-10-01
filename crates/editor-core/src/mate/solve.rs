//! **The mate solve** — reading edges, partitions, groups, and the
//! constructive placement (ASM-R2a D-2/D-3/D-4/D-5; A9/A10/A11/A12).
//!
//! Everything here is recipe data plus decided predicates (with the
//! one qualifier `ASSEMBLY.md` A11 rule 5 states for the lever —
//! [`MateReach`]), and nothing derived is stored beside the DAG. The
//! entry points, in the order the layers use them:
//!
//! - [`reading_edges`] — A12's second sort of edge, RECOMPUTED by
//!   walking from each reference's OPERAND every time it is wanted.
//! - [`relative_freedom_components`] — A9's partition, over consuming
//!   ∪ reading edges (so mates couple components).
//! - [`groups`] — A11's placement groups, the finer partition over
//!   instances alone, each with its document-order-first ROOT.
//! - [`solve_document`] — the per-pair coset fold along a deterministic
//!   spanning tree, yielding every instance's pose around its group's
//!   frame, every group's space, and every mate's role.
//! - [`admit_mate`] — one mate's own admission, the per-mate prefix
//!   of the solve asked by the edit door of a mate being inserted.
//!
//! # Why the failure surface is per node, not per document
//!
//! A refusing group must not fail an unrelated one: GQ2/W5 make a
//! node's failure poison its descendants and nothing else, and a
//! second group has no dependence on the first. So [`solve_document`]
//! is TOTAL — it returns per-node faults rather than one document-wide
//! `Err`, and every node the fault actually reaches (the refusing mate
//! and its group's instances, which now have no pose) carries it.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use geom_core::Tol;
use geom_core::linalg::frame::{FrameError, FrameInput, FrameVector};
use geom_core::linalg::{Affine3, Mat3, Point3, UnitVec3, UnitVec3Error, Vec3};
use geom_core::predicate::Band;

use super::coset::{Coset, FoldStop, Measured, Subgroup};
use super::member::{Member, Walk, check_reference, derived_offset, walk_of};
use super::reach::MateReach;
use super::{Alignment, AxisSense, Clash, Lever, MateFault, MatePrimitive, MateSide, OffsetCheck};
use crate::doc::Doc;
use crate::eval::NodeRefusal;
use crate::expr::ParamEnv;
use crate::node::{Node, RecipeNodeId};
use crate::placement::Frame;

/// What a mate did in the solve (A11 rule 4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MateRole {
    /// A tree mate: it placed its child. Its pair's fold was DETERMINED.
    Determining,
    /// A non-tree mate: it solved nothing and is carried to evaluation
    /// as a pure contact declaration. R2-b verifies it against the
    /// solved geometry; this unit records only that it declares.
    Declaring,
    /// The mate refused; see the fault recorded against it.
    Refused,
}

/// **Which space an instance lives in** (A9, A11 (2)): the world,
/// or the own space of a group nothing places.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Space {
    /// The world: the group is placed, by its root's offset on a live
    /// gauge chain.
    World,
    /// The own space of a group nothing places, named by the group's
    /// earliest instance, which sits at that space's origin.
    Own {
        /// The group's earliest instance.
        group: RecipeNodeId,
    },
}

/// **Why a group is unplaced** (A11 (2)): what a placement would have
/// come from, and is missing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unplaced {
    /// No member carries an offset: the group's placed member or the
    /// placing mate that tied it to one was deleted, or its offset
    /// cleared.
    NoOffset,
    /// The group's gauge chain names a gauge that was deleted.
    DeadGauge {
        /// The deleted gauge the chain names.
        gauge: RecipeNodeId,
    },
}

impl core::fmt::Display for Unplaced {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NoOffset => f.write_str("no instance in it carries an offset"),
            Self::DeadGauge { gauge } => {
                write!(
                    f,
                    "its gauge chain names node {}, which was deleted",
                    gauge.0
                )
            }
        }
    }
}

/// The one recourse an unplaced group's refusal names: the three
/// things that place a group (A11 (2)).
pub const UNPLACED_RECOURSE: &str = "place it: give one of its instances an offset (SetOffset), set its gauge to a live one \
     (SetGauge), or mate it to a placed instance on its gauge";

/// **An instance's solved pose, decomposed** around its group's frame:
/// the instance sits at `left ∘ F ∘ right`, where `F` is the group's
/// frame — its gauge chain composed with its root's offset in the
/// world, the identity in a group's own space.
///
/// `right` is the composed coset representatives along the spanning
/// tree from the root; `left` is the composed derived offsets of the
/// placers (transforms, pattern copies) the tree's mates read through,
/// `None` when no placer is on the path, which is the identity BY
/// CONSTRUCTION. Keeping the two apart is what lets the solve read no
/// gauge frame: a placer's offset is a map in document coordinates,
/// so it composes OUTSIDE the group's frame, and the evaluation
/// composes that frame in its own lane.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Pose {
    /// The composed placer offsets, outside the group's frame.
    pub(crate) left: Option<Frame>,
    /// The composed representatives, inside it.
    pub(crate) right: Frame,
}

/// The document's solved poses (D-5's compose-outward input).
///
/// A solve is a solve OF a document, and it says so: `document` is the
/// id [`solve_document`] read, and [`SolvedPoses::placement`] — the one
/// door that takes a `Doc` back — refuses a mispairing with it (DI3).
/// No `Default`, for that reason: a poses value with no document is a
/// value that cannot answer which document it is about.
#[derive(Debug, Clone)]
pub struct SolvedPoses {
    /// **Which document this is a solve OF** (DI3), stamped by
    /// [`solve_document`].
    document: crate::ident::DocumentId,
    /// The tolerance the solve decided under, which the nominal
    /// world-pose door ([`SolvedPoses::placement`]) evaluates a gauge
    /// chain under too.
    tol: Tol,
    /// Each live instance's pose, decomposed around its group's frame
    /// ([`Pose`]). The root's own entry is the identity, bit-exactly.
    pose: BTreeMap<RecipeNodeId, Pose>,
    /// Each live instance's group root: the member whose offset places
    /// the group, or, in a group nothing places, its earliest instance.
    root: BTreeMap<RecipeNodeId, RecipeNodeId>,
    /// Each unplaced group's cause, keyed by its root.
    unplaced: BTreeMap<RecipeNodeId, Unplaced>,
    /// Each live mate's role.
    roles: BTreeMap<RecipeNodeId, MateRole>,
    /// Per-node refusals: the refusing mate, and every node in its
    /// group that consequently has no pose.
    faults: BTreeMap<RecipeNodeId, MateFault>,
}

/// **Why the nominal world-pose door has no pose** for an instance
/// ([`SolvedPoses::placement`]).
#[derive(Debug, Clone, PartialEq)]
pub enum PoseRefusal {
    /// The solve refused for the instance.
    Mate(Box<MateFault>),
    /// The instance's group is unplaced, so it has no world pose — only
    /// its pose in its group's own space ([`SolvedPoses::relative`]).
    Unplaced {
        /// The instance asked about.
        instance: RecipeNodeId,
        /// Its group's root.
        group: RecipeNodeId,
        /// What is missing.
        cause: Unplaced,
    },
    /// A placement on the group's frame did not evaluate at the
    /// document's own parameters: a gauge on the chain, or the root's
    /// offset.
    Placement {
        /// The gauge, or the root instance whose offset refused.
        node: RecipeNodeId,
        /// The evaluation layer's own refusal.
        error: NodeRefusal,
    },
}

impl core::fmt::Display for PoseRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Mate(fault) => write!(f, "{fault}"),
            Self::Unplaced {
                instance,
                group,
                cause,
            } => write!(
                f,
                "instance {} has no world pose: its group (rooted at node {}) is unplaced, \
                 because {cause}. {}",
                instance.0,
                group.0,
                crate::sentence::Recourse(UNPLACED_RECOURSE)
            ),
            Self::Placement { node, error } => write!(
                f,
                "the placement at node {} does not evaluate: {}",
                node.0,
                error.kind()
            ),
        }
    }
}

impl core::error::Error for PoseRefusal {}

impl SolvedPoses {
    /// An empty solve OF `document`: no poses, no roles, no faults.
    fn empty(document: crate::ident::DocumentId, tol: Tol) -> Self {
        Self {
            document,
            tol,
            pose: BTreeMap::new(),
            root: BTreeMap::new(),
            unplaced: BTreeMap::new(),
            roles: BTreeMap::new(),
            faults: BTreeMap::new(),
        }
    }

    /// **Which document this solve is of** (DI3). A caller holding a
    /// solve and a document can check the pairing itself; every door
    /// here that takes a `Doc` checks it already.
    pub fn document(&self) -> crate::ident::DocumentId {
        self.document
    }

    /// A node's recorded fault, if the solve refused for it.
    pub fn fault(&self, node: RecipeNodeId) -> Option<&MateFault> {
        self.faults.get(&node)
    }

    /// A mate's role, `None` if the node is not a live mate.
    pub fn role(&self, mate: RecipeNodeId) -> Option<MateRole> {
        self.roles.get(&mate).copied()
    }

    /// An instance's group root, `None` if the node is not a live
    /// instance.
    pub fn root(&self, instance: RecipeNodeId) -> Option<RecipeNodeId> {
        self.root.get(&instance).copied()
    }

    /// **The space an instance lives in**, `None` if the node is not a
    /// live instance.
    pub fn space(&self, instance: RecipeNodeId) -> Option<Space> {
        let root = self.root(instance)?;
        Some(match self.unplaced.get(&root) {
            None => Space::World,
            Some(_) => Space::Own { group: root },
        })
    }

    /// Why an instance's group is unplaced, `None` when it is placed
    /// or the node is not a live instance.
    pub fn unplaced(&self, instance: RecipeNodeId) -> Option<Unplaced> {
        self.unplaced.get(&self.root(instance)?).copied()
    }

    /// **An instance's pose in its group's own space**: where it sits
    /// when the group's frame is the identity — relative to its root
    /// when no placer stands on the path from the root, and in an
    /// unplaced group the pose the group is evaluated at.
    pub fn relative(&self, instance: RecipeNodeId) -> Option<Frame> {
        let pose = self.pose.get(&instance)?;
        Some(match pose.left {
            None => pose.right,
            Some(left) => left.compose(&pose.right),
        })
    }

    /// The decomposed pose the evaluation composes the group's frame
    /// into.
    pub(crate) fn pose(&self, instance: RecipeNodeId) -> Option<Pose> {
        self.pose.get(&instance).copied()
    }

    /// **The instance's world placement, at the document's own
    /// parameters** (A11 (5)): the group's frame — its gauge chain
    /// composed with its root's offset — composed into the solved pose
    /// ([`Pose`]). A lone instance returns its placement's frame bit
    /// for bit, and on the world at the empty offset the identity.
    ///
    /// `doc` is read for its gauges and offsets, and it must be the
    /// document this solve is OF: composing this document's poses onto
    /// another one's placements is a pose of neither. The pairing is
    /// CHECKED here (DI3) — the solve carries the id it was built
    /// from — rather than left to the caller.
    ///
    /// # Errors
    ///
    /// [`PoseRefusal::Mate`] holding [`MateFault::PosesOfAnotherDocument`]
    /// when `doc` is not the document this solve is of, or the group's
    /// own refusal when it did not solve; [`PoseRefusal::Unplaced`]
    /// when nothing places the group; [`PoseRefusal::Placement`] when
    /// a placement on the frame does not evaluate.
    pub fn placement<P>(&self, doc: &Doc<P>, instance: RecipeNodeId) -> Result<Frame, PoseRefusal> {
        if let Some(m) = crate::ident::mispaired(doc.id(), self.document) {
            return Err(PoseRefusal::Mate(Box::new(m.into())));
        }
        if let Some(fault) = self.faults.get(&instance) {
            return Err(PoseRefusal::Mate(Box::new(fault.clone())));
        }
        let root = self.root.get(&instance).copied().unwrap_or(instance);
        if let Some(&cause) = self.unplaced.get(&root) {
            return Err(PoseRefusal::Unplaced {
                instance,
                group: root,
                cause,
            });
        }
        let band = Band::linear(self.tol)
            .map_err(|error| PoseRefusal::Mate(Box::new(MateFault::Band { error })))?;
        let env = doc.param_env::<f64>();
        let frame =
            group_frame(doc, root, &env, band).map_err(|(node, error)| PoseRefusal::Placement {
                node,
                error: error.into(),
            })?;
        let pose = self.pose.get(&instance).copied().unwrap_or(Pose {
            left: None,
            right: Frame::IDENTITY,
        });
        Ok(Frame::from_motion(pose.compose_around(frame)))
    }
}

impl Pose {
    /// `left ∘ frame ∘ right`, through the composition rule's identity
    /// fast paths (`placement::Motion`), in whichever lane `frame` was
    /// evaluated in.
    pub(crate) fn compose_around<T: geom_core::Real>(
        self,
        frame: crate::placement::Motion<T>,
    ) -> crate::placement::Motion<T> {
        let left = self
            .left
            .map_or(crate::placement::Motion::Identity, |l| l.motion());
        left.compose(frame).compose(self.right.motion())
    }
}

// ---- A9: spaces ----

/// **Which space each node's value lives in** (A9, A11 (2)): an
/// instance lives in its group's space, a node consuming geometry
/// lives in its inputs' space, and a node that denotes no geometry of
/// its own — a gauge, a mate, a declaration, a measure, an assertion —
/// lives in none and constrains nothing.
#[derive(Debug, Clone, Default)]
pub(crate) struct Spaces {
    /// Every node that lives in a space: `None` the world, `Some` an
    /// unplaced group's own — the group, by its root, and why it is
    /// unplaced.
    pub(crate) space: BTreeMap<RecipeNodeId, Option<(RecipeNodeId, Unplaced)>>,
    /// The nodes of `space` that live in an unplaced group's own.
    pub(crate) own: BTreeMap<RecipeNodeId, (RecipeNodeId, Unplaced)>,
    /// Every node whose inputs lie in two spaces — one of them an
    /// unplaced group's, which this names. Such a node compares the
    /// group with something outside it, and refuses.
    pub(crate) across: BTreeMap<RecipeNodeId, (RecipeNodeId, Unplaced)>,
}

/// [`Spaces`] for `doc` under `poses`.
pub(crate) fn spaces_of<P: crate::ProfilePayload>(doc: &Doc<P>, poses: &SolvedPoses) -> Spaces {
    spaces_with(doc, |instance| {
        let root = poses.root(instance)?;
        Some((root, *poses.unplaced.get(&root)?))
    })
}

/// [`Spaces`] for `doc`, given each instance's own space (`None` the
/// world), in one pass in document order, which puts every input
/// before its consumer.
pub(crate) fn spaces_with<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    own_of: impl Fn(RecipeNodeId) -> Option<(RecipeNodeId, Unplaced)>,
) -> Spaces {
    let mut out = Spaces::default();
    for &id in doc.order() {
        let Some(node) = doc.node(id) else { continue };
        let here = match node {
            Node::Gauge { .. } | Node::Mate { .. } | Node::Declare { .. } => continue,
            Node::InstantiatePart { .. } => own_of(id),
            _ => {
                let mut distinct: Vec<Option<(RecipeNodeId, Unplaced)>> = Vec::new();
                for input in node.inputs() {
                    if let Some(&s) = out.space.get(&input)
                        && !distinct.iter().any(|d| d.map(|x| x.0) == s.map(|x| x.0))
                    {
                        distinct.push(s);
                    }
                }
                if distinct.len() > 1
                    && let Some(&group) = distinct.iter().flatten().next()
                {
                    out.across.insert(id, group);
                }
                // A measure and an assertion read geometry and answer
                // a number, which lives in no space.
                if matches!(node, Node::Measure { .. } | Node::Assertion { .. }) {
                    continue;
                }
                distinct.first().copied().flatten()
            }
        };
        if let Some(own) = here {
            out.own.insert(id, own);
        }
        out.space.insert(id, here);
    }
    out
}

// ---- A11 (2): gauges ----

/// **The gauge chain from `start` outward**: the gauges a reference to
/// `start` stands on, innermost first, ending at the world.
///
/// # Errors
///
/// The first reference on the chain that names no live gauge: a
/// deleted one, kept as A11 (2) keeps it. The doors refuse a live
/// non-gauge and a cycle (`DocEdit::SetGauge`, the load door), so this
/// reports one only for a document that skipped them, and stops
/// rather than loop.
pub fn gauge_chain<P>(
    doc: &Doc<P>,
    start: Option<RecipeNodeId>,
) -> Result<Vec<RecipeNodeId>, RecipeNodeId> {
    let mut chain = Vec::new();
    let mut at = start;
    while let Some(gauge) = at {
        let Some(Node::Gauge { parent, .. }) = doc.node(gauge) else {
            return Err(gauge);
        };
        if chain.contains(&gauge) {
            return Err(gauge);
        }
        chain.push(gauge);
        at = *parent;
    }
    Ok(chain)
}

/// **The frame a gauge reference names, at `env`**: the chain's
/// placements composed outermost first, so `[world, g1, g0]` is
/// `P(g1) ∘ P(g0)` acting on what sits on `g0`. The world is the
/// bit-exact identity.
///
/// # Errors
///
/// The node whose placement refused, with its refusal; a dead chain
/// is the caller's to have refused first, and is reported at the dead
/// reference with the gauge-kind refusal here.
pub(crate) fn gauge_frame<P, T: geom_core::Decide>(
    doc: &Doc<P>,
    gauge: Option<RecipeNodeId>,
    env: &ParamEnv<T>,
    band: Band,
) -> Result<crate::placement::Motion<T>, (RecipeNodeId, crate::eval::NodeErrorKind)> {
    let chain = gauge_chain(doc, gauge).map_err(|dead| {
        (
            dead,
            crate::eval::NodeErrorKind::Unplaced {
                group: dead,
                cause: Unplaced::DeadGauge { gauge: dead },
            },
        )
    })?;
    let mut frame = crate::placement::Motion::Identity;
    for &g in chain.iter().rev() {
        let Some(Node::Gauge { placement, .. }) = doc.node(g) else {
            unreachable!("gauge_chain yields live gauges only")
        };
        let step = placement.motion_at(env, band).map_err(|e| (g, e))?;
        frame = frame.compose(step);
    }
    Ok(frame)
}

/// **A placed group's frame, at `env`** (A11 (5)): its root's gauge
/// chain composed with its root's offset.
///
/// # Errors
///
/// [`gauge_frame`]'s, and the root's own when its offset refuses.
pub(crate) fn group_frame<P, T: geom_core::Decide>(
    doc: &Doc<P>,
    root: RecipeNodeId,
    env: &ParamEnv<T>,
    band: Band,
) -> Result<crate::placement::Motion<T>, (RecipeNodeId, crate::eval::NodeErrorKind)> {
    let Some(Node::InstantiatePart { gauge, offset, .. }) = doc.node(root) else {
        return Ok(crate::placement::Motion::Identity);
    };
    let frame = gauge_frame(doc, *gauge, env, band)?;
    let offset = match offset {
        Some(offset) => offset.motion_at(env, band).map_err(|e| (root, e))?,
        None => crate::placement::Motion::Identity,
    };
    Ok(frame.compose(offset))
}

// ---- A12: reading edges, recomputed ----

/// **A12's reading edges**, recomputed from the recipe:
/// `(mate, instance)` for every mate reference that resolves to a
/// member of the A11 vocabulary. The edge lands on the MEMBER's
/// instance — the one the walk from the operand ends on — which is the
/// vertex the A9/A11 partitions see.
///
/// Never stored — the DAG stays the single structure, and a reference
/// that resolves to no member simply contributes no edge (N5).
/// Deterministic order: document order of the mate, then `a` before
/// `b`.
pub fn reading_edges<P>(doc: &Doc<P>) -> Vec<(RecipeNodeId, RecipeNodeId)> {
    let mut out = Vec::new();
    for &id in doc.order() {
        let Some(Node::Mate { a, b, .. }) = doc.node(id) else {
            continue;
        };
        for (side, name) in [(MateSide::A, a), (MateSide::B, b)] {
            if let Ok(w) = walk_of(doc, id, side, name) {
                out.push((id, w.member.instance));
            }
        }
    }
    out
}

/// **A9's relative-freedom partition**: the connected components of the
/// recipe DAG over CONSUMING ∪ READING edges, each component's nodes in
/// document order, the components themselves ordered by their first
/// node.
///
/// Two instances are relatively unconstrained exactly when they land in
/// different components — decidable from recipe structure alone, which
/// is the whole content of A9. Mates couple components precisely
/// because their reading edges count here (A12) even though A10's
/// invariants never see them.
pub fn relative_freedom_components<P: crate::ProfilePayload>(
    doc: &Doc<P>,
) -> Vec<Vec<RecipeNodeId>> {
    let mut adjacency: BTreeMap<RecipeNodeId, BTreeSet<RecipeNodeId>> = BTreeMap::new();
    let mut edges: Vec<(RecipeNodeId, RecipeNodeId)> = Vec::new();
    for &id in doc.order() {
        adjacency.entry(id).or_default();
        if let Some(node) = doc.node(id) {
            edges.extend(node.inputs().into_iter().map(|input| (id, input)));
        }
    }
    edges.extend(reading_edges(doc));
    for (x, y) in edges {
        adjacency.entry(x).or_default().insert(y);
        adjacency.entry(y).or_default().insert(x);
    }
    components(doc.order(), &adjacency)
}

/// Connected components over `adjacency`, seeded in `order` so both the
/// components and their contents are document-ordered.
fn components(
    order: &[RecipeNodeId],
    adjacency: &BTreeMap<RecipeNodeId, BTreeSet<RecipeNodeId>>,
) -> Vec<Vec<RecipeNodeId>> {
    let position: BTreeMap<RecipeNodeId, usize> =
        order.iter().enumerate().map(|(i, &id)| (id, i)).collect();
    let mut seen: BTreeSet<RecipeNodeId> = BTreeSet::new();
    let mut out = Vec::new();
    for &seed in order {
        if !seen.insert(seed) {
            continue;
        }
        let mut members = vec![seed];
        let mut stack = vec![seed];
        while let Some(id) = stack.pop() {
            for &next in adjacency.get(&id).into_iter().flatten() {
                if position.contains_key(&next) && seen.insert(next) {
                    members.push(next);
                    stack.push(next);
                }
            }
        }
        members.sort_by_key(|id| position[id]);
        out.push(members);
    }
    out
}

// ---- A11: placement groups ----

/// **Whether a mate on these two instances places** (A11 (2)): both
/// name the same gauge reference — the world, or one gauge id, the
/// identity intensional. Otherwise it declares (A5 verifies it).
pub fn places<P>(doc: &Doc<P>, a: RecipeNodeId, b: RecipeNodeId) -> bool {
    let gauge = |id| doc.node(id).and_then(Node::gauge_ref);
    gauge(a) == gauge(b)
}

/// **A11's placement groups**: the connected components of the
/// instance graph under PLACING mates ([`places`]), each in document
/// order, the groups ordered by their earliest instance.
///
/// Every member of a group names one gauge reference, since a placing
/// mate joins only instances that do. A lone instance is a singleton
/// group.
pub fn groups<P>(doc: &Doc<P>) -> Vec<Vec<RecipeNodeId>> {
    groups_welded_by(doc, &welds(doc, &read_mates(doc)))
}

/// **A group's root, and why it is unplaced if it is** (A11 (2), (3)):
/// its earliest member, in document order, that carries an offset,
/// when its gauge chain is live; otherwise its earliest instance, and
/// the cause. `members` is one of [`groups`]'s, in document order.
pub(crate) fn root_and_cause<P>(
    doc: &Doc<P>,
    members: &[RecipeNodeId],
) -> (RecipeNodeId, Option<Unplaced>) {
    let Some(&earliest) = members.first() else {
        unreachable!("a group has at least one instance")
    };
    let gauge = doc.node(earliest).and_then(Node::gauge_ref);
    if let Err(dead) = gauge_chain(doc, gauge) {
        return (earliest, Some(Unplaced::DeadGauge { gauge: dead }));
    }
    let offset = |id: &&RecipeNodeId| {
        matches!(
            doc.node(**id),
            Some(Node::InstantiatePart {
                offset: Some(_),
                ..
            })
        )
    };
    match members.iter().find(offset) {
        Some(&root) => (root, None),
        None => (earliest, Some(Unplaced::NoOffset)),
    }
}

/// One mate's two references as [`read_mates`] read them: both walks,
/// or the first refusal that stops the mate being an edge at all.
type ReadMate = Result<(Walk, Walk), MateFault>;

/// **Which mates WELD, read once**: each live mate in document order
/// with both its references walked, or the first refusal that stops
/// it being an edge at all.
///
/// The one reading [`groups`] and [`solve_document`] share, so the
/// partition the roots are read off and the partition the solve folds
/// over cannot disagree.
///
/// STRUCTURAL, and that is the point: it walks and nothing more, so
/// the partition never depends on a slot value. The solve's own
/// further checks ([`check_reference`]) can refuse a mate this admits
/// — such a mate welds its group and contributes no PAIR, so its
/// instances keep the group's frame and no pose is invented for
/// them.
fn read_mates<P>(doc: &Doc<P>) -> Vec<(RecipeNodeId, ReadMate)> {
    let mut out = Vec::new();
    for &id in doc.order() {
        let Some(Node::Mate { a, b, .. }) = doc.node(id) else {
            continue;
        };
        out.push((
            id,
            walk_of(doc, id, MateSide::A, a)
                .and_then(|wa| Ok((wa, walk_of(doc, id, MateSide::B, b)?))),
        ));
    }
    out
}

/// The instance pairs [`read_mates`] welds — its resolving PLACING
/// mates, projected onto the vertices A11's groups see.
fn welds<P>(doc: &Doc<P>, read: &[(RecipeNodeId, ReadMate)]) -> Vec<(RecipeNodeId, RecipeNodeId)> {
    read.iter()
        .filter_map(|(_, r)| r.as_ref().ok())
        .map(|(wa, wb)| (wa.member.instance, wb.member.instance))
        .filter(|&(a, b)| places(doc, a, b))
        .collect()
}

/// The groups a given set of WELDS produces.
///
/// A weld standing on ONE instance (two copies of a pattern mated to
/// each other) joins nothing and is dropped here rather than at each
/// caller, so no caller can forget it.
fn groups_welded_by<P>(
    doc: &Doc<P>,
    welds: &[(RecipeNodeId, RecipeNodeId)],
) -> Vec<Vec<RecipeNodeId>> {
    let instances: Vec<RecipeNodeId> = doc
        .order()
        .iter()
        .copied()
        .filter(|&id| matches!(doc.node(id), Some(Node::InstantiatePart { .. })))
        .collect();
    let mut adjacency: BTreeMap<RecipeNodeId, BTreeSet<RecipeNodeId>> = BTreeMap::new();
    for &id in &instances {
        adjacency.entry(id).or_default();
    }
    for &(x, y) in welds {
        if x == y {
            continue;
        }
        adjacency.entry(x).or_default().insert(y);
        adjacency.entry(y).or_default().insert(x);
    }
    components(&instances, &adjacency)
}

/// The root of `instance`'s group ([`root_and_cause`]), or `instance`
/// itself when it is not a live instance.
pub fn root_of<P>(doc: &Doc<P>, instance: RecipeNodeId) -> RecipeNodeId {
    groups(doc)
        .into_iter()
        .find(|c| c.contains(&instance))
        .map_or(instance, |c| root_and_cause(doc, &c).0)
}

// ---- D-4: the per-pair coset solve ----

/// **One solve's inputs**, borrowed for its duration and read by every
/// step below `solve_document`: the document, its one nominal
/// environment, the reach the lever is asked through, and the
/// decision band with the tolerance it was derived from. Nothing here
/// outlives the solve and nothing is derived into it — a cache would
/// be a second answer to a question the document already answers.
struct Solve<'a, P> {
    doc: &'a Doc<P>,
    env: &'a ParamEnv<f64>,
    reach: &'a dyn MateReach,
    band: Band,
    tol: Tol,
}

/// The frame flip that applies an OPPOSED axis sense: the half turn
/// about the mate frame's own local X, which reverses the axis and the
/// handedness of the cross axis while staying proper (det = +1).
fn opposed() -> Affine3<f64> {
    Affine3::from_parts(
        Mat3::from_cols(
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
        ),
        Vec3::new(0.0, 0.0, 0.0),
    )
}

/// **What a caller of [`mate_coset`] answers when the rider on a
/// coincidence asks for its lever.**
enum LeverArm {
    /// The lever, formed: the two mated parts' reach plus the datum's
    /// own terms, over which the rider is decided.
    Formed(f64),
    /// No lever, and no decision: the caller is replaying an edit
    /// whose rider the recording door decided ([`admit_mate`] states
    /// the rule).
    Declined,
}

/// One mate's coset: the relative poses (b's part coordinates into a's)
/// its primitive admits.
///
/// The construction is the same three lines every time — build both
/// mate frames, apply the sense, ride the primitive's own displacement
/// onto the `a` side, and read off the subgroup the primitive leaves.
/// The clocking RIDER is applied here too, because it never stands
/// alone: it modifies its carrier's target frame and cuts its residual.
///
/// Each side's frame is read ONCE ([`super::MateFrame::frame`]): its
/// affine is the placement and its `w` the axis witness, negated
/// exactly for an opposed sense. Every primitive's target keeps that
/// axis — a standoff translates along it and the rider spins about it
/// — so no direction here is read back off a product of matrices,
/// which would be unit only to rounding and a witness to nothing.
///
/// `lever` forms this mate's lever — the two mated parts' reach
/// summed ([`pair_reach`]) plus the datum's own terms
/// ([`Alignment::lever_arm`]) — and is asked at exactly one site: the
/// rider on a coincidence, the one row of the table that levers a
/// decision. Every other row decides on the datum alone, so a caller
/// with no lever in hand (the edit door, [`admit_mate`]) forms none
/// for them, and a caller that holds one already (the fold, which
/// levers its intersections too) hands it in. [`LeverArm::Declined`]
/// is replay's answer, on the rule stated at [`admit_mate`].
fn mate_coset(
    mate: RecipeNodeId,
    alignment: &Alignment,
    lever: impl FnOnce() -> Result<LeverArm, Box<MateFault>>,
    band: Band,
    tol: Tol,
) -> Result<Coset, Box<MateFault>> {
    let frame = |side: MateSide, f: &super::MateFrame| {
        f.frame(tol)
            .map_err(|error| Box::new(MateFault::Frame { mate, side, error }))
    };
    let fa = frame(MateSide::A, &alignment.a)?;
    let fb = frame(MateSide::B, &alignment.b)?.to_affine();
    let (fa, axis) = match alignment.sense {
        AxisSense::Aligned => (fa.to_affine(), fa.w()),
        AxisSense::Opposed => (fa.to_affine() * opposed(), -fa.w()),
    };
    let local_z = Vec3::new(0.0, 0.0, 1.0);
    let spin = |theta: f64| {
        Affine3::from_parts(
            Mat3::rotation_about(local_z, theta),
            Vec3::new(0.0, 0.0, 0.0),
        )
    };
    let (target, subgroup) = match alignment.primitive {
        MatePrimitive::FrameCoincidence => {
            // The table: on frame-coincidence the clocking rider is
            // redundant-or-contradictory, DECIDED. A coincidence has
            // already pinned the roll, so the only clocking it can
            // agree with is zero.
            // The lever is asked HERE and nowhere else in the table:
            // no rider, no ask. A declined lever (replay only) yields
            // the coset with the rider UNDECIDED — the door that
            // recorded the entry decided it.
            if let Some(theta) = alignment.clocking
                && let LeverArm::Formed(arm) = lever()?
            {
                let roll = Measured::Lever(Lever::Roll {
                    radians: theta,
                    arm,
                });
                let sign =
                    geom_core::k_stats::decide("mate_clocking_redundant", roll.margin(), band)
                        .map_err(|diag| {
                            Box::new(MateFault::Indeterminate {
                                mate,
                                diag: Box::new(diag),
                            })
                        })?;
                if sign != geom_core::predicate::Sign::Zero {
                    return Err(Box::new(MateFault::Contradictory {
                        held: mate,
                        added: mate,
                        predicate: "mate_clocking_redundant",
                        clash: roll.clash(),
                    }));
                }
            }
            (fa, Subgroup::Trivial)
        }
        MatePrimitive::Coaxial => match alignment.clocking {
            // The rider cuts the cylindrical residual to translation
            // along the axis — the table's coaxial+clocking row.
            Some(theta) => {
                let target = fa * spin(theta);
                (target, Subgroup::Prismatic { direction: axis })
            }
            None => {
                let point = Point3::origin() + fa.translation;
                (
                    fa,
                    Subgroup::Cylindrical {
                        point,
                        direction: axis,
                    },
                )
            }
        },
        MatePrimitive::PlanarRest { offset } => {
            // The table's static gap, read where the table keeps it
            // (`super::table_gap`), at this arm so a frame refusal
            // still precedes it.
            if let Some(what) = super::table_gap(alignment.primitive, alignment.clocking) {
                return Err(Box::new(MateFault::TableLacks { mate, what }));
            }
            let target = fa * Affine3::translation(local_z * offset);
            (target, Subgroup::Planar { normal: axis })
        }
        MatePrimitive::Clocking => {
            // The table's other static gap, from the same home
            // (`super::table_gap`), which gaps a standalone clocking
            // for every rider: an answer of `None` here is the table
            // contradicting itself, not a mate the table admits.
            let Some(what) = super::table_gap(alignment.primitive, alignment.clocking) else {
                unreachable!(
                    "table_gap admits a standalone clocking, which the table has no row for"
                )
            };
            return Err(Box::new(MateFault::TableLacks { mate, what }));
        }
    };
    Ok(Coset {
        subgroup,
        representative: target * fb.inverse(),
    })
}

/// The coset of the INVERSE relation: `{ x⁻¹ : x ∈ c }`, written as a
/// left coset again by conjugating the subgroup.
///
/// A mate is authored on an ordered pair; the spanning tree may want it
/// the other way round. Inverting the relation rather than re-reading
/// the alignment keeps ONE construction of a mate's meaning.
///
/// The subgroup's directions are transported by the representative's
/// rotation and re-minted under the band ([`derived_direction`]): a
/// proper rotation keeps a witness's length one within rounding, so
/// the mint decides a length within rounding of one and refuses on no
/// document the doors build.
///
/// # Errors
///
/// The frame ladder's own refusal for a transported direction whose
/// length could not be decided ([`derived_direction`]).
fn invert(c: Coset, band: Band) -> Result<Coset, FrameError> {
    let r = c.representative.inverse();
    let dir = |u: UnitVec3<f64>| derived_direction(r.linear * u.get(), "mate_coset_inverse", band);
    let pt = |p: Point3<f64>| r.transform_point(p);
    let subgroup = match c.subgroup {
        Subgroup::Se3 => Subgroup::Se3,
        Subgroup::Trivial => Subgroup::Trivial,
        Subgroup::Empty => Subgroup::Empty,
        Subgroup::Planar { normal } => Subgroup::Planar {
            normal: dir(normal)?,
        },
        Subgroup::Prismatic { direction } => Subgroup::Prismatic {
            direction: dir(direction)?,
        },
        Subgroup::Cylindrical { point, direction } => Subgroup::Cylindrical {
            point: pt(point),
            direction: dir(direction)?,
        },
        Subgroup::Revolute { point, direction } => Subgroup::Revolute {
            point: pt(point),
            direction: dir(direction)?,
        },
    };
    Ok(Coset {
        subgroup,
        representative: r,
    })
}

/// **A direction the fold derives from a witness, re-minted under the
/// run's band**, with the direction door's refusal in the frame
/// ladder's vocabulary — the refusal a mate frame's own axis gets —
/// so a decided ZERO stays a definite refusal and only an in-band
/// length is the escalation: `Degenerate` and an underflowed length
/// are the ladder's `Degenerate` and `UnderflowedLength` at the aim,
/// an in-band length carries its diagnostic, a length that is no
/// number is `NonFiniteLength`. The invariant a caller relies on is
/// that a proper rotation of a witness has length one within
/// rounding, so on a document the doors build the mint never refuses;
/// a refusal is the witness doing its job.
fn derived_direction(
    v: Vec3<f64>,
    site: &'static str,
    band: Band,
) -> Result<UnitVec3<f64>, FrameError> {
    UnitVec3::new(v, site, band).map_err(|error| match error {
        UnitVec3Error::Degenerate => FrameError::Degenerate {
            input: FrameInput::Aim,
            indeterminate: None,
        },
        UnitVec3Error::Escalated(diag) => FrameError::Degenerate {
            input: FrameInput::Aim,
            indeterminate: Some(diag),
        },
        UnitVec3Error::UnderflowedLength => FrameError::UnderflowedLength {
            input: FrameVector::Aim,
        },
        UnitVec3Error::NonFiniteLength => FrameError::NonFiniteLength {
            input: FrameVector::Aim,
        },
    })
}

/// **The per-reference prefix of a mate's admission**, after its two
/// walks: the checks that need a number ([`check_reference`]) on both
/// sides, then the pair as two DISTINCT members
/// ([`MateFault::SelfMate`]) — the one function the solve's first
/// loop and [`admit_mate`] both call, so the two cannot meet these
/// refusals in different orders. The walks themselves stay
/// [`walk_of`]'s, made before this is asked: the loop needs them for
/// the welds whether or not these checks admit the mate. Two COPIES
/// of one pattern are two members and pass: what a mate cannot relate
/// is a member to itself.
fn check_references<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    env: &ParamEnv<f64>,
    mate: RecipeNodeId,
    wa: &Walk,
    wb: &Walk,
) -> Result<(), MateFault> {
    check_reference(doc, env, mate, MateSide::A, wa)?;
    check_reference(doc, env, mate, MateSide::B, wb)?;
    if wa.member == wb.member {
        return Err(MateFault::SelfMate {
            mate,
            instance: wa.member.instance,
        });
    }
    Ok(())
}

/// **The class door of a mate's own admission**: the class inside the
/// vocabulary ([`MateFault::ClassNotAdmitted`]). The class table is
/// the policy (`super::class_admission`); this door enforces its own
/// half of it and nothing more.
fn admit_class(mate: RecipeNodeId, class: super::ContactClass) -> Result<(), Box<MateFault>> {
    if super::class_admission(class) == super::ClassAdmission::NotAdmitted {
        return Err(Box::new(MateFault::ClassNotAdmitted { mate }));
    }
    Ok(())
}

/// **One mate's own admission** — what the solve decides about a mate
/// from its own datum alone, asked of that one mate (`node`, the
/// `Node::Mate` the document holds or is about to hold at `mate`): the
/// two walks ([`walk_of`]), the per-reference prefix
/// ([`check_references`]), the class door ([`admit_class`]) and the
/// coset table ([`mate_coset`]), in the order the solve meets them,
/// with the lever formed through `reach` exactly where the table
/// levers a decision (the rider on a coincidence) and nowhere else.
/// It is the per-mate prefix of the solve: [`solve_with_env`]'s first
/// loop makes the walks and asks [`check_references`], and
/// [`fold_pair`] asks [`admit_class`] and [`mate_coset`] of every
/// mate it folds, adding only what a PAIR needs — the lever formed
/// for every mate, because the fold levers its intersections too.
///
/// The edit door asks this of a mate being inserted, so a mate the
/// solve refuses on its own datum is refused at the insert door
/// (`ASSEMBLY.md` A11 rule 1). The solve records the same fault
/// against the mate whenever it reads the datum; a mate on a pair the
/// fold never reads — two members over one instance — is refused on
/// the datum alone all the same, since which pairs the fold reads is
/// a group fact this door does not decide. It folds nothing and
/// reads no other mate: the relational verdicts — UNDER, a
/// contradiction against ANOTHER mate, an escalation on a fold — are
/// the solve's, because they are facts about a pair, not about a
/// mate. A rider the band decides redundant is admitted, as the solve
/// admits it. What the door does not cover is a state: a mate that
/// COMES to carry one of these faults after insert — a head a rebind
/// or a shrunk pattern strands, a `Part` re-pointed, a snapshot loaded
/// from a doctored or older file — is the solve's at evaluation.
///
/// `env` is the document's own nominal environment, built by the
/// door that asks (the evaluation's arrangement at [`solve_with_env`]:
/// one build per entry, every reader handed it).
///
/// `reach` absent is replay's, and the one decision that needs a lever
/// — the rider on a coincidence — is then DECLINED: the door that
/// recorded the edit decided it over the parts it had in hand, and
/// re-deciding it would need a store replay never holds. Everything
/// decided on the datum alone is decided again. A declined decision
/// leaves nothing false in the document: the datum is what it was, and
/// the next solve decides it again.
///
/// # Errors
///
/// The fault the solve records against the mate for its own datum,
/// unaltered: [`MateFault::Band`] when no band forms; the walk's
/// [`MateFault::DanglingHead`]; [`check_references`]'s; the class
/// door's; and [`mate_coset`]'s — `Frame`, `TableLacks`, the decided
/// contradictory rider or its escalation, and
/// [`MateFault::Unleverable`] where the rider needs a lever the reach
/// cannot form.
pub(crate) fn admit_mate<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    mate: RecipeNodeId,
    node: &Node<P>,
    env: &ParamEnv<f64>,
    reach: Option<&dyn MateReach>,
    tol: Tol,
) -> Result<(), Box<MateFault>> {
    // The door asks this of the mate it is inserting, under the id it
    // minted for it; any other node here is the caller's mistake, not
    // a refusal the mate earned.
    let Node::Mate {
        a,
        b,
        class,
        alignment,
    } = node
    else {
        unreachable!("admit_mate is asked of a mate; node {} is not one", mate.0)
    };
    let band = Band::linear(tol).map_err(|error| Box::new(MateFault::Band { error }))?;
    let wa = walk_of(doc, mate, MateSide::A, a).map_err(Box::new)?;
    let wb = walk_of(doc, mate, MateSide::B, b).map_err(Box::new)?;
    check_references(doc, env, mate, &wa, &wb).map_err(Box::new)?;
    admit_class(mate, *class)?;
    // The two parts are asked in DOCUMENT order, which is the order
    // the fold asks a pair that is a group of its own: its root is
    // the earlier instance and the tree's parent, so a refusal that
    // names the first part not in hand names the same part here.
    let (first, second) = if wa.member.instance <= wb.member.instance {
        (&wa.member, &wb.member)
    } else {
        (&wb.member, &wa.member)
    };
    let lever = || match reach {
        Some(reach) => pair_reach(doc, reach, first, second)
            .map(|parts| LeverArm::Formed(parts + alignment.lever_arm()))
            .map_err(|refusal| Box::new(MateFault::Unleverable { mate, refusal })),
        None => Ok(LeverArm::Declined),
    };
    mate_coset(mate, alignment, lever, band, tol).map(|_| ())
}

/// **The per-pair fold** (A11 rule 1): every mate on the ordered
/// MEMBER pair `(parent, child)`, intersected. The result's
/// representative maps the child MEMBER's part coordinates into the
/// parent MEMBER's — for a pattern-placed member, the coordinates its
/// mate frames are authored in (the master's, which the copy shares
/// key-for-key). The pair's derived offsets are NOT in this coset:
/// they are the pair's static left factor
/// ([`pair_left_factor`]), composed outside the fold, which is what
/// keeps the coset algebra itself unchanged (the rider's rule-1
/// clause).
///
/// Each mate passes its own admission first — the pair door and the
/// coset table, the doors [`admit_mate`] asks of a mate alone — and
/// only then meets the fold.
///
/// # Errors
///
/// The first refusal: a malformed alignment, a table gap, an
/// Indeterminate case split, or the CONTRADICTORY empty intersection —
/// which names both mates, the predicate, and the measured clash.
fn fold_pair<P: crate::ProfilePayload>(
    s: &Solve<'_, P>,
    parent: &Member,
    child: &Member,
    mates: &[PairMate],
) -> Result<Coset, Box<MateFault>> {
    let Solve {
        doc,
        reach,
        band,
        tol,
        ..
    } = *s;
    let mut held = Coset::unconstrained();
    let mut held_mate = None;
    // The fold's lever is the largest of the mates' own: each is the
    // pair's two part reaches plus that mate's datum terms, so it
    // starts at nothing and no constant seeds it. The reaches are
    // asked ONCE per pair, lazily, at the first mate that forms a
    // lever — after that mate's own class and self-mate checks, so a
    // part that does not resolve never pre-empts a refusal the mate
    // earns on its own.
    let mut arm = 0.0_f64;
    let mut parts_reach: Option<f64> = None;
    for pm in mates {
        let mate = pm.mate;
        let Some(Node::Mate {
            class, alignment, ..
        }) = doc.node(mate)
        else {
            continue;
        };
        // The members these two references resolved to, walked once
        // where the pair map was built and carried here.
        let (ha, hb) = (&pm.a.member, &pm.b.member);
        admit_class(mate, *class)?;
        let parts = match parts_reach {
            Some(parts) => parts,
            None => {
                let parts = pair_reach(doc, reach, parent, child)
                    .map_err(|refusal| Box::new(MateFault::Unleverable { mate, refusal }))?;
                parts_reach = Some(parts);
                parts
            }
        };
        // This mate's lever, formed once: the pair's parts plus its
        // own datum terms. The fold's is the largest so far.
        let mate_arm = parts + alignment.lever_arm();
        arm = arm.max(mate_arm);
        let mut coset = mate_coset(
            mate,
            alignment,
            || Ok(LeverArm::Formed(mate_arm)),
            band,
            tol,
        )?;
        // The authored order is `a`'s coordinates from `b`'s; the tree
        // may need the other direction.
        // The transported direction is `a`'s axis carried into `b`'s
        // coordinates, so a refusal is reported at side `a`.
        if (ha, hb) != (parent, child) {
            coset = invert(coset, band).map_err(|error| {
                Box::new(MateFault::Frame {
                    mate,
                    side: MateSide::A,
                    error,
                })
            })?;
        }
        held = match super::coset::intersect(held, coset, band, arm) {
            Ok(next) => next,
            Err(FoldStop::Indeterminate(diag)) => {
                return Err(Box::new(MateFault::Indeterminate { mate, diag }));
            }
            Err(FoldStop::Clash { predicate, clash }) => {
                return Err(Box::new(MateFault::Contradictory {
                    held: held_mate.unwrap_or(mate),
                    added: mate,
                    predicate,
                    clash,
                }));
            }
        };
        if matches!(held.subgroup, Subgroup::Empty) {
            return Err(Box::new(MateFault::Contradictory {
                held: held_mate.unwrap_or(mate),
                added: mate,
                predicate: super::MATE_MEMBER_EMPTY,
                clash: Clash::Structural,
            }));
        }
        held_mate.get_or_insert(mate);
    }
    Ok(held)
}

/// **The two mated parts' reach, summed** — the body terms of the
/// pair's lever ([`Alignment::lever_arm`] states the whole sum). Each
/// member's part is the one its instance stands on: a pattern copy or
/// a transform on the chain moves the part rigidly and changes no
/// reach, so the member's chain is not consulted.
///
/// # Errors
///
/// The first part whose reach is not in hand, in pair order — or a
/// member standing on a node that is not a live instantiate node,
/// which the member walk excludes and this door still names rather
/// than assumes.
fn pair_reach<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    reach: &dyn MateReach,
    parent: &Member,
    child: &Member,
) -> Result<f64, super::LeverRefusal> {
    let of = |instance: RecipeNodeId| {
        let Some(doc_ref) = crate::eval::parts::instantiated(doc, instance) else {
            return Err(super::LeverRefusal::NotAnInstance { node: instance });
        };
        reach
            .reach(&doc_ref)
            .map_err(|refusal| super::LeverRefusal::of(refusal, instance, doc_ref))
    };
    Ok(of(parent.instance)? + of(child.instance)?)
}

/// **The pair's static left factor**: what the members' derived
/// offsets contribute to the child's pose. A placer's map acts in
/// DOCUMENT coordinates — outside the instance's pose, where the
/// evaluation composes it — so the tree's step is
///
/// ```text
/// world_child = (O_child⁻¹ ∘ O_parent) ∘ world_parent ∘ rep
/// ```
///
/// and this answers `O_child⁻¹ ∘ O_parent`, which [`Pose::left`]
/// gathers OUTSIDE the group's frame. `None` when neither reference
/// passes a placer — the factor is then the identity BY CONSTRUCTION,
/// not numerically, so a document with no transform and no pattern
/// between its mates and their instances composes nothing and its
/// solve stays bit-for-bit what it was. No gauge frame is read: the
/// group's frame sits between the two factors, and the evaluation
/// composes it in its own lane.
///
/// The faults a reference's offset can raise are attributed through
/// `mate` — the pair's first mate, whose sides name these members.
fn pair_left_factor<P: crate::ProfilePayload>(
    s: &Solve<'_, P>,
    parent: &Member,
    first: &PairMate,
) -> Result<Option<Affine3<f64>>, Box<MateFault>> {
    // The authored sides for attribution: whichever of the pair the
    // parent member is, the other is the child. The pair's mates all
    // relate these two members, so the FIRST mate's own two
    // references are the ones the offsets are derived from — and its
    // two walks are in hand, so neither is walked a second time.
    let mate = first.mate;
    let ((parent_side, parent_walk), (child_side, child_walk)) = if first.a.member == *parent {
        ((MateSide::A, &first.a), (MateSide::B, &first.b))
    } else {
        ((MateSide::B, &first.b), (MateSide::A, &first.a))
    };
    let Solve { doc, env, band, .. } = *s;
    let op = derived_offset(doc, env, mate, parent_side, parent_walk, band)?;
    let oc = derived_offset(doc, env, mate, child_side, child_walk, band)?;
    Ok(match (oc, op) {
        (None, None) => None,
        (Some(oc), Some(op)) => Some(oc.inverse() * op),
        (Some(oc), None) => Some(oc.inverse()),
        (None, Some(op)) => Some(op),
    })
}

/// **The document's solve** (D-4 + D-5): every group's spanning tree
/// from its root, every tree pair folded and required DETERMINED,
/// every other mate recorded DECLARING, and every instance's pose
/// composed outward from the root.
///
/// Total by construction — a refusing group records its fault against
/// its own mates and instances and leaves every other group solved.
///
/// `reach` is the one geometric read the solve makes: each mated
/// part's own extent, asked lazily per pair and entering only as the
/// lever a parallelism verdict is decided over. The evaluation hands
/// its own part cache (`eval::mate_reach` is the door every other
/// caller builds one through), so a mated part is evaluated exactly
/// once and the instantiate node hits the cache afterwards.
///
/// **One nominal environment per solve.** Every number the solve
/// reads out of the recipe — a pattern's count, a `Part`'s index, the
/// slots a derived offset is composed from — is evaluated at the
/// document's own parameter bindings, under no box and no seed: the
/// solve is a fact about the document, not about any run over it.
/// That environment is built here, once, and handed to every reader
/// (`check_reference`, `derived_offset` and what they call) as a
/// parameter, so "the document's own" is decided at one site and the
/// readers cannot be given different ones. An evaluation, which has
/// already built that same environment for its own f64-pinned
/// readers, hands it in through [`solve_with_env`] instead of paying
/// for a second.
pub fn solve_document<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    reach: &dyn MateReach,
    tol: Tol,
) -> SolvedPoses {
    let env = doc.param_env::<f64>();
    solve_with_env(doc, &env, reach, tol)
}

/// [`solve_document`] over an environment the caller already holds —
/// the evaluation's `LaneEnv::nominal`, which is the document's own
/// by that field's contract. `env` must be that environment: the
/// solve answers about the document, and a boxed or seeded one would
/// make it answer about a run.
pub(crate) fn solve_with_env<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    env: &ParamEnv<f64>,
    reach: &dyn MateReach,
    tol: Tol,
) -> SolvedPoses {
    let mut out = SolvedPoses::empty(doc.id(), tol);
    let band = match Band::linear(tol) {
        Ok(band) => band,
        Err(error) => {
            // No band, no decisions: every mate and instance refuses
            // with the same typed cause rather than any of them
            // guessing.
            let fault = MateFault::Band { error };
            for &id in doc.order() {
                if matches!(
                    doc.node(id),
                    Some(Node::Mate { .. } | Node::InstantiatePart { .. })
                ) {
                    out.faults.insert(id, fault.clone());
                }
            }
            return out;
        }
    };
    let s = Solve {
        doc,
        env,
        reach,
        band,
        tol,
    };
    // Mates by the unordered MEMBER pair they relate, document order
    // within a pair. The member — not just its instance — is the key:
    // mates to sibling copies of one pattern are DIFFERENT pairs, so
    // the second of them is a non-tree edge (declaring) rather than a
    // fold-mate of the first, which is the rider's loop clause. A
    // self-mate is one MEMBER named on both sides; two distinct copies
    // of one pattern are a pair like any other (their edge just joins
    // no groups, both ends standing on the same instance).
    let mut by_pair: BTreeMap<(Member, Member), Vec<PairMate>> = BTreeMap::new();
    let mut broken: Vec<(RecipeNodeId, MateFault)> = Vec::new();
    // ONE walk per reference, here: the members below, the pair
    // keying, the group welds and every derived offset the fold
    // needs are all read off these two walks.
    let read = read_mates(doc);
    for (id, walked) in &read {
        let id = *id;
        out.roles.insert(id, MateRole::Declaring);
        let (wa, wb) = match walked {
            Ok(pair) => pair,
            Err(fault) => {
                broken.push((id, fault.clone()));
                continue;
            }
        };
        // **The checks that need a number, at the site the solve
        // reads the reference** — for EVERY reference of EVERY live
        // mate, not only the ones a tree edge's offset happens to
        // derive — and the pair as two distinct members: the same
        // prefix the edit door asks (`check_references`). The walk
        // itself evaluated nothing, so this is where the name meets a
        // count.
        if let Err(fault) = check_references(doc, env, id, wa, wb) {
            broken.push((id, fault));
            continue;
        }
        // A mate across gauges places nothing (A11 (2)): it stays
        // DECLARING, and the at-rest gate verifies it.
        if !places(doc, wa.member.instance, wb.member.instance) {
            continue;
        }
        let (ha, hb) = (wa.member.clone(), wb.member.clone());
        by_pair
            .entry(unordered(ha, hb))
            .or_default()
            .push(PairMate {
                mate: id,
                a: wa.clone(),
                b: wb.clone(),
            });
    }
    for (mate, fault) in broken {
        out.roles.insert(mate, MateRole::Refused);
        out.faults.insert(mate, fault);
    }
    for group in groups_welded_by(doc, &welds(doc, &read)) {
        let (root, cause) = root_and_cause(doc, &group);
        if let Some(cause) = cause {
            out.unplaced.insert(root, cause);
        }
        match solve_group(&s, &group, root, &by_pair) {
            Ok(solved) => {
                // The ROOT is the group's, and every instance in it
                // is keyed by that root whether or not a pair placed
                // it. A mate this solve refused still WELDS its
                // group (the partition is structural), so an
                // instance the spanning tree could not reach sits at
                // the group's frame instead of being given a pose
                // nothing decided.
                for &instance in &group {
                    out.root.insert(instance, root);
                }
                // A placed group's further offsets are statements the
                // solve checks (A11 (2)); an unplaced group's state
                // nothing about where anything sits.
                if cause.is_none() {
                    for (instance, fault) in check_offsets(&s, &group, root, &solved.pose) {
                        out.faults.insert(instance, fault);
                    }
                }
                out.pose.extend(solved.pose);
                for (mate, role) in solved.roles {
                    out.roles.insert(mate, role);
                }
            }
            Err(fault) => {
                // The refusal reaches exactly what it stops: every
                // instance in the group (which now has no pose) and
                // every mate holding it together.
                for &instance in &group {
                    out.root.insert(instance, root);
                    out.faults
                        .entry(instance)
                        .or_insert_with(|| (*fault).clone());
                }
                for (pair, mates) in &by_pair {
                    if group.contains(&pair.0.instance) {
                        for pm in mates {
                            out.roles.insert(pm.mate, MateRole::Refused);
                            out.faults
                                .entry(pm.mate)
                                .or_insert_with(|| (*fault).clone());
                        }
                    }
                }
            }
        }
    }
    out
}

/// **One mate on a member pair, with both its references' walks.**
///
/// The walks are the solve's one reading of those two references: the
/// members they resolved to key the pair, and the chains they carry
/// are what [`pair_left_factor`] folds — so nothing below re-walks a
/// reference the pair map already resolved.
struct PairMate {
    /// The mate node.
    mate: RecipeNodeId,
    /// The `a` side's walk, as authored.
    a: Walk,
    /// The `b` side's walk, as authored.
    b: Walk,
}

/// One group's solved poses and mate roles.
struct GroupSolve {
    pose: BTreeMap<RecipeNodeId, Pose>,
    roles: BTreeMap<RecipeNodeId, MateRole>,
}

fn unordered<T: Ord>(x: T, y: T) -> (T, T) {
    if x <= y { (x, y) } else { (y, x) }
}

/// One group: the deterministic spanning tree from the root, each
/// tree pair folded and required DETERMINED (A11 rule 4), the poses
/// composed outward (rule 5).
///
/// The tree spans INSTANCES, but its edges are member pairs: between
/// two instances the tree takes the first member pair in key order and
/// every other pair between them — a sibling copy's mate, say — is a
/// non-tree edge and stays DECLARING. A pair standing on one instance
/// twice (two copies of the same pattern mated to each other) can
/// never be a tree edge at all — the pattern already determined both
/// ends — so it stays declaring the same way.
fn solve_group<P: crate::ProfilePayload>(
    s: &Solve<'_, P>,
    group: &[RecipeNodeId],
    root: RecipeNodeId,
    by_pair: &BTreeMap<(Member, Member), Vec<PairMate>>,
) -> Result<GroupSolve, Box<MateFault>> {
    let position: BTreeMap<RecipeNodeId, usize> =
        group.iter().enumerate().map(|(i, &id)| (id, i)).collect();
    let mut neighbours: BTreeMap<RecipeNodeId, Vec<RecipeNodeId>> = BTreeMap::new();
    // The tree edge between two instances: the FIRST member pair
    // relating them, in pair-key order (deterministic). Every other
    // pair between the same two is a non-tree edge and stays
    // declaring.
    let mut edge_of: BTreeMap<(RecipeNodeId, RecipeNodeId), (&Member, &Member)> = BTreeMap::new();
    for ((x, y), _) in by_pair
        .iter()
        .filter(|((x, _), _)| position.contains_key(&x.instance))
    {
        if x.instance == y.instance {
            continue;
        }
        neighbours.entry(x.instance).or_default().push(y.instance);
        neighbours.entry(y.instance).or_default().push(x.instance);
        edge_of
            .entry(unordered(x.instance, y.instance))
            .or_insert((x, y));
    }
    for list in neighbours.values_mut() {
        list.sort_by_key(|id| position.get(id).copied().unwrap_or(usize::MAX));
        list.dedup();
    }
    // Each pose as its two factors around the group's frame
    // ([`Pose`]): the placer offsets outside, the representatives
    // inside.
    let mut poses: BTreeMap<RecipeNodeId, (Option<Affine3<f64>>, Affine3<f64>)> = BTreeMap::new();
    poses.insert(root, (None, Affine3::identity()));
    let mut pose: BTreeMap<RecipeNodeId, Pose> = BTreeMap::new();
    pose.insert(
        root,
        Pose {
            left: None,
            right: Frame::IDENTITY,
        },
    );
    // Only THIS group's mates get a role here: a role written for
    // another group's mate would race that group's own answer,
    // and which one won would depend on document order.
    let mut roles: BTreeMap<RecipeNodeId, MateRole> = BTreeMap::new();
    for (pair, mates) in by_pair {
        if position.contains_key(&pair.0.instance) {
            for pm in mates {
                roles.insert(pm.mate, MateRole::Declaring);
            }
        }
    }
    let mut queue = VecDeque::from([root]);
    let mut visited: BTreeSet<RecipeNodeId> = BTreeSet::from([root]);
    while let Some(parent) = queue.pop_front() {
        for &child in neighbours.get(&parent).into_iter().flatten() {
            if !visited.insert(child) {
                continue;
            }
            let (x, y) = edge_of[&unordered(parent, child)];
            let (pm, cm) = if x.instance == parent { (x, y) } else { (y, x) };
            let mates = &by_pair[&(x.clone(), y.clone())];
            let coset = fold_pair(s, pm, cm, mates)?;
            if !coset.subgroup.is_determined() {
                // A11 rule 4: a tree edge that does not determine
                // refuses, naming the residual and its parameters.
                return Err(Box::new(MateFault::Under {
                    mate: mates[0].mate,
                    parent,
                    child,
                    residual: coset.subgroup,
                }));
            }
            let (parent_left, parent_right) = poses[&parent];
            let right = parent_right * coset.representative;
            let left = match (pair_left_factor(s, pm, &mates[0])?, parent_left) {
                (None, left) => left,
                (Some(factor), None) => Some(factor),
                (Some(factor), Some(left)) => Some(factor * left),
            };
            poses.insert(child, (left, right));
            pose.insert(
                child,
                Pose {
                    left: left.map(Frame::from_affine),
                    right: Frame::from_affine(right),
                },
            );
            for pm in mates {
                roles.insert(pm.mate, MateRole::Determining);
            }
            queue.push_back(child);
        }
    }
    Ok(GroupSolve { pose, roles })
}

/// **A placed group's checked offsets** (A11 (2)): every member other
/// than the root that carries an offset states where it sits, and the
/// statement is verified against the solve — never trusted and never
/// ignored. A member that disagrees, or whose statement cannot be
/// decided, is faulted itself; the rest of the group stands.
///
/// The member sits at `G ∘ offset` by its own statement and at
/// `left ∘ G ∘ root_offset ∘ right` by the solve, `G` its gauge's
/// frame, so the two agree when `offset⁻¹ ∘ G⁻¹ ∘ left ∘ G ∘
/// root_offset ∘ right` is the identity, decided as a coset member of
/// the trivial subgroup over the member part's reach. With no placer
/// on the path (`left` absent) the gauge cancels and is not read; a
/// placer's offset is a map in document coordinates, so only then is
/// the gauge frame read, at the document's own parameters, as every
/// number the solve reads is.
fn check_offsets<P: crate::ProfilePayload>(
    s: &Solve<'_, P>,
    group: &[RecipeNodeId],
    root: RecipeNodeId,
    poses: &BTreeMap<RecipeNodeId, Pose>,
) -> Vec<(RecipeNodeId, MateFault)> {
    let Solve {
        doc,
        env,
        reach,
        band,
        ..
    } = *s;
    let offset_of = |id: RecipeNodeId| match doc.node(id) {
        Some(Node::InstantiatePart {
            offset: Some(offset),
            ..
        }) => Some(offset),
        _ => None,
    };
    let unchecked = |instance, cause| MateFault::OffsetUnchecked {
        instance,
        cause: Box::new(cause),
    };
    let placement = |instance, node, error: crate::eval::NodeErrorKind| {
        unchecked(
            instance,
            OffsetCheck::Placement {
                node,
                error: error.into(),
            },
        )
    };
    let mut out = Vec::new();
    for &instance in group {
        if instance == root {
            continue;
        }
        let (Some(stated), Some(pose)) = (offset_of(instance), poses.get(&instance)) else {
            continue;
        };
        let check = || -> Result<(), MateFault> {
            let stated = stated
                .eval(env, band)
                .map_err(|e| placement(instance, instance, e))?;
            let root_offset = match offset_of(root) {
                Some(o) => o
                    .eval(env, band)
                    .map_err(|e| placement(instance, root, e))?,
                None => Affine3::identity(),
            };
            let left = match pose.left {
                None => Affine3::identity(),
                Some(left) => {
                    let gauge = doc.node(instance).and_then(Node::gauge_ref);
                    let g = gauge_frame(doc, gauge, env, band)
                        .map_err(|(node, e)| placement(instance, node, e))?
                        .affine();
                    g.inverse() * left.affine::<f64>() * g
                }
            };
            let solved = left * root_offset * pose.right.affine::<f64>();
            let arm = crate::eval::parts::instantiated(doc, instance)
                .ok_or(super::LeverRefusal::NotAnInstance { node: instance })
                .and_then(|doc_ref| {
                    reach
                        .reach(&doc_ref)
                        .map_err(|r| super::LeverRefusal::of(r, instance, doc_ref))
                })
                .map_err(|refusal| unchecked(instance, OffsetCheck::Unleverable(refusal)))?;
            super::coset::trivial_member(stated.inverse() * solved, band, arm).map_err(|stop| {
                match stop {
                    FoldStop::Clash { predicate, clash } => MateFault::OffsetDisagrees {
                        instance,
                        root,
                        predicate,
                        clash,
                    },
                    FoldStop::Indeterminate(diag) => {
                        unchecked(instance, OffsetCheck::Indeterminate(diag))
                    }
                }
            })
        };
        if let Err(fault) = check() {
            out.push((instance, fault));
        }
    }
    out
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::ErrorTextReading;

    const SITE: &str = "solve_test_direction";

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// **A derived direction refuses in the frame ladder's own
    /// vocabulary, and a decided zero is not an escalation.** A length
    /// inside `(ε, Kε)` is the in-band escalation carrying its
    /// diagnostic under the site's name; one the band calls zero is
    /// the definite `Degenerate` with no diagnostic; one that
    /// underflowed the format, or is no number, is that arm; a proper
    /// rotation of a witness mints.
    #[test]
    fn a_derived_direction_refuses_as_the_frame_ladder_does() {
        let eps = Tol::witness().eps();
        let in_band = derived_direction(Vec3::new(3.0 * eps, 0.0, 0.0), SITE, band()).unwrap_err();
        let FrameError::Degenerate {
            input: FrameInput::Aim,
            indeterminate: Some(diag),
        } = in_band
        else {
            panic!("an in-band length escalates with its diagnostic: {in_band:?}");
        };
        assert_eq!(diag.predicate, Some(SITE));
        assert!(
            matches!(diag.margin.diagnostic_f64_for_error_text(), ErrorTextReading::Value(m) if (m - 3.0 * eps).abs() <= eps * 1e-9)
        );
        assert_eq!(
            derived_direction(Vec3::new(0.5 * eps, 0.0, 0.0), SITE, band()).unwrap_err(),
            FrameError::Degenerate {
                input: FrameInput::Aim,
                indeterminate: None,
            }
        );
        assert_eq!(
            derived_direction(Vec3::new(f64::NAN, 0.0, 0.0), SITE, band()).unwrap_err(),
            FrameError::NonFiniteLength {
                input: FrameVector::Aim,
            }
        );
        assert_eq!(
            derived_direction(Vec3::new(1e-200, 0.0, 0.0), SITE, band()).unwrap_err(),
            FrameError::UnderflowedLength {
                input: FrameVector::Aim,
            }
        );
        let axis = UnitVec3::new(Vec3::new(1.0, 2.0, -3.0), SITE, band()).unwrap();
        let turned = Mat3::rotation_about(Vec3::new(0.3, -0.7, 0.2), 1.234) * axis.get();
        let minted = derived_direction(turned, SITE, band()).unwrap().get();
        assert!((minted - turned).norm() <= 4.0 * f64::EPSILON);
    }
}
