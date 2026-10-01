//! **In-op exhaustiveness** — the never-silence obligation, mechanized
//! (M5 PR 7 spec §4, C3).
//!
//! Marching finds what it is seeded onto. The banked principle names
//! the classic silent disaster — a small loop nobody seeded and nobody
//! missed *loudly* — so found-ness here is never allowed to depend on
//! luck. The bounded domain is subdivided and **every** cell must
//! finish in one of three states:
//!
//! 1. **excluded** — a certified enclosure of `f₁` (or `f₂`, or the
//!    chart form `φ`) over the cell does not contain zero, so no
//!    solution can be in it;
//! 2. **accounted** — the cell lies inside a found branch's uniqueness
//!    tube, where limb 3 already proved there is exactly one arc;
//! 3. **refine** — split and recurse.
//!
//! At the named floor [`SSI_FLOOR`]·ε a cell that is still in state 3
//! ends the operation with the typed
//! `SsiExhaustivenessInconclusive` refusal. "Every branch found" is
//! then a theorem about enclosures, or it is a typed failure. It is
//! never silence.
//!
//! The floor itself is minted once over the domain it bisects
//! ([`SweepFloor`]), and that door refuses a floor no bisection of the
//! domain can reach: one that is not a positive finite width, or one
//! narrower than the finest cell at the domain's largest coordinate. It
//! refuses by name ([`SsiError::FloorUnresolvable`]) with the rate that
//! crossed it, so the cell budget never answers for a floor no cell can
//! reach. An attainable floor can still spend the budget by cell count.
//!
//! Every length on a receipt or on that refusal is in the units its
//! own lane measures cells in, and the receipt says which lane that
//! was: [`ExhaustLane`] carries the chart lane's certified
//! [`SupSpeed`] beside the tag, so metres come from
//! [`Exhaustiveness::floor_meters`] rather than from a caller holding
//! the rate by hand. Which bound direction a crossing needs is the
//! rate pair's own rule, stated once in `geom_core::predicate`'s
//! module doc.
//!
//! # The subdivision is also the seed generator — on the caller's word
//!
//! The same recursion, run at the coarser [`SSI_SEED_FLOOR`]·extent
//! floor, returns the cells that survived exclusion. Their centers are
//! the marcher's seeds. A branch that touches no domain boundary — the
//! interior small loop that boundary seeding provably cannot reach — is
//! found here or the operation refuses; there is no third outcome.
//!
//! One structure, two duties — and **which duty is a parameter the
//! caller states**, never a condition the recursion reads off its own
//! input. [`seed_r3`] and [`seed_chart_plane`] generate seeds: they take
//! no tube set and cannot refuse at the floor. [`account_r3`] and
//! [`account_chart_plane`] discharge the obligation: they return the
//! receipt and nothing else, and they refuse at the floor unconditionally
//! — including when their tube set is empty, which is precisely the run
//! that has proved nothing: every seed failed to refine, or every
//! certified branch yielded no window worth banking. Nothing in either
//! signature can express the other duty's answer.
//!
//! # Brute force, deliberately, for now
//!
//! Cells are enumerated by recursive bisection on the widest axis with
//! a fixed tie-break (D9). PR 8's BVH swaps in under its already-merged
//! differential suite when profiling asks for it; the plan explicitly
//! permits brute force here and nothing in the contract changes when
//! the pruning does.

use geom::{NurbsSurface, Surface};
use geom_core::Bounds;
use geom_core::interval::certification::Certification;
use geom_core::interval::max_bound;
use geom_core::{Interval, Point3, SizedPass, SupSpeed, Vec3};

use super::SsiError;
use super::enclose::{Box3, NurbsBoxes, implicit_enclosure};
use crate::recourse::{Reading, RefusedArm, SizedDecision, StoredDefinite, defect_ending};

/// The refinement floor, as a multiple of ε: a cell narrower than this
/// is not split again. Fixed and named (C3: "a named constant tied to
/// ε").
pub const SSI_FLOOR: f64 = 1.0;

/// The seed-generation floor, as a fraction of the caller's named
/// **extent** (not of ε — see `SsiDomain::seed_floor`): a seed only has
/// to land in Newton's basin, whose size is a property of the geometry.
pub const SSI_SEED_FLOOR: f64 = 1.0 / 64.0;

/// The cell budget. Exceeding it is a typed refusal, never a silent
/// truncation of the search.
pub const SSI_MAX_CELLS: usize = 200_000;

/// **Which lane a receipt came from**, and the rate that lane's
/// lengths were stated in.
///
/// The subdivision decides `cell.width() <= floor` in whatever units
/// its own cells are measured in — metres for the ℝ³ lane's boxes,
/// chart parameter units for the chart lane's rectangles — so a length
/// on a receipt means nothing without the lane that produced it. The
/// chart arm carries the certified chart speed that crossed the
/// caller's metre floor into those units, which is what
/// [`Exhaustiveness::floor_meters`] crosses back, and which is the
/// number a caller reading a refusal needs in order to tell a fine
/// floor from a slow chart.
///
/// The rate is a [`SupSpeed`] by signature. An
/// [`InfSpeed`](geom_core::InfSpeed) cannot reach this arm:
///
/// ```compile_fail,E0308
/// use geom_brep::ExhaustLane;
/// use geom_core::InfSpeed;
/// let _ = ExhaustLane::Chart { speed: InfSpeed::new(2.0_f64) };
/// ```
///
/// Its twin differs in one respect — the tag matches the arm — and
/// compiles:
///
/// ```
/// use geom_brep::ExhaustLane;
/// use geom_core::SupSpeed;
/// let _ = ExhaustLane::Chart { speed: SupSpeed::new(2.0_f64) };
/// ```
///
/// Stable rustdoc checks only that a `compile_fail` block fails to
/// build and not which error it is, which is what the twin is for: a
/// typo shared by both would redden it. The code was read off `rustc`
/// at the pinned toolchain (1.97.0).
///
/// No `PartialEq`: [`SupSpeed`] has none, by the `Real` surface's rule
/// that a tagged rate is never compared without `get()`.
///
/// **Why a tag rather than public rate fields.** The in-tree receipt
/// with the same duty, `offset_meters::PatchRegularity`, carries its
/// rates as public `SupSpeed` fields, which it can because every
/// patch it describes has them. A receipt here comes from one of two
/// subdivisions and only one of them has a rate at all, so a public
/// field would have to be an `Option` — or a fabricated number — on
/// the ℝ³ lane, and the unit a length is stated in would still be
/// read off something other than the rate. The tag answers both at
/// once: which lane, and the rate if that lane has one.
#[derive(Clone, Copy, Debug)]
pub enum ExhaustLane {
    /// Cells are boxes in ℝ³ and every length on the receipt is
    /// already metres. No rate exists on this lane: nothing on the
    /// path divides or multiplies by one.
    R3,
    /// Cells are rectangles in a surface's parameter domain, so every
    /// length on the receipt is in chart units.
    Chart {
        /// Metres per chart parameter unit — the certified bound the
        /// caller's metre floor was divided by, and the one the
        /// receipt's lengths are multiplied back through.
        speed: SupSpeed<f64>,
    },
}

impl ExhaustLane {
    /// **The one crossing**: a length this lane stated, read in metres.
    ///
    /// Every metre reading either type offers goes through here, so
    /// which lane needs a multiply is decided once. On ℝ³ the value
    /// already is metres; on the chart lane [`SupSpeed::to_meters`] is
    /// one operation, so the reading is the bare product's bits.
    #[must_use]
    pub fn meters(self, x: f64) -> f64 {
        match self {
            Self::R3 => x,
            Self::Chart { speed } => speed.to_meters(x),
        }
    }

    /// The certified rate this lane's lengths are stated in, or `None`
    /// on the lane that has no rate. The `Option` is the ℝ³ lane's
    /// answer, not an unknown: nothing on that path divides or
    /// multiplies by a speed.
    #[must_use]
    pub fn speed(self) -> Option<SupSpeed<f64>> {
        match self {
            Self::R3 => None,
            Self::Chart { speed } => Some(speed),
        }
    }
}

/// How a chart-lane text names the rate it crossed with.
#[derive(Clone, Copy)]
pub(super) enum RateClause {
    /// Name the certified speed inside the metre reading's own
    /// parenthesis. Each text does this exactly once.
    Name,
    /// State the metres and stop — the text names the rate elsewhere.
    Omit,
    /// Start from the metres the length was divided from, and name the
    /// rate that divided them. A floor the chart cannot carry is read
    /// this way, because crossing it back through the rate would print
    /// a different number from the one the caller named.
    From {
        /// The length in metres the chart length was crossed from.
        meters: f64,
    },
}

/// **One spelling of a chart-lane length**, for every chart-lane
/// `Display`.
///
/// A chart-unit number is unreadable without the metres it stands for
/// and the rate that crossed it, and the receipt and the refusals make
/// the same claim about the same pair, so they write it with the same
/// words.
pub(super) fn write_chart_length(
    f: &mut core::fmt::Formatter<'_>,
    x: f64,
    speed: SupSpeed<f64>,
    rate: RateClause,
) -> core::fmt::Result {
    let m = speed.to_meters(x);
    match rate {
        RateClause::Name => write!(
            f,
            "{x:e} chart units ({m:e} m at a certified chart speed of {:e} m per \
             chart unit)",
            speed.get()
        ),
        RateClause::Omit => write!(f, "{x:e} chart units ({m:e} m)"),
        RateClause::From { meters } => write!(
            f,
            "{meters:e} m is {x:e} chart units at a certified chart speed of {:e} m per \
             chart unit",
            speed.get()
        ),
    }
}

/// What the subdivision proved about the domain.
#[derive(Clone, Copy, Debug)]
pub struct Exhaustiveness {
    /// Which lane proved it, and — on the chart lane — the rate this
    /// receipt's lengths are stated in.
    pub lane: ExhaustLane,
    /// Cells examined in total.
    pub examined: u32,
    /// Cells proved solution-free by enclosure.
    pub excluded: u32,
    /// Cells proved to lie inside a found branch's uniqueness tube.
    pub accounted: u32,
    /// Cells that were neither, and were split — the interior nodes of
    /// the subdivision tree. Reported so the receipt adds up:
    /// `examined == excluded + accounted + refined`, with every LEAF in
    /// one of the first two. That identity is the theorem.
    ///
    /// It holds for every receipt that escapes this module, because only
    /// the accounting duty returns one: there, a leaf is excluded,
    /// accounted, or the typed refusal, so no leaf can land outside a
    /// bucket. The seeding duty's surviving leaves are the seeds, and
    /// [`seed_r3`] / [`seed_chart_plane`] hand back no receipt at all.
    pub refined: u32,
    /// The deepest recursion reached.
    pub max_depth: u32,
    /// The floor used, in [`lane`](Self::lane)'s own units: the units
    /// `cell.width() <= floor` was decided in. [`Self::floor_meters`]
    /// reads it in metres.
    pub floor: f64,
}

impl Exhaustiveness {
    /// The refinement floor in metres, whichever lane this is.
    ///
    /// On the chart lane this is the caller's own metre floor come
    /// back: it was divided into chart units by this rate and is
    /// multiplied back through it, so what a reader sees is that round
    /// trip's two roundings, not a new measurement.
    #[must_use]
    pub fn floor_meters(&self) -> f64 {
        self.lane.meters(self.floor)
    }
}

impl core::fmt::Display for Exhaustiveness {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "ssi exhaustiveness: {} cells to depth {} — {} excluded, {} accounted, \
             {} refined — ",
            self.examined, self.max_depth, self.excluded, self.accounted, self.refined
        )?;
        match self.lane {
            ExhaustLane::R3 => write!(
                f,
                "on the ℝ³ lane, at the refinement floor {:e} m",
                self.floor
            ),
            ExhaustLane::Chart { speed } => {
                write!(f, "on the chart lane, at the refinement floor ")?;
                write_chart_length(f, self.floor, speed, RateClause::Name)
            }
        }
    }
}

/// **What the subdivision could not prove** — the payload of
/// [`SsiError::ExhaustivenessInconclusive`].
///
/// A named payload rather than four fields on the variant, mirroring
/// [`Exhaustiveness`]: the two metre readings belong to this refusal
/// and to no other refusal in the taxonomy, so hung on the error they
/// had to answer for twenty variants and hand back an `Option` that
/// every caller who had already matched the variant knew was `Some`.
#[derive(Clone, Copy, Debug)]
pub struct ExhaustivenessRefusal {
    /// Which lane's subdivision refused, and — on the chart lane — the
    /// rate this refusal's two lengths are stated in.
    pub lane: ExhaustLane,
    /// The offending cell's width, in [`lane`](Self::lane)'s own units.
    /// [`Self::cell_width_meters`] reads it in metres.
    pub cell_width: f64,
    /// The floor it hit, in the same units. [`Self::floor_meters`]
    /// reads it in metres.
    pub floor: f64,
    /// Cells examined before the refusal.
    pub examined: u32,
}

impl ExhaustivenessRefusal {
    /// The offending cell's width in metres.
    ///
    /// On the chart lane this is the cell's widest side times the
    /// larger axis's certified chart speed: a bound on how far the
    /// surface moves along ONE parameter direction across the cell. It
    /// is not a bound on the unproved region's size. The two directions
    /// add, so the cell's image in metres can be up to twice this
    /// across, and the speed is an upper bound, so it can also be much
    /// less.
    #[must_use]
    pub fn cell_width_meters(&self) -> f64 {
        self.lane.meters(self.cell_width)
    }

    /// The refinement floor in metres.
    ///
    /// On the chart lane this is the caller's own metre floor come
    /// back: it was divided into chart units by this rate and is
    /// multiplied back through it, so what a reader sees is that round
    /// trip's two roundings, not a second measurement.
    #[must_use]
    pub fn floor_meters(&self) -> f64 {
        self.lane.meters(self.floor)
    }
}

/// Which of the subdivision's two floors a [`FloorRefusal`] is about.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FloorKind {
    /// The seeding floor, a fraction of the caller's extent
    /// ([`SSI_SEED_FLOOR`]).
    Seeding,
    /// The accounting floor, a multiple of ε ([`SSI_FLOOR`]): the proof
    /// obligation's.
    Accounting,
}

impl FloorKind {
    fn name(self) -> &'static str {
        match self {
            Self::Seeding => "seeding floor",
            Self::Accounting => "accounting floor",
        }
    }
}

/// What is wrong with a floor ([`FloorRefusal::fault`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FloorFault {
    /// The floor in metres is not a positive finite length. The caller's
    /// domain knobs (`extent`, `floor_scale`) are what it is made of.
    NotALength,
    /// The floor in metres is a length, but divided by the chart speed
    /// it is not finite: the surface moves so slowly across its
    /// parameters that the floor spans no finite parameter width.
    Overflows,
    /// The domain's largest coordinate is not a finite number, so it
    /// has no resolution to compare the floor with. Every SSI door
    /// checks the caller's domain first, and the chart lane's root is a
    /// validated knot domain, so nothing a caller holds reaches it.
    DomainUnreadable,
    /// The floor is narrower than the finest cell bisection can cut
    /// where the domain reaches furthest from zero, so the subdivision
    /// can never refine to it there.
    BelowResolution {
        /// The finest cell's width there, in the lane's units: the
        /// width the lane's own cell measure gives two adjacent floats
        /// just below `reach`.
        resolution: f64,
        /// The domain's largest coordinate magnitude, in the lane's
        /// units.
        reach: f64,
    },
}

/// **A floor the subdivision's domain cannot resolve** — the payload of
/// [`SsiError::FloorUnresolvable`].
///
/// The subdivision stops refining a cell once its width is at most the
/// floor. A floor no cell can reach would leave the cell budget to
/// answer in its place. That is the wrong diagnosis: the floor was
/// never usable, and the refusal says why by the rate that crossed it.
#[derive(Clone, Copy, Debug)]
pub struct FloorRefusal {
    /// Which lane's domain, and on the chart lane the rate the metres
    /// were divided by.
    pub lane: ExhaustLane,
    /// Which floor.
    pub floor: FloorKind,
    /// The floor in metres, as the caller's domain named it.
    pub meters: f64,
    /// The floor in [`lane`](Self::lane)'s own units: `meters` itself on
    /// ℝ³, `meters` divided by the chart speed on the chart lane.
    pub width: f64,
    /// What is wrong with it.
    pub fault: FloorFault,
}

impl FloorRefusal {
    /// The ending at `reading` (D4 ¶1). A floor too fine for its domain
    /// is the geometry's scale, decided exactly (no band, so no
    /// tolerance is named); a floor that is not a length is the
    /// caller's knobs; an unreadable root is no input's.
    #[must_use]
    pub fn ending(&self, reading: Reading) -> String {
        let decision = match (self.fault, self.lane) {
            (FloorFault::NotALength, _) => {
                return "Recourse: give the domain a feature extent and floor scale whose \
                        floors are positive finite lengths"
                    .to_owned();
            }
            (FloorFault::DomainUnreadable, _) => return defect_ending(reading).to_owned(),
            (FloorFault::Overflows, _) => CHART_TOO_SLOW,
            (FloorFault::BelowResolution { .. }, ExhaustLane::R3) => R3_SCALE,
            (FloorFault::BelowResolution { .. }, ExhaustLane::Chart { .. }) => CHART_SCALE,
        };
        decision.recourse(RefusedArm::SignCertain, reading)
    }
}

/// The ℝ³ lane's floor is finer than the slab's coordinates resolve:
/// the geometry sits too far from the origin for ε.
pub(super) const R3_SCALE: SizedDecision = SizedDecision {
    lever: "move the geometry nearer the origin, within the model's size range, where its \
            coordinates resolve the tolerance",
    size: "scale",
    passes: SizedPass::Positive,
    stored: StoredDefinite::Lever,
    at_zero: None,
};

/// The chart lane's floor is finer than the wall's parameters resolve:
/// the wall moves more than the floor across the smallest step of its
/// parameters.
pub(super) const CHART_SCALE: SizedDecision = SizedDecision {
    lever: "bring the spline face within the model's size range, or move its parameter domain \
            nearer zero, so the face moves less than the tolerance across the finest step of \
            its parameters",
    size: "scale",
    passes: SizedPass::Positive,
    stored: StoredDefinite::Lever,
    at_zero: None,
};

/// The chart lane's floor spans no finite parameter width: the wall is
/// too small for the floor to be stated in its parameters.
const CHART_TOO_SLOW: SizedDecision = SizedDecision {
    lever: "bring the spline face within the model's size range; it is too small for a length \
            in metres to be stated in its parameters",
    size: "scale",
    passes: SizedPass::Positive,
    stored: StoredDefinite::Lever,
    at_zero: None,
};

impl core::fmt::Display for FloorRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let (meters, width) = (self.meters, self.width);
        write!(f, "the {} ", self.floor.name())?;
        match self.lane {
            ExhaustLane::R3 => write!(f, "{meters:e} m")?,
            ExhaustLane::Chart { speed } => {
                write_chart_length(f, width, speed, RateClause::From { meters })?;
            }
        }
        let unit = match self.lane {
            ExhaustLane::R3 => "m",
            ExhaustLane::Chart { .. } => "chart units",
        };
        match self.fault {
            FloorFault::NotALength => write!(
                f,
                ", made of the domain's extent and floor scale, is not a positive finite \
                 length, so the subdivision has no width to refine to"
            ),
            FloorFault::Overflows => write!(
                f,
                ", which is not a finite width: the surface moves too slowly across its \
                 parameters for the floor to be stated in them"
            ),
            FloorFault::DomainUnreadable => write!(
                f,
                ", over a domain whose largest coordinate is not a finite number, which has \
                 no resolution to compare it with"
            ),
            FloorFault::BelowResolution { resolution, reach } => write!(
                f,
                ", which the domain cannot resolve: where it reaches {reach:e} {unit} from \
                 zero the finest cell bisection can cut is {resolution:e} {unit} wide, so the \
                 subdivision could never refine to the floor"
            ),
        }
    }
}

/// **The subdivision's floor, minted once over the domain it bisects.**
///
/// Holding one is the proof that the floor is a positive finite width
/// in the lane's units and that bisection of `root` can reach it: no
/// cell of the domain is too wide to split yet too narrow for the split
/// to land strictly inside it. Every sweep takes its root from here, so
/// a floor checked against one domain cannot be used against another.
#[derive(Clone, Copy, Debug)]
pub(crate) struct SweepFloor<C> {
    root: C,
    lane: ExhaustLane,
    width: f64,
}

impl SweepFloor<Box3> {
    /// The ℝ³ lane's floor over `root`: a length in metres, which is
    /// already the lane's unit.
    ///
    /// # Errors
    ///
    /// [`SsiError::FloorUnresolvable`] as [`SweepFloor::mint`].
    pub(crate) fn r3(root: Box3, meters: f64, floor: FloorKind) -> Result<Self, SsiError> {
        Self::mint(root, ExhaustLane::R3, floor, meters, meters)
    }
}

impl SweepFloor<UvRect> {
    /// The chart lane's floor over `root`: `meters` divided into chart
    /// units by `speed`, the certified chart speed the lane's receipts
    /// cross back through.
    ///
    /// # Errors
    ///
    /// [`SsiError::FloorUnresolvable`] as [`SweepFloor::mint`].
    pub(crate) fn chart(
        root: UvRect,
        meters: f64,
        speed: SupSpeed<f64>,
        floor: FloorKind,
    ) -> Result<Self, SsiError> {
        let lane = ExhaustLane::Chart { speed };
        Self::mint(root, lane, floor, meters, speed.to_param(meters))
    }
}

impl<C: SweepCell> SweepFloor<C> {
    /// The one door.
    ///
    /// # Errors
    ///
    /// [`SsiError::FloorUnresolvable`] when `meters` is not a positive
    /// finite length, when `width` is not finite, or when `width` is
    /// below the width of the finest cell bisection can cut at the
    /// root's largest coordinate — the exact boundary, below which the
    /// split of that cell returns the cell itself.
    fn mint(
        root: C,
        lane: ExhaustLane,
        floor: FloorKind,
        meters: f64,
        width: f64,
    ) -> Result<Self, SsiError> {
        let refuse = |fault| {
            Err(SsiError::FloorUnresolvable(FloorRefusal {
                lane,
                floor,
                meters,
                width,
                fault,
            }))
        };
        if !(meters.is_finite() && meters > 0.0) {
            return refuse(FloorFault::NotALength);
        }
        if !width.is_finite() {
            return refuse(FloorFault::Overflows);
        }
        let reach = root.reach();
        if !reach.is_finite() {
            return refuse(FloorFault::DomainUnreadable);
        }
        let resolution = C::finest_at(reach).width();
        if width < resolution {
            return refuse(FloorFault::BelowResolution { resolution, reach });
        }
        Ok(Self { root, lane, width })
    }

    /// The floor, in the lane's units.
    fn width(self) -> f64 {
        self.width
    }
}

/// The spacing of adjacent floats where `root` reaches furthest from
/// zero, in the lane's units, with that reach: the width of the finest
/// cell bisection can cut there, which the floors are held to. The gap
/// is `NaN` where the reach is not finite.
pub(super) fn coordinate_gap<C: SweepCell>(root: C) -> (f64, f64) {
    let reach = root.reach();
    if !reach.is_finite() {
        return (reach, f64::NAN);
    }
    (reach, C::finest_at(reach).width())
}

/// **Which of the subdivision's two duties the caller is asking for.**
///
/// The recursion is one structure with two jobs, and the job is named
/// by the caller. It is deliberately not inferable from the data: an
/// empty tube set is a legitimate accounting input — the run where
/// nothing was found, and therefore nothing is proved — not a request
/// to generate seeds.
///
/// Private to this module, and every constructor of it is inside one of
/// the four entry points below, so no caller can name the wrong duty.
#[derive(Clone, Copy)]
enum SweepDuty<'a, C> {
    /// Return the centers of the cells that survived exclusion.
    Seed,
    /// Prove every leaf is excluded or lies inside one of these
    /// uniqueness tubes; refuse at the floor otherwise.
    Account {
        /// The uniqueness tubes limb 3 banked, in the floor's lane's
        /// units.
        tubes: &'a [C],
    },
}

impl<C: SweepCell> SweepDuty<'_, C> {
    /// Whether limb 3 has already proved what is in this cell. Seeding
    /// proves nothing about any cell, so the answer is `false` because
    /// of the duty — not because a tube set happens to be empty.
    fn accounts(self, cell: C) -> bool {
        match self {
            Self::Seed => false,
            Self::Account { tubes } => tubes.iter().any(|t| cell.contained_in(*t)),
        }
    }
}

/// A cell of the subdivision. The two lanes differ in their shape and
/// in what a survivor hands the marcher; everything the recursion does
/// with a cell is here, so the recursion itself is written once.
pub(crate) trait SweepCell: Copy {
    /// What a surviving cell gives the marcher to start from.
    type Seed;

    /// The widest side, in the lane's own units — what the floor is
    /// compared against.
    fn width(self) -> f64;
    /// The seed a survivor contributes: the cell's center.
    fn seed(self) -> Self::Seed;
    /// Bisect the widest axis with a fixed tie-break (D9).
    fn split(self) -> (Self, Self);
    /// Containment, for the accounting test against a tube.
    fn contained_in(self, other: Self) -> bool;
    /// The largest coordinate magnitude over the cell's sides (`NaN`
    /// when a side has none).
    fn reach(self) -> f64;
    /// The finest cell bisection can cut at coordinate magnitude
    /// `reach`: two adjacent floats just below it on every side, which
    /// no split separates. The door reads its width through
    /// [`SweepCell::width`], so the floor and the sweep measure it alike.
    fn finest_at(reach: f64) -> Self;
}

impl SweepCell for Box3 {
    type Seed = Point3<f64>;

    fn width(self) -> f64 {
        Box3::width(self)
    }
    fn seed(self) -> Point3<f64> {
        self.center()
    }
    fn split(self) -> (Self, Self) {
        Box3::split(self)
    }
    fn contained_in(self, other: Self) -> bool {
        Box3::contained_in(self, other)
    }
    fn reach(self) -> f64 {
        [self.y.mag(), self.z.mag()]
            .into_iter()
            .fold(self.x.mag(), max_bound)
    }
    fn finest_at(reach: f64) -> Self {
        let side = Interval::from_bounds(reach.next_down(), reach);
        Self {
            x: side,
            y: side,
            z: side,
        }
    }
}

/// The recursion's own bookkeeping: counts, in no units and on no
/// lane.
///
/// The lane belongs to the ANSWER rather than to the walk. Both
/// seeding doors could name their lane — [`seed_r3`] is on ℝ³ and
/// [`seed_chart_plane`] holds the chart rate — and neither has
/// anywhere to put it: a seeding run's survivors are seeds and it
/// returns no receipt at all. So the two accounting doors attach the
/// lane to this, where it is read, rather than the walk carrying a
/// tag half its callers discard.
#[derive(Clone, Copy, Debug, Default)]
struct SweepTally {
    examined: u32,
    excluded: u32,
    accounted: u32,
    refined: u32,
    max_depth: u32,
}

impl SweepTally {
    /// The receipt this walk earned, stated on `lane` and in `lane`'s
    /// own units.
    fn receipt(self, lane: ExhaustLane, floor: f64) -> Exhaustiveness {
        Exhaustiveness {
            lane,
            examined: self.examined,
            excluded: self.excluded,
            accounted: self.accounted,
            refined: self.refined,
            max_depth: self.max_depth,
            floor,
        }
    }
}

/// **The one recursion**, shared by both lanes and both duties.
///
/// Iterative (an explicit stack, so recursion depth is not a stack-
/// overflow path) and depth-first with the two halves pushed in a fixed
/// order — same cells, same order, every run (D9). `excluded` answers
/// the lane's own question: does a certified enclosure over this cell
/// exclude zero, or is the enclosure unusable (a typed refusal)?
///
/// The floor is the only place the two duties differ, and they differ
/// by `duty`, never by the shape of the data: seeding banks the
/// survivor, accounting refuses. Every other line — the budget, the
/// bookkeeping, the exclude/account/refine ladder — is written once, so
/// the never-silence refusal cannot be edited in one lane and forgotten
/// in the other.
fn sweep<C: SweepCell>(
    floor: SweepFloor<C>,
    duty: SweepDuty<'_, C>,
    excluded: impl Fn(C) -> Result<bool, SsiError>,
) -> Result<(SweepTally, Vec<C::Seed>), SsiError> {
    let (lane, width) = (floor.lane, floor.width());
    let mut stats = SweepTally::default();
    let mut out = Vec::new();
    let mut stack = vec![(floor.root, 0u32)];
    while let Some((cell, depth)) = stack.pop() {
        if stats.examined as usize >= SSI_MAX_CELLS {
            return Err(SsiError::CellBudget {
                budget: SSI_MAX_CELLS,
            });
        }
        stats.examined += 1;
        stats.max_depth = stats.max_depth.max(depth);

        // (i) exclusion: no solution can be in this cell.
        if excluded(cell)? {
            stats.excluded += 1;
            continue;
        }
        // (ii) accounted: inside a found branch's uniqueness tube.
        if duty.accounts(cell) {
            stats.accounted += 1;
            continue;
        }
        // (iii) refine, unless we are at the floor.
        if cell.width() <= width {
            match duty {
                SweepDuty::Seed => {
                    out.push(cell.seed());
                    continue;
                }
                SweepDuty::Account { .. } => {
                    return Err(SsiError::ExhaustivenessInconclusive(
                        ExhaustivenessRefusal {
                            lane,
                            cell_width: cell.width(),
                            floor: width,
                            examined: stats.examined,
                        },
                    ));
                }
            }
        }
        stats.refined += 1;
        let (a, b) = cell.split();
        stack.push((b, depth + 1));
        stack.push((a, depth + 1));
    }
    Ok((stats, out))
}

/// **Seed generation**, ℝ³ lane: the centers of the cells that survived
/// exclusion over the floor's root at the floor.
///
/// # Errors
///
/// As [`account_r3`], minus the floor refusal — seeding has no
/// obligation to discharge, so a cell that reaches the floor is a seed.
pub(crate) fn seed_r3(
    s1: &Surface<f64>,
    s2: &Surface<f64>,
    floor: SweepFloor<Box3>,
) -> Result<Vec<Point3<f64>>, SsiError> {
    let (_, seeds) = sweep_r3(s1, s2, floor, SweepDuty::Seed)?;
    Ok(seeds)
}

/// **The accounting proof**, ℝ³ lane: every leaf of the subdivision of
/// `root` is excluded by enclosure or lies inside one of `tubes`.
///
/// A cell that is neither, at the floor, is the typed refusal —
/// whatever `tubes` holds, empty included.
///
/// # Errors
///
/// [`SsiError::ExhaustivenessInconclusive`] at the floor,
/// [`SsiError::CellBudget`] if the enumeration exceeds
/// [`SSI_MAX_CELLS`], [`SsiError::UnsupportedCertificate`] when an
/// operand admits no ring-computable enclosure — which, behind this
/// door, means a degenerate instance of a supported kind.
pub(crate) fn account_r3(
    s1: &Surface<f64>,
    s2: &Surface<f64>,
    tubes: &[Box3],
    floor: SweepFloor<Box3>,
) -> Result<Exhaustiveness, SsiError> {
    let (tally, _) = sweep_r3(s1, s2, floor, SweepDuty::Account { tubes })?;
    Ok(tally.receipt(floor.lane, floor.width()))
}

/// The ℝ³ lane's exclusion rule, over the one shared [`sweep`]: a cell
/// is solution-free when a certified enclosure of either surface's
/// implicit form over it does not contain zero.
fn sweep_r3(
    s1: &Surface<f64>,
    s2: &Surface<f64>,
    floor: SweepFloor<Box3>,
    duty: SweepDuty<'_, Box3>,
) -> Result<(SweepTally, Vec<Point3<f64>>), SsiError> {
    sweep(floor, duty, |cell| {
        let e1 = implicit_enclosure(s1, cell);
        let e2 = implicit_enclosure(s2, cell);
        if !e1.is_certified() || !e2.is_certified() {
            // The kind list in this sentence is held true by the lane
            // gate: `cylinder_sphere_ssi` refuses `WrongLane` for
            // every operand that is not a cylinder or a sphere.
            // Widening that gate (admitting Cone/Torus/Nurbs to this
            // lane) must revisit this sentence with it, or the arm
            // starts blaming kinds it never sees.
            return Err(SsiError::UnsupportedCertificate {
                what: "a degenerate operand — a sphere or cylinder of zero \
                       radius, whose implicit form divides by that radius — \
                       has no ring-computable implicit enclosure, so its \
                       domain cannot be proved exhausted",
            });
        }
        Ok(excludes_zero(e1) || excludes_zero(e2))
    })
}

fn excludes_zero(i: Interval) -> bool {
    i.is_certified() && (i.lo() > 0.0 || i.hi() < 0.0)
}

/// A rectangle in a surface's parameter domain — the chart lane's cell.
#[derive(Clone, Copy, Debug)]
pub(crate) struct UvRect {
    /// `u` bounds, in chart units.
    pub u: (f64, f64),
    /// `v` bounds, in chart units.
    pub v: (f64, f64),
}

impl UvRect {
    /// The wider side; `NaN` when either side is, so a NaN side fails
    /// the floor test rather than dropping out of it.
    fn width(self) -> f64 {
        max_bound(self.u.1 - self.u.0, self.v.1 - self.v.0)
    }

    fn center(self) -> (f64, f64) {
        (0.5 * (self.u.0 + self.u.1), 0.5 * (self.v.0 + self.v.1))
    }

    fn split(self) -> (Self, Self) {
        if (self.u.1 - self.u.0) >= (self.v.1 - self.v.0) {
            let m = 0.5 * (self.u.0 + self.u.1);
            (
                Self {
                    u: (self.u.0, m),
                    ..self
                },
                Self {
                    u: (m, self.u.1),
                    ..self
                },
            )
        } else {
            let m = 0.5 * (self.v.0 + self.v.1);
            (
                Self {
                    v: (self.v.0, m),
                    ..self
                },
                Self {
                    v: (m, self.v.1),
                    ..self
                },
            )
        }
    }

    fn contained_in(self, o: Self) -> bool {
        o.u.0 <= self.u.0 && self.u.1 <= o.u.1 && o.v.0 <= self.v.0 && self.v.1 <= o.v.1
    }
}

impl SweepCell for UvRect {
    type Seed = (f64, f64);

    fn width(self) -> f64 {
        UvRect::width(self)
    }
    fn seed(self) -> (f64, f64) {
        self.center()
    }
    fn split(self) -> (Self, Self) {
        UvRect::split(self)
    }
    fn contained_in(self, other: Self) -> bool {
        UvRect::contained_in(self, other)
    }
    fn reach(self) -> f64 {
        [self.u.1, self.v.0, self.v.1]
            .into_iter()
            .fold(self.u.0.abs(), |m, x| max_bound(m, x.abs()))
    }
    fn finest_at(reach: f64) -> Self {
        let side = (reach.next_down(), reach);
        Self { u: side, v: side }
    }
}

/// **Seed generation**, chart lane: the centers of the parameter cells
/// that survived exclusion against the plane.
///
/// Takes its floor as [`account_chart_plane`] does, minted over the
/// same root by the same door ([`SweepFloor::chart`]): the chart lane
/// is entered through one shape whichever duty is being asked for.
/// What it does not do is hand back a lane — seeding hands back no
/// receipt to state one on.
///
/// # Errors
///
/// As [`account_chart_plane`], minus the floor refusal.
pub(crate) fn seed_chart_plane(
    surface: &NurbsSurface<f64>,
    plane_origin: Point3<f64>,
    plane_normal: Vec3<f64>,
    floor: SweepFloor<UvRect>,
) -> Result<Vec<(f64, f64)>, SsiError> {
    let (_, seeds) =
        sweep_chart_plane(surface, plane_origin, plane_normal, floor, SweepDuty::Seed)?;
    Ok(seeds)
}

/// **The accounting proof**, chart lane: every leaf of the floor's
/// parameter rectangle is excluded or lies inside one of `tubes`; a
/// cell that is neither, at the floor, is the typed refusal — `tubes`
/// empty included.
///
/// The floor carries the certified chart speed of `surface` that
/// crossed the caller's metres into chart units, and hands it on to
/// the receipt so no caller has to hold the rate to read the answer.
///
/// # Errors
///
/// As [`account_r3`].
pub(crate) fn account_chart_plane(
    surface: &NurbsSurface<f64>,
    plane_origin: Point3<f64>,
    plane_normal: Vec3<f64>,
    tubes: &[UvRect],
    floor: SweepFloor<UvRect>,
) -> Result<Exhaustiveness, SsiError> {
    let (tally, _) = sweep_chart_plane(
        surface,
        plane_origin,
        plane_normal,
        floor,
        SweepDuty::Account { tubes },
    )?;
    Ok(tally.receipt(floor.lane, floor.width()))
}

/// The chart lane's exclusion rule, over the one shared [`sweep`]:
/// against a **plane** the locus is `φ(u,v) = n·(S(u,v) − p₀) = 0`, so
/// a cell is solution-free when a zero-free enclosure of `φ` — computed
/// from the surface's certified first-order box — says so.
fn sweep_chart_plane(
    surface: &NurbsSurface<f64>,
    plane_origin: Point3<f64>,
    plane_normal: Vec3<f64>,
    floor: SweepFloor<UvRect>,
    duty: SweepDuty<'_, UvRect>,
) -> Result<(SweepTally, Vec<(f64, f64)>), SsiError> {
    let boxes = NurbsBoxes::new(surface);
    sweep(floor, duty, |cell| {
        let b = boxes.rect_box(cell.u.0, cell.u.1, cell.v.0, cell.v.1);
        let phi = Interval::point(plane_normal.x) * (b.x - Interval::point(plane_origin.x))
            + Interval::point(plane_normal.y) * (b.y - Interval::point(plane_origin.y))
            + Interval::point(plane_normal.z) * (b.z - Interval::point(plane_origin.z));
        if !phi.is_certified() {
            // The one measured route here is weight underflow: the
            // chart-speed mint refuses every net whose homogeneous
            // arithmetic leaves the finite range before this sweep
            // runs, so that cause is named nowhere below.
            return Err(SsiError::UnsupportedCertificate {
                what: "the NURBS control-net enclosure refused over a cell — \
                       a weight so small that the rational's own denominator \
                       underflows to zero",
            });
        }
        Ok(excludes_zero(phi))
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use geom_core::interval::certification::Certification;
    use geom_core::{Bounds, Interval, SupSpeed};

    use super::{
        Box3, ExhaustLane, FloorFault, FloorKind, FloorRefusal, SSI_MAX_CELLS, SweepDuty,
        SweepFloor, UvRect, sweep,
    };
    use crate::ssi::SsiError;

    fn unit_square() -> UvRect {
        UvRect {
            u: (0.0, 1.0),
            v: (0.0, 1.0),
        }
    }

    fn refusal(r: Result<impl core::fmt::Debug, SsiError>) -> FloorRefusal {
        match r {
            Err(SsiError::FloorUnresolvable(r)) => r,
            other => panic!("expected the floor door to refuse, got {other:?}"),
        }
    }

    /// Seed the corner nearest `reach` by excluding every cell that does
    /// not hold it, so the sweep refines toward the domain's largest
    /// coordinate and nowhere else: what it does there is all it does.
    fn corner_sweep<C: super::SweepCell + core::fmt::Debug>(
        floor: SweepFloor<C>,
        holds_corner: impl Fn(C) -> bool,
    ) -> Result<usize, SsiError> {
        sweep(floor, SweepDuty::Seed, |cell| Ok(!holds_corner(cell))).map(|(t, _)| {
            assert!((t.examined as usize) < SSI_MAX_CELLS);
            t.examined as usize
        })
    }

    /// **The door's boundary is exact, on both lanes.** At the domain's
    /// largest coordinate the finest cell bisection cuts is two
    /// adjacent floats, and its split returns the cell itself. A floor
    /// of that cell's width (as the lane measures widths) is minted,
    /// and the sweep refining toward that corner stops there in a few
    /// hundred cells. One ulp of floor narrower is refused by name, and
    /// the same sweep handed it past the door spins to the cell budget,
    /// which is the wrong diagnosis the door exists to replace.
    #[test]
    fn the_floor_door_refuses_exactly_the_floors_bisection_cannot_reach() {
        // Chart lane: `[0, 1]²`, whose finest cell at `1` is
        // `1 − next_down(1)` wide. A unit speed makes metres chart units.
        let gap = 1.0 - 1.0f64.next_down();
        let speed = SupSpeed::new(1.0);
        let at = SweepFloor::chart(unit_square(), gap, speed, FloorKind::Accounting).unwrap();
        let holds = |c: UvRect| c.u.1 == 1.0 && c.v.1 == 1.0;
        let cells = corner_sweep(at, holds).unwrap();
        assert!(
            cells < 1_000,
            "the chart sweep stops at the minted floor: {cells}"
        );
        let below = gap.next_down();
        let r = refusal(SweepFloor::chart(
            unit_square(),
            below,
            speed,
            FloorKind::Accounting,
        ));
        assert_eq!(
            r.fault,
            FloorFault::BelowResolution {
                resolution: gap,
                reach: 1.0
            }
        );
        let forced = SweepFloor { width: below, ..at };
        assert!(
            matches!(
                corner_sweep(forced, holds),
                Err(SsiError::CellBudget { .. })
            ),
            "past the door, one ulp under the boundary, the budget answers"
        );

        // ℝ³ lane: a box reaching `1e8`, where a cell of two adjacent
        // floats is `next_up(ulp)` wide, because `Interval::width`
        // rounds its difference up.
        let reach = 1.0e8f64;
        let side = Interval::from_bounds(reach - 1.0, reach);
        let root = Box3 {
            x: side,
            y: side,
            z: side,
        };
        let finest = Interval::from_bounds(reach.next_down(), reach).width();
        assert_eq!(finest, (reach - reach.next_down()).next_up());
        let at = SweepFloor::r3(root, finest, FloorKind::Accounting).unwrap();
        let holds = |c: Box3| c.x.hi() == reach && c.y.hi() == reach && c.z.hi() == reach;
        let cells = corner_sweep(at, holds).unwrap();
        assert!(
            cells < 1_000,
            "the ℝ³ sweep stops at the minted floor: {cells}"
        );
        let r = refusal(SweepFloor::r3(root, finest.next_down(), FloorKind::Seeding));
        assert_eq!(
            r.fault,
            FloorFault::BelowResolution {
                resolution: finest,
                reach
            }
        );
        let forced = SweepFloor {
            width: finest.next_down(),
            ..at
        };
        assert!(
            matches!(
                corner_sweep(forced, holds),
                Err(SsiError::CellBudget { .. })
            ),
            "past the door, one ulp under the boundary, the budget answers"
        );
    }

    /// **A floor that is not a positive finite width refuses, by why.**
    /// A metre floor of zero, below zero, NaN or `∞` is the caller's
    /// knobs; a metre floor that is a length but divides by a tiny
    /// chart speed to `∞` is the wall's scale.
    #[test]
    fn a_floor_that_is_no_width_refuses_by_its_fault() {
        let speed = SupSpeed::new(1.0);
        for meters in [0.0, -1.0e-9, f64::NAN, f64::INFINITY] {
            let r = refusal(SweepFloor::chart(
                unit_square(),
                meters,
                speed,
                FloorKind::Seeding,
            ));
            assert_eq!(r.fault, FloorFault::NotALength, "{meters:e}");
            assert!(
                r.ending(crate::recourse::Reading::Build)
                    .contains("floor scale"),
                "{meters:e}: the caller's knobs"
            );
        }
        let r = refusal(SweepFloor::chart(
            unit_square(),
            1.0,
            SupSpeed::new(1.0e-320),
            FloorKind::Seeding,
        ));
        assert_eq!(r.fault, FloorFault::Overflows);
        assert!(r.width.is_infinite() && r.meters == 1.0, "{r:?}");
        // A root with a refused side or an infinite one has no reach to
        // resolve against: never "move the geometry nearer the origin".
        let side = Interval::from_bounds(0.0, 1.0);
        for bad in [
            Interval::refused(),
            Interval::from_bounds(0.0, f64::INFINITY),
        ] {
            let root = Box3 {
                x: side,
                y: bad,
                z: side,
            };
            let r = refusal(SweepFloor::r3(root, 1.0e-9, FloorKind::Accounting));
            assert_eq!(r.fault, FloorFault::DomainUnreadable, "{bad:?}");
            let ending = r.ending(crate::recourse::Reading::Build);
            assert!(!ending.contains("nearer the origin"), "{ending}");
        }
    }

    /// **The refusal names the rate, through the chart-length spelling**,
    /// starting from the caller's own metres: `1e-9 m` at a certified
    /// chart speed of `1e150` is `1e-159` chart units, below the
    /// `[0, 1]` domain's resolution.
    #[test]
    fn the_floor_refusal_names_the_rate_that_crossed_it() {
        let r = refusal(SweepFloor::chart(
            unit_square(),
            1.0e-9,
            SupSpeed::new(1.0e150),
            FloorKind::Accounting,
        ));
        assert!(matches!(r.lane, ExhaustLane::Chart { .. }));
        let shown = r.to_string();
        assert_eq!(
            shown,
            format!(
                "the accounting floor 1e-9 m is {:e} chart units at a certified chart speed \
                 of 1e150 m per chart unit, which the domain cannot resolve: where it reaches \
                 1e0 chart units from zero the finest cell bisection can cut is {:e} chart \
                 units wide, so the subdivision could never refine to the floor",
                1.0e-9 / 1.0e150,
                1.0 - 1.0f64.next_down()
            )
        );
    }

    /// **A refused axis keeps a cell's width unreadable.** The sweep
    /// stops refining at `width <= floor`, so a width that dropped the
    /// refused axis would read a cell as floor-sized on the two axes it
    /// can read. On the chart lane a NaN side is the same case.
    #[test]
    fn a_refused_side_makes_the_cell_width_nan() {
        let side = Interval::from_bounds(0.0, 1.0);
        for i in 0..3 {
            let mut axes = [side; 3];
            axes[i] = Interval::refused();
            let cell = Box3 {
                x: axes[0],
                y: axes[1],
                z: axes[2],
            };
            assert!(cell.width().is_nan(), "axis {i}: {}", cell.width());
        }
        for cell in [
            UvRect {
                u: (0.0, f64::NAN),
                v: (0.0, 1.0),
            },
            UvRect {
                u: (0.0, 1.0),
                v: (f64::NAN, 1.0),
            },
        ] {
            assert!(cell.width().is_nan(), "{cell:?}");
        }
    }
}
