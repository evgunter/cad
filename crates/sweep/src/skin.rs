//! §10.3 skinned (lofted) and §10.4 swept NURBS surfaces — the
//! DEFINITIONAL geometry behind the `Loft` and `Sweep` feature nodes
//! (M5 PR 10; C11, The NURBS Book ch. 10).
//!
//! # Q8, stated where the geometry is made
//!
//! **The produced NURBS *is* the definition.** The recipe (the
//! sections, the path, the degrees) is PROVENANCE: it says how the
//! surface was chosen, not what it approximates. There is no residual
//! obligation to some "true" swept locus, no approximating-surface
//! machinery, and no tolerance attached to the construction itself.
//! Only DERIVED items — intersections with these walls, pcurves of
//! non-iso edges — carry certificates, because only they make a claim
//! about something other than themselves.
//!
//! That is why §10.4's general sweep is honestly implemented as
//! *instantiate and skin*: placing rigid copies of the profile along
//! the path and skinning them does not approximate a swept surface,
//! it DEFINES one. The Book presents the same construction; we simply
//! decline to pretend the result is anything but itself.
//!
//! # Placement in this crate (M5 PR 10, numbered deviation 1)
//!
//! Not "constructions live here" — `geom` builds NURBS payloads too
//! (`geom::curves::fit`, which this module calls). The binding reason
//! is the layering: skinning reads a `profile::ValidatedProfile`
//! through a `SketchPlane` and a `geom_brep::SketchSegment`, so it
//! depends on two layers ABOVE `geom` and cannot live below them. Of
//! the crates that sit above both, `sweep` is the one whose subject
//! this is — §10.3/§10.4 are sweep operations by name. The import list
//! below is the check: nothing in it may be pushed down.
//!
//! The produced payload is `geom::NurbsSurface`, so nothing about the
//! surface type moved.
//!
//! # Structure selection is f64 (C6/D9)
//!
//! Degrees, knot multiplicities, the degree raise, the merged knots and
//! the parameters' order are chosen in `f64`, deterministically, on the
//! value channel of the data ([`SpanLocate::point_value`]): same
//! sections in, same structure out. An arc's sub-arc count and its turn
//! and endpoint branches are decided over the brackets they read, for
//! every member ([`SkinError::ArcSweepUndecided`]).
//! A positivity the structure asks of a value (a chord that is not
//! empty, a parameter step that ascends) holds for every value it
//! stands for ([`SpanLocate::structure_positive`]), so at an enclosure
//! scalar a value wider than a point passes where its whole bracket
//! does. Where the answer selects a branch rather than refusing (a
//! control row that abstains from the parameterization), a value whose
//! members would not all take one branch refuses
//! ([`SkinError::RowMotionUndecided`]).
//!
//! # The values are computed in the sections' scalar
//!
//! Every value is formed in the scalar the sections are held in, under
//! those decisions: an arc's weight `cos(Δ/2)` and its poles
//! ([`segment_curve`]), the compatible nets through the knot algebra
//! ([`make_compatible`]), the chord-length parameters
//! ([`skin_parameters`]), the averaged knots and the banded collocation
//! solve ([`skin_on`], `geom::curves::fit::interpolate_columns_in`). So
//! a section whose arc sweep is a function of a document's variables
//! gives a wall whose weights, knots and poles are functions of them
//! too, and at an enclosure scalar a section over a box gives walls
//! that enclose the box's family. The sines and cosines are `Real`'s,
//! the pure-Rust `libm` D9 prescribes.
//!
//! A carrier's weights and knots are points at every scalar for now, so
//! at an enclosure scalar an arc whose weight `cos(Δ/2)` is wider than a
//! point refuses at the carrier's door
//! (`SplineError::CarrierWeightNotAPoint`, until C6's S4/S5), as does an
//! interior averaged knot wider than a point (`CarrierKnotNotAPoint`:
//! the fit encloses over it, C6 S5c, and the wall's carrier refuses it
//! until S5z). An arc's sweep structure
//! (non-zero, its sub-arc count, a turn) is certified for every value
//! it stands for ([`SkinError::ArcSweepUndecided`]). The knot algebra
//! that makes sections compatible runs homogeneous at an enclosure
//! scalar ([`make_compatible`], C6 S4), and its result enters through
//! the same door. No topology decision is
//! made here, so nothing routes through `k_stats`; the raw `f64`
//! comparisons below are structure selection under C6.

use core::num::NonZeroUsize;
use std::sync::Arc;

use geom::NurbsCurve3;
use geom::NurbsSurface;
use geom::curves::fit::{FitError, interpolate_columns_in};
use geom_brep::SketchSegment;
use geom_core::Tol;
use geom_core::spline::{KnotAlgebraError, Knots, SpanLocate, SplineError};
use geom_core::{
    Affine3, Arc2, Band, BandError, COINCIDENCE_RECOURSE, Decide, Indeterminate, MarginDiag,
    Point2, Point3, RANGE_RECOURSE, Real, RigidMotion, TurnError, Turns, UnitVec3, UnitVec3Error,
};
use profile::{
    CanonicalStructure, ConstructedLoop, ConstructedProfile, Profile, ProfileError, ProfileLoop,
    SketchPlane, ValidatedProfile,
};

/// η, the slack above a quarter turn one sub-arc may take (C6's
/// sub-arc rule): the count's breakpoints sit at `k·(π/2)(1 + η)`, off
/// the multiples of π/2 where documents' sweeps sit, so an arc whose
/// true sweep is `k·π/2` counts `k` for every value it stands for.
/// `1/16` is exact, keeps `k·π/2` at count `k` for every `k ≤ 4` (a
/// full turn: it needs `4/(1 + η) > 3`), and keeps a sub-arc's
/// tangent-intersection point well conditioned.
const SUB_ARC_ETA: f64 = 1.0 / 16.0;

/// The ceiling on one rational-quadratic arc span, `(π/2)(1 + η)`: every
/// sub-arc of a converted profile arc is at most this wide, so the
/// tangent-intersection control point stays finite and well
/// conditioned (`cos(Δ/2) ≥ cos((π/4)(1 + η))`).
const MAX_SUB_ARC: f64 = core::f64::consts::FRAC_PI_2 * (1.0 + SUB_ARC_ETA);

/// Which of an arc's sweep decisions [`SkinError::ArcSweepUndecided`]
/// names: each is structure read off the sweep, certified for every
/// value it stands for (C6).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArcSweepDecision {
    /// That the sweep is not zero.
    NonZero,
    /// How many sub-arcs it takes: one sub-arc reaches `(π/2)(1 + η)`.
    SubArcCount,
    /// That an arc between two points stops short of a full turn.
    TurnBetweenPoints,
    /// That an arc back to its own start is one full turn, nearer one
    /// than none or two.
    TurnToItself,
    /// Whether the arc ends where it starts (its endpoints' distance
    /// is zero) or between two points: the branch the two turn checks
    /// above select between.
    Endpoints,
}

impl core::fmt::Display for ArcSweepDecision {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::NonZero => "whether its sweep is zero",
            Self::SubArcCount => "how many sub-arcs its sweep takes",
            Self::TurnBetweenPoints => "whether its sweep stops short of a full turn",
            Self::TurnToItself => "whether its sweep is one full turn",
            Self::Endpoints => "whether it ends where it starts",
        })
    }
}

/// Which of a path sweep's decisions [`SkinError::PathUndecided`]
/// names ([`sweep_places`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PathDecision {
    /// Whether the path's tangent at the station vanishes.
    Tangent,
    /// Whether the station coincides with the one before it.
    Chord,
    /// Whether the path turns a half turn between the station and the
    /// one before it.
    HalfTurn,
}

impl core::fmt::Display for PathDecision {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::Tangent => "whether the path's tangent vanishes at station",
            Self::Chord => "whether the path stands still into station",
            Self::HalfTurn => "whether the path turns a half turn into station",
        })
    }
}

/// Typed refusal of the §10.3/§10.4 constructions (closed enum, D4 ¶3).
///
/// Every arm but the path sweep's is a STRUCTURAL statement about the
/// inputs — section counts, loop counts, degrees, weights,
/// solvability. Structure selection is exact-`f64` (C6), so none of
/// those has an in-band twin; [`SkinError::DegenerateSection`]
/// documents where the banded half of its user situation actually
/// lives and carries that door's recourse. The path sweep decides its
/// station tangents, chords and half turns under the band
/// ([`sweep_places`]), so [`SkinError::PathTangentVanishes`],
/// [`SkinError::PathStationsCoincide`] and [`SkinError::PathHalfTurn`]
/// have their in-band twin, [`SkinError::PathUndecided`].
#[derive(Clone, Debug, PartialEq)]
pub enum SkinError {
    /// Fewer sections than a skin needs (≥ 2 always; ≥ `degree + 1`
    /// for the requested v-degree).
    TooFewSections {
        /// Sections supplied.
        have: usize,
        /// Sections required.
        need: usize,
    },
    /// The sections do not describe the same thing: their loop counts,
    /// or one loop's segment counts, differ. A skin matches segment
    /// `j` of every section to segment `j` of every other; there is no
    /// honest way to guess a correspondence that was not given.
    SectionShapeMismatch {
        /// The section that disagrees with section 0.
        section: usize,
        /// What section 0 had.
        expected: usize,
        /// What this section had.
        found: usize,
        /// Which count disagreed (`"loops"` or `"segments"`).
        what: &'static str,
    },
    /// A section failed the profile crate's validation door — the
    /// same gate every extruded/revolved profile passes (one
    /// profile vocabulary for all four body ops, so a section that
    /// would not extrude does not skin either). Open chains are not a
    /// case this door can meet: [`ProfileLoop`] closes by
    /// construction, so the former open/closed-mixed refusal has
    /// nothing left to refuse and the mixed skin is unrepresentable.
    SectionProfile {
        /// The offending section.
        section: usize,
        /// The profile door's typed refusal.
        source: ProfileError,
    },
    /// A section curve does not live on the unit parameter domain the
    /// compatibility pass requires (`[0, 1]`, clamped).
    DomainNotUnit {
        /// The offending section.
        section: usize,
        /// Its domain.
        domain: (f64, f64),
    },
    /// A section, or the path, is degenerate: a zero-length chord or a
    /// zero-radius arc.
    ///
    /// **Where the in-band twin lives** (M5 PR 10 fix pass, review
    /// MIN-1): not here. This module decides no section's degeneracy —
    /// every comparison about a section is exact-`f64` structure
    /// selection (C6), so there is no `Escalated` arm to pair with, and
    /// inventing one would advertise a refusal nothing can produce. The banded half
    /// of the user's situation sits one layer UP, in
    /// `profile::validate`'s `vertex_separation` /
    /// `segment_straightness` / `arc_diameter_clearance` predicates,
    /// which every section passes before it reaches this module. This
    /// arm catches the exactly-zero residue those doors let through by
    /// construction, and it names the SAME recourse they do, so both
    /// halves of one user situation still read alike across the layer
    /// boundary (D4 ¶1's intent, honoured where the decision actually
    /// is).
    DegenerateSection {
        /// Where it sits.
        section: usize,
        /// What was degenerate.
        what: &'static str,
    },
    /// The chord-length v-parameterization takes no `f64` step from
    /// section `section − 1` to section `section`: the control rows it
    /// is measured on do not move between them, or move by less than
    /// `f64` resolves against their whole travel. A structure question
    /// about the parameters, decided exactly (C6), and NOT a verdict
    /// that the sections coincide — whether two sections are apart is
    /// a banded question this module has no band for, and the loft
    /// asks it before skinning (`LoftError::DegenerateStacking`).
    NoParameterStep {
        /// The second section of the stepless pair.
        section: usize,
    },
    /// The requested v-degree is 0, or ≥ the section count.
    BadDegree {
        /// The requested degree.
        degree: usize,
        /// The sections available.
        sections: usize,
    },
    /// The path's tangent at a station has no direction: its length
    /// is not finite and decided positive ([`sweep_places`]).
    PathTangentVanishes {
        /// The station where the tangent vanished.
        station: usize,
    },
    /// The path's station coincides with the one before it: the chord
    /// between them is decided zero, so no reflection carries the
    /// frame across it ([`sweep_places`]).
    PathStationsCoincide {
        /// The later station of the pair.
        station: usize,
    },
    /// The path turns a half turn between a station and the one before
    /// it: the earlier tangent mirrored in their chord is decided
    /// opposed to the later one, so the step's second reflection has no
    /// plane ([`sweep_places`]).
    PathHalfTurn {
        /// The later station of the pair.
        station: usize,
    },
    /// One of the path sweep's decisions at a station could not be
    /// taken under the band ([`sweep_places`]).
    PathUndecided {
        /// The station where the decision escalated.
        station: usize,
        /// Which decision.
        which: PathDecision,
        /// The decision's own diagnostic.
        source: Indeterminate,
    },
    /// A station's placement is not a finite number: the start frame
    /// carried along the path overflowed.
    StationNotFinite {
        /// The station whose placement overflowed.
        station: usize,
    },
    /// The tolerance gave no band for the path sweep's decisions.
    Band(BandError),
    /// Whether control row `row` moves between the sections is not
    /// the same for every value they stand for: its chord-length total
    /// is wider than a point and reaches zero. A row that never moves
    /// abstains from the parameterization, so the members would not
    /// share one, and the skin refuses rather than stand for those
    /// whose row moves.
    RowMotionUndecided {
        /// The control row, counted down the rows the
        /// parameterization reads.
        row: usize,
    },
    /// Section `section` has an arc whose structure is not decided
    /// over the values it stands for (C6): the bracket a decision reads
    /// (`which` names it; the sweep's, or for [`ArcSweepDecision::Endpoints`]
    /// the endpoints' distance) straddles that decision's breakpoint.
    /// The margin is the bracket's distance to the breakpoint, as read;
    /// inputs narrow enough that it clears zero lift the refusal. A
    /// bracket may be wide from rounding alone: a constant sweep of
    /// exactly a full turn between two points reads `T::tau()`'s own
    /// enclosure, which no narrower input clears.
    ArcSweepUndecided {
        /// The section.
        section: usize,
        /// The decision.
        which: ArcSweepDecision,
        /// The sweep's distance to the breakpoint, as read.
        margin: MarginDiag,
    },
    /// The v-direction interpolation refused.
    Fit(FitError),
    /// A knot-algebra operation (degree elevation, refinement) refused.
    KnotAlgebra(KnotAlgebraError),
    /// The produced spline structure refused validated construction —
    /// notably a non-positive interpolated weight, which would break
    /// the convex-hull invariant every C9 bound stands on.
    Structure(SplineError),
}

/// The recourse a path sweep's vanishing tangent ends with.
const PATH_RECOURSE: &str = "smooth the path there";

/// The recourse a path sweep's half turn between two stations ends
/// with: closer stations turn less between them.
const HALF_TURN_RECOURSE: &str = "add stations";

/// The recourse a path sweep's coincident stations end with.
const COINCIDENT_STATIONS_RECOURSE: &str = "change the station count so consecutive stations \
                                            are apart, or smooth the path there";

/// The recourse a path sweep's undecided decision ends with.
const PATH_UNDECIDED_RECOURSE: &str = "add stations, or smooth the path there";

impl core::fmt::Display for SkinError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::TooFewSections { have, need } => write!(
                f,
                "{have} sections were given and at least {need} are needed. Recourse: supply \
                 more sections"
            ),
            Self::SectionShapeMismatch {
                section,
                expected,
                found,
                what,
            } => write!(
                f,
                "section {section} has {found} {what} and section 0 has {expected}, but \
                 sections are matched by index. Recourse: supply sections with the same shape"
            ),
            Self::SectionProfile { section, source } => {
                write!(f, "section {section} is not a valid profile: {source}")
            }
            Self::DomainNotUnit { section, domain } => write!(
                f,
                "section {section} lives on [{}, {}], not the unit domain the sections are \
                 matched on",
                domain.0, domain.1
            ),
            Self::DegenerateSection { section, what } => write!(
                f,
                "section {section} is degenerate ({what}). Recourse: {COINCIDENCE_RECOURSE}"
            ),
            Self::NoParameterStep { section } => write!(
                f,
                "the skin's chord-length parameterization takes no step from section {} to \
                 section {section}: the curves it is measured on do not move between them by \
                 anything f64 resolves against their whole travel. Recourse: move section \
                 {section} away from section {}",
                section.saturating_sub(1),
                section.saturating_sub(1),
            ),
            Self::BadDegree { degree, sections } => write!(
                f,
                "degree {degree} is not usable for {sections} sections (it must be at least 1 \
                 and below the section count)"
            ),
            Self::PathTangentVanishes { station } => write!(
                f,
                "the path's tangent vanishes at station {station}, so no rigid frame carries \
                 the profile through it. Recourse: {PATH_RECOURSE}"
            ),
            Self::PathStationsCoincide { station } => write!(
                f,
                "the path's stations {} and {station} coincide, so no chord carries the frame \
                 between them. Recourse: {COINCIDENT_STATIONS_RECOURSE}",
                station.saturating_sub(1),
            ),
            Self::PathHalfTurn { station } => write!(
                f,
                "the path turns a half turn between stations {} and {station}, so no \
                 rotation-minimizing step carries the frame across it. Recourse: \
                 {HALF_TURN_RECOURSE}",
                station.saturating_sub(1),
            ),
            Self::PathUndecided {
                station,
                which,
                source,
            } => source
                .undecided(
                    format_args!("{which} {station}"),
                    source.lever_ending(PATH_UNDECIDED_RECOURSE),
                )
                .fmt(f),
            Self::StationNotFinite { station } => write!(
                f,
                "the profile's placement at station {station} is not a finite number. \
                 Recourse: {RANGE_RECOURSE}"
            ),
            Self::Band(e) => write!(f, "{e}"),
            Self::RowMotionUndecided { row } => write!(
                f,
                "whether control row {row} moves between the sections is not decided over the \
                 values they stand for, so the skin's parameterization is not one for them \
                 all. Recourse: narrow the range, or evaluate the loft at a point scalar"
            ),
            Self::ArcSweepUndecided {
                section,
                which,
                margin,
            } => write!(
                f,
                "section {section} has an arc that leaves {which} undecided over the values it \
                 stands for: what it reads is {margin} from the breakpoint. Recourse: narrow the \
                 range of that sweep or those endpoints until it clears the breakpoint; the width \
                 may be rounding alone (a constant sweep of exactly a full turn), and then \
                 evaluate at a point scalar"
            ),
            Self::Fit(e) => write!(f, "{e}"),
            Self::KnotAlgebra(e) => write!(f, "{e}"),
            Self::Structure(e) => write!(f, "{e}"),
        }
    }
}

impl core::error::Error for SkinError {}

impl From<KnotAlgebraError> for SkinError {
    fn from(e: KnotAlgebraError) -> Self {
        Self::KnotAlgebra(e)
    }
}

impl From<SplineError> for SkinError {
    fn from(e: SplineError) -> Self {
        Self::Structure(e)
    }
}

impl From<FitError> for SkinError {
    fn from(e: FitError) -> Self {
        Self::Fit(e)
    }
}

// ---------------------------------------------------------------------
// Analytic → NURBS: the section curves
// ---------------------------------------------------------------------

/// One profile segment as a NURBS curve in WORLD space, exactly.
///
/// - **Line** → degree 1, two control points, unit weights: the chord,
///   with the segment's own affine parameter.
/// - **Arc** → the standard rational-quadratic circular representation
///   (Book §7.5 / A7.1), split into `⌈|θ| / ((π/2)(1+η))⌉` equal
///   sub-arcs, η = 1/16 (`MAX_SUB_ARC`), so every tangent-intersection
///   control point stays finite and the interior weights `cos(Δ/2)`
///   stay ≥ `cos((π/4)(1+η))`. Exact in ℝ: the
///   locus IS the carrier circle, not a fit of it. The circle is the
///   one [`SketchSegment::eval`] describes — `a` turned about the
///   centre through the sweep — so the control points are `a − centre`
///   rotated about the centre, and the stored radius is not read: the
///   curve and the evaluation certification meters cannot describe two
///   different circles.
///
/// The 2-D control points are mapped through `place` — an affine map,
/// under which a NURBS is exactly a NURBS with the same weights and
/// knots (control points transform, nothing else moves).
///
/// # Errors
///
/// [`SkinError::DegenerateSection`] for a line of zero length, a zero
/// or non-finite sweep, an arc whose start is not a finite, positive
/// distance from its centre, an arc between two points sweeping a full
/// turn or more, or an arc back to its own start that is not one full
/// turn (D1's full turn, one segment at one vertex, is the one that
/// converts); [`SkinError::ArcSweepUndecided`] for an arc whose sweep
/// or endpoints leave one of those decisions, or its sub-arc count,
/// undecided over the values they stand for;
/// [`SkinError::Structure`] if validated construction refuses
/// (`CarrierWeightNotAPoint` for a weight wider than a point, C6's
/// interim until S4/S5).
// `!(x > 0)` and `!(a < b)` are deliberate NaN-catching (the
// geom-core::spline::algebra note): a poisoned coordinate must take
// the refusal arm, not slip through a negated comparison.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
pub fn segment_curve<T: SpanLocate>(
    section: usize,
    seg: SketchSegment<T>,
    place: Affine3<T>,
) -> Result<NurbsCurve3<T>, SkinError> {
    let degenerate = |what: &'static str| SkinError::DegenerateSection { section, what };
    let world = |p: Point2<T>| place.transform_point(Point3::new(p.x, p.y, T::zero()));
    let (two, three) = (
        NonZeroUsize::MIN.saturating_add(1),
        NonZeroUsize::MIN.saturating_add(2),
    );
    match seg {
        SketchSegment::Line { a, b } => {
            if !a.distance(b).structure_positive() {
                return Err(degenerate("zero-length chord"));
            }
            let knots = Knots::new(vec![(T::zero(), two), (T::one(), two)], 1)?;
            Ok(NurbsCurve3::new(
                knots,
                vec![world(a), world(b)],
                vec![T::one(), T::one()],
            )?)
        }
        SketchSegment::Arc { a, b, arc } => {
            let Arc2 {
                centre,
                sweep: theta,
                ..
            } = arc;
            // The arc's structure is decided for every value it stands
            // for (C6), over |θ|'s bracket: that it is not zero, how many
            // sub-arcs it takes, and that it is short of a turn (an arc
            // between two points) or one turn (an arc back to its start);
            // and over the endpoints' distance, which of those two it is.
            // A bracket that straddles a breakpoint refuses typed, with
            // the margin a narrower box lifts.
            let undecided = |which, lo: f64, hi: f64| SkinError::ArcSweepUndecided {
                section,
                which,
                margin: MarginDiag::enclosure(lo, hi),
            };
            let Some((lo, hi)) = theta.abs().structure_bracket() else {
                return Err(degenerate("zero or non-finite sweep"));
            };
            // A sweep with no finite member, or none but zero, is
            // degenerate for every member; one some of whose members are
            // finite and others not straddles every count's breakpoint
            // and refuses below as the count undecided.
            if !lo.is_finite() || hi == 0.0 {
                return Err(degenerate("zero or non-finite sweep"));
            }
            if lo == 0.0 {
                return Err(undecided(ArcSweepDecision::NonZero, lo, hi));
            }
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let count = |x: f64| ((x / MAX_SUB_ARC).ceil() as usize).max(1);
            let m = count(lo);
            if count(hi) != m {
                #[allow(clippy::cast_precision_loss)]
                let at = m as f64 * MAX_SUB_ARC;
                return Err(undecided(ArcSweepDecision::SubArcCount, lo - at, hi - at));
            }
            #[allow(clippy::cast_precision_loss)]
            let steps = m as f64;
            let delta = theta / T::from_f64(steps);
            let half = T::from_f64(0.5);
            // The weight `cos(Δ/2)` is held in `T`; where it is no point
            // (at an enclosure scalar, by rounding alone), the carrier's
            // door refuses it (`CarrierWeightNotAPoint`) until C6's S4/S5.
            let (_, half_w) = (delta * half).sin_cos();
            // An arc that ends where it starts lands there after whole
            // turns, and converts only as ONE (D1's full turn): its
            // sweep is nearer one turn than none or two. An arc between
            // two points stops short of a turn. Both read the true 2π's
            // bracket.
            let Some((tau_lo, tau_hi)) = T::tau().structure_bracket() else {
                unreachable!("τ is a number at every scalar")
            };
            let chord = a.distance(b);
            let between = chord.structure_positive();
            if !between
                && let Some((chord_lo, chord_hi)) = chord.structure_bracket()
                && chord_hi > 0.0
            {
                return Err(undecided(ArcSweepDecision::Endpoints, chord_lo, chord_hi));
            }
            if between {
                if lo >= tau_hi {
                    return Err(degenerate("a full turn or more between two points"));
                }
                if hi >= tau_lo {
                    return Err(undecided(
                        ArcSweepDecision::TurnBetweenPoints,
                        lo - tau_lo,
                        hi - tau_lo,
                    ));
                }
            } else {
                let (half_lo, half_hi) = (0.5 * tau_lo, 0.5 * tau_hi);
                let (three_lo, three_hi) = (1.5 * tau_lo, 1.5 * tau_hi);
                if hi <= half_lo || lo >= three_hi {
                    return Err(degenerate(
                        "an arc from a point back to itself that is not one full turn",
                    ));
                }
                if lo <= half_hi {
                    return Err(undecided(
                        ArcSweepDecision::TurnToItself,
                        lo - half_hi,
                        hi - half_hi,
                    ));
                }
                if hi >= three_lo {
                    return Err(undecided(
                        ArcSweepDecision::TurnToItself,
                        lo - three_lo,
                        hi - three_lo,
                    ));
                }
            }
            let spoke = a - centre;
            let rim = spoke.norm();
            if !rim.structure_positive() || !rim.is_bounded() {
                return Err(degenerate(
                    "arc start is not a finite, positive distance from its centre",
                ));
            }
            // The on-arc control points are the evaluation's own points
            // ([`Arc2::point_from`], `a` itself at the start); the
            // tangent-intersection points are the spoke `a − centre`
            // turned to mid-span about the centre and pushed out by
            // 1/cos(Δ/2).
            let tangent_point = |ang: T| {
                let (s, c) = ang.sin_cos();
                let (x, y) = (spoke.x / half_w, spoke.y / half_w);
                world(Point2::new(
                    centre.x + (x * c - y * s),
                    centre.y + (x * s + y * c),
                ))
            };
            let mut control = vec![world(a)];
            let mut weights = vec![T::one()];
            for k in 0..m {
                // The sub-arc's breakpoints `k/m` are structure: the
                // constants the knot vector below holds, and the ones
                // knot merging enters for the same breakpoint. One
                // spelling, [`geom_brep::sub_arc_joint`], which the
                // mesher's chord schedule shares to land on the joint.
                #[allow(clippy::cast_precision_loss)]
                let (base, next) = (
                    delta * T::from_f64(k as f64),
                    T::from_f64(geom_brep::sub_arc_joint(k + 1, m)),
                );
                control.push(tangent_point(base + delta * half));
                weights.push(half_w);
                control.push(world(arc.point_from(a, next)));
                weights.push(T::one());
            }
            let mut runs = vec![(T::zero(), three)];
            for k in 1..m {
                runs.push((T::from_f64(geom_brep::sub_arc_joint(k, m)), two));
            }
            runs.push((T::one(), three));
            Ok(NurbsCurve3::new(Knots::new(runs, 2)?, control, weights)?)
        }
    }
}

// ---------------------------------------------------------------------
// §5.3 + §5.5: compatibility
// ---------------------------------------------------------------------

/// Makes section curves COMPATIBLE (Book §10.3's precondition): one
/// common degree (§5.5 degree elevation) on one common knot vector
/// (§5.3 knot merging — refine each section by whatever the union
/// needs).
///
/// Both operations are **exact in ℝ**: they change the representation,
/// never the locus. Afterwards every section has the same degree, the
/// same knots, and therefore the same control-point count — which is
/// what makes a tensor-product skin possible at all.
/// In floating point, elevation agrees with the authored section to
/// about `1e-12` of its largest control coordinate up to degree 10 and
/// a few `1e-11` at degree 12, and surfaces no bound
/// (`geom_core::spline::algebra::elevate_plan`).
///
/// They are the knot algebra, at every scalar: projective at an
/// evaluation scalar, homogeneous at an enclosure scalar (C6 S4: the
/// net as `(w·P, w)`, no weight quotient formed). At an enclosure
/// scalar the result enters through the carrier's door, so a rational
/// section, whose refined weights are enclosures from rounding alone,
/// refuses there ([`SplineError::CarrierWeightNotAPoint`], C6's interim
/// until S5), while a polynomial one keeps its unit weights exactly and
/// is admitted.
///
/// # Errors
///
/// [`SkinError::TooFewSections`], [`SkinError::DomainNotUnit`] for a
/// section off the unit domain, and the propagated knot-algebra /
/// structure refusals.
pub fn make_compatible<T: SpanLocate>(
    sections: &[NurbsCurve3<T>],
) -> Result<Vec<NurbsCurve3<T>>, SkinError> {
    if sections.len() < 2 {
        return Err(SkinError::TooFewSections {
            have: sections.len(),
            need: 2,
        });
    }
    for (i, c) in sections.iter().enumerate() {
        let domain = c.domain();
        if domain.0 != 0.0 || domain.1 != 1.0 {
            return Err(SkinError::DomainNotUnit { section: i, domain });
        }
    }
    // §5.5: one common degree.
    let degree = sections.iter().map(NurbsCurve3::degree).max().unwrap_or(0);
    let elevated: Vec<NurbsCurve3<T>> = sections
        .iter()
        .map(|c| {
            let raise = degree - c.degree();
            if raise == 0 {
                Ok(c.clone())
            } else {
                c.elevate_degree(raise).map_err(SkinError::KnotAlgebra)
            }
        })
        .collect::<Result<_, _>>()?;
    // §5.3: one common knot vector — the union, at one degree and on
    // one domain by the two checks above.
    NurbsCurve3::refine_to_union(&elevated).map_err(SkinError::KnotAlgebra)
}

// ---------------------------------------------------------------------
// §10.3: the skinned surface
// ---------------------------------------------------------------------

/// The v-direction parameters Book Eq. 10.8 chooses for a set of
/// COMPATIBLE sections: chord lengths accumulated down each control
/// row, then **averaged across the rows** — one parameterization for
/// the whole surface, which is what keeps the u-columns' solves
/// consistent.
///
/// # Errors
///
/// [`SkinError::TooFewSections`], [`SkinError::SectionShapeMismatch`]
/// for sections that did not come out of [`make_compatible`], and
/// [`SkinError::NoParameterStep`] where the parameters do not strictly
/// ascend.
pub fn skin_parameters<T: SpanLocate>(sections: &[NurbsCurve3<T>]) -> Result<Vec<T>, SkinError> {
    let k = sections.len();
    if k < 2 {
        return Err(SkinError::TooFewSections { have: k, need: 2 });
    }
    let n = sections[0].control().len();
    // Every section must present the same control row count — the
    // caller's `make_compatible` obligation, checked rather than
    // assumed (an out-of-range index would be a panic, and this
    // entry is public).
    for (i, c) in sections.iter().enumerate().skip(1) {
        if c.control().len() != n {
            return Err(SkinError::SectionShapeMismatch {
                section: i,
                expected: n,
                found: c.control().len(),
                what: "compatible control points",
            });
        }
    }
    chord_length_parameters(k, (0..n).map(|i| control_row(sections, i)))
}

/// Control row `i` of compatible `sections`: the `i`-th control point
/// of each section, in section order.
fn control_row<T: SpanLocate>(sections: &[NurbsCurve3<T>], i: usize) -> Vec<Point3<T>> {
    sections.iter().map(|c| c.control()[i]).collect()
}

/// Book Eq. 10.8 over `rows`, each one control row of `k` points: the
/// chord lengths accumulated down the row, normalised to `[0, 1]`,
/// averaged across the rows. Each section's per-row shares are SORTED
/// before they are summed, so the answer is a function of the multiset
/// of rows, bit for bit — the order the rows arrive in cannot move it.
///
/// # Errors
///
/// [`SkinError::NoParameterStep`] when every row is pinned or the
/// parameters do not strictly ascend.
fn chord_length_parameters<T: SpanLocate>(
    k: usize,
    rows: impl IntoIterator<Item = Vec<Point3<T>>>,
) -> Result<Vec<T>, SkinError> {
    let mut shares: Vec<Vec<T>> = vec![Vec::new(); k];
    let mut counted = 0usize;
    for (index, row) in rows.into_iter().enumerate() {
        let chords: Vec<T> = row.windows(2).map(|w| w[0].distance(w[1])).collect();
        // Summed from `+0`: every chord is a length, `+0` or above, so
        // this is `f64`'s `Sum` (which starts at `-0`) bit for bit.
        let total = chords.iter().fold(T::zero(), |acc, c| acc + *c);
        // A row that never moves (a corner pinned across every
        // section) carries no parameterization information — it is not
        // an error, it simply abstains from the average. A row that
        // moves for some values its bracket stands for and not others
        // would abstain for some members only, so it refuses.
        if !total.structure_positive() {
            if total.point_value().is_none() && !total.is_poison() {
                return Err(SkinError::RowMotionUndecided { row: index });
            }
            continue;
        }
        counted += 1;
        let mut acc = T::zero();
        for (j, chord) in chords.iter().enumerate() {
            acc = acc + *chord;
            shares[j + 1].push(acc / total);
        }
    }
    if counted == 0 {
        return Err(SkinError::NoParameterStep { section: 1 });
    }
    #[allow(clippy::cast_precision_loss)]
    let denom = T::from_f64(counted as f64);
    // Each section's mean is taken as `min + Σ (sᵢ − min) / m` over its
    // shares sorted on the value channel: a function of the multiset,
    // and EXACT when the rows agree (evenly stacked identical sections
    // give every row the same share, and the mean must not round it to
    // a neighbour). Every `sᵢ − min` is `+0` or above, so the sum from
    // `+0` is `f64`'s `Sum` bit for bit.
    let mut params: Vec<T> = shares
        .iter_mut()
        .map(|s| {
            s.sort_by(|a, b| (*a).value_channel().total_cmp(&(*b).value_channel()));
            let lo = s.first().copied().unwrap_or_else(T::zero);
            lo + s.iter().fold(T::zero(), |acc, x| acc + (*x - lo)) / denom
        })
        .collect();
    // The averaged accumulation pins 0 and 1 in exact arithmetic; make
    // the ends EXACT so the averaged-knot construction's clamped
    // `0 → 1` precondition holds bit-for-bit (D9 structure selection).
    params[0] = T::zero();
    params[k - 1] = T::one();
    for j in 1..k {
        if !geom_core::spline::structure_ascends(params[j - 1], params[j]) {
            return Err(SkinError::NoParameterStep { section: j });
        }
    }
    Ok(params)
}

/// **§10.3, the skinned surface.** Interpolates COMPATIBLE section
/// curves v-directionally at `v_degree`, in homogeneous ℝ⁴ — the
/// Book's rational treatment: the weighted control points are the data,
/// one shared collocation solve carries every column, and the
/// projective divide happens once at the end.
///
/// # No synthesized weight channel
///
/// The homogeneous treatment above is the RATIONAL lane, and it is
/// chosen per control ROW: row `i` is the `i`-th control point of every
/// section, which is the net's `u`-index-`i` line and the iso line a
/// seam at that `u` reads. A row whose weight is the same constant `c`
/// in every section is **constant-weight**: it is interpolated in plain
/// Cartesian ℝ³ and every weight of its output row is exactly `c`.
/// `c = 1` is the **integral** case. Every other row takes the
/// homogeneous lane, so an input whose rows are all integral skins to an
/// integral surface.
///
/// The two lanes agree exactly in ℝ: with `w ≡ c` the homogeneous data
/// is `c` times the Cartesian data, and the weight's interpolant is the
/// constant control vector `c` by partition of unity (`Σⱼ Nⱼ(u) = 1` at
/// every parameter, so a right-hand side of `c`s has the vector of `c`s
/// as its exact solution); the `c` cancels in the projective divide.
/// They do NOT agree in `f64`: solving the weight by LU and dividing
/// every coordinate through by the result is a normalization ROUND-TRIP,
/// and it lands off `c` by an ulp for most parameterizations — measured
/// `1.0000000000000002` and `0.9999999999999998` on plain polyline
/// sections at unevenly spaced stations.
///
/// That ulp is a **change of kind**, not a small error. On an integral
/// wall it makes the wall bitwise rational: the seam carrier
/// `geom_brep::boundary_iso_u` reads back rational,
/// `NurbsCurve::speed_lower_bound` returns its documented poison (the
/// derivative of a rational B-spline is not a convex combination of any
/// control net), the `nurbs_span_meter` margin is `Invalid`, and the
/// body refuses at assembly. On a rational wall it lands on the wall's
/// corner rows — an arc segment's ends, which carry weight `1.0` in
/// every section and are the rows the wall shares with its neighbours.
/// The seam certificate (`topo::mint_pcurves`) needs the two walls'
/// rows in one spline space, knots and weights compared exactly, and a
/// corner row an ulp off its neighbour's `1.0` is a different space. So
/// constant-weight-in ⇒ constant-weight-out holds row by row, for every
/// row a seam can read.
///
/// The lane never manufactures a constant-weight row's weight channel,
/// rather than solving for it and snapping the answer back to `c` — a
/// silent heal, which would also leave the control points divided by the
/// pre-snap denominator. A row's lane is decided by `==` on the input's
/// own bits — its weights' value channel
/// ([`NurbsCurve3::weights_f64_interim`]): C6 structure selection about
/// what the caller handed in, never a predicate, never a tolerance. The
/// output weights of a constant-weight row are section 0's weight for
/// that row (the constant `1` for an integral row), never a solved value.
///
/// A rational row comes out bitwise as a whole-surface homogeneous solve
/// would give it: the collocation solve factors the matrix once from the
/// parameters alone and substitutes each right-hand-side column
/// independently, so one row's layout cannot move another row's
/// columns. A uniformly spaced loft skins to bit-identical walls
/// (`m7_skin_integral::the_uniform_loft_is_bitwise_unchanged`); the lane
/// split is pinned by
/// `m7_skin_integral::an_arc_walls_rows_each_skin_cartesian_at_their_own_weight`
/// and `m7_skin_integral::each_row_takes_its_own_lane`.
///
/// # The solve is banded
///
/// The collocation system is `k × k` in the SECTION count `k` — not in
/// the control count — with `3` simultaneous right-hand sides per
/// constant-weight row and `4` per rational one (`3·n` to `4·n` in all,
/// `n` = control points per section).
/// `interpolate_columns_in` factors it by the fit's fixed-order banded LU
/// (`geom::Collocation`): `O(k·q²)` for the
/// factorization at v-degree `q`, plus `O(k·q·n)` for the
/// substitutions, in `O(k·q + k·n)` memory.
///
/// The u-direction structure is the sections' shared one, untouched.
/// The result therefore passes through every section exactly (in ℝ) at
/// its own v-parameter — the property the acceptance suite verifies by
/// evaluation, not by assertion.
///
/// # Errors
///
/// [`SkinError::TooFewSections`], [`SkinError::BadDegree`],
/// [`SkinError::SectionShapeMismatch`] for sections that did not come
/// out of [`make_compatible`], [`SkinError::NoParameterStep`],
/// [`SkinError::Fit`] for a degenerate collocation system (at an
/// enclosure scalar, also one whose elimination over the sections'
/// enclosures leaves a pivot undecided, though every member's system is
/// regular), and
/// [`SkinError::Structure`] — notably a non-positive interpolated
/// weight, refused at the door because the convex-hull invariant every
/// C9 bound stands on requires `w > 0`.
pub fn skin<T: SpanLocate>(
    sections: &[NurbsCurve3<T>],
    v_degree: usize,
) -> Result<NurbsSurface<T>, SkinError> {
    let k = sections.len();
    if k < 2 {
        return Err(SkinError::TooFewSections { have: k, need: 2 });
    }
    if v_degree == 0 || v_degree >= k {
        return Err(SkinError::BadDegree {
            degree: v_degree,
            sections: k,
        });
    }
    let knots_u = sections[0].knots_f64_interim().clone();
    let n = sections[0].control().len();
    for (i, c) in sections.iter().enumerate().skip(1) {
        if c.control().len() != n || c.knots_f64_interim().knots() != knots_u.knots() {
            return Err(SkinError::SectionShapeMismatch {
                section: i,
                expected: n,
                found: c.control().len(),
                what: "compatible control points",
            });
        }
    }
    let params = skin_parameters(sections)?;
    skin_on(sections, v_degree, &params)
}

/// [`skin`] on an EXPLICIT v-parameterization — the entry a multi-wall
/// body needs, so every wall of one loft agrees on where its sections
/// sit (see [`loft_geometry`]).
///
/// # Errors
///
/// Exactly [`skin`]'s, plus [`SkinError::Fit`] for a parameterization
/// that is not clamped `0 → 1` and strictly ascending.
pub fn skin_on<T: SpanLocate>(
    sections: &[NurbsCurve3<T>],
    v_degree: usize,
    params: &[T],
) -> Result<NurbsSurface<T>, SkinError> {
    let k = sections.len();
    if k < 2 {
        return Err(SkinError::TooFewSections { have: k, need: 2 });
    }
    if v_degree == 0 || v_degree >= k {
        return Err(SkinError::BadDegree {
            degree: v_degree,
            sections: k,
        });
    }
    let knots_u = sections[0].knot_values().clone();
    let n = sections[0].control().len();
    for (i, c) in sections.iter().enumerate().skip(1) {
        if c.control().len() != n
            || c.knots_f64_interim().knots() != knots_u.structure_f64_interim().knots()
        {
            return Err(SkinError::SectionShapeMismatch {
                section: i,
                expected: n,
                found: c.control().len(),
                what: "compatible control points",
            });
        }
    }
    // Per row: `Some(c)` for a constant-weight row (see the docs).
    let constant: Vec<Option<T>> = (0..n)
        .map(|i| {
            let ws: Vec<f64> = sections
                .iter()
                .map(|s| s.weights_f64_interim()[i])
                .collect();
            geom_core::spline::constant_weight(&ws).map(|c| {
                if c == 1.0 {
                    T::one()
                } else {
                    sections[0].weights()[i]
                }
            })
        })
        .collect();
    let rows: Vec<Vec<T>> = sections
        .iter()
        .map(|c| {
            let mut row = Vec::new();
            for ((p, w), lane) in c.control().iter().zip(c.weights()).zip(&constant) {
                if lane.is_some() {
                    row.extend(p.to_array());
                } else {
                    row.extend([p.x * *w, p.y * *w, p.z * *w, *w]);
                }
            }
            row
        })
        .collect();
    let (knots_v, solved) = interpolate_columns_in(params, v_degree, &rows)?;
    // Into the surface's u-major layout, `idx = iu · nv + iv`.
    let nv = solved.len();
    let mut control = Vec::with_capacity(n * nv);
    let mut weights = Vec::with_capacity(n * nv);
    let mut at = 0;
    for lane in &constant {
        for row in &solved {
            if let Some(c) = lane {
                control.push(Point3::new(row[at], row[at + 1], row[at + 2]));
                weights.push(*c);
            } else {
                let (x, y, z, w) = (row[at], row[at + 1], row[at + 2], row[at + 3]);
                control.push(Point3::new(x / w, y / w, z / w));
                weights.push(w);
            }
        }
        at += if lane.is_some() { 3 } else { 4 };
    }
    // `NurbsSurface::new` is the `w > 0` door (and the count door).
    NurbsSurface::new(knots_u, knots_v, control, weights).map_err(SkinError::Structure)
}

// ---------------------------------------------------------------------
// §10.3/§10.4 as FEATURE geometry: the walls of a lofted body
// ---------------------------------------------------------------------

/// The definitional geometry of a loft or a path sweep: one skinned
/// wall per profile segment, plus the v-parameters the sections sit
/// at.
///
/// The **segment structure is the u-direction** (spec §3): wall `[l][j]`
/// spans profile loop `l`'s segment `j` across every section, so a
/// profile corner is a wall–wall seam by construction and every
/// cap–wall and wall–wall edge is an ISO-PARAMETER curve of the wall
/// it bounds — `v = 0` / `v = 1` for the caps, `u = 0` / `u = 1` for
/// the seams. That is the definitional payoff: those edges' pcurves are
/// exact straight lines in UV, no fit anywhere.
#[derive(Clone, Debug)]
pub struct LoftGeometry<T: Real = f64> {
    /// Skinned walls, per loop (outer first, then holes in canonical
    /// order), per segment in canonical order.
    pub walls: Vec<Vec<Arc<NurbsSurface<T>>>>,
    /// The v-parameter each section sits at (`params[0] = 0`,
    /// `params[k - 1] = 1`), shared by every wall — the sections are
    /// planar cross-sections of the whole loft at these values.
    pub section_params: Vec<T>,
    /// The compatible section curves per (loop, segment), kept because
    /// the cap and rim geometry is exactly section 0's and section
    /// `k − 1`'s — no re-derivation, no drift.
    pub sections: Vec<Vec<Vec<NurbsCurve3<T>>>>,
    /// **Each input section's canonical form**, in input order — the
    /// `ValidatedProfile` [`validate_sections`] decided at the door and
    /// the walls were skinned from. Kept for the same reason
    /// [`Self::sections`] is: the body assembly's end caps ARE section
    /// 0's and section `k − 1`'s profiles, so handing the decided form
    /// over is what makes the caps the walls' own sections rather than
    /// a second validation of the same data agreeing by determinism.
    pub canonical: Vec<ValidatedProfile<T>>,
}

/// One section of a loft or sweep: its loops in the profile
/// vocabulary — outer boundary first, holes after, each a
/// [`ProfileLoop`] (closed by construction; segment `j` runs from
/// vertex `j` to vertex `(j + 1) mod n`, as the loop's segment `j`)
/// — the same vocabulary extrude and revolve speak. Each
/// interior joint has exactly one naming, so a walls-vs-caps
/// disagreement is unrepresentable.
pub type Section<L = ProfileLoop<f64>> = Vec<L>;

/// **A loop a section is made of, at the scalar `T`**, and the
/// provenance its validation reads: a [`ProfileLoop`] is a table, whose
/// arcs [`Profile::validate`] checks against their vertices, and a
/// [`ConstructedLoop`] is the path lattice's own output, whose arcs
/// were verified at their construction (D1) and validate through
/// [`ConstructedProfile::validate`] without being decided again. A
/// section is all one or all the other.
///
/// A section's structure is decided once, at `f64`, on its loops'
/// value channel ([`SectionLoop::channel`]); at `T` the section is
/// validated GUIDED by that record ([`SectionLoop::validate_guided`]),
/// so its canonical form is the `f64` one and its values are the
/// loops' own.
pub trait SectionLoop<T: Decide = f64>: Clone {
    /// The loop at `f64`.
    type Channel: SectionLoop<f64>;

    /// This loop's value channel ([`SpanLocate::point_value`]), each
    /// value's one `f64`; NaN for a value that has none (poison, a wide
    /// enclosure), which validation refuses.
    fn channel(&self) -> Self::Channel;

    /// The loop's vertices and segments, as authored.
    fn as_profile_loop(&self) -> &ProfileLoop<T>;

    /// Validates `loops` as one section on `plane`, by their
    /// provenance, keeping the structure record.
    ///
    /// # Errors
    ///
    /// [`ProfileError`], as [`Profile::validate`].
    fn validate_recording(
        plane: SketchPlane<T>,
        loops: Vec<Self>,
        tol: Tol,
    ) -> Result<(ValidatedProfile<T>, CanonicalStructure), ProfileError>;

    /// Validates `loops` as one section on `plane`, by their
    /// provenance, consuming `structure`'s decisions
    /// ([`ConstructedProfile::validate_guided`]).
    ///
    /// # Errors
    ///
    /// [`ProfileError`], as [`ConstructedProfile::validate_guided`].
    fn validate_guided(
        plane: SketchPlane<T>,
        loops: Vec<Self>,
        tol: Tol,
        structure: &CanonicalStructure,
    ) -> Result<ValidatedProfile<T>, ProfileError>;
}

impl<T: Decide> SectionLoop<T> for ProfileLoop<T> {
    type Channel = ProfileLoop<f64>;

    fn channel(&self) -> ProfileLoop<f64> {
        self.map_scalar(SpanLocate::value_channel)
    }

    fn as_profile_loop(&self) -> &ProfileLoop<T> {
        self
    }

    fn validate_recording(
        plane: SketchPlane<T>,
        loops: Vec<Self>,
        tol: Tol,
    ) -> Result<(ValidatedProfile<T>, CanonicalStructure), ProfileError> {
        Profile::new(plane, loops).validate_recording(tol)
    }

    fn validate_guided(
        plane: SketchPlane<T>,
        loops: Vec<Self>,
        tol: Tol,
        structure: &CanonicalStructure,
    ) -> Result<ValidatedProfile<T>, ProfileError> {
        Profile::new(plane, loops).validate_guided(tol, structure)
    }
}

impl<T: Decide> SectionLoop<T> for ConstructedLoop<T> {
    type Channel = ConstructedLoop<f64>;

    fn channel(&self) -> ConstructedLoop<f64> {
        self.value_channel()
    }

    fn as_profile_loop(&self) -> &ProfileLoop<T> {
        self.as_loop()
    }

    fn validate_recording(
        plane: SketchPlane<T>,
        loops: Vec<Self>,
        tol: Tol,
    ) -> Result<(ValidatedProfile<T>, CanonicalStructure), ProfileError> {
        ConstructedProfile::new(plane, loops).validate_recording(tol)
    }

    fn validate_guided(
        plane: SketchPlane<T>,
        loops: Vec<Self>,
        tol: Tol,
        structure: &CanonicalStructure,
    ) -> Result<ValidatedProfile<T>, ProfileError> {
        ConstructedProfile::new(plane, loops).validate_guided(tol, structure)
    }
}

/// The section's world-space traversal data for segment `j`, as the
/// [`SketchSegment`] [`segment_curve`] consumes (`segment_curve` stays
/// public as the exact-path-leg door, and routing every wall through
/// it keeps the produced NURBS on one code path). The carrier is
/// selected by the validated segment's kind; an arc crosses into the
/// sketch-segment form with its carrier and sweep.
fn vertex_segment<T: Real>(lp: &profile::ValidatedLoop<T>, j: usize) -> SketchSegment<T> {
    crate::swept::sketch_segment(&crate::swept::SweptSeg::forward(lp, j))
}

/// Validates every section at the door: each section runs through
/// its loops' validation ([`SectionLoop`]) against its own placement —
/// the same gate extrude and revolve profiles pass — and the CANONICAL
/// loops (outer counterclockwise first, holes clockwise, canonical
/// start vertex) are what the skin consumes, so the walls and the
/// body assembly's caps read the same traversal by construction.
fn validate_sections<T: Decide, L: SectionLoop<T>>(
    sections: &[Section<L>],
    places: &[SketchPlane<T>],
    tol: Tol,
) -> Result<Vec<(ValidatedProfile<T>, CanonicalStructure)>, SkinError> {
    sections
        .iter()
        .zip(places)
        .enumerate()
        .map(|(i, (loops, place))| {
            L::validate_recording(*place, loops.clone(), tol)
                .map_err(|source| SkinError::SectionProfile { section: i, source })
        })
        .collect()
}

/// Each section validated at `T`, its decisions taken there: the door
/// for sections with no `f64` value to select a record on (values wider
/// than a point, at an enclosure scalar).
///
/// # Errors
///
/// [`SkinError::SectionProfile`] for a section that does not validate.
pub(crate) fn validate_at_t<T: Decide, L: SectionLoop<T>>(
    sections: &[Section<L>],
    places: &[SketchPlane<T>],
    tol: Tol,
) -> Result<Vec<ValidatedProfile<T>>, SkinError> {
    Ok(validate_sections(sections, places, tol)?
        .into_iter()
        .map(|(validated, _)| validated)
        .collect())
}

/// Each section at `T`, validated guided by `structures[i]`, the record
/// its value channel's validation kept ([`validate_loft`]): the
/// canonical form the `f64` pass decided, holding the loops' own
/// values.
///
/// # Errors
///
/// [`SkinError::SectionProfile`] for a section whose values this scalar
/// cannot validate along the record.
pub(crate) fn validate_guided<T: Decide, L: SectionLoop<T>>(
    sections: &[Section<L>],
    places: &[SketchPlane<T>],
    structures: &[CanonicalStructure],
    tol: Tol,
) -> Result<Vec<ValidatedProfile<T>>, SkinError> {
    sections
        .iter()
        .zip(places)
        .zip(structures)
        .enumerate()
        .map(|(i, ((loops, place), structure))| {
            L::validate_guided(*place, loops.clone(), tol, structure)
                .map_err(|source| SkinError::SectionProfile { section: i, source })
        })
        .collect()
}

/// Builds the definitional walls of a loft (§10.3) from profile-loop
/// sections, their placements, and the v-parameter each section sits
/// at.
///
/// `sections[k][l]` is section `k`, loop `l` (a [`ProfileLoop`]) in
/// the section's own sketch coordinates; `places[k]` is that
/// section's rigid placement; `params[k]` is its v-parameter, shared
/// by every wall — as [`skin_on`] takes it one layer down, and checked
/// there (clamped `0 → 1`, strictly ascending, one per section). Each
/// section passes [`Profile::validate`] at the door (fail loud — a
/// section that would not extrude does not skin either), and the
/// canonical loops are what get skinned.
///
/// The walls interpolate their sections, so section `k` is the body's
/// cross-section at `v = params[k]` for any parameters: the parameters
/// shape only the surface between sections. A loft body's, and a path
/// sweep's ([`sweep_geometry`]), are [`loft_parameters`].
///
/// # The correspondence is the author's
///
/// Wall `j` of loop `l` skins canonical segment `j` of canonical loop
/// `l` in every section, so every section must present the same loop
/// and vertex counts. The canonical form decides only what validity
/// forces — each loop's traversal sense, outer counterclockwise and
/// holes clockwise about the section's OWN plane normal — and keeps
/// everything else as authored. Whether two sections' senses agree
/// depends on how their normals face along the stack, which this door
/// does not check: the loft body door decides every section's normal
/// against its slabs (`crate::loft`'s stacking fold). Beyond the sense, each loop starts
/// at its authored vertex 0 and holes keep their authored order. So segment `j` is
/// counted from the author's start in every section, and which edges
/// line up — the loft's twist — is the author's choice, carried by
/// where each section's loop starts. No per-section rule could recover
/// it: a square rotated 90° about its centre is the same point set,
/// and the untwisted and quarter-twisted lofts between it and the
/// original are different solids.
///
/// # Errors
///
/// [`SkinError::TooFewSections`], [`SkinError::SectionShapeMismatch`]
/// naming the disagreeing section and count,
/// [`SkinError::SectionProfile`] for a section the profile door
/// refuses, [`SkinError::BadDegree`], and every refusal
/// [`segment_curve`], [`make_compatible`] and [`skin_on`] carry.
pub fn loft_geometry<L: SectionLoop>(
    sections: &[Section<L>],
    places: &[SketchPlane<f64>],
    v_degree: usize,
    params: &[f64],
    tol: Tol,
) -> Result<LoftGeometry, SkinError> {
    let (validated, _) = validate_loft(sections, places, v_degree, tol)?;
    let compat = compatible_strips(&validated, places)?;
    skin_strips(validated, compat, v_degree, params)
}

/// [`loft_geometry`]'s first half: every refusal that reads the
/// sections one at a time or by count — section and placement counts,
/// the degree, each section's profile door, and the loop and segment
/// counts every section must share. What it returns is the canonical
/// form [`skin_validated`] skins, so a caller can decide something
/// about the validated sections (the loft's stacking fold) before any
/// wall is built, and each section's structure record, which a lane
/// scalar's sections are validated along ([`validate_guided`]).
#[allow(clippy::type_complexity)]
pub(crate) fn validate_loft<L: SectionLoop>(
    sections: &[Section<L>],
    places: &[SketchPlane<f64>],
    v_degree: usize,
    tol: Tol,
) -> Result<(Vec<ValidatedProfile<f64>>, Vec<CanonicalStructure>), SkinError> {
    check_counts(sections.len(), places.len(), v_degree)?;
    let (validated, structures): (Vec<_>, Vec<_>) = validate_sections(sections, places, tol)?
        .into_iter()
        .unzip();
    check_shapes(&validated)?;
    Ok((validated, structures))
}

/// The counts a loft needs before any section is read: at least two
/// sections, one placement each, and a v-degree in `1..k`.
///
/// # Errors
///
/// [`SkinError::TooFewSections`], [`SkinError::SectionShapeMismatch`]
/// (placements) and [`SkinError::BadDegree`].
pub(crate) fn check_counts(k: usize, places: usize, v_degree: usize) -> Result<(), SkinError> {
    if k < 2 {
        return Err(SkinError::TooFewSections { have: k, need: 2 });
    }
    if places != k {
        return Err(SkinError::SectionShapeMismatch {
            section: places.min(k),
            expected: k,
            found: places,
            what: "placements",
        });
    }
    if v_degree == 0 || v_degree >= k {
        return Err(SkinError::BadDegree {
            degree: v_degree,
            sections: k,
        });
    }
    Ok(())
}

/// Every canonical section has section 0's loop count and, loop by
/// loop, its vertex count: the shape a by-index pairing needs.
///
/// # Errors
///
/// [`SkinError::SectionShapeMismatch`] naming the disagreeing section
/// and count.
pub(crate) fn check_shapes<T: Real>(validated: &[ValidatedProfile<T>]) -> Result<(), SkinError> {
    let loops = validated[0].loops().len();
    for (i, s) in validated.iter().enumerate().skip(1) {
        if s.loops().len() != loops {
            return Err(SkinError::SectionShapeMismatch {
                section: i,
                expected: loops,
                found: s.loops().len(),
                what: "loops",
            });
        }
        for (l, lp) in s.loops().iter().enumerate() {
            if lp.vertices().len() != validated[0].loops()[l].vertices().len() {
                return Err(SkinError::SectionShapeMismatch {
                    section: i,
                    expected: validated[0].loops()[l].vertices().len(),
                    found: lp.vertices().len(),
                    what: "segments",
                });
            }
        }
    }
    Ok(())
}

/// The compatible section curves of every strip, `[loop][segment]`,
/// each strip one curve per section.
#[allow(clippy::type_complexity)]
fn compatible_strips<T: SpanLocate>(
    validated: &[ValidatedProfile<T>],
    places: &[SketchPlane<T>],
) -> Result<Vec<Vec<Vec<NurbsCurve3<T>>>>, SkinError> {
    (0..validated[0].loops().len())
        .map(|l| {
            (0..validated[0].loops()[l].vertices().len())
                .map(|j| {
                    let raw: Vec<NurbsCurve3<T>> = validated
                        .iter()
                        .zip(places)
                        .enumerate()
                        .map(|(i, (s, place))| {
                            segment_curve(i, vertex_segment(&s.loops()[l], j), place.placement())
                        })
                        .collect::<Result<_, _>>()?;
                    make_compatible(&raw)
                })
                .collect()
        })
        .collect()
}

/// A loft's v-parameters: Book Eq. 10.8 over every compatible control
/// row of the OUTER loop's strips; every hole is interpolated at those
/// parameters. One vector per loop is forced — the seam at a vertex is
/// one wall's `u = 0` iso and its neighbour's `u = 1` iso, one curve
/// only if both walls interpolate at the same parameters — and taking
/// it over the whole loop makes it a function of the section set: no
/// strip, and so no authored start vertex, roll or sense, is singled
/// out. The outer loop is the one whose role validation fixes, so
/// reading it is still label-free; and a hole cannot move the outer
/// walls, so a hole that is the outer loop scaled lofts to the outer
/// surface scaled.
fn strip_parameters<T: SpanLocate>(
    strips: &[Vec<Vec<NurbsCurve3<T>>>],
) -> Result<Vec<T>, SkinError> {
    let k = strips[0][0].len();
    chord_length_parameters(
        k,
        strips[0]
            .iter()
            .flat_map(|strip| (0..strip[0].control().len()).map(move |i| control_row(strip, i))),
    )
}

/// Skins every strip at `params` into the loft's geometry.
fn skin_strips<T: SpanLocate>(
    validated: Vec<ValidatedProfile<T>>,
    strips: Vec<Vec<Vec<NurbsCurve3<T>>>>,
    v_degree: usize,
    params: &[T],
) -> Result<LoftGeometry<T>, SkinError> {
    let walls = strips
        .iter()
        .map(|lp| {
            lp.iter()
                .map(|strip| skin_on(strip, v_degree, params).map(Arc::new))
                .collect()
        })
        .collect::<Result<_, _>>()?;
    Ok(LoftGeometry {
        walls,
        section_params: params.to_vec(),
        sections: strips,
        canonical: validated,
    })
}

/// [`loft_geometry`]'s second half: the walls skinned over sections
/// [`validate_loft`] already accepted, at `params` where the caller
/// states them (a path sweep's path parameters) and otherwise at
/// [`loft_parameters`]' answer — one pass over the compatible strips
/// serves both the parameters and the walls.
pub(crate) fn skin_validated<T: SpanLocate>(
    validated: Vec<ValidatedProfile<T>>,
    places: &[SketchPlane<T>],
    v_degree: usize,
    params: Option<&[f64]>,
) -> Result<LoftGeometry<T>, SkinError> {
    let compat = compatible_strips(&validated, places)?;
    let params = match params {
        Some(params) => params.iter().map(|v| T::from_computed(*v)).collect(),
        None => strip_parameters(&compat)?,
    };
    skin_strips(validated, compat, v_degree, &params)
}

/// **What v-parameters will a loft put my sections at?** — the
/// read-back door for [`LoftGeometry::section_params`] of a
/// [`loft_body`](crate::loft_body), answerable from what a section
/// author already holds ([`Section`]s and their placements), before
/// any surface is built, and the parameters to hand [`loft_geometry`]
/// for the loft body's walls.
///
/// Book Eq. 10.8 over every compatible control row of the outer
/// loop's walls (holes read the outer loop's parameters): chord
/// lengths accumulated down each row, normalised, averaged. The
/// answer is a function of the section SET — re-spelling a section
/// from another vertex, or rolling it by one of its own symmetries,
/// does not move it. It is literally the computation `loft_body` runs
/// (one shared helper), so the answer is the construction's, not a
/// re-derivation of it — and it is NOT the z-spacing, which is why
/// it is worth asking rather than deriving by hand.
///
/// `v_degree` is not used to place the sections; it is validated with
/// everything else [`loft_geometry`] checks before skinning, so this
/// door refuses every section, count and degree fault
/// [`loft_geometry`] would. It does NOT run the loft's stacking fold:
/// it answers for a reversed stack or a sliver slab that
/// [`loft_body`](crate::loft_body) refuses, and two sections the body
/// refuses as not apart reach it as [`SkinError::NoParameterStep`].
///
/// # Errors
///
/// [`SkinError::TooFewSections`], [`SkinError::SectionShapeMismatch`]
/// (placement, loop and segment counts), [`SkinError::SectionProfile`],
/// [`SkinError::BadDegree`], and every refusal [`segment_curve`],
/// [`make_compatible`] and [`skin_parameters`] carry.
///
/// ```
/// use geom_core::{Point2, Point3, Tol};
/// use profile::{Open, SketchPlane, Start};
/// use sweep::{ProfileLoop, Section, loft_parameters};
///
/// let tol = Tol::witness();
/// // A section is authored through the PATHS lattice, which classifies
/// // each corner as it is written; the loop it lowers to is the vertex
/// // table this door consumes.
/// let quad = |pts: [(f64, f64); 4]| -> Section {
///     let p = |i: usize| Point2::new(pts[i].0, pts[i].1);
///     let loop_: ProfileLoop<f64> = Open
///         .at(p(0))
///         .line_to(p(1), tol).expect("a definitely-sharp corner")
///         .line_to(p(2), tol).expect("a definitely-sharp corner")
///         .line_to(p(3), tol).expect("a definitely-sharp corner")
///         .line_to(Start, tol).expect("the seam closes")
///         .into();
///     vec![loop_]
/// };
/// // The corpus's non-uniform prism: square, flared trapezoid,
/// // square — placed at z = 0, 1, 3.
/// let sections = vec![
///     quad([(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
///     quad([(-1.375, -1.0), (1.375, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
///     quad([(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
/// ];
/// let places: Vec<SketchPlane<f64>> = [0.0, 1.0, 3.0]
///     .iter()
///     .map(|z| SketchPlane::xy_at(Point3::new(0.0, 0.0, *z)))
///     .collect();
///
/// let params = loft_parameters(&sections, &places, 2, tol).expect("the sections skin");
/// // Ends pinned; the middle is the CHORD-length share averaged over
/// // every corner, not the z-share (which would be 1/3): the flare
/// // lengthens the first chord at the two bottom corners, giving
/// // t = (√73 / (√73 + √265) + 1/3) / 2.
/// assert_eq!(params[0], 0.0);
/// assert_eq!(params[2], 1.0);
/// assert_eq!(params[1], 0.33876641703757304);
/// ```
pub fn loft_parameters<L: SectionLoop>(
    sections: &[Section<L>],
    places: &[SketchPlane<f64>],
    v_degree: usize,
    tol: Tol,
) -> Result<Vec<f64>, SkinError> {
    let (validated, _) = validate_loft(sections, places, v_degree, tol)?;
    strip_parameters(&compatible_strips(&validated, places)?)
}

// ---------------------------------------------------------------------
// §10.4: the swept surface, by instantiate-and-skin
// ---------------------------------------------------------------------

/// **§10.4, the path sweep.** Places rigid copies of `profile` at
/// `stations` points along `path` and skins them (Book §10.4's
/// instantiate-and-skin; C11's scope box, verbatim: rigid profile,
/// path-following, **no** variable sections and **no** scaling laws).
///
/// The stations are placed by the path's carry of `plane`
/// ([`sweep_places`]; the law is `crates/sweep/README.md`
/// `[sweep.frame]`), and station `i`'s section sits at
/// `v = i / (stations − 1)`, the share of the path's parameter it was
/// sampled at.
///
/// `plane` — the STARTING frame the rest is carried from — is the
/// caller's, and `geom_core::linalg::frame::path_start_frame` is where
/// a caller gets it: the frame through the path's start point whose
/// local +Z is the start tangent, its roll off a reference ladder
/// decided under the band. A caller wanting a different roll composes
/// a rotation about the tangent onto that frame; there is no second
/// door.
///
/// Under Q8 the produced skin is not an approximation of a swept
/// locus — it IS the sweep (crate-module docs).
///
/// # Errors
///
/// [`SkinError::TooFewSections`] for `stations < 2`, every refusal
/// [`sweep_places`] makes, plus every refusal [`loft_geometry`]
/// carries.
pub fn sweep_geometry<L: SectionLoop>(
    profile: &[L],
    plane: SketchPlane<f64>,
    path: &NurbsCurve3<f64>,
    stations: usize,
    v_degree: usize,
    tol: Tol,
) -> Result<LoftGeometry, SkinError> {
    let carry = PathCarry::of(path, stations, tol)?;
    let places = carry.place(plane)?;
    let sections: Vec<Section<L>> = core::iter::repeat_n(profile.to_vec(), stations).collect();
    loft_geometry(&sections, &places, v_degree, &carry.params(), tol)
}

/// The K funnel name of a path sweep's station tangent: the path's
/// derivative decided into a direction witness.
pub(crate) const SWEEP_PATH_TANGENT: &str = "sweep_path_tangent";

/// The K funnel name of a path sweep's chord: the step from one
/// station to the next decided into a direction witness.
pub(crate) const SWEEP_PATH_CHORD: &str = "sweep_path_chord";

/// The K funnel name of a path sweep's step turn: `1 + m·tᵢ₊₁`, for
/// `m` the tangent `tᵢ` mirrored in the chord, the margin of the
/// minimal rotation carrying `m` onto `tᵢ₊₁`
/// ([`RigidMotion::turned_between`]).
pub(crate) const SWEEP_PATH_TURN: &str = "sweep_path_turn";

/// **The path's carry**: for each station, where the path is and the
/// rotation carrying the start frame there — `crates/sweep/README.md`
/// `[sweep.frame]`, the rotation-minimizing frame by double reflection.
/// It is a function of the `f64` path and the station count alone, so
/// it is computed once and every scalar lane composes it onto its own
/// start plane ([`Self::place`]).
pub(crate) struct PathCarry {
    /// The path's start point, the pivot every station's rotation
    /// turns about.
    start: Point3<f64>,
    /// Each station's point on the path and the rotation carrying the
    /// start frame there, about the coordinate origin; `None` where
    /// that rotation is the identity with no rounding.
    stations: Vec<(Point3<f64>, Option<RigidMotion<f64>>)>,
}

impl PathCarry {
    /// The carry of `path` sampled at `stations` uniform shares of its
    /// parameter.
    ///
    /// Each step from station `i` to `i + 1` is Wang, Jüttler, Zheng and
    /// Liu's double reflection (ACM TOG 2008): the reflection in the
    /// plane normal to the chord `c`, then the reflection carrying the
    /// mirrored tangent onto `tᵢ₊₁`. Their product is evaluated as the
    /// product of the half turns about `tᵢ` and about `ĉ` (the two
    /// reflections' product with `H_tᵢ` inserted), followed by the
    /// minimal rotation carrying `m = H_c·H_tᵢ·tᵢ` onto `tᵢ₊₁`
    /// ([`RigidMotion::turned_between`]), whose one decision is the
    /// second reflection's margin. A step whose two tangents are equal
    /// bit for bit is the identity with no rounding, as the exact law
    /// makes it (the second reflection is then the first).
    ///
    /// Three decisions, each under the band:
    ///
    /// - **the tangent** at each station (`sweep_path_tangent`): a
    ///   vanishing, overflowed or poisoned derivative has no direction;
    /// - **the chord** into each station after the first
    ///   (`sweep_path_chord`): coincident stations have no chord to
    ///   reflect in;
    /// - **the half turn** (`sweep_path_turn`): `1 + m·tᵢ₊₁ > 0`.
    ///   Where the path turns a half turn between two stations, `m` is
    ///   opposed to `tᵢ₊₁` and the second reflection has no plane.
    pub(crate) fn of(
        path: &NurbsCurve3<f64>,
        stations: usize,
        tol: Tol,
    ) -> Result<Self, SkinError> {
        if stations < 2 {
            return Err(SkinError::TooFewSections {
                have: stations,
                need: 2,
            });
        }
        let band = Band::linear(tol).map_err(SkinError::Band)?;
        let (lo, hi) = path.domain();
        let shares = station_shares(stations);
        let samples = shares
            .iter()
            .enumerate()
            .map(|(station, s)| {
                let (p, d) = path.ders1((hi - lo).mul_add(*s, lo));
                UnitVec3::new(d, SWEEP_PATH_TANGENT, band)
                    .map(|t| (p, t))
                    .map_err(|e| direction_refusal(e, station, PathDecision::Tangent))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let not_finite = |station| move |_| SkinError::StationNotFinite { station };
        let half = Turns::new(1, 2).unwrap_or_else(|| unreachable!("a nonzero denominator"));
        let o = Point3::origin();
        let mut turn: Option<RigidMotion<f64>> = None;
        let mut carried = vec![(samples[0].0, None)];
        for (i, pair) in samples.windows(2).enumerate() {
            let station = i + 1;
            let ((p, t), (q, next)) = (pair[0], pair[1]);
            let chord = UnitVec3::new(q - p, SWEEP_PATH_CHORD, band)
                .map_err(|e| direction_refusal(e, station, PathDecision::Chord))?;
            let (a, b) = (t.get(), next.get());
            let same = [(a.x, b.x), (a.y, b.y), (a.z, b.z)]
                .iter()
                .all(|(x, y)| x.to_bits() == y.to_bits());
            if !same {
                let mirror = RigidMotion::turned(o, chord, half)
                    .compose(RigidMotion::turned(o, t, half))
                    .map_err(not_finite(station))?;
                let step =
                    RigidMotion::turned_between(mirror.turn(t), next, o, SWEEP_PATH_TURN, band)
                        .map_err(|e| match e {
                            TurnError::Opposed => SkinError::PathHalfTurn { station },
                            TurnError::Undecided(source) => SkinError::PathUndecided {
                                station,
                                which: PathDecision::HalfTurn,
                                source,
                            },
                            TurnError::NonFinite(_) => SkinError::StationNotFinite { station },
                        })?
                        .compose(mirror)
                        .map_err(not_finite(station))?;
                turn = Some(match turn {
                    None => step,
                    Some(c) => step.compose(c).map_err(not_finite(station))?,
                });
            }
            carried.push((q, turn));
        }
        Ok(Self {
            start: samples[0].0,
            stations: carried,
        })
    }

    /// Each station's normalised path parameter, `i / (stations − 1)`:
    /// the v its section sits at.
    pub(crate) fn params(&self) -> Vec<f64> {
        station_shares(self.stations.len())
    }

    /// The carry composed onto `plane`: station 0 is `plane` itself;
    /// station `i` is `plane` turned about the path's start point `p₀`
    /// by the station's rotation `Cᵢ`, then translated onto the path —
    /// its origin `pᵢ + Cᵢ·(o − p₀)`. The rotation enters `T` as the
    /// computed values it is ([`RigidMotion::computed_at`]), so a lane
    /// decides nothing again.
    pub(crate) fn place<T: Real>(
        &self,
        plane: SketchPlane<T>,
    ) -> Result<Vec<SketchPlane<T>>, SkinError> {
        let start = plane.frame();
        let at = |p: Point3<f64>| p.map(T::from_computed);
        let back = Point3::origin() - at(self.start);
        self.stations
            .iter()
            .enumerate()
            .map(|(station, (p, turn))| {
                if station == 0 {
                    return Ok(plane);
                }
                match turn {
                    None => start.translated((*p - self.start).map(T::from_computed)),
                    Some(turn) => start.translated(back).and_then(|f| {
                        turn.computed_at::<T>()
                            .compose(f)?
                            .translated(at(*p) - Point3::origin())
                    }),
                }
                .map(SketchPlane::from_frame)
                .map_err(|_| SkinError::StationNotFinite { station })
            })
            .collect()
    }
}

/// `i / (stations − 1)` for each of `stations ≥ 2` stations: `0` and
/// `1` exactly at the ends.
fn station_shares(stations: usize) -> Vec<f64> {
    #[allow(clippy::cast_precision_loss)]
    let last = (stations - 1) as f64;
    #[allow(clippy::cast_precision_loss)]
    (0..stations).map(|i| i as f64 / last).collect()
}

/// A path direction's refusal at `station`: a vector with no direction
/// refuses as its decision's own arm, and an undecided one names it.
fn direction_refusal(e: UnitVec3Error, station: usize, which: PathDecision) -> SkinError {
    match e {
        UnitVec3Error::Escalated(source) => SkinError::PathUndecided {
            station,
            which,
            source,
        },
        UnitVec3Error::Degenerate
        | UnitVec3Error::NonFiniteLength
        | UnitVec3Error::UnderflowedLength => match which {
            PathDecision::Chord => SkinError::PathStationsCoincide { station },
            PathDecision::Tangent | PathDecision::HalfTurn => {
                SkinError::PathTangentVanishes { station }
            }
        },
    }
}

/// The rigid section planes of a §10.4 path sweep: `plane` carried
/// along `path` to each of `stations` uniform shares of its parameter
/// (`crates/sweep/README.md` `[sweep.frame]`). The body assembly
/// ([`crate::sweep_body`]) and [`sweep_geometry`] place their stations
/// here.
///
/// # Errors
///
/// [`SkinError::TooFewSections`] for `stations < 2`,
/// [`SkinError::Band`] for a tolerance with no band,
/// [`SkinError::PathTangentVanishes`] naming the station whose
/// tangent vanishes, [`SkinError::PathStationsCoincide`] the station
/// that coincides with the one before it, [`SkinError::PathHalfTurn`]
/// the station the path turns a half turn into,
/// [`SkinError::PathUndecided`] naming the station and decision that
/// could not be taken, and [`SkinError::StationNotFinite`] for a
/// station whose placement overflows.
pub fn sweep_places<T: Real>(
    plane: SketchPlane<T>,
    path: &NurbsCurve3<f64>,
    stations: usize,
    tol: Tol,
) -> Result<Vec<SketchPlane<T>>, SkinError> {
    PathCarry::of(path, stations, tol)?.place(plane)
}
