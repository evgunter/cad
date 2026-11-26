//! **Where an edge may touch a curved carrier, read against the face.**
//!
//! The crossing layer's root doors answer a span whose meetings with a
//! carrier they cannot certify — a tangency, whose roots coincide within
//! the band, or a count the door cannot settle — as uncertain. That is a
//! statement about the CARRIER. Whether the span meets the FACE is a
//! second question, and [`off_face`] answers it where it can: the span's
//! meetings with the carrier are localized into a few small balls, and a
//! ball whose centre is placed `Out` of the face, with no vertex or edge
//! of the face reaching it, holds no point of the face.
//!
//! # Localizing
//!
//! The span is bisected. A piece reaches at most `ℓ` metres along the
//! curve from its middle point `m`, and it meets the carrier nowhere when
//! a lower bound on `|dist|` over it decides positive. Two bounds are
//! taken, the larger deciding: `|dist(m)| − ℓ`, since the distance is
//! 1-Lipschitz; and, where the piece keeps clear of the carrier's medial
//! axis, the second-order `|dist(m)| − ℓ·|g| − ℓ²·H/2`, `g` the
//! distance's derivative along the curve at `m` and `H` a bound on its
//! second: `1/(ρ − |dist(m)| − ℓ)` for the carrier's level sets (ρ
//! below), plus the curve's own curvature
//! ([`geom::carrier_speed::curvature_hi`]). `ℓ` is metered at the curve's
//! speed bound ([`geom::carrier_speed::speed_range`]) on every kind. The
//! second bound needs the curve `C¹` across the piece, and a curvature
//! bound is finite only where it is: a spline with a crease, or whose
//! speed floor certifies nothing, bounds none, and the first bound
//! decides alone (at `Interval`, a bound unbounded above leaves the
//! second bound unbounded below, which the larger-of drops). Near a tangency the first
//! clears nothing finer than the band, the second clears pieces in
//! proportion to their distance from the touch. The pieces that do not
//! clear, bisected down to [`geom_core::bisect::tangency_floor`] of ρ, run together
//! into clusters. A cluster reaching `ℓ` about `m` meets the carrier, if
//! at all, within `ℓ + |dist(m)|` of `m`'s foot on the carrier, and that
//! radius, widened by `escalate`, is its ball's.
//!
//! # Reading a ball against the face
//!
//! ρ is the carrier's reach — the distance from it to its medial axis:
//! a sphere's or a cylinder's radius, a ring torus's `min(r, R − r)`,
//! and no bound on a plane. A ball about a point of the carrier with
//! radius below ρ meets the carrier in one disc. With no vertex or edge of the face inside the
//! ball, the face holds all of that disc or none of it, and the foot,
//! placed `Out` of the face by its door
//! ([`crate::contain::curved_face_placement`]), says none. Every edge is
//! decided clear by a lower bound on its distance from the foot
//! ([`edge_clear_of_ball`]).
//!
//! On a plane the level sets are flat, so the second bound charges the
//! curve's curvature alone, and the walk stops at the floor the curve's
//! own bend sets.
//!
//! A carrier with no closed-form distance and foot (a cone, a spline)
//! localizes nothing, and the door stays.

use geom_core::{Band, Bounds, Decide, Margin, Point3, Sign, SupSpeed};

use super::BooleanError;
use super::face_reading::{MedialDistance, ball_off_face, distance, reach};
use crate::body::Body;
use crate::boxes;
use crate::entity::{EdgeKey, FaceKey};
use crate::validate::decide;
use geom_core::bisect::Step;
use geom_core::ladder::{Answering, Climb, Rung, climb};

/// The most clusters a span may localize into: the most meetings a span
/// of a line or a conic has with these carriers. A conic against a
/// torus is degree 8 in the conic's half-angle tangent, and every other
/// such pair (a line against a torus, any of them against a quadric) is
/// at most 4. A spiric or spline span meeting the carrier more often
/// than that is not localized.
const CLUSTER_BUDGET: usize = 8;

/// **Whether every meeting of the span `carrier(t0..t1)` with
/// `surface` is certified off `face`** (module docs). `false` keeps the
/// caller's door.
///
/// # Errors
///
/// The placement's escalation, a stale face, and an edge of the face
/// with no certified carrier.
pub(super) fn off_face<T: Decide + Bounds>(
    y: &Body<T>,
    face: FaceKey,
    surface: &geom::Surface<T>,
    carrier: &geom::Curve3<T>,
    (t0, t1): (T, T),
    band: Band,
) -> Result<bool, BooleanError> {
    let Some(rho) = reach(surface) else {
        return Ok(false);
    };
    let speed = geom::carrier_speed::speed_range(carrier).1;
    let bend = geom::carrier_speed::curvature_hi(carrier);
    let Some(clusters) = clusters(surface, carrier, (speed, bend), rho, (t0, t1), band) else {
        return Ok(false);
    };
    let half = T::from_f64(0.5);
    // Every cluster off the face, an identity: any cluster not certified
    // off answers `false`, whatever an earlier cluster read ([`climb`],
    // [`Answering::Any`]). A cluster in band is kept by its standing
    // ([`BooleanError::refusal`]), refusing only where every other is off.
    let not_off = climb(Answering::Any, clusters, |(a, b)| {
        let m = carrier.eval((a + b) * half);
        let Some((dist, foot, _)) = distance(surface, m) else {
            return Rung::Answers(Ok(()));
        };
        let radius =
            geom_core::bisect::reach(speed, (a, b)) + dist.abs() + T::from_f64(band.escalate());
        Rung::holds(
            ball_off_face(y, face, foot, radius, rho, band).map(|off| !off),
            (),
            BooleanError::refusal,
        )
    });
    match not_off {
        Climb::Answered { answer, .. } => answer.map(|()| false),
        Climb::Passed => Ok(true),
        Climb::Undecided(e) => Err(e),
    }
}

/// A localization that stopped with no clusters to answer: a piece
/// with no distance, or more clusters than [`CLUSTER_BUDGET`].
struct Unlocalized;

/// The parameter runs of the span that may meet the carrier, in order
/// (module docs), the curve's speed and curvature bounds and the
/// carrier's reach given; `None` past the budgets.
fn clusters<T: Decide + Bounds>(
    surface: &geom::Surface<T>,
    carrier: &geom::Curve3<T>,
    (speed, bend): (SupSpeed<T>, T),
    rho: MedialDistance<T>,
    (t0, t1): (T, T),
    band: Band,
) -> Option<Vec<(T, T)>> {
    // The radius the carrier's level sets part from a tangent curve over
    // (module docs): its reach, or on a plane the curve's own radius of
    // curvature. A bend with no finite bound sets no floor, and the walk
    // runs to its budget.
    let parting = match rho {
        MedialDistance::Bounded(rho) => rho,
        MedialDistance::Unbounded if geom_core::is_finite_length(bend) => T::one() / bend,
        MedialDistance::Unbounded => T::zero(),
    };
    let floor = geom_core::bisect::tangency_floor(parting, band.escalate()).hi();
    let mut out: Vec<(T, T)> = Vec::new();
    // Whether a cleared piece came after the last open one: the next open
    // piece starts a cluster.
    let mut broken = true;
    let half = T::from_f64(0.5);
    let walk = geom_core::bisect::bisect([(t0, t1)], |(a, b)| {
        let mid = (a + b) * half;
        let ell = geom_core::bisect::reach(speed, (a, b));
        let Some((dist, _, normal)) = distance(surface, carrier.eval(mid)) else {
            return Step::Stop(Unlocalized);
        };
        let tangent = carrier.deriv(mid);
        let slope = (normal.dot(tangent) / tangent.norm()).abs();
        let lipschitz = dist.abs() - ell;
        // A bend with no finite bound (a spline's crease or cusp) takes
        // the first bound alone. At `Interval` a bend unbounded above (a
        // speed floor straddling zero) passes here, and leaves the second
        // bound's lower end unbounded below, so the `max` keeps the
        // first's. A plane's level sets do not bend.
        let curl = match rho {
            _ if !geom_core::is_finite_length(bend) => None,
            MedialDistance::Unbounded => Some(bend),
            MedialDistance::Bounded(rho) => {
                let axis_gap = rho - dist.abs() - ell;
                matches!(
                    decide("bool_touch_piece_off_axis", Margin::of(axis_gap), band),
                    Ok(Sign::Positive)
                )
                .then(|| T::one() / axis_gap + bend)
            }
        };
        let bound = match curl {
            Some(curl) => lipschitz.max(dist.abs() - ell * slope - ell.powi(2) * curl * half),
            None => lipschitz,
        };
        if let Ok(Sign::Positive) = decide("bool_touch_piece_clear", Margin::of(bound), band) {
            broken = true;
            return Step::Settled;
        }
        if ell.hi() > floor {
            return Step::Halve([(a, mid), (mid, b)]);
        }
        match out.last_mut() {
            Some(last) if !broken => last.1 = b,
            _ => {
                if out.len() == CLUSTER_BUDGET {
                    return Step::Stop(Unlocalized);
                }
                out.push((a, b));
            }
        }
        broken = false;
        Step::Settled
    });
    match walk {
        Ok(None) => Some(out),
        Ok(Some(Unlocalized)) | Err(_) => None,
    }
}

/// Whether a lower bound `gap` on a distance from a ball's centre
/// definitely exceeds its `radius`.
pub(super) fn gap_clears<T: Decide>(gap: T, radius: T, band: Band) -> bool {
    matches!(
        decide("bool_touch_edge_clear", Margin::of(gap - radius), band),
        Ok(Sign::Positive)
    )
}

/// **Whether `edge` of `y` stays definitely farther than `radius` from
/// `at`.** Its certified box, padded by the sweep's pad, clear of the
/// ball's box answers at once. Otherwise a lower bound on the distance
/// is decided against `radius`, read on the edge's own carrier
/// ([`crate::splitting::containment::arc_distance_floor`]: a segment,
/// any other carrier's cover, and then a spiric's or a spline's arc's
/// pieces). A carrier with no cover over its span (a spline net carrying
/// poison) is not clear.
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] for an edge with no
/// certified carrier.
pub(super) fn edge_clear_of_ball<T: Decide + Bounds>(
    y: &Body<T>,
    edge: EdgeKey,
    at: Point3<T>,
    radius: T,
    band: Band,
) -> Result<bool, BooleanError> {
    let pad = boxes::sweep_pad(band);
    if !boxes::edge_box(y, edge, pad).overlaps(&boxes::centred_box(at, radius, pad)) {
        return Ok(true);
    }
    let curve = super::reduce::certified(y.proven_edge(edge))?;
    Ok(arc_clear_of_ball(
        curve.carrier(),
        curve.params(),
        at,
        radius,
        band,
    ))
}

/// **Whether the arc `span` of `carrier` stays definitely farther than
/// `radius` from `at`**: its distance floor
/// ([`crate::splitting::containment::arc_distance_floor`]) decided past
/// `radius`. A carrier with no cover over the span is not clear.
pub(super) fn arc_clear_of_ball<T: Decide>(
    carrier: &geom::Curve3<T>,
    span: (T, T),
    at: Point3<T>,
    radius: T,
    band: Band,
) -> bool {
    crate::splitting::containment::arc_distance_floor(carrier, span, at)
        .is_some_and(|floor| floor.clears(|gap| gap_clears(gap, radius, band)))
}

#[cfg(test)]
#[path = "carrier_touch_rows.rs"]
mod carrier_touch_rows;
