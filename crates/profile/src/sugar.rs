//! Constructor sugar: human-friendly arc forms lowered to bulges.
//!
//! The **stored** form is always the bulge chain (the crate docs'
//! zero-consistency-conditions rule); sugar may take direction hints
//! ([`ArcSweep`]) or through-points, but nothing beyond the computed
//! bulge survives into the data.
//!
//! Sugar is *evaluation code*: total, comparison-free, no decisions —
//! with ONE documented exception: **the fillet constructor's gates**
//! ([`LoopBuilder::fillet`] / [`LoopBuilder::fillet_corner`]), reified
//! predicates through the k_stats funnel. The original (#101 review
//! MAJOR-1) is `fillet_leg_fit`, which refuses a radius whose tangent
//! points fall outside their legs; M5 S2's arc-leg corners add
//! `fillet_leg_reach` (the corner-side extent test, same exact-order
//! band) and, against the run's linear band, `fillet_corner_arm`,
//! `fillet_corner_turn` and the `fillet_offset_*` carrier-intersection
//! family. They are the one place sugar decides, and they decide only
//! *which construction the author asked for* — never a geometric
//! verdict about the finished loop, which stays
//! [`crate::Profile::validate`]'s. Everything else holds everywhere:
//! degenerate inputs (a through-point collinear-outside its chord, a
//! zero-radius center) produce well-defined poison or degenerate values
//! that [`crate::Profile::validate`] rejects with typed errors — the
//! sugar never guesses and never panics.

use geom_core::{Band, Bounds, Decide, Indeterminate, Point2, Real, Sign, Tolerance, Vec2};

use crate::k_stats::decide;
use crate::validate::{EscalationSite, FilletLeg, FilletLegCarrier, NoCornerReason, ProfileError};
use crate::{ProfileLoop, ProfileVertex};

/// The sweep direction hint for [`bulge_from_center`] /
/// [`LoopBuilder::close_arc_center`]: which way the arc winds about its
/// center (a hint consumed by sugar — the stored bulge carries the same
/// information as its sign).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArcSweep {
    /// Counterclockwise sweep (positive included angle; positive bulge).
    Ccw,
    /// Clockwise sweep (negative included angle; negative bulge).
    Cw,
}

/// The shape of one leg of a fillet corner
/// ([`LoopBuilder::fillet_corner`]).
///
/// A circular leg is named by its **carrier** — the circle about
/// `center` through the corner, swept `sweep` in chain order — rather
/// than by a bulge, because a fillet trims its legs and a bulge
/// describes only the untrimmed chord (see `fillet_corner`'s docs). The
/// carrier is trim-invariant, so one value describes the leg on both
/// sides of any trim.
#[derive(Clone, Copy, Debug)]
pub enum FilletLegShape<T: Real> {
    /// A straight leg: the segment between the corner and the leg's far
    /// end (the chain head for the incoming leg, `next` for the
    /// outgoing one).
    Line,
    /// A circular leg on the carrier about `center` through the corner,
    /// running in the `sweep` sense in chain order.
    Arc {
        /// The carrier circle's center in sketch coordinates; the
        /// carrier's radius is |corner − center|.
        center: Point2<T>,
        /// Which way the leg winds about `center` in chain order.
        sweep: ArcSweep,
    },
}

/// The bulge of the arc from `a` to `b` passing through `via`.
///
/// By the inscribed-angle theorem the arc's included angle is
/// θ = 2·Δ, where Δ is the signed turn from chord `a`→`via` to chord
/// `via`→`b` (independent of where on the arc `via` sits); the bulge is
/// tan(θ/4) = tan(Δ/2). Computed exactly in that form (fixed order,
/// D9): Δ = atan2(perp_dot(d₁, d₂), d₁·d₂), result = tan(Δ/2).
///
/// **Degenerate inputs, honestly:** `via` collinear between `a` and `b`
/// gives Δ = 0 ⇒ bulge 0 (the line segment the three points describe);
/// `via` collinear *outside* the chord gives Δ = ±π ⇒ tan(±π/2), an
/// infinite/huge value whose downstream geometry validation rejects;
/// `via` coincident with `a` or `b` makes one chord zero and the turn
/// ill-defined — atan2(0, 0) = 0 at `f64`, so the result degrades to a
/// line-ish bulge that validation judges on its merits. Total, never a
/// panic; the sugar does not decide (no predicates in evaluation code).
///
/// **Session-box gap (deferred D4 ¶4 item — see the crate docs):** a
/// *near*-collinear-outside `via` produces a finite but astronomically
/// large bulge — a carrier of ~1e15 m radius that today validates if
/// the loop is simple. Until kernel-wide session-box enforcement
/// lands, callers own the sanity of through-points.
pub fn bulge_from_via<T: Real>(a: Point2<T>, via: Point2<T>, b: Point2<T>) -> T {
    let d1 = via - a;
    let d2 = b - via;
    let turn = d1.perp_dot(d2).atan2(d1.dot(d2));
    (turn / T::from_f64(2.0)).tan()
}

/// The bulge of the arc from `a` to `b` about `center`, sweeping in the
/// `sweep` direction.
///
/// The included angle is the angular displacement from `a` to `b` as
/// seen from `center`, reduced into [0, 2π) for [`ArcSweep::Ccw`] (and
/// its negative-period mirror for [`ArcSweep::Cw`]) via
/// [`Real::reduce_periodic`]; the bulge is tan(θ/4). Fixed evaluation
/// order as written (D9).
///
/// **The center is a hint, not stored data**: the stored segment is
/// chord + bulge, whose implied center is the perpendicular-bisector
/// point at the implied radius. If `b` does not lie on the circle
/// through `a` about `center`, the stored arc still runs `a`→`b` with
/// the computed sweep — the intent's angles, the chord's geometry.
/// Coincident endpoints (θ that reduces to 0) or a center coincident
/// with an endpoint produce degenerate/poison values for validation to
/// reject; total, never a panic.
pub fn bulge_from_center<T: Real>(
    a: Point2<T>,
    b: Point2<T>,
    center: Point2<T>,
    sweep: ArcSweep,
) -> T {
    let va = a - center;
    let vb = b - center;
    let phi_a = va.y.atan2(va.x);
    let phi_b = vb.y.atan2(vb.x);
    let ccw = (phi_b - phi_a).reduce_periodic(T::tau());
    let theta = match sweep {
        ArcSweep::Ccw => ccw,
        ArcSweep::Cw => ccw - T::tau(),
    };
    (theta / T::from_f64(4.0)).tan()
}

/// A chain builder: `start` → (`line_to` | `arc_to` | `arc_to_via` |
/// `arc_to_center`)* → one `close*` call, yielding a [`ProfileLoop`].
///
/// Each step appends a vertex and sets the bulge of the segment
/// *arriving* at it on the previous vertex; the `close*` variants set
/// the implicit closing segment's bulge on the last vertex (closure
/// itself is by construction — there is no way to build an open chain).
#[derive(Clone, Debug)]
pub struct LoopBuilder<T: Real> {
    vertices: Vec<ProfileVertex<T>>,
    tangent: Vec<usize>,
}

impl<T: Real> LoopBuilder<T> {
    /// Starts a chain at `start`.
    pub fn start(start: Point2<T>) -> Self {
        Self {
            vertices: vec![ProfileVertex {
                pos: start,
                bulge: T::zero(),
            }],
            tangent: Vec::new(),
        }
    }

    /// The current chain end (the last vertex's position).
    fn head(&self) -> Point2<T> {
        // The vector is nonempty by construction (`start` seeds it).
        self.vertices[self.vertices.len() - 1].pos
    }

    /// Sets the bulge of the segment leaving the current last vertex.
    fn set_leaving_bulge(&mut self, bulge: T) {
        let last = self.vertices.len() - 1;
        self.vertices[last].bulge = bulge;
    }

    /// Appends a straight segment to `p`.
    pub fn line_to(mut self, p: Point2<T>) -> Self {
        self.set_leaving_bulge(T::zero());
        self.vertices.push(ProfileVertex {
            pos: p,
            bulge: T::zero(),
        });
        self
    }

    /// Appends an arc segment to `p` with an explicit `bulge` (the raw
    /// form — see the crate docs for the sign convention).
    pub fn arc_to(mut self, p: Point2<T>, bulge: T) -> Self {
        self.set_leaving_bulge(bulge);
        self.vertices.push(ProfileVertex {
            pos: p,
            bulge: T::zero(),
        });
        self
    }

    /// Appends the arc through `via` ending at `p` (three-point form;
    /// see [`bulge_from_via`]).
    pub fn arc_to_via(self, via: Point2<T>, p: Point2<T>) -> Self {
        let bulge = bulge_from_via(self.head(), via, p);
        self.arc_to(p, bulge)
    }

    /// Appends the arc about `center` ending at `p`, sweeping `sweep`
    /// (center form; see [`bulge_from_center`]).
    pub fn arc_to_center(self, p: Point2<T>, center: Point2<T>, sweep: ArcSweep) -> Self {
        let bulge = bulge_from_center(self.head(), p, center, sweep);
        self.arc_to(p, bulge)
    }

    /// Declares the **joint at the current chain end** tangent: the
    /// junction between the segment arriving at the last vertex and
    /// the segment that will leave it (the next `*_to`/`close*` call —
    /// or, called on a fresh builder, the junction between the closing
    /// segment and the first). The explicit hand-authoring form of the
    /// #101 discipline ([`crate::ProfileLoop::tangent_joints`]);
    /// validation *verifies* the claim and refuses
    /// `TangencyContradicted` if the carriers are definitely not
    /// tangent. Prefer [`LoopBuilder::fillet`], which computes tangent
    /// geometry and declares it by construction.
    pub fn declare_tangent(mut self) -> Self {
        self.tangent.push(self.vertices.len() - 1);
        self
    }

    /// Appends a **tangent fillet** rounding the STRAIGHT corner the
    /// chain is heading for — the primary authoring path of the #101
    /// discipline (constructive, solver-free): from the chain end H, a
    /// straight leg toward `corner` C stopping at the tangent point T₁, then
    /// the radius-`radius` arc to the tangent point T₂ on the outgoing
    /// leg C→`next`; **the joints at strictly-interior tangent points
    /// are declared tangent by construction** (exact-fit sides: see
    /// the gate below). The caller must continue toward `next`
    /// (`line_to(next)`, or another `fillet` whose `corner` is `next`)
    /// — leaving T₂ any other way is a contradicted declaration, which
    /// validation refuses loudly.
    ///
    /// Closed form, exact where inputs are exact (D9: fixed order; no
    /// transcendentals — one square root per length, the "usual sqrt
    /// forms"): with legs v₁ = C − H, v₂ = `next` − C and
    /// m = √(‖v₁‖²·‖v₂‖²),
    /// tan(φ/2) = v₁×v₂ / (m + v₁·v₂) (φ = the corner's turn angle),
    /// setback = r·|tan(φ/2)|, T₁/T₂ = C ∓ setback·v̂₁/₂, and the arc's
    /// bulge = tan(φ/4) by the quarter-angle identity
    /// tan(φ/4) = tan(φ/2) / (1 + √(1 + tan²(φ/2))). For a
    /// right-angle corner with dyadic legs everything but the bulge is
    /// *bit-exact* (tan(φ/2) = ±1), and the residual carrier-clearance
    /// margin is rounding-level (~1e-16) — definite Zero at every
    /// supported ε.
    ///
    /// # The leg-fit gate (the one decision sugar takes)
    ///
    /// A tangent point outside its leg means the arc never approaches
    /// the corner the caller asked to round — a silent intent mismatch
    /// (the exact disease this discipline refuses), and one the
    /// resulting loop can VALIDATE through (the overshot arc is still
    /// carrier-tangent at both points; #101 review, MAJOR-1). The
    /// constructor therefore refuses it: each leg's fit margin
    /// (leg length − setback, meters) is classified through the
    /// reified **`fillet_leg_fit`** predicate against the exact-order
    /// band (`crate::validate` module docs — the representation-exact
    /// device, not a new ε; the knife-edge neighborhoods on BOTH sides
    /// are covered by validation's `vertex_separation` door, so exact
    /// classification is honest here):
    ///
    /// - **Negative** (either leg) ⇒ [`ProfileError::FilletDoesNotFit`]
    ///   naming the first overrun leg with setback/length diagnostics;
    /// - **Zero** (exact fit) ⇒ that side's straight piece has zero
    ///   length and is **not emitted**, and — having no collinear
    ///   straight piece adjacent — that tangent point is **not
    ///   declared**: an exact-fit incoming leg springs the arc directly
    ///   off the chain head (the head joint's tangency, if the caller
    ///   arranged one, is the caller's to declare); an exact-fit
    ///   outgoing leg ends the arc at `next` itself (continue from the
    ///   corner AT `next` — a `line_to(next)` would be degenerate);
    /// - **Positive** ⇒ the normal emission (straight piece + declared
    ///   tangent joint);
    /// - in-band / poisoned (interval knife-edge, NaN legs) ⇒
    ///   [`ProfileError::Escalated`] at
    ///   [`crate::EscalationSite::Fillet`] — this is also where a
    ///   doubled-back corner (φ = ±π) or zero-length leg lands: the
    ///   0/0 → NaN closed form poisons the fit margin, refused typed
    ///   at the constructor door.
    ///
    /// Remaining degenerates, honestly (executed, pinned in the
    /// `declared_tangency` suite): a straight-through corner (φ = 0)
    /// or `radius` = 0 yields setback 0, T₁ = T₂ = C, and a degenerate
    /// zero-length arc that validation refuses as `DegenerateSegment`;
    /// a negative radius puts the tangent points beyond the corner and
    /// the loop fails simplicity (`NonSimple`/`Crossing`). Never a
    /// panic, never a guess, never a silently-wrong shape.
    ///
    /// # Errors
    ///
    /// [`ProfileError::FilletDoesNotFit`], [`ProfileError::Escalated`]
    /// (site [`crate::EscalationSite::Fillet`]), or
    /// [`ProfileError::Band`] (unreachable for the built-in band) — see
    /// the gate above.
    ///
    /// # Arc legs
    ///
    /// This form is the line×line corner. Corners with a circular leg
    /// (line×arc, arc×line, arc×arc) go through
    /// [`LoopBuilder::fillet_corner`], which is the same construction
    /// stated on offset carriers; `fillet_corner` with two straight legs
    /// delegates here, so the two doors emit bit-identical geometry.
    pub fn fillet(self, corner: Point2<T>, next: Point2<T>, radius: T) -> Result<Self, ProfileError>
    where
        T: Decide + Bounds,
    {
        let v1 = corner - self.head();
        let v2 = next - corner;
        // powi(2)-discipline squares (interval lane: a straddling-zero
        // factor must not poison the enclosure — memories/
        // interval-square-poison.md); norm_squared is powi inside.
        let m = (v1.norm_squared() * v2.norm_squared()).sqrt();
        let half_tan = v1.perp_dot(v2) / (m + v1.dot(v2));
        let bulge = half_tan / (T::one() + (T::one() + half_tan.powi(2)).sqrt());
        let setback = radius * half_tan.abs();
        let len1 = v1.norm_squared().sqrt();
        let len2 = v2.norm_squared().sqrt();
        // The exact-order band (validate module docs): no representable
        // f64 lies strictly inside it, so f64 classification is total.
        let exact = Band::new(f64::from_bits(1), f64::from_bits(2)).map_err(ProfileError::Band)?;
        let fit = |len: T, leg: FilletLeg| -> Result<Sign, ProfileError> {
            match decide("fillet_leg_fit", len - setback, exact) {
                Ok(Sign::Negative) => Err(ProfileError::FilletDoesNotFit {
                    leg,
                    carrier: FilletLegCarrier::Line,
                    setback: setback.lo(),
                    leg_length: len.lo(),
                }),
                Ok(sign) => Ok(sign),
                Err(source) => Err(ProfileError::Escalated {
                    site: EscalationSite::Fillet,
                    source,
                }),
            }
        };
        let fit_in = fit(len1, FilletLeg::Incoming)?;
        let fit_out = fit(len2, FilletLeg::Outgoing)?;
        let t2 = corner + v2 * (setback / len2);
        let mut chain = self;
        if fit_in == Sign::Positive {
            let t1 = corner - v1 * (setback / len1);
            chain = chain.line_to(t1).declare_tangent();
        }
        chain = chain.arc_to(t2, bulge);
        if fit_out == Sign::Positive {
            chain = chain.declare_tangent();
        }
        Ok(chain)
    }

    /// Appends a **tangent fillet** rounding a corner whose legs may be
    /// straight *or circular* — the M5 S2 generalization of
    /// [`LoopBuilder::fillet`] (#101 R4): line×arc, arc×line, arc×arc,
    /// and (by delegation) line×line.
    ///
    /// The corner is the meeting point `corner` C of the **incoming**
    /// leg (chain head H → C, shaped by `incoming`) and the **outgoing**
    /// leg (C → `next` N, shaped by `outgoing`). As in
    /// [`LoopBuilder::fillet`] this emits the trimmed incoming leg, then
    /// the radius-`radius` arc from T₁ to T₂, declaring the
    /// strictly-interior tangent joints **by construction**; the caller
    /// continues from T₂ toward `next` — `line_to(next)` for a straight
    /// outgoing leg, `arc_to_center(next, center, sweep)` for a circular
    /// one (or another `fillet_corner` whose `corner` is `next`).
    /// Leaving T₂ any other way is a contradicted declaration, refused
    /// loudly by validation.
    ///
    /// # Why arc legs are given by CARRIER, not bulge
    ///
    /// [`FilletLegShape::Arc`] names the leg's carrier circle
    /// (`center` + sweep sense), not a bulge, because a fillet **trims**
    /// its legs: a bulge describes the untrimmed chord H→C, and after
    /// trimming the chain runs T₁→C on the same circle with a different
    /// bulge. The carrier is trim-invariant, so the same
    /// [`FilletLegShape`] value describes the leg before and after any
    /// trim — including the next corner's incoming leg. The carrier is
    /// the circle about `center` **through the corner** (radius
    /// R = |C − center|); the leg's far end contributes only its angular
    /// position, so an off-carrier far end still bounds the extent by
    /// its angle (the conventional-data posture of
    /// [`LoopBuilder::arc_to_center`]).
    ///
    /// # Construction: offset carriers (solver-free, D9)
    ///
    /// The fillet's center P is the intersection of the two legs'
    /// **offset carriers**, each pushed by r toward the side the corner
    /// turns to (σ = the sign of the corner's turn):
    ///
    /// - a straight leg's carrier offsets to the parallel line through
    ///   C + σ·r·n̂ (n̂ the left normal of the leg's travel direction);
    /// - a circular leg of radius R and own sweep sense τ offsets to the
    ///   concentric circle of **signed** radius ρ = R − σ·τ·r —
    ///   internal tangency (ρ = R − r) when the fillet curves the same
    ///   way as the leg, external (ρ = R + r) when it curves the other
    ///   way. The sign carries the enclosing case (r > R) without a
    ///   branch: the tangent point is `center + (P − center)·(R/ρ)`
    ///   either way.
    ///
    /// Tangent points are then exact from P: the foot along n̂ on a
    /// straight leg, along the center-to-center direction on a circular
    /// one. The arc's bulge is `tan(θ/4)` in the half-angle form
    /// σ·(L/2) / (r + copysign(|M − P|, σ·(T₁−P)×(T₂−P))) with L the
    /// chord |T₂ − T₁| and M its midpoint — algebraic, one square root
    /// per length, correct for major arcs (|θ| > π) too.
    ///
    /// The tangent points, the fillet centre and the FILLET arc's bulge
    /// stay in those sqrt forms, and the gate quantities use `atan2`
    /// (gate inputs, not emitted geometry). The one emitted number that
    /// is *not* algebraic is the trimmed circular leg's own bulge: that
    /// piece is re-emitted through [`LoopBuilder::arc_to_center`], which
    /// derives its bulge from the endpoints' angles about the centre —
    /// `atan2` then `tan`. That is the pre-existing contract of the
    /// `arc_to_center` door (this constructor adds no transcendental of
    /// its own there), but it does mean "every emitted coordinate stays
    /// in sqrt forms" is true of the fillet arc, not of the trimmed leg.
    ///
    /// # The branch rule (deterministic, never a guess)
    ///
    /// Two carriers meet in at most two points, so at most two circles
    /// of radius r are tangent to both legs' carriers. Candidates whose
    /// tangent points lie on the legs' **corner-side extents** survive —
    /// on each leg, `0 ≤ setback ≤ extent`, the two ends classified by
    /// `fillet_leg_reach` and `fillet_leg_fit` against the exact-order
    /// band. Outcomes:
    ///
    /// - exactly one survivor ⇒ that is the fillet;
    /// - **two** survivors ⇒ the candidate **nearest the authored
    ///   corner** wins (M5 S8, Evan's ruling 2026-07-30): smallest total
    ///   tangent setback, ties falling down [`nearest_candidate`]'s
    ///   documented ladder. Both survivors are valid tangent fillets of
    ///   the authored legs, so the selection is between constructions,
    ///   not geometric truths — and the far circle stays deliberately
    ///   authorable as the NEAR fillet of the other carrier intersection
    ///   (the S8 reachability rows pin both directions);
    /// - none, but some candidate did reach the corner side and only
    ///   overran a leg's far end ⇒ [`ProfileError::FilletDoesNotFit`]
    ///   (the radius-does-not-fit situation, first candidate and
    ///   incoming leg first);
    /// - none at all ⇒ [`ProfileError::NoCornerForFillet`], either
    ///   because the offset carriers never meet
    ///   (`OffsetCarriersDisjoint`) or because every tangent circle
    ///   touches past the corner (`NoCornerSideCandidate`).
    ///
    /// # The gates
    ///
    /// In fixed order (D9), each a reified predicate through the k_stats
    /// funnel; every escalation lands at
    /// [`crate::EscalationSite::Fillet`] with the predicate named:
    ///
    /// 1. **`fillet_corner_arm`** (linear band) — the lever arm
    ///    `min(leg extents, circular legs' radii)`. Not definitely
    ///    positive ⇒ [`ProfileError::FilletLegDegenerate`]: with no
    ///    length scale the corner's angle means nothing (D4 ¶1; the
    ///    `dihedral_arm` gate's sibling), so nothing is classified.
    /// 2. **`fillet_corner_turn`** (linear band) — the turn margin
    ///    `sin φ · arm` in meters, φ the angle from the incoming to the
    ///    outgoing travel direction at C. Its sign is σ; Zero ⇒
    ///    [`ProfileError::FilletCornerAlreadyTangent`] (the legs meet
    ///    tangentially or double back — no corner to cut; declare the
    ///    tangency instead).
    /// 3. **`fillet_offset_line_circle`** / **`fillet_offset_circles_external`**
    ///    / **`fillet_offset_circles_internal`** (linear band) — the
    ///    offset carriers' clearances, which say whether there are two,
    ///    one, or no candidate centers.
    /// 4. **`fillet_leg_reach`** (exact-order band) — each candidate's
    ///    setback from the corner: the corner end of the extent test.
    /// 5. **`fillet_leg_fit`** (exact-order band) — leg extent − setback,
    ///    exactly as [`LoopBuilder::fillet`] gates it, but with the arc
    ///    leg's setback and extent measured as **arc lengths** `R·Δθ`.
    ///    Negative ⇒ [`ProfileError::FilletDoesNotFit`], now naming the
    ///    leg's carrier and carrying the angular margin `(extent −
    ///    setback)/R` in radians. On the chosen candidate, Zero (exact
    ///    fit) suppresses that side's trimmed piece and its declaration,
    ///    Positive emits both — the same three-way rule as the straight
    ///    case.
    ///
    /// # Errors
    ///
    /// [`ProfileError::FilletLegDegenerate`],
    /// [`ProfileError::FilletCornerAlreadyTangent`],
    /// [`ProfileError::NoCornerForFillet`],
    /// [`ProfileError::FilletDoesNotFit`], [`ProfileError::Escalated`]
    /// (site [`crate::EscalationSite::Fillet`]), or
    /// [`ProfileError::Band`] (only for a misconfigured ε) — see the
    /// gates above.
    pub fn fillet_corner(
        self,
        incoming: FilletLegShape<T>,
        corner: Point2<T>,
        outgoing: FilletLegShape<T>,
        next: Point2<T>,
        radius: T,
        tol: Tolerance,
    ) -> Result<Self, ProfileError>
    where
        T: Decide + Bounds,
    {
        let band = Band::new(tol.eps, tol.k * tol.eps).map_err(ProfileError::Band)?;
        // The exact-order band (validate module docs): no representable
        // f64 lies strictly inside it, so f64 classification is total.
        let exact = Band::new(f64::from_bits(1), f64::from_bits(2)).map_err(ProfileError::Band)?;
        let leg_in = Leg::at_corner(incoming, corner, self.head(), FilletLeg::Incoming);
        let leg_out = Leg::at_corner(outgoing, corner, next, FilletLeg::Outgoing);

        // (1) the lever arm: an angle is nothing without one (D4 ¶1).
        let arm = leg_in.arm.min(leg_out.arm);
        if decide("fillet_corner_arm", arm, band).map_err(fillet_escalated)? != Sign::Positive {
            return Err(ProfileError::FilletLegDegenerate {
                // Diagnostic-channel comparison (naming the leg for the
                // message), not a geometric decision.
                leg: if leg_in.arm.lo() <= leg_out.arm.lo() {
                    FilletLeg::Incoming
                } else {
                    FilletLeg::Outgoing
                },
                arm: arm.lo(),
            });
        }

        // (2) the corner's turn: its sign is the side both carriers
        // offset toward, so the offset construction never searches.
        let turn = leg_in.dir.perp_dot(leg_out.dir) * arm;
        let sgn = match decide("fillet_corner_turn", turn, band).map_err(fillet_escalated)? {
            Sign::Positive => T::one(),
            Sign::Negative => -T::one(),
            Sign::Zero => {
                return Err(ProfileError::FilletCornerAlreadyTangent {
                    reversed: leg_in.dir.dot(leg_out.dir).lo() < 0.0,
                    margin: turn.lo(),
                    arm: arm.lo(),
                });
            }
        };

        // (3) the offset carriers' intersection — the candidate centers.
        let centers = match (leg_in.arc, leg_out.arc) {
            // Two straight legs: the ratified line×line closed form,
            // bit-identical to `fillet`'s (one door, one construction).
            (None, None) => return self.fillet(corner, next, radius),
            (Some(arc), None) => leg_out.offset_line_circle(corner, sgn, radius, arc, band)?,
            (None, Some(arc)) => leg_in.offset_line_circle(corner, sgn, radius, arc, band)?,
            (Some(a), Some(b)) => a.offset_circles(b, sgn, radius, band)?,
        };

        // (4/5) the branch rule and the fit gate, one pass in candidate
        // order: a candidate survives when both tangent points lie
        // *within* their legs' corner-side extents — setback ≥ 0
        // (`fillet_leg_reach`, the corner end; SIGNED on both leg kinds,
        // so "past the corner" really does classify Negative) and
        // extent − setback ≥ 0 (`fillet_leg_fit`, the far end,
        // generalized to arc lengths R·Δθ on a circular leg).
        //
        // Four classifications per candidate, always all four — no
        // short-circuit, so the recorded sample sequence depends only on
        // the corner CLASS (how many candidates the carriers admit), not
        // on the numbers. That invariance is stated for the
        // non-escalating path: any one of the four `decide` calls may
        // return Indeterminate, and the `?` below aborts the whole
        // constructor with that escalation, leaving the remaining
        // classifications of this candidate and every later candidate
        // unfired. This is deliberate — an escalation means the gate
        // cannot be classified at this scalar, and continuing would be
        // deciding the branch rule on a margin we just admitted we
        // cannot read. The escalation names which predicate stopped it.
        let mut surviving: Vec<(Candidate<T>, Sign, Sign, [f64; 2])> =
            Vec::with_capacity(centers.len());
        let mut overrun: Option<ProfileError> = None;
        for center in centers {
            let t1 = leg_in.tangent_point(center, sgn, radius);
            let t2 = leg_out.tangent_point(center, sgn, radius);
            let sb_in = leg_in.setback(t1, corner);
            let sb_out = leg_out.setback(t2, corner);
            let reach_in = decide("fillet_leg_reach", sb_in, exact).map_err(fillet_escalated)?;
            let reach_out = decide("fillet_leg_reach", sb_out, exact).map_err(fillet_escalated)?;
            let margin_in = leg_in.len - sb_in;
            let margin_out = leg_out.len - sb_out;
            let fit_in = decide("fillet_leg_fit", margin_in, exact).map_err(fillet_escalated)?;
            let fit_out = decide("fillet_leg_fit", margin_out, exact).map_err(fillet_escalated)?;
            let corner_side = reach_in != Sign::Negative && reach_out != Sign::Negative;
            if corner_side && fit_in != Sign::Negative && fit_out != Sign::Negative {
                surviving.push((
                    Candidate { center, t1, t2 },
                    fit_in,
                    fit_out,
                    [sb_in.lo(), sb_out.lo()],
                ));
            } else if corner_side && overrun.is_none() {
                // This candidate rounds the corner the caller named, but
                // the radius pushes a tangent point off the far end of
                // its leg: the radius-does-not-fit situation, reported
                // incoming leg first exactly as `fillet` gates it.
                //
                // Attribution is sound because `corner_side` is now a
                // real test (signed setback, review MAJOR-1): a candidate
                // rounding the OTHER intersection of the two carriers has
                // a tangent point past the corner, classifies Negative,
                // and never reaches this arm. So the setback and leg
                // length rendered below are this candidate's own numbers,
                // for the corner the author actually named — not a
                // wrap-around distance to a corner they never mentioned.
                let (leg, setback, margin) = if fit_in == Sign::Negative {
                    (&leg_in, sb_in, margin_in)
                } else {
                    (&leg_out, sb_out, margin_out)
                };
                overrun = Some(ProfileError::FilletDoesNotFit {
                    leg: leg.side,
                    carrier: leg.carrier_diag(margin),
                    setback: setback.lo(),
                    leg_length: leg.len.lo(),
                });
            }
        }
        if surviving.is_empty() {
            return Err(overrun.unwrap_or(ProfileError::NoCornerForFillet {
                reason: NoCornerReason::NoCornerSideCandidate,
                radius: radius.lo(),
            }));
        }
        let setbacks: Vec<[f64; 2]> = surviving.iter().map(|s| s.3).collect();
        let (picked, fit_in, fit_out, _) = surviving[nearest_candidate(&setbacks)];

        let bulge = fillet_bulge(picked.t1, picked.t2, picked.center, radius, sgn);
        let mut chain = self;
        if fit_in == Sign::Positive {
            chain = match leg_in.arc {
                None => chain.line_to(picked.t1),
                Some(arc) => chain.arc_to_center(picked.t1, arc.center, arc.sweep),
            };
            chain = chain.declare_tangent();
        }
        chain = chain.arc_to(picked.t2, bulge);
        if fit_out == Sign::Positive {
            chain = chain.declare_tangent();
        }
        Ok(chain)
    }

    /// The finished loop: vertices plus the declared-tangent joints
    /// accumulated by `declare_tangent`/`fillet`.
    fn build(self) -> ProfileLoop<T> {
        ProfileLoop {
            vertices: self.vertices,
            tangent_joints: self.tangent,
        }
    }

    /// Closes the chain with a straight segment back to the start.
    pub fn close(mut self) -> ProfileLoop<T> {
        self.set_leaving_bulge(T::zero());
        self.build()
    }

    /// Closes the chain with an arc of the given `bulge` back to the
    /// start.
    pub fn close_with_bulge(mut self, bulge: T) -> ProfileLoop<T> {
        self.set_leaving_bulge(bulge);
        self.build()
    }

    /// Closes the chain with the arc through `via` back to the start.
    pub fn close_arc_via(self, via: Point2<T>) -> ProfileLoop<T> {
        let first = self.vertices[0].pos;
        let bulge = bulge_from_via(self.head(), via, first);
        self.close_with_bulge(bulge)
    }

    /// Closes the chain with the arc about `center` back to the start,
    /// sweeping `sweep`.
    pub fn close_arc_center(self, center: Point2<T>, sweep: ArcSweep) -> ProfileLoop<T> {
        let first = self.vertices[0].pos;
        let bulge = bulge_from_center(self.head(), first, center, sweep);
        self.close_with_bulge(bulge)
    }
}

// ------------------------------------------------------------------
// The arc-leg fillet's private machinery (M5 S2). Everything here is
// evaluation code except the `decide` calls, which are the documented
// exception named in the module docs.
// ------------------------------------------------------------------

/// Wraps a gate escalation at the fillet constructor's site.
fn fillet_escalated(source: Indeterminate) -> ProfileError {
    ProfileError::Escalated {
        site: EscalationSite::Fillet,
        source,
    }
}

/// Nearest-the-authored-corner branch selection (M5 S8; Evan's ruling,
/// in-chat 2026-07-30): the index of the surviving candidate to build,
/// given each survivor's `[incoming, outgoing]` tangent setbacks in
/// meters (arc lengths `R·Δθ` on circular legs).
///
/// This is a **plain deterministic selection rule, not a Q1 predicate**
/// (amended per the follow-up ruling, same day): every surviving
/// candidate is a valid fillet, exactly tangent to both authored legs,
/// so choosing between them asserts nothing about geometric truth — and
/// below ε_input the author cannot have meant a distinguishable
/// preference (D4 ¶1). A within-ε pick is therefore
/// arbitrary-but-deterministic and both-valid; an author who wants the
/// other pocket forces it by authoring that pocket's own corner (the
/// far circle is always the near fillet of the second carrier
/// intersection — the S8 reachability rows pin this). No K-funnel
/// entry, no escalation arm, no error. It compares the f64
/// **diagnostic channel** (the enclosure's lower bound at interval
/// scalars — the same channel `FilletLegDegenerate`'s leg naming uses):
/// a representation-level choice between two already-classified
/// constructions, never a re-decision of geometry. The lanes' channels
/// differ by enclosure width, but the survivors' setback gap is
/// macroscopic next to it (the dominance argument below), so the lanes
/// pick the same candidate — the cross-lane row pins it.
///
/// # The ladder (fixed order)
///
/// 1. strict `<` on **total setback** (incoming + outgoing) — the near
///    pocket sets both tangent points back a little, the far pocket a
///    lot;
/// 2. exact tie ⇒ strict `<` on the **incoming** setback alone;
/// 3. both tied ⇒ the **first-classified** candidate (enumeration
///    order, incoming-leg-first construction order).
///
/// # Equivariance (`memories/equivariance-principle.md`)
///
/// Rungs 1–2 compare arc lengths, which are isometry-invariant, so in ℝ
/// the selection commutes with every rigid motion AND reflection of the
/// sketch (bitwise f64 equivariance is already forgone by D9's fixed
/// evaluation orders). Rung 3 cannot be equivariant: candidates carrying
/// identical per-leg setback pairs are exactly the class where a
/// candidate-swapping symmetry would have to swap the pick, so it falls
/// to enumeration order — the kernel's first knowingly-designed
/// non-equivariant residual, recorded as such.
///
/// # Why sum (and when the later rungs can even fire)
///
/// Among survivors of the corner-side extent gates, the nearer candidate
/// is nearer on BOTH legs, so every monotone combination (sum, max, …)
/// agrees and sum is the symmetric spelling. The argument, for
/// non-enclosing tangencies (offset radius ρ > 0): both candidates lie
/// on both offset circles, mirror-symmetric about the line L through the
/// offset centers; each tangent point is its candidate's same-side
/// radial projection, so the candidate on the corner's side of L is
/// angularly nearer the corner on both carriers at once. The corner
/// strictly off L and distinct candidates are guaranteed by the
/// `fillet_corner_turn` and offset-clearance gates, so the dominance is
/// strict and rungs 2–3 are unreachable through this constructor in
/// exact arithmetic — they exist as the total, documented fallback for
/// computed-value collisions on the diagnostic channel. Enclosing
/// tangencies (ρ < 0, the tangent point antipodally flipped) were never
/// observed to admit two corner-side survivors at all (S8 probe: 20M+
/// random corners across all leg classes plus a targeted
/// mixed-tangency search, zero hits), so the two-survivor situation
/// arises only in the non-enclosing lens class where the dominance
/// argument holds.
fn nearest_candidate(setbacks: &[[f64; 2]]) -> usize {
    let mut best = 0;
    for (i, pair) in setbacks.iter().enumerate().skip(1) {
        let (total, best_total) = (pair[0] + pair[1], setbacks[best][0] + setbacks[best][1]);
        if total < best_total || (total == best_total && pair[0] < setbacks[best][0]) {
            best = i;
        }
    }
    best
}

/// The angle swept from `from` to `to` about a center, in the `turn`
/// sense (+1 counterclockwise, −1 clockwise), reduced into [0, 2π) —
/// the arc-leg analogue of "distance along the leg".
///
/// This is the right reduction for a leg's own **extent**, which is a
/// forward sweep by construction and may legitimately exceed π. It is
/// the WRONG reduction for a setback, which must be able to come out
/// negative — see [`signed_swept`].
fn swept<T: Real>(from: T, to: T, turn: T) -> T {
    ((to - from) * turn).reduce_periodic(T::tau())
}

/// The **signed** angle swept from `from` to `to` in the `turn` sense,
/// reduced into [−π, π) — [`swept`]'s forward sweep folded so that
/// "more than half a turn forward" reads as "backward", which is what a
/// *setback* needs (review MAJOR-1).
///
/// The unsigned [`swept`] cannot express "behind the corner": a tangent
/// point one degree past the corner reads as a setback of 359°·R. With
/// that reading `fillet_leg_reach` could never classify Negative on an
/// arc leg, the `NoCornerSideCandidate` refusal was unreachable for
/// arc×arc, and an overrun refusal could render the wrap-around numbers
/// of the candidate rounding the OTHER carrier intersection while naming
/// the corner the author asked for.
///
/// # Why folding loses nothing
///
/// A fillet tangent point is never more than half a turn from the
/// corner. It lies on the ray from the leg's centre O through the
/// fillet centre P, so its angular offset from the corner is the
/// *unsigned* angle between `C − O` and `P − O`, which is in [0, π] by
/// definition. A genuine setback therefore always lies in the kept
/// half, however far the leg itself sweeps — a leg may legitimately
/// sweep more than π, and its extent still uses the unsigned [`swept`].
///
/// # Why the fold, and not `atan2` of the cross/dot
///
/// `atan2(τ·(u × w), u · w)` computes the same angle in one call, but as
/// a *different* floating-point expression from the one [`swept`] uses
/// for the leg's extent — and the exact-fit rows turn on `extent −
/// setback` being bit-zero when the tangent point lands on the leg's far
/// end. The fold below is `x − τ·⌊x/τ + ½⌋`, which for `x ∈ [0, π)`
/// multiplies τ by a floored zero and returns `x − 0` — **bit-identical
/// to [`swept`]**. So this changes the value only where the shipped code
/// was wrong (past the corner), and the knife-edge exactness the fit
/// gate relies on is preserved by construction rather than by luck.
fn signed_swept<T: Real>(from: T, to: T, turn: T) -> T {
    let forward = swept(from, to, turn);
    let tau = T::tau();
    forward - tau * (forward / tau + T::from_f64(0.5)).floor()
}

/// The +90° rotation of `v` (its left normal).
fn left_normal<T: Real>(v: Vec2<T>) -> Vec2<T> {
    Vec2::new(-v.y, v.x)
}

/// A circular leg's carrier: the circle about `center` through the
/// corner, with the leg's own sweep sense.
#[derive(Clone, Copy, Debug)]
struct ArcCarrier<T: Real> {
    /// The carrier circle's center.
    center: Point2<T>,
    /// Its radius, |corner − center|.
    radius: T,
    /// +1 for a counterclockwise leg, −1 for a clockwise one.
    turn: T,
    /// The same sense as the authored hint (for re-emitting the trimmed
    /// piece through `arc_to_center`).
    sweep: ArcSweep,
    /// The corner's angular coordinate on the carrier (radians).
    corner_angle: T,
}

/// One leg of a fillet corner, resolved at the corner.
#[derive(Clone, Copy, Debug)]
struct Leg<T: Real> {
    /// The path's unit travel direction **at the corner**.
    dir: Vec2<T>,
    /// The carrier, for a circular leg; `None` for a straight one.
    arc: Option<ArcCarrier<T>>,
    /// The leg's extent in meters (chord length, or arc length R·Δθ).
    len: T,
    /// The leg's lever arm: its extent, folded with its carrier radius
    /// (the curvature arm) for a circular leg.
    arm: T,
    /// Which side of the corner this leg is.
    side: FilletLeg,
}

/// A circular leg's **signed** offset radius ρ = R − σ·τ·r (the
/// construction section of [`LoopBuilder::fillet_corner`]).
fn offset_radius<T: Real>(arc: &ArcCarrier<T>, sgn: T, radius: T) -> T {
    arc.radius - sgn * arc.turn * radius
}

impl<T: Real> Leg<T> {
    /// Resolves `shape` into the leg between `corner` and `far` (the
    /// chain head for the incoming side, `next` for the outgoing one).
    ///
    /// Total: a zero-length straight leg or a corner-coincident arc
    /// center yields poison, which the constructor's arm gate refuses
    /// typed.
    fn at_corner(
        shape: FilletLegShape<T>,
        corner: Point2<T>,
        far: Point2<T>,
        side: FilletLeg,
    ) -> Self {
        match shape {
            FilletLegShape::Line => {
                let v = match side {
                    FilletLeg::Incoming => corner - far,
                    FilletLeg::Outgoing => far - corner,
                };
                let len = v.norm_squared().sqrt();
                Self {
                    dir: v / len,
                    arc: None,
                    len,
                    arm: len,
                    side,
                }
            }
            FilletLegShape::Arc { center, sweep } => {
                let turn = match sweep {
                    ArcSweep::Ccw => T::one(),
                    ArcSweep::Cw => -T::one(),
                };
                let to_corner = corner - center;
                let radius = to_corner.norm_squared().sqrt();
                // The tangent at the corner: τ·(C − O)⟂ / R.
                let dir = left_normal(to_corner) * (turn / radius);
                let corner_angle = to_corner.y.atan2(to_corner.x);
                let to_far = far - center;
                let far_angle = to_far.y.atan2(to_far.x);
                let len = radius
                    * match side {
                        FilletLeg::Incoming => swept(far_angle, corner_angle, turn),
                        FilletLeg::Outgoing => swept(corner_angle, far_angle, turn),
                    };
                Self {
                    dir,
                    arc: Some(ArcCarrier {
                        center,
                        radius,
                        turn,
                        sweep,
                        corner_angle,
                    }),
                    len,
                    // The curvature arm folded with the extent — the
                    // `dihedral_wedge` lever-arm rule (D4 ¶1).
                    arm: len.min(radius),
                    side,
                }
            }
        }
    }

    /// The exact tangent point of the radius-`radius` circle centered at
    /// `center` on this leg's carrier.
    fn tangent_point(&self, center: Point2<T>, sgn: T, radius: T) -> Point2<T> {
        match self.arc {
            None => center - left_normal(self.dir) * (sgn * radius),
            Some(arc) => {
                arc.center + (center - arc.center) * (arc.radius / offset_radius(&arc, sgn, radius))
            }
        }
    }

    /// The tangent point's setback from the corner, measured **along
    /// the leg** (meters; an arc length R·Δθ on a circular leg).
    /// **Positive on the corner side, negative past the corner** — on
    /// both leg kinds: a straight leg projects onto its travel
    /// direction, a circular one takes the signed sweep
    /// ([`signed_swept`], whose docs carry why the fold is both
    /// necessary and lossless here).
    fn setback(&self, tangent: Point2<T>, corner: Point2<T>) -> T {
        match self.arc {
            None => match self.side {
                FilletLeg::Incoming => (corner - tangent).dot(self.dir),
                FilletLeg::Outgoing => (tangent - corner).dot(self.dir),
            },
            Some(arc) => {
                let to_tangent = tangent - arc.center;
                let angle = to_tangent.y.atan2(to_tangent.x);
                arc.radius
                    * match self.side {
                        FilletLeg::Incoming => signed_swept(angle, arc.corner_angle, arc.turn),
                        FilletLeg::Outgoing => signed_swept(arc.corner_angle, angle, arc.turn),
                    }
            }
        }
    }

    /// The `FilletDoesNotFit` carrier payload, with the fit `margin`
    /// re-expressed in radians for a circular leg.
    fn carrier_diag(&self, margin: T) -> FilletLegCarrier
    where
        T: Bounds,
    {
        match self.arc {
            None => FilletLegCarrier::Line,
            Some(arc) => FilletLegCarrier::Arc {
                radius: arc.radius.lo(),
                angular_margin: (margin / arc.radius).lo(),
            },
        }
    }
}

/// One surviving branch of the offset-carrier construction.
#[derive(Clone, Copy, Debug)]
struct Candidate<T: Real> {
    /// The fillet arc's center.
    center: Point2<T>,
    /// The tangent point on the incoming leg.
    t1: Point2<T>,
    /// The tangent point on the outgoing leg.
    t2: Point2<T>,
}

// The two offset-carrier intersections live as inherent methods on the
// types they belong to, and NOT as free functions, for a discipline
// reason worth stating: their scalar parameter carries decision
// capability (`Decide + Bounds`) on top of the scalar bound, and the
// evaluation-code tripwire greps `crates/*/src` for a scalar type
// parameter widened in place, while clippy's `multiple_bound_locations`
// rejects splitting one function's bounds across the parameter list and
// a `where` clause. An impl block whose scalar bound is the plain one,
// with the decision capability declared on the method, satisfies both —
// and it is the same shape `LoopBuilder`'s own gated methods use.

impl<T: Real> Leg<T> {
    /// The candidate centers where THIS (straight) leg's offset line
    /// meets the other leg's offset circle `arc` (0, 1 or 2, in fixed
    /// order).
    fn offset_line_circle(
        &self,
        corner: Point2<T>,
        sgn: T,
        radius: T,
        arc: ArcCarrier<T>,
        band: Band,
    ) -> Result<Vec<Point2<T>>, ProfileError>
    where
        T: Decide + Bounds,
    {
        let normal = left_normal(self.dir);
        let on_offset = corner + normal * (sgn * radius);
        let rho = offset_radius(&arc, sgn, radius);
        let h = (arc.center - on_offset).dot(normal);
        let foot = arc.center - normal * h;
        Ok(
            match decide("fillet_offset_line_circle", rho.abs() - h.abs(), band)
                .map_err(fillet_escalated)?
            {
                // powi(2)-discipline squares: ρ and h both straddle zero
                // in general (memories/interval-square-poison.md).
                Sign::Positive => {
                    let half = (rho.powi(2) - h.powi(2)).sqrt();
                    vec![foot + self.dir * half, foot - self.dir * half]
                }
                // Decided tangent: the single candidate IS the foot — no
                // sqrt of a rounding-signed zero.
                Sign::Zero => vec![foot],
                Sign::Negative => {
                    return Err(ProfileError::NoCornerForFillet {
                        reason: NoCornerReason::OffsetCarriersDisjoint,
                        radius: radius.lo(),
                    });
                }
            },
        )
    }
}

impl<T: Real> ArcCarrier<T> {
    /// The candidate centers where this carrier's offset circle meets
    /// `other`'s (0, 1 or 2, in fixed order).
    fn offset_circles(
        self,
        other: ArcCarrier<T>,
        sgn: T,
        radius: T,
        band: Band,
    ) -> Result<Vec<Point2<T>>, ProfileError>
    where
        T: Decide + Bounds,
    {
        let rho1 = offset_radius(&self, sgn, radius);
        let rho2 = offset_radius(&other, sgn, radius);
        let r1 = rho1.abs();
        let r2 = rho2.abs();
        let link = other.center - self.center;
        let dist_squared = link.norm_squared();
        let dist = dist_squared.sqrt();
        let external = decide("fillet_offset_circles_external", r1 + r2 - dist, band)
            .map_err(fillet_escalated)?;
        let internal = decide(
            "fillet_offset_circles_internal",
            dist - (r1 - r2).abs(),
            band,
        )
        .map_err(fillet_escalated)?;
        if external == Sign::Negative || internal == Sign::Negative {
            return Err(ProfileError::NoCornerForFillet {
                reason: NoCornerReason::OffsetCarriersDisjoint,
                radius: radius.lo(),
            });
        }
        let along = (dist_squared + rho1.powi(2) - rho2.powi(2)) / (dist + dist);
        let base = self.center + link * (along / dist);
        if external == Sign::Zero || internal == Sign::Zero {
            return Ok(vec![base]);
        }
        let half = (rho1.powi(2) - along.powi(2)).sqrt();
        let offset = left_normal(link) * (half / dist);
        Ok(vec![base + offset, base - offset])
    }
}

/// The fillet arc's bulge tan(θ/4) from its tangent points and center.
///
/// With u = T₁ − P, w = T₂ − P, ψ = |θ|/2 ∈ (0, π): sin ψ = L/(2r) from
/// the chord L = |T₂ − T₁|, and cos ψ = ±|M − P|/r from the apothem at
/// the chord midpoint M, the sign being that of σ·(u × w) (positive iff
/// |θ| < π). Then tan(θ/4) = σ·sin ψ/(1 + cos ψ), written below without
/// dividing through by r. Correct for major arcs, no square root of a
/// cancelling difference, and no transcendental.
///
/// # The negative-apothem (major-arc) branch is defensive
///
/// `copysign` flips the apothem when σ·(u × w) < 0, i.e. when the fillet
/// sweeps MORE than half a turn. That branch is **not reachable through
/// [`LoopBuilder::fillet_corner`]** (review NOTE): the corner-side
/// extent gates put both tangent points on the corner side of their
/// legs, which bounds the fillet's turn below π — a 200k-corner search
/// over all four corner classes (line/arc × line/arc, radii 0.03–3,
/// enclosing cases included, 106k accepted) tops out at |bulge| =
/// 0.9804, i.e. θ = 0.987·π, approaching the bound from below and never
/// crossing it. The nearest approach is a near-cusp line×line corner.
///
/// The general form is kept, and the branch is covered by a direct unit
/// test (`fillet_bulge_major_arc_branch`) rather than an e2e row that
/// cannot exist, because this is the general tan(θ/4) identity and the
/// bound is a property of the *gates*, not of the formula: a future
/// caller that relaxes the extent rule (or a v2 lowering that anchors
/// sides differently) must not silently get a wrong bulge.
fn fillet_bulge<T: Real>(t1: Point2<T>, t2: Point2<T>, center: Point2<T>, radius: T, sgn: T) -> T {
    let two = T::from_f64(2.0);
    let cross = (t1 - center).perp_dot(t2 - center);
    let chord = t2 - t1;
    let half_chord = chord.norm_squared().sqrt() / two;
    let apothem = (t1.lerp(t2, T::from_f64(0.5)) - center)
        .norm_squared()
        .sqrt();
    sgn * half_chord / (radius + apothem.copysign(sgn * cross))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Tolerance;

    use super::*;
    use crate::{Profile, SketchPlane};

    fn p2(x: f64, y: f64) -> Point2<f64> {
        Point2::new(x, y)
    }

    #[test]
    fn bulge_from_via_quarter_circle() {
        // Unit-circle quarter arc (1,0) → (0,1) through the apex.
        let b = bulge_from_via(
            p2(1.0, 0.0),
            p2(
                core::f64::consts::FRAC_1_SQRT_2,
                core::f64::consts::FRAC_1_SQRT_2,
            ),
            p2(0.0, 1.0),
        );
        assert!((b - (core::f64::consts::FRAC_PI_8).tan()).abs() < 1e-15);
    }

    /// [`fillet_bulge`]'s major-arc branch (review NOTE): the
    /// `copysign`-negated apothem. Unreachable through `fillet_corner`
    /// (its docs carry the search that establishes the θ < π bound), so
    /// the general identity is pinned here directly, against `tan(θ/4)`
    /// for sweeps on both sides of π and in both traversal senses.
    #[test]
    fn fillet_bulge_major_arc_branch() {
        let radius = 1.5;
        let center = p2(-0.25, 0.75);
        let on = |deg: f64| {
            let (s, c) = f64::to_radians(deg).sin_cos();
            p2(center.x + radius * c, center.y + radius * s)
        };
        // θ swept from t1 to t2 in the `sgn` sense; the pairs straddle π
        // so both signs of σ·(u × w) are exercised.
        for &theta in &[60.0, 179.0, 181.0, 270.0, 359.0] {
            for &sgn in &[1.0, -1.0] {
                let start = 17.0;
                let t1 = on(start);
                let t2 = on(start + sgn * theta);
                let b = fillet_bulge(t1, t2, center, radius, sgn);
                let want = sgn * f64::tan(f64::to_radians(theta) / 4.0);
                // Relative: tan(θ/4) blows up as θ → 2π (229 at 359°).
                assert!(
                    (b - want).abs() <= 1e-11 * want.abs().max(1.0),
                    "theta {theta} sgn {sgn}: bulge {b} vs tan(theta/4) {want}"
                );
                // Above π the apothem must have been negated, which is
                // exactly when |bulge| exceeds tan(π/4) = 1.
                assert_eq!(
                    b.abs() > 1.0,
                    theta > 180.0,
                    "theta {theta}: |bulge| {} on the wrong side of 1",
                    b.abs()
                );
            }
        }
    }

    /// The S8 selection ladder, rung by rung (`nearest_candidate` docs):
    /// rungs 2–3 are unreachable through `fillet_corner` in exact
    /// arithmetic (strict dominance among survivors — see the fn docs),
    /// so the total fallback rule is pinned here at the unit level, in
    /// both enumeration orders where an order exists to swap.
    #[test]
    fn nearest_candidate_ladder() {
        // Rung 1: strictly smaller total setback wins, either side.
        assert_eq!(nearest_candidate(&[[2.0, 2.0], [1.0, 1.5]]), 1);
        assert_eq!(nearest_candidate(&[[1.0, 1.5], [2.0, 2.0]]), 0);
        // Rung 1 sees the sum, not the parts: a crossed pair with the
        // strictly smaller total wins even with the larger incoming.
        assert_eq!(nearest_candidate(&[[1.0, 4.0], [3.0, 1.0]]), 1);
        // Rung 2: exactly tied totals — the smaller INCOMING setback
        // wins, either side.
        assert_eq!(nearest_candidate(&[[2.0, 1.0], [1.0, 2.0]]), 1);
        assert_eq!(nearest_candidate(&[[1.0, 2.0], [2.0, 1.0]]), 0);
        // Rung 3: identical pairs — the first-classified candidate (the
        // sole designed non-equivariant residual).
        assert_eq!(nearest_candidate(&[[1.5, 2.5], [1.5, 2.5]]), 0);
        // The single-survivor path funnels through the same rule.
        assert_eq!(nearest_candidate(&[[3.0, 4.0]]), 0);
    }

    #[test]
    fn bulge_from_via_is_via_position_independent() {
        // The inscribed-angle theorem: any via on the arc gives the
        // same bulge. Points on the unit circle at 10° and 80°.
        let at = |deg: f64| {
            let (s, c) = deg.to_radians().sin_cos();
            p2(c, s)
        };
        let b1 = bulge_from_via(p2(1.0, 0.0), at(10.0), p2(0.0, 1.0));
        let b2 = bulge_from_via(p2(1.0, 0.0), at(80.0), p2(0.0, 1.0));
        assert!((b1 - b2).abs() < 1e-14);
    }

    #[test]
    fn bulge_from_via_semicircle_and_sign() {
        // Through the lower apex: a counterclockwise semicircle,
        // bulge +1.
        let b = bulge_from_via(p2(0.0, 0.0), p2(1.0, -1.0), p2(2.0, 0.0));
        assert!((b - 1.0).abs() < 1e-15);
        // Mirrored via: clockwise, bulge −1.
        let b = bulge_from_via(p2(0.0, 0.0), p2(1.0, 1.0), p2(2.0, 0.0));
        assert!((b + 1.0).abs() < 1e-15);
    }

    #[test]
    fn bulge_from_via_degenerate_inputs_are_total() {
        // Collinear between: a line.
        assert_eq!(
            bulge_from_via(p2(0.0, 0.0), p2(1.0, 0.0), p2(2.0, 0.0)),
            0.0
        );
        // Collinear outside: tan(±π/2) — huge, for validation to
        // reject; never a panic.
        let b = bulge_from_via(p2(0.0, 0.0), p2(3.0, 0.0), p2(2.0, 0.0));
        assert!(b.abs() > 1e12);
    }

    #[test]
    fn bulge_from_center_quarter_arcs_both_ways() {
        let b = bulge_from_center(p2(1.0, 0.0), p2(0.0, 1.0), p2(0.0, 0.0), ArcSweep::Ccw);
        assert!((b - core::f64::consts::FRAC_PI_8.tan()).abs() < 1e-15);
        // Clockwise from (1,0) to (0,1) is the long way round:
        // θ = −3π/2, bulge = tan(−3π/8).
        let b = bulge_from_center(p2(1.0, 0.0), p2(0.0, 1.0), p2(0.0, 0.0), ArcSweep::Cw);
        assert!((b - (-3.0 * core::f64::consts::FRAC_PI_8).tan()).abs() < 1e-12);
    }

    #[test]
    fn builder_builds_the_two_arc_circle_by_all_three_forms() {
        let tol = Tolerance::with_eps(1e-9);
        let raw = ProfileLoop::builder(p2(-1.0, 0.0))
            .arc_to(p2(1.0, 0.0), 1.0)
            .close_with_bulge(1.0);
        let via = ProfileLoop::builder(p2(-1.0, 0.0))
            .arc_to_via(p2(0.0, -1.0), p2(1.0, 0.0))
            .close_arc_via(p2(0.0, 1.0));
        let center = ProfileLoop::builder(p2(-1.0, 0.0))
            .arc_to_center(p2(1.0, 0.0), p2(0.0, 0.0), ArcSweep::Ccw)
            .close_arc_center(p2(0.0, 0.0), ArcSweep::Ccw);
        for lp in [raw, via, center] {
            assert_eq!(lp.vertices.len(), 2);
            for v in &lp.vertices {
                assert!((v.bulge - 1.0).abs() < 1e-12, "bulge {}", v.bulge);
            }
            let vp = Profile::new(SketchPlane::xy(), vec![lp])
                .validate(tol)
                .expect("the built circle must validate");
            match vp.loops()[0].segments()[0].kind {
                crate::SegmentKind::Arc { center, radius, .. } => {
                    assert!(center.x.abs() < 1e-12 && center.y.abs() < 1e-12);
                    assert!((radius - 1.0).abs() < 1e-12);
                }
                crate::SegmentKind::Line => panic!("must classify as an arc"),
            }
        }
    }

    #[test]
    fn builder_line_and_arc_mix() {
        // A stadium: two straight sides, two semicircular caps. Every
        // cap joint is an exact line/arc tangency — declared (the #101
        // discipline; undeclared it is refused, see the validate
        // suites).
        let tol = Tolerance::with_eps(1e-9);
        let lp = ProfileLoop::builder(p2(0.0, 0.0))
            .declare_tangent() // left cap → bottom side (closing joint)
            .line_to(p2(2.0, 0.0))
            .declare_tangent() // bottom side → right cap
            .arc_to_center(p2(2.0, 1.0), p2(2.0, 0.5), ArcSweep::Ccw)
            .declare_tangent() // right cap → top side
            .line_to(p2(0.0, 1.0))
            .declare_tangent() // top side → left cap
            .close_arc_center(p2(0.0, 0.5), ArcSweep::Ccw);
        let vp = Profile::new(SketchPlane::xy(), vec![lp])
            .validate(tol)
            .expect("the stadium must validate");
        let kinds: Vec<bool> = vp.loops()[0]
            .segments()
            .iter()
            .map(|s| matches!(s.kind, crate::SegmentKind::Line))
            .collect();
        // Canonical start is (0, 0); chain: line, arc, line, arc.
        assert_eq!(kinds, vec![true, false, true, false]);
    }
}
