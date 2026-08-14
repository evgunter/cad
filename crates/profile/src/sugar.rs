//! Constructor sugar: human-friendly arc forms lowered to bulges.
//!
//! The **stored** form is always the bulge chain (the crate docs'
//! zero-consistency-conditions rule); sugar may take direction hints
//! ([`ArcSweep`]) or through-points, but nothing beyond the computed
//! bulge survives into the data.
//!
//! Sugar is *evaluation code*: total, comparison-free, no decisions —
//! with ONE documented exception: **the fillet constructor's gates**
//! ([`line_line_fillet_trims`] / [`arc_fillet_trims`]), reified
//! predicates through the k_stats funnel. The original (#101 review
//! MAJOR-1) is `fillet_leg_fit`, which refuses a radius whose tangent
//! points fall outside their legs; M5 S2's arc-leg corners add
//! `fillet_leg_reach` (the corner-side extent test, same exact-order
//! band) and, against the run's linear band, `fillet_corner_arm`,
//! `fillet_corner_turn` and the `fillet_offset_*` carrier-intersection
//! family — the last of which, `fillet_offset_lever` (M8), decides not
//! whether the offset carriers meet but whether the meeting point can be
//! back-projected onto a leg's carrier within ε at all.
//! They are the one place sugar decides, and they decide only
//! *which construction the author asked for* — never a geometric
//! verdict about the finished loop, which stays
//! [`crate::Profile::validate`]'s. Everything else holds everywhere:
//! degenerate inputs (a through-point collinear-outside its chord, a
//! zero-radius center) produce well-defined poison or degenerate values
//! that [`crate::Profile::validate`] rejects with typed errors — the
//! sugar never guesses and never panics.

use geom_core::{
    Band, BandError, Decide, Indeterminate, Margin, Point2, Real, Sign, Tolerance, Vec2,
};

use crate::k_stats::decide;
use crate::validate::{FilletLeg, NoCornerReason};

/// The sweep direction hint for [`bulge_from_center`] /
/// the closing arc constructors: which way the arc winds about its
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
/// ([`arc_fillet_trims`]).
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

/// The line×line fillet's computed trim geometry — the output of
/// [`line_line_fillet_trims`], shared verbatim by the PATHS algebra's
/// lowering (`crate::path`) and the test-support twin so both doors emit
/// bit-identical geometry from one closed form (LIB-U2 PR-1; the same
/// one-door discipline `fillet_corner`'s line×line delegation states).
#[derive(Clone, Copy, Debug)]
pub(crate) struct LineFilletTrims<T: Real> {
    /// The incoming tangent point `corner − setback·v̂₁`. Meaningful only
    /// when `fit_in` is `Positive` (an exact-fit side has `t1` at the
    /// head itself and emits no straight piece; a shorter leg never gets
    /// here — the fit gate refuses first).
    pub t1: Point2<T>,
    /// The outgoing tangent point `corner + setback·v̂₂` — always the
    /// fillet arc's end.
    pub t2: Point2<T>,
    /// The fillet arc's bulge tan(φ/4), by the quarter-angle identity on
    /// `half_tan` (see [`line_line_fillet_trims`]'s docs).
    pub bulge: T,
    /// tan(φ/2): the corner's signed half-turn — its sign is the turn
    /// side σ (positive = left/counterclockwise).
    pub half_tan: T,
    /// The incoming leg's fit classification (`fillet_leg_fit`,
    /// exact-order band): `Positive` emits the straight piece + declared
    /// joint, `Zero` suppresses both (exact fit).
    pub fit_in: Sign,
    /// The outgoing leg's fit classification, same rule.
    pub fit_out: Sign,
}

/// A refusal from [`line_line_fillet_trims`], carried at the scalar so
/// the helper itself needs no bracket read (Bounds scope rule): each
/// door maps it into its own error vocabulary —
/// the test-support twin to `ProfileError` with `.lo()`
/// diagnostics (this file is the ratified fillet-gate seam), the PATHS
/// lowering to `PathError` with scalar payloads.
#[derive(Clone, Copy, Debug)]
pub(crate) enum TrimRefusal<T: Real> {
    /// A Negative `fillet_leg_fit`: the tangent setback exceeds the
    /// (first, in incoming→outgoing order) overrun leg's extent.
    DoesNotFit {
        /// The overrun leg.
        leg: FilletLeg,
        /// The tangent setback from the corner along the leg.
        setback: T,
        /// The overrun leg's extent.
        leg_length: T,
    },
    /// An in-band or poisoned fit margin (doubled-back corners and
    /// zero legs land here).
    Escalated(Indeterminate),
    /// The exact-order band could not be formed (unreachable for the
    /// built-in band).
    Band(BandError),
}

/// The ratified line×line fillet closed form (#101 review MAJOR-1;
/// docs on the fillet constructor), extracted verbatim so the algebra
/// lowering and the builder door share one code path: with legs
/// v₁ = corner − head, v₂ = next − corner, computes tan(φ/2), the arc
/// bulge, the setback, both `fillet_leg_fit` gates (exact-order band),
/// and the tangent points.
///
/// # Errors
///
/// [`TrimRefusal`], mapped by each door — see its docs.
pub(crate) fn line_line_fillet_trims<T: Decide>(
    head: Point2<T>,
    corner: Point2<T>,
    next: Point2<T>,
    radius: T,
) -> Result<LineFilletTrims<T>, TrimRefusal<T>> {
    let v1 = corner - head;
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
    let exact = Band::new(f64::from_bits(1), f64::from_bits(2)).map_err(TrimRefusal::Band)?;
    let fit = |len: T, leg: FilletLeg| -> Result<Sign, TrimRefusal<T>> {
        match decide("fillet_leg_fit", Margin::of(len - setback), exact) {
            Ok(Sign::Negative) => Err(TrimRefusal::DoesNotFit {
                leg,
                setback,
                leg_length: len,
            }),
            Ok(sign) => Ok(sign),
            Err(source) => Err(TrimRefusal::Escalated(source)),
        }
    };
    let fit_in = fit(len1, FilletLeg::Incoming)?;
    let fit_out = fit(len2, FilletLeg::Outgoing)?;
    // t1's division is guarded by use: it is consumed only on a Positive
    // fit (len1 > setback ≥ 0 ⇒ len1 > 0); on other fits the value may
    // be poison and is dead — total evaluation code, no branch.
    let t1 = corner - v1 * (setback / len1);
    let t2 = corner + v2 * (setback / len2);
    Ok(LineFilletTrims {
        t1,
        t2,
        bulge,
        half_tan,
        fit_in,
        fit_out,
    })
}

/// One surviving branch of the arc-carrier offset construction: the
/// fillet arc it would emit, plus the classifications and the setback
/// pair the selection ladder reads.
///
/// Every field is at the SCALAR — the setbacks in particular, because
/// the S8 ladder compares their f64 diagnostic channel and the bracket
/// read belongs to the calling door (Bounds scope rule), not here.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ArcFilletCandidate<T: Real> {
    /// The tangent point on the incoming leg.
    pub t1: Point2<T>,
    /// The tangent point on the outgoing leg.
    pub t2: Point2<T>,
    /// The fillet arc's bulge tan(θ/4).
    pub bulge: T,
    /// The fillet arc's own centre — the offset-carrier intersection
    /// this candidate IS. Carried for the algebra door's §4 item 4
    /// carrier-identity checks; no emitted coordinate reads it, and the
    /// builder door ignores it.
    pub center: Point2<T>,
    /// The incoming leg's `fillet_leg_fit` classification: `Positive`
    /// emits the trimmed piece + declared joint, `Zero` suppresses both.
    pub fit_in: Sign,
    /// The outgoing leg's fit classification, same rule.
    pub fit_out: Sign,
    /// The `[incoming, outgoing]` tangent setbacks from the corner, in
    /// meters (arc lengths `R·Δθ` on circular legs) — [`nearest_candidate`]'s
    /// input once the door has read the diagnostic channel.
    pub setbacks: [T; 2],
}

/// What [`arc_fillet_trims`] resolved a corner into.
#[derive(Clone, Debug)]
pub(crate) enum ArcFilletOutcome<T: Real> {
    /// Both legs are straight: the ratified line×line closed form owns
    /// this corner (one door, one construction). The arm and turn gates
    /// have already fired, in that order — delegating is the LAST thing
    /// this helper does, exactly as the shipped constructor did.
    LineLine,
    /// At least one circular leg: the resolved legs (their carriers are
    /// what the door re-emits the trimmed piece along) and the surviving
    /// candidates in enumeration order — never empty.
    Arc {
        /// `[incoming, outgoing]`, resolved at the corner.
        legs: [Leg<T>; 2],
        /// The corner-side, fitting candidates, in construction order.
        survivors: Vec<ArcFilletCandidate<T>>,
    },
}

/// A refusal from [`arc_fillet_trims`], carried at the scalar so the
/// helper itself needs no bracket read (Bounds scope rule) — the
/// [`TrimRefusal`] pattern, one level up: each door maps it into its own
/// error vocabulary, and the test-support twin does so with the
/// `.lo()` diagnostics through its own mapper.
///
/// Some payload fields are read only by the door that maps this into
/// [`crate::validate::ProfileError`] — the banished raw builder, which
/// lives behind `test-support` (PROFILES-V2 §V6). The algebra's own
/// mapper (`path::arc_fillet`) reads the margins it reports and leaves
/// the rest; the fields stay because they are the CONSTRUCTION's
/// diagnostics, not one caller's.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(not(feature = "test-support"), allow(dead_code))]
pub(crate) enum ArcTrimRefusal<T: Real> {
    /// `fillet_corner_arm` not Positive: no length scale, so the
    /// corner's angle means nothing. Both arms ride so the door can name
    /// the shorter leg on its own channel.
    LegDegenerate {
        /// The incoming leg's lever arm.
        leg_in_arm: T,
        /// The outgoing leg's lever arm.
        leg_out_arm: T,
        /// Their minimum — the gate's margin.
        arm: T,
    },
    /// `fillet_corner_turn` Zero: the legs meet tangentially or double
    /// back, so there is no corner to cut.
    AlreadyTangent {
        /// `dir_in · dir_out` — negative means the legs double back.
        align: T,
        /// The turn margin in meters.
        margin: T,
        /// The lever arm it was levered by.
        arm: T,
    },
    /// `fillet_offset_lever` not Positive: the leg's **offset radius**
    /// ρ = R − σ·τ·r — the lever the tangent point is recovered over —
    /// is shorter than the least lever the run's band can support, so
    /// the fillet centre's back-projection onto that carrier cannot be
    /// placed within ε. A corner and a tangent circle both exist; what
    /// does not exist is a *certifiable* tangent point (D4 ¶2).
    ///
    /// See [`ArcCarrier::offset_circles`] for the gate's derivation and
    /// for why only the outgoing leg can reach this arm.
    OffsetLeverTooShort {
        /// The leg whose offset lever is short.
        leg: FilletLeg,
        /// That leg's carrier radius R.
        carrier_radius: T,
        /// Its signed offset radius ρ = R − σ·τ·r — the lever itself.
        offset_radius: T,
        /// The least |ρ| the band supports at this corner's scale.
        least_lever: T,
        /// The gate's margin |ρ| − least_lever, meters.
        margin: T,
    },
    /// A Negative `fillet_leg_fit` on a corner-side candidate: the
    /// radius pushes a tangent point off the far end of its leg.
    DoesNotFit {
        /// The overrun leg.
        leg: FilletLeg,
        /// The leg's carrier radius, `None` for a straight leg — the
        /// door turns it into a [`FilletLegCarrier`].
        carrier_radius: Option<T>,
        /// The fit margin `extent − setback`, meters (the door divides
        /// by the carrier radius for the angular story).
        margin: T,
        /// The tangent setback from the corner along the leg.
        setback: T,
        /// The overrun leg's extent.
        leg_length: T,
    },
    /// The offset carriers admit no corner-side candidate at all.
    NoCorner {
        /// Which way it failed.
        reason: NoCornerReason,
        /// The authored radius.
        radius: T,
    },
    /// An in-band or poisoned gate margin.
    Escalated(Indeterminate),
    /// The band could not be formed (only for a misconfigured ε).
    Band(BandError),
}

/// The ratified arc-carrier fillet construction (M5 S2), extracted
/// verbatim from the raw builder's corner door so the twin and
/// the PATHS algebra lowering share one code path — the
/// [`line_line_fillet_trims`] pattern, applied to the offset-carrier
/// corner.
///
/// Runs the arm gate, the turn gate, the offset-carrier intersection and
/// the per-candidate reach/fit pass, in exactly the shipped order and
/// with exactly the shipped `decide` sequence, and returns EVERY
/// surviving candidate rather than one pick. The selection is the
/// caller's: the test-support twin applies
/// [`nearest_candidate`] over the diagnostic channel (the S8 rule), and
/// the algebra applies it over the joint corner×candidate space, which
/// is why the survivors — not a winner — are what crosses this seam.
///
/// `T: Decide` only: no bracket is read here, so the interval lane's
/// diagnostics stay at the doors (Bounds scope rule).
///
/// # Errors
///
/// [`ArcTrimRefusal`], mapped by each door — see its docs.
pub(crate) fn arc_fillet_trims<T: Decide>(
    incoming: FilletLegShape<T>,
    head: Point2<T>,
    corner: Point2<T>,
    outgoing: FilletLegShape<T>,
    next: Point2<T>,
    radius: T,
    tol: Tolerance,
) -> Result<ArcFilletOutcome<T>, ArcTrimRefusal<T>> {
    let band = Band::new(tol.eps, tol.k * tol.eps).map_err(ArcTrimRefusal::Band)?;
    // The exact-order band (validate module docs): no representable
    // f64 lies strictly inside it, so f64 classification is total.
    let exact = Band::new(f64::from_bits(1), f64::from_bits(2)).map_err(ArcTrimRefusal::Band)?;
    let leg_in = Leg::at_corner(incoming, corner, head, FilletLeg::Incoming);
    let leg_out = Leg::at_corner(outgoing, corner, next, FilletLeg::Outgoing);

    // (1) the lever arm: an angle is nothing without one (D4 ¶1).
    let arm = leg_in.arm.min(leg_out.arm);
    if decide("fillet_corner_arm", Margin::of(arm), band).map_err(ArcTrimRefusal::Escalated)?
        != Sign::Positive
    {
        return Err(ArcTrimRefusal::LegDegenerate {
            leg_in_arm: leg_in.arm,
            leg_out_arm: leg_out.arm,
            arm,
        });
    }

    // (2) the corner's turn: its sign is the side both carriers offset
    // toward, so the offset construction never searches.
    let turn = Margin::levered(leg_in.dir.perp_dot(leg_out.dir), arm);
    let sgn = match decide("fillet_corner_turn", turn, band).map_err(ArcTrimRefusal::Escalated)? {
        Sign::Positive => T::one(),
        Sign::Negative => -T::one(),
        Sign::Zero => {
            return Err(ArcTrimRefusal::AlreadyTangent {
                align: leg_in.dir.dot(leg_out.dir),
                margin: turn.value(),
                arm,
            });
        }
    };

    // (3) the offset carriers' intersection — the candidate centers.
    let centers = match (leg_in.arc, leg_out.arc) {
        (None, None) => return Ok(ArcFilletOutcome::LineLine),
        (Some(arc), None) => leg_out.offset_line_circle(corner, sgn, radius, arc, band)?,
        (None, Some(arc)) => leg_in.offset_line_circle(corner, sgn, radius, arc, band)?,
        (Some(a), Some(b)) => a.offset_circles(b, sgn, radius, band)?,
    };

    // (4/5) the branch rule and the fit gate, one pass in candidate
    // order: a candidate survives when both tangent points lie *within*
    // their legs' corner-side extents — setback ≥ 0 (`fillet_leg_reach`,
    // the corner end; SIGNED on both leg kinds, so "past the corner"
    // really does classify Negative) and extent − setback ≥ 0
    // (`fillet_leg_fit`, the far end, generalized to arc lengths R·Δθ on
    // a circular leg).
    //
    // Four classifications per candidate, always all four — no
    // short-circuit, so the recorded sample sequence depends only on the
    // corner CLASS (how many candidates the carriers admit), not on the
    // numbers. That invariance is stated for the non-escalating path:
    // any one of the four `decide` calls may return Indeterminate, and
    // the `?` below aborts with that escalation, leaving the remaining
    // classifications of this candidate and every later candidate
    // unfired. This is deliberate — an escalation means the gate cannot
    // be classified at this scalar, and continuing would be deciding the
    // branch rule on a margin we just admitted we cannot read. The
    // escalation names which predicate stopped it.
    //
    // The bulge is computed per SURVIVOR rather than once for the pick:
    // it is pure evaluation (no gate, no sample), and it is the same
    // expression on the same inputs, so the emitted value is unchanged
    // bit for bit while the survivors become self-contained.
    let mut survivors: Vec<ArcFilletCandidate<T>> = Vec::with_capacity(centers.len());
    let mut overrun: Option<ArcTrimRefusal<T>> = None;
    for center in centers {
        let t1 = leg_in.tangent_point(center, sgn, radius);
        let t2 = leg_out.tangent_point(center, sgn, radius);
        let sb_in = leg_in.setback(t1, corner);
        let sb_out = leg_out.setback(t2, corner);
        let reach_in = decide("fillet_leg_reach", Margin::of(sb_in), exact)
            .map_err(ArcTrimRefusal::Escalated)?;
        let reach_out = decide("fillet_leg_reach", Margin::of(sb_out), exact)
            .map_err(ArcTrimRefusal::Escalated)?;
        let margin_in = leg_in.len - sb_in;
        let margin_out = leg_out.len - sb_out;
        let fit_in = decide("fillet_leg_fit", Margin::of(margin_in), exact)
            .map_err(ArcTrimRefusal::Escalated)?;
        let fit_out = decide("fillet_leg_fit", Margin::of(margin_out), exact)
            .map_err(ArcTrimRefusal::Escalated)?;
        let corner_side = reach_in != Sign::Negative && reach_out != Sign::Negative;
        if corner_side && fit_in != Sign::Negative && fit_out != Sign::Negative {
            survivors.push(ArcFilletCandidate {
                t1,
                t2,
                bulge: fillet_bulge(t1, t2, center, radius, sgn),
                center,
                fit_in,
                fit_out,
                setbacks: [sb_in, sb_out],
            });
        } else if corner_side && overrun.is_none() {
            // This candidate rounds the corner the caller named, but the
            // radius pushes a tangent point off the far end of its leg:
            // the radius-does-not-fit situation, reported incoming leg
            // first exactly as `fillet` gates it.
            //
            // Attribution is sound because `corner_side` is a real test
            // (signed setback, review MAJOR-1): a candidate rounding the
            // OTHER intersection of the two carriers has a tangent point
            // past the corner, classifies Negative, and never reaches
            // this arm. So the numbers rendered are this candidate's
            // own, for the corner the author actually named.
            let (leg, setback, margin) = if fit_in == Sign::Negative {
                (&leg_in, sb_in, margin_in)
            } else {
                (&leg_out, sb_out, margin_out)
            };
            overrun = Some(ArcTrimRefusal::DoesNotFit {
                leg: leg.side,
                carrier_radius: leg.arc.map(|a| a.radius),
                margin,
                setback,
                leg_length: leg.len,
            });
        }
    }
    if survivors.is_empty() {
        return Err(overrun.unwrap_or(ArcTrimRefusal::NoCorner {
            reason: NoCornerReason::NoCornerSideCandidate,
            radius,
        }));
    }
    Ok(ArcFilletOutcome::Arc {
        legs: [leg_in, leg_out],
        survivors,
    })
}

// ------------------------------------------------------------------
// The arc-leg fillet's private machinery (M5 S2). Everything here is
// evaluation code except the `decide` calls, which are the documented
// exception named in the module docs.
// ------------------------------------------------------------------

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
pub(crate) fn signed_swept<T: Real>(from: T, to: T, turn: T) -> T {
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
pub(crate) struct ArcCarrier<T: Real> {
    /// The carrier circle's center.
    pub center: Point2<T>,
    /// Its radius, |corner − center|.
    pub radius: T,
    /// +1 for a counterclockwise leg, −1 for a clockwise one.
    pub turn: T,
    /// The same sense as the authored hint (for re-emitting the trimmed
    /// piece through `arc_to_center`).
    pub sweep: ArcSweep,
    /// The corner's angular coordinate on the carrier (radians).
    pub corner_angle: T,
}

/// One leg of a fillet corner, resolved at the corner.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Leg<T: Real> {
    /// The path's unit travel direction **at the corner**.
    pub dir: Vec2<T>,
    /// The carrier, for a circular leg; `None` for a straight one.
    pub arc: Option<ArcCarrier<T>>,
    /// The leg's extent in meters (chord length, or arc length R·Δθ).
    pub len: T,
    /// The leg's lever arm: its extent, folded with its carrier radius
    /// (the curvature arm) for a circular leg.
    pub arm: T,
    /// Which side of the corner this leg is.
    pub side: FilletLeg,
}

/// A circular leg's **signed** offset radius ρ = R − σ·τ·r (the
/// construction section of [`arc_fillet_trims`]).
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
    ///
    /// # The spoke is measured, not assumed
    ///
    /// On a circular leg the tangent point is the fillet centre pushed
    /// out along its spoke `center − O` to the carrier radius. The
    /// spoke's length **ought** to be the offset radius ρ, and the
    /// obvious scale factor is therefore `R/ρ` — but that is an
    /// assumption about a computed point, and where it is false it is
    /// false by exactly the radial error of `center`, which the factor
    /// then multiplies by `R/|ρ|` and deposits off the carrier. That is
    /// the amplification [`ArcCarrier::offset_circles`]' gate exists to
    /// bound, and the M8 `review_s2` red (a 2.29e-9 residual) was one
    /// instance of it.
    ///
    /// So divide by the spoke's **measured** length instead, signed like
    /// ρ (a negative offset radius puts the tangent point on the far
    /// side, and that sign is geometry, not roundoff). The returned point
    /// then lies on the carrier BY CONSTRUCTION, to a relative ulp of R,
    /// for any `center` whatever.
    ///
    /// # Why this reduces the error rather than hiding it
    ///
    /// The worry the shape invites — that zeroing the residual the tests
    /// measure just moves the error somewhere unmeasured — does not
    /// survive being written down. Let P\* be the exact fillet centre and
    /// δ = P − P\*. The true tangent point is
    /// `t* = O + (P* − O)·(R/ρ)`, because `|P* − O| = ρ` **exactly** —
    /// that is what it means for P\* to be the solution.
    ///
    /// - Scaling by the nominal ρ gives `t = t* + δ·(R/ρ)`: the whole of
    ///   δ, magnified.
    /// - Scaling by the measured length gives `t = t* + δ_⊥·(R/ρ) +
    ///   O(δ²)`, where `δ_⊥` is δ's component *across* the spoke.
    ///
    /// The difference is δ's radial component, and that component is
    /// **identifiable**: we know the true spoke length independently (it
    /// is ρ), so every part of `|P − O| − ρ` is error and none of it is
    /// signal. Discarding it is a projection onto a constraint P\* is
    /// known to satisfy, and such a projection cannot increase the
    /// distance to P\*. So `|t − t*|` is never larger than before and is
    /// usually much smaller — strictly better, for both legs, on every
    /// input.
    ///
    /// What remains is `δ_⊥·(R/ρ)`: the spoke's *angular* error, which
    /// displaces the tangent point ALONG its carrier. That one is real,
    /// is not removable here, and is what a conditioning gate should be
    /// measured against — see [`ArcCarrier::offset_circles`].
    fn tangent_point(&self, center: Point2<T>, sgn: T, radius: T) -> Point2<T> {
        match self.arc {
            None => center - left_normal(self.dir) * (sgn * radius),
            Some(arc) => {
                let spoke = center - arc.center;
                let rho = offset_radius(&arc, sgn, radius);
                arc.center + spoke * (arc.radius / spoke.norm().copysign(rho))
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
// and it is the same shape the raw builder's own gated methods use.

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
    ) -> Result<Vec<Point2<T>>, ArcTrimRefusal<T>>
    where
        T: Decide,
    {
        let normal = left_normal(self.dir);
        let on_offset = corner + normal * (sgn * radius);
        let rho = offset_radius(&arc, sgn, radius);
        let h = (arc.center - on_offset).dot(normal);
        let foot = arc.center - normal * h;
        Ok(
            match decide(
                "fillet_offset_line_circle",
                Margin::of(rho.abs() - h.abs()),
                band,
            )
            .map_err(ArcTrimRefusal::Escalated)?
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
                    return Err(ArcTrimRefusal::NoCorner {
                        reason: NoCornerReason::OffsetCarriersDisjoint,
                        radius,
                    });
                }
            },
        )
    }
}

/// The back-projection error constant of [`ArcCarrier::offset_circles`],
/// in units of `f64::EPSILON` — the ONE empirical number in the
/// conditioning gate, and the only thing in it that is not algebra.
///
/// The derivation it multiplies is in the gate's docs; what is measured
/// is the constant in front. Over a 2 000 000-corner sweep of the
/// `review_s2` draw distribution (1 272 793 accepted corners, 371 604 of
/// them arc×arc) the worst observed ratio of the actual off-carrier
/// residual to the derived expression was 1.94e-15 = 8.7·`f64::EPSILON`.
/// The shipped constant is 32 — **3.7x the measured worst**, chosen from
/// the two sides it is squeezed between:
///
/// - it must not refuse corners the kernel constructs correctly today.
///   At 32 the same sweep newly refuses **2 of 1 272 793** accepted
///   corners (0.16 per 100 000); the six `review_s2` enclosing fixtures
///   clear it by 135x–894x.
/// - it must refuse before the residual can reach ε. Over an 804 418-corner
///   sweep drawn ADVERSARIALLY (near-tangent turns, `R_out` within
///   1e-7 relative of `r`), of which 15 968 produced a residual above ε,
///   the gate refused **all 15 968**, the nearest of them 40x before its
///   threshold, and the worst residual among the corners it KEPT was
///   2.09e-11 — 48x inside ε.
///
/// It is a machine-precision count, not a tolerance: ε enters the gate
/// through the band, so the affordable conditioning loosens at ε = 1e-6
/// and tightens at ε = 1e-12 without this number moving.
///
/// # This constant is now CONSERVATIVE, and knowingly so
///
/// Every measurement above was taken before [`Leg::tangent_point`]
/// started dividing the spoke by its measured length. That change
/// removes one of the two `1/ρ₂` amplifications the derivation below
/// composes — the back-projection's — so the residual this constant is
/// calibrated against no longer occurs. On the draws that produced the
/// numbers above, the worst off-carrier residual fell from 1.18e-2 to
/// 2.22e-16, and the worst remaining defect (in the emitted fillet
/// RADIUS, not the carrier residual) now scales as `1/|ρ₂|` rather than
/// `1/ρ₂²`.
///
/// The gate is therefore refusing corners the kernel can now build
/// correctly — by roughly three decades of lever at ε = 1e-9. It is
/// left in place, unchanged and over-tight, rather than retuned in the
/// same change that invalidated its calibration: loosening a refusal is
/// a capability claim and deserves its own derivation, its own
/// adversarial sweep and its own constant, exactly as this one got.
/// Retuning it is tracked as follow-up work.
const BACK_PROJECTION_ULPS: f64 = 32.0;

impl<T: Real> ArcCarrier<T> {
    /// The candidate centers where this carrier's offset circle meets
    /// `other`'s (0, 1 or 2, in fixed order).
    ///
    /// # The conditioning gate (`fillet_offset_lever`)
    ///
    /// **Read this section as history.** It derives the gate from an
    /// amplification that [`Leg::tangent_point`] no longer performs: it
    /// used to rescale the spoke by the nominal `R/ρ`, which multiplied
    /// P's radial error by `R/|ρ|` and deposited it off the carrier, and
    /// it now divides by the spoke's measured length, which discards
    /// that error instead. What follows is still the right account of
    /// WHY a lever gate belongs here, and its `1/ρ₂²` law was correct
    /// for the code it was written against; but the surviving defect is
    /// one power of `ρ₂` weaker, so the constant is over-tight. See
    /// [`BACK_PROJECTION_ULPS`].
    ///
    /// A tangent point is recovered from the fillet centre P by
    /// projecting it back onto the leg's carrier,
    /// `t = O + R·(P − O)/|P − O|` with `|P − O| = |ρ|`
    /// ([`Leg::tangent_point`]). So a **radial** error in P — a
    /// deviation of `|P − O|` from `|ρ|` — lands at the tangent point
    /// multiplied by `R/|ρ|`, and the offset radius ρ is the lever the
    /// whole recovery hangs from. Nothing above gates that lever:
    /// `fillet_corner_turn` gates whether a corner EXISTS, and the two
    /// clearances below gate whether the offset carriers MEET. Whether
    /// the meeting point can be placed accurately enough to certify the
    /// tangent point at ε is this gate, and it is the one the M8
    /// `review_s2` red (a 2.29e-9 residual against a 1e-9 assertion)
    /// found missing.
    ///
    /// **Only `other`'s lever is gated, and the asymmetry is real.** Of
    /// the two offset circles, THIS one's radius is honoured by
    /// identity: `|P − self.center|² = along² + half²` and
    /// `half = √(ρ₁² − along²)`, so the two cancel exactly and P sits at
    /// `|ρ₁|` from `self.center` up to a relative ulp, whatever `along`
    /// did. The other circle has no such identity —
    /// `|P − other.center|² = d² − 2·d·along + ρ₁²`, whose derivative in
    /// `along` is `−d/|ρ₂|` — so `along`'s rounding arrives at
    /// `other`'s carrier amplified by `d/|ρ₂|`, and then again by
    /// `R₂/|ρ₂|` at the back-projection. `offset_line_circle` has the
    /// identity on BOTH sides (`h² + half² = ρ²`), which is why it
    /// carries no such gate — and measurement agrees: over 900 000
    /// line×arc corners the worst off-carrier residual was 1.3e-13,
    /// against 2.6e-10 for arc×arc on the same sweep.
    ///
    /// **The derivation.** `along = (d² + ρ₁² − ρ₂²)/2d` is evaluated in
    /// a scalar of unit roundoff `u`, so its absolute error is
    /// `≲ u·(d² + ρ₁² + ρ₂²)/d`. Propagating it through the two
    /// amplifications above:
    ///
    /// ```text
    ///   residual  ≈  (R₂/|ρ₂|) · (d/|ρ₂|) · u·(d² + ρ₁² + ρ₂²)/d
    ///             =  C · R₂ · scale² / ρ₂²,   scale² = d² + ρ₁² + ρ₂²
    /// ```
    ///
    /// Requiring `residual ≤ ε` inverts to a **least lever**, which is
    /// what the gate classifies (and which is why the margin is a
    /// length, in the currency of ρ itself):
    ///
    /// ```text
    ///   |ρ₂|  ≥  scale · √(C·R₂/ε)
    /// ```
    ///
    /// ε is read from the band, never written down — see
    /// [`BACK_PROJECTION_ULPS`] for C and for the measurements that fix
    /// it.
    fn offset_circles(
        self,
        other: ArcCarrier<T>,
        sgn: T,
        radius: T,
        band: Band,
    ) -> Result<Vec<Point2<T>>, ArcTrimRefusal<T>>
    where
        T: Decide,
    {
        let rho1 = offset_radius(&self, sgn, radius);
        let rho2 = offset_radius(&other, sgn, radius);
        let r1 = rho1.abs();
        let r2 = rho2.abs();
        let link = other.center - self.center;
        let dist_squared = link.norm_squared();
        let dist = dist_squared.sqrt();
        let external = decide(
            "fillet_offset_circles_external",
            Margin::of(r1 + r2 - dist),
            band,
        )
        .map_err(ArcTrimRefusal::Escalated)?;
        let internal = decide(
            "fillet_offset_circles_internal",
            Margin::of(dist - (r1 - r2).abs()),
            band,
        )
        .map_err(ArcTrimRefusal::Escalated)?;
        if external == Sign::Negative || internal == Sign::Negative {
            return Err(ArcTrimRefusal::NoCorner {
                reason: NoCornerReason::OffsetCarriersDisjoint,
                radius,
            });
        }
        // The conditioning gate, after the clearances on purpose: "the
        // carriers do meet, and what I cannot do is place the tangent
        // point there" is the precise story, and a pair that never met
        // keeps its own (stronger) refusal unchanged.
        //
        // powi(2)-discipline squares: both ρ straddle zero in general
        // (memories/interval-square-poison.md).
        let scale = (dist_squared + rho1.powi(2) + rho2.powi(2)).sqrt();
        let least_lever = scale
            * (T::from_f64(BACK_PROJECTION_ULPS * f64::EPSILON) * other.radius
                / T::from_f64(band.zero()))
            .sqrt();
        // `other` is the OUTGOING leg at the one call site that reaches
        // here (`(Some(a), Some(b)) => a.offset_circles(b, ..)`, a being
        // the incoming leg's carrier), and the docs above say why the
        // exposed side is the second argument's rather than either.
        if decide("fillet_offset_lever", Margin::of(r2 - least_lever), band)
            .map_err(ArcTrimRefusal::Escalated)?
            != Sign::Positive
        {
            return Err(ArcTrimRefusal::OffsetLeverTooShort {
                leg: FilletLeg::Outgoing,
                carrier_radius: other.radius,
                offset_radius: rho2,
                least_lever,
                margin: r2 - least_lever,
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
/// [`arc_fillet_trims`]** (review NOTE): the corner-side
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
    use super::*;

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
}
