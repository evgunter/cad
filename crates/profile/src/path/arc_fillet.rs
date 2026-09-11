//! The PATHS algebra's **arc-carrier fillet boundary** (LIB-G2 §3b):
//! the one place the algebra derives a fillet corner from two carriers
//! at least one of which is circular, and the one place the lifted S8
//! selection ladder is called.
//!
//! # Why this file exists, and why it is allowlisted
//!
//! The builder door authors its corner; the algebra forbids that (a
//! corner exists only as the carrier intersection — PATHS-DESIGN §2),
//! so it must DERIVE the corner, and a ray×circle or circle×circle
//! carrier pair admits 0, 1 or 2 of them. Each derived corner is then
//! the ratified S2 construction's input, and each yields its own
//! surviving candidates, so the choice is over (corner, candidate)
//! PAIRS. Ranking pairs is [`crate::fillet_select::nearest_joint`],
//! which
//! reads the f64 diagnostic channel — a `Bounds` read.
//!
//! This module therefore takes a compound `Decide + Bounds`: it
//! DECIDES (the carrier-meet and angular advance/reach gates) and reads
//! the selection channel, in that order. The justification is
//! [`crate::fillet_select`]'s, which is where the S8 rule has its one
//! home — restated here only because this file's allowlist line needs
//! a purpose-matched sentence of its own; the rule itself is the same
//! rule on the same channel: **a representation-level choice between
//! already-classified constructions, never a re-decision of geometry**
//! (M5 S8; the ruling's "plain deterministic selection rule, not a Q1
//! predicate" — no funnel entry, no escalation arm, no error). The
//! compound bound is confined here so that `path.rs` itself stays
//! bracket-free; ratified as LIB-G2's LB3 with the fence amended for
//! exactly this one allowlist line.
//!
//! # The squared-radius rule (LB4, a design rule and not an optimizer)
//!
//! Every derived corner is computed from **squared** radii
//! `R² = |anchor − centre|²` and never round-trips `√(R²)²`. That is not
//! a micro-optimization: on the rocker eye's circle×circle corner the
//! radius form lands the corner **one ulp low**
//! (`0.8660254037844385` against the authored `…86`), and the squared
//! form lands it bitwise, which is what lets a derived corner reproduce
//! an authored one exactly. The `sqrt`s that DO appear here feed
//! classification margins only (metre gates), never an emitted
//! coordinate.
//!
//! # Order of operations
//!
//! 1. derive the corners (0, 1 or 2), squared-radius forms;
//! 2. gate each corner: **advance** on the incoming side (ahead of its
//!    anchor) and **reach** on the arrival side (behind its anchor) —
//!    the linear `path_corner_advance` gates generalized to the angular
//!    `path_corner_advance_arc` / `path_corner_reach_arc`;
//! 3. run the ratified [`crate::sugar::arc_fillet_trims`] at every
//!    surviving corner,
//!    with exactly the arguments the builder door would have passed had
//!    the author written that corner by hand;
//! 4. flatten the survivors in corner-then-candidate order and pick
//!    with the lifted ladder.
//!
//! Step 3's argument identity is the whole bit-identity contract: a
//! migrated site feeds the SAME closed form the SAME numbers, so the
//! only thing that can move an ulp is step 1 — hence the squared-radius
//! rule.

use geom_core::k_stats::decide;
use geom_core::{Band, Bounds, Decide, Indeterminate, Margin, Point2, Real, Sign, Tol, Vec2};

use super::{
    ArcData, CornerReason, CornerRefusal, CornerWindow, Dir, PathError, PathNoCornerReason,
    linear_band,
};
use crate::fillet_select::nearest_joint;
use crate::structure::{
    CornerGate, Decision, DecisionValue, FilletDecision, Guide, StructureRefusal,
};
use crate::sugar::{
    ArcFilletCandidate, ArcFilletOutcome, ArcSweep, ArcTrimRefusal, FilletLegShape,
    arc_fillet_trims, signed_swept,
};
use crate::validate::FilletLegCarrier;

/// A fillet side's carrier, as the algebra binds it: a straight ray, or
/// a circle about a centre with a structural winding.
///
/// The circular spelling is what a `Center`-mode side binds; the radius is
/// never authored — it is `|anchor − centre|`, carried squared.
#[derive(Clone, Copy, Debug)]
pub(crate) enum SideCarrier<T: Real> {
    /// The line through the side's anchor along this **unit** ray.
    Ray(Vec2<T>),
    /// The circle through the side's anchor about `centre`, swept
    /// `winding` (structural, as in the `Center` mode).
    Circle {
        /// The carrier circle's centre.
        centre: Point2<T>,
        /// Which way the path travels around it.
        winding: ArcSweep,
    },
}

/// One side of a derived fillet corner: the real on-path point that
/// anchors it (PATHS-DESIGN §2's "every side is anchored") plus the
/// carrier that anchor sits on.
#[derive(Clone, Copy, Debug)]
pub(crate) struct FilletSide<T: Real> {
    /// The side's anchoring on-path point.
    pub anchor: Point2<T>,
    /// Its carrier.
    pub carrier: SideCarrier<T>,
}

/// The resolved arc-carrier fillet: what the door emits, named rather
/// than returned as a wide tuple.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ArcFilletTrims<T: Real> {
    /// The tangent point on the incoming side.
    pub t1: Point2<T>,
    /// The tangent point on the arrival side.
    pub t2: Point2<T>,
    /// The fillet arc's bulge.
    pub bulge: T,
    /// The incoming side's `fillet_leg_fit` classification.
    pub fit_in: Sign,
    /// The arrival side's fit classification.
    pub fit_out: Sign,
    /// The incoming side's carrier when it is circular — what the door
    /// re-emits the trimmed piece along (`(centre, winding)`).
    pub in_arc: Option<(Point2<T>, ArcSweep)>,
    /// The fillet arc's own carrier, for the §4 item 4 identity checks.
    pub arc: ArcData<T>,
}

impl<T: Real> SideCarrier<T> {
    /// The leg shape the ratified S2 construction takes for this
    /// carrier (the corner is supplied separately, as the builder door
    /// supplies its authored one).
    fn leg_shape(self) -> FilletLegShape<T> {
        match self {
            Self::Ray(_) => FilletLegShape::Line,
            Self::Circle { centre, winding } => FilletLegShape::Arc {
                center: centre,
                sweep: winding,
            },
        }
    }
}

impl<T: Real> FilletSide<T> {
    /// The signed distance travelled along THIS side's carrier from
    /// `from` to `to`, in metres (an arc length `R·Δθ` on a circular
    /// carrier), together with the funnel predicate name that
    /// classifies it: the shipped linear `path_corner_advance` on a
    /// straight carrier, `arc_name` on a circular one.
    ///
    /// One helper serves both gates because "ahead of the anchor" and
    /// "behind the anchor" are the same signed quantity read in the two
    /// argument orders — which is exactly why [`signed_swept`], and not
    /// the forward-only `swept`, is the angular spelling: past-the-end
    /// must come out NEGATIVE rather than wrapping to nearly 2π.
    ///
    /// So a STRAIGHT side reports `path_corner_advance` for both
    /// windows while [`super::CornerWindow`] distinguishes them, and
    /// that is right: the predicate names the MARGIN being classified,
    /// which is one margin computed one way, whereas the window names
    /// which side's anchor the corner fell outside. The two circular
    /// spellings differ only because the arc gates were new margins,
    /// not because the windows are different questions.
    fn travel(&self, from: Point2<T>, to: Point2<T>, arc_name: &'static str) -> (&'static str, T) {
        match self.carrier {
            SideCarrier::Ray(u) => ("path_corner_advance", (to - from).dot(u)),
            SideCarrier::Circle { centre, winding } => {
                let turn = match winding {
                    ArcSweep::Ccw => T::one(),
                    ArcSweep::Cw => -T::one(),
                };
                let r = (self.anchor - centre).norm_squared().sqrt();
                (
                    arc_name,
                    r * signed_swept(about(from, centre), about(to, centre), turn),
                )
            }
        }
    }
}

/// A carrier pair that admits no corner, named by which way it failed.
///
/// Only the PAIR-level conditions reach this: a corner the gates or the
/// construction refuse is a fact about that corner and rides the
/// envelope instead, with its point.
fn no_corner<T: Real>(reason: PathNoCornerReason, radius: T) -> PathError<T> {
    PathError::NoCornerForFillet { reason, radius }
}

/// An anchor-window gate's refusal: which window the corner falls
/// outside, or a margin the band could not classify.
enum GateRefusal {
    /// The corner is outside this window — a fact about the corner, so
    /// it becomes an envelope entry beside the corner point.
    Outside(CornerWindow),
    /// The margin escalated; nothing was decided about this corner.
    Escalated(Indeterminate),
}

/// A guided pass's refusal on a consumed decision, in the elaboration's
/// own error vocabulary.
fn structure<T: Real>(refusal: StructureRefusal) -> PathError<T> {
    PathError::Structure(refusal)
}

/// The corners of a **ray × circle** pair, in increasing ray parameter.
///
/// Squared-radius form: the signed perpendicular offset of the centre
/// from the ray is exact in metres (`u` is unit), so the discriminant
/// is `R² − offset²` with `powi(2)` — no `√(R²)²` anywhere on the path
/// to a coordinate. The metre gate `R − |offset|` is a classification
/// margin and is allowed its `sqrt`.
fn ray_circle<T: Decide>(
    origin: Point2<T>,
    u: Vec2<T>,
    centre: Point2<T>,
    r_sq: T,
    radius: T,
    band: Band,
) -> Result<[Point2<T>; 2], PathError<T>> {
    let w = centre - origin;
    let along = w.dot(u);
    let offset = w.perp_dot(u);
    let margin = r_sq.sqrt() - offset.abs();
    match decide("path_carrier_meet", Margin::of(margin), band) {
        Ok(Sign::Positive) => {}
        Ok(Sign::Zero) => {
            return Err(no_corner(PathNoCornerReason::CarriersParallel, radius));
        }
        Ok(Sign::Negative) => {
            return Err(no_corner(PathNoCornerReason::CarriersDoNotMeet, radius));
        }
        Err(source) => return Err(PathError::Escalated { source }),
    }
    let h = (r_sq - offset.powi(2)).sqrt();
    Ok([origin + u * (along - h), origin + u * (along + h)])
}

/// The corners of a **circle × circle** pair, in fixed order (the
/// `+n` root first, `n` the left normal of `centre₁ → centre₂`).
///
/// THE squared-radius form (LB4): `k`, the midpoint and `h²` are built
/// from `d²`, `R₁²` and `R₂²` alone, so the only root taken on the way
/// to a coordinate is the final `√h²`. Three metre gates classify the
/// pair first — concentric, too far apart, one inside the other — and
/// each may legitimately spend a `sqrt`, being margins and not
/// coordinates. A hairline-tangent pair that passes the gate and still
/// rounds `h²` below zero yields NaN coordinates: poison, which every
/// downstream `decide` refuses loudly (D4 ¶2), never a silent repair.
fn circle_circle<T: Decide>(
    c1: Point2<T>,
    r1_sq: T,
    c2: Point2<T>,
    r2_sq: T,
    radius: T,
    band: Band,
) -> Result<[Point2<T>; 2], PathError<T>> {
    let d = c2 - c1;
    let d_sq = d.norm_squared();
    let d_len = d_sq.sqrt();
    let (r1, r2) = (r1_sq.sqrt(), r2_sq.sqrt());
    // Concentric carriers name no corner (identical ones name every
    // point of the circle, which is worse, not better).
    let gone = PathNoCornerReason::CarriersDoNotMeet;
    let tangent = PathNoCornerReason::CarriersParallel;
    for (margin, on_zero) in [
        // concentric: no transversal crossing at any radius pair
        (d_len, gone),
        // externally tangent / disjoint
        (r1 + r2 - d_len, tangent),
        // internally tangent / one inside the other
        (d_len - (r1 - r2).abs(), tangent),
    ] {
        match decide("path_carrier_meet", Margin::of(margin), band) {
            Ok(Sign::Positive) => {}
            Ok(Sign::Zero) => return Err(no_corner(on_zero, radius)),
            Ok(Sign::Negative) => return Err(no_corner(gone, radius)),
            Err(source) => return Err(PathError::Escalated { source }),
        }
    }
    let k = (d_sq + r1_sq - r2_sq) / (T::from_f64(2.0) * d_sq);
    let mid = c1 + d * k;
    let h = (r1_sq / d_sq - k.powi(2)).sqrt();
    let n = Vec2::new(-d.y, d.x);
    Ok([mid + n * h, mid - n * h])
}

/// The angular coordinate of `p` on the carrier about `centre`.
fn about<T: Real>(p: Point2<T>, centre: Point2<T>) -> T {
    let v = p - centre;
    v.y.atan2(v.x)
}

/// The **advance** gate: the corner must lie strictly ahead of the
/// incoming side's anchor along its carrier.
///
/// On a straight carrier this is the shipped linear rule (the ray
/// parameter in metres, `path_corner_advance`). On a circular one it is
/// the same statement in the carrier's own currency: the SIGNED swept
/// angle from the anchor to the corner, levered to metres by the
/// carrier radius — `path_corner_advance_arc`, a new funnel predicate
/// because it is a new margin, not a rescaling of the old one
/// ([`signed_swept`] is what makes "past the anchor" classify Negative
/// rather than wrapping to nearly 2π).
fn advance_gate<T: Decide>(
    side: &FilletSide<T>,
    corner: Point2<T>,
    band: Band,
) -> Result<(), GateRefusal> {
    let (name, margin) = side.travel(side.anchor, corner, "path_corner_advance_arc");
    match decide(name, Margin::of(margin), band) {
        Ok(Sign::Positive) => Ok(()),
        Ok(_) => Err(GateRefusal::Outside(CornerWindow::BehindIncomingRay)),
        Err(source) => Err(GateRefusal::Escalated(source)),
    }
}

/// The **reach** gate: the corner must lie strictly behind the arrival
/// side's anchor along that side's carrier — the arrival really did
/// come from the corner.
///
/// Straight: the shipped `path_corner_advance` on the negated ray
/// parameter. Circular: `path_corner_reach_arc` on the signed swept
/// angle from the corner FORWARD to the anchor, levered to metres.
fn reach_gate<T: Decide>(
    side: &FilletSide<T>,
    corner: Point2<T>,
    band: Band,
) -> Result<(), GateRefusal> {
    let (name, margin) = side.travel(corner, side.anchor, "path_corner_reach_arc");
    match decide(name, Margin::of(margin), band) {
        Ok(Sign::Positive) => Ok(()),
        Ok(_) => Err(GateRefusal::Outside(CornerWindow::BehindArrivalAnchor)),
        Err(source) => Err(GateRefusal::Escalated(source)),
    }
}

/// The corners the two carriers admit, in each pair's fixed
/// enumeration order (never inferred from values — the pair's KIND
/// decides which closed form runs).
fn derive<T: Decide>(
    inc: &FilletSide<T>,
    arr: &FilletSide<T>,
    radius: T,
    band: Band,
) -> Result<Vec<Point2<T>>, PathError<T>> {
    match (inc.carrier, arr.carrier) {
        (SideCarrier::Ray(u), SideCarrier::Circle { centre, .. }) => ray_circle(
            inc.anchor,
            u,
            centre,
            (arr.anchor - centre).norm_squared(),
            radius,
            band,
        )
        .map(Vec::from),
        (SideCarrier::Circle { centre, .. }, SideCarrier::Ray(u)) => ray_circle(
            arr.anchor,
            u,
            centre,
            (inc.anchor - centre).norm_squared(),
            radius,
            band,
        )
        .map(Vec::from),
        (SideCarrier::Circle { centre: c1, .. }, SideCarrier::Circle { centre: c2, .. }) => {
            circle_circle(
                c1,
                (inc.anchor - c1).norm_squared(),
                c2,
                (arr.anchor - c2).norm_squared(),
                radius,
                band,
            )
            .map(Vec::from)
        }
        // Two straight carriers never reach this boundary: `path.rs`
        // owns that case through the ratified `line_line_fillet_trims`,
        // bracket-free. A backstop, not a path.
        (SideCarrier::Ray(_), SideCarrier::Ray(_)) => Err(PathError::UnderdeterminedLeg {
            site: "arc-carrier fillet boundary reached with two straight carriers",
        }),
    }
}

/// What the ratified construction's refusal at one derived corner
/// turns out to be ABOUT — which decides where it goes.
///
/// The construction runs per corner, but not everything it can refuse
/// is a fact about that corner: the carriers being tangent where they
/// cross, a leg with no length scale, a band that cannot classify and
/// the M8 lever gate are all facts about the pair or about the run.
/// Those become the resolve's whole-pair refusal, which outranks the
/// envelope. What is left is a statement about this corner and becomes
/// an entry beside its point.
enum CornerOutcome<T: Real> {
    /// A statement about this corner: it becomes an envelope entry,
    /// beside the corner point.
    Reason(CornerReason<T>),
    /// A refusal that names no corner — the pair is parallel at the
    /// derived corner, a leg has no length scale, the band failed, or
    /// the conditioning gate aborted. First one wins, and it OUTRANKS
    /// the envelope: a fact about the pair answers before any fact
    /// about one of its corners does.
    Whole(PathError<T>),
}

/// An [`ArcTrimRefusal`] in the algebra's error vocabulary. The door
/// owns the bracket reads, exactly as `fillet_corner` does: the leg
/// diagnostics are `f64` enclosure lower bounds, for messages and never
/// for re-deciding.
///
/// Three reads on three lines, none of them a re-decision: `arm.lo()`
/// is a value-channel BRANCH between two message sites, `r.lo()` is an
/// `f64` payload field, and `(margin / r).lo()` brackets a quotient
/// computed at `T` into a second one. Nothing read here re-enters the
/// computation, so the sole `T: Bounds` is the whole obligation: this
/// door decides nothing, which is why it does not carry the module's
/// `Decide` half. At a dual scalar the three are the value channel's
/// bit for bit (D9), and a degraded tangent cannot reach them.
fn map_refusal<T: Bounds>(refusal: ArcTrimRefusal<T>, radius: T) -> CornerOutcome<T> {
    match refusal {
        ArcTrimRefusal::Band(source) => CornerOutcome::Whole(PathError::Band(source)),
        ArcTrimRefusal::Escalated(source) => CornerOutcome::Whole(PathError::Escalated { source }),
        ArcTrimRefusal::LegDegenerate { arm, .. } => {
            CornerOutcome::Whole(PathError::UnderdeterminedLeg {
                site: if arm.lo() <= 0.0 {
                    "arc-carrier fillet leg with no length scale"
                } else {
                    "arc-carrier fillet leg arm indeterminate"
                },
            })
        }
        // Carriers meeting tangentially at the derived corner: the same
        // situation `path_corner_turn` names on the straight pair, and
        // a statement about the PAIR rather than about one crossing.
        ArcTrimRefusal::AlreadyTangent { .. } => {
            CornerOutcome::Whole(no_corner(PathNoCornerReason::CarriersParallel, radius))
        }
        ArcTrimRefusal::NoCorner { reason, .. } => {
            CornerOutcome::Reason(CornerReason::NoTangentCircle(reason))
        }
        // M8's conditioning gate. Deliberately NOT laundered into a
        // "no corner" reason: a corner and a tangent circle both exist
        // here, and saying "no corner" about a corner that is right
        // there would send the author looking for the wrong thing. The
        // lever the message names is the one they can move, and this
        // gate ABORTS the resolve (see `resolve`), so it is never an
        // envelope entry.
        ArcTrimRefusal::OffsetLeverTooShort {
            leg,
            carrier_radius,
            offset_radius,
            least_lever,
            margin,
        } => CornerOutcome::Whole(PathError::FilletOffsetLeverTooShort {
            side: leg,
            carrier_radius,
            offset_radius,
            least_lever,
            margin,
        }),
        // The enclosing class, refused in its own words. Like the
        // conditioning gate above it is deliberately NOT laundered into
        // a "no corner" reason: the corner exists and the author can see
        // it — what does not exist, at this radius and permanently, is a
        // fillet OF it (`crates/profile/README.md`). Unlike that
        // gate this one does NOT abort the resolve: rho's sign is a fact
        // about THIS corner's turn side, and the pair's other crossing
        // turns the other way, where the same radius is an ordinary
        // tangency the author is entitled to. So it is an entry like any
        // other and surfaces exactly when no corner of the pair could be
        // served.
        ArcTrimRefusal::EnclosesLegCarrier {
            leg,
            carrier_radius,
            offset_radius,
            largest_tangent_radius,
            ..
        } => CornerOutcome::Reason(CornerReason::EnclosesLegCarrier {
            side: leg,
            carrier_radius,
            offset_radius,
            largest_tangent_radius,
        }),
        // §3c: the anchor-fit refusal carries the CARRIER KIND, so
        // an arc side gets its angular story (`FilletLegCarrier::Arc`'s
        // `angular_margin`) instead of a bare linear setback that means
        // nothing on a circle.
        ArcTrimRefusal::DoesNotFit {
            leg,
            carrier_radius,
            margin,
            setback,
            leg_length,
        } => CornerOutcome::Reason(CornerReason::AnchorOutsideTrimmedExtent {
            side: leg,
            carrier: match carrier_radius {
                None => FilletLegCarrier::Line,
                Some(r) => FilletLegCarrier::Arc {
                    radius: r.lo(),
                    angular_margin: (margin / r).lo(),
                },
            },
            setback,
            available: leg_length,
        }),
    }
}

/// The presentation key: how far a derived corner sits from the two
/// bracketing anchors, summed, as an `f64` enclosure lower bound.
///
/// A sort key and nothing else. Ties break on enumeration order because
/// the sort is stable, so the ORDER is a function of the inputs (D9)
/// even though the key is read off the diagnostic channel; and the
/// entries the sort permutes carry the same payloads whatever order
/// they land in, so nothing downstream can branch on it.
fn anchor_span<T: Bounds>(
    corner: Point2<T>,
    incoming: &FilletSide<T>,
    arrival: &FilletSide<T>,
) -> f64 {
    let reach = |anchor: Point2<T>| (corner - anchor).norm_squared().sqrt().lo();
    reach(incoming.anchor) + reach(arrival.anchor)
}

/// Resolves an arc-carrier fillet end to end: derive the corners, gate
/// them, run the ratified construction at each survivor, and pick with
/// the lifted ladder (the module docs' four steps).
///
/// `incoming` is the departure side (its anchor is the ray origin / the
/// point the carrier was bound at), `arrival` the side the fillet lands
/// on. At least one of the two carries a circle; a straight pair is
/// `path.rs`'s own business and never arrives here.
///
/// # Errors
///
/// [`PathError`], in the order the answers outrank one another. A
/// refusal that names NO corner comes first — a pair with no crossing
/// or a tangency at one ([`PathError::NoCornerForFillet`]), a leg with
/// no length scale, the M8 lever gate, a band failure — because it is
/// a fact about the pair. Otherwise
/// [`PathError::NoCornerOfPair`]: the corners that refused at the
/// answering stage, each with its own reason and point, the
/// construction's stage answering when any corner reached it and the
/// anchor windows' when none did. An escalation ANYWHERE aborts
/// immediately: a joint space one of whose members cannot be
/// classified cannot be honestly ranked.
pub(crate) fn resolve<T: Decide + Bounds>(
    guide: &mut Guide<T>,
    incoming: FilletSide<T>,
    arrival: FilletSide<T>,
    radius: T,
    tol: Tol,
) -> Result<ArcFilletTrims<T>, PathError<T>> {
    let consumed = guide.consume().map_err(structure)?;
    let band = linear_band(tol)?;
    // Two ENTRY channels and one whole-pair slot; each entry channel is
    // a LIST rather than a first-one-wins pick.
    //
    // A corner the GATES discard is the weaker story — the author's
    // anchors simply do not bracket it, and the other root is usually
    // the one they meant. A corner that PASSED the gates and then
    // failed to admit a tangent circle is the real answer, so the
    // construction's list answers when it is non-empty. The two lists
    // are never merged, because the spec's acceptance row asks for a
    // ONE-entry envelope where only one crossing sits in the windows:
    // the unbracketed corner's "you did not bracket me" beside the
    // answer is noise, not attribution. Within the answering channel
    // nothing is picked: every corner that refused there is an entry,
    // with its own reason and its own point.
    let mut build_entries: Vec<CornerRefusal<T>> = Vec::new();
    let mut gate_entries: Vec<CornerRefusal<T>> = Vec::new();
    // The refusals that name NO corner. First one wins, and one
    // OUTRANKS both entry lists — see the terminal choice below.
    let mut whole_refused: Option<PathError<T>> = None;
    // (1/2) derive, then gate — advance on the incoming side, reach on
    // the arrival side. A rejected corner is remembered, not returned:
    // the OTHER root may be the author's corner.
    //
    // Under guidance the gates still RUN — re-verifying them at this
    // scalar is the whole point — but their answers are compared, never
    // adopted: the corner list the joint space is built from is the
    // recorded one either way, so a lane that disagrees reports the
    // disagreement instead of quietly elaborating a different corner
    // set. An escalation names the corner it could not classify rather
    // than surfacing as a bare band refusal.
    let mut kept: Vec<Point2<T>> = Vec::new();
    let mut gates: Vec<CornerGate> = Vec::new();
    // Deriving the corners is where the joint space comes from, so a
    // scalar that cannot classify the carrier meet has no survivor list
    // for the recorded index to address. Under guidance that is named
    // as the joint space going unconfirmed rather than surfacing as a
    // bare band refusal about `path_carrier_meet`.
    let corners = derive(&incoming, &arrival, radius, band).map_err(|e| match (&consumed, &e) {
        (Some((fillet, _)), PathError::Escalated { source }) => structure(
            StructureRefusal::indeterminate(Decision::JointSpace { fillet: *fillet }, *source),
        ),
        _ => e,
    })?;
    for (ci, corner) in corners.into_iter().enumerate() {
        let outcome = match advance_gate(&incoming, corner, band) {
            Ok(()) => match reach_gate(&arrival, corner, band) {
                Ok(()) => Ok(()),
                Err(e) => Err((CornerGate::RefusedReach, e)),
            },
            Err(e) => Err((CornerGate::RefusedAdvance, e)),
        };
        let found = match &outcome {
            Ok(()) => CornerGate::Admitted,
            Err((g, _)) => *g,
        };
        let admit = match &consumed {
            // Unguided: this pass's own answer IS the decision.
            None => found == CornerGate::Admitted,
            Some((fillet, decision)) => {
                let site = Decision::CornerGate {
                    fillet: *fillet,
                    corner: ci,
                };
                // A record with no entry for this corner is not about
                // this program: the carrier pair enumerated a different
                // number of corners than the elaboration did.
                let Some(&recorded) = decision.corners.get(ci) else {
                    return Err(structure(StructureRefusal::flipped(
                        site,
                        DecisionValue::Count(decision.corners.len()),
                        DecisionValue::Count(ci + 1),
                    )));
                };
                match &outcome {
                    // The escalation NAMES the gate it stopped at
                    // rather than surfacing as a bare band refusal: a
                    // driver bisecting the parameter box needs to know
                    // which decision went unconfirmed.
                    Err((_, GateRefusal::Escalated(source))) => {
                        return Err(structure(StructureRefusal::indeterminate(site, *source)));
                    }
                    _ if found != recorded => {
                        return Err(structure(StructureRefusal::flipped(
                            site,
                            DecisionValue::Gate(recorded),
                            DecisionValue::Gate(found),
                        )));
                    }
                    _ => {}
                }
                recorded == CornerGate::Admitted
            }
        };
        gates.push(found);
        match (admit, outcome) {
            (true, _) => kept.push(corner),
            // An escalation is fatal here exactly as before: a joint
            // space one of whose members cannot be classified cannot be
            // honestly ranked. (A guided pass never reaches this arm —
            // it has already refused above, naming the corner.)
            (false, Err((_, GateRefusal::Escalated(source)))) => {
                return Err(PathError::Escalated { source });
            }
            (false, Err((_, GateRefusal::Outside(window)))) => gate_entries.push(CornerRefusal {
                at: corner,
                reason: CornerReason::OutsideAnchors(window),
            }),
            (false, Ok(())) => {}
        }
    }
    // (3) the ratified construction at each surviving corner, fed
    // exactly the arguments the builder door would have taken.
    let (in_shape, out_shape) = (incoming.carrier.leg_shape(), arrival.carrier.leg_shape());
    let mut joints: Vec<ArcFilletCandidate<T>> = Vec::new();
    let mut legs_of: Vec<Option<(Point2<T>, ArcSweep)>> = Vec::new();
    for corner in kept {
        let outcome = arc_fillet_trims(
            in_shape,
            incoming.anchor,
            corner,
            out_shape,
            arrival.anchor,
            radius,
            tol,
        );
        match outcome {
            Ok(ArcFilletOutcome::Arc { legs, survivors }) => {
                // `legs` is `[incoming, outgoing]`; the door re-emits the
                // trimmed incoming piece along its own carrier.
                let in_arc = legs[0].arc.map(|a| (a.center, a.sweep));
                for c in survivors {
                    joints.push(c);
                    legs_of.push(in_arc);
                }
            }
            Ok(ArcFilletOutcome::LineLine) => {
                return Err(PathError::UnderdeterminedLeg {
                    site: "arc-carrier fillet resolved to a straight pair",
                });
            }
            Err(ArcTrimRefusal::Escalated(source)) => {
                // The fit signs are produced INSIDE this construction,
                // so a guided pass that cannot get through it has not
                // reached a sign to compare — it names the resolution
                // whose construction went unconfirmed, and the payload
                // carries the predicate that could not be classified.
                return Err(match &consumed {
                    Some((fillet, _)) => structure(StructureRefusal::indeterminate(
                        Decision::FilletConstruction { fillet: *fillet },
                        source,
                    )),
                    None => PathError::Escalated { source },
                });
            }
            // The M8 conditioning gate ABORTS the resolve exactly as an
            // escalation does, and for the same reason: a joint space
            // one of whose members the band cannot CERTIFY (a tangent
            // point over an unsupported lever) cannot be honestly
            // ranked. Falling through to another corner's build would
            // let the twin corner of a near-tangent carrier pair mask
            // the refusal — the silent-build class the gate exists to
            // keep refused at every band.
            Err(refusal @ ArcTrimRefusal::OffsetLeverTooShort { .. }) => {
                return Err(match map_refusal(refusal, radius) {
                    CornerOutcome::Whole(e) => e,
                    // `map_refusal` sends the conditioning gate down the
                    // whole-pair arm; nothing else can arrive here.
                    CornerOutcome::Reason(_) => unreachable!(
                        "the offset-lever gate is a whole-pair refusal, never an entry"
                    ),
                });
            }
            Err(refusal) => match map_refusal(refusal, radius) {
                CornerOutcome::Reason(reason) => {
                    build_entries.push(CornerRefusal { at: corner, reason });
                }
                CornerOutcome::Whole(e) => {
                    if whole_refused.is_none() {
                        whole_refused = Some(e);
                    }
                }
            },
        }
    }
    // (4) the lifted ladder over the flattened joint space.
    if joints.is_empty() {
        // A whole-pair refusal OUTRANKS the envelope, as it did before
        // the envelope existed. It is a fact about the pair — these
        // carriers are tangent at a derived corner, a leg has no length
        // scale, the band could not classify — so a per-corner sentence
        // beside it, or instead of it, would be a smaller and weaker
        // claim about a situation the whole pair is in. Nothing is
        // discarded silently: the refusal reported is the strongest
        // true statement available, and the entries it outranks are
        // statements about corners of a pair that has already been
        // refused as a pair.
        if let Some(whole) = whole_refused {
            return Err(whole);
        }
        let mut entries = if build_entries.is_empty() {
            gate_entries
        } else {
            build_entries
        };
        if !entries.is_empty() {
            // Presentation order: the sum of the distances from the
            // corner to the two bracketing anchors, ascending, ties in
            // enumeration order (the sort is stable). A bracket read of
            // a quantity nothing decides on — the entries and their
            // payloads are the same set whatever order they are in, and
            // no caller branches on the order — so this is the module's
            // ratified diagnostic channel and not a re-decision.
            entries.sort_by(|a, b| {
                anchor_span(a.at, &incoming, &arrival)
                    .total_cmp(&anchor_span(b.at, &incoming, &arrival))
            });
            return Err(PathError::NoCornerOfPair {
                radius,
                corners: entries,
            });
        }
        return Err(no_corner(PathNoCornerReason::CarriersDoNotMeet, radius));
    }
    //
    // A guided pass does not run the ladder AT ALL. That is not
    // caution about agreement, it is the ladder's own contract: within
    // a hairline lens the survivors' setback gap can sit inside the
    // diagnostic channel's enclosure width, and two lanes may then
    // legally rank them differently — both picks being valid fillets of
    // the authored legs. Re-running the rule at a second scalar is
    // therefore a second CHOICE, not a check, so the guided pass takes
    // the recorded index and verifies only that the joint space it
    // indexes into still has the shape the index was written against.
    let (picked, fit_in, fit_out) = match &consumed {
        None => {
            let picked = nearest_joint(&joints);
            (picked, joints[picked].fit_in, joints[picked].fit_out)
        }
        Some((fillet, decision)) => {
            // The index is only meaningful against a joint space of the
            // shape it was written for, so the shape is what gets
            // verified — and a differing shape refuses BEFORE the index
            // is used, never gets clamped into range.
            if joints.len() != decision.survivors || decision.candidate >= joints.len() {
                return Err(structure(StructureRefusal::flipped(
                    Decision::JointSpace { fillet: *fillet },
                    DecisionValue::Count(decision.survivors),
                    DecisionValue::Count(joints.len()),
                )));
            }
            // WHY THE INDEX IS SOUND, which is not the reason the
            // ladder's own docs give. The recorded index addresses a
            // position in the flattened (corner, candidate) list, so it
            // means what it meant only if THIS pass built the same list
            // in the same order. That holds because the list is
            // produced by one formula from one corner list: `derive`
            // enumerates corners by the carrier pair's KIND (never by
            // value), the gate outcomes are consumed from the record so
            // the kept set is the recorded one by construction, and
            // `arc_fillet_trims` appends survivors per corner in its
            // own fixed order. The `survivors` comparison above is what
            // guards that chain — it is the one observable that a
            // differing corner list or a differing per-corner survivor
            // count would move — which is why it is checked BEFORE the
            // index is used rather than beside it.
            let picked = decision.candidate;
            for (site, recorded, found) in [
                (
                    Decision::FitIn { fillet: *fillet },
                    decision.fit_in,
                    joints[picked].fit_in,
                ),
                (
                    Decision::FitOut { fillet: *fillet },
                    decision.fit_out,
                    joints[picked].fit_out,
                ),
            ] {
                if recorded != found {
                    return Err(structure(StructureRefusal::flipped(
                        site,
                        DecisionValue::Sign(recorded),
                        DecisionValue::Sign(found),
                    )));
                }
            }
            // The RECORDED fits are what the emission branches read:
            // a fit sign decides whether a straight piece and its
            // declared joint exist at all, so adopting this lane's
            // answer would be selecting structure at the lane.
            (picked, decision.fit_in, decision.fit_out)
        }
    };
    let c = joints[picked];
    guide.record(FilletDecision {
        corners: gates,
        survivors: joints.len(),
        candidate: picked,
        fit_in,
        fit_out,
    });
    Ok(ArcFilletTrims {
        t1: c.t1,
        t2: c.t2,
        bulge: c.bulge,
        fit_in,
        fit_out,
        in_arc: legs_of[picked],
        arc: ArcData {
            center: c.center,
            radius,
        },
    })
}

// ------------------------------------------------------------------
// The carrier-aware binder family (§3a; LB6 for the closing arrival).
//
// These doors live HERE, not in `path.rs`, for the reason the module
// docs give: resolving an arc-carrier fillet reaches the lifted ladder,
// the ladder reads the S8 diagnostic channel, and a `Bounds` bound
// propagates to every caller. Confining the family to this file is what
// keeps `path.rs` itself bracket-free.
// ------------------------------------------------------------------

/// The carrier's unit tangent at `p`, in the travel sense `winding`
/// names — the DERIVED direction a `Center`-mode binding puts in
/// the angle slot. Written as `Leg::at_corner` writes it (τ·(P−O)⟂/R),
/// so the two doors agree at the bit.
///
/// The radius is gated definitely positive on the same funnel predicate
/// the `Center` leg mode uses: an anchor at the centre names no tangent.
pub(crate) fn carrier_tangent<T: Decide>(
    p: Point2<T>,
    centre: Point2<T>,
    winding: ArcSweep,
    band: Band,
) -> Result<Dir<T>, PathError<T>> {
    let v = p - centre;
    let radius = v.norm_squared().sqrt();
    match decide("path_arc_center_radius", Margin::of(radius), band) {
        Ok(Sign::Positive) => {}
        Ok(_) => return Err(PathError::DegenerateArcCenter { radius }),
        Err(source) => return Err(PathError::Escalated { source }),
    }
    let turn = match winding {
        ArcSweep::Ccw => T::one(),
        ArcSweep::Cw => -T::one(),
    };
    Ok(Dir::from_unit(Vec2::new(-v.y, v.x) * (turn / radius)))
}

/// The scalar obligation the arc-carrier arrival binders impose, NAMED.
///
/// The interior arc arrival and the closing one both run the S8 selection
/// ladder, so they are `Decide + Bounds` honestly — the compound bound
/// this file is allowlisted for. The replay driver
/// ([`super::program::replay`]) must be able to call them, so its own
/// signature inherits the obligation; naming it here keeps the compound
/// bound CONFINED to this file, which is exactly what the confinement
/// exists for ("so path.rs itself stays bracket-free"). The driver reads
/// no bracket of its own: it propagates this obligation and nothing more.
pub trait ArcCarrierScalar: Decide + Bounds {}

impl<T: Decide + Bounds> ArcCarrierScalar for T {}
