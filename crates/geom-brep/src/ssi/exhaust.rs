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
use geom_core::{Point3, RingInterval, SupSpeed, Vec3};

use super::SsiError;
use super::enclose::{Box3, NurbsBoxes, implicit_enclosure};

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
}

/// **One spelling of a chart-lane length**, for both types' `Display`.
///
/// A chart-unit number is unreadable without the metres it stands for
/// and the rate that crossed it, and the receipt and the refusal make
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
    /// On the chart lane this multiplies a real chart width by an
    /// UPPER bound on the surface's speed, so it over-states the cell:
    /// it is a ceiling on how large the unproved region can be, never
    /// a measurement of it.
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
        /// The uniqueness tubes limb 3 banked.
        tubes: &'a [C],
        /// The lane whose units `tubes`, the floor and every cell
        /// width are in — carried by this arm and not by the other
        /// because only the accounting duty hands back a lane-tagged
        /// answer.
        lane: ExhaustLane,
    },
}

impl<C: SweepCell> SweepDuty<'_, C> {
    /// Whether limb 3 has already proved what is in this cell. Seeding
    /// proves nothing about any cell, so the answer is `false` because
    /// of the duty — not because a tube set happens to be empty.
    fn accounts(self, cell: C) -> bool {
        match self {
            Self::Seed => false,
            Self::Account { tubes, .. } => tubes.iter().any(|t| cell.contained_in(*t)),
        }
    }
}

/// A cell of the subdivision. The two lanes differ in their shape and
/// in what a survivor hands the marcher; everything the recursion does
/// with a cell is here, so the recursion itself is written once.
trait SweepCell: Copy {
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
    root: C,
    duty: SweepDuty<'_, C>,
    floor: f64,
    excluded: impl Fn(C) -> Result<bool, SsiError>,
) -> Result<(SweepTally, Vec<C::Seed>), SsiError> {
    let mut stats = SweepTally::default();
    let mut out = Vec::new();
    let mut stack = vec![(root, 0u32)];
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
        if cell.width() <= floor {
            match duty {
                SweepDuty::Seed => {
                    out.push(cell.seed());
                    continue;
                }
                SweepDuty::Account { lane, .. } => {
                    return Err(SsiError::ExhaustivenessInconclusive(
                        ExhaustivenessRefusal {
                            lane,
                            cell_width: cell.width(),
                            floor,
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
/// exclusion over `root` at `floor`.
///
/// # Errors
///
/// As [`account_r3`], minus the floor refusal — seeding has no
/// obligation to discharge, so a cell that reaches the floor is a seed.
pub(crate) fn seed_r3(
    s1: &Surface<f64>,
    s2: &Surface<f64>,
    root: Box3,
    floor_meters: f64,
) -> Result<Vec<Point3<f64>>, SsiError> {
    let (_, seeds) = sweep_r3(s1, s2, root, SweepDuty::Seed, floor_meters)?;
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
    root: Box3,
    tubes: &[Box3],
    floor_meters: f64,
) -> Result<Exhaustiveness, SsiError> {
    let lane = ExhaustLane::R3;
    let (tally, _) = sweep_r3(
        s1,
        s2,
        root,
        SweepDuty::Account { tubes, lane },
        floor_meters,
    )?;
    Ok(tally.receipt(lane, floor_meters))
}

/// The ℝ³ lane's exclusion rule, over the one shared [`sweep`]: a cell
/// is solution-free when a certified enclosure of either surface's
/// implicit form over it does not contain zero.
fn sweep_r3(
    s1: &Surface<f64>,
    s2: &Surface<f64>,
    root: Box3,
    duty: SweepDuty<'_, Box3>,
    floor: f64,
) -> Result<(SweepTally, Vec<Point3<f64>>), SsiError> {
    sweep(root, duty, floor, |cell| {
        let e1 = implicit_enclosure(s1, cell);
        let e2 = implicit_enclosure(s2, cell);
        if e1.is_poison() || e2.is_poison() {
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

fn excludes_zero(i: RingInterval) -> bool {
    !i.is_poison() && (i.lo() > 0.0 || i.hi() < 0.0)
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
    fn width(self) -> f64 {
        (self.u.1 - self.u.0).max(self.v.1 - self.v.0)
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
}

/// **Seed generation**, chart lane: the centers of the parameter cells
/// that survived exclusion against the plane.
///
/// Takes `speed` and `floor_meters` as [`account_chart_plane`] does,
/// and crosses the one into the other's units the same way: the chart
/// lane is entered through one shape whichever duty is being asked
/// for. What it does not do is hand back a lane — seeding hands back
/// no receipt to state one on.
///
/// # Errors
///
/// As [`account_chart_plane`], minus the floor refusal.
pub(crate) fn seed_chart_plane(
    surface: &NurbsSurface<f64>,
    plane_origin: Point3<f64>,
    plane_normal: Vec3<f64>,
    root: UvRect,
    speed: SupSpeed<f64>,
    floor_meters: f64,
) -> Result<Vec<(f64, f64)>, SsiError> {
    let (_, seeds) = sweep_chart_plane(
        surface,
        plane_origin,
        plane_normal,
        root,
        SweepDuty::Seed,
        speed.to_param(floor_meters),
    )?;
    Ok(seeds)
}

/// **The accounting proof**, chart lane: every leaf of the parameter
/// rectangle is excluded or lies inside one of `tubes`; a cell that is
/// neither, at the floor, is the typed refusal — `tubes` empty
/// included.
///
/// `floor_meters` is the caller's floor as a length, and `speed` the
/// certified chart speed of `surface`; the door crosses the one into
/// the other's units once, and hands both on to the receipt so no
/// caller has to hold the rate to read the answer.
///
/// # Errors
///
/// As [`account_r3`].
pub(crate) fn account_chart_plane(
    surface: &NurbsSurface<f64>,
    plane_origin: Point3<f64>,
    plane_normal: Vec3<f64>,
    root: UvRect,
    tubes: &[UvRect],
    speed: SupSpeed<f64>,
    floor_meters: f64,
) -> Result<Exhaustiveness, SsiError> {
    let lane = ExhaustLane::Chart { speed };
    let floor_uv = speed.to_param(floor_meters);
    let (tally, _) = sweep_chart_plane(
        surface,
        plane_origin,
        plane_normal,
        root,
        SweepDuty::Account { tubes, lane },
        floor_uv,
    )?;
    Ok(tally.receipt(lane, floor_uv))
}

/// The chart lane's exclusion rule, over the one shared [`sweep`]:
/// against a **plane** the locus is `φ(u,v) = n·(S(u,v) − p₀) = 0`, so
/// a cell is solution-free when a zero-free enclosure of `φ` — computed
/// from the surface's certified first-order box — says so.
fn sweep_chart_plane(
    surface: &NurbsSurface<f64>,
    plane_origin: Point3<f64>,
    plane_normal: Vec3<f64>,
    root: UvRect,
    duty: SweepDuty<'_, UvRect>,
    floor_uv: f64,
) -> Result<(SweepTally, Vec<(f64, f64)>), SsiError> {
    let boxes = NurbsBoxes::new(surface);
    sweep(root, duty, floor_uv, |cell| {
        let b = boxes.rect_box(cell.u.0, cell.u.1, cell.v.0, cell.v.1);
        let phi = RingInterval::point(plane_normal.x) * (b.x - RingInterval::point(plane_origin.x))
            + RingInterval::point(plane_normal.y) * (b.y - RingInterval::point(plane_origin.y))
            + RingInterval::point(plane_normal.z) * (b.z - RingInterval::point(plane_origin.z));
        if phi.is_poison() {
            // The one measured route here is weight underflow: the
            // seeding guard refuses every net whose homogeneous
            // arithmetic leaves the finite range before this sweep
            // runs, so that cause is named nowhere below.
            return Err(SsiError::UnsupportedCertificate {
                what: "the NURBS control-net enclosure poisoned over a cell — \
                       a weight so small that the rational's own denominator \
                       underflows to zero",
            });
        }
        Ok(excludes_zero(phi))
    })
}
