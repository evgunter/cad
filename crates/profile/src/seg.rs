//! Segment geometry and the named trilean predicates of validation.
//!
//! Everything here follows one rule, inherited from geom-core's Q1
//! machinery: **every margin is a length in meters**, classified against
//! the run's linear band through the [`geom_core::k_stats::decide`] funnel.
//! Angular and dimensionless questions are converted to displacements by
//! multiplying through their lever arm *in `T`* (D4 ¶1: d = r·θ — the
//! displacement an angle induces at the arm the decision turns on), so
//! no second band and no f64 extraction from `T` is ever needed. Each
//! predicate's margin and lever arm are documented at its definition.
//!
//! The pair-contact classification ([`pair_contacts`]) is the closed-form
//! simplicity core: line/line, line/arc, arc/arc, all exact. Its outcome
//! vocabulary: a **crossing** (transversal interior contact), a
//! **touch** (isolated contact at a segment endpoint), a **tangency**
//! (isolated interior contact without crossing — semantically
//! indeterminate for simplicity, escalated by the caller), an
//! **overlap** (shared sub-locus of positive length on a common
//! carrier), or nothing. Ray casting ([`ray_crossings`]) reuses the same
//! carrier closed forms for the containment forest's parity test.

use geom_core::k_stats::{check_unlogged, decide, decide_reported};
use geom_core::one_line::on_one_line_2d;
use geom_core::period::{Exact, Extent, Period, TurnRows};
use geom_core::{
    Arc2, Band, Circles2, Decide, Decided, Indeterminate, InfSpeed, Margin, MarginDiag, Point2,
    Real, Sign, Vec2,
};

use crate::Segment;
use crate::validate::ArcCheck;

/// The left normal: `v` rotated +90° counterclockwise, (−y, x).
pub(crate) fn perp<T: Real>(v: Vec2<T>) -> Vec2<T> {
    Vec2::new(-v.y, v.x)
}

/// A classified segment of a loop: the chord data plus the carrier
/// geometry of its canonical segment.
pub(crate) struct Seg<T: Real> {
    /// Start point.
    pub a: Point2<T>,
    /// End point.
    pub b: Point2<T>,
    /// The chord vector `b − a`: zero for a full turn, whose `b` is
    /// its `a`.
    pub chord: Vec2<T>,
    /// The chord length |b − a|: definitely positive between two
    /// vertices (degeneracy is rejected before a `Seg` is built), and
    /// zero for a full turn, whose reach `vertex_separation` decides
    /// instead.
    pub len: T,
    /// The unit chord direction `chord / len`; a full turn, which has
    /// no chord, stores its traversal heading at its vertex
    /// ([`Seg::heading_at`]). Finite and unit either way.
    pub unit: Vec2<T>,
    /// Line or arc, decided by the `segment_straightness` predicate.
    pub kind: SegKind<T>,
}

/// A segment's classified carrier kind.
pub(crate) enum SegKind<T: Real> {
    /// A straight segment on the chord.
    Line,
    /// A circular arc.
    Arc(ArcGeom<T>),
}

/// Arc geometry: the canonical segment's carrier and sweep, plus the
/// span its membership margin is written on.
pub(crate) struct ArcGeom<T: Real> {
    /// The canonical segment's carrier and signed sweep.
    pub arc: Arc2<T>,
    /// Which carrier points the arc holds.
    pub span: ArcSpan<T>,
    /// The turn sense: `Positive` = counterclockwise sweep,
    /// `Negative` = clockwise.
    pub turn: Sign,
}

/// Which carrier points an arc holds ([`arc_span`]).
pub(crate) enum ArcSpan<T: Real> {
    /// An arc between two vertices, 0 < |Δθ| < 2π.
    Chord {
        /// The arc's apex (its midpoint — the point farthest from the
        /// chord).
        apex: Point2<T>,
        /// |a − apex|: the chordal span threshold for membership (a
        /// carrier point q lies on the arc iff |q − apex| ≤ this —
        /// chord length is monotone in angular distance up to π, and
        /// the apex splits the arc into two halves of angle |θ|/2 ≤ π).
        span_chord: T,
    },
    /// D1's full turn, one segment at one vertex: every carrier point,
    /// and its vertex is not an endpoint — it joins the carrier to
    /// itself, so a contact there is interior like any other.
    FullTurn,
}

/// Why a segment's shape could not be read, each arm carrying the
/// margin the predicate that refused it classified — what a caller's
/// sentence names when it reports the refusal. Everything a constructed
/// arc can be refused for ([`build_constructed_seg`]).
pub(crate) enum ShapeIssue<T: Real> {
    /// The chord is degenerate: consecutive vertices coincident at
    /// tolerance (`vertex_separation` classified Zero — or Negative,
    /// unreachable for a true distance but mapped here defensively).
    Degenerate {
        /// The chord length |b − a|, meters.
        margin: T,
    },
    /// A full turn reaches nowhere: its carrier is below the band or
    /// its sweep is nothing (`full_turn_reach` classified Zero, or
    /// `full_turn_sense` did after it).
    DegenerateTurn {
        /// The turn's signed reach 2r·Δθ/2π, meters.
        margin: T,
    },
    /// The arc is within tolerance of a full circle
    /// (`arc_diameter_clearance` classified Zero — or Negative, only
    /// reachable through rounding at the boundary).
    NearFull {
        /// The diameter clearance 2r − |a − apex|, meters.
        margin: T,
    },
    /// A classification landed in the ambiguity band or was poisoned.
    Escalated(Indeterminate),
}

impl<T: Real> ShapeIssue<T> {
    /// The predicate that refused the segment — the name the run's own
    /// funnel recorded, so a caller reporting the refusal names what
    /// the classification named.
    pub(crate) fn predicate(&self) -> &'static str {
        match self {
            Self::Degenerate { .. } => "vertex_separation",
            Self::DegenerateTurn { .. } => "full_turn_reach",
            Self::NearFull { .. } => "arc_diameter_clearance",
            Self::Escalated(source) => source.predicate.unwrap_or("<unnamed>"),
        }
    }
}

/// Why a table's segment could not be built: its shape, or its stored
/// arc against its vertices ([`build_seg`] under [`Consistency::Decide`]).
pub(crate) enum SegIssue<T: Real> {
    /// The segment's shape ([`ShapeIssue`]).
    Shape(ShapeIssue<T>),
    /// The stored arc disagrees with its vertices: one of validation's
    /// three consistency checks classified the stored carrier or sweep
    /// definitely off (`arc_start_on_carrier`, `arc_landing`,
    /// `arc_sweep_range`).
    Inconsistent {
        /// Which check refused it.
        check: ArcCheck,
    },
    /// A difference check (`arc_start_on_carrier`, `arc_landing`)
    /// classified the stored arc off its vertices, at a magnitude whose
    /// `f64` rounding is itself past the band: the scene cannot read
    /// that difference, so the reading is not a statement about the
    /// input (`arc_carrier_resolution` classified the headroom Zero or
    /// Negative).
    BelowSceneResolution {
        /// Which check read the difference.
        check: ArcCheck,
        /// What that check classified.
        margin: MarginDiag,
        /// What `arc_carrier_resolution` classified
        /// ([`resolves`]'s margin).
        headroom: MarginDiag,
    },
}

impl<T: Real> From<ShapeIssue<T>> for SegIssue<T> {
    fn from(issue: ShapeIssue<T>) -> Self {
        Self::Shape(issue)
    }
}

impl<T: Real> SegIssue<T> {
    /// An escalation, as a table segment's refusal.
    fn escalated(source: Indeterminate) -> Self {
        Self::Shape(ShapeIssue::Escalated(source))
    }

    /// The predicate that refused the segment ([`ShapeIssue::predicate`]).
    #[cfg(test)]
    pub(crate) fn predicate(&self) -> &'static str {
        match self {
            Self::Shape(issue) => issue.predicate(),
            Self::Inconsistent { check } => check.predicate(),
            Self::BelowSceneResolution { .. } => "arc_carrier_resolution",
        }
    }
}

/// The chord frame of the segment a → b: its length, chord vector and
/// unit direction — computed ONCE, in one spelling, for every
/// expression the segment's own predicates ([`build_seg`]) write on it.
pub(crate) struct ChordFrame<T: Real> {
    /// |b − a|.
    pub len: T,
    /// b − a.
    pub chord: Vec2<T>,
    /// (b − a) / |b − a|.
    pub unit: Vec2<T>,
}

impl<T: Real> ChordFrame<T> {
    /// The frame of a → b. Total: a zero-length chord yields a poisoned
    /// unit, which the `vertex_separation` gate in
    /// [`build_seg`] refuses before anything reads them.
    pub(crate) fn of(a: Point2<T>, b: Point2<T>) -> Self {
        let len = a.distance(b);
        let chord = b - a;
        let unit = chord / len;
        Self { len, chord, unit }
    }
}

/// Whether [`build_seg`] decides an arc's three consistency checks
/// (D1): a table's arcs are checked here, and an arc a construction
/// built was checked by the door that built it, at its scalar, so the
/// checks are not decided again.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Consistency {
    /// Decide the three checks (a table: data no construction produced
    /// — the fixture door, a hand-built loop, `map_scalar`, a pinned
    /// lift).
    Decide,
    /// The arc is the construction's own output, proved over the reals
    /// by its construction (its own predicate, such as a `Center` arc's
    /// `path_arc_center_equidistant`, or the identities its lowering
    /// registers) and checked at its scalar: at an exact witness by
    /// those registered identities, at a point scalar by the door's
    /// read-back ([`read_back_arc`]).
    /// Passed by every validation of a constructed loop
    /// ([`crate::ConstructedProfile`]) and by the path door's re-read of
    /// the fillet arcs it has just lowered.
    ByConstruction,
}

/// Builds and classifies a segment from its endpoints and its stored
/// canonical segment. An arc's carrier and sweep are the stored ones,
/// verified against the endpoints here and never re-derived from them.
///
/// Predicates fired, in order:
///
/// - **`vertex_separation`** — margin: the chord length |b − a|
///   (meters, a direct displacement; no lever arm). Zero ⇒ degenerate
///   segment (the Q1 sliver semantics: within band ⇒ escalation).
/// - **`arc_sweep_range`** (arcs only, and only under
///   [`Consistency::Decide`], refused typed as the consistency checks
///   below are) — two margins, in meters at the radius: the arc length
///   r·Δθ, definitely nonzero (`arc_sweep_length`; its sign is the turn
///   sense), then the gap r·(2π − |Δθ|) to a full turn (the turn rule),
///   which must be definitely short: 0 < |Δθ| < 2π. A full turn
///   is refused here: between two vertices it is a zero chord,
///   and D1's full turn is one segment at one vertex
///   ([`build_loop_seg`]'s other arm). Asked before the straightness
///   read because that read's tan(Δθ/4) is 4π-periodic: a sweep of
///   4π + δ bows like δ, and would read as δ's chord.
/// - **`segment_straightness`** — margin: the signed sagitta
///   s = (L/2)·tan(Δθ/4) (meters): the apex's offset from the chord,
///   zero for a stored line. This is the dimensionless quarter-sweep
///   tangent acting through its lever arm, the half-chord L/2. Zero ⇒
///   line (the arc is chord-coincident at tolerance, and its carrier is
///   not carried); a definite sign ⇒ arc with that turn sense, which is
///   the sweep's own sign over the range just asked; in-band ⇒ a sliver
///   arc, escalated.
/// - **The difference checks** (arcs read as arcs, and only under
///   [`Consistency::Decide`]; with `arc_sweep_range`, D1's table arcs
///   "verified at validate as ε-decisions"), each refused typed
///   ([`SegIssue::Inconsistent`]) on a definite answer it does not
///   accept:
///   - **`arc_start_on_carrier`** — margin: ‖a − c‖ − r (meters,
///     direct; [`Arc2::rim`]). Zero ⇒ the start lies on the carrier.
///   - **`arc_landing`** — margin: ‖point_from(a, 1) − b‖ (meters,
///     direct; [`Arc2::landing`]). Zero ⇒ the start, turned by the
///     sweep about the centre, lands on the end.
/// - **`arc_diameter_clearance`** (arcs only) — margin:
///   2r − |a − apex| = 2r·cos²(|Δθ|/4)/(1 + sin(|Δθ|/4)) (meters,
///   computed in the second form, which does not cancel near a full
///   turn as `1 − sin(|Δθ|/4)` does): how far the arc's half-span chord sits below the
///   carrier diameter, ≈ L²/(16r) for a near-full arc of
///   endpoint chord L. Zero ⇒ the arc is within tolerance of a full
///   circle — rejected as [`ShapeIssue::NearFull`] (the angular gap g
///   satisfies r·g²/16 ≤ ε, so the complement is a sliver and
///   `arc_span`'s chordal-defect margins would compress arc-length
///   distances by cos(θ/4) ≤ √(ε/r) — false-coincidence territory;
///   review finding, M2 PR 2 fix pass); in-band ⇒ escalated. This gate
///   keeps `arc_span`'s Zero an honest positive claim — see its docs
///   for the residual √(2rε/K) endpoint-zone bound that survives the
///   gate.
pub(crate) fn build_seg<T: Decide>(
    a: Point2<T>,
    b: Point2<T>,
    segment: Segment<T>,
    consistency: Consistency,
    band: Band,
) -> Result<Seg<T>, SegIssue<T>> {
    match consistency {
        Consistency::Decide => classify(
            a,
            b,
            segment,
            band,
            |arc| sweep_range(arc, Range::Chord, band),
            |arc| check_carrier(arc, a, b, band),
        ),
        Consistency::ByConstruction => {
            build_constructed_seg(a, b, segment, band).map_err(SegIssue::Shape)
        }
    }
}

/// Builds and classifies segment `k` of the loop `vertices` /
/// `segments` ([`build_seg`] with the loop's arity in hand): a loop of
/// one vertex holding an arc is D1's full turn ([`crate::is_full_turn`]),
/// read by [`full_turn`]; every other segment runs from vertex `k` to
/// vertex `k + 1 (mod n)` through [`build_seg`] — a one-vertex LINE
/// included, whose zero chord `vertex_separation` refuses.
pub(crate) fn build_loop_seg<T: Decide>(
    vertices: &[Point2<T>],
    segments: &[Segment<T>],
    k: usize,
    consistency: Consistency,
    band: Band,
) -> Result<Seg<T>, SegIssue<T>> {
    let a = vertices[k];
    match segments[k] {
        Segment::Arc(arc) if crate::is_full_turn(segments) => match consistency {
            Consistency::Decide => full_turn(a, arc, band, |arc| {
                check_carrier(arc, a, a, band)?;
                sweep_range(arc, Range::FullTurn, band)
            }),
            // No door constructs a full turn yet, so nothing has yet
            // shown a construction that writes one exactly (its start
            // on its carrier, |Δθ| = 2π): this arm decides the turn's
            // reach and sense and trusts the rest, like every
            // constructed arc. `circle-lowers-to-one-segment` owns that
            // construction and its proof.
            Consistency::ByConstruction => {
                full_turn(a, arc, band, |_| Ok(())).map_err(SegIssue::Shape)
            }
        },
        _ => build_seg(
            a,
            vertices[(k + 1) % vertices.len()],
            segments[k],
            consistency,
            band,
        ),
    }
}

/// **A full turn at one vertex** (D1: the loop's one segment, from its
/// vertex `a` back to it through |Δθ| = 2π) — [`build_seg`]'s
/// questions where a full turn has no chord to write them on, under
/// predicate names of their own:
///
/// - **`full_turn_reach`** — margin: the turn's reach from its vertex,
///   |m| with m = 2r·Δθ/2π (meters: the sweep as a fraction of a turn,
///   levered by the diameter — the diameter at |Δθ| = 2π). Zero ⇒
///   degenerate (a carrier below the band, or a sweep of nothing).
/// - **`full_turn_sense`** — margin: m itself, the antipode's signed
///   offset from the tangent at the vertex (the sagitta, whose limit a
///   chord arc's (L/2)·tan(Δθ/4) is). Its sign is the turn; its
///   magnitude was just decided Positive, so Zero is unreachable and
///   is refused as degenerate rather than read as a line.
/// - **The consistency checks** (under [`Consistency::Decide`]), as
///   [`build_seg`] states them with `b = a`, except that
///   **`arc_sweep_length`** and **`arc_sweep_range`** read the same
///   two margins, the second ACCEPTING a whole turn — |Δθ| = 2π — refusing short of it or past it (a
///   landing two turns round is Zero at `arc_landing`).
///
/// No `arc_diameter_clearance`: a full turn is the circle that gate
/// keeps chord arcs short of. Its span is [`ArcSpan::FullTurn`]: every
/// carrier point, the vertex included, so a contact at or near the
/// vertex is interior. Its chord frame is the finite one [`Seg`]
/// documents (zero chord, zero length, the heading at the vertex).
fn full_turn<T: Decide, E: From<ShapeIssue<T>>>(
    a: Point2<T>,
    arc: Arc2<T>,
    band: Band,
    check: impl FnOnce(Arc2<T>) -> Result<(), E>,
) -> Result<Seg<T>, E> {
    let escalated = |source| E::from(ShapeIssue::Escalated(source));
    let reach = Margin::levered(arc.sweep / T::tau(), arc.radius + arc.radius);
    let m = reach.value();
    match decide("full_turn_reach", Margin::of(m.abs()), band).map_err(escalated)? {
        Sign::Positive => {}
        Sign::Zero | Sign::Negative => return Err(ShapeIssue::DegenerateTurn { margin: m }.into()),
    }
    let turn = match decide("full_turn_sense", reach, band).map_err(escalated)? {
        Sign::Zero => return Err(ShapeIssue::DegenerateTurn { margin: m }.into()),
        turn @ (Sign::Positive | Sign::Negative) => turn,
    };
    check(arc)?;
    let kind = SegKind::Arc(ArcGeom {
        arc,
        span: ArcSpan::FullTurn,
        turn,
    });
    let heading = heading_on(&kind, a, Vec2::zero());
    Ok(Seg {
        a,
        b: a,
        chord: Vec2::zero(),
        len: T::zero(),
        unit: heading,
        kind,
    })
}

/// Which sweeps [`sweep_range`] accepts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Range {
    /// 0 < |Δθ| < 2π: an arc between two vertices.
    Chord,
    /// |Δθ| = 2π: the full turn at one vertex.
    FullTurn,
}

/// [`build_seg`] of a constructed arc's segment
/// ([`Consistency::ByConstruction`]): no consistency check is decided,
/// so none can refuse, and the error type has no arm for one.
pub(crate) fn build_constructed_seg<T: Decide>(
    a: Point2<T>,
    b: Point2<T>,
    segment: Segment<T>,
    band: Band,
) -> Result<Seg<T>, ShapeIssue<T>> {
    classify(a, b, segment, band, |_| Ok(()), |_| Ok(()))
}

/// [`build_seg`]'s body, with an arc's consistency checks: `range` on
/// its stored sweep before the straightness read, which a sweep outside
/// the range would fool, and `check` on its carrier once it is read as
/// an arc.
fn classify<T: Decide, E: From<ShapeIssue<T>>>(
    a: Point2<T>,
    b: Point2<T>,
    segment: Segment<T>,
    band: Band,
    range: impl FnOnce(Arc2<T>) -> Result<(), E>,
    check: impl FnOnce(Arc2<T>) -> Result<(), E>,
) -> Result<Seg<T>, E> {
    let escalated = |source| E::from(ShapeIssue::Escalated(source));
    let frame = ChordFrame::of(a, b);
    let len = frame.len;
    match decide("vertex_separation", Margin::of(len), band).map_err(escalated)? {
        Sign::Positive => {}
        Sign::Zero | Sign::Negative => {
            return Err(ShapeIssue::Degenerate { margin: len }.into());
        }
    }
    // The decision is fired for every segment, since the K stream
    // records it for every segment, but only an arc's is read: a stored
    // line has no turn for its margin to report.
    let bow = match segment {
        Segment::Line => T::zero(),
        Segment::Arc(arc) => {
            range(arc)?;
            arc.quarter_tan()
        }
    };
    let straightness =
        decide("segment_straightness", sagitta(bow, len), band).map_err(escalated)?;
    let kind = match segment {
        Segment::Line => SegKind::Line,
        Segment::Arc(..) if straightness == Sign::Zero => SegKind::Line,
        Segment::Arc(arc) => {
            let turn = straightness;
            let span = match turn {
                Sign::Negative => arc.reversed().sweep,
                Sign::Positive | Sign::Zero => arc.sweep,
            };
            check(arc)?;
            let apex = arc.apex(a, b);
            let span_chord = a.distance(apex);
            // 2r − |a − apex| with |a − apex| = 2r·sin(|Δθ|/4), spelled
            // on the carrier as 2r·cos²(|Δθ|/4)/(1 + sin(|Δθ|/4)): the
            // chord-scale apex carries tan(|Δθ|/4), whose relative error
            // grows as 1/(2π − |Δθ|) near a full turn, and
            // 1 − sin(|Δθ|/4) cancels there.
            let quarter = span * T::from_f64(0.25);
            let clearance =
                (arc.radius + arc.radius) * quarter.cos().powi(2) / (T::one() + quarter.sin());
            match decide("arc_diameter_clearance", Margin::of(clearance), band)
                .map_err(escalated)?
            {
                Sign::Positive => {}
                Sign::Zero | Sign::Negative => {
                    return Err(ShapeIssue::NearFull { margin: clearance }.into());
                }
            }
            SegKind::Arc(ArcGeom {
                arc,
                span: ArcSpan::Chord { apex, span_chord },
                turn,
            })
        }
    };
    Ok(Seg {
        a,
        b,
        chord: frame.chord,
        len,
        unit: frame.unit,
        kind,
    })
}

/// The rounding a lowered f64 arc carries into `arc_start_on_carrier`'s
/// difference ‖a − c‖ − r, in ulps of the check's scale (`2⁻⁵²·scale`),
/// to first order with every rounding at most half an ulp of a
/// magnitude at most twice the scale: the lowered centre 4 (the chord,
/// its midpoint, its unit normal, the apothem's product and the sum
/// onto the midpoint), the lowered radius 2 (the quotient
/// L(1 + b²)/(4b)), and the check's own subtraction, norm and
/// difference 2.
const ON_CARRIER_ULPS: f64 = 8.0;

/// The rounding a lowered f64 arc carries into `arc_landing`'s
/// difference ‖landing(a) − b‖, in ulps of the check's scale, counted
/// as [`ON_CARRIER_ULPS`] is: the lowered centre 4, the sweep 4·atan(b)
/// turned through the radius 4 (the arctangent's rounding over
/// |Δθ| ≤ 2π), the rotation's sine, cosine, products and sums 4, and
/// the landing's sum onto `a` and its distance to `b` 2 — 14, stated
/// as 16.
#[cfg_attr(
    not(any(test, feature = "test-support")),
    allow(
        unreachable_pub,
        reason = "re-exported by the crate root only under \
     `test-support`; interior in every other build"
    )
)]
pub const LANDING_ULPS: f64 = 16.0;

/// **`arc_carrier_resolution`** — whether a difference a consistency
/// check read at magnitude `scale` is one the scene can read at all.
/// Margin: K·ε/(ulps·2⁻⁵²) − scale (meters): how far the check's
/// magnitude sits below the one at which `ulps` of an `f64`'s rounding
/// there, ulps·scale·2⁻⁵², reaches the escalation band K·ε. `ulps` is
/// the check's own rounding bound ([`ON_CARRIER_ULPS`],
/// [`LANDING_ULPS`]). Written on magnitudes rather than on the two
/// resolutions so the margin is at the scene's scale, where the band is
/// no lever on it. Positive ⇒ the check's definite answer stands; Zero
/// or Negative ⇒ the answer may be the representation's rounding, and
/// the refusal is [`SegIssue::BelowSceneResolution`], not an
/// inconsistency.
///
/// `2⁻⁵²` (`f64::EPSILON`) at every scalar, for the reason the path
/// door's `FilletCarrierBelowSceneResolution` gives: every scalar this
/// kernel ships carries an `f64` value channel.
fn resolves<T: Decide>(
    check: ArcCheck,
    margin: MarginDiag,
    scale: T,
    ulps: f64,
    band: Band,
) -> Result<(), SegIssue<T>> {
    let gate = decide_reported(
        "arc_carrier_resolution",
        Margin::of(resolution_headroom(scale, ulps, band)),
        band,
    )
    .map_err(SegIssue::escalated)?;
    match gate.sign {
        Sign::Positive => Ok(()),
        Sign::Zero | Sign::Negative => Err(SegIssue::BelowSceneResolution {
            check,
            margin,
            headroom: gate.margin,
        }),
    }
}

/// [`resolves`]'s margin: K·ε/(ulps·2⁻⁵²) − `scale`, meters.
#[cfg_attr(
    not(any(test, feature = "test-support")),
    allow(
        unreachable_pub,
        reason = "re-exported by the crate root only under \
     `test-support`; interior in every other build"
    )
)]
pub fn resolution_headroom<T: Real>(scale: T, ulps: f64, band: Band) -> T {
    T::from_f64(band.escalate() / (ulps * f64::EPSILON)) - scale
}

/// Validation's two difference checks of a stored arc against its
/// endpoints `a → b`, `arc_start_on_carrier` then `arc_landing`,
/// decided at the run's band ([`build_seg`] states each margin).
fn check_carrier<T: Decide>(
    arc: Arc2<T>,
    a: Point2<T>,
    b: Point2<T>,
    band: Band,
) -> Result<(), SegIssue<T>> {
    let refuse = |check: ArcCheck| Err(SegIssue::Inconsistent { check });
    // A difference check's definite answer is a statement about the
    // input only where the scene's `f64` can read a difference that
    // small at the magnitude it was taken at ([`resolves`]).
    let scale = arc.radius.max(reach(arc.centre));
    let on_carrier = arc.rim(a) - arc.radius;
    let read = decide_reported("arc_start_on_carrier", Margin::of(on_carrier), band)
        .map_err(SegIssue::escalated)?;
    match read.sign {
        Sign::Zero => {}
        Sign::Positive | Sign::Negative => {
            resolves(
                ArcCheck::OnCarrier,
                read.margin,
                scale.max(reach(a)),
                ON_CARRIER_ULPS,
                band,
            )?;
            return refuse(ArcCheck::OnCarrier);
        }
    }
    let landing = arc.landing(a).distance(b);
    let read =
        decide_reported("arc_landing", Margin::of(landing), band).map_err(SegIssue::escalated)?;
    match read.sign {
        Sign::Zero => {}
        Sign::Positive | Sign::Negative => {
            resolves(
                ArcCheck::Landing,
                read.margin,
                scale.max(reach(a)).max(reach(b)),
                LANDING_ULPS,
                band,
            )?;
            return refuse(ArcCheck::Landing);
        }
    }
    Ok(())
}

/// One read-back of a constructed arc against what was authored, past
/// the band ([`read_back`]).
#[derive(Clone, Copy, Debug)]
pub(crate) struct ArcMiss<T: Real> {
    /// The read's funnel name: [`START_READ`], [`LANDING_READ`],
    /// [`THROUGH_POINT_READ`] or [`BRANCH_READ`].
    pub(crate) predicate: &'static str,
    /// How far the stored arc is off what was authored, meters.
    pub(crate) miss: T,
    /// The magnitude the read was taken at, meters.
    pub(crate) scale: T,
    /// The rounding the read carries at that magnitude,
    /// `ulps · 2⁻⁵² · scale`, meters.
    pub(crate) resolution: T,
}

/// How a constructed arc's read-back refuses ([`read_back`]).
pub(crate) enum ReadBackIssue<T: Real> {
    /// Off past the band at a magnitude the scene resolves: the
    /// construction's arithmetic lost the digits its proof is about.
    Defect(ArcMiss<T>),
    /// Off past the band at a magnitude whose rounding reaches the
    /// band: the input's conditioning or the construction's, which a
    /// point scalar cannot tell apart.
    BelowSceneResolution(ArcMiss<T>),
    /// A read, or its resolution gate, in the band or poisoned.
    Escalated(Indeterminate),
}

/// The start read's funnel name: ‖a − centre‖ − r.
pub(crate) const START_READ: &str = "path_arc_start_on_carrier";

/// The landing read's funnel name: ‖landing(a) − b‖, less the radial
/// part an inline decision admitted.
pub(crate) const LANDING_READ: &str = "path_arc_landing";

/// The through-point read's funnel name: ‖q − centre‖ − r.
pub(crate) const THROUGH_POINT_READ: &str = "path_arc_through_point";

/// The branch read's funnel name: the stored arc's length, signed by
/// the authored turn sense.
pub(crate) const BRANCH_READ: &str = "path_arc_branch";

/// **A segment's signed sagitta**, `(|b − a|/2)·tan(Δθ/4)` meters: the
/// apex's offset from the chord, `bow` (tan(Δθ/4), zero for a line)
/// acting through the half-chord. The one spelling of
/// `segment_straightness`'s margin, read by [`build_seg`] and by the
/// door's read-back ([`read_back_arc`]).
fn sagitta<T: Real>(bow: T, len: T) -> Margin<T> {
    Margin::levered(bow * T::from_f64(0.5), len)
}

/// **The path door's read-back of a constructed arc** from `a` to `b`
/// (audit row F24), on the witness lane (`Witness::Inexact`) only: the
/// stored arc against what its construction authored, each read through
/// [`read_back`].
///
/// - **Start on the carrier** ([`START_READ`], ‖a − centre‖ − r,
///   [`READ_BACK_RIM_ULPS`]): the stored radius against the vertex the
///   arc leaves.
/// - **Landing** ([`LANDING_READ`], [`LANDING_ULPS`]): ‖landing(a) − b‖.
///   For an arc whose end is on its carrier only to an inline
///   ε-decision the door made ([`crate::Facts::Decided`]), the landing
///   against `b` moved along its ray from the centre onto the landing's
///   own circle: 2r·sin of half the sweep's own error, with `b`'s
///   distance from the centre, which that decision admitted, left out.
///   An in-band sweep error escalates there exactly as on a registered
///   arc.
/// - **Through-point** ([`THROUGH_POINT_READ`], [`READ_BACK_RIM_ULPS`]):
///   a `Via` arc's carrier against its authored `q`.
/// - **Branch** ([`BRANCH_READ`], [`LANDING_ULPS`]): the stored arc's
///   length `r·Δθ`, signed by the turn sense the construction states
///   ([`crate::BuiltArc::sense`]). Positive passes; Negative is the
///   other arc on the same carrier between the same ends, which lands
///   exactly. Both span the chord, so neither reads shorter than it.
///   The sense is a decision the construction made on its inputs (the
///   side of the chord `q` lies on, the side of the departure a target
///   lies on, a winding, a corner's turn), and for a `Via`, `Center`,
///   side-run, tangent or fillet arc it is the same primitive that fixes
///   the sign of the arc's quarter-tangent X. So the read catches a
///   defect after that sign (an X spelled −1/X, the other arc on the
///   carrier), not one in the primitive itself. A `Bulge` leg states no
///   sense apart from its bulge, which is the authored data.
///
/// An arc whose bulge is the authored data itself
/// ([`crate::BuiltArc::bulge_authored`]) and whose sagitta reads as a
/// line is not read: validation reads it as a line that keeps no
/// carrier, as authored. Every other arc is read whatever its stored
/// form looks like, so a construction that flattened an arc it
/// authored is read and refused. An exact scalar's stored arc encloses
/// the arc its construction proved, and its registered identities are
/// its check, so nothing is read there.
pub(crate) fn read_back_arc<T: Decide>(
    built: &crate::BuiltArc<T>,
    a: Point2<T>,
    b: Point2<T>,
    band: Band,
) -> Result<(), ReadBackIssue<T>> {
    if T::WITNESS == geom_core::Witness::Exact {
        return Ok(());
    }
    let arc = built.arc;
    let len = a.distance(b);
    let bow = sagitta(arc.quarter_tan(), len).value();
    if built.bulge_authored {
        match check_unlogged("segment_straightness", bow, band, "F24") {
            Ok(Sign::Positive | Sign::Negative) => {}
            Ok(Sign::Zero) => return Ok(()),
            Err(_) if !bow.is_poison() => return Ok(()),
            Err(_) => {}
        }
    }
    let carrier = || arc.radius.max(reach(arc.centre));
    read_back(
        START_READ,
        "path_arc_start_on_carrier_resolution",
        Reading::Difference(arc.rim(a) - arc.radius),
        || reach(a),
        || carrier().max(reach(a)),
        READ_BACK_RIM_ULPS,
        band,
    )?;
    let landing = if built.facts == crate::Facts::Decided {
        // The landing against `b` moved along its ray from the centre
        // onto the landing's own circle: 2r·sin of half the sweep's own
        // error, whatever `b`'s distance from the centre, which is the
        // decision's.
        let towards = b - arc.centre;
        let onto = arc.centre + towards * (arc.rim(a) / towards.norm());
        arc.landing(a).distance(onto)
    } else {
        arc.landing(a).distance(b)
    };
    read_back(
        LANDING_READ,
        "path_arc_landing_resolution",
        Reading::Difference(landing),
        || reach(a).max(reach(b)),
        || carrier().max(reach(a)).max(reach(b)),
        LANDING_ULPS,
        band,
    )?;
    if let Some(q) = built.through {
        read_back(
            THROUGH_POINT_READ,
            "path_arc_through_point_resolution",
            Reading::Difference(arc.rim(q) - arc.radius),
            || reach(a).max(reach(b)).max(reach(q)),
            || carrier().max(reach(q)),
            READ_BACK_RIM_ULPS,
            band,
        )?;
    }
    if let Some(sense) = built.sense {
        // The stored arc's length, signed by the authored sense. Both the
        // authored arc and the other one on the carrier span the chord,
        // so each reads at least the chord's length: the authored one
        // Positive, the other Negative.
        let signed = Margin::levered(arc.sweep * sense.sign::<T>(), arc.radius).value();
        let reading = match check_unlogged(BRANCH_READ, signed, band, "F24") {
            Ok(Sign::Positive) => Reading::Passed,
            Ok(Sign::Zero | Sign::Negative) => Reading::Off(signed),
            Err(source) => return Err(ReadBackIssue::Escalated(source)),
        };
        read_back(
            BRANCH_READ,
            "path_arc_branch_resolution",
            reading,
            || reach(a).max(reach(b)),
            || carrier().max(reach(a)).max(reach(b)),
            LANDING_ULPS,
            band,
        )?;
    }
    Ok(())
}

/// The rounding the door's two rim reads (`‖p − centre‖ − r`, the start
/// and a `Via` arc's through-point) carry, in ulps of the read's
/// magnitude. Validation's [`ON_CARRIER_ULPS`] is derived for a lowered
/// arc, whose centre and radius come from one chord; a construction can
/// derive both from points it derived itself. Counted as there: the
/// centre from a lowered carrier 4; a radius re-read as the norm of a
/// difference of derived points (a fused side's `‖anchor − centre‖`),
/// with the centre's rounding inside it, 4; a vertex the construction
/// derived (a trim point) 4; the read's own subtraction, norm and
/// difference 2 — 14, stated as 16. A sound fused `Via` fillet was
/// measured at 9.0 ulps.
#[cfg_attr(
    not(any(test, feature = "test-support")),
    allow(
        unreachable_pub,
        reason = "re-exported by the crate root only under \
     `test-support`; interior in every other build"
    )
)]
pub const READ_BACK_RIM_ULPS: f64 = 16.0;

/// One read of the door's read-back, as [`read_back`] takes it.
enum Reading<T> {
    /// A difference the authored data makes zero: Zero passes, a
    /// definite sign is off.
    Difference(T),
    /// A read its caller classified as off, by this much.
    Off(T),
    /// A read its caller classified as passing.
    Passed,
}

/// **One read of the door's read-back** ([`read_back_arc`]), and where
/// it reads off, whether the miss is a kernel defect: one margin, the
/// lesser of
///
/// - the **headroom** of the authored data's magnitude
///   ([`resolution_headroom`] at `ulps`, as validation's [`resolves`]
///   reads a table's): whether the scene the user authored resolves the
///   read at all; and
/// - the miss's **clearance** of the read's own rounding at the
///   magnitude it was taken at, `miss − ulps·2⁻⁵²·scale`: whether this
///   miss could be that rounding.
///
/// Positive on both is [`ReadBackIssue::Defect`]: a miss no rounding of
/// a sound construction reaches, on a scene that resolves it. Anything
/// else definite is [`ReadBackIssue::BelowSceneResolution`]. The
/// magnitude gate reads the AUTHORED data, so a construction that lost
/// digits and also inflated the magnitude it stored (a carrier flattened
/// to 1e12 m) still reads as the defect it is; the clearance reads the
/// STORED magnitude, so a sound arc whose stored form is legitimately
/// far out is never called one. A miss that is not finite (a rim past
/// 1.3e154 m overflows) is the scene's, never a defect. A difference in
/// the band, or poisoned, escalates. Not a verdict
/// (`k_stats::check_unlogged`): a box leaf has no row for it.
///
/// It shares [`resolves`]'s headroom and not its door: validation's
/// difference checks are logged verdicts whose refusal reports each
/// classification's [`MarginDiag`] (`decide_reported`), and these are
/// unlogged cross-checks at the witness lane, whose door reports a
/// sign.
fn read_back<T: Decide>(
    predicate: &'static str,
    resolution_predicate: &'static str,
    reading: Reading<T>,
    authored: impl FnOnce() -> T,
    stored: impl FnOnce() -> T,
    ulps: f64,
    band: Band,
) -> Result<(), ReadBackIssue<T>> {
    let miss = match reading {
        Reading::Passed => return Ok(()),
        Reading::Off(miss) => miss,
        Reading::Difference(miss) => match check_unlogged(predicate, miss, band, "F24") {
            Ok(Sign::Zero) => return Ok(()),
            Ok(Sign::Positive | Sign::Negative) => miss,
            Err(source) => return Err(ReadBackIssue::Escalated(source)),
        },
    };
    let authored = authored();
    let scale = stored().max(authored);
    let found = ArcMiss {
        predicate,
        miss: miss.abs(),
        scale,
        resolution: scale * T::from_f64(ulps * f64::EPSILON),
    };
    if !found.miss.is_bounded() {
        return Err(ReadBackIssue::BelowSceneResolution(found));
    }
    let clearance = found.miss - found.resolution;
    let margin = resolution_headroom(authored, ulps, band).min(clearance);
    match check_unlogged(resolution_predicate, margin, band, "F24") {
        Ok(Sign::Positive) => Err(ReadBackIssue::Defect(found)),
        Ok(Sign::Zero | Sign::Negative) => Err(ReadBackIssue::BelowSceneResolution(found)),
        Err(source) => Err(ReadBackIssue::Escalated(source)),
    }
}

/// Validation's third consistency check, `arc_sweep_range`, of a stored
/// arc's sweep against the sweeps `range` accepts ([`build_seg`] states
/// the margins, [`full_turn`] the full turn's): its arc length at the
/// radius, decided definitely nonzero (`arc_sweep_length`), and that
/// magnitude against one turn (`arc_sweep_range`, the turn rule), a
/// chord arc's strictly partial ([`Period::strictly_partial`]) and a full
/// turn's whole ([`Period::whole_turn`]).
pub(crate) fn sweep_range<T: Decide>(
    arc: Arc2<T>,
    range: Range,
    band: Band,
) -> Result<(), SegIssue<T>> {
    let inconsistent = || SegIssue::Inconsistent {
        check: ArcCheck::SweepRange,
    };
    let rate = InfSpeed::new(arc.radius);
    let (_, sweep) =
        Extent::decide_signed("arc_sweep_length", arc.sweep, rate, band).map_err(|source| {
            match source.margin.rejected_sign() {
                Some(Sign::Zero) => inconsistent(),
                _ => SegIssue::escalated(source),
            }
        })?;
    let (one_turn, rows, rate) = (
        Period::new(T::tau()),
        TurnRows::exact("arc_sweep_range"),
        Exact::new(arc.radius),
    );
    let accepted = match range {
        Range::Chord => one_turn.strictly_partial(rows, sweep, rate, band),
        Range::FullTurn => one_turn.whole_turn(rows, sweep, rate, band),
    }
    .map_err(|undecided| SegIssue::escalated(undecided.diag))?;
    if accepted {
        Ok(())
    } else {
        Err(inconsistent())
    }
}

/// **`chord_side`** — which side of a segment's (infinite) chord line a
/// point lies on. Margin: the signed perpendicular distance
/// perp_dot(û, q − a) (meters; positive = left of the chord direction).
///
/// Returns the margin beside the sign because one caller — the joint
/// classification — has to REPORT what it classified, and the others
/// (`line_line`'s four, [`arc_clear_of_carrier`]) discard it
/// deliberately: their question is which side, and a distance they
/// neither read nor render would be a second value to keep in step
/// with the first.
fn chord_side<T: Decide>(s: &Seg<T>, q: Point2<T>, band: Band) -> Result<(Sign, T), Indeterminate> {
    chord_side_reported(s, q, band).map(|(decided, margin)| (decided.sign, margin))
}

/// [`chord_side`], keeping the reading its classifier decided on.
fn chord_side_reported<T: Decide>(
    s: &Seg<T>,
    q: Point2<T>,
    band: Band,
) -> Result<(Decided, T), Indeterminate> {
    let margin = s.unit.perp_dot(q - s.a);
    Ok((
        decide_reported("chord_side", Margin::of(margin), band)?,
        margin,
    ))
}

/// **`circle_side`** — which side of an arc's carrier circle a point
/// lies on. Margin: |q − c| − r, the rim through [`Arc2::rim`]
/// (meters; positive = outside).
fn circle_side<T: Decide>(g: &ArcGeom<T>, q: Point2<T>, band: Band) -> Result<Sign, Indeterminate> {
    decide("circle_side", Margin::of(g.arc.rim(q) - g.arc.radius), band)
}

/// **`line_span`** — whether a point *known to lie on the carrier line*
/// lies within the segment's span. Margin: min(t, L − t) where
/// t = (q − a)·û is the arc-length parameter (meters along the carrier;
/// positive = strictly interior, Zero = at an endpoint, negative =
/// outside).
fn line_span<T: Decide>(s: &Seg<T>, q: Point2<T>, band: Band) -> Result<Sign, Indeterminate> {
    let t = (q - s.a).dot(s.unit);
    decide("line_span", Margin::of(t.min(s.len - t)), band)
}

/// **`arc_span`** — whether a point *known to lie on the carrier
/// circle* lies within the arc's span. Margin: |a − apex| − |q − apex|,
/// the chordal defect from the apex (meters; chord length is monotone
/// in angular distance up to π, and each half-arc spans |θ|/2 ≤ π, so
/// the comparison is exact in the reals). Positive = strictly on the
/// arc, Zero = at an endpoint, negative = on the complement arc.
///
/// Conditioning, stated honestly: near an endpoint the margin moves as
/// cos(|θ|/4)·(arc-length distance), so it degrades by 1/cos(θ/4) as
/// θ → 2π. The `arc_diameter_clearance` gate at segment construction
/// rejects arcs within band of a full circle, which bounds the
/// compression: surviving arcs have cos(θ/4) ≥ √(Kε/2r), so a Zero
/// here means the point is within ≈ √(2rε/K) of an endpoint *along the
/// carrier* — a √(rε)-scale endpoint zone, not the raw ε, and the
/// residual conditioning of chordal span testing (documented, not
/// hidden). No wrong-accept path exists through the zone: Zero routes
/// to endpoint-Touch contacts, which are errors for non-adjacent pairs
/// and are discounted only within ε of the actual shared vertex
/// (`contact_at_shared_vertex` is an uncompressed direct distance), and
/// an in-band reading of a candidate either escalates or hands it to
/// the segment's end ([`end_in_stretch`]), whose readings are direct
/// distances and do not depend on how far the zone reaches.
///
/// [`touches`] reads a segment end's radial foot here, so the same
/// compression is how far past the arc's end, along the carrier, an end
/// reads a touch or escalates: s* = 2r·asin(sin(θ/4) + Kε/2r) − rθ/2,
/// about Kε/cos(θ/4). That is 1.08Kε at a quarter turn, 1.41Kε at a
/// half, 2.6Kε at three quarters, 14Kε at 6 rad and 490Kε at 6.275 rad
/// (r = 1). Every such reading refuses or escalates, and none accepts.
///
/// A full turn ([`ArcSpan::FullTurn`]) holds every carrier point, so
/// nothing is decided: its vertex is not an endpoint.
///
/// [`arc_clear_of_carrier`] passes a point off the carrier, under its
/// own argument.
fn arc_span<T: Decide>(g: &ArcGeom<T>, q: Point2<T>, band: Band) -> Result<Sign, Indeterminate> {
    match g.span {
        ArcSpan::Chord { apex, span_chord } => {
            decide("arc_span", Margin::of(span_chord - q.distance(apex)), band)
        }
        ArcSpan::FullTurn => Ok(Sign::Positive),
    }
}

/// **`contact_at_shared_vertex`** (and other point-coincidence
/// questions, per the passed name) — margin: |p − q| (meters, a direct
/// displacement). Zero = coincident (D4's positive claim).
pub(crate) fn coincident<T: Decide>(
    name: &'static str,
    p: Point2<T>,
    q: Point2<T>,
    band: Band,
) -> Result<Sign, Indeterminate> {
    decide(name, Margin::of(p.distance(q)), band)
}

/// How the two carriers of a *joint* — adjacent segments at their
/// shared vertex — meet there (D1's profile-tangency classification
/// vocabulary).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum JointClass {
    /// Distinct carriers in first-order (tangent) contact. Tangent
    /// carriers share exactly one point, and the shared vertex lies on
    /// both, so the tangency *is* at the joint: a tangent joint.
    Tangent,
    /// Distinct carriers meeting transversally (or, defensively, a
    /// definitely-negative clearance, unreachable for carriers sharing
    /// a vertex): definitely not tangent. A constructed tangent joint
    /// here is contradicted.
    Transversal,
    /// One shared carrier (collinear line/line, cocircular arc/arc —
    /// e.g. the minimal two-arc circle's joints): *continuation*, a
    /// tangent joint like any other — every zero-turn joint is one (Ev,
    /// in-chat, 2026-09-02), because identity is a fact about the
    /// carriers and tangency a fact about the directions, and the
    /// directions agree here. Both readers act on that: the verify
    /// layer's joint pass and the path door's stored-form read.
    SameCarrier,
}

/// A classified joint: the class, plus the predicate the funnel stopped
/// at and the margin it classified. A caller refusing a joint names
/// what the classification named, rather than re-deriving a second
/// expression for the same question.
pub(crate) struct JointReading<T: Real> {
    /// What the two carriers do at the shared vertex.
    pub class: JointClass,
    /// The predicate that decided it.
    pub predicate: &'static str,
    /// The margin that predicate classified, meters.
    pub margin: T,
    /// The reading that predicate's classifier decided on.
    pub diag: MarginDiag,
    /// The largest magnitude the margin's arithmetic passes through —
    /// the carrier radii, and the COORDINATES the centres and anchors
    /// are given in. Both matter and for the same reason: a coordinate
    /// of size `M` is stored to about `M·2^-52`, and a difference of
    /// such coordinates inherits that however small the difference is,
    /// so the margin resolves only to about `scale·2^-52` whatever its
    /// own size. A band finer than that is reading rounding rather than
    /// geometry, and a refusal built on this reading says so with this
    /// number.
    pub scale: T,
    /// About the finest the margin could have been read: `scale·2⁻⁵²`
    /// for an identity reading; for a turn, each leg's heading rounded
    /// at that magnitude over the length it is taken across (a line's
    /// length, an arc's radius), levered by the joint's arm.
    pub resolution: T,
}

/// **`path_junction_side`** — whether a junction whose departure is
/// parallel to its arrival departs along it or REVERSES it (a cusp):
/// the alignment `cos φ` of the two unit headings, levered by the
/// arriving leg's arm. The one home of that question: the path door
/// asks it of a zero-turn junction it is about to refuse or construct,
/// and validation asks it of every tangent joint between distinct
/// carriers to record which are cusps
/// ([`crate::ValidatedLoop::cusp_joints`]).
///
/// `true` iff the alignment is definitely negative. A `Zero` reads as
/// NOT reversed — the arm itself is degenerate (both components
/// sub-ε), which the path door refuses as the tangent class and which
/// a validated loop cannot reach (its legs are definitely non-degenerate
/// and its tangent joints classified tangent, so the margin is ± the
/// arm).
pub(crate) fn junction_reverses<T: Decide>(
    arriving: Vec2<T>,
    departing: Vec2<T>,
    arm: T,
    band: Band,
) -> Result<bool, Indeterminate> {
    Ok(matches!(
        decide(
            "path_junction_side",
            Margin::levered(arriving.dot(departing), arm),
            band
        )?,
        Sign::Negative
    ))
}

/// [`Seg::heading_at`] on a segment of kind `kind` whose chord
/// direction is `unit`.
fn heading_on<T: Real>(kind: &SegKind<T>, p: Point2<T>, unit: Vec2<T>) -> Vec2<T> {
    match kind {
        SegKind::Line => unit,
        SegKind::Arc(g) => {
            let ccw = perp(p - g.arc.centre) * (T::one() / g.arc.radius);
            match g.turn {
                Sign::Negative => -ccw,
                Sign::Positive | Sign::Zero => ccw,
            }
        }
    }
}

/// **An arc leg's lever arm**: the smaller of its carrier's radius and
/// its chord. The radius is what an angular margin displaces over; the
/// chord bounds it for an arc shorter than its own radius, where the
/// radius would overstate how far the leg actually reaches.
pub(crate) fn arc_lever<T: Real>(radius: T, chord: T) -> T {
    radius.min(chord)
}

impl<T: Real> Seg<T> {
    /// The leg's lever arm at a junction: a line's length, an arc's
    /// [`arc_lever`], a full turn's radius.
    pub(crate) fn arm(&self) -> T {
        match &self.kind {
            SegKind::Line => self.len,
            SegKind::Arc(g) => match g.span {
                ArcSpan::Chord { .. } => arc_lever(g.arc.radius, self.len),
                ArcSpan::FullTurn => g.arc.radius,
            },
        }
    }

    /// The length a heading of this segment is taken across: a line's
    /// length (its unit is its chord over it), an arc's radius (its
    /// tangent is its radius vector over it).
    pub(crate) fn heading_basis(&self) -> T {
        match &self.kind {
            SegKind::Line => self.len,
            SegKind::Arc(g) => g.arc.radius,
        }
    }

    /// The unit heading of the traversal at `p`, one of the segment's
    /// endpoints: a line's chord direction; an arc's counterclockwise
    /// carrier tangent at `p`, reversed on a clockwise turn.
    pub(crate) fn heading_at(&self, p: Point2<T>) -> Vec2<T> {
        heading_on(&self.kind, p, self.unit)
    }
}

/// Which way a [`junction_turn`] reading keeps its sign.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TurnSense {
    /// `sin φ`: the path door authors a direction, so which way it
    /// turns is part of what it decided (a range sweep reads a sign
    /// change as a branch change).
    Signed,
    /// `|sin φ|`: validation judges a loop canonicalization may reverse,
    /// so a joint read along either traversal must decide, and record,
    /// the same margin.
    Unsigned,
}

/// **`path_junction_turn`** — whether a junction turns: the cross
/// product `sin φ` of the arriving and departing unit headings (its
/// magnitude under [`TurnSense::Unsigned`]), levered by `arm`, the
/// shorter of the two legs' arms ([`Seg::arm`]): the two legs lie side
/// by side only over the shorter one, so that is the displacement the
/// turn opens between them. Zero = the directions agree (a continuation
/// or a reversal, which [`junction_reverses`] tells apart); definite = a
/// corner; in band escalates. The one home of that question: the path
/// door asks it of every junction it builds, and of a declared seam
/// arrival under that check's own `name` (`path_seam_arrival_turn`);
/// [`joint_tangency`] asks it of every joint validation judges and
/// every stored fillet joint the door verifies. Returned with the
/// signed levered turn, which a refusal quotes.
pub(crate) fn junction_turn<T: Decide>(
    name: &'static str,
    arriving: Vec2<T>,
    departing: Vec2<T>,
    arm: T,
    sense: TurnSense,
    band: Band,
) -> Result<(Decided, T), Indeterminate> {
    let turn = arriving.perp_dot(departing);
    let read = match sense {
        TurnSense::Signed => turn,
        TurnSense::Unsigned => turn.abs(),
    };
    Ok((
        decide_reported(name, Margin::levered(read, arm), band)?,
        turn * arm,
    ))
}

/// The magnitude a segment's heading at its ends passes through: its
/// endpoints' coordinates, and an arc's centre and radius
/// ([`JointReading::scale`]).
fn heading_scale<T: Real>(s: &Seg<T>) -> T {
    let ends = reach(s.a).max(reach(s.b));
    match &s.kind {
        SegKind::Line => ends,
        SegKind::Arc(g) => ends.max(g.arc.radius).max(reach(g.arc.centre)),
    }
}

/// Classifies the joint between two adjacent segments — `prev` arrives
/// at the shared vertex, `next` leaves it. The single per-junction
/// tangency question, kept separate from profile-wide validation so an
/// authoring layer can ask it per-junction.
///
/// Carrier identity is asked first, because it is a fact about the
/// carriers: line/line on `chord_side` of the next leg's far end
/// against the arriving carrier (the shared vertex is on both, so
/// identity ⟺ that end lies on it), arc/arc on
/// `carrier_circles_identity`. Every other joint is decided by its
/// turn ([`junction_turn`], `path_junction_turn`), because tangency is
/// a fact about the directions: Zero ⇒ [`JointClass::Tangent`],
/// definite ⇒ [`JointClass::Transversal`], in band or poisoned ⇒
/// escalated verbatim. The turn is levered by the shorter of the two
/// legs' arms ([`Seg::arm`]), as the path door levers it: the legs lie
/// side by side only over the shorter one, and the minimum does not
/// depend on which way the loop is traversed.
pub(crate) fn joint_tangency<T: Decide>(
    prev: &Seg<T>,
    next: &Seg<T>,
    turn_name: &'static str,
    band: Band,
) -> Result<JointReading<T>, Indeterminate> {
    let scale = heading_scale(prev).max(heading_scale(next));
    let ulp = scale * T::from_f64(f64::EPSILON);
    match (&prev.kind, &next.kind) {
        (SegKind::Line, SegKind::Line) => {
            let (side, margin) = chord_side_reported(prev, next.b, band)?;
            if side.sign == Sign::Zero {
                return Ok(JointReading {
                    class: JointClass::SameCarrier,
                    predicate: "chord_side",
                    margin,
                    diag: side.margin,
                    scale,
                    resolution: ulp,
                });
            }
        }
        (SegKind::Arc(g1), SegKind::Arc(g2)) => {
            let identity = carrier_pair(g1, g2).pair().identity();
            let same = decide_reported("carrier_circles_identity", Margin::of(identity), band)?;
            if same.sign != Sign::Positive {
                return Ok(JointReading {
                    class: JointClass::SameCarrier,
                    predicate: "carrier_circles_identity",
                    margin: identity,
                    diag: same.margin,
                    scale,
                    resolution: ulp,
                });
            }
        }
        (SegKind::Line, SegKind::Arc(_)) | (SegKind::Arc(_), SegKind::Line) => {}
    }
    let arm = prev.arm().min(next.arm());
    let (turn, margin) = junction_turn(
        turn_name,
        prev.heading_at(prev.b),
        next.heading_at(next.a),
        arm,
        TurnSense::Unsigned,
        band,
    )?;
    Ok(JointReading {
        class: match turn.sign {
            Sign::Zero => JointClass::Tangent,
            Sign::Positive | Sign::Negative => JointClass::Transversal,
        },
        predicate: turn_name,
        margin,
        diag: turn.margin,
        scale,
        resolution: ulp * arm * (T::one() / prev.heading_basis() + T::one() / next.heading_basis()),
    })
}

/// **`carrier_line_circle`'s margin, in one place**: the clearance
/// `r − |h|` between a line's carrier and a circle's, where
/// `h = perp_dot(û, C − a)` is the centre's signed offset from the
/// line. Returned with the SCALE its arithmetic passes through — the
/// radius and the two points' own coordinate magnitudes — because both
/// the subtraction and the `C − a` inside `h` cancel first-order in
/// that magnitude, and the clearance therefore resolves only to about
/// `scale·2^-52` however small it is. A caller reporting a refusal can
/// then say how finely this margin could have been read at all.
///
/// Every reader of this clearance goes through here — the pair contact
/// and the ray cast — so the two cannot drift apart.
pub(crate) fn carrier_line_circle_margin<T: Real>(
    unit: Vec2<T>,
    from: Point2<T>,
    g: &ArcGeom<T>,
) -> (T, T) {
    let h = unit.perp_dot(g.arc.centre - from).abs();
    (
        g.arc.radius - h,
        g.arc.radius.max(reach(g.arc.centre)).max(reach(from)),
    )
}

/// A point's coordinate magnitude — the largest `|x|`, `|y|`, which is
/// the size its stored representation rounds at. The infinity norm and
/// not the Euclidean one on purpose: rounding is per coordinate, and
/// this is read only to SCALE a resolution, never to compare lengths.
pub(crate) fn reach<T: Real>(p: Point2<T>) -> T {
    Vec2::from_array(p.to_array()).norm_inf()
}

/// The kind of an isolated contact between two segments.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CKind {
    /// Contact at a segment endpoint (of at least one of the two).
    Touch,
    /// Transversal contact interior to both segments.
    Crossing,
    /// Tangential carrier contact interior to both segments (touching
    /// without crossing — semantically indeterminate for simplicity;
    /// the caller escalates it as a typed error).
    Tangency,
}

/// One isolated contact point.
pub(crate) struct Contact<T: Real> {
    /// Where the segments meet.
    pub point: Point2<T>,
    /// How they meet.
    pub kind: CKind,
}

/// The classified contact set of a segment pair.
pub(crate) enum PairOutcome<T: Real> {
    /// Finitely many isolated contacts (possibly none; duplicates
    /// allowed — the same touch seen from both segments).
    Contacts(Vec<Contact<T>>),
    /// A shared sub-locus of positive length (collinear or cocircular
    /// overlap).
    Overlap,
}

/// Classifies every contact between two segments — the simplicity core.
/// Dispatches on carrier kinds; all closed form. Errors are in-band /
/// poisoned classifications, propagated for the caller to wrap (D4 ¶3).
/// `shared` is a stored vertex the pair shares, an end of each (an
/// adjacent pair, `validate::judge_pair`); only two lines read it, where
/// their sides escalate ([`line_line`]).
pub(crate) fn pair_contacts<T: Decide>(
    s1: &Seg<T>,
    s2: &Seg<T>,
    shared: Option<Point2<T>>,
    band: Band,
) -> Result<PairOutcome<T>, Indeterminate> {
    match (&s1.kind, &s2.kind) {
        (SegKind::Line, SegKind::Line) => line_line(s1, s2, shared, band),
        (SegKind::Line, SegKind::Arc(g2)) => line_arc(s1, s2, g2, band),
        (SegKind::Arc(g1), SegKind::Line) => line_arc(s2, s1, g1, band),
        (SegKind::Arc(g1), SegKind::Arc(g2)) => arc_arc(s1, g1, s2, g2, band),
    }
}

/// Joint span membership of a candidate contact point: `None` = not a
/// contact (outside a span), `Interior` = strictly interior to both,
/// `Boundary` = at an endpoint of at least one.
enum Joint {
    Interior,
    Boundary,
}

/// A point definitely outside either span is no contact at that point,
/// whatever the other span reads, so `m2` is not read past a definite
/// miss of `m1`; only then does an indeterminate reading escalate.
fn joint(
    m1: Result<Sign, Indeterminate>,
    m2: impl FnOnce() -> Result<Sign, Indeterminate>,
) -> Result<Option<Joint>, Indeterminate> {
    if let Ok(Sign::Negative) = m1 {
        return Ok(None);
    }
    match (m1, m2()) {
        (_, Ok(Sign::Negative)) => Ok(None),
        (Err(source), _) | (_, Err(source)) => Err(source),
        (Ok(Sign::Positive), Ok(Sign::Positive)) => Ok(Some(Joint::Interior)),
        _ => Ok(Some(Joint::Boundary)),
    }
}

/// Whether two straight segments' projections lie apart past the band
/// on either one's axis (the **`segment_gap`** predicate, [`line_line`]):
/// on `s1`'s axis first, then on `s2`'s where `s1`'s did not decide.
/// Either axis apart holds every pair of points apart, so the reading is
/// the pair's, whichever segment is named first, as check 9's
/// (`topo::validate`'s `gaps_apart`) is.
fn gaps_apart<T: Decide>(s1: &Seg<T>, s2: &Seg<T>, band: Band) -> bool {
    let apart = |a: &Seg<T>, b: &Seg<T>| {
        let along = on_one_line_2d(a.unit, (a.a, a.b), (b.a, b.b), T::zero());
        decide("segment_gap", Margin::of(along.apart()), band) == Ok(Sign::Positive)
    };
    apart(s1, s2) || apart(s2, s1)
}

/// Whether two straight segments that share a stored vertex, an end of
/// each, can meet only within the band of it: `other` read along
/// `host`'s axis ([`geom_core::one_line`]), on the
/// **`collinear_overlap`** predicate, where the overlap's upper bound
/// (`extent + noise`) reads no more than Zero.
///
/// The shared vertex is an end of both, so its projection lies in both
/// projected intervals, and the true overlap contains it and is no
/// longer than that bound. A point on both segments projects into the
/// overlap, and along `host` its projection is its distance from
/// `host`'s end, so every such point is within the band of the shared
/// vertex: the vertex's own touch. No angle between the two enters, so
/// it holds at a straight joint, a turned one and a reversal alike.
///
/// [`line_line`] asks it only where a carrier side has escalated, so a
/// decided verdict never moves. That pair's joint is then read in band
/// too: the short leg's far end stands off the long one's carrier by
/// `sin θ` times the short leg, the turn [`joint_tangency`] reads levered
/// by the shorter arm. So the escalation moves to the joint, the question
/// it is, and a refusal later in the pass is read. The collinear arm's
/// straddle is not asked: there the joint can read a continuation, and
/// the pair would validate where the overlap's charge, far from the
/// origin, passes the band.
///
/// topo's check 9 (`lines_meet`) has no such pair to read: it pairs a
/// ring's edges with its outer loop's, and a vertex the two loops share
/// is its arm 1's contact before any edge pair is read.
fn touches_only_near_the_shared_end<T: Decide>(host: &Seg<T>, other: &Seg<T>, band: Band) -> bool {
    let along = on_one_line_2d(host.unit, (host.a, host.b), (other.a, other.b), T::zero());
    matches!(
        decide(
            "collinear_overlap",
            Margin::of(along.extent() + along.noise),
            band
        ),
        Ok(Sign::Zero | Sign::Negative)
    )
}

/// The collinear arm of [`line_line`]: `other` read along `host`'s axis,
/// through the one reading of two spans on one line
/// ([`geom_core::one_line`]), on the **`collinear_overlap`** predicate —
/// margin: the overlap's signed length (meters; positive = overlap, Zero
/// = endpoint touch, negative = disjoint), decided past its charge both
/// ways. An overlap where even its lower bound is definitely positive;
/// disjoint where even its upper bound is definitely negative; a touch,
/// at the overlap's middle, where both read zero; and the bounds'
/// straddle otherwise.
fn collinear_overlap<T: Decide>(
    host: &Seg<T>,
    other: &Seg<T>,
    band: Band,
) -> Result<PairOutcome<T>, Indeterminate> {
    let along = on_one_line_2d(host.unit, (host.a, host.b), (other.a, other.b), T::zero());
    let lower = decide_reported(
        "collinear_overlap",
        Margin::of(along.extent() - along.noise),
        band,
    )?;
    if lower.sign == Sign::Positive {
        return Ok(PairOutcome::Overlap);
    }
    let upper = decide_reported(
        "collinear_overlap",
        Margin::of(along.extent() + along.noise),
        band,
    )?;
    Ok(match (lower.sign, upper.sign) {
        (Sign::Negative, Sign::Negative) => PairOutcome::Contacts(Vec::new()),
        (Sign::Zero, Sign::Zero) => PairOutcome::Contacts(vec![Contact {
            point: host.a + host.unit * along.mid(),
            kind: CKind::Touch,
        }]),
        _ => {
            return Err(geom_core::k_stats::straddle(
                "collinear_overlap",
                lower,
                upper,
                band,
            ));
        }
    })
}

/// Line/line contacts, read as check 9 reads two straight edges
/// (`topo::validate`'s `lines_meet`), so the two doors give one answer.
///
/// - First the **`segment_gap`** predicate ([`gaps_apart`]): the gap
///   between the two spans' projections on either one's axis, past its
///   rounding ([`geom_core::one_line`]; meters, positive = apart). Spans
///   apart along either axis past the band hold every pair of points
///   apart by as much, so the pair has no contact whatever its sides
///   would read. The four points are the loop's stored vertices
///   (`build_loop_seg`), the geometry the simplicity pass reads, so
///   nothing here computed them and none is charged for its making. A
///   pair with a `shared` vertex is not read: that vertex projects into
///   both spans on either axis, so their gap is zero by construction,
///   never past the band.
/// - Then the four `chord_side` orientations (the Shewchuk-style segment
///   test, trileanized), each end against the other's carrier. Both of
///   one segment's ends definitely on one side of the other's carrier
///   separates them, so the pair has no contact, however the other two
///   sides read.
/// - Both of one segment's ends on the other's carrier ⇒ collinear,
///   resolved by [`collinear_overlap`] along that carrier's axis, however
///   the other pair of sides reads: the whole segment is within the band
///   of that carrier.
/// - Only then does an in-band side escalate. Strictly opposite sides
///   both ways ⇒ one proper crossing; any Zero ⇒ endpoint-on-carrier
///   configurations resolved by `line_span`.
fn line_line<T: Decide>(
    s1: &Seg<T>,
    s2: &Seg<T>,
    shared: Option<Point2<T>>,
    band: Band,
) -> Result<PairOutcome<T>, Indeterminate> {
    if shared.is_none() && gaps_apart(s1, s2, band) {
        return Ok(PairOutcome::Contacts(Vec::new()));
    }
    let side = |host: &Seg<T>, q: Point2<T>| chord_side(host, q, band).map(|(sign, _)| sign);
    let on_1 = [side(s1, s2.a), side(s1, s2.b)];
    let on_2 = [side(s2, s1.a), side(s2, s1.b)];
    let separated = |pair: &[Result<Sign, Indeterminate>; 2]| {
        matches!(
            pair,
            [Ok(Sign::Positive), Ok(Sign::Positive)] | [Ok(Sign::Negative), Ok(Sign::Negative)]
        )
    };
    if separated(&on_1) || separated(&on_2) {
        return Ok(PairOutcome::Contacts(Vec::new()));
    }
    // Both of one segment's ends within the band of the other's carrier
    // put all of it there, so the overlap along that carrier decides the
    // pair, whatever the other pair of sides reads.
    let collinear =
        |pair: &[Result<Sign, Indeterminate>; 2]| matches!(pair, [Ok(Sign::Zero), Ok(Sign::Zero)]);
    if collinear(&on_1) {
        return collinear_overlap(s1, s2, band);
    }
    if collinear(&on_2) {
        return collinear_overlap(s2, s1, band);
    }
    let sides = (|| Ok([on_1[0]?, on_1[1]?, on_2[0]?, on_2[1]?]))();
    let [o_c, o_d, o_a, o_b] = match sides {
        Ok(sides) => sides,
        // An in-band side of an adjacent pair whose overlap along `s1`
        // is within the band: its only contact is the shared vertex.
        Err(_)
            if let Some(v) = shared
                && touches_only_near_the_shared_end(s1, s2, band) =>
        {
            return Ok(PairOutcome::Contacts(vec![Contact {
                point: v,
                kind: CKind::Touch,
            }]));
        }
        Err(source) => return Err(source),
    };
    let opposite = |x: Sign, y: Sign| {
        matches!(
            (x, y),
            (Sign::Positive, Sign::Negative) | (Sign::Negative, Sign::Positive)
        )
    };
    let mut contacts = Vec::new();
    if opposite(o_c, o_d) && opposite(o_a, o_b) {
        // Proper crossing, interior to both; the crossing parameter is
        // well-conditioned here (definite sides bound the transversal
        // angle away from zero).
        let denom = s2.chord.perp_dot(s1.chord);
        let t = s2.chord.perp_dot(s2.a - s1.a) / denom;
        contacts.push(Contact {
            point: s1.a + s1.chord * t,
            kind: CKind::Crossing,
        });
    } else {
        // Non-collinear with some endpoint on a carrier: the carriers
        // meet exactly once, at that endpoint, read by [`touches`]' rule
        // from the side read above.
        for (host, o, q) in [
            (s1, o_c, s2.a),
            (s1, o_d, s2.b),
            (s2, o_a, s1.a),
            (s2, o_b, s1.b),
        ] {
            if touches_from(host, q, Ok(o), band)? {
                contacts.push(Contact {
                    point: q,
                    kind: CKind::Touch,
                });
            }
        }
    }
    Ok(PairOutcome::Contacts(contacts))
}

/// Line/arc contacts via the **`carrier_line_circle`** predicate —
/// margin: r − |h| where h = perp_dot(û, C − a) is the center's signed
/// distance to the carrier line (meters: the clearance between the line
/// and the circle; this is the tangency question, and its lever-arm
/// story is the D4 ¶1 one — an angular tangency deviation φ at contact
/// radius r displaces the carriers by ≈ r·φ²/2, so the *displacement*
/// is the honest margin). Negative = the carriers miss; Zero = tangent
/// (one candidate point, the foot of the perpendicular); Positive =
/// secant (two candidates, foot ± √(r² − h²) along the line — the
/// radicand is a product of two definitely-positive factors
/// (r − |h|)(r + |h|), so it cannot poison in this branch).
///
/// An in-band clearance is a carrier question the segments need not
/// ask: where [`arc_clear_of_carrier`] certifies the arc away from the
/// whole line, or [`segment_clear_of_circle`] the segment away from the
/// whole circle, there is no contact; otherwise it escalates. The
/// latter reads **`circle_foot_past_segment`** and
/// **`circle_end_clear`**, in that order, only on that `Err`.
fn line_arc<T: Decide>(
    line: &Seg<T>,
    arc: &Seg<T>,
    g: &ArcGeom<T>,
    band: Band,
) -> Result<PairOutcome<T>, Indeterminate> {
    let to_center = g.arc.centre - line.a;
    let h = line.unit.perp_dot(to_center);
    let tc = to_center.dot(line.unit);
    let foot = line.a + line.unit * tc;
    let (clearance, _) = carrier_line_circle_margin(line.unit, line.a, g);
    let mut contacts = Vec::new();
    let carriers = match decide("carrier_line_circle", Margin::of(clearance), band) {
        Ok(sign) => sign,
        Err(_)
            if arc_clear_of_carrier(line, arc, g, foot, band)
                || segment_clear_of_circle(line, g, tc, band) =>
        {
            return Ok(PairOutcome::Contacts(contacts));
        }
        Err(source) => return Err(source),
    };
    let spans = |q| (line_span(line, q, band), arc_span(g, q, band));
    match carriers {
        Sign::Negative => {}
        Sign::Zero => {
            candidates(
                &mut contacts,
                (line, arc),
                &[foot],
                CKind::Tangency,
                spans,
                Settle::InStretch,
                band,
            )?;
        }
        Sign::Positive => {
            let half = (g.arc.radius.powi(2) - h.powi(2)).sqrt();
            let crossings = [tc - half, tc + half].map(|t| line.a + line.unit * t);
            candidates(
                &mut contacts,
                (line, arc),
                &crossings,
                CKind::Crossing,
                spans,
                Settle::InStretch,
                band,
            )?;
        }
    }
    Ok(PairOutcome::Contacts(contacts))
}

/// Whether an arc certainly keeps off a line's whole carrier, read from
/// the arc's span rather than the carriers' clearance. Along a circle
/// the signed distance to a line has one local minimum, at the point P
/// facing the line, which lies toward `foot` (the line's point nearest
/// the centre) from the centre. An arc whose span excludes P is
/// therefore nearest the line at an endpoint, and with both endpoints
/// definitely on the centre's side (`chord_side`) no point of it
/// reaches the line.
///
/// The span is read at `foot`, which is off the carrier unless the
/// carriers are tangent, so a definite miss of `foot` has to be shown
/// to exclude P. In the reals: where the line misses the circle every
/// arc point is on the centre's side; where it is tangent, `foot` is P.
/// Where it cuts the circle, an arc holding P with both endpoints on
/// the centre's side holds the whole cap beyond the line, so both
/// crossing points lie in the disc of radius `span_chord` about the
/// apex, and so does `foot`, their midpoint; `arc_span` would not read
/// it a definite miss. That carries to `Interval`, whose definite
/// readings hold of the real points. At `f64` the clearance is in
/// band, so |`foot` − P| < Kε, and the margin, 1-Lipschitz in the
/// point, moves by less than the Kε a definite miss clears.
fn arc_clear_of_carrier<T: Decide>(
    line: &Seg<T>,
    arc: &Seg<T>,
    g: &ArcGeom<T>,
    foot: Point2<T>,
    band: Band,
) -> bool {
    let side = |q: Point2<T>| chord_side(line, q, band).map(|(sign, _)| sign);
    matches!(arc_span(g, foot, band), Ok(Sign::Negative))
        && match side(g.arc.centre) {
            Ok(centre @ (Sign::Positive | Sign::Negative)) => {
                matches!(side(arc.a), Ok(s) if s == centre)
                    && matches!(side(arc.b), Ok(s) if s == centre)
            }
            Ok(Sign::Zero) | Err(_) => false,
        }
}

/// Whether a line segment certainly keeps off an arc's whole carrier,
/// read from the segment's span rather than the carriers' clearance: the
/// line's side of [`arc_clear_of_carrier`]. Along a line the distance to
/// a circle's centre has one local minimum, at `foot`, the line's point
/// nearest the centre. A segment whose span definitely misses `foot` is
/// therefore nearest the centre at an end, and with both ends
/// definitely outside the circle ([`end_clear_of_circle`]) no point of
/// it reaches the circle.
///
/// **The miss, charged** (**`circle_foot_past_segment`**). The foot is
/// read by its parameter `tc = (c − a)·û` along the segment, computed,
/// so the miss is `line_span`'s margin `min(tc, L − tc)` with `tc`'s
/// making added to it: it must read definitely negative with
/// [`rounding_charge`](geom_core::rounding_charge) of `tc`'s production
/// (the difference `c − a` at `c`'s and `a`'s coordinate magnitudes, the
/// dot product at `|c − a|`) and of the span margin's own arithmetic
/// (`L`). At `Interval` the readings hold of the real points and the
/// charge only widens them.
fn segment_clear_of_circle<T: Decide>(line: &Seg<T>, g: &ArcGeom<T>, tc: T, band: Band) -> bool {
    let made = (g.arc.centre - line.a).norm() + reach(line.a) + reach(g.arc.centre) + line.len;
    let miss = tc.min(line.len - tc) + geom_core::rounding_charge(made);
    decide("circle_foot_past_segment", Margin::of(miss), band) == Ok(Sign::Negative)
        && [line.a, line.b]
            .into_iter()
            .all(|q| end_clear_of_circle(g, q, true, band))
}

/// Whether a segment's end lies definitely outside a circle, or with
/// `outside` false definitely inside it (**`circle_end_clear`**,
/// positive = outside): `circle_side`'s margin `|q − c| − r`, charged.
/// The end is stored, but the margin is not: the difference `q − c`,
/// its norm and the subtraction of `r` round at the coordinates' and the
/// radius's magnitude. Inside the arcs' reach that rounding stays below
/// the band's room, but a line's end has no such reach, and far out it
/// passes it, so the margin must read definite with
/// [`rounding_charge`](geom_core::rounding_charge) of `|q|`, `|c|` and
/// `r` taken against it.
fn end_clear_of_circle<T: Decide>(g: &ArcGeom<T>, q: Point2<T>, outside: bool, band: Band) -> bool {
    let charge = geom_core::rounding_charge(reach(q) + reach(g.arc.centre) + g.arc.radius);
    let rim = g.arc.rim(q) - g.arc.radius;
    let (margin, side) = if outside {
        (rim - charge, Sign::Positive)
    } else {
        (rim + charge, Sign::Negative)
    };
    decide("circle_end_clear", Margin::of(margin), band) == Ok(side)
}

/// Whether an arc certainly keeps off another circle, read from the
/// arc's span rather than the carriers' clearances; `toward` is the unit
/// from the arc's centre to the other's. Along a circle the distance to
/// another centre has one local minimum, at the point facing it, and one
/// maximum, opposite. An arc whose span definitely excludes the nearest
/// is nearest the other centre at an end, so with both ends definitely
/// outside the other circle ([`end_clear_of_circle`]) no point of it
/// reaches that circle; one excluding the farthest is farthest at an end, so with
/// both ends definitely inside, no point of it does either.
///
/// **The exclusion, charged** (**`circle_extreme_past_arc`**). The two
/// extremes `c ± r·û` are computed, so each is read as `arc_span`'s
/// margin `|a − apex| − |q − apex|` with its making added: it must read
/// definitely negative with [`rounding_charge`](geom_core::rounding_charge)
/// of the point's production (the centre's coordinate magnitude, `r`
/// through `û`, itself a quotient by the centres' distance) and of the
/// span margin's arithmetic (the span chord and `|q − apex|`, each at
/// most `2r`). `toward` divides by the centres' distance, so it is read
/// only where the identity margin was definite ([`arc_arc`]).
fn arc_clear_of_circle<T: Decide>(
    arc: &Seg<T>,
    g: &ArcGeom<T>,
    toward: Vec2<T>,
    other: &ArcGeom<T>,
    band: Band,
) -> bool {
    let ArcSpan::Chord { apex, span_chord } = g.span else {
        // A full turn holds both extremes.
        return false;
    };
    let ends = |outside: bool| {
        [arc.a, arc.b]
            .into_iter()
            .all(|q| end_clear_of_circle(other, q, outside, band))
    };
    let excludes = |q: Point2<T>| {
        let made = reach(g.arc.centre) + reach(q) + g.arc.radius * T::from_f64(5.0);
        let miss = span_chord - q.distance(apex) + geom_core::rounding_charge(made);
        decide("circle_extreme_past_arc", Margin::of(miss), band) == Ok(Sign::Negative)
    };
    let arm = toward * g.arc.radius;
    (excludes(g.arc.centre + arm) && ends(true)) || (excludes(g.arc.centre - arm) && ends(false))
}

/// Arc/arc contacts. Predicates, in gate order, all margins in meters:
///
/// - **`carrier_circles_identity`** — margin: |C₂ − C₁| + ||r₁| − |r₂||
///   (the carrier distance in "same circle" terms). Zero ⇒ cocircular:
///   contact is span overlap on the shared carrier, resolved by
///   `arc_span` of each other's endpoints plus the
///   **`arc_apex_identity`** coincidence (identical spans have
///   coincident apexes; complementary spans have antipodal ones).
///   In-band ⇒ nearly-identical carriers — a genuine sliver, escalated.
/// - **`carrier_circles_external`** — margin: d − (r₁ + r₂), the
///   external clearance g. Positive ⇒ separate; Zero ⇒ externally
///   tangent.
/// - **`carrier_circles_internal`** — margin: d − |r₁ − r₂|, the
///   internal clearance g. Negative ⇒ nested, no contact; Zero ⇒
///   internally tangent to within ε, which the next margin reads;
///   Positive (with external Negative) ⇒ two proper intersections, on
///   the radical line ([`Circles2::crossings`]), whose half-chord's
///   square is a product of four factors: the two clearances, each
///   definite in this branch, and `d + |r₁ − r₂|` and `d + r₁ + r₂`, so
///   it cannot poison.
/// - **`carrier_circles_chord`** — margin: the radical foot's depth
///   inside both circles, the mean of rᵢ − |foot − Cᵢ|, the sagitta of
///   the chord the circles cut, or zero where the foot stands outside
///   them. Read where the internal clearance is Zero, which bounds g by
///   ε but not the chord: the foot stands g·r₂/d and g·r₁/d off the
///   circles (the two differ by g), so on near-concentric carriers a g
///   within ε cuts a chord whose ends lie far apart. Positive ⇒ two
///   crossings, [`Circles2::crossings`] as for a proper secant, whose
///   half-chord's square has the sign of g, which the depth past Kε
///   puts definitely positive; Zero ⇒ a tangency: the circles cross, if
///   at all, within √(2r·ε) of the foot, or are nested. In band the
///   pair is read both ways, and where the two readings agree on
///   whether the arcs meet away from their ends, it takes the contacts
///   of both: each is a point within ε of both arcs.
///
///   **What the band costs.** Where the readings disagree, the pair
///   escalates: a crossing pair and a tangency cannot be told apart
///   there, and which one the data is decides the verdict. That is two
///   arcs leaving a point near the tangency direction the same way, a
///   cusp among them, on carriers whose g lies within a factor
///   2d/(r₁ + r₂) of ε (d = 0.1, g from 0.2ε to ε, at r = 1). A smooth
///   joint, or a pair whose arcs hold neither crossing, reads the same
///   both ways. The escalation is sized: its ending offers the
///   tolerance at which the sagitta decides, and the near-tangency note
///   offers the PATHS `.fillet(r)` door, which constructs an exact
///   tangency.
///
/// The margins and the crossings read one [`Circles2`], so the factors
/// the crossings take are the margins decided, bit for bit.
///
/// An in-band external or internal reading is a carrier question
/// the arcs need not ask: where [`arc_clear_of_circle`] certifies either
/// arc away from the other's whole circle, there is no contact; otherwise
/// it escalates. That certificate reads **`circle_extreme_past_arc`**
/// and **`circle_end_clear`**, in that order, only on such an `Err`.
/// It is not asked where the identity margin is in band:
/// arcs on nearly one circle stand within about ε of each other's circle
/// everywhere, so no radial margin separates them.
///
/// A tangency's candidate is the midpoint of the circles' nearest
/// points, which lie on the line of centres |g| apart, so it stands
/// |g|/2 off each and both spans read it as a carrier point, however
/// far the near-concentric radical foot falls from either circle.
///
/// Divisions by d occur only in branches where the identity margin was
/// definitely positive with the relevant clearances pinned, so d is
/// bounded away from zero there (documented per branch).
fn arc_arc<T: Decide>(
    s1: &Seg<T>,
    g1: &ArcGeom<T>,
    s2: &Seg<T>,
    g2: &ArcGeom<T>,
    band: Band,
) -> Result<PairOutcome<T>, Indeterminate> {
    let circles = carrier_pair(g1, g2);
    let pair = circles.pair();
    match decide(
        "carrier_circles_identity",
        Margin::of(pair.identity()),
        band,
    )? {
        Sign::Zero | Sign::Negative => {
            // Cocircular: span overlap on the shared carrier. Every end
            // lies on it, so the span alone decides each, and an end
            // strictly inside the other span is an overlap, not the
            // touch [`touches`] would read.
            let mut contacts = Vec::new();
            for (host, q) in [(g1, s2.a), (g1, s2.b), (g2, s1.a), (g2, s1.b)] {
                match arc_span(host, q, band)? {
                    Sign::Positive => return Ok(PairOutcome::Overlap),
                    Sign::Zero => contacts.push(Contact {
                        point: q,
                        kind: CKind::Touch,
                    }),
                    Sign::Negative => {}
                }
            }
            // A full turn holds every carrier point, so a pair with
            // one never reaches here: the loop above found an overlap.
            if let (ArcSpan::Chord { apex: apex1, .. }, ArcSpan::Chord { apex: apex2, .. }) =
                (&g1.span, &g2.span)
                && coincident("arc_apex_identity", *apex1, *apex2, band)? == Sign::Zero
            {
                // Same endpoints AND same apex: identical spans.
                return Ok(PairOutcome::Overlap);
            }
            Ok(PairOutcome::Contacts(contacts))
        }
        Sign::Positive => {
            let mut contacts = Vec::new();
            let spans = |q| (arc_span(g1, q, band), arc_span(g2, q, band));
            // A tangency's candidate: the midpoint of the circles'
            // nearest points c₁ + û·s₁ and c₂ + û·s₂, where each arm has
            // bounded d away from zero.
            let tangency = |s1: T, s2: T| {
                let u = circles.direction();
                let (p1, p2) = (g1.arc.centre + u * s1, g2.arc.centre + u * s2);
                p1 + (p2 - p1) * T::from_f64(0.5)
            };
            // Either arc certainly off the other's circle is no contact,
            // however the carriers' clearances read; the identity margin
            // was definite, so `d` is bounded away from zero.
            let clear = || {
                let u = circles.direction();
                arc_clear_of_circle(s1, g1, u, g2, band)
                    || arc_clear_of_circle(s2, g2, -u, g1, band)
            };
            let no_contact = || Ok(PairOutcome::Contacts(Vec::new()));
            let external = match decide(
                "carrier_circles_external",
                Margin::of(pair.external()),
                band,
            ) {
                Err(_) if clear() => return no_contact(),
                reading => reading?,
            };
            match external {
                Sign::Positive => {}
                Sign::Zero => {
                    // Externally tangent; d = r₁ + r₂ ≥ the definite
                    // identity margin, so the division is safe.
                    let q = tangency(g1.arc.radius, -g2.arc.radius);
                    candidates(
                        &mut contacts,
                        (s1, s2),
                        &[q],
                        CKind::Tangency,
                        spans,
                        Settle::InStretch,
                        band,
                    )?;
                }
                Sign::Negative => {
                    let internal = match decide(
                        "carrier_circles_internal",
                        Margin::of(pair.internal()),
                        band,
                    ) {
                        Err(_) if clear() => return no_contact(),
                        reading => reading?,
                    };
                    match internal {
                        Sign::Negative => {}
                        Sign::Zero => {
                            // Internally tangent: d = |Δr| and the
                            // identity margin d + |Δr| = 2d is definite,
                            // so d is bounded away from zero. The foot
                            // lies beyond the smaller circle's centre,
                            // and `radical` is its signed distance
                            // from c₁, about ±r₁.
                            let foot = circles.foot();
                            let depth = (g1.arc.radius - g1.arc.rim(foot) + g2.arc.radius
                                - g2.arc.rim(foot))
                                * T::from_f64(0.5);
                            let read = |points: &[Point2<T>], kind, settle| {
                                let mut found = Vec::new();
                                candidates(&mut found, (s1, s2), points, kind, spans, settle, band)
                                    .map(|()| found)
                            };
                            let tangent = || {
                                let side = |r: T| pair.radical().select_le_zero(-r, r);
                                let q = tangency(side(g1.arc.radius), side(g2.arc.radius));
                                read(&[q], CKind::Tangency, Settle::InStretch)
                            };
                            // The carriers stay within ε between the
                            // crossings, so an end there is on both.
                            let secant = || {
                                read(&circles.crossings(), CKind::Crossing, Settle::ApartFromEnds)
                            };
                            // Only a chord definitely deeper than the
                            // band changes the reading, so a foot outside
                            // the circles reads as the tangency it is.
                            let chord = decide(
                                "carrier_circles_chord",
                                Margin::of(depth.max(T::zero())),
                                band,
                            );
                            contacts = match chord {
                                Ok(Sign::Positive) => secant()?,
                                Ok(Sign::Zero | Sign::Negative) => tangent()?,
                                Err(undecided) => {
                                    let interior = |found: &[Contact<T>]| {
                                        found.iter().any(|c| !matches!(c.kind, CKind::Touch))
                                    };
                                    // No certificate is asked here: an arc
                                    // clear of the other's circle keeps every
                                    // candidate, a point on or within ε of
                                    // that circle, definitely off its span, so
                                    // both readings already agree on no
                                    // contact. Two million near-concentric
                                    // draws reached no disagreement with an
                                    // arc clear (PR 5552's review fold).
                                    match (tangent(), secant()) {
                                        (Ok(t), Ok(x)) if interior(&t) == interior(&x) => {
                                            t.into_iter().chain(x).collect()
                                        }
                                        _ => return Err(undecided),
                                    }
                                }
                            };
                        }
                        Sign::Positive => {
                            // Proper secant: d > |Δr| definitely, so
                            // d > 0.
                            let crossings = circles.crossings();
                            candidates(
                                &mut contacts,
                                (s1, s2),
                                &crossings,
                                CKind::Crossing,
                                spans,
                                Settle::InStretch,
                                band,
                            )?;
                        }
                    }
                }
            }
            Ok(PairOutcome::Contacts(contacts))
        }
    }
}

/// The two carriers as [`arc_arc`] and [`joint_tangency`] read them.
/// The margins and the crossings read one [`Circles2`], so each factor
/// the crossings take is a margin the caller decided.
fn carrier_pair<T: Real>(g1: &ArcGeom<T>, g2: &ArcGeom<T>) -> Circles2<T> {
    Circles2::new(
        (g1.arc.centre, g1.arc.radius),
        (g2.arc.centre, g2.arc.radius),
    )
}

/// Which ends may settle a candidate a span reads in band
/// ([`end_in_stretch`]).
#[derive(Clone, Copy)]
enum Settle {
    /// Any end in the candidate's stretch.
    InStretch,
    /// An end in the candidate's stretch that stands definitely apart
    /// from both of the other segment's ends: for carriers within ε of
    /// each other between their crossings ([`end_in_stretch`] says
    /// why).
    ApartFromEnds,
}

/// Reads the candidate contacts of two carriers, `points`, each from
/// the two spans' readings of it (`spans`, through [`joint`]): held by
/// both, a candidate is a contact, of kind `interior` inside both spans
/// and a touch at an end. A candidate a span definitely misses settles
/// that point and nothing more, so the pair then reads its ends
/// ([`end_touches`]). In band on a span, it escalates unless the
/// segment's end nearest it stands inside its stretch
/// ([`end_in_stretch`]), and then the ends settle it as for a miss.
fn candidates<T: Decide>(
    contacts: &mut Vec<Contact<T>>,
    (s1, s2): (&Seg<T>, &Seg<T>),
    points: &[Point2<T>],
    interior: CKind,
    spans: impl Fn(Point2<T>) -> (Result<Sign, Indeterminate>, Result<Sign, Indeterminate>),
    settle: Settle,
    band: Band,
) -> Result<(), Indeterminate> {
    let mut missed = false;
    for (i, &q) in points.iter().enumerate() {
        let (m1, m2) = spans(q);
        let read = [(s1, s2, m1.is_err()), (s2, s1, m2.is_err())];
        match joint(m1, || m2) {
            Ok(None) => missed = true,
            Ok(Some(j)) => contacts.push(Contact {
                point: q,
                kind: match j {
                    Joint::Interior => interior,
                    Joint::Boundary => CKind::Touch,
                },
            }),
            Err(source) => {
                let rival = match points {
                    [first, second] => Some(if i == 0 { second } else { first }),
                    _ => None,
                };
                let settled = read
                    .into_iter()
                    .filter(|&(_, _, in_band)| in_band)
                    .all(|(seg, other, _)| end_in_stretch(seg, other, q, rival, settle, band));
                if !settled {
                    return Err(source);
                }
                missed = true;
            }
        }
    }
    if missed {
        end_touches(contacts, s1, s2, band)?;
    }
    Ok(())
}

/// Whether `seg`'s end nearest the candidate `q`, which `seg`'s span
/// reads in band, stands in `q`'s own stretch: the end `nearer_end`
/// picks (margin (q − a)·û − L/2, the projection's offset from the
/// chord's bisector, so the end nearer `q` in distance; on a line, the
/// same t `line_span` reads, less L/2) reads on `other`'s carrier
/// (`circle_side` or `chord_side`), and, where the carriers have a
/// second candidate `rival`, definitely nearer `q` than `rival`
/// (`own_candidate`, margin |end − rival| − |end − q|).
/// Any other reading, in band included, is false, and the caller's
/// escalation stands.
///
/// When it holds, the ends settle the pair as they do after a miss
/// ([`end_touches`]). Were `q` on `seg`, the part of `seg` between `q`
/// and that end is the shorter way round `seg`'s carrier between them
/// (an end nearer in distance is nearer along the carrier, and the
/// part of an arc inside it is at most a half turn). Along `seg`'s
/// carrier the distance to `other`'s rises from each candidate to one
/// peak midway to the next, so that shorter way, from a candidate to a
/// point nearer it than the rival, holds no peak: the distance grows
/// from `q` to the end, which is within ε. That holds on a line and on
/// a circle of any radius or sweep, and whatever `arc_span`'s
/// compression lets the in-band reading reach, because the reading
/// asks the end, not the span. So were `q` on both segments, either
/// `other` holds the end's foot, and the end touches it, or `other`
/// ends between `q` and the foot, within ε of `seg`'s carrier and over
/// the part of `seg` between `q` and its end, and that end touches
/// `seg`. Neither reads clear. Were `q` off either segment, the
/// candidate is a miss, which the ends settle anyway.
///
/// So the end read confines the settle to `q`'s stretch, at any
/// crossing angle: at a steep crossing that stretch is short, and the
/// end lies in it only where it stands within about ε of `q` itself.
/// Where `seg` and `other` are adjacent, their shared vertex is on both
/// carriers. If the carriers read secant it is one candidate, `rival`
/// to the other, and settles that one only from nearer it than itself.
/// That fails where the carriers stay within ε between the crossings,
/// an internal near-tangency read as a secant: the shared vertex is on
/// both carriers anywhere there, and the touch the settle leaves may be
/// its own, which the discount absorbs. There the end must stand apart
/// from both of `other`'s ends ([`Settle::ApartFromEnds`], decided as
/// `contact_at_shared_vertex`).
/// If they read tangent, they cut no chord deeper than ε: a line's
/// `carrier_line_circle` margin is that depth, and two circles' is
/// `carrier_circles_chord`, which reads a deeper chord as a secant even
/// where the internal clearance is within ε, and a chord within the
/// band as a secant too, keeping a verdict only where the two readings
/// agree. So the carriers re-cross, if at all, within √(2rε) of the
/// tangency, the pair meets there only as the joint does, and the
/// joint pass classifies the joint by its turn ([`joint_tangency`]).
/// How far the two run within ε of each other past the joint is not
/// read: that neighbourhood is the joint's own, and its extent is not a
/// number any pass reads (`[topology.census]`).
fn end_in_stretch<T: Decide>(
    seg: &Seg<T>,
    other: &Seg<T>,
    q: Point2<T>,
    rival: Option<&Point2<T>>,
    settle: Settle,
    band: Band,
) -> bool {
    let half = seg.len * T::from_f64(0.5);
    let end = match decide(
        "nearer_end",
        Margin::of((q - seg.a).dot(seg.unit) - half),
        band,
    ) {
        Ok(Sign::Negative) => seg.a,
        Ok(Sign::Positive) => seg.b,
        Ok(Sign::Zero) | Err(_) => return false,
    };
    let own = rival.is_none_or(|&rival| {
        matches!(
            decide(
                "own_candidate",
                Margin::of(end.distance(rival) - end.distance(q)),
                band
            ),
            Ok(Sign::Positive)
        )
    });
    let apart = || {
        [other.a, other.b].into_iter().all(|e| {
            matches!(
                coincident("contact_at_shared_vertex", end, e, band),
                Ok(Sign::Positive)
            )
        })
    };
    own && matches!(carrier_side(other, end, band), Ok(Sign::Zero))
        && match settle {
            Settle::InStretch => true,
            Settle::ApartFromEnds => apart(),
        }
}

/// Which side of `host`'s carrier the point `p` lies on: `chord_side`
/// for a line, `circle_side` for an arc.
fn carrier_side<T: Decide>(host: &Seg<T>, p: Point2<T>, band: Band) -> Result<Sign, Indeterminate> {
    match &host.kind {
        SegKind::Line => chord_side(host, p, band).map(|(side, _)| side),
        SegKind::Arc(g) => circle_side(g, p, band),
    }
}

/// The touches of two segments whose carriers meet near a candidate
/// one span definitely missed, or one [`end_in_stretch`] settled: every
/// end of either that touches the other ([`touches`]).
///
/// A candidate stands for a stretch, not a point. Carriers within ε at
/// a tangency stay within ε for about √(2rε) either side of it, and at
/// a crossing of angle φ for about ε/sin φ, which exceeds the Kε a span
/// reading resolves once φ < 1/K. Along either carrier the distance to
/// the other has one local minimum per candidate and grows away from
/// it, so the points of each segment within ε of the other carrier form
/// stretches whose ends inside that zone are segment ends. Two such
/// stretches meet only where an end of one lies in the other, so in the
/// reals the pair comes within ε exactly where some end does. Where the
/// two are adjacent, the stretch through their shared vertex is the
/// joint's own: the touch there is the vertex's to discount, the far
/// ends are what the pair reads (within ε touches, in band escalates,
/// definite is clear), and how far the stretch runs before them is not
/// read (`[topology.census]`).
///
/// The reading of that is one-sided. An end within ε of the other
/// segment never reads clear: each margin is at most the end's distance
/// from the segment. An end further off reads a touch or escalates out
/// to Kε from the carrier and to the span reading's reach past the
/// span's end, which is Kε along a line but grows toward a full turn
/// along an arc ([`arc_span`]'s compression): a refusal, never an
/// acceptance.
fn end_touches<T: Decide>(
    contacts: &mut Vec<Contact<T>>,
    s1: &Seg<T>,
    s2: &Seg<T>,
    band: Band,
) -> Result<(), Indeterminate> {
    for (host, other) in [(s1, s2), (s2, s1)] {
        for end in [other.a, other.b] {
            if touches(host, end, band)? {
                contacts.push(Contact {
                    point: end,
                    kind: CKind::Touch,
                });
            }
        }
    }
    Ok(())
}

/// Whether the point `p` touches the segment `host`: it lies on the
/// host's carrier (`chord_side`, or `circle_side` for an arc), and the
/// host's span holds its projection onto that carrier.
fn touches<T: Decide>(host: &Seg<T>, p: Point2<T>, band: Band) -> Result<bool, Indeterminate> {
    touches_from(host, p, carrier_side(host, p, band), band)
}

/// [`touches`], from `p`'s side of the host's carrier as already read.
/// On the carrier the point is held by it, and off it, on either side,
/// missed, so [`joint`] settles the pair of readings: a definite miss
/// of the carrier or of the span (`line_span`, or `arc_span` of the
/// radial projection) is no touch, and only then does an in-band
/// reading escalate.
fn touches_from<T: Decide>(
    host: &Seg<T>,
    p: Point2<T>,
    side: Result<Sign, Indeterminate>,
    band: Band,
) -> Result<bool, Indeterminate> {
    let on = side.map(|side| match side {
        Sign::Zero => Sign::Positive,
        Sign::Positive | Sign::Negative => Sign::Negative,
    });
    let held = || match &host.kind {
        SegKind::Line => line_span(host, p, band),
        SegKind::Arc(g) => {
            let foot = g.arc.centre + (p - g.arc.centre) * (g.arc.radius / g.arc.rim(p));
            arc_span(g, foot, band)
        }
    };
    Ok(joint(on, held)?.is_some())
}

/// A grazing ray: the parity question could not be answered definitely
/// with this ray (an endpoint or tangency on the ray line, an in-band
/// margin, a contact at the origin). Not a profile error — the caller
/// deterministically retries the next candidate ray (Mäntylä ch. 13's
/// "try another ray" made principled: a grazing answer is refused, never
/// fudged; exhaustion becomes a typed error at the validation layer).
pub(crate) struct Graze;

/// Counts the crossings of the half-line from `origin` along unit `dir`
/// with one segment, for ray-parity containment.
///
/// Predicates: **`ray_side`** — margin: the signed perpendicular
/// distance of a segment endpoint from the ray's carrier line (meters);
/// any Zero ⇒ graze. **`ray_advance`** — margin: the candidate
/// intersection's arc-length parameter along the ray (meters); Zero ⇒
/// the contact is at the ray origin ⇒ graze (unreachable for validated
/// disjoint loops, kept total). Arc segments reuse
/// `carrier_line_circle` (tangent carrier ⇒ graze) and `arc_span`
/// (endpoint hit ⇒ graze). Every in-band or poisoned classification is
/// a graze, never a guess.
pub(crate) fn ray_crossings<T: Decide>(
    origin: Point2<T>,
    dir: Vec2<T>,
    seg: &Seg<T>,
    band: Band,
) -> Result<usize, Graze> {
    let side = |q: Point2<T>| -> Result<Sign, Graze> {
        decide("ray_side", Margin::of(dir.perp_dot(q - origin)), band).map_err(|_| Graze)
    };
    match &seg.kind {
        SegKind::Line => {
            let o_a = side(seg.a)?;
            let o_b = side(seg.b)?;
            match (o_a, o_b) {
                (Sign::Zero, _) | (_, Sign::Zero) => Err(Graze),
                (Sign::Positive, Sign::Positive) | (Sign::Negative, Sign::Negative) => Ok(0),
                _ => {
                    // Endpoints strictly straddle the ray line: one
                    // carrier crossing; count it if strictly ahead.
                    let denom = seg.chord.perp_dot(dir);
                    let s = seg.chord.perp_dot(seg.a - origin) / denom;
                    match decide("ray_advance", Margin::of(s), band) {
                        Ok(Sign::Positive) => Ok(1),
                        Ok(Sign::Negative) => Ok(0),
                        Ok(Sign::Zero) | Err(_) => Err(Graze),
                    }
                }
            }
        }
        SegKind::Arc(g) => {
            let to_center = g.arc.centre - origin;
            let h = dir.perp_dot(to_center);
            let (clearance, _) = carrier_line_circle_margin(dir, origin, g);
            match decide("carrier_line_circle", Margin::of(clearance), band).map_err(|_| Graze)? {
                Sign::Negative => Ok(0),
                Sign::Zero => Err(Graze),
                Sign::Positive => {
                    let tc = to_center.dot(dir);
                    let half = (g.arc.radius.powi(2) - h.powi(2)).sqrt();
                    let mut count = 0;
                    for t in [tc - half, tc + half] {
                        match decide("ray_advance", Margin::of(t), band) {
                            Ok(Sign::Negative) => {}
                            Ok(Sign::Positive) => {
                                let q = origin + dir * t;
                                match arc_span(g, q, band) {
                                    Ok(Sign::Positive) => count += 1,
                                    Ok(Sign::Negative) => {}
                                    Ok(Sign::Zero) | Err(_) => return Err(Graze),
                                }
                            }
                            Ok(Sign::Zero) | Err(_) => return Err(Graze),
                        }
                    }
                    Ok(count)
                }
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::panic)]
mod pair_contact_tests {
    use super::*;
    use geom_core::{Bounds, Interval, Tol};

    /// What `line_arc` read: the contact count, or the predicate it
    /// escalated on.
    type Read = Result<usize, Option<&'static str>>;

    /// What a pair read, kind by kind: its contacts' kinds, or the
    /// predicate it escalated on or that refused to build an arc.
    type KindsRead = Result<Vec<CKind>, Option<&'static str>>;

    fn pt<T: Real>((x, y): (f64, f64)) -> Point2<T> {
        Point2::new(T::from_f64(x), T::from_f64(y))
    }

    fn line<T: Decide>(a: (f64, f64), b: (f64, f64), band: Band) -> Seg<T> {
        build_seg(pt(a), pt(b), Segment::Line, Consistency::Decide, band)
            .unwrap_or_else(|e| panic!("line {a:?} → {b:?}: {}", e.predicate()))
    }

    /// A segment's two stored ends.
    type Ends = [(f64, f64); 2];

    /// The real gap between two stored segments on `a`'s axis and on
    /// `b`'s, enclosed in `Interval`: each the distance between the two
    /// projected intervals on that segment's real unit chord, negative
    /// where they overlap.
    fn real_gaps(a: Ends, b: Ends) -> [Interval; 2] {
        let p = pt::<Interval>;
        let gap = |[o0, o1]: Ends, [q0, q1]: Ends| {
            let (o0, o1) = (p(o0), p(o1));
            let chord = o1 - o0;
            let unit = chord * (Interval::one() / chord.norm());
            let along = |q: Point2<Interval>| (q - o0).dot(unit);
            let (t0, t1, end) = (along(p(q0)), along(p(q1)), along(o1));
            (t0.min(t1) - end).max(Interval::zero() - t0.max(t1))
        };
        [gap(a, b), gap(b, a)]
    }

    /// **The gap read apart is a real gap, on stored points far from the
    /// origin.** Two segments whose coordinates are arbitrary `f64`s up to
    /// `10⁶` m out, up to `3·10⁶` m long, `b` starting a small step
    /// (`10⁻¹³` … `10⁻⁷` m, either sign) along `a`'s line past its end and
    /// in band across it. Wherever [`gaps_apart`] reads them apart at the
    /// fine band, the `Interval` enclosure of the real gap on one of the
    /// two axes is at least `ε`, what a positive reading promises (past
    /// `Kε` less the charge): the segments are apart in ℝ by the band's
    /// zero. A counterexample sweep: its seed varies, and
    /// [`a_far_pair_reads_apart_only_past_its_charge`] is its static
    /// witness that the reading fires here at all.
    #[test]
    fn a_gap_read_apart_is_a_real_gap_on_points_far_from_the_origin() {
        let band = Band::new(1e-12, 1e-11).unwrap_or_else(|e| panic!("band: {e}"));
        let mut rng = test_utils::fuzz::start("seg-gap-far");
        for _ in 0..test_utils::fuzz::scaled(20_000) {
            let far = 10f64.powf(rng.range(0.0, 6.0));
            let a0 = (far * rng.range(-1.0, 1.0), far * rng.range(-1.0, 1.0));
            let th = rng.range(0.0, core::f64::consts::TAU);
            let (dx, dy) = (th.cos(), th.sin());
            let len = 10f64.powf(rng.range(-2.0, 6.5));
            let a1 = (a0.0 + dx * len, a0.1 + dy * len);
            let step = rng.range(-1.0, 1.0) * 10f64.powf(rng.range(-13.0, -7.0));
            let across = rng.range(-1.0, 1.0) * 5e-12;
            let b0 = (
                a1.0 + dx * step - dy * across,
                a1.1 + dy * step + dx * across,
            );
            let phi = th + rng.range(-1.0, 1.0);
            let b1 = (b0.0 + phi.cos() * len, b0.1 + phi.sin() * len);
            if (b1.0 - a1.0) * dx + (b1.1 - a1.1) * dy < 0.0 {
                continue;
            }
            let (s1, s2) = (line::<f64>(a0, a1, band), line::<f64>(b0, b1, band));
            if !gaps_apart(&s1, &s2, band) {
                continue;
            }
            let gaps = real_gaps([a0, a1], [b0, b1]);
            assert!(
                gaps.iter().any(|g| g.lo() >= band.zero()),
                "read apart, but the real gaps enclose {gaps:?}: a {a0:?}→{a1:?}, \
                 b {b0:?}→{b1:?} — {}",
                test_utils::fuzz::replay()
            );
        }
    }

    /// **The collinear arm reads the home's projections, bit for bit,
    /// and the old arm's verdict.** [`collinear_overlap`] reads two
    /// collinear spans through [`geom_core::one_line`]; before, `line_line`
    /// read its own `tc`, `td`, `lo = max(min(tc, td), 0)` and
    /// `hi = min(max(tc, td), L)`, uncharged, kept here as the oracle.
    /// `s2`'s ends are drawn on `s1`'s line near either of its ends
    /// (`10⁻¹³` … `10⁻⁷` m either way), up to `10⁶` m out, `s1` up to
    /// `10⁶` m long. On every draw the home's `lo` is the old one's, bit
    /// for bit, and so is its `hi` except where it is bounded by `s1`'s
    /// own end, which the home projects (`(s1.b − s1.a)·û`) where the old
    /// arm read the stored length: there they differ by at most four
    /// ulps of `L`. And the verdict is the old arm's wherever the old
    /// margin stands further than the charge (and that ulp gap) from a
    /// band edge.
    #[test]
    fn the_collinear_arm_reads_the_old_arms_projections_and_verdict() {
        let band = Band::new(1e-12, 1e-11).unwrap_or_else(|e| panic!("band: {e}"));
        let mut rng = test_utils::fuzz::start("seg-collinear-home");
        let sign = |s: Result<Sign, Indeterminate>| s.map_err(|_| ());
        for _ in 0..test_utils::fuzz::scaled(20_000) {
            let far = 10f64.powf(rng.range(0.0, 6.0));
            let a0 = (far * rng.range(-1.0, 1.0), far * rng.range(-1.0, 1.0));
            let th = rng.range(0.0, core::f64::consts::TAU);
            let (dx, dy) = (th.cos(), th.sin());
            let len = 10f64.powf(rng.range(-2.0, 6.0));
            let a1 = (a0.0 + dx * len, a0.1 + dy * len);
            let (end0, end1) = if rng.range(0.0, 1.0) < 0.5 {
                (0.0, -len)
            } else {
                (len, 2.0 * len)
            };
            let mut near = |end: f64| {
                let step = rng.range(-1.0, 1.0) * 10f64.powf(rng.range(-13.0, -7.0));
                let t = end + step;
                (a0.0 + dx * t, a0.1 + dy * t)
            };
            let (b0, b1) = (near(end0), near(end1));
            let (s1, s2) = (line::<f64>(a0, a1, band), line::<f64>(b0, b1, band));
            let tc = (s2.a - s1.a).dot(s1.unit);
            let td = (s2.b - s1.a).dot(s1.unit);
            let (lo, hi) = (tc.min(td).max(0.0), tc.max(td).min(s1.len));
            let along = on_one_line_2d(s1.unit, (s1.a, s1.b), (s2.a, s2.b), 0.0);
            let replay = test_utils::fuzz::replay;
            assert_eq!(along.lo().to_bits(), lo.to_bits(), "lo — {}", replay());
            let slack = if along.hi().to_bits() == hi.to_bits() {
                0.0
            } else {
                let end = (s1.b - s1.a).dot(s1.unit);
                assert!(
                    hi == s1.len
                        && along.hi() == end
                        && (end - s1.len).abs() <= 4.0 * s1.len * f64::EPSILON,
                    "hi {} against {hi}, s1's end projected {end}, L {} — {}",
                    along.hi(),
                    s1.len,
                    replay()
                );
                (end - s1.len).abs()
            };
            let margin = hi - lo;
            let reach = along.noise + slack;
            let edges = [band.zero(), band.escalate()];
            if edges.iter().any(|&e| (margin.abs() - e).abs() <= reach) {
                continue;
            }
            let old = sign(decide("collinear_overlap", Margin::of(margin), band));
            let new = match collinear_overlap(&s1, &s2, band) {
                Ok(PairOutcome::Overlap) => Ok(Sign::Positive),
                Ok(PairOutcome::Contacts(c)) if c.is_empty() => Ok(Sign::Negative),
                Ok(PairOutcome::Contacts(_)) => Ok(Sign::Zero),
                Err(_) => Err(()),
            };
            assert_eq!(
                new,
                old,
                "margin {margin}, noise {} — {}",
                along.noise,
                replay()
            );
        }
    }

    /// **A pair apart only along the second segment's axis is no contact,
    /// whichever is named first.** At ε `10⁻⁹`, `K` 10: `s2` runs from the
    /// origin to `(−0.3731…, 0.03117…)`, and `s1` from just past `s2`'s
    /// end on to `(−0.4665…, −0.0340…)`. On `s1`'s axis the projections
    /// touch within the band and no carrier side separates the pair; on
    /// `s2`'s axis the gap is apart. Read with `s1` first the pair needs
    /// the second axis, so both orders are pinned.
    #[test]
    fn a_pair_apart_only_along_the_second_axis_is_no_contact_both_ways() {
        let band = Band::new(1e-9, 1e-8).unwrap_or_else(|e| panic!("band: {e}"));
        let s2 = line::<f64>(
            (0.0, 0.0),
            (-0.3731081377223676, 0.031178661125134788),
            band,
        );
        let s1 = line::<f64>(
            (-0.3731081484093557, 0.03117866152700223),
            (-0.46657191024747935, -0.03400099107344501),
            band,
        );
        for (name, a, b) in [("s1 first", &s1, &s2), ("s2 first", &s2, &s1)] {
            match pair_contacts(a, b, None, band) {
                Ok(PairOutcome::Contacts(c)) if c.is_empty() => {}
                Ok(PairOutcome::Contacts(c)) => panic!("{name}: {} contacts", c.len()),
                Ok(PairOutcome::Overlap) => panic!("{name}: an overlap"),
                Err(e) => panic!("{name}: escalated on {:?}", e.predicate),
            }
        }
    }

    /// The grid point nearest `s1`'s line whose projection along the
    /// Pythagorean direction `(a, b, c)` exceeds the origin's by exactly
    /// `n` (grid units times `c`): an integer solution of `a·x + b·y = n`,
    /// chosen of the family `(x + b·j, y − a·j)` to stand within `c/2`
    /// grid steps of the line.
    fn along_exactly(n: i128, (a, b, c): (i128, i128, i128)) -> [i128; 2] {
        // a·s + b·t = 1 (extended Euclid; a and b are coprime).
        let (mut r0, mut r1, mut s0, mut s1, mut t0, mut t1) = (a, b, 1i128, 0i128, 0i128, 1i128);
        while r1 != 0 {
            let q = r0.div_euclid(r1);
            (r0, r1) = (r1, r0 - q * r1);
            (s0, s1) = (s1, s0 - q * s1);
            (t0, t1) = (t1, t0 - q * t1);
        }
        let (s, t) = (s0 * r0.signum(), t0 * r0.signum());
        let (x, y) = (n * s, n * t);
        // The cross x·b − y·a moves by c² per j; take it nearest zero.
        let cross = x * b - y * a;
        let j = -(cross as f64 / (c * c) as f64).round() as i128;
        [x + b * j, y - a * j]
    }

    /// **The collinear arm's verdict holds of the real overlap, on exact
    /// points.** `s1` runs `k` times a Pythagorean direction `(a, b)` (its
    /// length `k·c` metres exactly, `k` from `10²` to `10⁴`) from a point
    /// up to `2·10⁵` m out, every point on the `2⁻³⁴` m grid. `s2` starts
    /// at a grid point whose projection along `s1` stands an exact
    /// `r ∈ [−30ε, 30ε]` past one of `s1`'s ends ([`along_exactly`], within
    /// the band's zero of `s1`'s line), and runs on away from `s1`, so the
    /// overlap is `r`, exact in `i128`. At ε `10⁻⁹`, `K` 10,
    /// [`collinear_overlap`] keeps its contract: an overlap only where
    /// `r ≥ Kε`, disjoint only where `r ≤ −Kε`, a touch only where
    /// `|r| ≤ ε`. A counterexample sweep: the charge acts only at those
    /// edges, which is where these draws stand.
    #[test]
    fn the_collinear_arm_keeps_its_contract_on_the_real_overlap() {
        let band = Band::new(1e-9, 1e-8).unwrap_or_else(|e| panic!("band: {e}"));
        let (eps, k_eps) = (band.zero(), band.escalate());
        let grid = 2f64.powi(-34);
        let unit = 1i128 << 34;
        let mut rng = test_utils::fuzz::start("seg-collinear-exact");
        let dirs: [(i128, i128, i128); 5] = [
            (3, 4, 5),
            (5, 12, 13),
            (8, 15, 17),
            (7, 24, 25),
            (20, 21, 29),
        ];
        let on_grid = |x: f64| (x / grid).round() as i128;
        let sign = |rng: &mut test_utils::fuzz::Rng| if rng.range(0.0, 1.0) < 0.5 { -1 } else { 1 };
        for _ in 0..test_utils::fuzz::scaled(20_000) {
            let (mut a, mut b, c) = dirs[(rng.range(0.0, 5.0) as usize).min(4)];
            if rng.range(0.0, 1.0) < 0.5 {
                core::mem::swap(&mut a, &mut b);
            }
            a *= sign(&mut rng);
            b *= sign(&mut rng);
            let k = 10f64.powf(rng.range(2.0, 4.0)).round() as i128;
            let p0 = [on_grid(rng.range(-2e5, 2e5)), on_grid(rng.range(-2e5, 2e5))];
            let p1 = [p0[0] + k * a * unit, p0[1] + k * b * unit];
            // s2 starts `n` past an end of s1 (outward positive) and runs
            // away from s1: the overlap is −n in grid units over c.
            let (from, away) = if rng.range(0.0, 1.0) < 0.5 {
                (p0, -1)
            } else {
                (p1, 1)
            };
            let reach = 30.0 * eps / grid * c as f64;
            let n = rng.range(-reach, reach) as i128;
            let off = along_exactly(n * away, (a, b, c));
            let q0 = [from[0] + off[0], from[1] + off[1]];
            let k2 = 10f64.powf(rng.range(2.0, 4.0)).round() as i128 * away;
            let q1 = [q0[0] + k2 * a * unit, q0[1] + k2 * b * unit];
            let f = |q: [i128; 2]| (q[0] as f64 * grid, q[1] as f64 * grid);
            let (s1, s2) = (
                line::<f64>(f(p0), f(p1), band),
                line::<f64>(f(q0), f(q1), band),
            );
            let along = |q: [i128; 2]| (q[0] - p0[0]) * a + (q[1] - p0[1]) * b;
            let (t0, t1, end) = (along(q0), along(q1), along(p1));
            let r = (t0.max(t1).min(end) - t0.min(t1).max(0)) as f64 * grid / c as f64;
            let read = match collinear_overlap(&s1, &s2, band) {
                Ok(PairOutcome::Overlap) => "overlap",
                Ok(PairOutcome::Contacts(c)) if c.is_empty() => "disjoint",
                Ok(PairOutcome::Contacts(_)) => "touch",
                Err(_) => continue,
            };
            let kept = match read {
                "overlap" => r >= k_eps,
                "disjoint" => r <= -k_eps,
                _ => r.abs() <= eps,
            };
            assert!(
                kept,
                "read {read} at a real overlap of {r:e} m (ε {eps:e}): s1 {:?}→{:?}, \
                 s2 {:?}→{:?} — {}",
                f(p0),
                f(p1),
                f(q0),
                f(q1),
                test_utils::fuzz::replay()
            );
        }
    }

    /// **A collinear pair a hair inside the band's edge escalates.** A
    /// case of [`the_collinear_arm_keeps_its_contract_on_the_real_overlap`]'s
    /// draw, every point on the `2⁻³⁴` m grid, `s1` running `5795` times
    /// `(20, −21)` (the 20-21-29 family) about `2·10⁵` m out: `s2` continues `s1` from
    /// just past its end, and their real overlap is `−9.9796·10⁻⁹` m, a
    /// hair inside `−Kε`. So [`collinear_overlap`] may read it neither
    /// disjoint (`r ≤ −Kε`) nor a touch (`|r| ≤ ε`) nor an overlap: it
    /// escalates, on the straddle of its charged bounds. With the charge
    /// dropped it reads disjoint.
    #[test]
    fn a_collinear_pair_a_hair_inside_the_band_edge_escalates() {
        let band = Band::new(1e-9, 1e-8).unwrap_or_else(|e| panic!("band: {e}"));
        let s1 = line::<f64>(
            (21578.073839960678, 177081.7702821999),
            (137478.07383996068, 55386.7702821999),
            band,
        );
        let s2 = line::<f64>(
            (137478.07383996708, 55386.770282192214),
            (211658.07383996708, -22502.229717807786),
            band,
        );
        match collinear_overlap(&s1, &s2, band) {
            Err(e) => assert_eq!(e.predicate, Some("collinear_overlap")),
            Ok(PairOutcome::Overlap) => panic!("read an overlap"),
            Ok(PairOutcome::Contacts(c)) => panic!("read {} contacts", c.len()),
        }
    }

    /// **An adjacent pair reads a touch near its shared end only where
    /// every point it shares is within the band of it, on exact points.**
    /// [`the_collinear_arm_keeps_its_contract_on_the_real_overlap`]'s
    /// draw: `s1` runs `k` times a Pythagorean direction `(a, b)` from a
    /// point up to `2·10⁵` m out, every point on the `2⁻³⁴` m grid. `s2`
    /// shares one of `s1`'s ends and reaches to a grid point whose
    /// projection along `s1` stands an exact `n ∈ [−30ε, 30ε]` past that
    /// end ([`along_exactly`]) and `j` grid steps across it, `j` from 1 to
    /// `10¹²`, arriving or leaving at random. The real overlap is `−n`
    /// where `s2` reaches back over `s1`, else zero, exact in `i128`. At
    /// ε `10⁻⁹`, `K` 10, wherever [`touches_only_near_the_shared_end`]
    /// fires the real overlap is at most `ε`. A counterexample sweep: the
    /// draws stand at the band's zero edge, where the charge acts.
    #[test]
    fn a_pair_reads_a_touch_near_its_shared_end_only_within_the_band_of_it() {
        let band = Band::new(1e-9, 1e-8).unwrap_or_else(|e| panic!("band: {e}"));
        let eps = band.zero();
        let grid = 2f64.powi(-34);
        let unit = 1i128 << 34;
        let mut rng = test_utils::fuzz::start("seg-adjacent-touch-exact");
        let dirs: [(i128, i128, i128); 5] = [
            (3, 4, 5),
            (5, 12, 13),
            (8, 15, 17),
            (7, 24, 25),
            (20, 21, 29),
        ];
        let on_grid = |x: f64| (x / grid).round() as i128;
        let sign = |rng: &mut test_utils::fuzz::Rng| if rng.range(0.0, 1.0) < 0.5 { -1 } else { 1 };
        for _ in 0..test_utils::fuzz::scaled(20_000) {
            let (mut a, mut b, c) = dirs[(rng.range(0.0, 5.0) as usize).min(4)];
            if rng.range(0.0, 1.0) < 0.5 {
                core::mem::swap(&mut a, &mut b);
            }
            a *= sign(&mut rng);
            b *= sign(&mut rng);
            let k = 10f64.powf(rng.range(2.0, 4.0)).round() as i128;
            let p0 = [on_grid(rng.range(-2e5, 2e5)), on_grid(rng.range(-2e5, 2e5))];
            let p1 = [p0[0] + k * a * unit, p0[1] + k * b * unit];
            // `n` past the shared end, outward positive: the overlap is
            // `−n` where `s2` reaches back over `s1`.
            let (shared, away) = if rng.range(0.0, 1.0) < 0.5 {
                (p0, -1)
            } else {
                (p1, 1)
            };
            let reach = 30.0 * eps / grid * c as f64;
            let n = rng.range(-reach, reach) as i128;
            let off = along_exactly(n * away, (a, b, c));
            let j = 10f64.powf(rng.range(0.0, 12.0)).round() as i128 * sign(&mut rng);
            let q = [shared[0] + off[0] - j * b, shared[1] + off[1] + j * a];
            let f = |q: [i128; 2]| pt::<f64>((q[0] as f64 * grid, q[1] as f64 * grid));
            let (from, to) = if rng.range(0.0, 1.0) < 0.5 {
                (shared, q)
            } else {
                (q, shared)
            };
            let Ok(s2) = build_seg(f(from), f(to), Segment::Line, Consistency::Decide, band) else {
                continue;
            };
            let s1: Seg<f64> = build_seg(f(p0), f(p1), Segment::Line, Consistency::Decide, band)
                .unwrap_or_else(|e| panic!("s1: {}", e.predicate()));
            if !touches_only_near_the_shared_end(&s1, &s2, band) {
                continue;
            }
            let overlap = (-n).max(0) as f64 * grid / c as f64;
            assert!(
                overlap <= eps,
                "read a touch near the shared end at a real overlap of {overlap:e} m \
                 (ε {eps:e}): s1 {:?}→{:?}, s2 {:?}→{:?} — {}",
                f(p0),
                f(p1),
                f(from),
                f(to),
                test_utils::fuzz::replay()
            );
        }
    }

    /// **A shared-end touch is read only past the overlap's charge.** A
    /// case of
    /// [`a_pair_reads_a_touch_near_its_shared_end_only_within_the_band_of_it`]'s
    /// draw, every point on the `2⁻³⁴` m grid: `s1` runs `7680` times
    /// `(−7, 24)`, `192` km, to an end about `3.4·10⁵` m out, and `s2`
    /// arrives at that end from back over `s1`: their real overlap is
    /// `1.0082·10⁻⁹` m, past ε. The overlap as computed reads within the
    /// band's zero, and only its charge lifts the upper bound past it, so
    /// [`touches_only_near_the_shared_end`] does not hold. With the charge
    /// dropped it would, and the pair would read a touch at a vertex it
    /// overlaps past.
    #[test]
    fn a_shared_end_touch_is_read_only_past_the_overlaps_charge() {
        let band = Band::new(1e-9, 1e-8).unwrap_or_else(|e| panic!("band: {e}"));
        let end = (-85837.30740873661, 327155.61704938876);
        let s1 = line::<f64>((-32077.30740873661, 142835.61704938876), end, band);
        let s2 = line::<f64>((-86069.63265372172, 327087.8555196004), end, band);
        let along = on_one_line_2d(s1.unit, (s1.a, s1.b), (s2.a, s2.b), 0.0);
        assert_eq!(
            decide("collinear_overlap", Margin::of(along.extent()), band),
            Ok(Sign::Zero),
            "the overlap as computed reads within the band's zero"
        );
        assert!(!touches_only_near_the_shared_end(&s1, &s2, band));
    }

    /// **A straight continuation whose overlap straddles its charge still
    /// escalates.** A case of
    /// [`every_pair_the_shared_end_touch_rescues_escalates_at_its_joint`]'s
    /// draw at ε `10⁻⁹`, `K` 10: a leg about `5.4·10⁵` m long arrives at
    /// `(4195.4, 1159.4)`, and a `960` m leg leaves it on the same line,
    /// its far end within the band's zero of the long leg's carrier. Both
    /// of the short leg's ends read on the long carrier, so [`line_line`]
    /// reads the pair collinear, and [`collinear_overlap`] straddles: the
    /// overlap's lower bound is in band, its upper bound Zero. That upper
    /// bound is what [`touches_only_near_the_shared_end`] reads, so the
    /// shared-end touch would hold here and the joint reads a continuation
    /// (`SameCarrier`): asked on the straddle, the rule would accept the
    /// pair. It is correct by the rule's argument and deliberately not
    /// accepted: the rule is asked only on an in-band side, so the pair
    /// escalates, shared vertex or not.
    #[test]
    fn a_straight_continuation_straddling_its_overlaps_charge_still_escalates() {
        let band = Band::new(1e-9, 1e-8).unwrap_or_else(|e| panic!("band: {e}"));
        let v = (4195.37792743973, 1159.414105497221);
        let leaving = line::<f64>(v, (5051.600599898648, 719.9751542338354), band);
        let arriving = line::<f64>((-483370.8154819185, 251392.89144105796), v, band);
        assert!(touches_only_near_the_shared_end(&leaving, &arriving, band));
        match joint_tangency(&arriving, &leaving, "path_junction_turn", band) {
            Ok(reading) => assert!(
                matches!(reading.class, JointClass::SameCarrier),
                "the joint reads a continuation, got {:?}",
                reading.class
            ),
            Err(e) => panic!("the joint decides, got an escalation on {:?}", e.predicate),
        }
        for (host, other) in [(&leaving, &arriving), (&arriving, &leaving)] {
            for shared in [None, Some(pt(v))] {
                match pair_contacts(host, other, shared, band) {
                    Err(e) => assert_eq!(e.predicate, Some("collinear_overlap"), "{shared:?}"),
                    Ok(_) => panic!("{shared:?}: the straddle must escalate"),
                }
            }
        }
    }

    /// **The pairs the shared-end touch rescues escalate at their joint.**
    /// Two lines meeting at a vertex at angle `θ`, the shorter `Ls` from
    /// `10⁻²` to `10³` m and the longer up to `10³` times it, about a
    /// vertex up to `10⁵` m out, with `sin θ·Ls` drawn across the band
    /// (`0.5ε` … `1.5Kε`), arriving and leaving either way round. Wherever
    /// [`pair_contacts`] escalates and [`touches_only_near_the_shared_end`]
    /// rescues the pair (read with either segment as host, as
    /// `validate::judge_pair` reads a pair in pass order), the joint pass
    /// ([`joint_tangency`]) escalates too: the short leg's far end stands
    /// off the long one's carrier by `sin θ·Ls`, which is the turn the
    /// joint reads, levered by the shorter arm. So the rule moves an
    /// escalation to the joint and never accepts a loop the pair pass
    /// escalated; a change to the joint pass that lets one through reds
    /// here, and owes the rule's soundness its review then. A floor on the
    /// rescues keeps the row from passing empty.
    #[test]
    fn every_pair_the_shared_end_touch_rescues_escalates_at_its_joint() {
        let band = Band::new(1e-9, 1e-8).unwrap_or_else(|e| panic!("band: {e}"));
        let (eps, k_eps) = (band.zero(), band.escalate());
        let mut rng = test_utils::fuzz::start("seg-adjacent-joint-coupling");
        let mut rescued = 0usize;
        let draws = test_utils::fuzz::scaled(20_000);
        for _ in 0..draws {
            let far = 10f64.powf(rng.range(0.0, 5.0));
            let v = (far * rng.range(-1.0, 1.0), far * rng.range(-1.0, 1.0));
            let short = 10f64.powf(rng.range(-2.0, 3.0));
            let long = short * 10f64.powf(rng.range(0.0, 3.0));
            let h = rng.range(0.5 * eps, 1.5 * k_eps);
            let theta = (h / short).asin();
            let th = rng.range(0.0, core::f64::consts::TAU);
            // The long leg arrives along `th`; the short one leaves at
            // `θ` off it, either side, or the other way round.
            let turn = if rng.range(0.0, 1.0) < 0.5 {
                theta
            } else {
                -theta
            };
            let (lu, su) = (th, th + turn);
            let at = |len: f64, ang: f64| (v.0 + len * ang.cos(), v.1 + len * ang.sin());
            let (far_long, far_short) = (at(-long, lu), at(short, su));
            let (prev, next) = if rng.range(0.0, 1.0) < 0.5 {
                ((far_long, v), (v, far_short))
            } else {
                // Reversed traversal: the short leg arrives.
                ((at(-short, su), v), (v, at(long, lu)))
            };
            let build = |(a, b): ((f64, f64), (f64, f64))| {
                build_seg::<f64>(pt(a), pt(b), Segment::Line, Consistency::Decide, band)
            };
            let (Ok(prev), Ok(next)) = (build(prev), build(next)) else {
                continue;
            };
            for (host, other) in [(&prev, &next), (&next, &prev)] {
                if pair_contacts(host, other, None, band).is_ok() {
                    continue;
                }
                if pair_contacts(host, other, Some(prev.b), band).is_err() {
                    continue;
                }
                rescued += 1;
                if let Ok(reading) = joint_tangency(&prev, &next, "path_junction_turn", band) {
                    panic!(
                        "rescued, and the joint decided {:?} on {}: prev {:?}→{:?}, \
                         next {:?}→{:?} — {}",
                        reading.class,
                        reading.predicate,
                        prev.a,
                        prev.b,
                        next.a,
                        next.b,
                        test_utils::fuzz::replay()
                    );
                }
            }
        }
        assert!(
            rescued * 10 >= draws,
            "only {rescued} rescues in {draws} draws — {}",
            test_utils::fuzz::replay()
        );
    }

    /// **A far, long pair reads apart only past its charge.** `a` runs
    /// `2·10⁶` m from `(123 456.7, −654 321.9)`, and `b` continues on its
    /// line, starting `step` past its end. The projections' rounding is
    /// about `2⁻⁵²` of their `4·10⁶` m, near `10⁻⁹` m, a hundred times the
    /// fine band. At a step of `10⁻⁷` m, past that charge, [`gaps_apart`]
    /// reads the pair apart, and the real gap is at least `ε`. At a step
    /// of `3·10⁻¹⁰` m, inside the rounding, it does not: with the charge
    /// dropped, the step would read as a gap.
    #[test]
    fn a_far_pair_reads_apart_only_past_its_charge() {
        let band = Band::new(1e-12, 1e-11).unwrap_or_else(|e| panic!("band: {e}"));
        let a0 = (123_456.7, -654_321.9);
        let (dx, dy) = (0.6, 0.8);
        let len = 2e6;
        let a1 = (a0.0 + dx * len, a0.1 + dy * len);
        let pair = |step: f64| {
            let b0 = (a1.0 + dx * step, a1.1 + dy * step);
            let b1 = (b0.0 + dx * len, b0.1 + dy * len);
            ([a0, a1], [b0, b1])
        };
        let read = |([a0, a1], [b0, b1]): (Ends, Ends)| {
            gaps_apart(&line::<f64>(a0, a1, band), &line(b0, b1, band), band)
        };
        let (a, b) = pair(1e-7);
        assert!(read((a, b)), "a step of 1e-7 m past the charge reads apart");
        let gaps = real_gaps(a, b);
        assert!(gaps.iter().any(|g| g.lo() >= band.zero()), "{gaps:?}");
        assert!(
            !read(pair(3e-10)),
            "a step of 3e-10 m, inside the projections' rounding, must not read apart"
        );
    }

    /// The arc on the circle `centre`, `radius` from polar angle `start`
    /// through the signed `sweep`.
    fn arc<T: Decide>(
        centre: (f64, f64),
        radius: f64,
        start: f64,
        sweep: f64,
        band: Band,
    ) -> Seg<T> {
        let at = |t: f64| (centre.0 + radius * t.cos(), centre.1 + radius * t.sin());
        let carrier = Arc2 {
            centre: pt(centre),
            radius: T::from_f64(radius),
            sweep: T::from_f64(sweep),
        };
        build_seg(
            pt(at(start)),
            pt(at(start + sweep)),
            Segment::Arc(carrier),
            Consistency::Decide,
            band,
        )
        .unwrap_or_else(|e| panic!("arc at {centre:?}, r = {radius}: {}", e.predicate()))
    }

    fn read<T: Decide>(line: &Seg<T>, arc: &Seg<T>, band: Band) -> Read {
        let SegKind::Arc(g) = &arc.kind else {
            panic!("the arc row is a line")
        };
        match line_arc(line, arc, g, band) {
            Ok(PairOutcome::Contacts(contacts)) => Ok(contacts.len()),
            Ok(PairOutcome::Overlap) => panic!("a line and an arc overlapped"),
            Err(source) => Err(source.predicate),
        }
    }

    fn read_arcs<T: Decide>(s1: &Seg<T>, s2: &Seg<T>, band: Band) -> Read {
        let (SegKind::Arc(g1), SegKind::Arc(g2)) = (&s1.kind, &s2.kind) else {
            panic!("an arc x arc row holds a line")
        };
        match arc_arc(s1, g1, s2, g2, band) {
            Ok(PairOutcome::Contacts(contacts)) => Ok(contacts.len()),
            Ok(PairOutcome::Overlap) => panic!("two arcs on distinct carriers overlapped"),
            Err(source) => Err(source.predicate),
        }
    }

    /// Every row at one scalar: (name, what the pair must read, what it
    /// read). Offsets are in the run's ε and K: `mid` is inside the
    /// band, `2·Kε` and `3·Kε` past it.
    fn rows<T: Decide>() -> Vec<(&'static str, Read, Read)> {
        let t = Tol::witness().get();
        let band = Band::linear(Tol::witness()).unwrap_or_else(|e| panic!("band: {e}"));
        let (eps, k) = (t.eps, t.k);
        let mid = eps * ((1.0 + k) / 2.0);
        let quarter = std::f64::consts::FRAC_PI_2;
        let pi = std::f64::consts::PI;
        // The circle tangent to y = 0 at the origin, where its carrier
        // clearance is decided Zero; the arc runs down its left side.
        let tangent = |sweep| arc::<T>((0.0, 1.0), 1.0, pi, sweep, band);
        // The circle under y = 0, externally tangent to it there; the
        // arc runs clockwise over its top from its left end.
        let below = |sweep: f64| arc::<T>((0.0, -1.0), 1.0, pi, -sweep, band);
        // The circle of radius 2 internally tangent to it there.
        let around = |sweep| arc::<T>((0.0, 2.0), 2.0, pi, sweep, band);
        // How far short of the tangency point an arc of the unit
        // circle stops for its end to stand 3Kε off the other carrier:
        // past the reach.
        let past = (1.0 - 3.0 * k * eps).acos();
        // The same carrier secant to y = 0 by an in-band clearance.
        let secant = arc::<T>((0.0, 1.0), 1.0 + mid, pi, quarter - 3.0 * k * eps, band);
        // The unit circle, cut by y = 0 at (±1, 0), its arc ending an
        // in-band distance past (1, 0).
        let unit = arc::<T>((0.0, 0.0), 1.0, quarter, -(quarter + mid), band);
        let kk = k * eps;
        vec![
            // The tangency point one segment misses and the other holds:
            // the missing segment's end stands within ε of the other,
            // which holds its foot, so the pair touches there.
            (
                "tangent: the line holds the foot, the arc stops 2Kε short of it",
                Ok(1),
                read(
                    &line((-1.0, 0.0), (1.0, 0.0), band),
                    &tangent(quarter - 2.0 * kk),
                    band,
                ),
            ),
            (
                "tangent: the line holds the foot in band, the arc stops 2Kε short of it",
                Ok(1),
                read(
                    &line((-1.0, 0.0), (mid, 0.0), band),
                    &tangent(quarter - 2.0 * kk),
                    band,
                ),
            ),
            (
                "tangent: the line stops 2Kε short of the foot, the arc holds it",
                Ok(1),
                read(
                    &line((-1.0, 0.0), (-2.0 * kk, 0.0), band),
                    &tangent(quarter + 2.0 * kk),
                    band,
                ),
            ),
            (
                "tangent: the line stops 2Kε short of the foot, the arc holds it in band",
                Ok(1),
                read(
                    &line((-1.0, 0.0), (-2.0 * kk, 0.0), band),
                    &tangent(quarter + mid),
                    band,
                ),
            ),
            (
                "tangent: both stop short of the foot on one side, the arc's end over the line",
                Ok(1),
                read(
                    &line((-1.0, 0.0), (-2.0 * kk, 0.0), band),
                    &tangent(quarter - 4.0 * kk),
                    band,
                ),
            ),
            (
                "external tangency: one arc stops 2Kε short of it, the other holds it",
                Ok(1),
                read_arcs(
                    &tangent(quarter - 2.0 * kk),
                    &below(quarter + 2.0 * kk),
                    band,
                ),
            ),
            (
                "external tangency: one arc stops 2Kε short of it, the other holds it in band",
                Ok(1),
                read_arcs(&tangent(quarter - 2.0 * kk), &below(quarter + mid), band),
            ),
            (
                "internal tangency: one arc stops 2Kε short of it, the other holds it in band",
                Ok(1),
                read_arcs(
                    &tangent(quarter - 2.0 * kk),
                    &around(quarter + mid / 2.0),
                    band,
                ),
            ),
            // Separated: the miss is past the reach, or the two stop
            // short on opposite sides of the tangency point.
            (
                "tangent: the line holds the foot, the arc stops past the reach",
                Ok(0),
                read(
                    &line((-1.0, 0.0), (1.0, 0.0), band),
                    &tangent(quarter - past),
                    band,
                ),
            ),
            (
                "tangent: the line stops past the reach, the arc holds the foot",
                Ok(0),
                read(
                    &line(
                        (-1.0, 0.0),
                        (-((1.0 + 3.0 * kk).powi(2) - 1.0).sqrt(), 0.0),
                        band,
                    ),
                    &tangent(quarter + 0.1),
                    band,
                ),
            ),
            (
                "tangent: the line and the arc stop 2Kε short of the foot on opposite sides",
                Ok(0),
                read(
                    &line((-1.0, 0.0), (-2.0 * kk, 0.0), band),
                    &arc::<T>((0.0, 1.0), 1.0, 0.0, -(quarter - 2.0 * kk), band),
                    band,
                ),
            ),
            (
                "external tangency: one arc stops past the reach, the other holds it",
                Ok(0),
                read_arcs(&tangent(quarter - past), &below(quarter + 0.1), band),
            ),
            (
                "internal tangency: one arc stops past the reach, the other holds it",
                Ok(0),
                read_arcs(&tangent(quarter - past), &around(quarter + 0.1), band),
            ),
            (
                "in-band secant: the arc skips the facing point but ends across the line",
                Err(Some("carrier_line_circle")),
                read(&line((-2.0, 0.0), (2.0, 0.0), band), &secant, band),
            ),
            (
                "secant: the line misses (1, 0), the arc holds it in band",
                Ok(0),
                read(&line((2.0, 0.0), (3.0, 0.0), band), &unit, band),
            ),
        ]
    }

    /// The rows whose touching end stands at an edge of the band off the
    /// other carrier, the pair's candidate missed by more than Kε: an
    /// arc of the circle tangent to y = 0 at the origin stops short of
    /// it, so that its end stands `δ` above the line (`chord_side`),
    /// outside the circle under it (`circle_side`, positive) or inside
    /// the circle of radius 2 around it (`circle_side`, negative), each
    /// holding the end's foot. Within ε the end touches; in band it
    /// escalates; past Kε the pair is apart.
    fn band_edge_rows<T: Decide>() -> Vec<(String, Read, Read)> {
        let t = Tol::witness().get();
        let band = Band::linear(Tol::witness()).unwrap_or_else(|e| panic!("band: {e}"));
        let (eps, k) = (t.eps, t.k);
        let quarter = std::f64::consts::FRAC_PI_2;
        let pi = std::f64::consts::PI;
        // How far short of the origin the arc stops for its end to stand
        // `δ` off each carrier, in the half-angle forms that keep `δ`'s
        // own digits.
        let over_line = |delta: f64| 2.0 * (delta / 2.0).sqrt().asin();
        let over_below = |delta: f64| 2.0 * ((2.0 * delta + delta * delta) / 8.0).sqrt().asin();
        let inside_around = |delta: f64| 4.0 * (delta.sqrt() / 2.0).asin();
        let short = |alpha: f64| arc::<T>((0.0, 1.0), 1.0, pi, quarter - alpha, band);
        let line = line::<T>((-1.0, 0.0), (1.0, 0.0), band);
        let below = arc::<T>((0.0, -1.0), 1.0, pi, -(quarter + 0.1), band);
        let around = arc::<T>((0.0, 2.0), 2.0, pi, quarter + 0.1, band);
        let mut rows = Vec::new();
        for (name, delta) in [
            ("0.5ε", 0.5 * eps),
            ("0.999ε", 0.999 * eps),
            ("1.001ε", 1.001 * eps),
            ("2ε", 2.0 * eps),
            ("0.999Kε", 0.999 * k * eps),
            ("1.001Kε", 1.001 * k * eps),
            ("2Kε", 2.0 * k * eps),
        ] {
            let want = |predicate| match delta {
                d if d <= eps => Ok(1),
                d if d <= k * eps => Err(Some(predicate)),
                _ => Ok(0),
            };
            rows.push((
                format!("an end {name} above the line"),
                want("chord_side"),
                read(&line, &short(over_line(delta)), band),
            ));
            rows.push((
                format!("an end {name} outside the circle under the line"),
                want("circle_side"),
                read_arcs(&short(over_below(delta)), &below, band),
            ));
            rows.push((
                format!("an end {name} inside the circle around it"),
                want("circle_side"),
                read_arcs(&short(inside_around(delta)), &around, band),
            ));
        }
        rows
    }

    /// **An end reads clear past the span reading's reach, and not
    /// before it**, at `f64` and at `Interval`. Past a line's end that
    /// reach is Kε. Past an arc's end, along its carrier, it is the arc
    /// length at which `arc_span`'s chordal margin leaves the band:
    /// s* = 2r·asin(sin(θ/4) + Kε/2r) − rθ/2 for extent θ, about
    /// Kε/cos(θ/4), which grows without bound toward a full turn. Off
    /// the carrier the reach is Kε.
    #[test]
    fn an_end_reads_clear_only_past_the_span_readings_reach() {
        fn reads<T: Decide>(band: Band) -> Vec<String> {
            let t = Tol::witness().get();
            let ke = t.k * t.eps;
            let at = |angle: f64, r: f64| (r * angle.cos(), r * angle.sin());
            let mut wrong = Vec::new();
            let mut check = |name: String, host: &Seg<T>, p: (f64, f64), clear: bool| {
                let got = touches(host, pt(p), band);
                if matches!(got, Ok(false)) != clear {
                    wrong.push(format!("{name}: clear {clear}, read {got:?}"));
                }
            };
            let line = line::<T>((0.0, 0.0), (1.0, 0.0), band);
            for f in [0.99, 1.01] {
                check(
                    format!("{f}·Kε past a line's end"),
                    &line,
                    (-f * ke, 0.0),
                    f > 1.0,
                );
                check(format!("{f}·Kε off a line"), &line, (0.5, f * ke), f > 1.0);
            }
            let quarter = std::f64::consts::FRAC_PI_2;
            for theta in [quarter, 2.0 * quarter, 3.0 * quarter, 6.0, 6.2] {
                let host = arc::<T>((0.0, 0.0), 1.0, 0.0, theta, band);
                let reach = 2.0 * ((theta / 4.0).sin() + ke / 2.0).asin() - theta / 2.0;
                for f in [0.99, 1.01] {
                    let name = format!("{f}·s* past the end of an arc of extent {theta:.3}");
                    check(name, &host, at(-f * reach, 1.0), f > 1.0);
                    let name = format!("{f}·Kε off an arc of extent {theta:.3}");
                    check(name, &host, at(theta / 2.0, 1.0 + f * ke), f > 1.0);
                }
            }
            wrong
        }
        let band = Band::linear(Tol::witness()).unwrap_or_else(|e| panic!("band: {e}"));
        let mut wrong_reads = reads::<f64>(band);
        wrong_reads.extend(reads::<Interval>(band));
        assert!(wrong_reads.is_empty(), "{}", wrong_reads.join("\n"));
    }

    /// The rows whose crossing a span reads in band. Two unit circles
    /// cross at the origin at angle `phi`, both carriers heading along
    /// +x there; one segment arrives from the left and the other leaves
    /// to the right, each ending `s` (in arc length) short of the
    /// crossing or past it. The chordal span reading of the crossing is
    /// then in band on both, and the segments are 2s apart, or cross.
    /// `rejections`'s dome is the same crossing read as a whole profile,
    /// through the public door, which cannot name these builders. A
    /// line tangent to a circle reads its one candidate in band where
    /// the line ends 0.8Kε either side of the tangency, and its end,
    /// on the circle, touches the arc.
    fn in_band_rows<T: Decide>() -> Vec<(&'static str, Read, Read)> {
        let t = Tol::witness().get();
        let band = Band::linear(Tol::witness()).unwrap_or_else(|e| panic!("band: {e}"));
        let kk = t.k * t.eps;
        let quarter = std::f64::consts::FRAC_PI_2;
        let s = 0.8 * kk;
        // The unit circle through the origin, heading +x there when
        // clockwise, turned by `phi` about the origin: its centre and
        // the polar angle of the origin about it.
        let circle = |phi: f64| ((phi.sin(), -phi.cos()), quarter + phi);
        // Arriving clockwise at the origin's polar angle less `end`.
        let arrive = |phi: f64, end: f64| {
            let (c, at) = circle(phi);
            arc::<T>(c, 1.0, at + quarter, -(quarter - end), band)
        };
        // Leaving clockwise from the origin's polar angle less `start`.
        let leave = |phi: f64, start: f64| {
            let (c, at) = circle(phi);
            arc::<T>(c, 1.0, at - start, -quarter, band)
        };
        let shallow = 0.03;
        // A unit circle's arc of three quarter turns whose end is 2Kε
        // past the crossing, crossed square by a radial line.
        let three_quarters = 3.0 * quarter;
        let x = (three_quarters - 2.0 * kk).sin_cos();
        let radial = line::<T>((0.5 * x.1, 0.5 * x.0), (1.5 * x.1, 1.5 * x.0), band);
        vec![
            (
                "shallow arc x arc: each stops 0.8Kε short of the crossing, opposite sides",
                Ok(0),
                read_arcs(&arrive(0.0, s), &leave(shallow, s), band),
            ),
            (
                "shallow line x arc: each stops 0.8Kε short of the crossing, opposite sides",
                Ok(0),
                read(
                    &line((-1.0, 0.0), (-s, 0.0), band),
                    &leave(shallow, s),
                    band,
                ),
            ),
            (
                "shallow arc x arc: each runs 0.8Kε past the crossing, so they cross",
                Ok(2),
                read_arcs(&arrive(0.0, -s), &leave(shallow, -s), band),
            ),
            (
                "shallow line x arc: each runs 0.8Kε past the crossing, so they cross",
                Ok(2),
                read(
                    &line((-1.0, 0.0), (s, 0.0), band),
                    &leave(shallow, -s),
                    band,
                ),
            ),
            (
                "shallow tangency: the line ends 0.8Kε past the tangency, on the arc",
                Ok(1),
                read(
                    &line((-1.0, 0.0), (s, 0.0), band),
                    &arc::<T>((0.0, 1.0), 1.0, -quarter - 1.0, 2.0, band),
                    band,
                ),
            ),
            (
                "shallow tangency: the line ends 0.8Kε short of the tangency, on the arc",
                Ok(1),
                read(
                    &line((-1.0, 0.0), (-s, 0.0), band),
                    &arc::<T>((0.0, 1.0), 1.0, -quarter - 1.0, 2.0, band),
                    band,
                ),
            ),
            (
                "square: the arc holds the crossing 2Kε from its end, read in band",
                Err(Some("arc_span")),
                read(
                    &radial,
                    &arc::<T>((0.0, 0.0), 1.0, 0.0, three_quarters, band),
                    band,
                ),
            ),
        ]
    }

    /// **A crossing a span reads in band settles on the segments' ends
    /// only inside its stretch**, at `f64` and at `Interval`
    /// ([`in_band_rows`]). At a shallow crossing two segments stopping
    /// short on opposite sides read no contact and two running past it
    /// touch; a line ending either side of its tangency with an arc
    /// touches it at that end; at a square crossing the arc's end lies
    /// off the line, and the in-band reading escalates.
    #[test]
    fn a_crossing_read_in_band_settles_on_the_ends_inside_its_stretch() {
        let mut wrong_rows = wrong("f64", in_band_rows::<f64>());
        wrong_rows.extend(wrong("Interval", in_band_rows::<Interval>()));
        assert!(wrong_rows.is_empty(), "{}", wrong_rows.join("\n"));
    }

    /// Arcs whose carriers cross inside both, as (centre, radius, start,
    /// sweep), their radii orders of magnitude apart: r = 62.76 against
    /// 1.28e-7, crossing 2.43 inside the first and 46ε at ε = 1e-9
    /// inside the second's ends, and r = 80.44 against 0.0317, crossing
    /// 20ε at ε = 1e-12 inside the first's end and 0.157 inside the
    /// second.
    #[allow(clippy::type_complexity)]
    const UNEQUAL_RADII: [(&str, [((f64, f64), f64, f64, f64); 2]); 2] = [
        (
            "r = 62.76 x 1.28e-7",
            [
                (
                    (-106.677_448_867_187_27, 59.130_528_391_886_16),
                    62.755_865_313_469_435,
                    0.986_377_959_232_007_9,
                    4.084_286_531_899_824,
                ),
                (
                    (-86.959_987_196_933_17, -0.447_322_568_277_620_46),
                    1.284_420_198_932_770_8e-7,
                    3.022_777_867_117_959_3,
                    -3.855_885_774_703_99,
                ),
            ],
        ),
        (
            "r = 80.44 x 0.0317",
            [
                (
                    (6.744_101_644_309_793, 79.705_363_072_029_3),
                    80.443_149_988_593_5,
                    5.633_354_791_777_812,
                    -0.902_846_282_788_287_3,
                ),
                (
                    (8.183_130_461_020_52, -0.750_331_493_417_868_4),
                    0.031_696_690_544_870_94,
                    2.274_954_781_868_850_7,
                    4.970_712_391_069_452,
                ),
            ],
        ),
    ];

    /// What each [`UNEQUAL_RADII`] pair reads, both ways round: its
    /// contact count, the predicate it escalated on, or the predicate
    /// that refused to build an arc.
    fn unequal_radii_reads<T: Decide>() -> Vec<(String, KindsRead)> {
        let band = Band::linear(Tol::witness()).unwrap_or_else(|e| panic!("band: {e}"));
        let build = |(c, r, start, sweep): ((f64, f64), f64, f64, f64)| {
            let at = |t: f64| (c.0 + r * t.cos(), c.1 + r * t.sin());
            let carrier = Arc2 {
                centre: pt(c),
                radius: T::from_f64(r),
                sweep: T::from_f64(sweep),
            };
            build_seg(
                pt(at(start)),
                pt(at(start + sweep)),
                Segment::Arc(carrier),
                Consistency::Decide,
                band,
            )
            .map_err(|e| Some(e.predicate()))
        };
        let mut reads = Vec::new();
        for (name, [first, second]) in UNEQUAL_RADII {
            for (order, (p, q)) in [("", (first, second)), (", reversed", (second, first))] {
                let read = build(p).and_then(|p| {
                    let q = build(q)?;
                    match pair_contacts(&p, &q, None, band) {
                        Ok(PairOutcome::Contacts(contacts)) => {
                            Ok(contacts.iter().map(|c| c.kind).collect())
                        }
                        Ok(PairOutcome::Overlap) => Err(Some("overlap")),
                        Err(source) => Err(source.predicate),
                    }
                });
                reads.push((format!("{name}{order}"), read));
            }
        }
        reads
    }

    /// **Arcs that cross read a contact at any ratio of their radii**,
    /// at `f64` and at `Interval` ([`UNEQUAL_RADII`]), and at each ε row
    /// exactly what [`unequal_radii_want`] says: never no contact, never
    /// an escalation, never a second crossing.
    #[test]
    fn arcs_crossing_at_unequal_radii_never_read_no_contact() {
        let eps = Tol::witness().eps();
        let want = unequal_radii_want(eps);
        let mut wrong_reads = Vec::new();
        for (scalar, reads) in [
            ("f64", unequal_radii_reads::<f64>()),
            ("Interval", unequal_radii_reads::<Interval>()),
        ] {
            for (i, (name, read)) in reads.into_iter().enumerate() {
                let bad = match &want {
                    Some(want) => read != want[i],
                    None => read.as_ref().is_ok_and(Vec::is_empty),
                };
                if bad {
                    wrong_reads.push(format!("{scalar}: {name} at eps {eps:e}: got {read:?}"));
                }
            }
        }
        assert!(wrong_reads.is_empty(), "{}", wrong_reads.join("\n"));
        if want.is_none() {
            test_utils::vacuity::stood_down(
                "unequal-radii reads",
                &format!(
                    "no pinned reads at eps {eps:e}; asserted only that no pair reads no contact"
                ),
            );
        } else if eps >= 1e-6 {
            test_utils::vacuity::stood_down(
                "unequal-radii reads",
                "the r = 62.76 x 1.28e-7 pair's small arc is sub-eps and refuses to build, \
                 so only the r = 80.44 x 0.0317 pair asserts a crossing here",
            );
        }
    }

    /// The reads [`unequal_radii_reads`] gives at the ε rows the suite
    /// runs, in its order. The first pair's small arc spans about
    /// 4.9e-7, so at 1e-6 its ends stand within ε and it refuses to
    /// build. The second pair crosses 2.0e-11 inside the large arc's
    /// end: within ε of it at 1e-9 and 1e-6, where the crossing reads
    /// at that end, a touch, and the end touches again; 20ε inside it at
    /// 1e-12, a crossing.
    fn unequal_radii_want(eps: f64) -> Option<[KindsRead; 4]> {
        use CKind::{Crossing, Touch};
        let (small, large): (KindsRead, KindsRead) = if eps == 1e-6 {
            (Err(Some("vertex_separation")), Ok(vec![Touch, Touch]))
        } else if eps == 1e-9 {
            (Ok(vec![Crossing]), Ok(vec![Touch, Touch]))
        } else if eps == 1e-12 {
            (Ok(vec![Crossing]), Ok(vec![Crossing]))
        } else {
            return None;
        };
        Some([small.clone(), small, large.clone(), large])
    }

    fn wrong<N: std::fmt::Display>(scalar: &str, rows: Vec<(N, Read, Read)>) -> Vec<String> {
        rows.into_iter()
            .filter(|(_, want, got)| want != got)
            .map(|(name, want, got)| format!("{scalar}: {name}: want {want:?}, got {got:?}"))
            .collect()
    }

    /// **A pair reads no contact only where its segments are apart**,
    /// at `f64` and at `Interval`. Decided-tangent carriers stay within
    /// ε of each other for ≈ √(2rε) along them, so where one span misses
    /// the tangency point the pair touches at an end that stands within
    /// ε of the other segment, line × arc and arc × arc, and reads no
    /// contact where every end is past that reach or off the other
    /// span. An in-band secant whose arc skips the facing point but ends
    /// across the line is not certified clear, and a definite line miss
    /// of a secant crossing is no contact whatever the arc reads there.
    #[test]
    fn a_pair_reads_no_contact_only_off_both_segments() {
        let mut wrong_rows = wrong("f64", rows::<f64>());
        wrong_rows.extend(wrong("Interval", rows::<Interval>()));
        assert!(wrong_rows.is_empty(), "{}", wrong_rows.join("\n"));
    }

    /// **An end at an edge of the band reads as its distance says**, at
    /// `f64` and at `Interval`: within ε of the other segment it
    /// touches, in band it escalates on the carrier reading, and past Kε
    /// the pair is apart ([`band_edge_rows`]).
    #[test]
    fn an_end_at_the_band_edge_touches_escalates_or_clears() {
        let mut wrong_rows = wrong("f64", band_edge_rows::<f64>());
        wrong_rows.extend(wrong("Interval", band_edge_rows::<Interval>()));
        assert!(wrong_rows.is_empty(), "{}", wrong_rows.join("\n"));
    }

    /// What two arcs about their circles' facing angles read: the
    /// contacts' kinds and points, or the predicate they escalated on.
    fn facing_contacts<T: Decide>(
        (c1, r1, t1): ((f64, f64), f64, f64),
        (c2, r2, t2): ((f64, f64), f64, f64),
        band: Band,
    ) -> Result<Vec<(CKind, Point2<T>)>, Option<&'static str>> {
        let s1 = arc::<T>(c1, r1, t1 - 0.5, 1.0, band);
        let s2 = arc::<T>(c2, r2, t2 - 0.5, 1.0, band);
        match pair_contacts(&s1, &s2, None, band) {
            Ok(PairOutcome::Contacts(contacts)) => {
                Ok(contacts.into_iter().map(|c| (c.kind, c.point)).collect())
            }
            Ok(PairOutcome::Overlap) => Err(Some("overlap")),
            Err(source) => Err(source.predicate),
        }
    }

    /// **A tangency's contact is the midpoint of the circles' nearest
    /// points**, `|g|/2` off each, wherever the radical foot falls, and
    /// a chord the circles cut deeper than the band reads as its two
    /// crossings although their internal clearance `g` is within ε. Two
    /// arcs a radian long about the facing points, at `f64` (kinds and
    /// point) and `Interval` (kinds); near-concentric circles stand
    /// `17ε` apart, where the foot is `g·r/d`, 0.03, off both.
    #[test]
    fn a_tangency_reads_at_the_midpoint_of_the_nearest_points() {
        let eps = Tol::witness().eps();
        let band = Band::linear(Tol::witness()).unwrap_or_else(|e| panic!("band: {e}"));
        let (d, g) = (17.0 * eps, 0.5 * eps);
        let pi = core::f64::consts::PI;
        let rows = [
            (
                "internal",
                ((0.0, 0.0), 1.0, 0.0),
                ((0.5, 0.0), 0.5 + g, 0.0),
                Some(1.0 + g / 2.0),
            ),
            (
                "internal, larger second",
                ((0.5, 0.0), 0.5 + g, 0.0),
                ((0.0, 0.0), 1.0, 0.0),
                Some(1.0 + g / 2.0),
            ),
            (
                "external",
                ((0.0, 0.0), 1.0, 0.0),
                ((2.0 + g, 0.0), 1.0, pi),
                Some(1.0 + g / 2.0),
            ),
            (
                "near-concentric, nested",
                ((0.0, 0.0), 1.0, 0.0),
                ((d, 0.0), 1.0 - d - g, 0.0),
                Some(1.0 - g / 2.0),
            ),
            (
                "near-concentric, nested, larger second",
                ((d, 0.0), 1.0 - d - g, 0.0),
                ((0.0, 0.0), 1.0, 0.0),
                Some(1.0 - g / 2.0),
            ),
            (
                "near-concentric chord",
                ((0.0, 0.0), 1.0, 0.0),
                ((d, 0.0), 1.0 - d + g, 0.0),
                None,
            ),
        ];
        let mut wrong_rows = Vec::new();
        for (name, one, two, at) in rows {
            let want = match at {
                Some(_) => vec![CKind::Tangency],
                None => vec![CKind::Crossing, CKind::Crossing],
            };
            match facing_contacts::<f64>(one, two, band) {
                Ok(got) => {
                    let kinds: Vec<CKind> = got.iter().map(|c| c.0).collect();
                    if kinds != want {
                        wrong_rows.push(format!("f64: {name}: want {want:?}, got {kinds:?}"));
                    }
                    if let (Some(x), [(_, p)]) = (at, got.as_slice()) {
                        let off = p.distance(Point2::new(x, 0.0));
                        if off > 0.01 * eps {
                            wrong_rows
                                .push(format!("f64: {name}: contact {p:?}, {off:e} off ({x}, 0)"));
                        }
                    }
                }
                Err(e) => wrong_rows.push(format!("f64: {name}: escalated on {e:?}")),
            }
            match facing_contacts::<Interval>(one, two, band) {
                Ok(got) if got.iter().map(|c| c.0).collect::<Vec<_>>() == want => {}
                got => wrong_rows.push(format!(
                    "Interval: {name}: want {want:?}, got {:?}",
                    got.map(|g| g.into_iter().map(|c| c.0).collect::<Vec<_>>())
                )),
            }
        }
        assert!(
            wrong_rows.is_empty(),
            "at eps {eps:e}:\n{}",
            wrong_rows.join("\n")
        );
    }
}

#[cfg(test)]
#[allow(clippy::panic)]
mod sweep_range_tests {
    use super::*;
    use geom_core::{Interval, Tol};

    /// What [`build_seg`] read: the segment's kind, or the predicate
    /// that refused it.
    type Read = Result<&'static str, &'static str>;

    /// The carrier's radius and the short arc's sweep δ: a chord rδ
    /// = 1e-4 past every ε row's band, and a sagitta rδ²/8 ≈ 1.3e-13
    /// within every row's ε, so the short arc reads as its chord at
    /// each row.
    const R: f64 = 1e4;
    const DELTA: f64 = 1e-8;

    /// The arc on the circle of radius [`R`] about the origin from
    /// (R, 0) to the polar angle [`DELTA`], storing `sweep`.
    fn read<T: Decide>(sweep: f64, band: Band) -> Read {
        let pt = |t: f64| Point2::new(T::from_f64(R * t.cos()), T::from_f64(R * t.sin()));
        let carrier = Arc2 {
            centre: Point2::new(T::zero(), T::zero()),
            radius: T::from_f64(R),
            sweep: T::from_f64(sweep),
        };
        match build_seg(
            pt(0.0),
            pt(DELTA),
            Segment::Arc(carrier),
            Consistency::Decide,
            band,
        ) {
            Ok(Seg {
                kind: SegKind::Line,
                ..
            }) => Ok("line"),
            Ok(Seg {
                kind: SegKind::Arc(..),
                ..
            }) => Ok("arc"),
            Err(issue) => Err(issue.predicate()),
        }
    }

    /// **A stored sweep outside 0 < |Δθ| ≤ 2π refuses whatever its
    /// sagitta**, at `f64` and at `Interval`. The quarter tangent the
    /// straightness read takes is 4π-periodic, so 4π ± δ bows like δ;
    /// 2π + δ bows far off its chord; δ itself is a legitimate arc
    /// whose sagitta is within ε, read as its chord.
    #[test]
    fn a_stored_sweep_past_a_full_turn_refuses_whatever_its_sagitta() {
        let band = Band::linear(Tol::witness()).unwrap_or_else(|e| panic!("band: {e}"));
        let tau = std::f64::consts::TAU;
        let rows: [(&str, f64, Read); 4] = [
            ("4π + δ", 2.0 * tau + DELTA, Err("arc_sweep_range")),
            ("−4π − δ", -2.0 * tau - DELTA, Err("arc_sweep_range")),
            ("2π + δ", tau + DELTA, Err("arc_sweep_range")),
            ("δ", DELTA, Ok("line")),
        ];
        let mut wrong = Vec::new();
        for (name, sweep, want) in rows {
            for (scalar, got) in [
                ("f64", read::<f64>(sweep, band)),
                ("Interval", read::<Interval>(sweep, band)),
            ] {
                if got != want {
                    wrong.push(format!("{scalar}: {name}: want {want:?}, got {got:?}"));
                }
            }
        }
        assert!(wrong.is_empty(), "{}", wrong.join("\n"));
    }
}

#[cfg(test)]
#[allow(clippy::panic)]
mod circles_accuracy_tests {
    use crate::dd::{Dd, off_circle, radical_foot};
    use geom_core::{Circles2, Point2};

    type P = (f64, f64);

    /// The most a crossing or a foot may sit off, in ulps of the pair's
    /// scale (the largest radius, centre distance or centre coordinate).
    /// The head reads at most 2.3 here, and each spelling that cancels
    /// reads 10⁴ or more on some row.
    const ULPS: f64 = 8.0;

    fn circles((c1, r1): (P, f64), (c2, r2): (P, f64)) -> Circles2<f64> {
        Circles2::new((Point2::new(c1.0, c1.1), r1), (Point2::new(c2.0, c2.1), r2))
    }

    fn scale((c1, r1): (P, f64), (c2, r2): (P, f64)) -> f64 {
        [
            c1.0,
            c1.1,
            c2.0,
            c2.1,
            r1,
            r2,
            (c2.0 - c1.0).hypot(c2.1 - c1.1),
        ]
        .into_iter()
        .fold(0.0, |m, x| m.max(x.abs()))
    }

    /// Two circles whose centres stand `d` apart along the unit
    /// direction (0.6, 0.8) from `c1`.
    fn pair(c1: P, r1: f64, d: f64, r2: f64) -> [(P, f64); 2] {
        [(c1, r1), ((c1.0 + 0.6 * d, c1.1 + 0.8 * d), r2)]
    }

    /// Proper secants: near-concentric, near either tangency, radii
    /// orders of magnitude apart (the issue's two pairs among them), and
    /// centres far from the origin.
    fn secants() -> Vec<(&'static str, [(P, f64); 2])> {
        vec![
            (
                "near-concentric, r = 1",
                pair((0.3, 0.7), 1.0, 1e-6, 1.0 + 3e-7),
            ),
            (
                "near-concentric, r = 1e3",
                pair((-2.0, 5.0), 1e3, 1e-5, 1e3 - 4e-6),
            ),
            (
                "near internal tangency",
                pair((1.0, -1.0), 10.0, 7.0 + 1e-9, 3.0),
            ),
            (
                "near external tangency",
                pair((1.0, -1.0), 10.0, 13.0 - 1e-9, 3.0),
            ),
            ("ratio 1e10", pair((0.5, 0.25), 1e4, 1e4, 1e-6)),
            (
                "ratio 1e6, far out",
                pair((1e3, 1e3), 1e3, 1e3 + 1e-4, 1e-3),
            ),
            ("ratio 1.5, far out", pair((1e5, -3e4), 139.0, 120.0, 90.0)),
            (
                "r = 62.76 x 1.28e-7",
                [
                    (
                        (-106.677_448_867_187_27, 59.130_528_391_886_16),
                        62.755_865_313_469_435,
                    ),
                    (
                        (-86.959_987_196_933_17, -0.447_322_568_277_620_46),
                        1.284_420_198_932_770_8e-7,
                    ),
                ],
            ),
            (
                "r = 80.44 x 0.0317",
                [
                    (
                        (6.744_101_644_309_793, 79.705_363_072_029_3),
                        80.443_149_988_593_5,
                    ),
                    (
                        (8.183_130_461_020_52, -0.750_331_493_417_868_4),
                        0.031_696_690_544_870_94,
                    ),
                ],
            ),
        ]
    }

    /// Internally tangent pairs, as the Zero arm reads them:
    /// near-concentric, radii orders of magnitude apart, and far out.
    fn tangents() -> Vec<(&'static str, [(P, f64); 2])> {
        vec![
            (
                "near-concentric, r = 1",
                pair((0.3, 0.7), 1.0, 1e-6, 1.0 - 1e-6),
            ),
            (
                "near-concentric, far out",
                pair((1e3, 2e3), 500.0, 1e-4, 500.0 - 1e-4),
            ),
            ("ratio 1e5", pair((-3.0, 4.0), 100.0, 100.0 - 1e-3, 1e-3)),
        ]
    }

    /// **The crossings of two circles sit on both carriers, and an
    /// internal tangency's foot where the radical line truly meets the
    /// line of centres, to a few ulps of the pair's scale**, at any
    /// ratio of the radii and near concentric, near tangent or far from
    /// the origin, in either order. The crossings are measured by their
    /// double-double residual on each circle and the foot against the
    /// textbook form in double-double ([`crate::dd`]), neither of which
    /// shares the factored algebra.
    #[test]
    fn circle_pair_points_sit_within_a_few_ulps_of_scale() {
        let mut wrong = Vec::new();
        let ulps = |err: f64, at: f64| err.abs() / (f64::EPSILON * at);
        for (name, [p, q]) in secants() {
            for (order, (first, second)) in [("", (p, q)), (", reversed", (q, p))] {
                let at = scale(first, second);
                for x in circles(first, second).crossings() {
                    let x = (x.x, x.y);
                    for (c, r) in [first, second] {
                        let off = ulps(off_circle(x, c, r), at);
                        if off.is_nan() || off > ULPS {
                            wrong.push(format!(
                                "secant {name}{order}: crossing {x:?} sits {off:.3e} ulps off \
                                 the circle at {c:?}, r = {r:e}"
                            ));
                        }
                    }
                }
            }
        }
        for (name, [p, q]) in tangents() {
            for (order, (first, second)) in [("", (p, q)), (", reversed", (q, p))] {
                let at = scale(first, second);
                let foot = circles(first, second).foot();
                let (x, y) = radical_foot(first.0, first.1, second.0, second.1);
                let err = Dd::of(foot.x)
                    .sub(x)
                    .value()
                    .hypot(Dd::of(foot.y).sub(y).value());
                let off = ulps(err, at);
                if off.is_nan() || off > ULPS {
                    wrong.push(format!(
                        "internal tangency {name}{order}: foot ({}, {}) sits {off:.3e} ulps \
                         from the radical foot",
                        foot.x, foot.y
                    ));
                }
            }
        }
        assert!(wrong.is_empty(), "{}", wrong.join("\n"));
    }
}

/// The radial certificates' fuzz ([`segment_clear_of_circle`],
/// [`arc_clear_of_circle`]).
#[cfg(test)]
#[allow(clippy::panic)]
mod clear_of_circle_tests {
    use super::*;
    use geom_core::{Bounds, Interval};

    fn pt<T: Real>((x, y): (f64, f64)) -> Point2<T> {
        Point2::new(T::from_f64(x), T::from_f64(y))
    }

    /// The fuzz rows' band: ε `10⁻¹²`, where a computed point's rounding
    /// at `10⁶` m out passes the band's room.
    fn fine_band() -> Band {
        Band::new(1e-12, 1e-11).unwrap_or_else(|e| panic!("band: {e}"))
    }

    /// A draw's scale: `10^lo … 10^hi`, log-uniform.
    fn decades(rng: &mut test_utils::fuzz::Rng, lo: f64, hi: f64) -> f64 {
        10f64.powf(rng.range(lo, hi))
    }

    /// A sign, either way with even odds.
    fn signed(rng: &mut test_utils::fuzz::Rng) -> f64 {
        if rng.range(0.0, 1.0) < 0.5 { -1.0 } else { 1.0 }
    }

    /// The arcs' reach at the fine band: no arc builds whose circle
    /// stands past about `2.8` km from the origin (validation's
    /// `arc_carrier_resolution`), so every circle these sweeps build sits
    /// inside `10^REACH` m, centre and radius together.
    const REACH: f64 = 3.3;

    /// An arc sweep's distance from the origin, radius and step past the
    /// point the certificate reads, inside the reach: `10⁰` … `10^REACH`
    /// m out, radius `10⁻²` … `10^REACH` m, step `10⁻¹³` … `10⁻⁷` m.
    fn draw_scales(rng: &mut test_utils::fuzz::Rng) -> (f64, f64, f64) {
        (
            decades(rng, 0.0, REACH),
            decades(rng, -2.0, REACH),
            signed(rng) * decades(rng, -13.0, -7.0),
        )
    }

    /// [`segment_clear_of_circle`] on one built case, and whether its
    /// premises hold in ℝ: the circle of radius `r` about `c`, and a
    /// segment of length `len` along the direction `th`, on the line `h`
    /// from the centre, starting `step` past the centre's foot. `None`
    /// where the case does not build; otherwise whether the certificate
    /// fired and, if it did, whether the `Interval` reading of its
    /// premises on the stored points (the real foot off the span, both
    /// ends outside the circle) holds.
    fn segment_case(
        c: (f64, f64),
        r: f64,
        th: f64,
        (h, step, len): (f64, f64, f64),
        band: Band,
    ) -> Option<(bool, bool)> {
        let (ux, uy) = (th.cos(), th.sin());
        let a = (c.0 - uy * h + ux * step, c.1 + ux * h + uy * step);
        let b = (a.0 + ux * len, a.1 + uy * len);
        let carrier = Arc2 {
            centre: pt(c),
            radius: r,
            sweep: core::f64::consts::PI,
        };
        let circle = build_seg(
            pt::<f64>((c.0 + r, c.1)),
            pt((c.0 - r, c.1)),
            Segment::Arc(carrier),
            Consistency::Decide,
            band,
        )
        .ok()?;
        let SegKind::Arc(g) = &circle.kind else {
            return None;
        };
        let s = build_seg(pt(a), pt(b), Segment::Line, Consistency::Decide, band).ok()?;
        let tc = (g.arc.centre - s.a).dot(s.unit);
        if !segment_clear_of_circle(&s, g, tc, band) {
            return Some((false, true));
        }
        let p = pt::<Interval>;
        let (ia, ib, ic) = (p(a), p(b), p(c));
        let along = (ic - ia).dot(ib - ia);
        let off = along.hi() < 0.0 || (along - (ib - ia).dot(ib - ia)).lo() > 0.0;
        let outside = |q: Point2<Interval>| ((q - ic).norm() - Interval::from_f64(r)).lo() > 0.0;
        Some((true, off && outside(ia) && outside(ib)))
    }

    /// **A segment read clear of a circle is clear of it in ℝ**
    /// ([`segment_clear_of_circle`]): a counterexample sweep
    /// ([`segment_case`]). Each circle builds inside the arcs' reach
    /// ([`REACH`]). A segment on a line `10⁻¹²` … `10⁻⁵` m either side of
    /// the circle's tangent starts a small step (either sign) past the
    /// centre's foot, or ends that step short of it, so the foot is read
    /// from either end. Two families, half the draws each:
    /// - the whole range: the circle and step of [`draw_scales`], the
    ///   segment `10⁻²` … `10⁶` m long;
    /// - **where the foot's charge binds**: a circle of radius `Kε` …
    ///   `10⁻⁶` m inside the reach, a segment `10³` … `10⁹` m long whose far
    ///   end is the one the foot is read from, and a step of `10⁻¹¹` …
    ///   `10⁻⁷` m. There the foot's rounding, at the far end's magnitude,
    ///   outgrows the band while the ends still clear the tiny circle.
    ///
    /// Wherever the certificate fires, its premises hold in ℝ. A floor of
    /// fires in each family keeps the row from passing empty; with the
    /// foot's charge cut to a sixteenth, the second family reds it.
    #[test]
    fn a_segment_read_clear_of_a_circle_is_clear_of_it() {
        let band = fine_band();
        let mut rng = test_utils::fuzz::start("seg-segment-clear-of-circle");
        let draws = test_utils::fuzz::scaled(20_000);
        let mut fired = [0usize; 2];
        for i in 0..draws {
            let binding = i % 2 == 1;
            let (far, r, step, len) = if binding {
                (
                    decades(&mut rng, 0.0, REACH),
                    decades(&mut rng, (band.escalate()).log10(), -6.0),
                    signed(&mut rng) * decades(&mut rng, -11.0, -7.0),
                    decades(&mut rng, 3.0, 9.0),
                )
            } else {
                let (far, r, step) = draw_scales(&mut rng);
                (far, r, step, decades(&mut rng, -2.0, 6.0))
            };
            let c = (far * rng.range(-1.0, 1.0), far * rng.range(-1.0, 1.0));
            let th = rng.range(0.0, core::f64::consts::TAU);
            let h = r + signed(&mut rng) * decades(&mut rng, -12.0, -5.0);
            // From the far end: the segment runs up to the foot, ending
            // `step` short of it (or past it), so its start is `len` back.
            let from_far = binding || rng.range(0.0, 1.0) < 0.5;
            let start = if from_far { -step - len } else { step };
            if let Some((true, holds)) = segment_case(c, r, th, (h, start, len), band) {
                fired[usize::from(binding)] += 1;
                assert!(
                    holds,
                    "read clear, but its premises fail in ℝ: c {c:?} r {r} th {th} h {h} \
                     start {start} len {len} — {}",
                    test_utils::fuzz::replay()
                );
            }
        }
        for (family, &n) in ["the whole range", "the binding regime"].iter().zip(&fired) {
            assert!(
                n * 20 >= draws / 2,
                "{family}: only {n} fires in {} draws — {}",
                draws / 2,
                test_utils::fuzz::replay()
            );
        }
    }

    /// **A segment reads clear of a circle only past its foot's charge.**
    /// A circle of radius `10³` m about `(1500.3, −1700.7)`, near the arcs'
    /// reach at the fine band ([`REACH`], past which no arc builds), and a
    /// 10 m segment on a line `10⁻⁶` m outside it, so both ends clear their
    /// own charge. The foot's charge there is
    /// about `1.1·10⁻¹¹` m, beside the band's `Kε = 10⁻¹¹`. At a step of
    /// `10⁻⁸` m past the foot the certificate fires, and its premises
    /// hold in ℝ. At `1.5·10⁻¹¹` m it does not: the step is past `Kε`, so
    /// with the foot's charge dropped it would read as a miss.
    #[test]
    fn a_segment_reads_clear_only_past_its_charge() {
        let band = fine_band();
        let (c, r, th) = ((1500.3, -1700.7), 1e3, 0.65);
        let read = |step: f64| segment_case(c, r, th, (r + 1e-6, step, 10.0), band);
        assert_eq!(
            read(1e-8),
            Some((true, true)),
            "a step of 1e-8 m past the foot"
        );
        assert_eq!(
            read(1.5e-11),
            Some((false, true)),
            "a step of 1.5e-11 m, inside the foot's charge, must not read clear"
        );
    }

    /// [`arc_clear_of_circle`] on one built case, and whether its
    /// premises hold in ℝ: an arc of the circle `(c1, r1)` from the polar
    /// angle `start` through `sweep` (under half a turn), against the
    /// circle `(c2, r2)`. `None` where the case does not build; otherwise
    /// whether the certificate fired and, if it did, whether the
    /// `Interval` reading of its premises on the stored points holds: the
    /// real extreme's direction off the arc's span (cross-product signs),
    /// and both ends outside, or both inside, the second circle.
    fn arc_case(
        (c1, r1): ((f64, f64), f64),
        (c2, r2): ((f64, f64), f64),
        (start, sweep): (f64, f64),
        band: Band,
    ) -> Option<(bool, bool)> {
        let on = |c: (f64, f64), r: f64, t: f64| (c.0 + r * t.cos(), c.1 + r * t.sin());
        let build = |c: (f64, f64), r: f64, t: f64, sweep: f64| {
            let carrier = Arc2 {
                centre: pt(c),
                radius: r,
                sweep,
            };
            build_seg(
                pt::<f64>(on(c, r, t)),
                pt(on(c, r, t + sweep)),
                Segment::Arc(carrier),
                Consistency::Decide,
                band,
            )
            .ok()
        };
        let (s1, s2) = (build(c1, r1, start, sweep)?, build(c2, r2, 0.0, 1.0)?);
        let (SegKind::Arc(g1), SegKind::Arc(g2)) = (&s1.kind, &s2.kind) else {
            return None;
        };
        let toward = carrier_pair(g1, g2).direction();
        if !arc_clear_of_circle(&s1, g1, toward, g2, band) {
            return Some((false, true));
        }
        let p = pt::<Interval>;
        let (ia, ib, ic1, ic2) = (p((s1.a.x, s1.a.y)), p((s1.b.x, s1.b.y)), p(c1), p(c2));
        let (va, vb) = (ia - ic1, ib - ic1);
        let rim = |q: Point2<Interval>| (q - ic2).norm() - Interval::from_f64(r2);
        // Off the span of an arc under half a turn: on the far side of
        // either end's radius, in the arc's sense.
        let off = |v: Vec2<Interval>| {
            let (from_a, to_b) = (va.perp_dot(v), v.perp_dot(vb));
            if sweep > 0.0 {
                from_a.hi() < 0.0 || to_b.hi() < 0.0
            } else {
                from_a.lo() > 0.0 || to_b.lo() > 0.0
            }
        };
        let near = ic2 - ic1;
        let outside = off(near) && rim(ia).lo() > 0.0 && rim(ib).lo() > 0.0;
        let inside = off(-near) && rim(ia).hi() < 0.0 && rim(ib).hi() < 0.0;
        Some((true, outside || inside))
    }

    /// **An arc read clear of another circle is clear of it in ℝ**
    /// ([`arc_clear_of_circle`]): a counterexample sweep ([`arc_case`]),
    /// every circle inside the arcs' reach ([`REACH`]). Two circles, the
    /// second of radius `10⁻²` … `10^REACH` m, externally or internally tangent but for
    /// `10⁻¹²` … `10⁻⁵` m either way, and an arc of the first, under half a
    /// turn, starting a small step (either sign, along it) past its point
    /// nearest or farthest from the second centre, at the scales of
    /// [`draw_scales`]. Wherever the certificate fires, its premises hold
    /// in ℝ, and a floor of fires keeps the row from passing empty. Inside
    /// the reach the extreme's uncharged rounding stays below `Kε`, so no
    /// soundness sweep at `f64` can tell its charge from none (the FULL
    /// review's oracle, PR 5552): [`an_arc_reads_clear_only_past_its_charge`]
    /// pins the charge's value instead.
    #[test]
    fn an_arc_read_clear_of_a_circle_is_clear_of_it() {
        let band = fine_band();
        let mut rng = test_utils::fuzz::start("seg-arc-clear-of-circle");
        let draws = test_utils::fuzz::scaled(20_000);
        let mut fired = 0usize;
        for _ in 0..draws {
            let (far, r1, step) = draw_scales(&mut rng);
            let c1 = (far * rng.range(-1.0, 1.0), far * rng.range(-1.0, 1.0));
            let r2 = decades(&mut rng, -2.0, REACH);
            let th = rng.range(0.0, core::f64::consts::TAU);
            let gap = signed(&mut rng) * decades(&mut rng, -12.0, -5.0);
            let d = if rng.range(0.0, 1.0) < 0.5 {
                r1 + r2 + gap
            } else {
                (r1 - r2).abs() + gap
            };
            if d <= 0.0 {
                continue;
            }
            let c2 = (c1.0 + d * th.cos(), c1.1 + d * th.sin());
            let extreme = if rng.range(0.0, 1.0) < 0.5 {
                th
            } else {
                th + core::f64::consts::PI
            };
            let sweep = rng.range(0.1, 3.0) * signed(&mut rng);
            let start = extreme + sweep.signum() * step / r1;
            if let Some((true, holds)) = arc_case((c1, r1), (c2, r2), (start, sweep), band) {
                fired += 1;
                assert!(
                    holds,
                    "read clear, but in ℝ neither half holds: c1 {c1:?} r1 {r1}, c2 {c2:?} \
                     r2 {r2}, start {start} sweep {sweep} — {}",
                    test_utils::fuzz::replay()
                );
            }
        }
        assert!(
            fired * 20 >= draws,
            "only {fired} fires in {draws} draws — {}",
            test_utils::fuzz::replay()
        );
    }

    /// **An arc reads clear of another circle only past its extreme's
    /// charge.** A circle of radius `10³` m about `(1500.3, −1700.7)`, near
    /// the arcs' reach at the fine band ([`REACH`]), a circle of radius 5 m
    /// externally `10⁻⁶` m clear of it along the direction `0.65`, and an
    /// arc of the first turning one radian away from its point facing the
    /// second, starting `step` along the circle past it. The extreme's
    /// charge there is about `1.7·10⁻¹¹` m. At a step of `10⁻⁸` m the
    /// certificate fires, and its premises hold in ℝ. At `2·10⁻¹¹` m it
    /// does not: the step is past `Kε = 10⁻¹¹`, so with the extreme's
    /// charge dropped it would read as an exclusion.
    #[test]
    fn an_arc_reads_clear_only_past_its_charge() {
        let band = fine_band();
        let (c1, r1, r2, th) = ((1500.3, -1700.7), 1e3, 5.0, 0.65);
        let d = r1 + r2 + 1e-6;
        let c2 = (c1.0 + d * th.cos(), c1.1 + d * th.sin());
        let read = |step: f64| arc_case((c1, r1), (c2, r2), (th + step / r1, 1.0), band);
        assert_eq!(
            read(1e-8),
            Some((true, true)),
            "a step of 1e-8 m past the extreme"
        );
        assert_eq!(
            read(2e-11),
            Some((false, true)),
            "a step of 2e-11 m, inside the extreme's charge, must not read clear"
        );
    }
}
