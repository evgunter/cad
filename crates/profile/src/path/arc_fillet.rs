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
use geom_core::{Band, Bounds, Decide, Margin, Point2, Real, Sign, Tol, Vec2};

use super::{ArcData, Dir, PathError, PathNoCornerReason};
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
fn no_corner<T: Real>(reason: PathNoCornerReason, radius: T) -> PathError<T> {
    PathError::NoCornerForFillet { reason, radius }
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
    radius: T,
    band: Band,
) -> Result<(), PathError<T>> {
    let (name, margin) = side.travel(side.anchor, corner, "path_corner_advance_arc");
    match decide(name, Margin::of(margin), band) {
        Ok(Sign::Positive) => Ok(()),
        Ok(_) => Err(no_corner(PathNoCornerReason::BehindIncomingRay, radius)),
        Err(source) => Err(PathError::Escalated { source }),
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
    radius: T,
    band: Band,
) -> Result<(), PathError<T>> {
    let (name, margin) = side.travel(corner, side.anchor, "path_corner_reach_arc");
    match decide(name, Margin::of(margin), band) {
        Ok(Sign::Positive) => Ok(()),
        Ok(_) => Err(no_corner(PathNoCornerReason::BehindArrivalAnchor, radius)),
        Err(source) => Err(PathError::Escalated { source }),
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

/// An [`ArcTrimRefusal`] in the algebra's error vocabulary. The door
/// owns the bracket reads, exactly as `fillet_corner` does: the leg
/// diagnostics are `f64` enclosure lower bounds, for messages and never
/// for re-deciding.
fn map_refusal<T: Bounds>(refusal: ArcTrimRefusal<T>, radius: T) -> PathError<T> {
    match refusal {
        ArcTrimRefusal::Band(source) => PathError::Band(source),
        ArcTrimRefusal::Escalated(source) => PathError::Escalated { source },
        ArcTrimRefusal::LegDegenerate { arm, .. } => PathError::UnderdeterminedLeg {
            site: if arm.lo() <= 0.0 {
                "arc-carrier fillet leg with no length scale"
            } else {
                "arc-carrier fillet leg arm indeterminate"
            },
        },
        // Carriers meeting tangentially at the derived corner: the same
        // situation `path_corner_turn` names on the straight pair.
        ArcTrimRefusal::AlreadyTangent { .. } => {
            no_corner(PathNoCornerReason::CarriersParallel, radius)
        }
        ArcTrimRefusal::NoCorner { reason, radius } => {
            no_corner(PathNoCornerReason::NoTangentCircle(reason), radius)
        }
        // M8's conditioning gate. Deliberately NOT laundered into
        // `NoCornerForFillet`: a corner and a tangent circle both exist
        // here, and saying "no corner" about a corner that is right
        // there would send the author looking for the wrong thing. The
        // lever the message names is the one they can move.
        ArcTrimRefusal::OffsetLeverTooShort {
            leg,
            carrier_radius,
            offset_radius,
            least_lever,
            margin,
        } => PathError::FilletOffsetLeverTooShort {
            side: leg,
            carrier_radius,
            offset_radius,
            least_lever,
            margin,
        },
        // §3c: the anchor-fit refusal now carries the CARRIER KIND, so
        // an arc side gets its angular story (`FilletLegCarrier::Arc`'s
        // `angular_margin`) instead of a bare linear setback that means
        // nothing on a circle.
        ArcTrimRefusal::DoesNotFit {
            leg,
            carrier_radius,
            margin,
            setback,
            leg_length,
        } => PathError::AnchorOutsideTrimmedExtent {
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
        },
    }
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
/// [`PathError`] — a carrier pair with no corner, a corner every gate
/// rejects, an anchor the trim would eat (now carrying the arc side's
/// angular margin), or an escalation. An escalation ANYWHERE aborts
/// immediately: a joint space one of whose members cannot be classified
/// cannot be honestly ranked.
pub(crate) fn resolve<T: Decide + Bounds>(
    guide: &mut Guide<T>,
    incoming: FilletSide<T>,
    arrival: FilletSide<T>,
    radius: T,
    tol: Tol,
) -> Result<ArcFilletTrims<T>, PathError<T>> {
    let consumed = guide.consume().map_err(structure)?;
    let band = Band::new(tol.eps(), tol.k() * tol.eps()).map_err(PathError::Band)?;
    // Two refusal channels, deliberately. A corner the GATES discard is
    // the weaker story — the author's anchors simply do not bracket it,
    // and the other root is usually the one they meant. A corner that
    // PASSED the gates and then failed to admit a tangent circle is the
    // real answer, so it outranks the gate refusal when both happened.
    // (Reporting the gate's refusal there would say "no corner ahead of
    // you" about a corner that is in fact ahead of you.)
    let mut gate_refused: Option<PathError<T>> = None;
    let mut build_refused: Option<PathError<T>> = None;
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
    for (ci, corner) in derive(&incoming, &arrival, radius, band)?
        .into_iter()
        .enumerate()
    {
        let outcome = match advance_gate(&incoming, corner, radius, band) {
            Ok(()) => match reach_gate(&arrival, corner, radius, band) {
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
                        DecisionValue::Index(decision.corners.len()),
                        DecisionValue::Index(ci + 1),
                    )));
                };
                match &outcome {
                    // The escalation NAMES the gate it stopped at
                    // rather than surfacing as a bare band refusal: a
                    // driver bisecting the parameter box needs to know
                    // which decision went unconfirmed.
                    Err((_, PathError::Escalated { source })) => {
                        return Err(structure(StructureRefusal::indeterminate(
                            site,
                            source.clone(),
                        )));
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
            (false, Err((_, e @ PathError::Escalated { .. }))) => return Err(e),
            (false, Err((_, e))) => {
                if gate_refused.is_none() {
                    gate_refused = Some(e);
                }
            }
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
                return Err(PathError::Escalated { source });
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
                return Err(map_refusal(refusal, radius));
            }
            Err(refusal) => {
                if build_refused.is_none() {
                    build_refused = Some(map_refusal(refusal, radius));
                }
            }
        }
    }
    // (4) the lifted ladder over the flattened joint space.
    if joints.is_empty() {
        return Err(match (build_refused, gate_refused) {
            (Some(e), _) | (None, Some(e)) => e,
            (None, None) => no_corner(PathNoCornerReason::CarriersDoNotMeet, radius),
        });
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
                    DecisionValue::Index(decision.survivors),
                    DecisionValue::Index(joints.len()),
                )));
            }
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
