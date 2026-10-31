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
//! ([`super::contain::curved_face_placement`]), says none. Every edge is
//! decided clear by a lower bound on its distance from the foot
//! ([`edge_clear_of_ball`]).
//!
//! On a plane the level sets are flat, so the second bound charges the
//! curve's curvature alone, and the walk stops at the floor the curve's
//! own bend sets.
//!
//! A carrier with no closed-form distance and foot (a cone, a spline)
//! localizes nothing, and the door stays.

use geom_core::{Band, Bounds, Decide, Margin, Point3, Real, Sign, SupSpeed, Vec3};

use super::BooleanError;
use super::boxes;
use super::contain::{ContainError, CurvedPlacement, FaceContainment};
use super::{BooleanDecision, reduce};
use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, FaceKey};
use crate::live::BoundaryMember;
use crate::validate::decide;
use geom_core::bisect::Step;

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
    for (a, b) in clusters {
        let m = carrier.eval((a + b) * half);
        let Some((dist, foot, _)) = distance(surface, m) else {
            return Ok(false);
        };
        let radius =
            geom_core::bisect::reach(speed, (a, b)) + dist.abs() + T::from_f64(band.escalate());
        if !ball_off_face(y, face, foot, radius, rho, band)? {
            return Ok(false);
        }
    }
    Ok(true)
}

/// A carrier's reach, ρ in the module docs.
#[derive(Clone, Copy, Debug)]
pub(super) enum Reach<T> {
    /// The distance to the medial axis.
    Bounded(T),
    /// A plane's: it has no medial axis, and its level sets do not bend.
    Unbounded,
}

/// The carrier's reach; `None` for a kind with no distance here.
pub(super) fn reach<T: Real>(surface: &geom::Surface<T>) -> Option<Reach<T>> {
    match *surface {
        geom::Surface::Sphere { radius, .. } | geom::Surface::Cylinder { radius, .. } => {
            Some(Reach::Bounded(radius))
        }
        geom::Surface::Torus {
            major_radius,
            minor_radius,
            ..
        } => Some(Reach::Bounded(
            minor_radius.min(major_radius - minor_radius),
        )),
        geom::Surface::Plane { .. } => Some(Reach::Unbounded),
        geom::Surface::Cone { .. } | geom::Surface::Nurbs(_) | geom::Surface::Approx(_) => None,
    }
}

/// A point's signed distance from the carrier, its foot on it, and the
/// distance's gradient there (the unit normal at the foot). Not
/// `carrier_eq::distance_to`, which answers the unsigned distance alone
/// from a `CarrierDesc`: the localization needs the sign (which side a
/// piece lies on), the foot (the ball's centre) and the normal (the
/// second-order bound's slope).
fn distance<T: Real>(surface: &geom::Surface<T>, q: Point3<T>) -> Option<(T, Point3<T>, Vec3<T>)> {
    // The point's height along `axis` from `origin`, and its offset
    // across it.
    let split = |origin: Point3<T>, axis: Vec3<T>| {
        let a = axis / axis.norm();
        let w = q - origin;
        let h = w.dot(a);
        (a * h, w - a * h)
    };
    match *surface {
        geom::Surface::Sphere { center, radius, .. } => {
            let w = q - center;
            let n = w.norm();
            Some((n - radius, center + w * (radius / n), w / n))
        }
        geom::Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => {
            let (along, across) = split(origin, axis);
            let rho = across.norm();
            Some((
                rho - radius,
                origin + along + across * (radius / rho),
                across / rho,
            ))
        }
        geom::Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => {
            let (_, across) = split(center, axis);
            let core = center + across * (major_radius / across.norm());
            let v = q - core;
            let d = v.norm();
            Some((d - minor_radius, core + v * (minor_radius / d), v / d))
        }
        geom::Surface::Plane { origin, normal, .. } => {
            let n = normal / normal.norm();
            let d = (q - origin).dot(n);
            Some((d, q - n * d, n))
        }
        geom::Surface::Cone { .. } | geom::Surface::Nurbs(_) | geom::Surface::Approx(_) => None,
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
    rho: Reach<T>,
    (t0, t1): (T, T),
    band: Band,
) -> Option<Vec<(T, T)>> {
    // The radius the carrier's level sets part from a tangent curve over
    // (module docs): its reach, or on a plane the curve's own radius of
    // curvature. A bend with no finite bound sets no floor, and the walk
    // runs to its budget.
    let parting = match rho {
        Reach::Bounded(rho) => rho,
        Reach::Unbounded if geom_core::is_finite_length(bend) => T::one() / bend,
        Reach::Unbounded => T::zero(),
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
            Reach::Unbounded => Some(bend),
            Reach::Bounded(rho) => {
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

/// Whether the ball of `radius` about `foot`, on the carrier, holds no
/// point of `face` (module docs).
///
/// The face's boundary is read against the ball before the foot is
/// placed, by the plane's door on a plane and the curved one elsewhere. The ball's radius is at least `escalate`, so a vertex or
/// edge the placement could not tell from the foot lies inside it and
/// answers `false`; the placement is asked only of a foot every member
/// stands clear of.
fn ball_off_face<T: Decide + Bounds>(
    y: &Body<T>,
    face: FaceKey,
    foot: Point3<T>,
    radius: T,
    rho: Reach<T>,
    band: Band,
) -> Result<bool, BooleanError> {
    if let Reach::Bounded(rho) = rho
        && !matches!(
            decide("bool_touch_ball_in_reach", Margin::of(rho - radius), band),
            Ok(Sign::Positive)
        )
    {
        return Ok(false);
    }
    let f = crate::live::proven(&y.faces, face, EntityId::Face);
    for member in y.face_boundary_linked(face, f) {
        let clear_of_ball = match member {
            BoundaryMember::Isolated { point, .. } => clear((point - foot).norm(), radius, band),
            BoundaryMember::Edge { ek, .. } => edge_clear_of_ball(y, ek, foot, radius, band)?,
        };
        if !clear_of_ball {
            return Ok(false);
        }
    }
    let placed = match *y.face_surface_linked(face, f) {
        geom::Surface::Plane { normal, .. } => {
            super::contain::contfp_decided(y, face, normal, foot, band)
                .map(|(at, _)| CurvedPlacement::Trim(Some(at)))
        }
        _ => super::contain::curved_face_placement(y, face, foot, band),
    };
    match placed {
        Ok(CurvedPlacement::Trim(Some(FaceContainment::Out))) => Ok(true),
        Ok(_)
        | Err(
            ContainError::EmptyLoop(_)
            | ContainError::LoopUnreadable(_)
            | ContainError::Curved(_)
            | ContainError::RayExhausted
            | ContainError::Uncrossable(_),
        ) => Ok(false),
        // The foot is placed only where every boundary edge's whole
        // carrier (a segment, a circle, an ellipse's annulus) stands
        // clear of a ball of at least `escalate` about it, so the
        // boundary pass reads it definitely off every carrier, and an
        // arc's end is read only at a point on its carrier.
        Err(ContainError::ArcEndClearOfCorners(gaps)) => unreachable!(
            "face {face:?} placed a foot at an arc's end ({gaps:?}), though every boundary \
             carrier stands clear of the ball of radius {radius:?} about it"
        ),
        Err(ContainError::Escalated {
            decision,
            escalation,
            diag,
        }) => Err(BooleanError::Escalated {
            decision: BooleanDecision::Containment {
                decision,
                escalation,
            },
            diag,
        }),
        Err(ContainError::StaleFace(face)) => super::contain::driver_face_stale(face),
    }
}

/// Whether a lower bound `gap` on a distance from a ball's centre
/// definitely exceeds its `radius`.
fn clear<T: Decide>(gap: T, radius: T, band: Band) -> bool {
    matches!(
        decide("bool_touch_edge_clear", Margin::of(gap - radius), band),
        Ok(Sign::Positive)
    )
}

/// **Whether `edge` of `y` stays definitely farther than `radius` from
/// `at`.** Its certified box, padded by the sweep's pad, clear of the
/// ball's box answers at once. Otherwise a lower bound on the distance
/// is decided against `radius`, read on the edge's own carrier
/// ([`crate::splitting::containment::arc_distance_floor`]: a segment, a
/// conic's own arc, any other carrier's ball and then its arc's
/// pieces). A spline with no control points is not clear.
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] for an edge with no
/// certified carrier.
fn edge_clear_of_ball<T: Decide + Bounds>(
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
    let e = crate::live::proven(&y.edges, edge, EntityId::Edge);
    let curve = reduce::certified(y.get_curve_geom(e.curve))?;
    Ok(
        crate::splitting::containment::arc_distance_floor(curve.carrier(), curve.params(), at)
            .is_some_and(|floor| floor.clears(|gap| clear(gap, radius, band))),
    )
}

#[cfg(test)]
#[path = "carrier_touch_rows.rs"]
mod carrier_touch_rows;
