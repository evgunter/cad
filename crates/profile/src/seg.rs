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

use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Indeterminate, Margin, Point2, Real, Sign, Vec2};

/// The left normal: `v` rotated +90° counterclockwise, (−y, x).
pub(crate) fn perp<T: Real>(v: Vec2<T>) -> Vec2<T> {
    Vec2::new(-v.y, v.x)
}

/// A classified segment of a loop: the chord data plus the carrier
/// geometry its bulge implies.
pub(crate) struct Seg<T: Real> {
    /// Start point.
    pub a: Point2<T>,
    /// End point.
    pub b: Point2<T>,
    /// The stored bulge (tan(θ/4); crate-doc sign convention).
    pub bulge: T,
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

/// Derived arc geometry (closed forms from the crate docs' bulge
/// identities).
pub(crate) struct ArcGeom<T: Real> {
    /// The carrier circle's center.
    pub center: Point2<T>,
    /// The carrier circle's radius (positive).
    pub radius: T,
    /// The arc's apex (its midpoint — the point farthest from the
    /// chord).
    pub apex: Point2<T>,
    /// |a − apex|: the chordal span threshold for membership (a carrier
    /// point q lies on the arc iff |q − apex| ≤ this — chord length is
    /// monotone in angular distance up to π, and the apex splits the arc
    /// into two halves of angle |θ|/2 ≤ π).
    pub span_chord: T,
    /// The turn sense: `Positive` = counterclockwise sweep (bulge > 0),
    /// `Negative` = clockwise.
    pub turn: Sign,
}

/// Why a segment could not be built.
pub(crate) enum SegIssue {
    /// The chord is degenerate: consecutive vertices coincident at
    /// tolerance (`vertex_separation` classified Zero — or Negative,
    /// unreachable for a true distance but mapped here defensively).
    Degenerate,
    /// The arc is within tolerance of a full circle
    /// (`arc_diameter_clearance` classified Zero — or Negative, only
    /// reachable through rounding at the boundary).
    NearFull,
    /// A classification landed in the ambiguity band or was poisoned.
    Escalated(Indeterminate),
}

/// Builds and classifies a segment.
///
/// Predicates fired, in order:
///
/// - **`vertex_separation`** — margin: the chord length |b − a|
///   (meters, a direct displacement; no lever arm). Zero ⇒ degenerate
///   segment (the Q1 sliver semantics: within band ⇒ escalation).
/// - **`segment_straightness`** — margin: the signed sagitta
///   s = L·bulge/2 (meters): the apex's offset from the chord. This is
///   the dimensionless bulge acting through its lever arm, the
///   half-chord L/2. Zero ⇒ line (the arc is chord-coincident at
///   tolerance); a definite sign ⇒ arc with that turn sense; in-band ⇒
///   a sliver arc, escalated.
/// - **`arc_diameter_clearance`** (arcs only) — margin:
///   2r − |a − apex| (meters): how far the arc's half-span chord sits
///   below the carrier diameter, ≈ L²/(16r) for a near-full arc of
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
    bulge: T,
    band: Band,
) -> Result<Seg<T>, SegIssue> {
    let len = a.distance(b);
    match decide("vertex_separation", Margin::of(len), band).map_err(SegIssue::Escalated)? {
        Sign::Positive => {}
        Sign::Zero | Sign::Negative => return Err(SegIssue::Degenerate),
    }
    let chord = b - a;
    let unit = chord / len;
    let half = T::from_f64(0.5);
    let sagitta = len * bulge * half;
    let kind = match decide(
        "segment_straightness",
        Margin::levered(bulge * half, len),
        band,
    )
    .map_err(SegIssue::Escalated)?
    {
        Sign::Zero => SegKind::Line,
        turn => {
            let mid = a.lerp(b, half);
            let n = perp(unit);
            let b2 = bulge.powi(2);
            let four_bulge = T::from_f64(4.0) * bulge;
            let apothem = len * (T::one() - b2) / four_bulge;
            let signed_radius = len * (T::one() + b2) / four_bulge;
            let center = mid + n * apothem;
            let apex = mid - n * sagitta;
            let radius = signed_radius.abs();
            let span_chord = a.distance(apex);
            match decide(
                "arc_diameter_clearance",
                Margin::of(radius + radius - span_chord),
                band,
            )
            .map_err(SegIssue::Escalated)?
            {
                Sign::Positive => {}
                Sign::Zero | Sign::Negative => return Err(SegIssue::NearFull),
            }
            SegKind::Arc(ArcGeom {
                center,
                radius,
                apex,
                span_chord,
                turn,
            })
        }
    };
    Ok(Seg {
        a,
        b,
        bulge,
        chord,
        len,
        unit,
        kind,
    })
}

/// **`chord_side`** — which side of a segment's (infinite) chord line a
/// point lies on. Margin: the signed perpendicular distance
/// perp_dot(û, q − a) (meters; positive = left of the chord direction).
fn chord_side<T: Decide>(s: &Seg<T>, q: Point2<T>, band: Band) -> Result<Sign, Indeterminate> {
    decide("chord_side", Margin::of(s.unit.perp_dot(q - s.a)), band)
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
    /// e.g. the minimal two-arc circle's joints): *continuation*, not
    /// tangency — legal undeclared; a declaration here is contradicted
    /// (there is no second carrier to be tangent to).
    SameCarrier,
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
) -> Result<JointClass, Indeterminate> {
    match (&prev.kind, &next.kind) {
        (SegKind::Line, SegKind::Line) => {
            // Distinct lines are never tangent; the only Zero question
            // is carrier identity (collinearity). The shared vertex is
            // on both carriers, so identity ⟺ the far endpoint of one
            // lies on the other's carrier line.
            Ok(match chord_side(prev, next.b, band)? {
                Sign::Zero => JointClass::SameCarrier,
                Sign::Positive | Sign::Negative => JointClass::Transversal,
            })
        }
        (SegKind::Line, SegKind::Arc(g)) => line_circle_joint(prev, g, band),
        (SegKind::Arc(g), SegKind::Line) => line_circle_joint(next, g, band),
        (SegKind::Arc(g1), SegKind::Arc(g2)) => {
            let d = g1.center.distance(g2.center);
            let dr = (g1.radius - g2.radius).abs();
            match decide("carrier_circles_identity", Margin::of(d + dr), band)? {
                Sign::Zero | Sign::Negative => Ok(JointClass::SameCarrier),
                Sign::Positive => {
                    match decide(
                        "carrier_circles_external",
                        Margin::of(d - (g1.radius + g2.radius)),
                        band,
                    )? {
                        Sign::Zero => Ok(JointClass::Tangent),
                        // Positive external clearance (disjoint) is
                        // unreachable for carriers sharing a vertex —
                        // defensively definite non-tangency.
                        Sign::Positive => Ok(JointClass::Transversal),
                        Sign::Negative => {
                            Ok(
                                match decide("carrier_circles_internal", Margin::of(d - dr), band)?
                                {
                                    Sign::Zero => JointClass::Tangent,
                                    Sign::Positive => JointClass::Transversal,
                                    // Nested carriers: unreachable with a
                                    // shared vertex; defensive.
                                    Sign::Negative => JointClass::Transversal,
                                },
                            )
                        }
                    }
                }
            }
        }
    }
}

/// The line/circle joint core: `carrier_line_circle` on the same
/// margin expression as [`line_arc`] (r − |h|).
fn line_circle_joint<T: Decide>(
    line: &Seg<T>,
    g: &ArcGeom<T>,
    band: Band,
) -> Result<JointClass, Indeterminate> {
    let h = line.unit.perp_dot(g.center - line.a);
    Ok(
        match decide("carrier_line_circle", Margin::of(g.radius - h.abs()), band)? {
            Sign::Zero => JointClass::Tangent,
            Sign::Positive => JointClass::Transversal,
            // A definitely-disjoint carrier pair cannot share a vertex —
            // defensively definite non-tangency.
            Sign::Negative => JointClass::Transversal,
        },
    )
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
    let o_c = chord_side(s1, s2.a, band)?;
    let o_d = chord_side(s1, s2.b, band)?;
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
    let o_a = chord_side(s2, s1.a, band)?;
    let o_b = chord_side(s2, s1.b, band)?;
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
    let to_center = g.center - line.a;
    let h = line.unit.perp_dot(to_center);
    let mut contacts = Vec::new();
    match decide("carrier_line_circle", Margin::of(g.radius - h.abs()), band)? {
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
            let half = (g.radius.powi(2) - h.powi(2)).sqrt();
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
    let delta = g2.center - g1.center;
    let d = g1.center.distance(g2.center);
    let dr = (g1.radius - g2.radius).abs();
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
            let sum = g1.radius + g2.radius;
            match decide("carrier_circles_external", Margin::of(d - sum), band)? {
                Sign::Positive => {}
                Sign::Zero => {
                    // Externally tangent; d = r₁ + r₂ ≥ the definite
                    // identity margin, so the division is safe.
                    let q = g1.center + delta * (g1.radius / d);
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
                            let a = (d.powi(2) + g1.radius.powi(2) - g2.radius.powi(2)) / two_d;
                            let q = g1.center + (delta / d) * a;
                            push_arc_arc_contact(&mut contacts, g1, g2, q, true, band)?;
                        }
                        Sign::Positive => {
                            // Proper secant: d > |Δr| definitely, so
                            // d > 0. Radical-line closed form.
                            let u = delta / d;
                            let n = perp(u);
                            let two_d = d + d;
                            let a = (d.powi(2) + g1.radius.powi(2) - g2.radius.powi(2)) / two_d;
                            let h = (g1.radius.powi(2) - a.powi(2)).sqrt();
                            let foot = g1.center + u * a;
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
            let to_center = g.center - origin;
            let h = dir.perp_dot(to_center);
            match decide("carrier_line_circle", Margin::of(g.radius - h.abs()), band)
                .map_err(|_| Graze)?
            {
                Sign::Negative => Ok(0),
                Sign::Zero => Err(Graze),
                Sign::Positive => {
                    let tc = to_center.dot(dir);
                    let half = (g.radius.powi(2) - h.powi(2)).sqrt();
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
