//! **The mate solve** — the A9 partition and the constructive
//! placement (A11; INTENT stage 3 C).
//!
//! A placement's bundle is folded where the placement evaluates, in
//! the schedule's order, so every pose a mate reads is a value: `on`
//! off the copied shapes in their own coordinates, `to` off the target
//! space's geometry in that space's, or the world read inside the mate
//! (`eval::wire`'s placement arms read them). What the fold answers is
//! the copy's motion in its target space's coordinates, or a loose copy
//! that roots a space of its own. The entry points:
//!
//! - [`relative_freedom_components`] — A9's partition, over reads (so
//!   a placement's constraints couple its copy to their target).
//! - [`mate_coset`] — one mate's coset: the target pose's symmetry,
//!   carried onto the copy's pose.
//! - [`solve_bundle`] — a placement's mates folded by coset
//!   intersection ([`super::coset`]), and the pin's overconstraint
//!   named.
//! - [`admit_mate`] — the edit door's question of a constraint, the
//!   part of it the recipe decides.
//! - [`SolvedPoses`] — every placement's answer, read off an
//!   evaluation for a caller outside one.
//!
//! # Why the failure surface is per placement, not per document
//!
//! A refusing bundle must not fail an unrelated placement: GQ2/W5 make
//! a node's failure poison its descendants and nothing else. A fold
//! refuses as its placement's node does, and the placements that read
//! that copy are poisoned through it, as any reader is.

use std::collections::{BTreeMap, BTreeSet};

use geom_core::linalg::{Affine3, Mat3};
use geom_core::predicate::Band;
use geom_core::{Decide, Real};

use super::coset::{Arm, Coset, FoldStop, PoseSymmetry, Subgroup};
use super::{Clash, MateFault, MateRef, MateSide, Target};
use crate::doc::Doc;
use crate::node::{Node, RecipeNodeId};
use crate::placement::{Frame, Motion};
use crate::pose::PoseValue;

/// **The scalar a mate solve runs at** (`ASSEMBLY.md` A11 (5)): the
/// evaluation's own — `f64` for the nominal solve every door outside an
/// evaluation asks, a dual for a seed run, an interval for a box run.
/// Every branch inside the solve is a named predicate decided at this
/// scalar through [`Decide`] (a dual on its value channel, so a seed
/// never decides one; an interval definitely or by escalating).
///
/// What a scalar answers here beyond deciding is per scalar. No pose value is derived from
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

    /// **The solve's two-argument arctangent** — the angle the coset
    /// fold solves for where two rotation constraints meet, read only
    /// through its sine and cosine, and asked only of two radii decided
    /// nonzero, never at the origin. At `f64` the platform's `atan2`,
    /// the one the nominal solve has always read; a dual's value
    /// channel reads that same one, so its value is the `f64` lane's
    /// bits and its tangent is the dual's own; an interval's is an
    /// enclosure on a branch its arguments do not cross.
    fn solve_atan2(y: Self, x: Self) -> Self;

    /// **Whether `map` carries no variation of the run's own** — no
    /// tangent a seed run's parameter gives it, no symbol a symbolic
    /// run records in it — so a map decided at the identity can be the
    /// identity without losing what the run reads ([`rest_verdict`]). A
    /// value lane's and a box lane's answer `true`.
    fn carries_no_variation(map: &Affine3<Self>) -> bool;

    /// **The identity carrying `map`'s variation**, for a map decided at
    /// the identity that carries some ([`Self::carries_no_variation`]):
    /// the value channel the identity, as the value lane's snap makes
    /// it, and the run's variation `map`'s own. `None` where the lane
    /// cannot part the two, and the map is kept whole.
    fn identity_varying_as(map: &Affine3<Self>) -> Option<Affine3<Self>>;

    /// **`map` and the identity in one enclosure**, for a map the band
    /// cannot decide at rest or apart ([`rest_verdict`]): a box lane's
    /// map spans points the value lane snaps to the identity and points
    /// it keeps, so the box's answer holds both. `None` where the lane
    /// keeps its map (a value lane's undecided map is kept whole).
    fn with_rest(map: &Affine3<Self>) -> Option<Affine3<Self>>;
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

    fn solve_atan2(y: f64, x: f64) -> f64 {
        f64::atan2(y, x)
    }

    fn carries_no_variation(_map: &Affine3<f64>) -> bool {
        true
    }

    fn identity_varying_as(_map: &Affine3<f64>) -> Option<Affine3<f64>> {
        None
    }

    fn with_rest(_map: &Affine3<f64>) -> Option<Affine3<f64>> {
        None
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

    fn solve_atan2(y: Self, x: Self) -> Self {
        geom_core::Probe(f64::atan2(y.0, x.0))
    }

    fn carries_no_variation(_map: &Affine3<Self>) -> bool {
        true
    }

    fn identity_varying_as(_map: &Affine3<Self>) -> Option<Affine3<Self>> {
        None
    }

    fn with_rest(_map: &Affine3<Self>) -> Option<Affine3<Self>> {
        None
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

/// A placement's answer: its space and its motion there (`None` the
/// identity).
type Placed<T> = (Space, Option<Affine3<T>>);

/// **Every placement's answer, read off an evaluation** — for a caller
/// outside one: where each copy sits, which space it lives in, or why
/// its placement refused. Built by [`SolvedPoses::of`] from the
/// evaluation that folded each bundle; there is no second solve.
///
/// A solve is a solve OF a document, and it says so:
/// [`SolvedPoses::placement`] — the one door that takes a `Doc` back —
/// refuses a mispairing with it (DI3).
#[derive(Debug)]
pub struct SolvedPoses<T: Decide = f64> {
    /// **Which document this is a solve OF** (DI3).
    pub(crate) document: crate::ident::DocumentId,
    /// Each placement's answer: its space and its motion in that
    /// space's coordinates (`None` the identity), or its refusal —
    /// read off an evaluation by `SolvedPoses::of`, at the evaluation
    /// seam (`eval/mod.rs`).
    pub(crate) placed: BTreeMap<RecipeNodeId, Result<Placed<T>, MateFault>>,
}

/// **Why the nominal world-pose door has no pose** for a placement
/// ([`SolvedPoses::placement`]).
#[derive(Debug, Clone, PartialEq)]
pub enum PoseRefusal {
    /// The placement refused, in its own words.
    Mate(Box<MateFault>),
    /// The copy is loose, or pinned to a loose copy, so it has no world
    /// pose — only its pose in the loose copy's space
    /// ([`SolvedPoses::relative`]).
    Loose {
        /// The copy asked about.
        placement: RecipeNodeId,
        /// The loose copy whose space it lives in.
        root: RecipeNodeId,
    },
    /// The placement did not evaluate to a value: it failed for a
    /// reason not its bundle's, or was poisoned through an input.
    Unevaluated {
        /// The placement.
        placement: RecipeNodeId,
    },
}

impl crate::spoken::Say for PoseRefusal {
    fn say(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        by: crate::spoken::Speaker<'_>,
    ) -> core::fmt::Result {
        match self {
            Self::Mate(fault) => fault.say(f, by),
            Self::Loose { placement, root } if placement == root => write!(
                f,
                "{} is a loose copy: nothing pins it against the world, so it has no world pose",
                by.node(*placement)
            ),
            Self::Loose { placement, root } => write!(
                f,
                "{} is pinned to the loose copy {}, which nothing pins against the world, so it \
                 has no world pose",
                by.node(*placement),
                by.node(*root)
            ),
            Self::Unevaluated { placement } => {
                write!(
                    f,
                    "{} did not evaluate, so it has no pose",
                    by.node(*placement)
                )
            }
        }
    }
}

impl core::fmt::Display for PoseRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        crate::spoken::Say::say(self, f, crate::spoken::Speaker::TAG)
    }
}

impl core::error::Error for PoseRefusal {}

impl<T: SolveScalar> SolvedPoses<T> {
    /// **Which document this solve is of** (DI3).
    pub fn document(&self) -> crate::ident::DocumentId {
        self.document
    }

    /// A placement's recorded fault, if its bundle refused.
    pub fn fault(&self, node: RecipeNodeId) -> Option<&MateFault> {
        self.placed.get(&node)?.as_ref().err()
    }

    /// **The space a placement's copy lives in**, `None` if the node is
    /// not a placement that evaluated.
    pub fn space(&self, placement: RecipeNodeId) -> Option<Space> {
        Some(self.placed.get(&placement)?.as_ref().ok()?.0)
    }

    /// **A copy's pose in its space, at the evaluation's scalar** — a
    /// seed run's carrying its tangent, a box run's an enclosure over
    /// the box. `None` where the placement has no pose.
    pub fn relative_map(&self, placement: RecipeNodeId) -> Option<Affine3<T>> {
        let (_, motion) = self.placed.get(&placement)?.as_ref().ok()?;
        Some(motion.unwrap_or_else(Affine3::identity))
    }
}

/// The nominal solve's doors: its poses read as the frames a document
/// stores.
impl SolvedPoses {
    /// **Whether `placement`'s copy stands at the identity of the
    /// world's coordinates**: the fold's own verdict, read off its
    /// answer — a copy the fold decided at rest ([`rest_verdict`], at the
    /// evaluation's band and the unplaced copy's reach) was placed by no
    /// map, and one pinned straight to the world's coordinates by the
    /// stored identity. A copy that is loose, refused or unplaced does
    /// not stand there.
    pub fn at_identity(&self, placement: RecipeNodeId) -> bool {
        let Some(Ok((Space::World, motion))) = self.placed.get(&placement) else {
            return false;
        };
        motion.is_none_or(|m| f64::is_stored_identity(&m))
    }

    /// **A copy's pose in its space**: where it sits in the world's
    /// coordinates for a copy pinned against the world, in a loose
    /// copy's own for one pinned to it.
    pub fn relative(&self, placement: RecipeNodeId) -> Option<Frame> {
        self.relative_map(placement).map(Frame::from_affine)
    }

    /// **The copy's world placement, at the document's own parameters**.
    ///
    /// `doc` must be the document this solve is OF: the pairing is
    /// CHECKED here (DI3).
    ///
    /// # Errors
    ///
    /// [`PoseRefusal::Mate`] holding [`MateFault::PosesOfAnotherDocument`]
    /// when `doc` is not the document this solve is of, or the
    /// placement's own refusal; [`PoseRefusal::Loose`] when the copy is
    /// loose or pinned to one; [`PoseRefusal::Unevaluated`] when it has
    /// no value.
    pub fn placement<P>(
        &self,
        doc: &Doc<P>,
        placement: RecipeNodeId,
    ) -> Result<Frame, PoseRefusal> {
        if let Some(m) = crate::ident::mispaired(doc.id(), self.document) {
            return Err(PoseRefusal::Mate(Box::new(m.into())));
        }
        match self.placed.get(&placement) {
            None => Err(PoseRefusal::Unevaluated { placement }),
            Some(Err(fault)) => Err(PoseRefusal::Mate(Box::new(fault.clone()))),
            Some(Ok((Space::Loose { root }, _))) => Err(PoseRefusal::Loose {
                placement,
                root: *root,
            }),
            Some(Ok((Space::World, motion))) => {
                Ok(motion.map_or(Frame::IDENTITY, Frame::from_affine))
            }
        }
    }
}

/// **The document's solve, at its own parameters**: every placement's
/// answer, read off the document's nominal evaluation under `opts`
/// ([`SolvedPoses::of`]). There is no solve apart from the evaluation
/// that folds each bundle where its placement evaluates.
pub fn solve_document(
    doc: &Doc<crate::ProfileProgram>,
    opts: &crate::eval::EvalOptions,
    tol: geom_core::Tol,
) -> SolvedPoses {
    solve_document_at(doc, opts, tol)
}

/// **[`solve_document`] at a run's own scalar**: declared at the
/// evaluation seam, whose scalar bound it needs
/// ([`crate::eval::solve_document_at`]).
pub use crate::eval::solve_document_at;

/// **Whether a solved map stands at the identity** (stage 3 C, fork
/// C-5), three ways: `Some(true)` decided at rest, `Some(false)`
/// decided apart, `None` where the band cannot decide. At rest, no
/// point of the copy moves further than the band's zero — its
/// translation's length plus its turn's departure from the identity
/// levered over the copy's `reach` (D4 ¶1), each bounded above by a sum
/// of magnitudes. A copy pinned over its geometry at the identity
/// stands there to rounding; so decided, the fold places it by no map
/// where the run carries no variation in it
/// ([`SolveScalar::carries_no_variation`]), by the identity's value
/// channel with the map's own tangent where it does
/// ([`SolveScalar::identity_varying_as`]), and a box spanning the band
/// by a map holding both ([`SolveScalar::with_rest`]). A margin the
/// band cannot decide is not guessed. The one predicate the fold's snap
/// reads, decided once per placement; [`SolvedPoses::at_identity`]
/// reads the snap's answer.
pub(crate) fn rest_verdict<T: SolveScalar>(
    map: &Affine3<T>,
    reach: f64,
    band: Band,
) -> Option<bool> {
    use geom_core::linalg::Vec3;
    // Sums of magnitudes bound the lengths from above — the turn's
    // departure ‖Q − I‖ and the translation's length — with no square
    // root, which a box lane's enclosure of a near-zero length would
    // lose its decoration through.
    let (one, zero) = (T::one(), T::zero());
    let q = map.linear;
    let off = [
        q.c0 - Vec3::new(one, zero, zero),
        q.c1 - Vec3::new(zero, one, zero),
        q.c2 - Vec3::new(zero, zero, one),
    ];
    let magnitude = |v: Vec3<T>| v.x.abs() + v.y.abs() + v.z.abs();
    let turn = magnitude(off[0]) + magnitude(off[1]) + magnitude(off[2]);
    let moved = magnitude(map.translation) + turn * T::from_computed(reach);
    geom_core::k_stats::decide(
        "solve_copy_at_identity",
        geom_core::predicate::Margin::of(moved),
        band,
    )
    .ok()
    .map(|sign| sign == geom_core::predicate::Sign::Zero)
}

/// **The solved map as the composition rule takes it**: the stored
/// identity marked as such where the lane stores one
/// ([`SolveScalar::is_stored_identity`]), so a copy pinned straight to
/// its target's coordinates is placed by no map at every scalar.
pub(crate) fn marked<T: SolveScalar>(map: Affine3<T>) -> Motion<T> {
    if T::is_stored_identity(&map) {
        Motion::Identity
    } else {
        Motion::Map(map)
    }
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
    let mut couple = |x: RecipeNodeId, y: RecipeNodeId| {
        adjacency.entry(x).or_default().insert(y);
        adjacency.entry(y).or_default().insert(x);
    };
    for id in doc.ids() {
        let Some(node) = doc.node(id) else { continue };
        for input in doc.upstream_of(id, node) {
            couple(id, input);
        }
        // A world read is inside the mate and reads no variable, and a
        // world pin reads nothing, so the world is coupled here.
        let pinned =
            bundle_of(node).is_some_and(|bundle| bundle.iter().any(super::Constraint::reads_world));
        if pinned {
            couple(id, RecipeNodeId::WORLD);
        }
    }
    // The world seeds first and couples, but is no member: no edit
    // authored it.
    let order: Vec<RecipeNodeId> = std::iter::once(RecipeNodeId::WORLD)
        .chain(doc.ids())
        .collect();
    components(&order, &adjacency)
        .into_iter()
        .map(|members| {
            members
                .into_iter()
                .filter(|&id| id != RecipeNodeId::WORLD)
                .collect::<Vec<_>>()
        })
        .filter(|members| !members.is_empty())
        .collect()
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

// ---- The coset of one mate, and the fold ----

/// **One mate, read**: its address and its two poses, `on` in the
/// copied shapes' own coordinates and `to` in the target space's.
#[derive(Debug, Clone)]
pub(crate) struct ReadMate<T: Real> {
    /// The constraint's address.
    pub(crate) mate: MateRef,
    /// The copy's pose.
    pub(crate) on: PoseValue<T>,
    /// The pose it is equated with.
    pub(crate) to: PoseValue<T>,
}

/// **A pose's frame**: a rigid map carrying the pose of its kind at the
/// origin (a point at the origin, a direction or an axis or a plane's
/// normal along `z`, the identity frame) onto it. Which map is a
/// representative: two differ by an element of the kind's own
/// symmetry, so a coset built through either is the same coset.
fn frame_of<T: Real>(pose: &PoseValue<T>) -> Affine3<T> {
    let aimed = |origin: geom_core::linalg::Point3<T>, d: geom_core::UnitVec3<T>| {
        let (b1, b2) = d.orthonormal_basis();
        Affine3::from_parts(
            Mat3::from_cols(b1, b2, d.get()),
            origin - geom_core::linalg::Point3::origin(),
        )
    };
    match *pose {
        PoseValue::Point { position } => Affine3::from_parts(
            Mat3::identity(),
            position - geom_core::linalg::Point3::origin(),
        ),
        PoseValue::Direction { dir } => aimed(geom_core::linalg::Point3::origin(), dir),
        PoseValue::Axis { origin, dir } => aimed(origin, dir),
        PoseValue::Plane { origin, normal } => aimed(origin, normal),
        PoseValue::Frame(f) => f.to_affine(),
    }
}

/// **One mate's coset** (A11 (1), FORK-S3M): the copy's relative poses
/// `x` carrying `on` onto `to`. `x·F_on = F_to·h` for an `h` of the
/// kind's own symmetry, so `x` lies in `G·(F_to·F_on⁻¹)` with `G` the
/// target pose's symmetry in the target's coordinates
/// ([`PoseSymmetry`]). A plane's normal and an axis's direction are
/// kept, so a mate's sense is its `Flip`.
///
/// The two poses are of one kind: the door refuses two of different
/// kinds ([`MateFault::KindsDiffer`]).
pub fn mate_coset<T: SolveScalar>(on: &PoseValue<T>, to: &PoseValue<T>) -> Coset<T> {
    Coset {
        subgroup: to.symmetry(),
        representative: frame_of(to) * frame_of(on).inverse(),
    }
}

/// **A bundle's fold** (A11 rule 1): every mate of one placement,
/// intersected, at the fold's arm.
///
/// # Errors
///
/// The first refusal: an Indeterminate case split, an unleverable arm,
/// a meeting point out of range, or the CONTRADICTORY empty
/// intersection, which names both mates, the predicate, and the
/// measured clash.
pub(crate) fn fold_bundle<T: SolveScalar>(
    mates: &[ReadMate<T>],
    arm: Arm,
    band: Band,
) -> Result<Coset<T>, Box<MateFault>> {
    let mut held = Coset::unconstrained();
    let mut held_mate: Option<MateRef> = None;
    for m in mates {
        let mate = m.mate;
        let coset = mate_coset(&m.on, &m.to);
        // The fold's identity element costs nothing and decides
        // nothing: the first mate's coset is the fold.
        if matches!(held.subgroup, Subgroup::Se3) {
            held = coset;
            held_mate.get_or_insert(mate);
            continue;
        }
        held = match super::coset::intersect(held, coset, band, arm) {
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

/// What one bundle's fold placed its copy at.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Folded<T: Real> {
    /// Nothing pins it: a loose copy, the root of its own space.
    Loose,
    /// Pinned in its target space, at this map from its own
    /// coordinates.
    Pinned(Affine3<T>),
}

/// **One placement's bundle, folded.** A bundle that pins less than its
/// copy needs makes a loose copy, never a refusal; one whose mates pin
/// it with a mate the rest already fix refuses, naming every such
/// mate: each whose removal leaves the copy pinned. Constraints have no
/// order, so each is asked alone. The values then place the copy inside
/// what the mates leave ([`super::value::settle`]).
///
/// # Errors
///
/// The fold's own ([`fold_bundle`]), [`MateFault::Overconstrained`], or
/// a value's ([`super::value::settle`]).
pub(crate) fn solve_bundle<T: SolveScalar>(
    placement: RecipeNodeId,
    mates: &[ReadMate<T>],
    values: &[super::value::ReadValue<T>],
    charts: &BTreeMap<crate::node::ConstraintId, Option<super::value::Chart<T>>>,
    arm: Arm,
    band: Band,
) -> Result<Folded<T>, Box<MateFault>> {
    if mates.is_empty() {
        return Ok(Folded::Loose);
    }
    let coset = fold_bundle(mates, arm, band)?;
    if coset.subgroup.is_determined() && mates.len() > 1 {
        let mut redundant = Vec::new();
        for k in 0..mates.len() {
            let rest: Vec<ReadMate<T>> = mates
                .iter()
                .enumerate()
                .filter(|&(j, _)| j != k)
                .map(|(_, m)| m.clone())
                .collect();
            if fold_bundle(&rest, arm, band)?.subgroup.is_determined() {
                redundant.push(mates[k].mate.id);
            }
        }
        if !redundant.is_empty() {
            return Err(Box::new(MateFault::Overconstrained {
                placement,
                constraints: redundant,
                held: Subgroup::Trivial,
            }));
        }
    }
    Ok(
        match super::value::settle(coset, values, charts, arm, band)? {
            Some(at) => Folded::Pinned(at),
            None => Folded::Loose,
        },
    )
}

/// **Whether a mate's target reads its pose in the opposite sense**: a
/// flip, through any slides and spins of it — what the chart normalises
/// to the copy's side (ruling C-1).
pub(crate) fn target_flipped<P>(doc: &Doc<P>, to: &Target) -> bool {
    fn read<P>(doc: &Doc<P>, var: crate::VarId) -> bool {
        use crate::pose::PoseDef as D;
        match doc.var(var).map(crate::Var::def) {
            Some(crate::var::VarDef::Pose(D::Flip { pose })) => !read(doc, pose.read),
            Some(crate::var::VarDef::Pose(
                D::Slide { pose, .. } | D::Spin { pose, .. } | D::Carried { pose, .. },
            )) if pose.role.is_none() => read(doc, pose.read),
            _ => false,
        }
    }
    fn world(w: &super::WorldRead) -> bool {
        use super::WorldRead as W;
        match w {
            W::Flip { pose } => !world(pose),
            W::Slide { pose, .. } | W::Spin { pose, .. } => world(pose),
            W::Frame | W::Role { .. } => false,
        }
    }
    match to {
        Target::Pose(var) => read(doc, *var),
        Target::World(w) => world(w),
    }
}

/// **The fold's arm**: the copy's reach in its own coordinates, plus
/// the farthest a mate's two poses sit from their coordinates' origins
/// — the length an angle's verdict is levered over (D4 ¶1), through
/// the one door that admits it ([`Arm::of`]).
///
/// # Errors
///
/// [`MateFault::Unleverable`] carrying [`super::LeverRefusal::OutOfRange`]
/// against the first mate.
pub(crate) fn bundle_arm<T: SolveScalar>(
    reach: f64,
    mates: &[ReadMate<T>],
) -> Result<Arm, Box<MateFault>> {
    let from_origin = |pose: &PoseValue<T>| {
        let o = frame_of(pose).translation;
        (o.x.powi(2) + o.y.powi(2) + o.z.powi(2)).sqrt().upper()
    };
    let datum = mates
        .iter()
        .map(|m| from_origin(&m.on) + from_origin(&m.to))
        .fold(0.0_f64, f64::max);
    Arm::of(reach, datum).map_err(|refusal| {
        Box::new(MateFault::Unleverable {
            mate: mates.first().map_or_else(
                || unreachable!("an arm is formed for a bundle with a mate"),
                |m| m.mate,
            ),
            refusal: Box::new(refusal),
        })
    })
}

// ---- The door's questions of one constraint ----

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

/// **Whether `placement`'s operand row `(seat, read)` is a mate's `on`
/// read through an instance's own copy**: `Doc::upstream`'s one
/// exemption, the content key's, and the split between the reads an
/// evaluation resolves before its op and those it resolves against the
/// copy before placement. The one spelling every such reader asks.
pub(crate) fn own_copy_side<P>(
    doc: &Doc<P>,
    placement: RecipeNodeId,
    seat: crate::OperandSlot,
    read: crate::VarId,
) -> bool {
    matches!(
        seat,
        crate::OperandSlot::Side {
            side: MateSide::On,
            ..
        }
    ) && matches!(doc.node(placement), Some(Node::InstantiatePart { .. }))
        && doc.read_ports(read).iter().any(|&(at, _)| at == placement)
}

/// **Whether `on` reads the copy `placement` defines**, before
/// placement: every operation its definition reaches is one of a
/// `Place`'s shapes, or an instance's own output. A read that reaches
/// nothing live (its body deleted) is stranded, which is the
/// evaluation's to refuse (N5), never this door's.
pub(crate) fn on_reads_the_copy<P>(
    doc: &Doc<P>,
    placement: RecipeNodeId,
    on: crate::VarId,
) -> bool {
    let reached = doc.read_ports(on);
    let copied: Vec<RecipeNodeId> = match doc.node(placement) {
        Some(Node::Place { shapes, .. }) => shapes
            .iter()
            .filter_map(|shape| doc.operation_of(shape.read))
            .collect(),
        Some(Node::InstantiatePart { .. }) => vec![placement],
        _ => return false,
    };
    reached.iter().all(|(at, _)| copied.contains(at))
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

/// **One constraint's own admission** — what the recipe decides about a
/// mate, asked by the edit door of a constraint being written and by
/// the load door of every constraint it reads: `on` reads the copy
/// ([`MateFault::NotOnTheCopy`]), a world read is a pose
/// ([`MateFault::WorldRead`]), the two poses are of one kind
/// ([`MateFault::KindsDiffer`]), and the class is admitted. What the
/// fold decides — a contradiction, an overconstraint — needs the poses'
/// values, and is the placement's own refusal where it evaluates.
///
/// # Errors
///
/// The first of those faults.
pub(crate) fn admit_mate<P>(
    doc: &Doc<P>,
    mate: MateRef,
    data: &super::Mate,
) -> Result<(), Box<MateFault>> {
    if !on_reads_the_copy(doc, mate.placement, data.on) {
        return Err(Box::new(MateFault::NotOnTheCopy { mate }));
    }
    let kind = |var: crate::VarId| doc.var(var).map(crate::Var::kind);
    let on = kind(data.on);
    let to = match &data.to {
        Target::Pose(var) => kind(*var),
        Target::World(world) => Some(
            world
                .kind()
                .map_err(|fault| Box::new(MateFault::WorldRead { mate, fault }))?,
        ),
    };
    if let (Some(on), Some(to)) = (on, to)
        && on != to
    {
        return Err(Box::new(MateFault::KindsDiffer { mate, on, to }));
    }
    admit_class(mate, data.class)
}

/// **The named interim's world pin stands alone** ([`super::WorldPin`]):
/// a bundle holding one holds no mate or value. Answers whether
/// `bundle` holds a world pin, and refuses a bundle holding one beside
/// any other constraint ([`MateFault::Overconstrained`], naming every
/// constraint beside the pin: the pin already fixes the copy). Two
/// world pins say one thing, as a replayed log may write. The one
/// reading of the rule: the edit doors (`AddConstraint`, an inserted
/// placement), the load door and the fold each ask it of the whole
/// bundle.
///
/// # Errors
///
/// [`MateFault::Overconstrained`] naming the constraints beside the pin.
pub(crate) fn world_pin_stands_alone(
    placement: RecipeNodeId,
    bundle: &[super::Constraint],
) -> Result<bool, Box<MateFault>> {
    if !bundle.iter().any(|c| c.world_pin().is_some()) {
        return Ok(false);
    }
    let beside: Vec<_> = bundle
        .iter()
        .filter(|c| c.world_pin().is_none())
        .map(|c| c.id)
        .collect();
    if beside.is_empty() {
        Ok(true)
    } else {
        Err(Box::new(MateFault::Overconstrained {
            placement,
            constraints: beside,
            held: Subgroup::Trivial,
        }))
    }
}

/// **The body a mate's target pose is charted on** (ruling C-1): the
/// placement whose copy every read of `to` reaches — through a
/// [`crate::pose::PoseDef::Carried`], its copy — or `None` where every
/// read reaches only geometry in the space's own coordinates (a
/// construction, the world).
///
/// # Errors
///
/// [`super::NoChartWhy::TwoBodies`] where the reads reach two bodies:
/// two copies, or a copy and the space's own geometry.
pub(crate) fn chart_body<P>(
    doc: &Doc<P>,
    to: crate::VarId,
) -> Result<Option<RecipeNodeId>, super::NoChartWhy> {
    fn walk<P>(doc: &Doc<P>, var: crate::VarId, out: &mut Vec<Option<RecipeNodeId>>) {
        let placed = |var: crate::VarId| {
            doc.read_operation(var).filter(|&at| {
                matches!(
                    doc.node(at),
                    Some(Node::Place { .. } | Node::InstantiatePart { .. })
                )
            })
        };
        let mut push = |body: Option<RecipeNodeId>| {
            if !out.contains(&body) {
                out.push(body);
            }
        };
        match doc.var(var).map(crate::Var::def) {
            Some(crate::var::VarDef::Pose(crate::pose::PoseDef::Carried { copy, .. })) => {
                push(placed(*copy));
            }
            Some(crate::var::VarDef::Pose(def)) => {
                for (_, &read, _, _) in def.reads() {
                    walk(doc, read, out);
                }
            }
            _ => push(placed(var)),
        }
    }
    let mut bodies = Vec::new();
    walk(doc, to, &mut bodies);
    match bodies.as_slice() {
        [] => Ok(None),
        [one] => Ok(*one),
        _ => Err(super::NoChartWhy::TwoBodies),
    }
}

/// **One value's own admission** (ruling C-2, the named interim C-3):
/// what the recipe decides about it — it names a mate of `bundle`
/// ([`MateFault::ValueNamesNoMate`]), the mate's kind leaves its
/// freedom ([`MateFault::NotAFreedom`]) with a chart
/// ([`MateFault::NoChart`]), and the mate's target reads one body. Its
/// number's dimension is the slot's, which the lowering checks. Until
/// F, what needs the bundle's residual — a freedom the bundle's other
/// constraints already fix — is the placement's own refusal where it
/// evaluates.
///
/// # Errors
///
/// The first of those faults.
pub(crate) fn admit_value<P>(
    doc: &Doc<P>,
    value: MateRef,
    bundle: &[super::Constraint],
    data: &super::Value,
) -> Result<(), Box<MateFault>> {
    let Some(mate) = bundle
        .iter()
        .find(|c| c.id == data.mate)
        .and_then(super::Constraint::mate)
    else {
        return Err(Box::new(MateFault::ValueNamesNoMate {
            value,
            mate: data.mate,
        }));
    };
    if let Some(kind) = doc.var(mate.on).map(crate::Var::kind) {
        data.coord.admitted_by(kind).map_err(|refusal| {
            Box::new(match refusal {
                super::FreedomRefusal::NotAFreedom => MateFault::NotAFreedom {
                    value,
                    coord: data.coord,
                    kind: Some(kind),
                },
                super::FreedomRefusal::NoChart => MateFault::NoChart {
                    value,
                    why: super::NoChartWhy::PointRotation,
                },
            })
        })?;
    }
    if let Target::Pose(to) = &mate.to {
        chart_body(doc, *to).map_err(|why| Box::new(MateFault::NoChart { value, why }))?;
    }
    Ok(())
}
