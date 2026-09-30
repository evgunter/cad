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

use geom_core::k_stats::{decide, decide_reported};
use geom_core::{Arc2, Band, Decide, Indeterminate, Margin, MarginDiag, Point2, Real, Sign, Vec2};

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
    /// The chord vector `b − a`.
    pub chord: Vec2<T>,
    /// The chord length |b − a| (definitely positive — degeneracy is
    /// rejected before a `Seg` is built).
    pub len: T,
    /// The unit chord direction `chord / len`.
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
/// apex and span chord the membership margins are written on.
pub(crate) struct ArcGeom<T: Real> {
    /// The canonical segment's carrier and signed sweep.
    pub arc: Arc2<T>,
    /// The arc's apex (its midpoint — the point farthest from the
    /// chord).
    pub apex: Point2<T>,
    /// |a − apex|: the chordal span threshold for membership (a carrier
    /// point q lies on the arc iff |q − apex| ≤ this — chord length is
    /// monotone in angular distance up to π, and the apex splits the arc
    /// into two halves of angle |θ|/2 ≤ π).
    pub span_chord: T,
    /// The turn sense: `Positive` = counterclockwise sweep,
    /// `Negative` = clockwise.
    pub turn: Sign,
}

/// Why a segment could not be built, each arm carrying the margin the
/// predicate that refused it classified — what a caller's sentence
/// names when it reports the refusal.
pub(crate) enum SegIssue<T: Real> {
    /// The chord is degenerate: consecutive vertices coincident at
    /// tolerance (`vertex_separation` classified Zero — or Negative,
    /// unreachable for a true distance but mapped here defensively).
    Degenerate {
        /// The chord length |b − a|, meters.
        margin: T,
    },
    /// The arc is within tolerance of a full circle
    /// (`arc_diameter_clearance` classified Zero — or Negative, only
    /// reachable through rounding at the boundary).
    NearFull {
        /// The diameter clearance 2r − |a − apex|, meters.
        margin: T,
    },
    /// The stored arc disagrees with its vertices: one of validation's
    /// three consistency checks classified the stored carrier or sweep
    /// definitely off (`arc_start_on_carrier`, `arc_landing`,
    /// `arc_sweep_range`).
    Inconsistent {
        /// Which check refused it.
        check: ArcCheck,
        /// The margin that check classified, meters.
        margin: T,
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
        /// The difference that check read, meters.
        value: T,
        /// What that check classified.
        margin: MarginDiag,
        /// What `arc_carrier_resolution` classified: K·ε/2⁻⁵² − scale,
        /// how far the check's magnitude sits below the one where
        /// `f64` rounding reaches the escalation band (here, not below
        /// it).
        headroom: MarginDiag,
    },
    /// A classification landed in the ambiguity band or was poisoned.
    Escalated(Indeterminate),
}

impl<T: Real> SegIssue<T> {
    /// The predicate that refused the segment — the name the run's own
    /// funnel recorded, so a caller reporting the refusal names what
    /// the classification named.
    pub(crate) fn predicate(&self) -> &'static str {
        match self {
            Self::Degenerate { .. } => "vertex_separation",
            Self::NearFull { .. } => "arc_diameter_clearance",
            Self::Inconsistent { check, .. } => check.predicate(),
            Self::BelowSceneResolution { .. } => "arc_carrier_resolution",
            Self::Escalated(source) => source.predicate.unwrap_or("<unnamed>"),
        }
    }
}

/// The chord frame of the segment a → b: its length, chord vector,
/// unit direction, midpoint and left unit normal — computed ONCE, in
/// one spelling, for every expression written on it: the segment's
/// own predicates ([`build_seg`]), the arc carrier ([`arc_carrier`])
/// at the lowering, and the validated form's lift, which rebuilds a
/// carried arc's carrier at the target scalar from the same frame.
pub(crate) struct ChordFrame<T: Real> {
    /// |b − a|.
    pub len: T,
    /// b − a.
    pub chord: Vec2<T>,
    /// (b − a) / |b − a|.
    pub unit: Vec2<T>,
    /// The chord's midpoint.
    pub mid: Point2<T>,
    /// The chord's left unit normal (the apex side of a
    /// counterclockwise arc is −normal).
    pub normal: Vec2<T>,
}

impl<T: Real> ChordFrame<T> {
    /// The frame of a → b. Total: a zero-length chord yields a poisoned
    /// unit and normal, which the `vertex_separation` gate in
    /// [`build_seg`] refuses before anything reads them.
    pub(crate) fn of(a: Point2<T>, b: Point2<T>) -> Self {
        let len = a.distance(b);
        let chord = b - a;
        let unit = chord / len;
        let mid = a.lerp(b, T::from_f64(0.5));
        let normal = perp(unit);
        Self {
            len,
            chord,
            unit,
            mid,
            normal,
        }
    }

    /// The midpoint of the arc on this frame whose sweep's quarter
    /// tangent is `quarter_tan` (tan(Δθ/4), [`quarter_tan`]): the
    /// sagitta L·tan(Δθ/4)/2 off the chord's midpoint, against the left
    /// normal (a counterclockwise arc bows right). The one spelling of
    /// an arc's apex.
    pub(crate) fn apex(&self, quarter_tan: T) -> Point2<T> {
        self.mid - self.normal * (self.len * quarter_tan * T::from_f64(0.5))
    }
}

/// tan(Δθ/4) of a sweep, spelled `sin(Δθ/4) / cos(Δθ/4)`: the signed
/// sagitta over the half-chord, the chord-scale reading of how far an
/// arc bows. The quotient rather than `tan` because the symbolic tier
/// folds `sin` and `cos` of a lowered sweep's `atan` (rule D) and holds
/// `tan` as an opaque atom.
pub(crate) fn quarter_tan<T: Real>(sweep: T) -> T {
    let quarter = sweep * T::from_f64(0.25);
    quarter.sin() / quarter.cos()
}

/// [`arc_carrier`]'s answer.
pub(crate) struct ArcCarrier<T: Real> {
    /// The carrier circle's center.
    pub center: Point2<T>,
    /// The carrier circle's radius (positive).
    pub radius: T,
}

/// The carrier of the arc on `frame` with `bulge`: the center at
/// apothem L·(1 − b²)/(4b) along the frame's normal from its midpoint,
/// the radius |L·(1 + b²)/(4b)|. Pure arithmetic over the segment's
/// input values — no predicate runs here — and the ONE spelling of
/// it: the bulge mode's lowering mints an arc's carrier through this
/// ([`crate::lower_to`]).
pub(crate) fn arc_carrier<T: Real>(frame: &ChordFrame<T>, bulge: T) -> ArcCarrier<T> {
    let b2 = bulge.powi(2);
    let four_bulge = T::from_f64(4.0) * bulge;
    let apothem = frame.len * (T::one() - b2) / four_bulge;
    let signed_radius = frame.len * (T::one() + b2) / four_bulge;
    ArcCarrier {
        center: frame.mid + frame.normal * apothem,
        radius: signed_radius.abs(),
    }
}

/// Whether [`build_seg`] decides an arc's three consistency checks
/// (D1): a table's arcs are verified here, and an arc a construction
/// built was verified at that construction, at this scalar, so the
/// checks are not decided again.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Consistency {
    /// Decide the three checks (a table: data no construction produced
    /// — the fixture door, a hand-built loop, `map_scalar`, a pinned
    /// lift).
    Decide,
    /// The arc is the construction's own output, verified there: by the
    /// construction's own predicate (a `Center` arc's
    /// `path_arc_center_equidistant`), or by the exact witness of the
    /// identities the lowering registers. Passed by every validation of
    /// a constructed loop ([`crate::ConstructedProfile`]) and by the path
    /// door's re-read of the fillet arcs it has just lowered.
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
/// - **`segment_straightness`** — margin: the signed sagitta
///   s = (L/2)·tan(Δθ/4) (meters): the apex's offset from the chord,
///   zero for a stored line. This is the dimensionless quarter-sweep
///   tangent acting through its lever arm, the half-chord L/2. Zero ⇒
///   line (the arc is chord-coincident at tolerance, and its carrier is
///   not carried); a definite sign ⇒ arc with that turn sense; in-band
///   ⇒ a sliver arc, escalated.
/// - **The three consistency checks** (arcs only, and only under
///   [`Consistency::Decide`]; D1's table arcs, "verified at validate
///   as ε-decisions"), each refused typed
///   ([`SegIssue::Inconsistent`]) on a definite answer it does not
///   accept:
///   - **`arc_start_on_carrier`** — margin: ‖a − c‖ − r (meters,
///     direct; [`Arc2::rim`]). Zero ⇒ the start lies on the carrier.
///   - **`arc_landing`** — margin: ‖point_from(a, 1) − b‖ (meters,
///     direct; [`Arc2::landing`]). Zero ⇒ the start, turned by the
///     sweep about the centre, lands on the end.
///   - **`arc_sweep_range`** — margin: r·|Δθ|·(2π − |Δθ|)/2π (meters:
///     the dimensionless range product levered by the radius — the arc
///     length r·|Δθ| near an empty sweep and the gap r·(2π − |Δθ|)
///     near a full turn), with |Δθ| the sweep signed by the decided
///     turn. Positive ⇒ 0 < |Δθ| < 2π; a sweep of the other sign than
///     its sagitta, or past a full turn, is Negative. A full turn is
///     Zero here and refused, as no loop of two or more vertices can
///     hold one past `vertex_separation`.
/// - **`arc_diameter_clearance`** (arcs only) — margin:
///   2r − |a − apex| = 2r·(1 − sin(|Δθ|/4)) (meters, computed in the
///   second form): how far the arc's half-span chord sits below the
///   carrier diameter, ≈ L²/(16r) for a near-full arc of
///   endpoint chord L. Zero ⇒ the arc is within tolerance of a full
///   circle — rejected as [`SegIssue::NearFull`] (the angular gap g
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
    let frame = ChordFrame::of(a, b);
    let len = frame.len;
    match decide("vertex_separation", Margin::of(len), band).map_err(SegIssue::Escalated)? {
        Sign::Positive => {}
        Sign::Zero | Sign::Negative => return Err(SegIssue::Degenerate { margin: len }),
    }
    let half = T::from_f64(0.5);
    // The decision is fired for every segment, since the K stream
    // records it for every segment, but only an arc's is read: a stored
    // line has no turn for its margin to report.
    let bow = match segment {
        Segment::Line => T::zero(),
        Segment::Arc(arc) => quarter_tan(arc.sweep),
    };
    let straightness = decide(
        "segment_straightness",
        Margin::levered(bow * half, len),
        band,
    )
    .map_err(SegIssue::Escalated)?;
    let kind = match segment {
        Segment::Line => SegKind::Line,
        Segment::Arc(..) if straightness == Sign::Zero => SegKind::Line,
        Segment::Arc(arc) => {
            let turn = straightness;
            let span = match turn {
                Sign::Negative => arc.reversed().sweep,
                Sign::Positive | Sign::Zero => arc.sweep,
            };
            match consistency {
                Consistency::Decide => check_carrier(arc, a, b, span, band)?,
                Consistency::ByConstruction => {}
            }
            let apex = frame.apex(bow);
            let span_chord = a.distance(apex);
            // 2r − |a − apex| with |a − apex| = 2r·sin(|Δθ|/4), spelled
            // on the carrier: the chord-scale apex carries tan(|Δθ|/4),
            // whose relative error grows as 1/(2π − |Δθ|) near a full
            // turn, while 1 − sin(|Δθ|/4) is flat there.
            let clearance =
                (arc.radius + arc.radius) * (T::one() - (span * T::from_f64(0.25)).sin());
            match decide("arc_diameter_clearance", Margin::of(clearance), band)
                .map_err(SegIssue::Escalated)?
            {
                Sign::Positive => {}
                Sign::Zero | Sign::Negative => {
                    return Err(SegIssue::NearFull { margin: clearance });
                }
            }
            SegKind::Arc(ArcGeom {
                arc,
                apex,
                span_chord,
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
const LANDING_ULPS: f64 = 16.0;

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
    value: T,
    margin: MarginDiag,
    scale: T,
    ulps: f64,
    band: Band,
) -> Result<(), SegIssue<T>> {
    let headroom = T::from_f64(band.escalate() / (ulps * f64::EPSILON)) - scale;
    let gate = decide_reported("arc_carrier_resolution", Margin::of(headroom), band)
        .map_err(SegIssue::Escalated)?;
    match gate.sign {
        Sign::Positive => Ok(()),
        Sign::Zero | Sign::Negative => Err(SegIssue::BelowSceneResolution {
            check,
            value,
            margin,
            headroom: gate.margin,
        }),
    }
}

/// Validation's three consistency checks of a stored arc against its
/// endpoints `a → b`, decided in order at the run's band ([`build_seg`]
/// states each margin). `span` is the sweep signed by the arc's decided
/// turn into |Δθ|, without an `abs`.
fn check_carrier<T: Decide>(
    arc: Arc2<T>,
    a: Point2<T>,
    b: Point2<T>,
    span: T,
    band: Band,
) -> Result<(), SegIssue<T>> {
    let refuse = |check: ArcCheck, margin: T| Err(SegIssue::Inconsistent { check, margin });
    // A difference check's definite answer is a statement about the
    // input only where the scene's `f64` can read a difference that
    // small at the magnitude it was taken at ([`resolves`]).
    let scale = arc.radius.max(reach(arc.centre));
    let on_carrier = arc.rim(a) - arc.radius;
    let read = decide_reported("arc_start_on_carrier", Margin::of(on_carrier), band)
        .map_err(SegIssue::Escalated)?;
    match read.sign {
        Sign::Zero => {}
        Sign::Positive | Sign::Negative => {
            resolves(
                ArcCheck::OnCarrier,
                on_carrier,
                read.margin,
                scale.max(reach(a)),
                ON_CARRIER_ULPS,
                band,
            )?;
            return refuse(ArcCheck::OnCarrier, on_carrier);
        }
    }
    let landing = arc.landing(a).distance(b);
    let read =
        decide_reported("arc_landing", Margin::of(landing), band).map_err(SegIssue::Escalated)?;
    match read.sign {
        Sign::Zero => {}
        Sign::Positive | Sign::Negative => {
            resolves(
                ArcCheck::Landing,
                landing,
                read.margin,
                scale.max(reach(a)).max(reach(b)),
                LANDING_ULPS,
                band,
            )?;
            return refuse(ArcCheck::Landing, landing);
        }
    }
    let tau = T::tau();
    let ratio = span * (tau - span) / tau;
    let range = ratio * arc.radius;
    match decide("arc_sweep_range", Margin::levered(ratio, arc.radius), band)
        .map_err(SegIssue::Escalated)?
    {
        Sign::Positive => Ok(()),
        Sign::Zero | Sign::Negative => refuse(ArcCheck::SweepRange, range),
    }
}

/// **`chord_side`** — which side of a segment's (infinite) chord line a
/// point lies on. Margin: the signed perpendicular distance
/// perp_dot(û, q − a) (meters; positive = left of the chord direction).
///
/// Returns the margin beside the sign because one caller — the joint
/// classification — has to REPORT what it classified, and the four
/// `line_line` callers below discard it deliberately: their question is
/// which side, and a distance they neither read nor render would be a
/// second value to keep in step with the first.
fn chord_side<T: Decide>(s: &Seg<T>, q: Point2<T>, band: Band) -> Result<(Sign, T), Indeterminate> {
    let margin = s.unit.perp_dot(q - s.a);
    Ok((decide("chord_side", Margin::of(margin), band)?, margin))
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
/// (`contact_at_shared_vertex` is an uncompressed direct distance).
fn arc_span<T: Decide>(g: &ArcGeom<T>, q: Point2<T>, band: Band) -> Result<Sign, Indeterminate> {
    decide(
        "arc_span",
        Margin::of(g.span_chord - q.distance(g.apex)),
        band,
    )
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
/// shared vertex — meet there (the #101 declared-tangency discipline's
/// classification vocabulary).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum JointClass {
    /// Distinct carriers in first-order (tangent) contact. Tangent
    /// carriers share exactly one point, and the shared vertex lies on
    /// both, so the tangency *is* at the joint. Must be declared.
    Tangent,
    /// Distinct carriers meeting transversally (or, defensively, a
    /// definitely-negative clearance, unreachable for carriers sharing
    /// a vertex): definitely not tangent. A declaration here is
    /// contradicted.
    Transversal,
    /// One shared carrier (collinear line/line, cocircular arc/arc —
    /// e.g. the minimal two-arc circle's joints): *continuation*, and
    /// legal both ways. Undeclared it is an ordinary continuation;
    /// DECLARED it is a declared tangent joint like any other — every
    /// zero-turn joint is one (Ev, in-chat, 2026-09-02), because
    /// identity is a fact about the carriers and tangency a fact about
    /// the directions, and the directions agree here. Both readers act
    /// on that: the verify layer's joint pass accepts it and the path
    /// door's stored-form read accepts it.
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
}

/// The magnitude a circle-pair clearance's arithmetic passes through:
/// the two radii and the two centres' own coordinates
/// ([`JointReading::scale`]).
fn circles_scale<T: Real>(g1: &ArcGeom<T>, g2: &ArcGeom<T>) -> T {
    g1.arc
        .radius
        .max(g2.arc.radius)
        .max(reach(g1.arc.centre))
        .max(reach(g2.arc.centre))
}

/// **`path_junction_side`** — whether a junction whose departure is
/// parallel to its arrival departs along it or REVERSES it (a cusp):
/// the alignment `cos φ` of the two unit headings, levered by the
/// arriving leg's arm. The one home of that question: the path door
/// asks it of a zero-turn junction it is about to refuse or declare,
/// and validation asks it of every declared tangent joint to record
/// which are cusps ([`crate::ValidatedLoop::cusp_joints`]).
///
/// `true` iff the alignment is definitely negative. A `Zero` reads as
/// NOT reversed — the arm itself is degenerate (both components
/// sub-ε), which the path door refuses as the tangent class and which
/// a validated loop cannot reach (its legs are definitely non-degenerate
/// and its declared joints verified tangent, so the margin is ± the
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

/// **An arc leg's lever arm**: the smaller of its carrier's radius and
/// its chord. The radius is what an angular margin displaces over; the
/// chord bounds it for an arc shorter than its own radius, where the
/// radius would overstate how far the leg actually reaches.
pub(crate) fn arc_lever<T: Real>(radius: T, chord: T) -> T {
    radius.min(chord)
}

impl<T: Real> Seg<T> {
    /// The leg's lever arm at a junction: a line's length, an arc's
    /// [`arc_lever`].
    pub(crate) fn arm(&self) -> T {
        match &self.kind {
            SegKind::Line => self.len,
            SegKind::Arc(g) => arc_lever(g.arc.radius, self.len),
        }
    }

    /// The unit heading of the traversal at `p`, one of the segment's
    /// endpoints: a line's chord direction; an arc's counterclockwise
    /// carrier tangent at `p`, reversed on a clockwise turn.
    pub(crate) fn heading_at(&self, p: Point2<T>) -> Vec2<T> {
        match &self.kind {
            SegKind::Line => self.unit,
            SegKind::Arc(g) => {
                let ccw = perp(p - g.arc.centre) * (T::one() / g.arc.radius);
                match g.turn {
                    Sign::Negative => -ccw,
                    Sign::Positive | Sign::Zero => ccw,
                }
            }
        }
    }
}

/// Classifies the joint between two adjacent segments — `prev` arrives
/// at the shared vertex, `next` leaves it (the classification is
/// symmetric; the roles only name the arguments). This is the single
/// per-junction tangency question of the #101 discipline, kept
/// separate from profile-wide validation so future authoring layers
/// can ask it per-junction.
///
/// Decisions reuse the pair-classification predicates: the carrier
/// clearance margins (`carrier_line_circle`,
/// `carrier_circles_identity` / `_external` / `_internal`) are
/// bit-identical to [`line_arc`]/[`arc_arc`]'s expressions; line/line
/// joints reuse `chord_side` in the same expression form on the
/// joint's *far* endpoint (a carrier-identity question — the pair pass
/// asks it of other points). Same funnel, same bands, no new ε.
/// In-band or poisoned margins escalate verbatim.
pub(crate) fn joint_tangency<T: Decide>(
    prev: &Seg<T>,
    next: &Seg<T>,
    band: Band,
) -> Result<JointReading<T>, Indeterminate> {
    match (&prev.kind, &next.kind) {
        (SegKind::Line, SegKind::Line) => {
            // Distinct lines are never tangent; the only Zero question
            // is carrier identity (collinearity). The shared vertex is
            // on both carriers, so identity ⟺ the far endpoint of one
            // lies on the other's carrier line.
            let (side, margin) = chord_side(prev, next.b, band)?;
            Ok(JointReading {
                class: match side {
                    Sign::Zero => JointClass::SameCarrier,
                    Sign::Positive | Sign::Negative => JointClass::Transversal,
                },
                predicate: "chord_side",
                margin,
                scale: reach(next.b).max(reach(prev.a)),
            })
        }
        (SegKind::Line, SegKind::Arc(g)) => line_circle_joint(prev, g, band),
        (SegKind::Arc(g), SegKind::Line) => line_circle_joint(next, g, band),
        (SegKind::Arc(g1), SegKind::Arc(g2)) => {
            let d = g1.arc.centre.distance(g2.arc.centre);
            let dr = (g1.arc.radius - g2.arc.radius).abs();
            let identity = d + dr;
            match decide("carrier_circles_identity", Margin::of(identity), band)? {
                Sign::Zero | Sign::Negative => Ok(JointReading {
                    class: JointClass::SameCarrier,
                    predicate: "carrier_circles_identity",
                    margin: identity,
                    scale: circles_scale(g1, g2),
                }),
                Sign::Positive => {
                    let external = d - (g1.arc.radius + g2.arc.radius);
                    match decide("carrier_circles_external", Margin::of(external), band)? {
                        Sign::Zero => Ok(JointReading {
                            class: JointClass::Tangent,
                            predicate: "carrier_circles_external",
                            margin: external,
                            scale: circles_scale(g1, g2),
                        }),
                        // Positive external clearance (disjoint) is
                        // unreachable for carriers sharing a vertex —
                        // defensively definite non-tangency.
                        Sign::Positive => Ok(JointReading {
                            class: JointClass::Transversal,
                            predicate: "carrier_circles_external",
                            margin: external,
                            scale: circles_scale(g1, g2),
                        }),
                        Sign::Negative => {
                            let internal = d - dr;
                            Ok(JointReading {
                                class: match decide(
                                    "carrier_circles_internal",
                                    Margin::of(internal),
                                    band,
                                )? {
                                    Sign::Zero => JointClass::Tangent,
                                    Sign::Positive => JointClass::Transversal,
                                    // Nested carriers: unreachable with a
                                    // shared vertex; defensive.
                                    Sign::Negative => JointClass::Transversal,
                                },
                                predicate: "carrier_circles_internal",
                                margin: internal,
                                scale: circles_scale(g1, g2),
                            })
                        }
                    }
                }
            }
        }
    }
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
/// Every reader of this clearance goes through here — the joint
/// classification, the pair contact and the ray cast — so the three
/// cannot drift apart.
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
    p.x.abs().max(p.y.abs())
}

/// The line/circle joint core: `carrier_line_circle` on the same
/// margin expression as [`line_arc`] ([`carrier_line_circle_margin`]).
fn line_circle_joint<T: Decide>(
    line: &Seg<T>,
    g: &ArcGeom<T>,
    band: Band,
) -> Result<JointReading<T>, Indeterminate> {
    let (margin, scale) = carrier_line_circle_margin(line.unit, line.a, g);
    Ok(JointReading {
        class: match decide("carrier_line_circle", Margin::of(margin), band)? {
            Sign::Zero => JointClass::Tangent,
            Sign::Positive => JointClass::Transversal,
            // A definitely-disjoint carrier pair cannot share a vertex —
            // defensively definite non-tangency.
            Sign::Negative => JointClass::Transversal,
        },
        predicate: "carrier_line_circle",
        margin,
        scale,
    })
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
pub(crate) fn pair_contacts<T: Decide>(
    s1: &Seg<T>,
    s2: &Seg<T>,
    band: Band,
) -> Result<PairOutcome<T>, Indeterminate> {
    match (&s1.kind, &s2.kind) {
        (SegKind::Line, SegKind::Line) => line_line(s1, s2, band),
        (SegKind::Line, SegKind::Arc(g2)) => line_arc(s1, g2, band),
        (SegKind::Arc(g1), SegKind::Line) => line_arc(s2, g1, band),
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

fn joint(m1: Sign, m2: Sign) -> Option<Joint> {
    match (m1, m2) {
        (Sign::Negative, _) | (_, Sign::Negative) => None,
        (Sign::Positive, Sign::Positive) => Some(Joint::Interior),
        _ => Some(Joint::Boundary),
    }
}

/// Line/line contacts via the four `chord_side` orientations (the
/// Shewchuk-style segment test, trileanized). Strictly opposite sides
/// both ways ⇒ one proper crossing; any Zero ⇒ endpoint-on-carrier
/// configurations resolved by `line_span`; both of one segment's
/// endpoints on the other's carrier ⇒ collinear, resolved by the
/// **`collinear_overlap`** predicate — margin: the length of the
/// parameter-interval intersection (meters; positive = overlap, Zero =
/// endpoint touch, negative = disjoint), computed with lattice min/max
/// (value operations, not branches).
fn line_line<T: Decide>(
    s1: &Seg<T>,
    s2: &Seg<T>,
    band: Band,
) -> Result<PairOutcome<T>, Indeterminate> {
    let (o_c, _) = chord_side(s1, s2.a, band)?;
    let (o_d, _) = chord_side(s1, s2.b, band)?;
    if o_c == Sign::Zero && o_d == Sign::Zero {
        // Collinear carriers: 1-D overlap on s1's arc-length axis.
        let tc = (s2.a - s1.a).dot(s1.unit);
        let td = (s2.b - s1.a).dot(s1.unit);
        let lo = tc.min(td).max(T::zero());
        let hi = tc.max(td).min(s1.len);
        return Ok(
            match decide("collinear_overlap", Margin::of(hi - lo), band)? {
                Sign::Positive => PairOutcome::Overlap,
                Sign::Zero => {
                    let mid = (lo + hi) * T::from_f64(0.5);
                    PairOutcome::Contacts(vec![Contact {
                        point: s1.a + s1.unit * mid,
                        kind: CKind::Touch,
                    }])
                }
                Sign::Negative => PairOutcome::Contacts(Vec::new()),
            },
        );
    }
    let (o_a, _) = chord_side(s2, s1.a, band)?;
    let (o_b, _) = chord_side(s2, s1.b, band)?;
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
        // meet exactly once, at that endpoint — check span membership.
        for (o, q) in [(o_c, s2.a), (o_d, s2.b)] {
            if o == Sign::Zero && line_span(s1, q, band)? != Sign::Negative {
                contacts.push(Contact {
                    point: q,
                    kind: CKind::Touch,
                });
            }
        }
        for (o, q) in [(o_a, s1.a), (o_b, s1.b)] {
            if o == Sign::Zero && line_span(s2, q, band)? != Sign::Negative {
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
fn line_arc<T: Decide>(
    line: &Seg<T>,
    g: &ArcGeom<T>,
    band: Band,
) -> Result<PairOutcome<T>, Indeterminate> {
    let to_center = g.arc.centre - line.a;
    let h = line.unit.perp_dot(to_center);
    let (clearance, _) = carrier_line_circle_margin(line.unit, line.a, g);
    let mut contacts = Vec::new();
    match decide("carrier_line_circle", Margin::of(clearance), band)? {
        Sign::Negative => {}
        Sign::Zero => {
            let foot = line.a + line.unit * to_center.dot(line.unit);
            if let Some(j) = joint(line_span(line, foot, band)?, arc_span(g, foot, band)?) {
                contacts.push(Contact {
                    point: foot,
                    kind: match j {
                        Joint::Interior => CKind::Tangency,
                        Joint::Boundary => CKind::Touch,
                    },
                });
            }
        }
        Sign::Positive => {
            let tc = to_center.dot(line.unit);
            let half = (g.arc.radius.powi(2) - h.powi(2)).sqrt();
            for t in [tc - half, tc + half] {
                let q = line.a + line.unit * t;
                if let Some(j) = joint(line_span(line, q, band)?, arc_span(g, q, band)?) {
                    contacts.push(Contact {
                        point: q,
                        kind: match j {
                            Joint::Interior => CKind::Crossing,
                            Joint::Boundary => CKind::Touch,
                        },
                    });
                }
            }
        }
    }
    Ok(PairOutcome::Contacts(contacts))
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
///   external clearance. Positive ⇒ separate; Zero ⇒ externally
///   tangent at the point dividing C₁C₂ in ratio r₁ : r₂.
/// - **`carrier_circles_internal`** — margin: d − |r₁ − r₂|, the
///   internal clearance. Negative ⇒ nested, no contact; Zero ⇒
///   internally tangent (contact on the center line beyond both
///   centers); Positive (with external Negative) ⇒ two proper
///   intersections at the standard radical-line closed form — the
///   radicand h² = r₁² − a² factors through both clearances, so it is
///   definitely positive in this branch and cannot poison.
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
    let delta = g2.arc.centre - g1.arc.centre;
    let d = g1.arc.centre.distance(g2.arc.centre);
    let dr = (g1.arc.radius - g2.arc.radius).abs();
    match decide("carrier_circles_identity", Margin::of(d + dr), band)? {
        Sign::Zero | Sign::Negative => {
            // Cocircular: span overlap on the shared carrier.
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
            if coincident("arc_apex_identity", g1.apex, g2.apex, band)? == Sign::Zero {
                // Same endpoints AND same apex: identical spans.
                return Ok(PairOutcome::Overlap);
            }
            Ok(PairOutcome::Contacts(contacts))
        }
        Sign::Positive => {
            let mut contacts = Vec::new();
            let sum = g1.arc.radius + g2.arc.radius;
            match decide("carrier_circles_external", Margin::of(d - sum), band)? {
                Sign::Positive => {}
                Sign::Zero => {
                    // Externally tangent; d = r₁ + r₂ ≥ the definite
                    // identity margin, so the division is safe.
                    let q = g1.arc.centre + delta * (g1.arc.radius / d);
                    push_arc_arc_contact(&mut contacts, g1, g2, q, true, band)?;
                }
                Sign::Negative => {
                    match decide("carrier_circles_internal", Margin::of(d - dr), band)? {
                        Sign::Negative => {}
                        Sign::Zero => {
                            // Internally tangent: d = |Δr| and the
                            // identity margin d + |Δr| = 2d is definite,
                            // so d is bounded away from zero.
                            let two_d = d + d;
                            let a =
                                (d.powi(2) + g1.arc.radius.powi(2) - g2.arc.radius.powi(2)) / two_d;
                            let q = g1.arc.centre + (delta / d) * a;
                            push_arc_arc_contact(&mut contacts, g1, g2, q, true, band)?;
                        }
                        Sign::Positive => {
                            // Proper secant: d > |Δr| definitely, so
                            // d > 0. Radical-line closed form.
                            let u = delta / d;
                            let n = perp(u);
                            let two_d = d + d;
                            let a =
                                (d.powi(2) + g1.arc.radius.powi(2) - g2.arc.radius.powi(2)) / two_d;
                            let h = (g1.arc.radius.powi(2) - a.powi(2)).sqrt();
                            let foot = g1.arc.centre + u * a;
                            for q in [foot + n * h, foot - n * h] {
                                push_arc_arc_contact(&mut contacts, g1, g2, q, false, band)?;
                            }
                        }
                    }
                }
            }
            Ok(PairOutcome::Contacts(contacts))
        }
    }
}

/// Span-checks one candidate carrier-intersection point of an arc/arc
/// pair and pushes the classified contact. `tangent` selects the
/// interior kind (tangency vs crossing).
fn push_arc_arc_contact<T: Decide>(
    contacts: &mut Vec<Contact<T>>,
    g1: &ArcGeom<T>,
    g2: &ArcGeom<T>,
    q: Point2<T>,
    tangent: bool,
    band: Band,
) -> Result<(), Indeterminate> {
    if let Some(j) = joint(arc_span(g1, q, band)?, arc_span(g2, q, band)?) {
        contacts.push(Contact {
            point: q,
            kind: match (j, tangent) {
                (Joint::Interior, true) => CKind::Tangency,
                (Joint::Interior, false) => CKind::Crossing,
                (Joint::Boundary, _) => CKind::Touch,
            },
        });
    }
    Ok(())
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
