//! **The mate solve** — partitions, groups, and the constructive
//! placement (ASM-R2a D-2/D-3/D-4/D-5; A9/A11).
//!
//! Everything here is recipe data plus decided predicates over the
//! sides' RESOLVED frames — the two reads `ASSEMBLY.md` A11 rule 5
//! states cross through one door, [`MateReach`]: each mated part's
//! extent, the lever, and a face base's pose, resolved before
//! the frame is read ([`resolve_side`]) — and nothing derived is
//! stored beside the DAG. The entry points, in the order the layers
//! use them:
//!
//! - [`relative_freedom_components`] — A9's partition, over reads and
//!   gauge references (so mates couple components).
//! - [`groups`] — A11's placement groups, the finer partition over
//!   instances alone, each with its document-order-first ROOT.
//! - [`solve_document`] — the per-pair coset fold along a deterministic
//!   spanning tree, yielding every instance's pose around its group's
//!   frame, every group's space, and every mate's role, at `f64`.
//! - `solve_with_env` — that fold at an evaluation's own scalar and
//!   environment, asked once per run, its decisions kept per mate.
//! - [`solve_document_at`] — that fold at a run's scalar for a caller
//!   outside an evaluation that holds the run's environment.
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

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Mutex;

use geom_core::k_stats::{Detached, detached, splice};
use geom_core::linalg::frame::{FrameError, FrameInput, FrameVector};
use geom_core::linalg::{Affine3, Mat3, OrthoFrame, Point3, UnitVec3, UnitVec3Error, Vec3};
use geom_core::predicate::Band;
use geom_core::{Decide, Real, Tol};

use super::coset::{Arm, Coset, FoldStop, Measured, PoseSymmetry, Subgroup};
use super::member::{Member, Walk, check_reference, derived_offset, walk_of};
use super::reach::MateReach;
use super::{
    Alignment, AxisSense, Clash, FaceRefusal, FrameBase, MateFault, MateFrame, MatePrimitive,
    MateRef, MateSide, Refuted, SideRead,
};
use crate::doc::Doc;
use crate::eval::NodeRefusal;
use crate::expr::VarEnv;
use crate::node::{Node, RecipeNodeId};
use crate::placement::{Frame, Motion};

/// **The scalar a mate solve runs at** (`ASSEMBLY.md` A11 (5)): the
/// evaluation's own — `f64` for the nominal solve every door outside an
/// evaluation asks, a dual for a seed run, an interval for a box run.
/// Every branch inside the solve is a named predicate decided at this
/// scalar through [`Decide`] (a dual on its value channel, so a seed
/// never decides one; an interval definitely or by escalating).
///
/// What a scalar answers here beyond deciding is per scalar, like
/// `eval`'s `SectionScalar`. No pose value is derived from
/// [`Self::quoted`] or [`Self::upper`]. `quoted` feeds payloads only.
/// `upper` forms the arm ([`Arm::of`]), so it feeds the lever's own
/// refusal branches — `LeverRefusal::OutOfRange` where the arm is
/// formed, `Unleverable` where it is asked to decide
/// ([`Arm::decides_over`]) — and there an over-stated arm refuses
/// sooner, never wrongly.
pub trait SolveScalar: Decide {
    /// **The number a refusal quotes for a measurement made at this
    /// scalar** — a [`Clash`]'s length or residual, which the fault's
    /// payload carries as `f64`. The value itself at `f64`; a dual's
    /// value channel, which is the `f64` lane's number; an interval's
    /// midpoint. Error text and payloads only: the measurement was
    /// decided at the scalar, through its margin, before this is read.
    fn quoted(self) -> f64;

    /// **An `f64` upper bound of a length measured at this scalar** —
    /// a side's `‖origin‖` term of the lever, which, like the parts'
    /// reach, only ever needs to over-state: the length itself at
    /// `f64`; a dual's value channel, the `f64` lane's number; an
    /// interval's `hi`, read through `Bounds` at the seam as the
    /// reach's is. It enters only the arm, and through it the lever's
    /// refusals.
    fn upper(self) -> f64;

    /// **Whether `map` is the stored identity, bit for bit** — the
    /// identity a [`crate::placement::Frame`] marks by its bits
    /// ([`crate::placement::Motion`]), which composes with no
    /// arithmetic. Only the `f64` lane stores a pose as a frame, so only
    /// it answers `true`; an analysis lane's map composes as a map.
    fn is_stored_identity(map: &Affine3<Self>) -> bool;

    /// **The residual an UNDER refusal names, at `f64`**
    /// ([`MateFault::Under`]): the subgroup itself at `f64`, and
    /// elsewhere its parameters quoted ([`Self::quoted`]) and each
    /// direction re-minted ([`quoted_residual`]). Total: the quote
    /// cannot refuse, so every lane's UNDER is an UNDER.
    fn quoted_residual(g: Subgroup<Self>) -> Subgroup;

    /// **The solve's two-argument arctangent** — the angle the coset
    /// fold solves for where two rotation constraints meet, read only
    /// through its sine and cosine, and asked only of two radii decided
    /// nonzero, never at the origin. At `f64` the platform's `atan2`,
    /// the one the nominal solve has always read; a dual's value
    /// channel reads that same one, so its value is the `f64` lane's
    /// bits and its tangent is the dual's own; an interval's is an
    /// enclosure on a branch its arguments do not cross.
    fn solve_atan2(y: Self, x: Self) -> Self;
}

impl SolveScalar for f64 {
    fn quoted(self) -> f64 {
        self
    }

    fn upper(self) -> f64 {
        self
    }

    fn is_stored_identity(map: &Affine3<f64>) -> bool {
        Frame::from_affine(*map).is_identity_bits()
    }

    fn quoted_residual(g: Subgroup) -> Subgroup {
        g
    }

    fn solve_atan2(y: f64, x: f64) -> f64 {
        f64::atan2(y, x)
    }
}

/// The K-telemetry scalar is `f64` wearing a counter: it quotes and
/// angles as `f64` does, and keeps no frame.
#[cfg(feature = "probe")]
impl SolveScalar for geom_core::Probe {
    fn quoted(self) -> f64 {
        self.0
    }

    fn upper(self) -> f64 {
        self.0
    }

    fn is_stored_identity(_map: &Affine3<Self>) -> bool {
        false
    }

    fn quoted_residual(g: Subgroup<Self>) -> Subgroup {
        quoted_residual(g)
    }

    fn solve_atan2(y: Self, x: Self) -> Self {
        geom_core::Probe(f64::atan2(y.0, x.0))
    }
}

/// The funnel name a quoted residual's direction is re-minted under
/// ([`quoted_residual`]).
const MATE_RESIDUAL_QUOTE: &str = "mate_residual_quote";

/// **A residual subgroup quoted at `f64`** — every point and direction
/// through [`SolveScalar::quoted`], each direction re-minted: the
/// implementation every analysis scalar's
/// [`SolveScalar::quoted_residual`] shares.
///
/// A quote decides nothing — the run decided each direction at its own
/// scalar — so the mint asks only that the quoted vector have a length,
/// under the narrowest band the format admits, never the run's (whose
/// escalate threshold a unit vector meets only where `Kε < 1`). Every
/// direction a run minted has one: its value channel is a unit vector,
/// and its enclosure excludes the origin, so some coordinate of the
/// enclosure has one sign, and the midpoint keeps it.
pub(crate) fn quoted_residual<T: SolveScalar>(g: Subgroup<T>) -> Subgroup {
    let band = Band::new(f64::MIN_POSITIVE, 2.0 * f64::MIN_POSITIVE)
        .unwrap_or_else(|error| unreachable!("the format's narrowest band is a band: {error}"));
    let dir = |u: UnitVec3<T>| {
        UnitVec3::new(u.get().map(T::quoted), MATE_RESIDUAL_QUOTE, band).unwrap_or_else(|error| {
            unreachable!(
                "a direction minted at the run's scalar quotes to a finite vector with a \
                 coordinate of one sign, so a length the format holds: {error}"
            )
        })
    };
    let pt = |p: Point3<T>| p.map(T::quoted);
    match g {
        Subgroup::Se3 => Subgroup::Se3,
        Subgroup::Trivial => Subgroup::Trivial,
        Subgroup::Empty => Subgroup::Empty,
        Subgroup::Planar { normal } => Subgroup::Planar {
            normal: dir(normal),
        },
        Subgroup::Prismatic { direction } => Subgroup::Prismatic {
            direction: dir(direction),
        },
        Subgroup::Cylindrical { point, direction } => Subgroup::Cylindrical {
            point: pt(point),
            direction: dir(direction),
        },
        Subgroup::Revolute { point, direction } => Subgroup::Revolute {
            point: pt(point),
            direction: dir(direction),
        },
        Subgroup::Spherical { point } => Subgroup::Spherical { point: pt(point) },
        Subgroup::Parallel { direction } => Subgroup::Parallel {
            direction: dir(direction),
        },
        Subgroup::Translation => Subgroup::Translation,
        Subgroup::PlaneTranslation { normal } => Subgroup::PlaneTranslation {
            normal: dir(normal),
        },
    }
}

/// **Which space a value lives in** (D10, Spaces): the world's kind, or
/// the kind of a loose copy, the one spelling of that fact every reader
/// of it is handed. Decided from the recipe: a placement's copy takes
/// its target's space once its bundle pins it, and a copy whose bundle
/// pins less than its body needs is a root of its own.
///
/// Until constructions are placed, every construction that reads no
/// body is of the world's kind, the one kind every such construction
/// shares today (the named interim of INTENT stage 3 B).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Space {
    /// The world's kind: the copy is pinned, transitively, against the
    /// world, or the value is a construction of the document's one kind.
    World,
    /// The kind of a loose copy: the placement whose bundle pins less
    /// than its body needs, which everything pinned to it shares.
    Loose {
        /// The loose copy's placement.
        root: RecipeNodeId,
    },
}

impl Space {
    /// The loose copy whose kind this is, `None` for the world's.
    #[must_use]
    pub fn loose(self) -> Option<RecipeNodeId> {
        match self {
            Self::World => None,
            Self::Loose { root } => Some(root),
        }
    }
}

/// **A copy's solved pose, decomposed** around its anchor's frame: the
/// copy sits at `left ∘ F ∘ right`, where `F` is the frame of the
/// placement its bundles chain back to: the world pose a bundle against
/// the world pins, or the identity for a loose copy.
///
/// `right` is the composed coset representatives along the chain of
/// bundles from the anchor; `left` is the composed derived offsets of
/// the placers (transforms, pattern copies) the chain's mates read
/// through, `None` when no placer is on the path, which is the identity
/// BY CONSTRUCTION. A placer's offset is a map in document coordinates,
/// so it composes OUTSIDE the anchor's frame.
///
/// At the solve's scalar: a seed run's pose carries its tangent and a
/// box run's encloses the pose over the box.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Pose<T: Real> {
    /// The composed placer offsets, outside the anchor's frame.
    pub(crate) left: Option<Affine3<T>>,
    /// The composed representatives, inside it.
    pub(crate) right: Affine3<T>,
}

/// The document's solved poses: one pose per placement (D10: an
/// instance is a placement), each pinned by its own bundle.
///
/// A solve is a solve OF a document, and it says so: `document` is the
/// id [`solve_document`] read, and [`SolvedPoses::placement`] — the one
/// door that takes a `Doc` back — refuses a mispairing with it (DI3).
/// No `Default`, for that reason, and no `Clone`: the evaluation's
/// solve carries each placement's recording
/// ([`SolvedPoses::take_recordings`]), which has one home.
///
/// At the scalar the solve ran at: `f64` — the default — from
/// [`solve_document`], the evaluation's own from [`solve_with_env`].
/// Only the poses and frames are at that scalar; the spaces are the
/// document's structure.
#[derive(Debug)]
pub struct SolvedPoses<T: Real = f64> {
    /// **Which document this is a solve OF** (DI3).
    document: crate::ident::DocumentId,
    /// Each live placement's pose, decomposed around its anchor's frame
    /// ([`Pose`]). An anchor's own entry is the identity, bit-exactly.
    pose: BTreeMap<RecipeNodeId, Pose<T>>,
    /// Each live placement's anchor: the placement its chain of
    /// bundles starts at.
    anchor: BTreeMap<RecipeNodeId, RecipeNodeId>,
    /// Each world anchor's frame: the world pose its bundle pins. An
    /// anchor absent here is loose, at the identity of its own space.
    frame: BTreeMap<RecipeNodeId, Motion<T>>,
    /// Per-placement refusals: a placement whose bundle refused, and
    /// every placement chained to it, which consequently has no pose.
    faults: BTreeMap<RecipeNodeId, MateFault>,
    /// **What the solve decided, by the placement whose bundle each
    /// decision is about** ([`Record`]), in decision order — filled by
    /// the evaluation's solve ([`solve_with_env`]) and taken once by
    /// that placement's own node, which splices it into its frame.
    recordings: Mutex<BTreeMap<RecipeNodeId, Vec<Detached>>>,
}

/// **Why the nominal world-pose door has no pose** for a placement
/// ([`SolvedPoses::placement`]).
#[derive(Debug, Clone, PartialEq)]
pub enum PoseRefusal {
    /// The solve refused for the placement.
    Mate(Box<MateFault>),
    /// The copy is loose, or pinned to a loose copy, so it has no world
    /// pose — only its pose in the loose copy's space
    /// ([`SolvedPoses::relative`]).
    Loose {
        /// The placement asked about.
        placement: RecipeNodeId,
        /// The loose copy whose space it lives in.
        root: RecipeNodeId,
    },
}

impl crate::spoken::Say for PoseRefusal {
    fn say(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        by: crate::spoken::Speaker<'_>,
    ) -> core::fmt::Result {
        use crate::spoken::Said;
        match self {
            Self::Mate(fault) => write!(f, "{}", Said(&**fault, by)),
            Self::Loose { placement, root } => {
                if placement == root {
                    write!(
                        f,
                        "{} has no world pose: its bundle pins less than its body needs, so \
                         it is a loose copy",
                        by.node(*placement)
                    )?;
                } else {
                    write!(
                        f,
                        "{} has no world pose: it is pinned to {}, a loose copy",
                        by.node(*placement),
                        by.node(*root)
                    )?;
                }
                write!(
                    f,
                    ". {}",
                    crate::sentence::Recourse("pin the loose copy against the world, or against a copy that is")
                )
            }
        }
    }
}

/// The refusal where no document is at hand: each node by its tag.
impl core::fmt::Display for PoseRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        crate::spoken::Say::say(self, f, crate::spoken::Speaker::TAG)
    }
}

impl PoseRefusal {
    /// **The refusal as the frame holding the solved document says it**:
    /// each node as `doc` holds it now ([`crate::Doc::spoken`]).
    #[must_use]
    pub fn spoken<P: crate::ProfilePayload>(&self, doc: &crate::doc::Doc<P>) -> String {
        crate::spoken::spoken_by(self, doc)
    }
}

impl core::error::Error for PoseRefusal {}

impl<T: Real> SolvedPoses<T> {
    /// An empty solve OF `document`.
    fn empty(document: crate::ident::DocumentId) -> Self {
        Self {
            document,
            pose: BTreeMap::new(),
            anchor: BTreeMap::new(),
            frame: BTreeMap::new(),
            faults: BTreeMap::new(),
            recordings: Mutex::new(BTreeMap::new()),
        }
    }

    /// **The solve's decisions about `placement`'s bundle, taken out**
    /// — every recording the solve kept for it, in decision order, for
    /// the placement's node to splice into its own frame. A second take
    /// answers nothing: each recording has one home.
    pub(crate) fn take_recordings(&self, placement: RecipeNodeId) -> Vec<Detached> {
        self.recordings
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .remove(&placement)
            .unwrap_or_default()
    }

    /// **Which document this solve is of** (DI3).
    pub fn document(&self) -> crate::ident::DocumentId {
        self.document
    }

    /// A placement's recorded fault, if the solve refused for it.
    pub fn fault(&self, node: RecipeNodeId) -> Option<&MateFault> {
        self.faults.get(&node)
    }

    /// **The space a placement's copy lives in**, `None` if the node is
    /// not a live placement or the solve refused for it.
    pub fn space(&self, placement: RecipeNodeId) -> Option<Space> {
        if self.faults.contains_key(&placement) {
            return None;
        }
        let anchor = *self.anchor.get(&placement)?;
        Some(if self.frame.contains_key(&anchor) {
            Space::World
        } else {
            Space::Loose { root: anchor }
        })
    }

    /// The decomposed pose the evaluation composes its anchor's frame
    /// into.
    pub(crate) fn pose(&self, placement: RecipeNodeId) -> Option<Pose<T>> {
        self.pose.get(&placement).copied()
    }

    /// The frame of `placement`'s anchor at the solve's scalar: the
    /// world pose its anchor's bundle pins, the identity in a loose
    /// copy's space, `None` where the solve gave the placement no pose.
    pub(crate) fn frame(&self, placement: RecipeNodeId) -> Option<Motion<T>>
    where
        Motion<T>: Copy,
    {
        if self.faults.contains_key(&placement) {
            return None;
        }
        let anchor = self.anchor.get(&placement)?;
        Some(self.frame.get(anchor).copied().unwrap_or(Motion::Identity))
    }
}

/// The nominal solve's doors: its poses read as the frames a document
/// stores.
impl SolvedPoses {
    /// **A copy's pose in its anchor's space**: where it sits when the
    /// anchor's frame is the identity — in a loose copy's space, the
    /// pose the copy is evaluated at.
    pub fn relative(&self, placement: RecipeNodeId) -> Option<Frame> {
        let pose = self.pose.get(&placement)?;
        let left = pose.left.map_or(Motion::Identity, marked);
        Some(Frame::from_motion(left.compose(marked(pose.right))))
    }

    /// **The copy's world placement, at the document's own parameters**:
    /// its anchor's frame composed into its solved pose ([`Pose`]). A
    /// copy pinned against the world by one coincidence of its own frame
    /// returns the world side's frame bit for bit.
    ///
    /// `doc` must be the document this solve is OF: the pairing is
    /// CHECKED here (DI3), the solve carrying the id it was built from.
    ///
    /// # Errors
    ///
    /// [`PoseRefusal::Mate`] holding [`MateFault::PosesOfAnotherDocument`]
    /// when `doc` is not the document this solve is of, or the
    /// placement's own refusal when it did not solve;
    /// [`PoseRefusal::Loose`] when the copy is loose or pinned to one.
    pub fn placement<P>(
        &self,
        doc: &Doc<P>,
        placement: RecipeNodeId,
    ) -> Result<Frame, PoseRefusal> {
        if let Some(m) = crate::ident::mispaired(doc.id(), self.document) {
            return Err(PoseRefusal::Mate(Box::new(m.into())));
        }
        if let Some(fault) = self.faults.get(&placement) {
            return Err(PoseRefusal::Mate(Box::new(fault.clone())));
        }
        let anchor = self.anchor.get(&placement).copied().unwrap_or(placement);
        let Some(&frame) = self.frame.get(&anchor) else {
            return Err(PoseRefusal::Loose {
                placement,
                root: anchor,
            });
        };
        let pose = self
            .pose
            .get(&placement)
            .copied()
            .unwrap_or_else(Pose::identity);
        Ok(Frame::from_motion(pose.compose_around(frame)))
    }
}

impl<T: SolveScalar> SolvedPoses<T> {
    /// **A copy's pose in its anchor's space, at the solve's scalar** —
    /// [`SolvedPoses::relative`]'s map: a seed run's carrying its
    /// tangent, a box run's an enclosure over the box. `None` where the
    /// solve posed no such placement.
    pub fn relative_map(&self, placement: RecipeNodeId) -> Option<Affine3<T>> {
        let pose = self.pose.get(&placement)?;
        let left = pose.left.map_or(Motion::Identity, marked);
        Some(left.compose(marked(pose.right)).affine())
    }
}

/// A solved map as the composition rule takes it: the stored identity
/// marked as such where the lane stores one ([`SolveScalar::is_stored_identity`]).
fn marked<T: SolveScalar>(map: Affine3<T>) -> Motion<T> {
    if T::is_stored_identity(&map) {
        Motion::Identity
    } else {
        Motion::Map(map)
    }
}

impl<T: Real> Pose<T> {
    /// The pose of an anchor: its frame itself.
    pub(crate) fn identity() -> Self {
        Self {
            left: None,
            right: Affine3::identity(),
        }
    }
}

impl<T: SolveScalar> Pose<T> {
    /// `left ∘ frame ∘ right`, through the composition rule's identity
    /// fast paths (`placement::Motion`), `frame` evaluated in the same
    /// lane as the pose.
    pub(crate) fn compose_around(self, frame: Motion<T>) -> Motion<T> {
        let left = self.left.map_or(Motion::Identity, marked);
        left.compose(frame).compose(marked(self.right))
    }
}

// ---- Spaces are kinds ----

/// **Which space each node's value lives in** (D10, Spaces): a
/// placement's copy lives in its bundle's space, the world in the
/// world's, a node consuming geometry in its inputs' space, and a node
/// that denotes no geometry of its own — a measure, an assertion —
/// in none.
#[derive(Debug, Clone, Default)]
pub(crate) struct Spaces {
    /// Every node that lives in a space, and which.
    pub(crate) space: BTreeMap<RecipeNodeId, Space>,
    /// **Every node whose reads lie in two spaces**, with the two. Such
    /// a node relates two raw coordinate systems, which only a mate
    /// may do, and refuses ([`crate::NodeErrorKind::SpaceMismatch`]).
    pub(crate) across: BTreeMap<RecipeNodeId, (Space, Space)>,
}

/// [`Spaces`] for `doc` under `poses`.
pub(crate) fn spaces_of<P: crate::ProfilePayload, T: Real>(
    doc: &Doc<P>,
    poses: &SolvedPoses<T>,
) -> Spaces {
    spaces_with(doc, |placement| poses.space(placement))
}

/// [`Spaces`] for `doc`, given each placement's space, in one pass in
/// the evaluation schedule's order, which puts every input before its
/// consumer. A placement the solve gave no space (it refused) lives in
/// none, and so does what reads it.
pub(crate) fn spaces_with<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    space_of: impl Fn(RecipeNodeId) -> Option<Space>,
) -> Spaces {
    let mut out = Spaces::default();
    for id in crate::eval::schedule::schedule(doc).order {
        let Some(node) = doc.node(id) else { continue };
        // The reads whose spaces must agree: a placement's shapes (it
        // copies them as one), every other node's inputs. A
        // constraint reads across spaces by design, so its reads are
        // no input here.
        let inputs = match node {
            Node::World => {
                out.space.insert(id, Space::World);
                continue;
            }
            Node::InstantiatePart { .. } => Vec::new(),
            Node::Place { shapes, .. } => shapes
                .iter()
                .filter_map(|shape| doc.operation_of(shape.read))
                .collect(),
            _ => doc.upstream_of(id, node),
        };
        let mut distinct: Vec<Space> = Vec::new();
        for input in inputs {
            if let Some(&s) = out.space.get(&input)
                && !distinct.contains(&s)
            {
                distinct.push(s);
            }
        }
        if let [first, second, ..] = distinct.as_slice() {
            out.across.insert(id, (*first, *second));
        }
        let here = match node {
            Node::InstantiatePart { .. } | Node::Place { .. } => match space_of(id) {
                Some(space) => space,
                None => continue,
            },
            // A measure and an assertion read geometry and answer a
            // number, which lives in no space.
            Node::Measure { .. } | Node::Assertion { .. } => continue,
            _ => distinct.first().copied().unwrap_or(Space::World),
        };
        out.space.insert(id, here);
    }
    out
}

// ---- A9: relative freedom ----

/// **A9's relative-freedom partition**: the connected components of the
/// recipe over its reads ([`Doc::upstream`]), each component's nodes in
/// document order, the components themselves ordered by their first
/// node.
///
/// Two copies are relatively unconstrained exactly when they land in
/// different components — decidable from recipe structure alone, which
/// is the whole content of A9. A constraint couples its copy to its
/// target because the placement holding it reads the target, and two
/// copies pinned against the world share the world.
pub fn relative_freedom_components<P: crate::ProfilePayload>(
    doc: &Doc<P>,
) -> Vec<Vec<RecipeNodeId>> {
    let mut adjacency: BTreeMap<RecipeNodeId, BTreeSet<RecipeNodeId>> = BTreeMap::new();
    for id in doc.ids() {
        adjacency.entry(id).or_default();
        let Some(node) = doc.node(id) else { continue };
        for input in doc.upstream_of(id, node) {
            adjacency.entry(id).or_default().insert(input);
            adjacency.entry(input).or_default().insert(id);
        }
    }
    components(&doc.ids(), &adjacency)
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

// ---- D-4: the per-pair coset solve ----

/// **One solve's inputs**, borrowed for its duration and read by every
/// step below `solve_document`: the document, its one nominal
/// environment, the reach the lever is asked through, and the
/// decision band with the tolerance it was derived from. Nothing here
/// outlives the solve and nothing is derived into it — a cache would
/// be a second answer to a question the document already answers.
struct Solve<'a, P, T: Real> {
    doc: &'a Doc<P>,
    env: &'a VarEnv<T>,
    reach: &'a dyn MateReach<T>,
    band: Band,
    tol: Tol,
    record: Record,
}

/// **Where the solve's decisions go: each to the log of exactly one
/// placement, the placement whose bundle it decided.** Every decision
/// the solve makes is made inside one [`Record::unit`], named by that
/// placement: its constraints' sides read, their frames and face poses,
/// their coset rows and riders, the fold, the overconstraint check and
/// the bundle's left factor.
///
/// A [`Detached`] is not `Clone`, so a decision recorded in one unit
/// has no second home.
enum Record {
    /// Into the caller's open frame as each unit finishes, which is
    /// the order the decisions were made in — so a door's frame holds
    /// what a solve that detached nothing would have put there
    /// ([`solve_document`]'s, the door every caller but the
    /// evaluation solves through).
    Caller,
    /// Kept per placement, in decision order, for the placement's own
    /// node to splice ([`SolvedPoses::take_recordings`]) — the evaluation's
    /// ([`solve_with_env`]), where the solve runs before any node's
    /// frame is open.
    PerMate(RefCell<BTreeMap<RecipeNodeId, Vec<Detached>>>),
}

impl Record {
    /// Runs `work` as one unit of the solve's work, its decisions
    /// recorded as `placement`'s.
    fn unit<R>(&self, placement: RecipeNodeId, work: impl FnOnce() -> R) -> R {
        let (out, recording) = detached(work);
        match self {
            Self::Caller => splice(recording),
            Self::PerMate(kept) => kept
                .borrow_mut()
                .entry(placement)
                .or_default()
                .push(recording),
        }
        out
    }

    /// What the units kept, for the solve's answer to carry.
    fn kept(self) -> BTreeMap<RecipeNodeId, Vec<Detached>> {
        match self {
            Self::Caller => BTreeMap::new(),
            Self::PerMate(kept) => kept.into_inner(),
        }
    }
}

/// The frame flip that applies an OPPOSED axis sense: the half turn
/// about the mate frame's own local X, which reverses the axis and the
/// handedness of the cross axis while staying proper (det = +1).
fn opposed<T: Real>() -> Affine3<T> {
    Affine3::from_parts(
        Mat3::from_cols(
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
        ),
        Vec3::new(0.0, 0.0, 0.0),
    )
    .map(T::from_f64)
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
/// Each side's frame arrives RESOLVED (`a`, `b`: [`resolve_side`] —
/// its base composed with its offset): its placement, and its axis
/// witness, negated exactly for an opposed sense. Every primitive's
/// target keeps that axis — a standoff translates along it and the
/// rider spins about it — so no direction here is read back off a
/// product of matrices, which would be unit only to rounding and a
/// witness to nothing.
///
/// `lever` forms this mate's lever — the two mated parts' reach
/// summed ([`pair_reach`]) plus the datum's own terms, through
/// [`lever`]'s one door — and is asked at exactly one site: the
/// rider on a coincidence, the one row of the table that levers a
/// decision. Every other row decides on the datum alone, so a caller
/// with no lever in hand (the edit door, [`admit_mate`]) forms none
/// for them, and a caller that holds one already (the fold, which
/// levers its intersections too) hands it in. Replay never reaches
/// the table: with no reach it declines at [`admit_mate`], on the
/// rule stated there.
fn mate_coset<T: SolveScalar>(
    mate: MateRef,
    alignment: &Alignment,
    a: &SideFrame<T>,
    b: &SideFrame<T>,
    lever: impl FnOnce() -> Result<Arm, Box<MateFault>>,
    band: Band,
) -> Result<Coset<T>, Box<MateFault>> {
    // The table's static gap is asked at the arm it falls in, after
    // both frames resolved — so a frame refusal precedes a gap, and
    // both precede any lever. `admit_mate`'s replay arm keeps that
    // order over the sides it holds.
    let fb = b.placement;
    let (fa, axis) = match alignment.sense {
        AxisSense::Aligned => (a.placement, a.axis),
        AxisSense::Opposed => (a.placement * opposed(), -a.axis),
    };
    let local_z = Vec3::new(0.0, 0.0, 1.0).map(T::from_f64);
    let spin = |theta: f64| {
        Affine3::from_parts(
            Mat3::rotation_about(local_z, T::from_f64(theta)),
            Vec3::new(0.0, 0.0, 0.0).map(T::from_f64),
        )
    };
    let (target, subgroup) = match alignment.primitive {
        MatePrimitive::FrameCoincidence => {
            // The table: on frame-coincidence the clocking rider is
            // redundant-or-contradictory, DECIDED. A coincidence has
            // already pinned the roll, so the only clocking it can
            // agree with is zero.
            // The lever is asked HERE and nowhere else in the table:
            // no rider, no ask.
            if let Some(theta) = alignment.clocking {
                let arm = lever()?.decides_over(band).map_err(|refusal| {
                    Box::new(MateFault::Unleverable {
                        mate,
                        refusal: Box::new(refusal),
                    })
                })?;
                let roll = Measured::<T>::Lever(super::Lever::Roll {
                    radians: theta,
                    arm: arm.get(),
                });
                let sign = geom_core::k_stats::decide(
                    Refuted::ClockingRedundant.name(),
                    roll.margin(),
                    band,
                )
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
                        predicate: Refuted::ClockingRedundant.name(),
                        clash: roll.clash(),
                    }));
                }
            }
            (
                fa,
                SideFrame {
                    placement: fa,
                    axis,
                    copy_unmoved: false,
                }
                .symmetry(),
            )
        }
        MatePrimitive::Coaxial => match alignment.clocking {
            // The rider cuts the cylindrical residual to translation
            // along the axis — the table's coaxial+clocking row.
            Some(theta) => {
                let target = fa * spin(theta);
                (target, Subgroup::Prismatic { direction: axis })
            }
            None => {
                let axis = crate::PoseValue::Axis {
                    origin: Point3::origin() + fa.translation,
                    dir: axis,
                };
                (fa, axis.symmetry())
            }
        },
        MatePrimitive::PlanarRest { offset } => {
            // The table's static gap, read where the table keeps it
            // (`super::table_gap`), at this arm so a frame refusal
            // still precedes it.
            if let Some(what) = super::table_gap(alignment.primitive, alignment.clocking) {
                return Err(Box::new(MateFault::TableLacks { mate, what }));
            }
            let target = fa * Affine3::translation(local_z * T::from_f64(offset));
            let plane = crate::PoseValue::Plane {
                origin: Point3::origin() + fa.translation,
                normal: axis,
            };
            (target, plane.symmetry())
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
    // The composition rule's identity fast path (`placement::Motion`):
    // a `b` side on its copy's own frame, unmoved, contributes nothing,
    // so a copy pinned to the world by one coincidence of its own frame
    // sits at the world side's frame bit for bit.
    let representative = if b.copy_unmoved && T::is_stored_identity(&fb) {
        target
    } else {
        target * fb.inverse()
    };
    Ok(Coset {
        subgroup,
        representative,
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
/// the mint decides a length within rounding of one and refuses under
/// no band that clears it (`Kε < 1`).
///
/// # Errors
///
/// The frame ladder's own refusal for a transported direction whose
/// length could not be decided ([`derived_direction`]).
fn invert<T: SolveScalar>(c: Coset<T>, band: Band) -> Result<Coset<T>, FrameError> {
    let r = c.representative.inverse();
    let dir = |u: UnitVec3<T>| derived_direction(r.linear * u.get(), "mate_coset_inverse", band);
    let pt = |p: Point3<T>| r.transform_point(p);
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
        Subgroup::Spherical { point } => Subgroup::Spherical { point: pt(point) },
        Subgroup::Parallel { direction } => Subgroup::Parallel {
            direction: dir(direction)?,
        },
        Subgroup::Translation => Subgroup::Translation,
        Subgroup::PlaneTranslation { normal } => Subgroup::PlaneTranslation {
            normal: dir(normal)?,
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
/// rounding, so the mint never refuses under a band that clears a
/// length of one (`Kε < 1`); under a coarser one it refuses whatever
/// the parts' scale
/// (`work/msolve/a-mate-frame-axis-is-decided-against-a-length-band.md`).
fn derived_direction<T: Decide>(
    v: Vec3<T>,
    site: &'static str,
    band: Band,
) -> Result<UnitVec3<T>, FrameError> {
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

/// **The part a member stands on**: its instance's reference, or the
/// node when it is not a live instantiate node — which the member walk
/// excludes and the readers still name rather than assume. The one
/// derivation of the part a mate reads: the lever's ([`pair_reach`]),
/// a face side's ([`resolve_side`]), and the part pin a face side's
/// memo key carries (`eval`'s `SolveAnswer`), so the key and the solve
/// cannot come to name two different parts for one side. It reads the
/// part cache's own census ([`crate::eval::parts::instantiated`]), so
/// every part a mate asks for is one the descent has entered.
pub(crate) fn part_of<P>(
    doc: &Doc<P>,
    member: &Member,
) -> Result<crate::ident::DocRef, RecipeNodeId> {
    crate::eval::parts::instantiated(doc, member.instance).ok_or(member.instance)
}

/// **One side's frame as the solve reads it**: the placement its base
/// composed with its offset denotes, in the side's part coordinates,
/// and its local +Z as a witness the coset table reads — at the solve's
/// scalar.
#[derive(Debug, Clone, Copy)]
struct SideFrame<T: Real> {
    /// The frame's placement.
    placement: Affine3<T>,
    /// Its axis, local +Z.
    axis: UnitVec3<T>,
    /// Whether the side is its copy's own frame with no offset
    /// ([`SideRead::Copy`] at the empty chain): the frame the
    /// composition rule's identity names.
    copy_unmoved: bool,
}

/// A resolved side is a frame: known outright.
impl<T: Real> PoseSymmetry<T> for SideFrame<T> {
    fn symmetry(&self) -> Subgroup<T> {
        Subgroup::Trivial
    }
}

impl<T: SolveScalar> SideFrame<T> {
    /// The frame's origin's distance from its part's origin — its term
    /// of the lever ([`Alignment::lever_arm`]) — as an `f64` upper bound
    /// ([`SolveScalar::upper`]): the distance itself at `f64`.
    fn origin_reach(&self) -> f64 {
        let t = self.placement.translation;
        (t.x.powi(2) + t.y.powi(2) + t.z.powi(2)).sqrt().upper()
    }
}

/// **One side's frame**, resolved: its base ([`side_base`]) composed
/// with its offset ([`compose_offset`]). `reach` absent is replay's,
/// and a face base is then DECLINED (`Ok(None)`), on the rule
/// [`admit_mate`] states.
///
/// Asked before the coset table reads the frame and before the lever
/// is formed (the datum's `‖origin‖` terms are the RESOLVED origins),
/// and asked ONCE per side per read: the door asks it of a mate being
/// inserted, the fold of every mate it folds.
///
/// # Errors
///
/// [`side_base`]'s and [`compose_offset`]'s.
#[allow(clippy::too_many_arguments)]
fn resolve_side<P: crate::ProfilePayload, T: SolveScalar>(
    doc: &Doc<P>,
    reach: Option<&dyn MateReach<T>>,
    env: &VarEnv<T>,
    band: Band,
    tol: Tol,
    mate: MateRef,
    side: MateSide,
    read: &Side,
    frame: &MateFrame,
) -> Result<Option<SideFrame<T>>, Box<MateFault>> {
    let base = match (frame.base, reach) {
        (FrameBase::Part | FrameBase::World, _) => None,
        (FrameBase::Face, Some(reach)) => Some(face_base(doc, reach, mate, side, read, tol)?),
        (FrameBase::Face, None) => return Ok(None),
    };
    let mut resolved = compose_offset(mate, side, base, &frame.offset, env, band)?;
    resolved.copy_unmoved = read.walk.is_none()
        && read.member.instance == mate.placement
        && frame.base == FrameBase::Part
        && frame.offset.is_identity_bits();
    Ok(Some(resolved))
}

/// **A side's face base**: the canonical pose of the face the side's
/// own HEAD names in the member's part (the walk's
/// [`Walk::part_face`]: the head with the walk's qualifiers stripped),
/// asked of that part through the reach ([`MateReach::face_pose`]) in
/// the part's own coordinates, with the pose's origin and CHART axis
/// (the orientation sense is not folded in; the mate's own
/// [`AxisSense`] says which way the sides point) and, for the roll,
/// the carrier's own in-frame reference direction (`Pose::u_ref`),
/// through the witness ladder.
///
/// # Errors
///
/// [`MateFault::FaceUnresolved`] naming the mate, the side, the
/// instance and the part, carrying the reach's refusal in its own
/// voice; [`MateFault::Frame`] where the ladder refuses the pose.
fn face_base<P: crate::ProfilePayload, T: SolveScalar>(
    doc: &Doc<P>,
    reach: &dyn MateReach<T>,
    mate: MateRef,
    side: MateSide,
    read: &Side,
    tol: Tol,
) -> Result<OrthoFrame<T>, Box<MateFault>> {
    let member = &read.member;
    let unresolved = |refusal| {
        Box::new(MateFault::FaceUnresolved {
            mate,
            side,
            refusal: Box::new(refusal),
        })
    };
    let part =
        part_of(doc, member).map_err(|node| unresolved(FaceRefusal::NotAnInstance { node }))?;
    // A side on an instance's part walked to its member; a side read
    // off a placement's shape did not, and stands on no part.
    let Some(read) = &read.walk else {
        return Err(unresolved(FaceRefusal::NotAnInstance {
            node: member.instance,
        }));
    };
    let face = read.part_face().ok_or_else(|| {
        unresolved(FaceRefusal::NoPartFace {
            instance: member.instance,
            head: read.head.clone(),
        })
    })?;
    let pose = reach.face_pose(&part, &face).map_err(|refusal| {
        unresolved(FaceRefusal::Reach {
            instance: member.instance,
            part,
            face: face.clone(),
            refusal,
        })
    })?;
    // `topo::readback::face_pose` answers every carrier it answers
    // through one constructor that fixes `u_ref` (an edge's pose is
    // the only readback with none), so a face's pose always carries
    // the roll reference the frame takes.
    let Some(u_ref) = pose.u_ref else {
        unreachable!("readback::face_pose fixes u_ref for every carrier it answers")
    };
    geom_core::linalg::frame::point_at_frame(pose.origin, pose.origin + pose.axis, u_ref, tol)
        .map_err(|error| Box::new(MateFault::Frame { mate, side, error }))
}

/// **A side's base composed with its offset** — `base` the face's
/// frame, `None` for the part frame — through the one fold every
/// placement chain takes ([`crate::placement::Placement::motion_after`]),
/// at `env`, the solve's own environment at its scalar — so a box or a
/// seed that binds a parameter a step reads moves the side in that run.
///
/// An offset whose every step is a bit-exact identity literal (the
/// empty chain among them) leaves the base as it is, its axis
/// included, and evaluates nothing. Any other offset is evaluated
/// once, folded onto the base, and the frame's axis is its third
/// column re-minted under the run's LINEAR band ([`derived_direction`]).
/// The column is unit to rounding, so the mint decides a length of one
/// against `(ε, Kε)`: it never refuses where `Kε < 1`, and at a coarser
/// ε (`ε ≥ 0.1` at the default K) every such side refuses
/// [`MateFault::Frame`], whatever the part's scale
/// (`work/msolve/a-mate-frame-axis-is-decided-against-a-length-band.md`).
///
/// # Errors
///
/// [`MateFault::FrameUnevaluated`] carrying the evaluation layer's
/// refusal, an expression's slot named by the mate's own address;
/// [`MateFault::Frame`] where the composed axis is no direction.
fn compose_offset<T: SolveScalar>(
    mate: MateRef,
    side: MateSide,
    base: Option<OrthoFrame<T>>,
    offset: &crate::placement::Placement,
    env: &VarEnv<T>,
    band: Band,
) -> Result<SideFrame<T>, Box<MateFault>> {
    let (before, base) = match base {
        None => (Motion::Identity, OrthoFrame::axes_xy(Point3::origin())),
        Some(face) => (Motion::Map(face.to_affine()), face),
    };
    let refused = |error: crate::eval::NodeErrorKind| {
        let error = match error {
            crate::eval::NodeErrorKind::Expr { slot, source } => {
                let slot = match slot.rigid_arg() {
                    Some((step, arg)) => crate::node::SlotId::MateFrameStep {
                        constraint: mate.id,
                        side,
                        step,
                        arg,
                    },
                    None => slot,
                };
                crate::eval::NodeErrorKind::Expr { slot, source }
            }
            other => other,
        };
        Box::new(MateFault::FrameUnevaluated {
            mate,
            side,
            refusal: Box::new(NodeRefusal::from(error)),
        })
    };
    if offset.is_identity_bits() {
        return Ok(SideFrame {
            placement: base.to_affine(),
            axis: base.w(),
            copy_unmoved: false,
        });
    }
    let placement = offset
        .motion_after(before, env, band)
        .map_err(refused)?
        .affine();
    let axis = derived_direction(placement.linear.c2, "mate_frame_offset_axis", band)
        .map_err(|error| Box::new(MateFault::Frame { mate, side, error }))?;
    Ok(SideFrame {
        placement,
        axis,
        copy_unmoved: false,
    })
}

// ---- Reading a bundle ----

/// **One side of a mate, as the solve reads it**: the member it stands
/// on and, for a side that walked a face read to a member, that walk.
///
/// The world stands on [`RecipeNodeId::WORLD`] and a placement's own
/// copy on the placement, each with an empty chain and no walk.
#[derive(Debug, Clone)]
pub(super) struct Side {
    pub(super) member: Member,
    pub(super) walk: Option<Walk>,
}

impl Side {
    /// The side standing on `node` itself, through no placer and no
    /// face: the world, or a placement's own copy.
    fn bare(node: RecipeNodeId) -> Self {
        Self {
            member: Member {
                instance: node,
                chain: Vec::new(),
            },
            walk: None,
        }
    }

    /// Whether this side stands on the world.
    fn is_world(&self) -> bool {
        self.member.instance == RecipeNodeId::WORLD
    }
}

/// **Whether `read`, a face read of a constraint of `placement`, is the
/// copy side read through the placement's own copy** — the one read a
/// placement makes of what it defines, and so the one read
/// [`Doc::upstream`] does not count as an input (`doc.rs`,
/// `own_copy_side`).
///
/// It is not a dependency on the copy's pose: what the solve reads
/// through it is the copied shape before placement, in its own
/// coordinates — a face's pose off the part an instance copies, and the
/// placer maps between the read and the part — and never anything the
/// placement computes. Only an instance makes it: a `Place` reads its
/// shapes, which are its inputs, and a `Place`'s read of its own output
/// is a cycle like any other.
pub(crate) fn reads_own_copy<P>(doc: &Doc<P>, placement: RecipeNodeId, read: crate::VarId) -> bool {
    matches!(doc.node(placement), Some(Node::InstantiatePart { .. }))
        && super::member::head_of(doc, read)
            .and_then(|head| super::member::member_of(doc, &head))
            .is_some_and(|member| member.instance == placement)
}

/// **One side of a constraint of `placement`, read.**
///
/// # Errors
///
/// [`MateFault::SideBase`] where the read and its frame's base
/// disagree; the walk's [`MateFault::SideUnresolved`] and
/// [`MateFault::DanglingHead`].
pub(super) fn read_side<P>(
    doc: &Doc<P>,
    mate: MateRef,
    side: MateSide,
    data: &super::Mate,
) -> Result<Side, MateFault> {
    let read = data.side(side);
    let base = data.frame(side).base;
    if !read.admits(base) {
        return Err(MateFault::SideBase { mate, side, base });
    }
    match read {
        SideRead::World => Ok(Side::bare(RecipeNodeId::WORLD)),
        SideRead::Copy => Ok(Side::bare(mate.placement)),
        SideRead::Face(var) => {
            // A `Place` reads its copy side off its shapes, before
            // placement: the side stands on the placement itself.
            if let Some(Node::Place { shapes, .. }) = doc.node(mate.placement)
                && let Some(select) = doc.selection(*var)
                && shapes.iter().any(|shape| shape.read == select.body)
            {
                return Ok(Side::bare(mate.placement));
            }
            walk_of(doc, mate, side, *var).map(|walk| Side {
                member: walk.member.clone(),
                walk: Some(walk),
            })
        }
    }
}

/// **The copy side and the target side of one constraint** — the copy
/// side the one standing on its placement. A constraint with neither
/// side on its copy pins nothing of it, and one with both relates the
/// copy to itself.
///
/// # Errors
///
/// [`MateFault::NotOnTheCopy`] and [`MateFault::SelfMate`].
fn copy_and_target(mate: MateRef, a: &Side, b: &Side) -> Result<(MateSide, MateSide), MateFault> {
    let on = |s: &Side| s.member.instance == mate.placement;
    match (on(a), on(b)) {
        (true, false) => Ok((MateSide::A, MateSide::B)),
        (false, true) => Ok((MateSide::B, MateSide::A)),
        (false, false) => Err(MateFault::NotOnTheCopy { mate }),
        (true, true) => Err(MateFault::SelfMate {
            mate,
            instance: mate.placement,
        }),
    }
}

/// **The per-reference prefix of a mate's admission**, after its two
/// sides are read: the checks that need a number ([`check_reference`])
/// on each walked side, then the pair as two DISTINCT members
/// ([`MateFault::SelfMate`]). Two COPIES of one pattern are two
/// members and pass: what a mate cannot relate is a member to itself.
fn check_references<P: crate::ProfilePayload, S>(
    doc: &Doc<P>,
    env: &VarEnv<S>,
    mate: MateRef,
    a: &Side,
    b: &Side,
) -> Result<(), MateFault> {
    for (side, read) in [(MateSide::A, a), (MateSide::B, b)] {
        if let Some(walk) = &read.walk {
            check_reference(doc, env, mate, side, walk)?;
        }
    }
    if a.member == b.member {
        return Err(MateFault::SelfMate {
            mate,
            instance: a.member.instance,
        });
    }
    Ok(())
}

/// **The class door of a mate's own admission**: the class inside the
/// vocabulary ([`MateFault::ClassNotAdmitted`]). The class table is
/// the policy (`super::class_admission`); this door enforces its own
/// half of it and nothing more.
fn admit_class(mate: MateRef, class: super::ContactClass) -> Result<(), Box<MateFault>> {
    if super::class_admission(class) == super::ClassAdmission::NotAdmitted {
        return Err(Box::new(MateFault::ClassNotAdmitted { mate }));
    }
    Ok(())
}

/// **One constraint's own admission** — what the solve decides about a
/// mate from its own datum alone, asked of the mate the document holds
/// or is about to hold at `mate`: its two sides read ([`read_side`]),
/// which one is on the copy ([`copy_and_target`]), the per-reference
/// prefix ([`check_references`]), the class door ([`admit_class`]),
/// each side's frame resolved ([`resolve_side`]) and the coset table
/// ([`mate_coset`]), in the order the solve meets them.
///
/// The edit door asks this of a constraint being added, so a mate the
/// solve refuses on its own datum is refused at the door (`ASSEMBLY.md`
/// A11 rule 1). What a bundle decides — an overconstraint, a
/// contradiction against ANOTHER constraint, targets that differ — is
/// the door's next question ([`admit_constraint`]) and the solve's.
///
/// `reach` absent is replay's, and what needs the parts is then
/// DECLINED — the rider on a coincidence, decided over a lever, and a
/// face-based side's frame: the door that recorded the edit decided
/// them over the parts it had in hand.
///
/// # Errors
///
/// The fault the solve records for the mate's own datum, unaltered.
pub(crate) fn admit_mate<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    mate: MateRef,
    data: &super::Mate,
    env: &VarEnv<f64>,
    reach: Option<&dyn MateReach>,
    tol: Tol,
) -> Result<(), Box<MateFault>> {
    let band = Band::linear(tol).map_err(|error| Box::new(MateFault::Band { error }))?;
    let a = read_side(doc, mate, MateSide::A, data).map_err(Box::new)?;
    let b = read_side(doc, mate, MateSide::B, data).map_err(Box::new)?;
    copy_and_target(mate, &a, &b).map_err(Box::new)?;
    check_references(doc, env, mate, &a, &b).map_err(Box::new)?;
    admit_class(mate, data.class)?;
    let alignment = &data.alignment;
    let fa = resolve_side(doc, reach, env, band, tol, mate, MateSide::A, &a, &alignment.a)?;
    let fb = resolve_side(doc, reach, env, band, tol, mate, MateSide::B, &b, &alignment.b)?;
    let (Some(reach), Some(fa), Some(fb)) = (reach, fa, fb) else {
        if let Some(what) = super::table_gap(alignment.primitive, alignment.clocking) {
            return Err(Box::new(MateFault::TableLacks { mate, what }));
        }
        return Ok(());
    };
    let form = || {
        let parts = pair_reach(doc, reach, &a.member, &b.member).map_err(|refusal| {
            Box::new(MateFault::Unleverable {
                mate,
                refusal: Box::new(refusal),
            })
        })?;
        lever(mate, parts, alignment, &fa, &fb)
    };
    mate_coset(mate, alignment, &fa, &fb, form, band).map(|_| ())
}

/// **A constraint added to a bundle, admitted as a bundle member**
/// (D10; FORK-S3O, its B half): the constraint's target agrees with the
/// bundle's ([`MateFault::TargetsDiffer`], the stage's named interim),
/// and the bundle does not already pin its copy
/// ([`MateFault::Overconstrained`]: a mate beyond the pin is refused,
/// never verified and kept). The bundle is folded from its definitions
/// at the door, at the document's own values.
///
/// `reach` absent is replay's: a bundle whose fold needs the parts is
/// declined, as [`admit_mate`] declines.
///
/// # Errors
///
/// [`MateFault::TargetsDiffer`], [`MateFault::Overconstrained`], or the
/// fold's own refusal of the bundle as it stands with the constraint.
pub(crate) fn admit_constraint<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    placement: RecipeNodeId,
    bundle: &[super::Constraint],
    added: &super::Constraint,
    env: &VarEnv<f64>,
    reach: Option<&dyn MateReach>,
    tol: Tol,
) -> Result<(), Box<MateFault>> {
    let band = Band::linear(tol).map_err(|error| Box::new(MateFault::Band { error }))?;
    let mut all: Vec<&super::Constraint> = bundle.iter().collect();
    all.push(added);
    let read = read_bundle(doc, env, placement, &all).map_err(Box::new)?;
    let Some(reach) = reach else { return Ok(()) };
    if bundle.is_empty() {
        return Ok(());
    }
    let s = Solve {
        doc,
        env,
        reach,
        band,
        tol,
        record: Record::Caller,
    };
    let held = fold_bundle(&s, &read[..read.len() - 1])?;
    if held.subgroup.is_determined() {
        return Err(Box::new(MateFault::Overconstrained {
            placement,
            constraints: vec![added.id],
            held: Subgroup::Trivial,
        }));
    }
    Ok(())
}

/// **One constraint of a bundle, its two sides read**, oriented: the
/// target side first, as the fold reads a pair.
pub(super) struct BundleMate<'d> {
    /// The constraint's address.
    pub(super) mate: MateRef,
    /// The mate.
    pub(super) data: &'d super::Mate,
    /// The `a` side, as authored.
    pub(super) a: Side,
    /// The `b` side, as authored.
    pub(super) b: Side,
    /// Which authored side is on the copy.
    pub(super) copy: MateSide,
}

impl BundleMate<'_> {
    /// The side on the copy.
    fn copy_side(&self) -> &Side {
        match self.copy {
            MateSide::A => &self.a,
            MateSide::B => &self.b,
        }
    }

    /// The target side.
    fn target_side(&self) -> &Side {
        match self.copy {
            MateSide::A => &self.b,
            MateSide::B => &self.a,
        }
    }
}

/// **A placement's bundle, read**: each constraint's two sides, which
/// one is on the copy, and the per-reference checks — every constraint
/// on one target and one reading of the copy (the stage's named
/// interim, [`MateFault::TargetsDiffer`]).
///
/// # Errors
///
/// The first constraint's own refusal, or [`MateFault::TargetsDiffer`].
fn read_bundle<'d, P: crate::ProfilePayload, S>(
    doc: &Doc<P>,
    env: &VarEnv<S>,
    placement: RecipeNodeId,
    bundle: &[&'d super::Constraint],
) -> Result<Vec<BundleMate<'d>>, MateFault> {
    let mut out: Vec<BundleMate<'d>> = Vec::with_capacity(bundle.len());
    for c in bundle {
        let mate = MateRef::new(placement, c.id);
        let a = read_side(doc, mate, MateSide::A, &c.mate)?;
        let b = read_side(doc, mate, MateSide::B, &c.mate)?;
        let (copy, _) = copy_and_target(mate, &a, &b)?;
        check_references(doc, env, mate, &a, &b)?;
        out.push(BundleMate {
            mate,
            data: &c.mate,
            a,
            b,
            copy,
        });
    }
    if let Some(first) = out.first()
        && out.iter().any(|m| {
            m.target_side().member != first.target_side().member
                || m.copy_side().member != first.copy_side().member
        })
    {
        return Err(MateFault::TargetsDiffer {
            placement,
            constraints: bundle.iter().map(|c| c.id).collect(),
        });
    }
    Ok(out)
}

/// **A bundle's fold** (A11 rule 1): every constraint of one placement,
/// on the one (target, copy) member pair its constraints read,
/// intersected. The result's representative maps the copy member's own
/// coordinates into the target member's. The members' derived offsets
/// are NOT in this coset: they are the pair's static left factor
/// ([`pair_left_factor`]), composed outside the fold.
///
/// Each mate passes its own admission first — the class door, each
/// side's frame resolved, and the coset table — and only then meets
/// the fold. The lever is formed for every mate on a pair of
/// instances, whose parts' reach is in hand; on a pair with another
/// member (a placement's shapes, a construction) it is formed only
/// where a decision turns on it, and refuses there
/// ([`MateFault::Unleverable`]) until such a member's extent reaches
/// the solve.
///
/// # Errors
///
/// The first refusal: a malformed alignment, a table gap, an
/// Indeterminate case split, or the CONTRADICTORY empty intersection,
/// which names both mates, the predicate, and the measured clash.
fn fold_bundle<P: crate::ProfilePayload, T: SolveScalar>(
    s: &Solve<'_, P, T>,
    mates: &[BundleMate<'_>],
) -> Result<Coset<T>, Box<MateFault>> {
    let Solve {
        doc,
        env,
        reach,
        band,
        tol,
        ..
    } = *s;
    let mut held = Coset::unconstrained();
    let mut held_mate: Option<MateRef> = None;
    // The fold's lever is the largest of the mates' own: each is the
    // pair's two part reaches plus that mate's datum terms. The
    // reaches are asked ONCE per bundle, lazily, at the first mate
    // that forms a lever.
    let mut arm: Option<Arm> = None;
    let mut parts_reach: Option<Result<f64, super::LeverRefusal>> = None;
    for m in mates {
        let mate = m.mate;
        let alignment = &m.data.alignment;
        let (target, copy) = (&m.target_side().member, &m.copy_side().member);
        s.record.unit(mate.placement, || {
            admit_class(mate, m.data.class)?;
            let side = |which, read: &Side, frame| {
                resolve_side(doc, Some(reach), env, band, tol, mate, which, read, frame)
                    .map(|frame| frame.unwrap_or_else(|| unreachable!("a reach declines nothing")))
            };
            let a = side(MateSide::A, &m.a, &alignment.a)?;
            let b = side(MateSide::B, &m.b, &alignment.b)?;
            let parts = parts_reach
                .get_or_insert_with(|| pair_reach(doc, reach, target, copy))
                .clone();
            let unleverable = |refusal| {
                Box::new(MateFault::Unleverable {
                    mate,
                    refusal: Box::new(refusal),
                })
            };
            // A pair of instances forms its lever for every mate, as
            // the fold has always formed it; any other pair forms one
            // only where a decision asks.
            let mate_arm: Result<Arm, super::LeverRefusal> = match &parts {
                Ok(parts) => Ok(lever(mate, *parts, alignment, &a, &b)?),
                Err(refusal) if pair_of_instances(doc, target, copy) => {
                    return Err(unleverable(refusal.clone()));
                }
                Err(refusal) => Err(refusal.clone()),
            };
            if let Ok(mate_arm) = mate_arm {
                arm = Some(arm.map_or(mate_arm, |held| held.max(mate_arm)));
            }
            let form = || mate_arm.clone().map_err(unleverable);
            let mut coset = mate_coset(mate, alignment, &a, &b, form, band)?;
            // The authored order is `a`'s coordinates from `b`'s; the
            // fold reads the target from the copy, so a mate authored
            // the other way is inverted. The transported direction is
            // `a`'s axis carried into `b`'s coordinates, so a refusal
            // is reported at side `a`.
            if m.copy != MateSide::B {
                coset = invert(coset, band).map_err(|error| {
                    Box::new(MateFault::Frame {
                        mate,
                        side: MateSide::A,
                        error,
                    })
                })?;
            }
            // The fold's identity element costs nothing and decides
            // nothing: the first mate's coset is the fold.
            if matches!(held.subgroup, Subgroup::Se3) {
                held = coset;
                return Ok(());
            }
            let fold_arm = match arm {
                Some(arm) => arm,
                None => return Err(form().unwrap_err()),
            };
            held = match super::coset::intersect(held, coset, band, fold_arm) {
                Ok(next) => next,
                Err(FoldStop::Indeterminate(diag)) => {
                    return Err(Box::new(MateFault::Indeterminate { mate, diag }));
                }
                Err(FoldStop::OutOfRange) => {
                    return Err(Box::new(MateFault::PoseOutOfRange {
                        held: held_mate.unwrap_or(mate),
                        added: mate,
                    }));
                }
                Err(FoldStop::Unleverable(refusal)) => {
                    return Err(Box::new(MateFault::Unleverable { mate, refusal }));
                }
                Err(FoldStop::Clash { predicate, clash }) => {
                    return Err(Box::new(MateFault::Contradictory {
                        held: held_mate.unwrap_or(mate),
                        added: mate,
                        predicate,
                        clash,
                    }));
                }
                Err(FoldStop::NoRepresentative { held, added }) => unreachable!(
                    "a mate pins a frame, an axis or a plane, whose closure the representative \
                     stage covers: {} against {}",
                    held.name(),
                    added.name()
                ),
            };
            if matches!(held.subgroup, Subgroup::Empty) {
                return Err(Box::new(MateFault::Contradictory {
                    held: held_mate.unwrap_or(mate),
                    added: mate,
                    predicate: super::MATE_MEMBER_EMPTY,
                    clash: Clash::Structural,
                }));
            }
            Ok(())
        })?;
        held_mate.get_or_insert(mate);
    }
    Ok(held)
}

/// Whether both members stand on live instances, whose parts' reach the
/// solve reads.
fn pair_of_instances<P>(doc: &Doc<P>, x: &Member, y: &Member) -> bool {
    let instance = |m: &Member| matches!(doc.node(m.instance), Some(Node::InstantiatePart { .. }));
    instance(x) && instance(y)
}

/// **A mate's lever, formed**: the pair's parts' reach `parts` plus
/// the datum's own terms over its two resolved sides, through the one
/// door that admits a length the predicates can decide over
/// ([`Arm::of`]).
///
/// # Errors
///
/// [`MateFault::Unleverable`] carrying [`super::LeverRefusal::OutOfRange`].
fn lever<T: SolveScalar>(
    mate: MateRef,
    parts: f64,
    alignment: &Alignment,
    a: &SideFrame<T>,
    b: &SideFrame<T>,
) -> Result<Arm, Box<MateFault>> {
    let datum = alignment.lever_terms(a.origin_reach(), b.origin_reach());
    Arm::of(parts, datum).map_err(|refusal| {
        Box::new(MateFault::Unleverable {
            mate,
            refusal: Box::new(refusal),
        })
    })
}

/// **The two mated members' reach, summed** — the body terms of the
/// pair's lever ([`Alignment::lever_arm`] states the whole sum). An
/// instance's is its part's reach; the world has no extent and adds
/// nothing. A pattern copy or a transform on the chain moves the part
/// rigidly and changes no reach, so the member's chain is not
/// consulted.
///
/// # Errors
///
/// The first member whose reach is not in hand, in pair order: a part
/// the reach refuses, or a member standing on something other than an
/// instance or the world.
fn pair_reach<P: crate::ProfilePayload, T: Real>(
    doc: &Doc<P>,
    reach: &dyn MateReach<T>,
    target: &Member,
    copy: &Member,
) -> Result<f64, super::LeverRefusal> {
    let of = |member: &Member| {
        if member.instance == RecipeNodeId::WORLD {
            return Ok(0.0);
        }
        let doc_ref =
            part_of(doc, member).map_err(|node| super::LeverRefusal::NotAnInstance { node })?;
        reach
            .reach(&doc_ref)
            .map_err(|refusal| super::LeverRefusal::Reach {
                instance: member.instance,
                part: doc_ref,
                refusal,
            })
    };
    Ok(of(target)? + of(copy)?)
}

/// **The bundle's static left factor**: what the members' derived
/// offsets contribute to the copy's pose. A placer's map acts in
/// DOCUMENT coordinates — outside the copy's pose, where the
/// evaluation composes it — so the step is
///
/// ```text
/// world_copy = (O_copy⁻¹ ∘ O_target) ∘ world_target ∘ rep
/// ```
///
/// and this answers `O_copy⁻¹ ∘ O_target`, which [`Pose::left`] gathers
/// OUTSIDE the anchor's frame. `None` when neither side passes a placer
/// — the factor is then the identity BY CONSTRUCTION, not numerically.
/// The bundle's mates all relate one member pair, so the first mate's
/// two sides are the ones the offsets are derived from.
fn pair_left_factor<P: crate::ProfilePayload, T: SolveScalar>(
    s: &Solve<'_, P, T>,
    first: &BundleMate<'_>,
) -> Result<Option<Affine3<T>>, Box<MateFault>> {
    let Solve { doc, env, band, .. } = *s;
    let target_side = match first.copy {
        MateSide::A => MateSide::B,
        MateSide::B => MateSide::A,
    };
    let derived = |side, read: &Side| match &read.walk {
        Some(walk) => derived_offset(doc, env, first.mate, side, walk, band),
        None => Ok(None),
    };
    let op = derived(target_side, first.target_side())?;
    let oc = derived(first.copy, first.copy_side())?;
    Ok(match (oc, op) {
        (None, None) => None,
        (Some(oc), Some(op)) => Some(oc.inverse() * op),
        (Some(oc), None) => Some(oc.inverse()),
        (None, Some(op)) => Some(op),
    })
}

/// **The document's solve**: every placement's bundle folded, in the
/// order the placements read one another ([`Doc::upstream`]), each
/// copy's pose its target's composed with its bundle's representative.
///
/// Total by construction — a refusing bundle records its fault against
/// its own placement and every placement chained to it, and leaves
/// every other placement solved.
///
/// `reach` is the door the solve's two geometric reads cross: each
/// mated part's own extent, asked lazily per bundle and entering only
/// as the lever a parallelism verdict is decided over, and each
/// face-based side's pose, asked once per side where the frame is
/// read.
///
/// **One environment per solve**, the document's own, at `f64`: the
/// nominal solve. An evaluation solves in its own lane's environment
/// ([`solve_with_env`]).
pub fn solve_document<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    reach: &dyn MateReach,
    tol: Tol,
) -> SolvedPoses {
    let env = doc.var_env::<f64>();
    solve(doc, &env, reach, tol, Record::Caller)
}

/// **[`solve_document`] at a run's own scalar**, for a caller outside
/// an evaluation that holds the run's environment and a reach at that
/// scalar: the answer an evaluation in that lane solves to, its
/// decisions recorded into the caller's frame as [`solve_document`]'s
/// are.
pub fn solve_document_at<P: crate::ProfilePayload, T: SolveScalar>(
    doc: &Doc<P>,
    env: &VarEnv<T>,
    reach: &dyn MateReach<T>,
    tol: Tol,
) -> SolvedPoses<T> {
    solve(doc, env, reach, tol, Record::Caller)
}

/// **The evaluation's solve**: [`solve_document`] at the evaluation's
/// own scalar and environment, each decision kept for the placement
/// whose bundle it decided ([`SolvedPoses::take_recordings`]).
pub(crate) fn solve_with_env<P: crate::ProfilePayload, T: SolveScalar>(
    doc: &Doc<P>,
    env: &VarEnv<T>,
    reach: &dyn MateReach<T>,
    tol: Tol,
) -> SolvedPoses<T> {
    solve(doc, env, reach, tol, Record::PerMate(RefCell::new(BTreeMap::new())))
}

/// **The constraints a placement holds**, `None` for a node that is no
/// placement.
pub(crate) fn bundle_of<P, S: crate::Slot>(node: &Node<P, S>) -> Option<&[super::Constraint<S>]> {
    match node {
        Node::Place { constraints, .. } | Node::InstantiatePart { constraints, .. } => {
            Some(constraints)
        }
        _ => None,
    }
}

fn solve<P: crate::ProfilePayload, T: SolveScalar>(
    doc: &Doc<P>,
    env: &VarEnv<T>,
    reach: &dyn MateReach<T>,
    tol: Tol,
    record: Record,
) -> SolvedPoses<T> {
    let mut out = SolvedPoses::empty(doc.id());
    let placements: Vec<RecipeNodeId> = crate::eval::schedule::schedule(doc)
        .order
        .into_iter()
        .filter(|&id| doc.node(id).and_then(bundle_of).is_some())
        .collect();
    let band = match Band::linear(tol) {
        Ok(band) => band,
        Err(error) => {
            // No band, no decisions: every placement refuses with the
            // same typed cause rather than any of them guessing.
            for id in placements {
                out.faults.insert(id, MateFault::Band { error });
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
        record,
    };
    for id in placements {
        let Some(bundle) = doc.node(id).and_then(bundle_of) else {
            continue;
        };
        match solve_placement(&s, &out, id, bundle) {
            Ok(Solved::Loose) => {
                out.anchor.insert(id, id);
                out.pose.insert(id, Pose::identity());
            }
            Ok(Solved::World(frame, left)) => {
                out.anchor.insert(id, id);
                out.frame.insert(id, frame);
                out.pose.insert(
                    id,
                    Pose {
                        left,
                        right: Affine3::identity(),
                    },
                );
            }
            Ok(Solved::On {
                target,
                rep,
                factor,
            }) => {
                let anchor = out.anchor[&target];
                let Pose {
                    left: target_left,
                    right: target_right,
                } = out.pose[&target];
                let left = match (factor, target_left) {
                    (None, left) => left,
                    (Some(factor), None) => Some(factor),
                    (Some(factor), Some(left)) => Some(factor * left),
                };
                out.anchor.insert(id, anchor);
                out.pose.insert(
                    id,
                    Pose {
                        left,
                        right: target_right * rep,
                    },
                );
            }
            Err(fault) => {
                out.faults.insert(id, *fault);
            }
        }
    }
    out.recordings = Mutex::new(s.record.kept());
    out
}

/// What one bundle's fold placed its copy at.
enum Solved<T: Real> {
    /// Nothing pins it: a loose copy, the root of its own space.
    Loose,
    /// Pinned against the world, at this frame, with the copy side's
    /// placer factor.
    World(Motion<T>, Option<Affine3<T>>),
    /// Pinned against the copy `target` defines (or a construction of
    /// the world's kind), at `rep` from it, with the pair's left factor.
    On {
        target: RecipeNodeId,
        rep: Affine3<T>,
        factor: Option<Affine3<T>>,
    },
}

/// **One placement's bundle, folded.** A bundle that pins less than
/// its copy needs makes a loose copy, never a refusal; one that pins it
/// with a constraint the rest already fix refuses, naming every such
/// constraint.
fn solve_placement<P: crate::ProfilePayload, T: SolveScalar>(
    s: &Solve<'_, P, T>,
    solved: &SolvedPoses<T>,
    placement: RecipeNodeId,
    bundle: &[super::Constraint],
) -> Result<Solved<T>, Box<MateFault>> {
    if bundle.is_empty() {
        return Ok(Solved::Loose);
    }
    let all: Vec<&super::Constraint> = bundle.iter().collect();
    let mates = s
        .record
        .unit(placement, || read_bundle(s.doc, s.env, placement, &all))
        .map_err(Box::new)?;
    let first = &mates[0];
    let target = first.target_side().member.instance;
    // The copy a target placement defines must have solved: a
    // placement chained to one that refused has no pose, and carries
    // the refusal that stopped it.
    let target_placement = s.doc.node(target).and_then(bundle_of).is_some();
    if target_placement && let Some(fault) = solved.fault(target) {
        return Err(Box::new(fault.clone()));
    }
    let coset = fold_bundle(s, &mates)?;
    if !coset.subgroup.is_determined() {
        return Ok(Solved::Loose);
    }
    if mates.len() > 1 {
        let redundant = redundant_constraints(s, &mates)?;
        if !redundant.is_empty() {
            return Err(Box::new(MateFault::Overconstrained {
                placement,
                constraints: redundant,
                held: Subgroup::Trivial,
            }));
        }
    }
    let factor = s
        .record
        .unit(placement, || pair_left_factor(s, first))?;
    if first.target_side().is_world() {
        return Ok(Solved::World(marked(coset.representative), factor));
    }
    if !target_placement {
        // A construction of the world's kind (the stage's interim): the
        // copy sits at its representative from the identity.
        return Ok(Solved::World(marked(coset.representative), factor));
    }
    // A loose target's copy is solved in the loose copy's space.
    Ok(Solved::On {
        target,
        rep: coset.representative,
        factor,
    })
}

/// **Every constraint of a pinning bundle that the rest already fix**:
/// each whose removal leaves the fold's codimension unchanged — the
/// rest still pin the copy. Constraints have no order, so each is
/// asked alone, and every one that answers is named.
fn redundant_constraints<P: crate::ProfilePayload, T: SolveScalar>(
    s: &Solve<'_, P, T>,
    mates: &[BundleMate<'_>],
) -> Result<Vec<crate::node::ConstraintId>, Box<MateFault>> {
    let mut out = Vec::new();
    for (k, m) in mates.iter().enumerate() {
        let rest: Vec<BundleMate<'_>> = mates
            .iter()
            .enumerate()
            .filter(|&(j, _)| j != k)
            .map(|(_, m)| BundleMate {
                mate: m.mate,
                data: m.data,
                a: m.a.clone(),
                b: m.b.clone(),
                copy: m.copy,
            })
            .collect();
        if fold_bundle(s, &rest)?.subgroup.is_determined() {
            out.push(m.mate.id);
        }
    }
    Ok(out)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::ErrorTextReading;

    const SITE: &str = "solve_test_direction";

    /// **The composed axis is decided against the run's LENGTH band,
    /// whatever the part's scale** (the limit
    /// `work/msolve/a-mate-frame-axis-is-decided-against-a-length-band.md`
    /// records). Under a band whose escalation edge clears one, an
    /// authored side — authored with a long axis, as a part at a large
    /// scale would author it — composes; under one that holds a length
    /// of one inside it, the same side refuses `Frame`, because its
    /// literal is unit-length. The part base with no step decides
    /// nothing and composes under both.
    #[test]
    fn a_composed_axis_is_decided_against_the_length_band() {
        let tol = Tol::witness();
        let env = VarEnv::<f64>::default();
        let frame =
            MateFrame::authored([0.0; 3], [0.0, 0.0, 1e4], [1e4, 0.0, 0.0], tol).expect("a frame");
        let fine = Band::linear_at(tol, 1e-3).expect("a band");
        let coarse = Band::linear_at(tol, 0.5).expect("a band");
        let compose = |offset: &crate::placement::Placement, band| {
            compose_offset(
                RecipeNodeId::new(0, 1),
                MateSide::A,
                None,
                offset,
                &env,
                band,
            )
        };
        assert!(
            compose(&frame.offset, fine).is_ok(),
            "a fine band decides it"
        );
        assert!(
            matches!(
                compose(&frame.offset, coarse).map_err(|f| *f),
                Err(MateFault::Frame { .. })
            ),
            "a band holding one refuses it"
        );
        let none = crate::placement::Placement::IDENTITY;
        assert!(
            compose(&none, coarse).is_ok(),
            "the empty chain decides nothing"
        );
    }

    /// **An authored side, as the solve composes it, is the literal's
    /// map with that map's third column as its axis.** Over every
    /// combination of a 22-value axis grid (signed units, halves,
    /// in-band and sub-band lengths, underflowing and overflowing
    /// magnitudes, the non-finite values), six references and four
    /// origins — 255 552 frames — the authored door either refuses
    /// typed, or builds a part base with one literal step that
    /// [`compose_offset`] composes to that literal bit for bit, with an
    /// axis that always decides and lies within rounding of the third
    /// column (the column is unit to rounding, so the re-mint moves a
    /// component by an ulp or two).
    #[test]
    fn an_authored_side_composes_to_its_literal_with_its_column_as_axis() {
        let tol = Tol::witness();
        let eps = tol.eps();
        let env = VarEnv::<f64>::default();
        let vals = [
            0.0,
            -0.0,
            1.0,
            -1.0,
            0.5,
            -0.3,
            1e-3,
            3.0 * eps,
            eps,
            0.5 * eps,
            -eps,
            1e-160,
            1e-200,
            1e154,
            1e200,
            1e308,
            -1e308,
            f64::NAN,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::MIN_POSITIVE,
            5e-324,
        ];
        let refs = [
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0],
            [1.0, 1.0, 1e-12],
            [0.0, 0.0, 2.0],
            [-0.0, 3e-9, 1.0],
        ];
        let origins = [
            [0.0, 0.0, 0.0],
            [1e6, -1e6, 1e6],
            [1e-6, 1e15, -1e-300],
            [-0.0, -0.0, -0.0],
        ];
        let (mut built, mut refused) = (0_usize, 0_usize);
        for x in vals {
            for y in vals {
                for z in vals {
                    for r in refs {
                        for o in origins {
                            let Ok(frame) = MateFrame::authored(o, [x, y, z], r, tol) else {
                                refused += 1;
                                continue;
                            };
                            built += 1;
                            let [crate::placement::Step::Literal(literal)] =
                                frame.offset.steps.as_slice()
                            else {
                                panic!("{frame:?}: one literal step");
                            };
                            let side = compose_offset(
                                RecipeNodeId::new(0, 1),
                                MateSide::A,
                                None,
                                &frame.offset,
                                &env,
                                band(),
                            )
                            .unwrap_or_else(|e| panic!("{frame:?}: composes: {e:?}"));
                            assert!(
                                Frame::from_affine(side.placement).bit_eq(literal),
                                "{frame:?}: the literal, bit for bit"
                            );
                            let d = (side.placement.linear.c2 - side.axis.get()).norm_witness();
                            assert!(d <= 2.0 * f64::EPSILON, "{frame:?}: moved by {d:e}");
                        }
                    }
                }
            }
        }
        assert_eq!(built + refused, 22 * 22 * 22 * 6 * 4);
        assert!(built > 0 && refused > 0, "{built} {refused}");
    }

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
