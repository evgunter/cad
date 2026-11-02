//! **The mate solve** — the A9 partition and the constructive
//! placement (A11; INTENT stage 3 C).
//!
//! A placement's bundle is folded where the placement evaluates, in
//! the schedule's order, so every pose a mate reads is a value: `on`
//! off the copied shapes in their own coordinates, `to` off the target
//! space's geometry in that space's, or the world read inside the mate
//! (`eval::wire`'s placement arms read them). What the fold answers is
//! the copy's motion in its target space's coordinates. The entry
//! points:
//!
//! - [`relative_freedom_components`] — A9's partition, over reads (so
//!   a placement's constraints couple its copy to their target).
//! - [`mate_coset`] — one mate's coset: the target pose's symmetry,
//!   carried onto the copy's pose.
//! - [`overconstrained`] — the bundle rule: a constraint any of whose
//!   equations the rest already fix refuses, by subgroup algebra over
//!   the families its definitions name ([`super::coset`]), measuring
//!   nothing.
//! - [`solve_bundle`] — a placement's mates folded by coset
//!   intersection, constructed and never checked, and its values
//!   placed inside what the mates leave.
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

use super::coset::{Arm, Coset, PoseSymmetry, SubgroupFamily};
use super::{MateFault, MateRef, MateSide};
use crate::doc::Doc;
use crate::node::{Node, RecipeNodeId};
use crate::placement::Frame;
use crate::pose::PoseValue;

/// **The scalar a mate solve runs at** (`ASSEMBLY.md` A11 (5)): the
/// evaluation's own — `f64` for the nominal solve every door outside an
/// evaluation asks, a dual for a seed run, an interval for a box run.
/// Every branch inside the solve is a named predicate decided at this
/// scalar through [`Decide`] (a dual on its value channel, so a seed
/// never decides one; an interval definitely or by escalating).
///
/// What a scalar answers here beyond deciding is per scalar. No pose
/// value is derived from [`Self::quoted`] or [`Self::upper`]. `quoted`
/// feeds a sign and a payload, and it chooses between two spellings of
/// one value where each is exact and only their conditioning differs:
/// the value chart's two formulas for `1 + c` ([`super::value`]), and
/// [`frame_of`]'s pole a direction is aimed from. Such a choice is no
/// verdict and is not sampled; a box lane may take the other spelling
/// than its value lane, and each encloses the same value. `upper` forms
/// the arm ([`Arm::of`]), so it feeds the lever's own refusal
/// (`LeverRefusal::OutOfRange` where the arm is formed), and there an
/// over-stated arm refuses sooner, never wrongly.
pub trait SolveScalar: Decide {
    /// **The number a payload quotes for a measurement made at this
    /// scalar**, as `f64`: the value itself at `f64`; a dual's value
    /// channel, which is the `f64` lane's number; an interval's
    /// midpoint.
    fn quoted(self) -> f64;

    /// **An `f64` upper bound of a length measured at this scalar** —
    /// a side's `‖origin‖` term of the lever, which, like the parts'
    /// reach, only ever needs to over-state: the length itself at
    /// `f64`; a dual's value channel, the `f64` lane's number; an
    /// interval's `hi`, read through `Bounds` at the seam as the
    /// reach's is. It enters only the arm, and through it the lever's
    /// refusals.
    fn upper(self) -> f64;

    /// **The solve's two-argument arctangent** — the angle the coset
    /// fold solves for where two rotation constraints meet, read only
    /// through its sine and cosine, and asked only of two radii decided
    /// nonzero, never at the origin. At `f64` the platform's `atan2`,
    /// the one the nominal solve has always read; a dual's value
    /// channel reads that same one, so its value is the `f64` lane's
    /// bits and its tangent is the dual's own; an interval's is an
    /// enclosure on a branch its arguments do not cross.
    fn solve_atan2(y: Self, x: Self) -> Self;

    /// **Which branch [`Self::solve_atan2`] reads `y` over `x` on** — the
    /// principal branch, but for a box scalar: an exact zero `y` over a
    /// positive `x` is the angle zero, and across the negative `x` axis
    /// the branch about `π`, where the principal cut would widen the
    /// enclosure to the whole circle. A scalar recording its expression
    /// over a lane scalar reads the lane's branch, so its angle is the
    /// lane's on the same branch.
    fn atan2_branch(_y: Self, _x: Self) -> Atan2Branch {
        Atan2Branch::Principal
    }

    /// Whether a value's spin sets the map's turn from the two charts'
    /// frames ([`super::value`]) rather than turning the folded map by
    /// the measured difference.
    fn spins_by_frames() -> bool {
        false
    }

    /// **Whether this scalar is exactly zero** (a point enclosure of
    /// zero; never at `f64`, whose zero is read exactly anyway). It
    /// asks no predicate and skips none: it tightens what a reading
    /// encloses ([`tightened`], the measured spin `turn_to` undoes),
    /// which changes the inputs of the predicates that follow, so a box
    /// lane may decide one of them where it would otherwise escalate.
    fn exactly_zero(self) -> bool {
        false
    }
}

/// **The branch an angle is read on** ([`SolveScalar::atan2_branch`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Atan2Branch {
    /// The angle zero exactly: a zero `y` over a positive `x`.
    Zero,
    /// The branch about `π`: `y` spanning zero over a negative `x`.
    AboutPi,
    /// The principal branch.
    Principal,
}

impl SolveScalar for f64 {
    fn quoted(self) -> f64 {
        self
    }

    fn upper(self) -> f64 {
        self
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

    fn solve_atan2(y: Self, x: Self) -> Self {
        geom_core::Probe(f64::atan2(y.0, x.0))
    }
}

/// A placement's answer: the space its copy is in, its target read
/// off the recipe, and its motion there (`None` the identity).
type Placed<T> = (crate::VarId, Option<Affine3<T>>);

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
    /// The copy is placed into a space other than the world's, so it
    /// has no world pose: only its pose in that space
    /// ([`SolvedPoses::relative`]).
    OtherSpace {
        /// The copy asked about.
        placement: RecipeNodeId,
        /// The space it is placed into.
        space: crate::VarId,
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
            Self::OtherSpace { placement, space } => write!(
                f,
                "{} places its copy into {}, not the world, so it has no world pose",
                by.node(*placement),
                by.space(*space)
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

/// **What [`in_space`] composes**: a body value ([`crate::Framed`]), a
/// gathered product ([`crate::Product`]), or a picked body
/// ([`crate::NodePick`]), each by the frame it holds.
pub struct Composing<'a, T: geom_core::Real>(&'a crate::eval::Frame<T>);

impl<'a, T: geom_core::Decide> From<&'a crate::eval::Framed<T>> for Composing<'a, T> {
    fn from(value: &'a crate::eval::Framed<T>) -> Self {
        Self(value.frame())
    }
}

impl<'a, T: geom_core::Decide> From<&'a crate::Product<T>> for Composing<'a, T> {
    fn from(product: &'a crate::Product<T>) -> Self {
        Self(&product.frame)
    }
}

impl<'a> From<&'a crate::NodePick> for Composing<'a, f64> {
    fn from(pick: &'a crate::NodePick) -> Self {
        Self(&pick.frame)
    }
}

/// **The one composing door** (D10 `[placement.world]`): the map from a
/// body's own coordinates into those of its space — the world's, for a
/// product — or `None` where the value is in its space's coordinates
/// already (its frame is `Own`), which is the one identity test every
/// composer reads: structurally, never by comparing a map's entries.
///
/// Export, the façade's read-outs (`pncad::export`) and the viewer's
/// picture compose a body through it, and nothing else does
/// (`scripts/gates/in-space-composes-for-export-and-picture.sh`): every
/// operation computes in a frame of its reads and relates two operands
/// through [`crate::Framed::relative`], so what this answers never
/// re-enters the model.
#[must_use]
pub fn in_space<'a, T: geom_core::Decide>(
    value: impl Into<Composing<'a, T>>,
) -> Option<geom_core::RigidMotion<T>> {
    value.into().0.map().copied()
}

impl<T: SolveScalar> SolvedPoses<T> {
    /// **Which document this solve is of** (DI3).
    pub fn document(&self) -> crate::ident::DocumentId {
        self.document
    }

    /// A placement's recorded fault, if its bundle refused.
    pub fn fault(&self, node: RecipeNodeId) -> Option<&MateFault> {
        self.placed.get(&node)?.as_ref().err()
    }

    /// **The space a placement's copy is in**, its target, `None` if
    /// the node is not a placement that evaluated.
    pub fn space(&self, placement: RecipeNodeId) -> Option<crate::VarId> {
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
    /// world's coordinates**, read off its recipe: it is placed into the
    /// world by the identity bundle ([`identity_bundle`]), the one
    /// placement the fold places by no map. A copy in another space,
    /// refused or unplaced does not stand there, and neither does one
    /// whose map comes out as the identity: that is a value, not the
    /// recipe.
    pub fn at_identity(&self, placement: RecipeNodeId) -> bool {
        matches!(
            self.placed.get(&placement),
            Some(Ok((crate::VarId::WORLD, None)))
        )
    }

    /// **A copy's pose in its space**: where it sits in its target's
    /// coordinates.
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
    /// placement's own refusal; [`PoseRefusal::OtherSpace`] when the
    /// copy is in another space; [`PoseRefusal::Unevaluated`] when it has
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
            Some(Ok((crate::VarId::WORLD, motion))) => {
                Ok(motion.map_or(Frame::IDENTITY, Frame::from_affine))
            }
            Some(Ok((space, _))) => Err(PoseRefusal::OtherSpace {
                placement,
                space: *space,
            }),
        }
    }
}

/// **The document's solve, at its own parameters**: every placement's
/// answer, read off the document's nominal evaluation under `opts`
/// ([`SolvedPoses::of`]). There is no solve apart from the evaluation
/// that folds each bundle where its placement evaluates, which `cancel`
/// cancels: a placement the run did not reach answers its standing.
pub fn solve_document(
    doc: &Doc<crate::ProfileProgram>,
    opts: &crate::eval::EvalOptions,
    cancel: &crate::CancelToken,
    tol: geom_core::Tol,
) -> SolvedPoses {
    solve_document_at(doc, opts, cancel, tol)
}

/// **[`solve_document`] at a run's own scalar**: declared at the
/// evaluation seam, whose scalar bound it needs
/// ([`crate::eval::solve_document_at`]).
pub use crate::eval::solve_document_at;

/// **The solved map admitted as the motion it is**: through the decided
/// door ([`geom_core::RigidMotion::check_rigid`]) where that door calls
/// it, and re-witnessed where the band cannot. The solve composes its
/// map of rigid pieces only — the poses' orthonormal frames, turns
/// about witnessed axes, slides — so it is a rotation and a translation
/// at every point a box lane encloses, while the enclosure's own
/// columns can be too wide for the seven rigidity margins to read zero
/// (a pose read off geometry that a widened parameter moves). Such a
/// map is admitted by Gram–Schmidt over its first two columns
/// ([`geom_core::RigidMotion::gram_schmidt`], under
/// [`SOLVE_MAP_REWITNESS`]): each point's map is its own orthonormal
/// frame, so the witness encloses every point's map, and its third
/// axis is the cross product, which a map of turns alone holds.
/// Neither a decided defect nor a map the door calls is touched, so a
/// map read at a point keeps its bits.
///
/// Every mated placement's map passes this door, whole or as the part
/// the solve did not compose as motions ([`super::value::Composed`]); a
/// bundle with no mate is composed as motions and asks it nothing
/// ([`super::value::unmated_motion`]).
/// Where the band decides the seven margins, every lane decides the
/// same seven. Where a box lane's cannot, the undecided margin is
/// logged as an escalation, so that leaf never certifies, and the
/// re-witness's own decisions ride only such a leaf: they never make a
/// certified leaf's decisions differ from the nominal's.
///
/// # Errors
///
/// [`crate::NodeErrorKind::Motion`]: the decided door's refusal, where
/// the band decided a defect or the re-witness could not read the
/// columns' frame either.
pub(crate) fn admitted<T: SolveScalar>(
    map: &Affine3<T>,
    band: Band,
) -> Result<geom_core::RigidMotion<T>, crate::NodeErrorKind> {
    let refusal = match geom_core::RigidMotion::check_rigid(map, band) {
        Err(refusal @ geom_core::RigidMotionError::Undecided { .. }) => refusal,
        decided => return decided.map_err(crate::NodeErrorKind::Motion),
    };
    let origin = geom_core::linalg::Point3::origin() + map.translation;
    geom_core::RigidMotion::gram_schmidt(
        origin,
        map.linear.c0,
        map.linear.c1,
        SOLVE_MAP_REWITNESS,
        band,
    )
    .map_err(|_| crate::NodeErrorKind::Motion(refusal))
}

/// **A direction two of whose coordinates are exactly zero is the
/// third axis**, its sign the third coordinate's: the unit it is, read
/// exactly, where a box lane's normalisation of a length it cannot
/// cancel against itself leaves the third coordinate wider than one.
pub(crate) fn on_its_axis<T: SolveScalar>(d: geom_core::UnitVec3<T>) -> geom_core::UnitVec3<T> {
    on_an_axis(d.get()).unwrap_or(d)
}

/// **The axis a direction two of whose coordinates are exactly zero
/// is**, its sign the third coordinate's where its bounds decide it
/// with no band, read exactly: `None` for any other direction.
pub(crate) fn on_an_axis<T: SolveScalar>(
    v: geom_core::linalg::Vec3<T>,
) -> Option<geom_core::UnitVec3<T>> {
    use geom_core::UnitVec3;
    let (k, e) = match (v.x.exactly_zero(), v.y.exactly_zero(), v.z.exactly_zero()) {
        (false, true, true) => (UnitVec3::exact_x(), v.x),
        (true, false, true) => (UnitVec3::exact_y(), v.y),
        (true, true, false) => (UnitVec3::exact_z(), v.z),
        _ => return None,
    };
    if e.upper() < 0.0 {
        Some(-k)
    } else if (T::zero() - e).upper() < 0.0 {
        Some(k)
    } else {
        None
    }
}

/// A pose with every direction it carries read [`on_its_axis`].
pub(crate) fn tightened<T: SolveScalar>(pose: PoseValue<T>) -> PoseValue<T> {
    match pose {
        PoseValue::Direction { dir } => PoseValue::Direction {
            dir: on_its_axis(dir),
        },
        PoseValue::Axis(line) => PoseValue::axis_through(line.foot(), on_its_axis(line.dir())),
        PoseValue::Plane { normal, offset } => PoseValue::Plane {
            normal: on_its_axis(normal),
            offset,
        },
        other => other,
    }
}

/// The funnel name the re-witness of a solved map decides its frame
/// under ([`admitted`]).
pub(crate) const SOLVE_MAP_REWITNESS: &str = "solve_map_rewitness";

// ---- A9: relative freedom ----

/// **A9's relative-freedom partition**: the connected components of the
/// recipe over its reads ([`Doc::upstream`]), each component's nodes in
/// document order, the components themselves ordered by their first
/// node.
///
/// Two copies are relatively unconstrained exactly when they land in
/// different components — decidable from recipe structure alone, which
/// is the whole content of A9. A constraint couples its copy to its
/// target because the placement holding it reads the target, and a
/// placement's values chart on its target space's axes, so it is
/// coupled to the node defining that space: two copies placed into the
/// world share the world.
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
        // A space holds nothing, so a target is no upstream read: the
        // placement is coupled here to what defines its space.
        if bundle_of(node).is_some()
            && let Some(at) = doc.node_space(id).and_then(|space| doc.operation_of(space))
        {
            couple(id, at);
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
///
/// A direction `d = (a, b, c)` is aimed by the shortest turn carrying
/// `z` onto it, whose columns are rational in `d`'s components with the
/// one divisor `1 + c`; where `c < 0` it is aimed from `−z` instead
/// (divisor `1 − c`), after the half-turn about `x`. Which of the two
/// is a choice of arithmetic, not a verdict ([`SolveScalar`]): both
/// carry `z` to `d`, and at a point neither divisor falls below one
/// where it is taken (over an enclosure, below one by at most half its
/// width in `c`). So at `Sym` a
/// frame is a rational function of the pose, with no guard the
/// symbolic tier cannot see through.
fn frame_of<T: SolveScalar>(pose: &PoseValue<T>) -> Affine3<T> {
    use geom_core::linalg::{Point3, Vec3};
    let aimed = |origin: Point3<T>, d: geom_core::UnitVec3<T>| {
        let d = d.get();
        let (a, b, c) = (d.x, d.y, d.z);
        let one = T::one();
        let (x, y) = if c.quoted() < 0.0 {
            let k = one - c;
            (
                Vec3::new(one - a.powi(2) / k, T::zero() - a * b / k, a),
                Vec3::new(a * b / k, b.powi(2) / k - one, T::zero() - b),
            )
        } else {
            let k = one + c;
            (
                Vec3::new(one - a.powi(2) / k, T::zero() - a * b / k, T::zero() - a),
                Vec3::new(T::zero() - a * b / k, one - b.powi(2) / k, T::zero() - b),
            )
        };
        Affine3::from_parts(Mat3::from_cols(x, y, d), origin - Point3::origin())
    };
    match *pose {
        PoseValue::Point { position } => {
            Affine3::from_parts(Mat3::identity(), position - Point3::origin())
        }
        PoseValue::Direction { dir } => aimed(Point3::origin(), dir),
        PoseValue::Axis(line) => aimed(line.foot(), line.dir()),
        PoseValue::Plane { normal, offset } => aimed(PoseValue::plane_foot(normal, offset), normal),
        PoseValue::Frame(f) => f.to_affine(),
    }
}

/// The inverse of a rigid map, its linear part transposed: a frame's
/// columns are orthonormal by construction, so no determinant is
/// divided by.
fn rigid_inverse<T: Real>(map: Affine3<T>) -> Affine3<T> {
    let linear = map.linear.transpose();
    Affine3::from_parts(linear, -(linear * map.translation))
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
        representative: frame_of(to) * rigid_inverse(frame_of(on)),
    }
}

/// **One constraint's equations, as the bundle rule reads them**
/// (FORK-S3O): a mate's are its pose kind's symmetry, a value's the one
/// freedom it sets on the target space's (carried) axes. Read once per bundle, off its
/// definitions ([`equations_of`]), by the edit doors and by the solve
/// alike, so the rule the door asks and the rule the fold asks are one
/// reading.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Equations {
    /// A mate, by its kind's symmetry.
    Mate(SubgroupFamily),
    /// A value: the freedom it sets.
    Value(super::Coord),
}

impl Equations {
    /// How many equations it is.
    fn codimension(self) -> u8 {
        match self {
            Self::Mate(family) => family.codimension(),
            Self::Value(_) => 1,
        }
    }
}

/// A bundle's mates, by id and family.
fn mates_of(
    bundle: &[(crate::node::ConstraintId, Equations)],
) -> Vec<(crate::node::ConstraintId, SubgroupFamily)> {
    bundle
        .iter()
        .filter_map(|&(id, e)| match e {
            Equations::Mate(family) => Some((id, family)),
            Equations::Value(_) => None,
        })
        .collect()
}

/// A bundle's values, by id and freedom, in bundle order.
fn values_of(
    bundle: &[(crate::node::ConstraintId, Equations)],
) -> Vec<(crate::node::ConstraintId, super::Coord)> {
    bundle
        .iter()
        .filter_map(|&(id, e)| match e {
            Equations::Value(coord) => Some((id, coord)),
            Equations::Mate(_) => None,
        })
        .collect()
}

/// The family a set of mates leaves ([`super::pin::residual`]).
fn residual(mates: &[(crate::node::ConstraintId, SubgroupFamily)]) -> SubgroupFamily {
    super::pin::residual(mates.iter().map(|&(_, family)| family))
}

/// How many equations a set of mates fixes.
fn mates_codimension(mates: &[(crate::node::ConstraintId, SubgroupFamily)]) -> u8 {
    residual(mates).codimension()
}

/// Whether a set of mates repeats no equation: its families meet in
/// as many equations as they are.
fn independent(mates: &[(crate::node::ConstraintId, SubgroupFamily)]) -> bool {
    u16::from(mates_codimension(mates))
        == mates
            .iter()
            .map(|&(_, f)| u16::from(f.codimension()))
            .sum::<u16>()
}

/// **Whether a family fixes no direction**, so a value's spins are a
/// written chain about the target space's own axes: no mate, or a lone
/// point mate.
pub(crate) fn chained(left: SubgroupFamily) -> bool {
    left.frees_every_rotation()
}

/// **Whether the mates' residual `left` leaves `coord`** (D10
/// `[placement.values]`): the one question a value is asked of its
/// bundle, read off families alone. The solve reads a slide as a
/// translation equation and a spin as a rotation equation, and settles
/// each structurally inside what the mates leave.
///
/// - No direction fixed ([`chained`]): a slide where the residual frees
///   every translation, and a spin about an axis, one link of a chain.
/// - A direction fixed: the spin with no axis, and a slide along any
///   carried axis where the residual frees a translation — as many as
///   it frees ([`slides_left`]); which of them the mate takes is the
///   source's geometry's to say, decided once where the chart is read
///   ([`super::NoChartWhy::SlidesDoNotSpan`]).
pub(crate) fn leaves(left: SubgroupFamily, coord: super::Coord) -> bool {
    use super::Coord;
    use SubgroupFamily as F;
    match coord {
        Coord::Spin(Some(_)) => chained(left),
        Coord::Spin(None) => {
            !chained(left) && matches!(left, F::Planar | F::Cylindrical | F::Parallel | F::Revolute)
        }
        Coord::Slide(_) => slides_left(left) > 0,
    }
}

/// **How many slides the residual `left` leaves**
/// ([`SubgroupFamily::slides`]).
pub(crate) fn slides_left(left: SubgroupFamily) -> usize {
    left.slides()
}

/// Whether a mate of `family` leaves `coord`'s whole class of freedom:
/// the mates [`already_fixed`] names beside a value that is not the
/// residual's.
fn frees(family: SubgroupFamily, coord: super::Coord) -> bool {
    match coord {
        super::Coord::Slide(_) => family.frees_every_translation(),
        super::Coord::Spin(_) => family.frees_every_rotation(),
    }
}

/// **A bundle's values read against its mates' residual**: how many of
/// the copy's freedoms they write, and the values that write none of
/// the residual's or one another value writes, in bundle order.
///
/// Where no direction is fixed, the spins are a chain about the target
/// space's axes taken in bundle order, of at most three, no two in a
/// row about one axis, counted by the turns it writes; where a mate
/// fixes one, each freedom of its chart is written once, and no more
/// slides than the residual frees translations.
fn written(
    left: SubgroupFamily,
    values: &[(crate::node::ConstraintId, super::Coord)],
) -> (u8, Vec<crate::node::ConstraintId>) {
    use super::Coord;
    let mut named: Vec<crate::node::ConstraintId> = Vec::new();
    let mut name = |id| {
        if !named.contains(&id) {
            named.push(id);
        }
    };
    let mut set: Vec<(Coord, crate::node::ConstraintId)> = Vec::new();
    let mut spins: Vec<(super::TargetAxis, crate::node::ConstraintId)> = Vec::new();
    for &(id, coord) in values {
        if !leaves(left, coord) {
            name(id);
            continue;
        }
        match coord {
            Coord::Spin(Some(axis)) if chained(left) => {
                if let Some(&(last, before)) = spins.last()
                    && last == axis
                {
                    name(before);
                    name(id);
                }
                spins.push((axis, id));
            }
            _ => {
                if let Some(&(_, first)) = set.iter().find(|&&(c, _)| c == coord) {
                    name(first);
                    name(id);
                } else {
                    set.push((coord, id));
                }
            }
        }
    }
    if spins.len() > 3 {
        for &(_, id) in &spins {
            name(id);
        }
    }
    // More slides than the residual frees translations: each fixes an
    // equation the rest already fix.
    let slides: Vec<_> = set
        .iter()
        .filter(|&&(c, _)| matches!(c, Coord::Slide(_)))
        .map(|&(_, id)| id)
        .collect();
    let excess = slides.len().saturating_sub(slides_left(left));
    if excess > 0 {
        for &id in &slides {
            name(id);
        }
    }
    // The chain's turns, counted as written: a link about the axis of
    // the one before it turns no further freedom.
    let turns = spins
        .iter()
        .enumerate()
        .filter(|&(k, &(axis, _))| k == 0 || spins[k - 1].0 != axis)
        .count()
        .min(3);
    let count = u8::try_from(set.len() - excess + turns)
        .unwrap_or(u8::MAX)
        .min(6);
    let order: Vec<_> = values
        .iter()
        .map(|&(id, _)| id)
        .filter(|id| named.contains(id))
        .collect();
    (count, order)
}

/// **How many equations a bundle fixes**, read off its families alone:
/// its mates met ([`mates_codimension`]), and the freedoms of their
/// residual its values write ([`written`]).
fn codimension(bundle: &[(crate::node::ConstraintId, Equations)]) -> u8 {
    let mates = mates_of(bundle);
    let (set, _) = written(residual(&mates), &values_of(bundle));
    mates_codimension(&mates).saturating_add(set).min(6)
}

/// **The constraints any of whose equations the rest of `bundle`
/// already fixes** (FORK-S3O, A11 (1)). A bundle fixes no equation
/// twice exactly when its count is the sum of its members' counts, so
/// that is the test, and it is order-free. Which to name is read in
/// two steps, the mates first, since a value lives inside what they
/// leave:
///
/// - where the mates fix an equation twice, each mate the others fix
///   an equation of (`codim(M∖m) + codim(m) > codim(M)`), and no value;
/// - else each value that writes no freedom of the mates' residual,
///   with the mates that fix its class; each value that writes a
///   freedom another writes too; and, past six equations, every value.
fn already_fixed(
    bundle: &[(crate::node::ConstraintId, Equations)],
) -> Vec<crate::node::ConstraintId> {
    let total: u16 = bundle
        .iter()
        .map(|&(_, e)| u16::from(e.codimension()))
        .sum();
    if u16::from(codimension(bundle)) == total {
        return Vec::new();
    }
    let mates = mates_of(bundle);
    let named = |id: crate::node::ConstraintId, named: &mut Vec<crate::node::ConstraintId>| {
        if !named.contains(&id) {
            named.push(id);
        }
    };
    let mut out = Vec::new();
    if !independent(&mates) {
        let met = mates_codimension(&mates);
        for (k, &(id, family)) in mates.iter().enumerate() {
            let rest: Vec<_> = mates
                .iter()
                .enumerate()
                .filter(|&(j, _)| j != k)
                .map(|(_, &m)| m)
                .collect();
            if mates_codimension(&rest) + family.codimension() > met {
                named(id, &mut out);
            }
        }
        return out;
    }
    let left = residual(&mates);
    let values = values_of(bundle);
    let (_, twice) = written(left, &values);
    // More slides than the mates leave translations: the mates that fix
    // a translation fix the one too many.
    let slides = super::TargetAxis::ALL
        .into_iter()
        .filter(|&a| {
            let c = super::Coord::Slide(a);
            leaves(left, c) && values.iter().any(|&(_, v)| v == c)
        })
        .count();
    let excess = slides > slides_left(left);
    for &(id, coord) in &values {
        if excess && matches!(coord, super::Coord::Slide(_)) {
            for &(other, family) in &mates {
                if !frees(family, coord) {
                    named(other, &mut out);
                }
            }
        }
        if !leaves(left, coord) {
            for &(other, family) in &mates {
                if !frees(family, coord) {
                    named(other, &mut out);
                }
            }
        }
        if twice.contains(&id) {
            named(id, &mut out);
        }
    }
    if out.is_empty() {
        for &(id, _) in &values {
            named(id, &mut out);
        }
    }
    // The bundle's order, whatever order the walk named them in.
    bundle
        .iter()
        .map(|&(id, _)| id)
        .filter(|id| out.contains(id))
        .collect()
}

/// **A slide the mates leave no translation to set**: where the mates
/// repeat no equation, the first slide beside a ball and a direction,
/// the one pair that meets transversally. In general position it would
/// fix the ball's turn, so it is no repeat, and it refuses
/// [`super::NoChartWhy::SlideTurnsABall`], not as an overconstraint.
fn slide_beside_a_ball(
    bundle: &[(crate::node::ConstraintId, Equations)],
) -> Option<crate::node::ConstraintId> {
    let mates = mates_of(bundle);
    let ball = mates.iter().any(|&(_, f)| f == SubgroupFamily::Spherical);
    let direction = mates.iter().any(|&(_, f)| f == SubgroupFamily::Parallel);
    if !(independent(&mates) && ball && direction) {
        return None;
    }
    values_of(bundle)
        .into_iter()
        .find(|&(_, coord)| matches!(coord, super::Coord::Slide(_)))
        .map(|(id, _)| id)
}

/// **The bundle rule** (FORK-S3O): `placement`'s constraints, read as
/// equations, refuse [`MateFault::Overconstrained`] where any of them
/// fixes an equation the others already fix, pinned or not, naming
/// every such constraint. Asked by every edit door that writes a
/// constraint or a bundle, and by the fold, for a state no edit door
/// saw (a loaded save: the load door asks only each constraint's own
/// admission).
///
/// # Errors
///
/// [`MateFault::Overconstrained`], with the freedoms the whole bundle
/// leaves.
pub(crate) fn overconstrained(
    placement: RecipeNodeId,
    bundle: &[(crate::node::ConstraintId, Equations)],
) -> Result<(), Box<MateFault>> {
    let named = already_fixed(bundle);
    if named.is_empty() {
        return Ok(());
    }
    if let Some(value) = slide_beside_a_ball(bundle) {
        return Err(Box::new(MateFault::NoChart {
            value: MateRef {
                placement,
                id: value,
            },
            why: super::NoChartWhy::SlideTurnsABall,
        }));
    }
    Err(Box::new(MateFault::Overconstrained {
        placement,
        constraints: named,
        free: 6 - codimension(bundle),
    }))
}

/// **Whether a bundle pins its copy**: its equations fix all six of
/// the copy's freedoms. Decided by codimension alone, so a copy's
/// space is the recipe's, never a value's.
pub(crate) fn pins(bundle: &[(crate::node::ConstraintId, Equations)]) -> bool {
    codimension(bundle) == 6
}

/// **Every bundle pins its copy** (D10 `[placement.placement]`): its
/// mates and values together set every freedom, so the door refuses a
/// bundle that leaves one unwritten, with mates or without, an empty
/// one included, and an unwritten freedom cannot be represented.
///
/// # Errors
///
/// [`MateFault::Unpinned`], with the freedoms the bundle leaves.
pub(crate) fn unpinned(
    placement: RecipeNodeId,
    bundle: &[(crate::node::ConstraintId, Equations)],
) -> Result<(), Box<MateFault>> {
    if pins(bundle) {
        return Ok(());
    }
    Err(Box::new(MateFault::Unpinned {
        placement,
        free: 6 - codimension(bundle),
    }))
}

/// **Whether a bundle is the identity bundle as written** (D10: "a
/// placement whose values are all zero and which has no mate is the
/// identity"): no mate, and every value the constant `0`, VR6's lone
/// constant. Read off the recipe and never off a value, so the one
/// placement the fold places by no map is the same at every scalar and
/// every value of the document's variables. The bundle rule
/// ([`overconstrained`], [`unpinned`]) is asked before it, so the
/// values are the six freedoms.
pub(crate) fn identity_bundle<P>(doc: &Doc<P>, bundle: &[super::Constraint]) -> bool {
    let defs = crate::param_source::definitions_of(doc);
    !bundle.is_empty()
        && bundle
            .iter()
            .all(|c| c.value().is_some_and(|v| written_as_zero(&defs, v.by)))
}

/// **Whether `var` is defined as the lone constant `0`** — the recipe
/// form [`identity_bundle`] reads, and which a content key therefore
/// keys beside the value's bits.
pub(crate) fn written_as_zero(
    defs: crate::param_source::Definitions<'_, '_>,
    var: crate::VarId,
) -> bool {
    defs(var).is_some_and(crate::expr::Expr::is_zero)
}

/// **A bundle's equations, read off its definitions** — what the doors
/// ask the bundle rule of. A mate whose `on` reads no live variable
/// states none: that is the evaluation's to refuse.
pub(crate) fn equations_of<P>(
    doc: &Doc<P>,
    bundle: &[super::Constraint],
) -> Vec<(crate::node::ConstraintId, Equations)> {
    bundle
        .iter()
        .filter_map(|c| {
            let e = match &c.kind {
                super::ConstraintKind::Mate(mate) => Equations::Mate(
                    doc.var(mate.on)
                        .map(crate::Var::kind)
                        .and_then(crate::VarKind::symmetry)?,
                ),
                super::ConstraintKind::Value(value) => Equations::Value(value.coord),
            };
            Some((c.id, e))
        })
        .collect()
}

/// **A bundle's mates, folded** (A11 (1)): their cosets met
/// ([`super::coset::intersect`]), constructed and never checked. The
/// bundle rule has admitted the bundle, so every meeting is transverse.
///
/// # Errors
///
/// [`MateFault::Overconstrained`] naming every mate where two do not
/// meet transversally (a pose whose kind is not its definition's, which
/// the pose evaluator refuses first).
///
/// **The meeting is always a finite length away.** The bundle's arm
/// ([`bundle_arm`]) is formed before the solve, and [`Arm::of`] refuses
/// one whose sixteenfold square overflows, so every mate's two poses
/// sit within `datum < 3.4e153` of their origins. A lone mate's
/// representative moves by at most `|o_on| + |o_to| ≤ datum`, and the
/// one transverse meeting ([`super::coset::intersect`], a ball and a
/// direction) by `p − R·p + R·t`, at most `3·datum`, whose squared
/// norm stays inside the format. So no range is asked here. The bound
/// is a point scalar's: an enclosure lane's product of rotation
/// enclosures can widen past it, and a translation that is then no
/// bounded number refuses where the placed copy's motion is minted
/// (`RigidMotion`'s mints ask `Real::is_bounded`), never silently.
pub(crate) fn fold_bundle<T: SolveScalar>(
    placement: RecipeNodeId,
    mates: &[ReadMate<T>],
) -> Result<Coset<T>, Box<MateFault>> {
    let mut held = Coset::unconstrained();
    for m in mates {
        held = super::coset::intersect(held, mate_coset(&m.on, &m.to)).ok_or_else(|| {
            let met = mates
                .iter()
                .fold(SubgroupFamily::Se3, |held, m| {
                    held.meet(mate_coset(&m.on, &m.to).subgroup.family())
                })
                .codimension();
            Box::new(MateFault::Overconstrained {
                placement,
                constraints: mates.iter().map(|m| m.mate.id).collect(),
                free: 6 - met,
            })
        })?;
    }
    Ok(held)
}

/// What one bundle's fold placed its copy at, in its target space.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Folded<T: Real> {
    /// At this map from its own coordinates, and the map as the rigid
    /// motion its composition made, where the lane composes one.
    Pinned(Affine3<T>, Option<super::value::Composed<T>>),
    /// By the motion the values compose: a bundle with no mate
    /// ([`super::value::unmated_motion`]), or a lone point mate's chain
    /// ([`super::value::point_motion`]).
    Chained(geom_core::RigidMotion<T>),
}

/// **Whether `placement`'s bundle is one the doors admit**: each mate's
/// own admission ([`admit_mate`]), the bundle rule ([`overconstrained`])
/// and the pin ([`unpinned`]), the reading the edit doors and the solve
/// ask, over the recipe alone. A loaded or rebound bundle no edit door
/// saw can fail it; the solve then refuses the placement, and what the
/// bundle's mates say is no statement.
pub(crate) fn admits<P>(
    doc: &Doc<P>,
    placement: RecipeNodeId,
    bundle: &[super::Constraint],
) -> bool {
    let equations = equations_of(doc, bundle);
    bundle.iter().all(|c| {
        c.mate()
            .is_none_or(|mate| admit_mate(doc, MateRef::new(placement, c.id), mate).is_ok())
    }) && overconstrained(placement, &equations).is_ok()
        && unpinned(placement, &equations).is_ok()
}

/// **One placement's bundle, solved.** `equations` are the bundle's,
/// read once off its definitions ([`equations_of`]) — the reading the
/// edit doors ask the rule of, so the door and the solve decide over
/// one value. The bundle rule ([`overconstrained`]) refuses a
/// constraint any of whose equations the rest already fix, a value
/// writing no freedom the mates leave among them, and every bundle pins
/// its copy ([`unpinned`]). A bundle with no mate, or with a lone point
/// mate, places its copy by the motion its values compose; any other
/// folds its mates, and the values place the copy inside what they
/// leave ([`super::value::settle`]) on the chart of the one mate fixing
/// a direction.
///
/// # Errors
///
/// [`MateFault::Overconstrained`], [`MateFault::NoChart`],
/// [`MateFault::Unpinned`], the fold's own ([`fold_bundle`]), or a
/// chart degenerate at its values ([`super::value::settle`]).
pub(crate) fn solve_bundle<T: SolveScalar>(
    placement: RecipeNodeId,
    equations: &[(crate::node::ConstraintId, Equations)],
    mates: &[ReadMate<T>],
    values: &[super::value::ReadValue<T>],
    chart: Option<super::value::Chart<T>>,
) -> Result<Folded<T>, crate::NodeErrorKind> {
    let fault = |f: MateFault| crate::NodeErrorKind::Mate(Box::new(f));
    overconstrained(placement, equations).map_err(|f| fault(*f))?;
    unpinned(placement, equations).map_err(|f| fault(*f))?;
    match mates {
        [] => return Ok(Folded::Chained(super::value::unmated_motion(values)?)),
        [
            ReadMate {
                on: PoseValue::Point { position: on },
                to: PoseValue::Point { position: to },
                ..
            },
        ] => {
            return Ok(Folded::Chained(super::value::point_motion(
                *on, *to, values,
            )?));
        }
        _ => {}
    }
    let coset = fold_bundle(placement, mates).map_err(|f| fault(*f))?;
    let (at, rigid) = super::value::settle(coset, values, chart)?;
    Ok(Folded::Pinned(at, rigid))
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
        Some(Node::Place { source, .. }) => doc.operation_of(source.read).into_iter().collect(),
        Some(Node::InstantiatePart { .. }) => vec![placement],
        _ => return false,
    };
    reached.iter().all(|(at, _)| copied.contains(at))
}

/// **One constraint's own admission** — what the recipe decides about a
/// mate, asked by the edit door of a constraint being written and by
/// the load door of every constraint it reads: `on` reads the copy
/// ([`MateFault::NotOnTheCopy`]), the two poses are of one kind
/// ([`MateFault::KindsDiffer`]), and a point or a frame mate carries no
/// orientation ([`MateFault::Unoriented`]). Whether it fixes an equation
/// the rest of its bundle already fixes is the bundle rule's
/// ([`overconstrained`]).
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
    if let (Some(on), Some(to)) = (on, kind(data.to))
        && on != to
    {
        return Err(Box::new(MateFault::KindsDiffer { mate, on, to }));
    }
    // A point or a frame mate fixes no one direction, so it carries no
    // orientation.
    if data.orient != super::Orientation::IDENTITY
        && on
            .and_then(crate::VarKind::symmetry)
            .is_some_and(|f| matches!(f, SubgroupFamily::Spherical | SubgroupFamily::Trivial))
    {
        return Err(Box::new(MateFault::Unoriented { mate }));
    }
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::{Equations, overconstrained, pins, unpinned};
    use crate::mate::coset::SubgroupFamily as F;
    use crate::mate::{Coord, MateFault, NoChartWhy, TargetAxis};
    use crate::node::ConstraintId;
    use crate::{MintId, RecipeNodeId};

    fn id(k: u32) -> ConstraintId {
        ConstraintId(MintId::new(k, u64::from(k)))
    }

    fn value(mate: u32, coord: Coord) -> (ConstraintId, Equations) {
        (
            id(100 + mate * 10 + u32::from(matches!(coord, Coord::Spin(_)))),
            Equations::Value(coord),
        )
    }

    /// The `k`th value of a bundle with no mate.
    fn unmated(k: u32, coord: Coord) -> (ConstraintId, Equations) {
        (id(200 + k), Equations::Value(coord))
    }

    /// The rule's verdict on `bundle`: `Ok(pinned)` where it admits,
    /// the named constraints and the freedoms left where it refuses.
    fn verdict(bundle: &[(ConstraintId, Equations)]) -> Result<bool, (Vec<ConstraintId>, u8)> {
        match overconstrained(RecipeNodeId::WORLD, bundle) {
            Ok(()) => Ok(pins(bundle)),
            Err(fault) => match *fault {
                MateFault::Overconstrained {
                    constraints, free, ..
                } => Err((constraints, free)),
                other => panic!("the rule refuses only Overconstrained, got {other:?}"),
            },
        }
    }

    /// **No slide the rule admits sits beside a second mate** — the
    /// premise `value::settle` moves each slide along its own chart's
    /// axis on. A slide is its own only beside mates that free every
    /// translation, which only a direction mate does; a direction mate
    /// beside a mate of any other kind, or another direction, repeats an
    /// equation, and beside a ball the slide refuses as having no chart.
    /// So every slide a bundle sets is on one lone mate's chart, whose
    /// `x`, `y` and `d` lie in its residual and are perpendicular.
    ///
    /// *Breaks if* a pair of mates comes to admit a slide on either: the
    /// settle would then move a slide's copy across a translation the
    /// other mate fixes (a ball and a direction with a slide moves the
    /// copy off the ball), where no structural move is right.
    #[test]
    fn no_admitted_slide_sits_beside_a_second_mate() {
        let kinds = [
            F::Trivial,
            F::Planar,
            F::Cylindrical,
            F::Parallel,
            F::Spherical,
        ];
        // A slide each kind's chart leaves, where it leaves one.
        let slide = |f: F| match f {
            F::Planar | F::Parallel => Some(Coord::Slide(TargetAxis::X)),
            F::Cylindrical => Some(Coord::Slide(TargetAxis::Z)),
            _ => None,
        };
        for a in kinds {
            for b in kinds {
                let Some(on_b) = slide(b) else { continue };
                let bundle = [
                    (id(1), Equations::Mate(a)),
                    (id(2), Equations::Mate(b)),
                    value(2, on_b),
                ];
                assert!(
                    overconstrained(RecipeNodeId::WORLD, &bundle).is_err(),
                    "{a:?} beside {b:?} admits a slide on the {b:?}"
                );
            }
        }
    }

    /// **A value counts beside its own mate, and beside another only
    /// where that one leaves its freedom** (spec §4, C-2): a lone plane
    /// admits a slide, an axis a slide along it and a spin, a ball and a
    /// direction a spin about the direction (the ball frees every
    /// rotation) but no slide (the ball fixes every translation, so the
    /// slide has no chart: `SlideTurnsABall`), and a
    /// plane beside an axis fixes, in general position, more than the
    /// two count apart, so both are named.
    #[test]
    fn the_four_value_availability_rows() {
        // A lone Plane with Slide(X): admitted by the rule, unpinned.
        assert_eq!(
            verdict(&[
                (id(1), Equations::Mate(F::Planar)),
                value(1, Coord::Slide(TargetAxis::X))
            ]),
            Ok(false)
        );
        // Cylindrical with Slide(Z) and Spin: admitted, pinned.
        assert_eq!(
            verdict(&[
                (id(1), Equations::Mate(F::Cylindrical)),
                value(1, Coord::Slide(TargetAxis::Z)),
                value(1, Coord::SPIN),
            ]),
            Ok(true)
        );
        // Point + Direction with a spin on the direction: admitted, pinned.
        let ball_and_direction = [
            (id(1), Equations::Mate(F::Spherical)),
            (id(2), Equations::Mate(F::Parallel)),
        ];
        assert_eq!(verdict(&ball_and_direction), Ok(false));
        let spun = [ball_and_direction.to_vec(), vec![value(2, Coord::SPIN)]].concat();
        assert_eq!(verdict(&spun), Ok(true));
        // ... and with a slide on the direction: no repeat (in general
        // position it would fix the ball's turn), but no translation the
        // ball leaves it to set, so it refuses as having no chart.
        let slid = [
            ball_and_direction.to_vec(),
            vec![value(2, Coord::Slide(TargetAxis::Z))],
        ]
        .concat();
        match *overconstrained(RecipeNodeId::WORLD, &slid).expect_err("the slide refuses") {
            MateFault::NoChart { value, why } => {
                assert_eq!(value.id, slid[2].0);
                assert_eq!(why, NoChartWhy::SlideTurnsABall);
            }
            other => panic!("a slide beside a ball has no chart, got {other:?}"),
        }
        // Plane + Axis: each fixes what the other does.
        assert_eq!(
            verdict(&[
                (id(1), Equations::Mate(F::Planar)),
                (id(2), Equations::Mate(F::Cylindrical)),
            ]),
            Err((vec![id(1), id(2)], 0))
        );
    }

    /// **Where the mates fix an equation twice, the mates are named,
    /// and no value**: a value lives inside what the mates leave, so
    /// deleting it never answers a refusal it did not cause. Two
    /// direction mates fix one rotation twice whatever values hang off
    /// either (a value of a dropped mate still counts as its equation),
    /// and a slide on one of two planes is not what the planes repeat.
    #[test]
    fn mates_that_repeat_an_equation_are_named_whatever_values_they_carry() {
        let x = Coord::Slide(TargetAxis::X);
        let directions = [
            (id(1), Equations::Mate(F::Parallel)),
            value(1, x),
            (id(2), Equations::Mate(F::Parallel)),
            value(2, x),
        ];
        assert!(matches!(verdict(&directions), Err((named, _)) if named == vec![id(1), id(2)]));
        let reversed: Vec<_> = directions.iter().rev().copied().collect();
        assert!(matches!(verdict(&reversed), Err((named, _)) if named == vec![id(2), id(1)]));
        let planes = [
            (id(1), Equations::Mate(F::Planar)),
            value(1, x),
            (id(2), Equations::Mate(F::Planar)),
        ];
        assert!(matches!(verdict(&planes), Err((named, _)) if named == vec![id(1), id(2)]));
        for (on, other) in [(F::Planar, F::Parallel), (F::Cylindrical, F::Parallel)] {
            let coord = if on == F::Planar {
                x
            } else {
                Coord::Slide(TargetAxis::Z)
            };
            let bundle = [
                (id(1), Equations::Mate(on)),
                value(1, coord),
                (id(2), Equations::Mate(other)),
            ];
            assert!(
                matches!(verdict(&bundle), Err((ref named, _)) if *named == vec![id(1), id(2)]),
                "{on:?} with a value beside {other:?}: {:?}",
                verdict(&bundle)
            );
        }
    }

    /// **Two values of one freedom name each other**, and
    /// nothing else: the mate fixes nothing they repeat.
    #[test]
    fn a_freedom_set_twice_names_its_two_values() {
        let x = Coord::Slide(TargetAxis::X);
        let bundle = [
            (id(1), Equations::Mate(F::Planar)),
            (id(7), Equations::Value(x)),
            (id(8), Equations::Value(x)),
        ];
        assert_eq!(verdict(&bundle), Err((vec![id(7), id(8)], 2)));
    }

    /// **A bundle with no mate writes every freedom, once** (D10
    /// `[placement.values]`, the door's rule): three slides and a chain
    /// of three spins, no two in a row about one axis, pin the copy; a
    /// repeat in a row, a fourth spin and a second slide along one axis
    /// each fix a freedom twice and name what does; a chain that leaves
    /// a freedom refuses as unpinned.
    ///
    /// *Breaks if* the rule counts a chain's links by axis rather than
    /// by position (ZXZ would refuse), lets a fourth spin or a repeat in
    /// a row through, or admits a bundle that leaves a freedom.
    #[test]
    fn a_bundle_with_no_mate_writes_every_freedom_once() {
        use TargetAxis::{X, Y, Z};
        let slides = [
            unmated(0, Coord::Slide(X)),
            unmated(1, Coord::Slide(Y)),
            unmated(2, Coord::Slide(Z)),
        ];
        let chain = |axes: &[TargetAxis]| -> Vec<(ConstraintId, Equations)> {
            let mut bundle = slides.to_vec();
            for (k, &axis) in (3u32..).zip(axes) {
                bundle.push(unmated(k, Coord::Spin(Some(axis))));
            }
            bundle
        };
        let unwritten =
            |bundle: &[(ConstraintId, Equations)]| match unpinned(RecipeNodeId::WORLD, bundle)
                .map_err(|f| *f)
            {
                Ok(()) => None,
                Err(MateFault::Unpinned { free, .. }) => Some(free),
                Err(other) => panic!("unpinned refuses only Unpinned, got {other:?}"),
            };
        for axes in [[X, Y, Z], [Z, X, Z], [Z, Y, X]] {
            let bundle = chain(&axes);
            assert_eq!(verdict(&bundle), Ok(true), "{axes:?} pins its copy");
            assert_eq!(unwritten(&bundle), None, "{axes:?} writes every freedom");
        }
        assert_eq!(
            verdict(&chain(&[X, X, Y])),
            Err((vec![id(203), id(204)], 1)),
            "two spins in a row about x name both and turn one freedom"
        );
        assert_eq!(
            verdict(&chain(&[X, Y, Z, X])),
            Err((vec![id(203), id(204), id(205), id(206)], 0)),
            "a fourth spin names the chain"
        );
        let twice = [chain(&[X, Y, Z]), vec![unmated(9, Coord::Slide(Y))]].concat();
        assert_eq!(
            verdict(&twice),
            Err((vec![id(201), id(209)], 0)),
            "two slides along y name both"
        );
        let short = chain(&[Z, X]);
        assert_eq!(verdict(&short), Ok(false), "two spins fix no freedom twice");
        assert_eq!(unwritten(&short), Some(1), "and leave one unwritten");
        assert_eq!(unwritten(&[]), Some(6), "an empty bundle pins nothing");
    }

    /// **A lone point mate's rotation is a chain** (D10
    /// `[placement.values]`, test 20): three spins about the target
    /// space's axes, no two in a row about one axis, pin a ball's copy;
    /// a second spin about x right after the first names both; a slide
    /// writes no freedom the ball leaves, and is named with the ball.
    ///
    /// *Breaks if* a point mate's rotation is refused for having no
    /// chart, or the chain is counted by axis rather than by position.
    #[test]
    fn a_point_mates_rotation_is_a_chain() {
        use TargetAxis::{X, Y, Z};
        let ball = (id(1), Equations::Mate(F::Spherical));
        let chain = |axes: &[TargetAxis]| -> Vec<(ConstraintId, Equations)> {
            std::iter::once(ball)
                .chain(
                    (3u32..)
                        .zip(axes)
                        .map(|(k, &a)| unmated(k, Coord::Spin(Some(a)))),
                )
                .collect()
        };
        assert_eq!(
            verdict(&chain(&[Z, X, Y])),
            Ok(true),
            "a chain pins the ball"
        );
        assert_eq!(verdict(&chain(&[Z, X, Z])), Ok(true), "ZXZ is three turns");
        assert_eq!(
            verdict(&chain(&[Z, X, X])),
            Err((vec![id(204), id(205)], 1)),
            "a second spin about x right after the first names both"
        );
        let slid = [chain(&[Z, X, Y]), vec![unmated(9, Coord::Slide(X))]].concat();
        assert_eq!(
            verdict(&slid),
            Err((vec![id(1), id(209)], 0)),
            "a slide beside a ball is fixed by it"
        );
    }

    /// **The symbolic tier reads an angle on its lane's branch**: `y`
    /// spanning zero over a negative `x`, at `Sym<Interval>`, is the
    /// angle about `π` the `Interval` lane reads, an enclosure a few
    /// ulps wide, not the whole circle the principal branch's cut gives;
    /// and an exact zero over a positive `x` is the angle zero on both.
    ///
    /// *Breaks if* `Sym`'s `solve_atan2` reads the plain `atan2`.
    #[test]
    fn the_symbolic_tier_reads_an_angle_on_its_lanes_branch() {
        use crate::mate::SolveScalar;
        use geom_core::{Bounds, Interval, Sym};
        let (y, x) = (
            Interval::from_bounds(-1e-3, 1e-3),
            Interval::from_bounds(-1.0, -0.9),
        );
        let lane = <Interval as SolveScalar>::solve_atan2(y, x);
        let sym = <Sym<Interval> as SolveScalar>::solve_atan2(Sym::opaque(y), Sym::opaque(x));
        assert_eq!(
            (sym.value.lo(), sym.value.hi()),
            (lane.lo(), lane.hi()),
            "the lane's branch"
        );
        assert!(sym.value.hi() - sym.value.lo() < 0.01, "{:?}", sym.value);
        let zero = <Sym<Interval> as SolveScalar>::solve_atan2(
            Sym::opaque(Interval::from_bounds(0.0, 0.0)),
            Sym::opaque(Interval::from_bounds(1.0, 2.0)),
        );
        assert_eq!((zero.value.lo(), zero.value.hi()), (0.0, 0.0));
    }
}
