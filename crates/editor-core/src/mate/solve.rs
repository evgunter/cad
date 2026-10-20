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
use super::{MateFault, MateRef, MateSide, Target};
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
/// [`super::reference_about`]'s two formulas for `1 + c`, and
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

    /// Whether a value's spin sets the map's turn from the two charts'
    /// frames ([`super::value`]) rather than turning the folded map by
    /// the measured difference.
    fn spins_by_frames() -> bool {
        false
    }

    /// **Whether this scalar is exactly zero** (a point enclosure of
    /// zero). It tightens what a reading encloses ([`tightened`]) and
    /// never chooses which predicates are decided, so every lane
    /// decides one list.
    fn exactly_zero(self) -> bool {
        false
    }
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

/// **Which space a value lives in** (D10, Spaces): the world's kind, or
/// the kind of a root other than the world, the one spelling of that
/// fact every reader of it is handed. Decided from the recipe: a
/// construction, which reads no body, is a root of its own; a
/// placement's copy takes its target's space once its bundle pins it;
/// and a copy whose bundle pins less than its body needs is a root of
/// its own.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Space {
    /// The world's kind: the copy is pinned, transitively, against the
    /// world.
    World,
    /// The kind of a root other than the world, which everything pinned
    /// to it shares: a construction, which reads no body, or a loose
    /// copy, whose bundle pins less than its body needs.
    Loose {
        /// The root: the construction, or the loose copy's placement.
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
    /// world's coordinates**, read off its recipe: its bundle is the
    /// identity bundle ([`identity_bundle`]), the one placement the fold
    /// places by no map. A copy that is loose, refused or unplaced does
    /// not stand there, and neither does one whose map comes out as the
    /// identity: that is a value, not the recipe.
    pub fn at_identity(&self, placement: RecipeNodeId) -> bool {
        matches!(self.placed.get(&placement), Some(Ok((Space::World, None))))
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
/// map read at a point keeps its bits. Every lane asks this one door of
/// every map it places, so every lane decides the same margins.
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
fn on_its_axis<T: SolveScalar>(d: geom_core::UnitVec3<T>) -> geom_core::UnitVec3<T> {
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
        PoseValue::Axis { origin, dir } => PoseValue::Axis {
            origin,
            dir: on_its_axis(dir),
        },
        PoseValue::Plane { origin, normal } => PoseValue::Plane {
            origin,
            normal: on_its_axis(normal),
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
        // A world read is inside the mate and reads no variable, so the
        // world is coupled here.
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
        PoseValue::Axis { origin, dir } => aimed(origin, dir),
        PoseValue::Plane { origin, normal } => aimed(origin, normal),
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
/// freedom it sets on the mate it names, or on the target space's own
/// axes where it names none. Read once per bundle, off its
/// definitions ([`equations_of`]), by the edit doors and by the solve
/// alike, so the rule the door asks and the rule the fold asks are one
/// reading.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Equations {
    /// A mate, by its kind's symmetry.
    Mate(SubgroupFamily),
    /// A value: the mate it names and the freedom it sets.
    Value {
        /// The mate, `None` for the target space's own axes.
        mate: Option<crate::node::ConstraintId>,
        /// The freedom.
        coord: super::Coord,
    },
}

impl Equations {
    /// How many equations it is.
    fn codimension(self) -> u8 {
        match self {
            Self::Mate(family) => family.codimension(),
            Self::Value { .. } => 1,
        }
    }
}

/// The pose kind whose symmetry `family` is, for the kinds a mate reads:
/// [`crate::VarKind::symmetry`] read backwards, so the two cannot drift.
fn kind_of(family: SubgroupFamily) -> Option<crate::VarKind> {
    use crate::VarKind as K;
    [K::Frame, K::Plane, K::Axis, K::Direction, K::Point]
        .into_iter()
        .find(|kind| kind.symmetry() == Some(family))
}

/// A bundle's mates, by id and family.
fn mates_of(
    bundle: &[(crate::node::ConstraintId, Equations)],
) -> Vec<(crate::node::ConstraintId, SubgroupFamily)> {
    bundle
        .iter()
        .filter_map(|&(id, e)| match e {
            Equations::Mate(family) => Some((id, family)),
            Equations::Value { .. } => None,
        })
        .collect()
}

/// How many equations a set of mates fixes: their families met in
/// general position ([`SubgroupFamily::meet`]).
fn mates_codimension(mates: &[(crate::node::ConstraintId, SubgroupFamily)]) -> u8 {
    mates
        .iter()
        .fold(SubgroupFamily::Se3, |held, &(_, family)| held.meet(family))
        .codimension()
}

/// **Whether a value's freedom is its own to set beside `mates`**: its
/// mate is in the bundle, the mate's kind leaves the freedom with a
/// chart, and every other mate leaves it too. The solve reads a slide
/// as a translation equation and a spin as a rotation equation, and
/// settles each structurally inside what the mates leave, so a slide
/// is its own only beside mates freeing every translation, a spin only
/// beside mates freeing every rotation. Beside a mate that fixes a
/// translation (a rotation), the value's equation is one that mate
/// already fixes in that reading — even where, through the copy's
/// lever, it would in general position fix a different freedom (a
/// slide beside a ball would fix the ball's turn): the solve does not
/// trade one freedom for another.
fn own_freedom(
    mates: &[(crate::node::ConstraintId, SubgroupFamily)],
    mate: Option<crate::node::ConstraintId>,
    coord: super::Coord,
) -> bool {
    let Some(mate) = mate else {
        return mates.is_empty() && coord.on_own_axes();
    };
    let Some(&(_, own)) = mates.iter().find(|&&(id, _)| id == mate) else {
        return false;
    };
    kind_of(own).is_some_and(|kind| coord.admitted_by(kind).is_ok())
        && mates
            .iter()
            .filter(|&&(id, _)| id != mate)
            .all(|&(_, f)| frees(f, coord))
}

/// Whether a mate of `family` leaves `coord`'s whole class of freedom.
fn frees(family: SubgroupFamily, coord: super::Coord) -> bool {
    match coord {
        super::Coord::Slide(_) => family.frees_every_translation(),
        super::Coord::Spin | super::Coord::SpinAbout(_) => family.frees_every_rotation(),
    }
}

/// **How many equations a bundle fixes**, read off its families alone:
/// its mates met ([`mates_codimension`]), and one more for each
/// freedom a value sets that is its own ([`own_freedom`]). Two values
/// of one freedom on one mate set it once.
fn codimension(bundle: &[(crate::node::ConstraintId, Equations)]) -> u8 {
    let mates = mates_of(bundle);
    if mates.is_empty() {
        return unmated(bundle).0;
    }
    let mut set = Vec::new();
    for &(_, e) in bundle {
        if let Equations::Value { mate, coord } = e
            && own_freedom(&mates, mate, coord)
            && !set.contains(&(mate, coord))
        {
            set.push((mate, coord));
        }
    }
    let set = u8::try_from(set.len()).unwrap_or(u8::MAX);
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
/// - else each value that is not its own freedom ([`own_freedom`]),
///   with the mates beside it that fix its class; each value that sets
///   a freedom another value of its mate sets too; and, past six
///   equations, every value.
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
    if mates.is_empty() {
        return unmated(bundle).1;
    }
    let met = mates_codimension(&mates);
    let named = |id: crate::node::ConstraintId, named: &mut Vec<crate::node::ConstraintId>| {
        if !named.contains(&id) {
            named.push(id);
        }
    };
    let mut out = Vec::new();
    if u16::from(met)
        < mates
            .iter()
            .map(|&(_, f)| u16::from(f.codimension()))
            .sum::<u16>()
    {
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
    let values: Vec<_> = bundle
        .iter()
        .filter_map(|&(id, e)| match e {
            Equations::Value { mate, coord } => Some((id, mate, coord)),
            Equations::Mate(_) => None,
        })
        .collect();
    for &(id, mate, coord) in &values {
        if !own_freedom(&mates, mate, coord) {
            for &(other, family) in &mates {
                if Some(other) != mate && !frees(family, coord) {
                    named(other, &mut out);
                }
            }
            named(id, &mut out);
        } else if values
            .iter()
            .any(|&(o, m, c)| o != id && m == mate && c == coord)
        {
            named(id, &mut out);
        }
    }
    if out.is_empty() {
        for &(id, ..) in &values {
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

/// **A bundle with no mate, read as the freedoms its values write**
/// (D10 `[placement.values]`): a slide along each of the target space's
/// axes, and a chain of spins about them taken in bundle order, of at
/// most three, no two in a row about one axis. Answers how many of the
/// copy's freedoms it writes, and the values that write one twice, in
/// bundle order: two slides along one axis, two spins in a row about
/// one axis, every spin of a chain longer than three, and a value
/// whose freedom is no axis of the target space's (a spin about a
/// mated direction, or one naming a mate the bundle does not hold).
fn unmated(
    bundle: &[(crate::node::ConstraintId, Equations)],
) -> (u8, Vec<crate::node::ConstraintId>) {
    use super::Coord;
    let mut named: Vec<crate::node::ConstraintId> = Vec::new();
    let mut name = |id| {
        if !named.contains(&id) {
            named.push(id);
        }
    };
    let mut slides: Vec<(super::TargetAxis, crate::node::ConstraintId)> = Vec::new();
    let mut spins: Vec<(super::TargetAxis, crate::node::ConstraintId)> = Vec::new();
    for &(id, e) in bundle {
        match e {
            Equations::Value {
                mate: None,
                coord: Coord::Slide(axis),
            } => {
                if let Some(&(_, first)) = slides.iter().find(|&&(a, _)| a == axis) {
                    name(first);
                    name(id);
                } else {
                    slides.push((axis, id));
                }
            }
            Equations::Value {
                mate: None,
                coord: Coord::SpinAbout(axis),
            } => {
                if let Some(&(last, before)) = spins.last()
                    && last == axis
                {
                    name(before);
                    name(id);
                }
                spins.push((axis, id));
            }
            Equations::Value { .. } | Equations::Mate(_) => name(id),
        }
    }
    if spins.len() > 3 {
        for &(_, id) in &spins {
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
    let written = u8::try_from(slides.len() + turns).unwrap_or(u8::MAX).min(6);
    let order: Vec<_> = bundle
        .iter()
        .map(|&(id, _)| id)
        .filter(|id| named.contains(id))
        .collect();
    (written, order)
}

/// **A slide the mates leave no translation to set**: where the mates
/// repeat no equation, the first slide on a mate of its kind whose
/// fellow mates fix a translation — a ball beside a direction, the one
/// pair that meets transversally. In general position it would fix the
/// ball's turn, so it is no repeat, and it refuses
/// [`super::NoChartWhy::SlideTurnsABall`], not as an overconstraint.
fn slide_beside_a_ball(
    bundle: &[(crate::node::ConstraintId, Equations)],
) -> Option<crate::node::ConstraintId> {
    let mates = mates_of(bundle);
    let independent = u16::from(mates_codimension(&mates))
        == mates
            .iter()
            .map(|&(_, f)| u16::from(f.codimension()))
            .sum::<u16>();
    if !independent {
        return None;
    }
    bundle.iter().find_map(|&(id, e)| match e {
        Equations::Value {
            mate: Some(mate),
            coord: coord @ super::Coord::Slide(_),
        } => {
            let own = mates.iter().find(|&&(m, _)| m == mate)?.1;
            let charted = kind_of(own).is_some_and(|kind| coord.admitted_by(kind).is_ok());
            let held = mates
                .iter()
                .any(|&(m, f)| m != mate && !f.frees_every_translation());
            (charted && held).then_some(id)
        }
        _ => None,
    })
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

/// **A bundle with no mate writes every freedom** (D10
/// `[placement.values]`): with no mate its values state the copy's
/// whole motion, so the door refuses one that leaves a freedom
/// unwritten. An empty bundle is a loose copy, not this rule's.
///
/// # Errors
///
/// [`MateFault::Unpinned`], with the freedoms the bundle leaves.
pub(crate) fn unpinned(
    placement: RecipeNodeId,
    bundle: &[(crate::node::ConstraintId, Equations)],
) -> Result<(), Box<MateFault>> {
    if bundle.is_empty() || !mates_of(bundle).is_empty() || pins(bundle) {
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
    !bundle.is_empty()
        && bundle.iter().all(|c| {
            c.value().is_some_and(|v| {
                v.mate.is_none()
                    && matches!(
                        doc.var(v.by).map(crate::Var::def),
                        Some(crate::var::VarDef::Defined(expr)) if expr.is_zero()
                    )
            })
        })
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
                super::ConstraintKind::Value(value) => Equations::Value {
                    mate: value.mate,
                    coord: value.coord,
                },
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

/// What one bundle's fold placed its copy at.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Folded<T: Real> {
    /// Nothing pins it: a loose copy, the root of its own space.
    Loose,
    /// Pinned in its target space, at this map from its own
    /// coordinates, and the map as the rigid motion its composition
    /// made, where the lane composes one.
    Pinned(Affine3<T>, Option<super::value::Composed<T>>),
    /// A bundle with no mate, its values writing every freedom: placed
    /// in the world by the motion its values state
    /// ([`super::value::unmated_motion`]).
    Unmated,
}

/// **One placement's bundle, solved.** `equations` are the bundle's,
/// read once off its definitions ([`equations_of`]) — the reading the
/// edit doors ask the rule of, so the door and the solve decide over
/// one value. Each value is first asked what its mate's kind decides (a
/// freedom the kind leaves, with a chart), then the bundle rule
/// ([`overconstrained`]) refuses a constraint any of whose equations
/// the rest already fix. The mates fold, and the values place the copy
/// inside what they leave ([`super::value::settle`]). A bundle with
/// mates that pins less than its copy needs ([`pins`]) makes a loose
/// copy, never a refusal; one with no mate writes every freedom
/// ([`unpinned`]) and is answered [`Folded::Unmated`].
///
/// # Errors
///
/// [`MateFault::ValueNamesNoMate`], [`MateFault::NotAFreedom`] or
/// [`MateFault::NoChart`] for a value, [`MateFault::Overconstrained`],
/// [`MateFault::Unpinned`], the fold's own ([`fold_bundle`]), or a
/// chart degenerate at its values ([`super::value::settle`]).
pub(crate) fn solve_bundle<T: SolveScalar>(
    placement: RecipeNodeId,
    equations: &[(crate::node::ConstraintId, Equations)],
    mates: &[ReadMate<T>],
    values: &[super::value::ReadValue<T>],
    charts: &BTreeMap<crate::node::ConstraintId, Option<super::value::Chart<T>>>,
) -> Result<Folded<T>, crate::NodeErrorKind> {
    let fault = |f: MateFault| crate::NodeErrorKind::Mate(Box::new(f));
    for v in values {
        let Some(mate) = v.mate else {
            unmated_admitted(v.value, v.coord, !mates.is_empty()).map_err(|f| fault(*f))?;
            continue;
        };
        let Some(&(_, Equations::Mate(family))) = equations.iter().find(|&&(id, _)| id == mate)
        else {
            return Err(fault(MateFault::ValueNamesNoMate {
                value: v.value,
                mate,
            }));
        };
        if let Some(kind) = kind_of(family) {
            v.coord
                .admitted_by(kind)
                .map_err(|refusal| fault(refused_freedom(v.value, v.coord, kind, refusal)))?;
        }
    }
    overconstrained(placement, equations).map_err(|f| fault(*f))?;
    unpinned(placement, equations).map_err(|f| fault(*f))?;
    if mates.is_empty() {
        return Ok(if values.is_empty() {
            Folded::Loose
        } else {
            Folded::Unmated
        });
    }
    let coset = fold_bundle(placement, mates).map_err(|f| fault(*f))?;
    let (at, rigid) = super::value::settle(coset, values, charts)?;
    Ok(if pins(equations) {
        Folded::Pinned(at, rigid)
    } else {
        Folded::Loose
    })
}

/// A value's freedom its mate's kind does not leave, as the fault
/// names it.
fn refused_freedom(
    value: MateRef,
    coord: super::Coord,
    kind: crate::VarKind,
    refusal: super::FreedomRefusal,
) -> MateFault {
    match refusal {
        super::FreedomRefusal::NotAFreedom => MateFault::NotAFreedom { value, coord, kind },
        super::FreedomRefusal::NoChart => MateFault::NoChart {
            value,
            why: super::NoChartWhy::PointRotation,
        },
    }
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
/// ([`MateFault::KindsDiffer`]), and the class is admitted. Whether it
/// fixes an equation the rest of its bundle already fixes is the bundle
/// rule's ([`overconstrained`]).
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

/// **A value naming no mate, admitted on its own**: its freedom is an
/// axis of the target space's own ([`super::Coord::on_own_axes`]), and
/// its bundle holds no mate (`beside`), since the target space's axes
/// chart a bundle with no mate.
///
/// # Errors
///
/// [`MateFault::NoChart`]: [`super::NoChartWhy::BesideAMate`] or
/// [`super::NoChartWhy::NoMatedDirection`].
fn unmated_admitted(
    value: MateRef,
    coord: super::Coord,
    beside: bool,
) -> Result<(), Box<MateFault>> {
    let why = if beside {
        super::NoChartWhy::BesideAMate
    } else if coord.on_own_axes() {
        return Ok(());
    } else {
        super::NoChartWhy::NoMatedDirection
    };
    Err(Box::new(MateFault::NoChart { value, why }))
}

/// **One value's own admission** (ruling C-2): it names a mate of
/// `bundle` ([`MateFault::ValueNamesNoMate`]) or none
/// ([`unmated_admitted`]), the mate's kind leaves
/// its freedom ([`MateFault::NotAFreedom`]) with a chart
/// ([`MateFault::NoChart`]), and the mate's target reads one body. Its
/// number's dimension is the slot's, which the lowering checks. A
/// freedom the bundle's other constraints already fix is the bundle
/// rule's ([`overconstrained`]).
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
    let Some(named) = data.mate else {
        let beside = bundle.iter().any(|c| c.mate().is_some());
        return unmated_admitted(value, data.coord, beside);
    };
    let Some(mate) = bundle
        .iter()
        .find(|c| c.id == named)
        .and_then(super::Constraint::mate)
    else {
        return Err(Box::new(MateFault::ValueNamesNoMate {
            value,
            mate: named,
        }));
    };
    if let Some(kind) = doc.var(mate.on).map(crate::Var::kind) {
        data.coord
            .admitted_by(kind)
            .map_err(|refusal| Box::new(refused_freedom(value, data.coord, kind, refusal)))?;
    }
    if let Target::Pose(to) = &mate.to {
        chart_body(doc, *to).map_err(|why| Box::new(MateFault::NoChart { value, why }))?;
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
            id(100 + mate * 10 + u32::from(matches!(coord, Coord::Spin))),
            Equations::Value {
                mate: Some(id(mate)),
                coord,
            },
        )
    }

    /// The `k`th value of a bundle with no mate.
    fn unmated(k: u32, coord: Coord) -> (ConstraintId, Equations) {
        (id(200 + k), Equations::Value { mate: None, coord })
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
        // A lone Plane with Slide(X): admitted, loose.
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
                value(1, Coord::Spin),
            ]),
            Ok(true)
        );
        // Point + Direction with a spin on the direction: admitted, pinned.
        let ball_and_direction = [
            (id(1), Equations::Mate(F::Spherical)),
            (id(2), Equations::Mate(F::Parallel)),
        ];
        assert_eq!(verdict(&ball_and_direction), Ok(false));
        let spun = [ball_and_direction.to_vec(), vec![value(2, Coord::Spin)]].concat();
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

    /// **Two values of one freedom on one mate name each other**, and
    /// nothing else: the mate fixes nothing they repeat.
    #[test]
    fn a_freedom_set_twice_names_its_two_values() {
        let x = Coord::Slide(TargetAxis::X);
        let bundle = [
            (id(1), Equations::Mate(F::Planar)),
            (
                id(7),
                Equations::Value {
                    mate: Some(id(1)),
                    coord: x,
                },
            ),
            (
                id(8),
                Equations::Value {
                    mate: Some(id(1)),
                    coord: x,
                },
            ),
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
                bundle.push(unmated(k, Coord::SpinAbout(axis)));
            }
            bundle
        };
        let unwritten = |bundle: &[(ConstraintId, Equations)]| {
            match unpinned(RecipeNodeId::WORLD, bundle).map_err(|f| *f) {
                Ok(()) => None,
                Err(MateFault::Unpinned { free, .. }) => Some(free),
                Err(other) => panic!("unpinned refuses only Unpinned, got {other:?}"),
            }
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
        assert_eq!(unwritten(&[]), None, "an empty bundle is a loose copy");
    }
}
