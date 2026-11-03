//! The **sector-shape** predicates of a corner: a face's corner at a
//! vertex, between two bounding chords leaving the vertex, read about
//! the face's outward normal there. Three rungs decide it, one metering
//! arm, one wideness verdict and one subdivision direction
//! ([`sector_shape`]), and [`corner_shape`] reads a corner whose bounds
//! the face wraps a full turn round.
//!
//! # The inputs
//!
//! A corner is given as two chords leaving the vertex, each scaled to
//! its edge's length (a line edge's chord; a curved edge's extent from
//! below, [`crate::edge_extent_inf`], through [`crate::ConicDeparture`]),
//! and the face's [`OutwardNormal`] at the vertex. `own` is the corner's
//! own chord, its counter-clockwise-**last** bound, and `next` the
//! counter-clockwise-**first**: the corner's interior directions sweep
//! from `next` counter-clockwise about the normal to `own`. The rungs
//! read the bounds as given and the normal's sense as it stands; they
//! apply no sense of their own.
//!
//! # What each rung decides
//!
//! 1. **Is the corner metered at all?** The lever arm is the shorter of
//!    the two bounds' lengths ([`sector_arm`]); a bound whose length is
//!    not a finite number, or underflowed out of the format, is refused
//!    as such, and a non-positive arm is a degenerate corner, refused
//!    typed.
//! 2. **Is the corner convex (< 180°)?** `sin θ = (b̂ × â)·n` metered at
//!    the arm. Positive: the corner is convex and one entry reads it
//!    (`bisector` is `None`).
//! 3. **If not definitely convex, where is it subdivided?** Definite
//!    reflex ⇒ `−normalize(â + b̂)`, the bisector of the reflex span; the
//!    straight band (θ ≈ π, where `â + b̂` collapses) ⇒ `n × b̂`, 90° into
//!    the interior. θ ≈ 0 or ≈ 2π between two distinct edges is a spike,
//!    ill-conditioned, and refused rather than guessed; between the two
//!    ends of one orbit half-edge (`full_circle`, a strut vertex) it is
//!    the legitimate full-circle sector and takes the 90° direction.
//!
//! **The full turn.** Where the caller's structure says the face wraps a
//! full turn round the bounds ([`CornerBounds::SlitTip`]: two coincident
//! edges at a slit's tip, or one edge walked out and straight back at a
//! wrap edge's tip), the spike reading is that turn: [`corner_shape`]
//! returns it ([`FullTurn`]), cut at its three quarters
//! ([`TurnOrder::quarter_fractions`]). Which refusals read as the turn
//! is one rule, [`reads_slit_tip_turn`].
//!
//! **Where the rungs do not apply.** They assume the face has a tangent
//! plane at the vertex. At a cone's apex it has none: the interior is
//! the fan of rulings between the bounds at every width, not the
//! positive cone of the bounds, so rungs 2 and 3 answer nothing about
//! it. A reader of such a corner takes [`sector_arm`] and
//! [`metering_arm`] from here and reads the window itself, and takes a
//! window of no width at [`CornerBounds::SlitTip`] bounds as the whole
//! turn by [`reads_slit_tip_turn`].
//!
//! # Why convex subdivision
//!
//! A face with a tangent plane at the vertex is planar there to first
//! order, so a corner's interior is exactly the positive cone of its two
//! bounding directions **iff its angle is < 180°**: then the bounds'
//! sides of any plane through the vertex decide every interior
//! direction's side (a positive combination of two same-side vectors
//! stays on that side), and one entry suffices. At ≥ 180° the cone
//! argument fails, the interior can cross the plane while both bounds
//! sit on one side, so the corner is split at an interior direction into
//! two corners each < 180°, which restores it. The book's
//! store-twice-with-bisector is this convex subdivision; the paper's
//! complement-and-negate at 180° (interior-vector sign past it) answers
//! point membership, not side classification, and offers no cone
//! argument.
//!
//! The subdivision direction need not be the exact bisector: any
//! interior direction with both sub-angles < 180° is valid, which is
//! what makes `n × b̂` sound throughout the straight band. So the
//! wideness verdict has **no escalation cliff**: subdividing is sound at
//! every angle (a convex corner subdivided is merely redundant), so a
//! zero or in-band reading of rung 2 takes the subdivision path. That is
//! the deliberate posture of this one predicate; rung 3's spike
//! refusal, and every side verdict a caller takes over the entries,
//! stay strict.
//!
//! # Which scalars the rungs are symmetric for
//!
//! Rung 3's reflex bisector sums `â + b̂` and its straight margin reads
//! `â · b̂`; each is the same bits with its two operands swapped at the
//! scalars below, so a caller that names its bounds in the other order
//! reads the same margins.
//!
//! - **`f64`, and the recording scalar `geom_core::k_stats::Probe`
//!   (an `f64` newtype): proven.** [`Vec3::dot`] is the fixed
//!   association `((x·x′) + (y·y′)) + (z·z′)`, so swapping the
//!   arguments commutes each product and permutes nothing in the sum;
//!   IEEE `*` and `+` are commutative on `f64` at every finite input,
//!   ±0 included. `geom_core`'s `dot_symmetry_bit_exact` proptest pins
//!   it (over `1.0e-3..1.0e3`, so 0, −0, inf and subnormals are argued
//!   rather than sampled). The one gap is NaN, whose payload
//!   propagation is not specified bitwise, and a NaN chord fails rung 1
//!   before it reaches rung 3.
//! - **`geom_core::Interval`: NOT proven.** It is also a [`Decide`]
//!   scalar, and its `Add`/`Mul` delegate to the enclosure backend,
//!   whose bit-level commutativity is asserted nowhere in-tree. Closing
//!   it needs a `Vec3<Interval>` analogue of `dot_symmetry_bit_exact` in
//!   `geom-core`. A divergence would be an enclosure differing in its
//!   last bit, not a verdict changing.
//!
//! # One K name per rung
//!
//! The three rungs emit `sector_arm`, `sector_reflex` and
//! `sector_straight`, **one name each, spelled here and nowhere else**:
//! a reader that wants one of these margins calls [`sector_shape`] (or
//! [`sector_arm`]). The names are private to this module, so no other
//! file can import one; a crate's own guard row reads its sources for
//! the quoted form (this crate's is `the_rungs_are_decided_in_one_place`
//! below). The ruling that pooled the per-reader names into these three,
//! and its evidence, is `docs/K-REPORT.md`'s census note (2026-08-19).

use crate::OutwardNormal;
use geom_core::k_stats::{decide, decide_negative, decide_positive};
use geom_core::{
    Band, Decide, Indeterminate, Lever, Margin, Point3, Real, Sign, Vec3, is_finite_length,
    is_underflowed_length,
};

/// Rung 1's K name: the metering arm is positive. Private, as its two
/// siblings are (module docs, "One K name per rung").
const SECTOR_ARM: &str = "sector_arm";

/// Rung 2's K name: the corner is convex (`sin θ` levered at the arm).
const SECTOR_REFLEX: &str = "sector_reflex";

/// Rung 3's K name: the straight/spike disambiguation (`cos θ` levered
/// at the arm, so a straight corner reads NEGATIVE), reached only when
/// rung 2 is not definitely signed. A recourse may read that sign: a
/// tighter tolerance can decide a straight corner, never a spike.
const SECTOR_STRAIGHT: &str = "sector_straight";

/// What the rung named `predicate` decides, in the words a flip report
/// states; `None` for a name that is not one of the three.
pub fn rung_words(predicate: &str) -> Option<&'static str> {
    Some(match predicate {
        SECTOR_ARM => SectorRung::Arm.subject(),
        SECTOR_REFLEX => "whether a corner is convex or reflex",
        SECTOR_STRAIGHT => SectorRung::Straight { full_circle: false }.subject(),
        _ => return None,
    })
}

/// Which rung of [`sector_shape`] refused: the refusal's words and what
/// the rung passes on. Rung 2 (convexity) never refuses, since an
/// undecided reading of it falls through to rung 3.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(
    any(test, feature = "test-support"),
    derive(strum::EnumDiscriminants),
    strum_discriminants(name(SectorRungKind), derive(strum::EnumIter), doc(hidden))
)]
pub enum SectorRung {
    /// Rung 1: the metering arm is a positive length.
    Arm,
    /// Rung 3: the corner is straight, not a spike.
    Straight {
        /// The two bounds are one orbit half-edge (a strut vertex),
        /// where a reading of θ ≈ 0 or ≈ 2π is legitimate too.
        full_circle: bool,
    },
}

impl SectorRung {
    /// What the rung decides, as a clause with no colon or dash of its
    /// own.
    #[must_use]
    pub const fn subject(self) -> &'static str {
        match self {
            Self::Arm => "whether a corner's edges are long enough to measure its angle over",
            Self::Straight { .. } => "whether a corner is straight or folds back on itself",
        }
    }
}

/// Why [`sector_shape`] refused.
///
/// Two causes, kept apart because their recourses are different
/// sentences: a bounding chord whose length is not a NUMBER is not a
/// coincidence at any tolerance and no tolerance lever reaches it,
/// while everything the rungs themselves refuse is a band question
/// carrying a classifier payload.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SectorFault {
    /// A bounding chord's length is not a finite number: its
    /// components overflow the norm (past ~1e154), or one of them is
    /// not a number. Carries nothing — which of the two chords it was
    /// is not a distinction a caller can act on, and the recourse is
    /// the same either way.
    NonFiniteChord,
    /// A bounding chord's length **underflowed out of the format**:
    /// its components are small enough (below ~1e-162 at `f64`) that
    /// `norm_squared` flushed to zero, so the norm is exactly zero for
    /// a chord that has a perfectly good direction. Carries nothing,
    /// for the reason its overflow sibling carries nothing.
    ///
    /// Its own arm because no tolerance lever reaches it either: the
    /// squared norm is zero at every eps, so the arm rung below decides
    /// `Zero` definitely and the corner is reported as an invalid
    /// (non-positive) arm. That is true of the arithmetic and false of
    /// the input, whose recourse is the overflow end's — scale.
    UnderflowedChord,
    /// A rung refused or escalated.
    Rung {
        /// Which rung.
        rung: SectorRung,
        /// The rung's diagnostics, named by the rung.
        diag: Indeterminate,
    },
}

/// What the sector-shape rungs decided about one corner.
#[derive(Clone, Copy, Debug)]
pub struct SectorShape<T: Real> {
    /// The metering arm: the shorter of the two bounding chords, in
    /// metres. Every downstream margin of this sector is levered at it.
    pub arm: T,
    /// Unit direction of the sector's own orbit chord (CCW-last bound).
    pub unit_own: Vec3<T>,
    /// Unit direction of the next orbit chord (CCW-first bound).
    pub unit_next: Vec3<T>,
    /// `None` when the corner is definitely convex (one entry
    /// suffices); otherwise the interior direction the sector is
    /// convexly subdivided at.
    pub bisector: Option<Vec3<T>>,
}

/// Climbs the three rungs for one sector (module docs).
///
/// `dir_own` and `dir_next` are the bounds **scaled to their edges'
/// lengths** (a line's chord, a curved edge's extent from below) — their
/// norms are the arm's raw material and their directions the bounds.
/// `normal` is the face's [`OutwardNormal`] at the base vertex, its sense
/// already folded in; the bounds are in the orbit order that sense
/// describes, so the rungs apply no sense of their own (a second factor
/// would read every convex corner as reflex).
/// `full_circle` says the two bounds are the SAME orbit half-edge (a
/// strut vertex), which is what makes a θ ≈ 0 / ≈ 2π reading
/// legitimate rather than a spike.
///
/// **Rung 0: are both chord lengths NUMBERS?** Asked before rung 1,
/// and asked of each chord SEPARATELY, because rung 1's arm is their
/// `min` and [`Real::min`] propagates `NaN` but not infinity — so
/// `min` hides an OVERFLOWED chord behind a finite one (`min(3, ∞) =
/// 3`) while a poisoned one stops rung 1 on its own. A chord past
/// [`Vec3::norm`]'s ~1e154 overflow band has an infinite norm;
/// hidden under the `min`, rung 1 sees a healthy arm and passes on a
/// length that is not a number.
///
/// Measured on this body before the question went first, over all
/// eight shapes (four chord pairs × `full_circle`): **two** of them
/// returned `Ok`, both with `full_circle` set and one chord
/// overflowed, once in each order. One produced an all-zero bisector;
/// the other produced `(-1,0,0)` — a unit direction built by `n ×
/// unit_next` from the chord that survived, with nothing about it to
/// smell. The other six refused, but never for this reason and never
/// with a recourse that could work: the two poisoned shapes escalated
/// at rung 1, the two doubly-overflowed ones at rung 3, and the two
/// single-overflow shapes with `full_circle` clear were refused as
/// SPIKES. The tests carry the full table.
///
/// **Rung 0 asks the other end of the format too**, per chord for the
/// same reason: a chord below `Vec3::norm`'s ~1e-162 underflow
/// band squares to zero, so `norm` is EXACTLY zero, and `min` puts that
/// zero straight into the arm — an underflowed chord hides behind a
/// good one exactly as an overflowed one did, and it hides HARDER,
/// because zero is what `min` prefers. The arm rung then decides `Zero`
/// definitely and the corner is refused as a non-positive arm: true of
/// the arithmetic, false of the chord, and offering a band that cannot
/// reach a squared norm of zero. `is_underflowed_length` is asked
/// against each chord's own largest `|component|`
/// (`Vec3::norm_witness`), which is that predicate's contract, and it
/// is asked AFTER the finiteness question because an overflowed or
/// poisoned length makes its two ratios non-finite for an unrelated
/// reason.
///
/// **Point-scalar gates.** Both are value questions, and at
/// `T = Interval` neither bites: `Interval::is_poison` is
/// `is_nai() || is_empty()`, and `[1e200, ∞] − [1e200, ∞]` is
/// `[−∞, ∞]`, which answers finite; and a norm whose lower end
/// underflowed still ENCLOSES the true length, so the underflow ratio
/// is an unbounded enclosure rather than poison and the question
/// answers `false`. So rung 0 bites at `f64` and `Probe` and waves an
/// interval chord through to the rungs below. No live caller
/// instantiates this body at `Interval` today; see
/// `geom_core::is_finite_length` for the general statement.
///
/// **K consequence.** Rung 0 refuses before the funnel, so a
/// non-finite chord contributes no sample under `sector_arm` (nor,
/// where it used to reach them, under `sector_reflex` or
/// `sector_straight`). Those samples were a `+∞` margin recorded as a
/// definite `Positive` and two NaN margins recorded as escalations.
/// Rung 0 emits no sample of its own: it is a value-channel question,
/// not a classification, so it adds no row to `docs/K-REPORT.md`.
///
/// # Errors
///
/// [`SectorFault::NonFiniteChord`] when either bounding chord's
/// length is not a finite number;
/// [`SectorFault::UnderflowedChord`] when either underflowed out of
/// the format; otherwise [`SectorFault::Rung`]
/// named by the rung that produced it — the funnel's escalation, which
/// for a definite verdict the rung does not admit (non-positive arm; a
/// spike between distinct edges) is the gate's rejection from
/// [`decide_positive`] or [`decide_negative`], carrying the decided
/// margin, on the frame's log.
pub fn sector_shape<T: Decide>(
    dir_own: Vec3<T>,
    dir_next: Vec3<T>,
    normal: OutwardNormal<T>,
    full_circle: bool,
    band: Band,
) -> Result<SectorShape<T>, SectorFault> {
    let arm = sector_arm(dir_own, dir_next, band)?;
    let (unit_own, unit_next) = (dir_own.normalize(), dir_next.normalize());
    // Wideness: sin θ = (b̂ × â)·n metered at the arm. Positive ⇒
    // convex; negative ⇒ reflex; zero-band ⇒ disambiguate by cosine
    // (for unit bounds sin and cos cannot both vanish, so the second
    // margin is definite whenever the first is not).
    let n = normal.vec();
    let reflex_margin = Margin::levered(unit_next.cross(unit_own).dot(n), metering_arm(arm));
    let bisector = match decide(SECTOR_REFLEX, reflex_margin, band) {
        Ok(Sign::Positive) => None,
        // Definite reflex, θ ∈ (π, 2π): symmetric in the two bounds at
        // the scalars the module docs name. The collapse â + b̂ → 0
        // happens only at θ → π, which lands in the Zero band below.
        Ok(Sign::Negative) => Some(-((unit_own + unit_next).normalize())),
        Ok(Sign::Zero) | Err(_) => {
            let straight_margin = Margin::levered(unit_own.dot(unit_next), metering_arm(arm));
            let rung = SectorRung::Straight { full_circle };
            if full_circle {
                // A one-edge orbit admits every definite reading: θ ≈ π,
                // or θ ≈ 0 / ≈ 2π, the legitimate full-circle sector.
                decide(SECTOR_STRAIGHT, straight_margin, band).map_err(refused(rung))?;
            } else {
                // Between two distinct edges only θ ≈ π (a negative
                // `cos θ`) is a corner; a spike has no interior
                // direction to guess.
                decide_negative(SECTOR_STRAIGHT, straight_margin, band).map_err(refused(rung))?;
            }
            // 90° into the interior, valid throughout the band.
            Some(n.cross(unit_next))
        }
    };
    Ok(SectorShape {
        arm,
        unit_own,
        unit_next,
        bisector,
    })
}

/// Rungs 0 and 1 of [`sector_shape`]: both bounds' lengths are numbers,
/// and the shorter, the corner's metering arm, is positive.
pub fn sector_arm<T: Decide>(
    dir_own: Vec3<T>,
    dir_next: Vec3<T>,
    band: Band,
) -> Result<T, SectorFault> {
    let (norm_own, norm_next) = (dir_own.norm(), dir_next.norm());
    if !is_finite_length(norm_own) || !is_finite_length(norm_next) {
        return Err(SectorFault::NonFiniteChord);
    }
    // Per CHORD, against that chord's own witness — the arm below is
    // their `min`, which has no witness of its own and would hide the
    // underflowed one behind the good one.
    if is_underflowed_length(norm_own, dir_own.norm_witness())
        || is_underflowed_length(norm_next, dir_next.norm_witness())
    {
        return Err(SectorFault::UnderflowedChord);
    }
    let arm = norm_own.min(norm_next);
    decide_positive(SECTOR_ARM, Margin::of(arm), band).map_err(refused(SectorRung::Arm))?;
    Ok(arm)
}

/// **A corner's metering arm** ([`sector_shape`]'s, the shorter of its
/// bounds' departure vectors' norms) as the lever its sectors read at.
pub fn metering_arm<T: Real>(arm: T) -> Lever<T> {
    // Unsourced lever: the norm of a departure vector, a curved bound's
    // read back from its extent times a unit tangent (or a line's chord),
    // which differs from that extent by rounding: the edges' `Lever::min`
    // would round differently.
    Lever::unsourced(arm)
}

/// [`sector_shape`] for the wedge square to an edge: the sector CCW
/// about the edge's own direction `axis` from `dir_next` to `dir_own`,
/// both square to it. The orientation is the edge's (its plus half runs
/// along `axis`), not a face's, so there is no sense bit to fold:
/// `axis` is the sector plane's normal as it stands.
///
/// # Errors
///
/// As [`sector_shape`].
pub fn edge_wedge_shape<T: Decide>(
    dir_own: Vec3<T>,
    dir_next: Vec3<T>,
    axis: Vec3<T>,
    band: Band,
) -> Result<SectorShape<T>, SectorFault> {
    sector_shape(
        dir_own,
        dir_next,
        OutwardNormal::from_chart(axis, true),
        false,
        band,
    )
}

/// A rung's escalation, as the fault naming that rung. The escalation
/// is the funnel's — an in-band margin, or a definite verdict the
/// rung's gate does not admit — so it is on the frame's log already.
fn refused(rung: SectorRung) -> impl FnOnce(Indeterminate) -> SectorFault {
    move |diag| SectorFault::Rung { rung, diag }
}

/// What the caller's structure says about a corner's two bounds, which
/// [`corner_shape`] reads a θ ≈ 0 / ≈ 2π reading by.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CornerBounds {
    /// Two distinct edges that do not coincide: such a reading is a
    /// spike.
    Distinct,
    /// One orbit half-edge, a strut vertex ([`sector_shape`]'s
    /// `full_circle`).
    OneEdge,
    /// Bounds the face wraps a full turn round: two coincident edges at
    /// a slit's tip, or one edge walked out and straight back at a wrap
    /// edge's tip.
    SlitTip,
}

/// What [`corner_shape`] read: the rungs' answer, or a full turn.
#[derive(Clone, Copy, Debug)]
pub enum CornerShape<T: Real> {
    /// The rungs decided the corner ([`sector_shape`]).
    Sector(SectorShape<T>),
    /// The face's full turn round bounds read [`CornerBounds::SlitTip`].
    FullTurn(FullTurn<T>),
}

/// A face's full turn round [`CornerBounds::SlitTip`] bounds, from the
/// next orbit chord counter-clockwise (about the face's outward normal)
/// round to the corner's own, the two along one ray.
#[derive(Clone, Copy, Debug)]
pub struct FullTurn<T: Real> {
    /// The metering arm, as [`SectorShape::arm`].
    pub arm: T,
    /// Unit direction of the next orbit chord, the turn's start.
    pub unit_next: Vec3<T>,
    /// Unit direction of the corner's own orbit chord, its end.
    pub unit_own: Vec3<T>,
    /// `unit_next` turned a quarter about the outward normal.
    quarter: Vec3<T>,
}

/// The order a reader takes a whole turn's quarters in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TurnOrder {
    /// Counter-clockwise from the next chord: the sector bounds' order.
    FromNext,
    /// From the corner's own chord round to the next: the orbit's order.
    FromOwn,
}

impl TurnOrder {
    /// **The quarter cuts of a whole turn, as fractions of it from the
    /// next bound**, listed in this order: a quarter, a half and three
    /// quarters from [`TurnOrder::FromNext`], the reverse from
    /// [`TurnOrder::FromOwn`]. Every whole turn is cut here, a tangent
    /// plane's ([`FullTurn::quarters`]) and a caller's own (a cone apex's
    /// window, cut in azimuth).
    #[must_use]
    pub const fn quarter_fractions(self) -> [f64; 3] {
        match self {
            Self::FromNext => [0.25, 0.5, 0.75],
            Self::FromOwn => [0.75, 0.5, 0.25],
        }
    }
}

impl<T: Real> FullTurn<T> {
    /// The three directions that cut the turn into convex quarters:
    /// `unit_next` turned [`TurnOrder::quarter_fractions`] of the turn,
    /// listed in `order`.
    pub fn quarters(&self, order: TurnOrder) -> [Vec3<T>; 3] {
        let (q, back) = (self.quarter, -self.unit_next);
        match order {
            TurnOrder::FromNext => [q, back, -q],
            TurnOrder::FromOwn => [-q, back, q],
        }
    }
}

/// [`sector_shape`], with a reading that the bounds leave along one ray
/// read by `bounds`: for [`CornerBounds::SlitTip`] the face's full turn,
/// otherwise a spike as the rungs refuse it.
///
/// # Errors
///
/// As [`sector_shape`], but for the full turn round
/// [`CornerBounds::SlitTip`] bounds.
pub fn corner_shape<T: Decide>(
    dir_own: Vec3<T>,
    dir_next: Vec3<T>,
    normal: OutwardNormal<T>,
    bounds: CornerBounds,
    band: Band,
) -> Result<CornerShape<T>, SectorFault> {
    match sector_shape(
        dir_own,
        dir_next,
        normal,
        bounds == CornerBounds::OneEdge,
        band,
    ) {
        Ok(shape) => Ok(CornerShape::Sector(shape)),
        Err(fault) if reads_slit_tip_turn(bounds, &fault, Sign::Positive) => {
            let unit_next = dir_next.normalize();
            Ok(CornerShape::FullTurn(FullTurn {
                arm: dir_own.norm().min(dir_next.norm()),
                unit_next,
                unit_own: dir_own.normalize(),
                quarter: normal.vec().cross(unit_next),
            }))
        }
        Err(fault) => Err(fault),
    }
}

/// **Whether a refused corner is a whole turn**: `bounds` is
/// [`CornerBounds::SlitTip`], and `fault` is the straight rung's
/// rejection of the reading `along`, which says the corner's bounds
/// leave along one ray. `along` is `Positive` for a tangent plane's
/// `cos θ = +1` ([`corner_shape`]) and `Zero` for a reading whose
/// window between the bounds has no width (a cone apex's chord, a
/// caller's own). Between distinct edges the same rejection is a spike,
/// and it stays refused.
#[must_use]
pub fn reads_slit_tip_turn(bounds: CornerBounds, fault: &SectorFault, along: Sign) -> bool {
    bounds == CornerBounds::SlitTip
        && matches!(
            fault,
            SectorFault::Rung {
                rung: SectorRung::Straight { .. },
                diag,
            } if diag.margin.rejected_sign() == Some(along)
        )
}

/// **A point's signed distance from a plane, in metres**: `q` against
/// the plane through `origin` with unit normal `normal`, positive on
/// the normal's side. A point needs no lever: its distance is what a
/// margin measures.
pub fn plane_offset<T: Real>(origin: Point3<T>, normal: Vec3<T>, q: Point3<T>) -> T {
    (q - origin).dot(normal)
}

/// [`plane_offset`] with a first-order bound on its own rounding, its
/// inputs exact ([`geom_core::running`]): the value is bit-identical to
/// [`plane_offset`]'s, so a door that decides on one charges the other.
pub fn plane_offset_rounded<T: Real>(
    origin: Point3<T>,
    normal: Vec3<T>,
    q: Point3<T>,
) -> geom_core::running::Rounded<T> {
    use geom_core::running::{self, Rounded};
    let d = [(q.x, origin.x), (q.y, origin.y), (q.z, origin.z)]
        .map(|(a, b)| Rounded::exact(a) - Rounded::exact(b));
    running::dot(d, running::exact_vec(normal))
}

/// **A point's distance from the closed segment `a → b`, in metres**:
/// to the foot of the perpendicular where it falls on the segment, to
/// the nearer end otherwise (the foot parameter clamped by `min`/`max`,
/// no comparison). The segment has positive length.
pub fn point_segment_distance<T: Real>(a: Point3<T>, b: Point3<T>, q: Point3<T>) -> T {
    let (d, w) = (b - a, q - a);
    let t = (w.dot(d) / d.norm_squared()).max(T::zero()).min(T::one());
    (w - d * t).norm()
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::{ErrorTextReading, Tol};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// A rung refusal as `(rung, predicate, the margin's reading, the
    /// sign its gate rejected)`, so a row compares against the payload
    /// rather than against the code under test; `None` for any other
    /// fault.
    fn gate_refusal(
        fault: Option<SectorFault>,
    ) -> Option<(
        SectorRung,
        Option<&'static str>,
        ErrorTextReading,
        Option<Sign>,
    )> {
        match fault? {
            SectorFault::Rung { rung, diag } => Some((
                rung,
                diag.predicate,
                diag.margin.diagnostic_f64_for_error_text(),
                diag.margin.rejected_sign(),
            )),
            _ => None,
        }
    }

    /// `Vec3` carries no `PartialEq`; these rows want BIT equality, not
    /// a tolerance.
    fn bits_eq(a: Vec3<f64>, b: Vec3<f64>) -> bool {
        a.x.to_bits() == b.x.to_bits()
            && a.y.to_bits() == b.y.to_bits()
            && a.z.to_bits() == b.z.to_bits()
    }

    fn assert_dir(got: Vec3<f64>, want: Vec3<f64>) {
        assert!(
            (got.x - want.x).abs() < 1e-15
                && (got.y - want.y).abs() < 1e-15
                && (got.z - want.z).abs() < 1e-15,
            "direction {got:?} is not {want:?}"
        );
    }

    fn shape(
        own: Vec3<f64>,
        next: Vec3<f64>,
        full_circle: bool,
    ) -> Result<SectorShape<f64>, SectorFault> {
        let n = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true);
        sector_shape(own, next, n, full_circle, band())
    }

    /// A right-angle corner (θ = 90°, `sin θ` = +1) is convex: no
    /// subdivision, arm = the shorter bounding chord.
    #[test]
    fn convex_corner_needs_no_subdivision() {
        // own = +y (CCW-last), next = +x (CCW-first): the sector sweeps
        // +x → +y CCW around +z, so (b̂ × â)·n = (x̂ × ŷ)·ẑ = +1.
        let s = shape(Vec3::new(0.0, 3.0, 0.0), Vec3::new(2.0, 0.0, 0.0), false).unwrap();
        assert!(s.bisector.is_none());
        assert_eq!(s.arm, 2.0);
        assert_dir(s.unit_own, Vec3::new(0.0, 1.0, 0.0));
        assert_dir(s.unit_next, Vec3::new(1.0, 0.0, 0.0));
    }
    /// A definitely reflex corner (θ = 270°) subdivides at
    /// `−normalize(â + b̂)`, which points INTO the reflex span.
    #[test]
    fn reflex_corner_subdivides_at_the_reflex_bisector() {
        // own = +x, next = +y: (ŷ × x̂)·ẑ = −1 ⇒ reflex.
        let s = shape(Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), false).unwrap();
        let b = s.bisector.expect("a reflex corner subdivides");
        assert!(bits_eq(b, -(Vec3::new(1.0, 1.0, 0.0).normalize())));
        // The sector sweeps +y → +x the LONG way; the bisector is
        // strictly interior to it, i.e. on the far side from (+1, +1).
        assert!(b.dot(Vec3::new(-1.0, -1.0, 0.0)) > 0.0);
    }
    /// A straight corner (θ = 180°, where `â + b̂` collapses) falls to
    /// the cosine rung and subdivides 90° into the interior.
    #[test]
    fn straight_corner_subdivides_ninety_degrees_in() {
        let s = shape(Vec3::new(1.0, 0.0, 0.0), Vec3::new(-1.0, 0.0, 0.0), false).unwrap();
        // n × b̂ = ẑ × (−x̂) = −ŷ.
        assert_dir(
            s.bisector.expect("a straight corner subdivides"),
            Vec3::new(0.0, -1.0, 0.0),
        );
    }
    /// A spike between two DISTINCT edges (θ ≈ 0) refuses, named by the
    /// cosine rung — it is ill-conditioned, not a full circle.
    #[test]
    fn spike_between_distinct_edges_refuses_named() {
        let e = shape(Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), false)
            .expect_err("a spike has no valid interior direction");
        let SectorFault::Rung { diag: e, .. } = e else {
            panic!("a spike is a rung refusal, not a chord-length one: {e:?}");
        };
        assert_eq!(e.predicate, Some("sector_straight"));
        assert_eq!(
            (
                e.margin.diagnostic_f64_for_error_text(),
                e.margin.rejected_sign()
            ),
            (ErrorTextReading::Value(1.0), Some(Sign::Positive)),
            "the decided cos θ · arm"
        );
        assert_eq!(e.band, band());
    }
    /// **Rung 0, the other end: a chord whose length UNDERFLOWED.**
    /// Components below ~1e-162 square to zero, so `norm` is exactly
    /// zero for a chord that names its direction perfectly well.
    ///
    /// **Measured on this body without the underflow arm, all eight
    /// shapes** — four chord pairs × `full_circle`. Every row was
    /// executed: all eight refused identically, as
    /// `Rung(Indeterminate { margin: Invalid, band, predicate:
    /// Some("sector_arm") })` — the arm gate's refusal, which says the
    /// arm is non-positive and offers the
    /// band. Both halves are false of the input: the chord is not
    /// short, it is unmeasurable, and no band reaches a squared norm
    /// of zero.
    ///
    /// **`min` hides this end HARDER than the overflow end.** There,
    /// `min(3.0, ∞) = 3.0` let a healthy-looking arm through to rung
    /// 3, where two of eight shapes returned `Ok`. Here
    /// `min(3.0, 0.0) = 0.0`, so the underflowed chord always WINS the
    /// `min` and the refusal is never silent — but it is never about
    /// the chord either, and the arm it names is the good chord's
    /// neighbour rather than the one that failed. Asking each chord
    /// separately is what recovers the cause; asking the arm could not,
    /// because the arm is a `min` of two chords and has no witness of
    /// its own.
    ///
    /// `full_circle` is carried through every row for the same reason
    /// the overflow rows carry it: it is the flag that separated silent
    /// from loud there, and a row that dropped it would be testing less
    /// than its sibling.
    #[test]
    fn a_chord_whose_length_underflowed_refuses_at_rung_zero() {
        let tiny = 1e-200;
        // The premise: the norm flushed, and the direction survives in
        // the witness the underflow question is asked against.
        assert_eq!(Vec3::new(0.0, tiny, 0.0).norm(), 0.0);
        assert_eq!(Vec3::new(0.0, tiny, 0.0).norm_witness(), tiny);
        for full_circle in [false, true] {
            for (own, next) in [
                (Vec3::new(0.0, tiny, 0.0), Vec3::new(0.0, tiny, 0.0)),
                (Vec3::new(0.0, 3.0, 0.0), Vec3::new(tiny, 0.0, 0.0)),
                (Vec3::new(tiny, 0.0, 0.0), Vec3::new(0.0, 3.0, 0.0)),
                (Vec3::new(tiny, tiny, tiny), Vec3::new(0.0, 3.0, 0.0)),
            ] {
                assert_eq!(
                    shape(own, next, full_circle).err(),
                    Some(SectorFault::UnderflowedChord),
                    "{own:?} / {next:?} / full_circle={full_circle}"
                );
            }
        }
        // A chord that is genuinely SHORT rather than unmeasurable is
        // still the arm rung's, not this one's: 1e-12 squares to 1e-24,
        // which the format holds.
        assert_eq!(
            gate_refusal(shape(Vec3::new(0.0, 1e-12, 0.0), Vec3::new(0.0, 3.0, 0.0), false).err()),
            Some((
                SectorRung::Arm,
                Some(SECTOR_ARM),
                ErrorTextReading::Value(1e-12),
                Some(Sign::Zero)
            ))
        );
        // And the zero chord is not an underflowed one: it has no
        // direction to recover, so its witness is zero too and the
        // predicate's two ratios are both poison.
        assert_eq!(
            gate_refusal(shape(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 3.0, 0.0), false).err()),
            Some((
                SectorRung::Arm,
                Some(SECTOR_ARM),
                ErrorTextReading::Value(0.0),
                Some(Sign::Zero)
            ))
        );
    }
    /// **Rung 0: a chord whose length is not a NUMBER.** A chord past
    /// `Vec3::norm`'s ~1e154 overflow band has an infinite norm,
    /// which is maximally definite to the arm rung.
    ///
    /// **Measured on this body at the merge base (`5fbdda206`), all
    /// EIGHT shapes** — four chord pairs × `full_circle`. Every line
    /// below was executed, not argued:
    ///
    /// | chords (own / next) | `full_circle` clear | `full_circle` set |
    /// |---|---|---|
    /// | `1e200` / `1e200` | rung 3 escalation | rung 3 escalation |
    /// | finite / `1e200` | rung 3 spike refusal | **`Ok`, bisector `(0,0,0)`** |
    /// | `1e200` / finite | rung 3 spike refusal | **`Ok`, bisector `(-1,0,0)`** |
    /// | `NaN` / finite | rung 1 escalation | rung 1 escalation |
    ///
    /// **Two shapes returned `Ok`, not one, and the second is the
    /// worse of the pair.** With `full_circle` set and ONE chord
    /// overflowed, the door reported success either way round:
    ///
    /// - own finite, next overflowed — `arm: 3.0, unit_own: (0,1,0),
    ///   unit_next: (0,0,0), bisector: Some((0,0,0))`. A subdivision
    ///   direction of nothing.
    /// - own overflowed, next finite — `arm: 3.0, unit_own: (0,0,0),
    ///   unit_next: (0,1,0), bisector: Some((-1,0,0))`. The bisector
    ///   is `n × unit_next`, built from the chord that SURVIVED, so
    ///   it is a unit vector in a plausible direction with nothing
    ///   about it to smell. The all-zero bisector above at least
    ///   looks wrong to a downstream reader; this one does not.
    ///
    /// **Why `min` hides exactly one of the two ends.** [`Real::min`]
    /// propagates `NaN` but not infinity, so `min(3.0, ∞) = 3.0`
    /// (the arm looks healthy and rung 1 passes) while
    /// `min(3.0, NaN) = NaN` (rung 1 escalates immediately). That is
    /// the whole of the split above: the OVERFLOW end is hidden by
    /// `min` and reaches rung 3 or slips through, the POISON end
    /// never gets past rung 1. It is also why rung 0 asks each chord
    /// separately rather than asking the arm — asking the arm would
    /// have left both silent shapes open.
    ///
    /// The six loud shapes were loud in two different ways, neither
    /// of which names the chord or a recourse that could work: the
    /// two `NaN` rows and the two `1e200`/`1e200` rows are `decide`
    /// ESCALATIONS (a poisoned margin, `MarginKind::Invalid`), while
    /// the two single-overflow rows with `full_circle` clear are this
    /// straightness gate's spike refusal. Those two OUTCOMES were
    /// indistinguishable by value — both carried `MarginKind::Invalid`
    /// under the rung's name.
    ///
    /// The `full_circle` flag is carried through every row because it
    /// is what separates the silent shapes from the loud ones: a row
    /// that only tested the loud shapes would have gone green on the
    /// merge base.
    #[test]
    fn a_chord_with_no_finite_length_refuses_at_rung_zero() {
        let big = 1e200;
        for full_circle in [false, true] {
            for (own, next) in [
                (Vec3::new(big, 0.0, 0.0), Vec3::new(big, 0.0, 0.0)),
                (Vec3::new(0.0, 3.0, 0.0), Vec3::new(big, 0.0, 0.0)),
                (Vec3::new(big, 0.0, 0.0), Vec3::new(0.0, 3.0, 0.0)),
                (Vec3::new(f64::NAN, 0.0, 0.0), Vec3::new(0.0, 3.0, 0.0)),
            ] {
                assert_eq!(
                    shape(own, next, full_circle).err(),
                    Some(SectorFault::NonFiniteChord),
                    "{own:?} / {next:?} / full_circle={full_circle}"
                );
            }
        }
        // A finite pair still climbs the rungs — the row above cannot
        // be passing because rung 0 refuses everything.
        assert!(shape(Vec3::new(0.0, 3.0, 0.0), Vec3::new(2.0, 0.0, 0.0), false).is_ok());
    }
    /// The SAME reading on a one-edge orbit is the legitimate strut
    /// full-circle sector and subdivides instead of refusing — the row
    /// that separates the two, so the spike row above cannot be passing
    /// for the wrong reason.
    #[test]
    fn strut_full_circle_subdivides() {
        let s = shape(Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), true).unwrap();
        assert_dir(
            s.bisector.expect("a strut vertex subdivides"),
            Vec3::new(0.0, 1.0, 0.0),
        );
    }
    /// A collapsed bounding chord fails the arm rung, named.
    #[test]
    fn degenerate_arm_refuses_named() {
        let e = shape(Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), false)
            .expect_err("a collapsed chord cannot meter the corner");
        // Literal pins, for the reason spelled out on the spike row.
        let SectorFault::Rung { diag: e, .. } = e else {
            panic!("a collapsed chord has a finite length: {e:?}");
        };
        assert_eq!(e.predicate, Some("sector_arm"));
        assert_eq!(
            (
                e.margin.diagnostic_f64_for_error_text(),
                e.margin.rejected_sign()
            ),
            (ErrorTextReading::Value(0.0), Some(Sign::Zero)),
            "the decided arm"
        );
        assert_eq!(e.band, band());
    }
    #[test]
    fn a_gate_refusal_is_the_funnels_escalation() {
        for (own, next, rung) in [
            (
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                SECTOR_ARM,
            ),
            (
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                SECTOR_STRAIGHT,
            ),
        ] {
            let bracket = geom_core::k_stats::Bracket::open();
            let got = shape(own, next, false);
            let recorded = bracket.finish();
            let Err(SectorFault::Rung { diag, .. }) = got else {
                panic!("{rung}: a gate refusal, not {got:?}");
            };
            assert_eq!(diag.predicate, Some(rung), "{rung}: named by its rung");
            let logged: Vec<Indeterminate> =
                recorded.escalations.iter().map(|e| e.source).collect();
            assert_eq!(
                logged,
                vec![diag],
                "{rung}: the refusal is the logged escalation"
            );
        }
    }

    /// At bounds along one ray, the full turn at a slit's tip and a
    /// spike between distinct edges: the turn starts at `next`, its
    /// quarters run counter-clockwise about the normal from it, and the
    /// two orders list the same three cuts reversed.
    #[test]
    fn bounds_along_one_ray_are_a_full_turn_only_at_a_slit_tip() {
        let n = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true);
        let ray = Vec3::new(2.0, 0.0, 0.0);
        let read = |bounds| corner_shape(ray, ray, n, bounds, band());
        assert!(
            matches!(
                read(CornerBounds::Distinct),
                Err(SectorFault::Rung {
                    rung: SectorRung::Straight { full_circle: false },
                    ..
                })
            ),
            "between distinct edges the reading is a spike, refused"
        );
        let Ok(CornerShape::FullTurn(turn)) = read(CornerBounds::SlitTip) else {
            panic!("at a slit's tip the reading is the full turn");
        };
        assert_eq!(turn.arm, 2.0, "the arm is the shorter bound's length");
        let from_next = turn.quarters(TurnOrder::FromNext);
        for (got, want) in from_next.iter().zip([
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
        ]) {
            assert_dir(*got, want);
        }
        let mut from_own = turn.quarters(TurnOrder::FromOwn);
        from_own.reverse();
        assert!(
            from_own
                .iter()
                .zip(&from_next)
                .all(|(&a, &b)| bits_eq(a, b)),
            "the orbit order lists the same quarters reversed"
        );
        let mut fractions = TurnOrder::FromOwn.quarter_fractions();
        fractions.reverse();
        assert_eq!(fractions, TurnOrder::FromNext.quarter_fractions());
    }

    /// **The anti-re-fork row, for this crate's sources.** The three
    /// rung names are spelled, in quotes, in this module and in no other
    /// file under `geom-brep/src`, and the six per-reader names they
    /// pooled (`bool_sector_*`, `split_sector_*`, `docs/K-REPORT.md`'s
    /// census note) are spelled nowhere. A crate that reads these rungs
    /// guards its own sources the same way (`topo::sector_shape`'s row).
    ///
    /// Both rosters are built from the rung words rather than written
    /// out, so this file is read like any other. The retired set is
    /// therefore tied to the current rung words: a rung renamed would
    /// stop guarding its old retired spellings, and whoever renames one
    /// carries them forward by hand.
    ///
    /// The check is textual over the quoted forms, so a doc comment
    /// that quotes a retired name reds it too; backticks do not.
    #[test]
    fn the_rungs_are_decided_in_one_place() {
        let (pooled, retired) = rung_rosters();
        let src = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("src");
        let home = src.join("sector_shape.rs");
        let files = test_utils::source::rust_sources(&src);
        assert!(
            files.contains(&home),
            "the walk found {} file(s) but not the home module",
            files.len()
        );
        for path in &files {
            let text = std::fs::read_to_string(path).expect("a readable source file");
            for name in &retired {
                assert!(
                    !text.contains(name.as_str()),
                    "{} spells the retired per-reader sector predicate {name}",
                    path.display()
                );
            }
            if path == &home {
                continue;
            }
            for name in &pooled {
                assert!(
                    !text.contains(name.as_str()),
                    "{} spells the sector-shape predicate {name}: the rungs are decided \
                     only in `sector_shape`",
                    path.display()
                );
            }
        }
        let here = std::fs::read_to_string(&home).expect("the home module is readable");
        for name in &pooled {
            assert!(
                here.contains(name.as_str()),
                "{name} is spelled in the home"
            );
        }
    }

    /// The three pooled names and the six retired ones, each with its
    /// quotes, assembled from the rung words.
    fn rung_rosters() -> (Vec<String>, Vec<String>) {
        let rungs = ["arm", "reflex", "straight"];
        let pooled = rungs.iter().map(|r| format!("\"sector_{r}\"")).collect();
        let retired = ["bool", "split"]
            .iter()
            .flat_map(|lane| rungs.iter().map(move |r| format!("\"{lane}_sector_{r}\"")))
            .collect();
        (pooled, retired)
    }
}
