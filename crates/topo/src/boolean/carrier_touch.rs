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
//! second: `1/(κ − |dist(m)| − ℓ)` for the carrier's level sets (κ
//! below), plus the curve's own curvature. Near a tangency the first
//! clears nothing finer than the band, the second clears pieces in
//! proportion to their distance from the touch. The pieces that do not
//! clear, bisected down to a floor of `√(κ·escalate)/8`, run together
//! into clusters. A cluster reaching `ℓ` about `m` meets the carrier, if
//! at all, within `ℓ + |dist(m)|` of `m`'s foot on the carrier, and that
//! radius, widened by `escalate`, is its ball's.
//!
//! # Reading a ball against the face
//!
//! κ is the carrier's reach — the distance from it to its medial axis:
//! a sphere's or a cylinder's radius, a ring torus's `min(r, R − r)`. A
//! ball about a point of the carrier with radius below κ meets the
//! carrier in one disc. With no vertex or edge of the face inside the
//! ball, the face holds all of that disc or none of it, and the foot,
//! placed `Out` of the face by its door
//! ([`super::contain::curved_face_placement`]), says none. Every edge is
//! decided clear by a lower bound on its distance from the foot
//! ([`edge_clear_of_ball`]).
//!
//! A carrier with no closed-form distance and foot (a cone, a spline)
//! and a curve with no speed bound localize nothing, and the door stays.

use geom_brep::implicit::Conic;
use geom_core::{Band, Bounds, Decide, Margin, Point3, Real, Sign, Vec3};

use super::BooleanError;
use super::boxes;
use super::contain::{ContainError, CurvedPlacement, FaceContainment};
use super::{BooleanDecision, reduce};
use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, FaceKey};
use crate::live::BoundaryMember;
use crate::validate::decide;

/// The most pieces a span is cut into before the localization gives up:
/// a bound on the walk (D9), the same as the crossing layer's other
/// bisections (`circle_roots::SUBDIVISION_BUDGET`,
/// `splitting::spiric_arc::MAX_PIECES`). Away from a meeting the
/// second-order bound clears a piece in proportion to its distance from
/// it, so each level of the bisection keeps a few open pieces per
/// meeting, down to the floor `log₂(span / floor)` levels below. The
/// rows here (`pierce_tangent_off_face`) spend 70 to 150 pieces a
/// meeting across ε 1e-6 to 1e-12, so [`CLUSTER_BUDGET`] meetings fit
/// with room to spare.
const PIECE_BUDGET: usize = 4096;

/// The most clusters a span may localize into: the most meetings a span
/// of these curves has with these carriers. A conic against a torus is
/// degree 8 in the conic's half-angle tangent, and every other pair
/// here (a line against a torus, any of them against a quadric) is at
/// most 4.
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
    let (Some(kappa), Some((speed, bend))) = (reach(surface), speed_bound(carrier)) else {
        return Ok(false);
    };
    let Some(clusters) = clusters(surface, carrier, (speed, bend), kappa, (t0, t1), band) else {
        return Ok(false);
    };
    let half = T::from_f64(0.5);
    for (a, b) in clusters {
        let m = carrier.eval((a + b) * half);
        let Some((dist, foot, _)) = distance(surface, m) else {
            return Ok(false);
        };
        let radius = speed * (b - a).abs() * half + dist.abs() + T::from_f64(band.escalate());
        if !ball_off_face(y, face, foot, radius, kappa, band)? {
            return Ok(false);
        }
    }
    Ok(true)
}

/// The carrier's reach, κ in the module docs; `None` for a kind with no
/// distance here.
fn reach<T: Real>(surface: &geom::Surface<T>) -> Option<T> {
    match *surface {
        geom::Surface::Sphere { radius, .. } | geom::Surface::Cylinder { radius, .. } => {
            Some(radius)
        }
        geom::Surface::Torus {
            major_radius,
            minor_radius,
            ..
        } => Some(minor_radius.min(major_radius - minor_radius)),
        geom::Surface::Plane { .. }
        | geom::Surface::Cone { .. }
        | geom::Surface::Nurbs(_)
        | geom::Surface::Approx(_) => None,
    }
}

/// Bounds on the curve's speed, metres per unit of its parameter, and on
/// its curvature. A conic's come from [`Conic`], which reads the
/// semi-axes as magnitudes in either order (an ellipse stored minor
/// first is as legal as one stored major first: `geom::Curve3::Ellipse`).
fn speed_bound<T: Real>(carrier: &geom::Curve3<T>) -> Option<(T, T)> {
    match *carrier {
        geom::Curve3::Line { dir, .. } => Some((dir.norm(), T::zero())),
        _ => Conic::of(carrier).map(|c| (c.speed_hi(), c.curvature_hi())),
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
        geom::Surface::Plane { .. }
        | geom::Surface::Cone { .. }
        | geom::Surface::Nurbs(_)
        | geom::Surface::Approx(_) => None,
    }
}

/// The parameter runs of the span that may meet the carrier, in order
/// (module docs); `None` past the budgets.
fn clusters<T: Decide + Bounds>(
    surface: &geom::Surface<T>,
    carrier: &geom::Curve3<T>,
    (speed, bend): (T, T),
    kappa: T,
    (t0, t1): (T, T),
    band: Band,
) -> Option<Vec<(T, T)>> {
    // A curve tangent to the carrier stays within `escalate` of it over
    // a half-length of about `√(2κ·escalate)`, and no bound clears a
    // piece in there, so cutting finer only multiplies pieces. The floor
    // keeps a cluster's pieces an eighth of that, so a cluster's
    // half-length adds little to the ball it reads.
    let floor = (kappa * T::from_f64(band.escalate())).sqrt().hi() / 8.0;
    let half = T::from_f64(0.5);
    let mut out: Vec<(T, T)> = Vec::new();
    // Whether a cleared piece came after the last open one: the next open
    // piece starts a cluster.
    let mut broken = true;
    // Depth first, left before right, so the pieces leave in order.
    let mut stack = vec![(t0, t1)];
    let mut cut = 0;
    while let Some((a, b)) = stack.pop() {
        cut += 1;
        if cut > PIECE_BUDGET {
            return None;
        }
        let mid = (a + b) * half;
        let ell = speed * (b - a).abs() * half;
        let (dist, _, normal) = distance(surface, carrier.eval(mid))?;
        let tangent = carrier.deriv(mid);
        let slope = (normal.dot(tangent) / tangent.norm()).abs();
        let lipschitz = dist.abs() - ell;
        let axis_gap = kappa - dist.abs() - ell;
        let bound = if let Ok(Sign::Positive) =
            decide("bool_touch_piece_off_axis", Margin::of(axis_gap), band)
        {
            let curl = T::one() / axis_gap + bend;
            lipschitz.max(dist.abs() - ell * slope - ell.powi(2) * curl * half)
        } else {
            lipschitz
        };
        if let Ok(Sign::Positive) = decide("bool_touch_piece_clear", Margin::of(bound), band) {
            broken = true;
            continue;
        }
        if ell.hi() > floor {
            stack.push((mid, b));
            stack.push((a, mid));
            continue;
        }
        match out.last_mut() {
            Some(last) if !broken => last.1 = b,
            _ => {
                if out.len() == CLUSTER_BUDGET {
                    return None;
                }
                out.push((a, b));
            }
        }
        broken = false;
    }
    Some(out)
}

/// Whether the ball of `radius` about `foot`, on the carrier, holds no
/// point of `face` (module docs).
///
/// The face's boundary is read against the ball before the foot is
/// placed. The ball's radius is at least `escalate`, so a vertex or
/// edge the placement could not tell from the foot lies inside it and
/// answers `false`; the placement is asked only of a foot every member
/// stands clear of.
fn ball_off_face<T: Decide + Bounds>(
    y: &Body<T>,
    face: FaceKey,
    foot: Point3<T>,
    radius: T,
    kappa: T,
    band: Band,
) -> Result<bool, BooleanError> {
    if !matches!(
        decide("bool_touch_ball_in_reach", Margin::of(kappa - radius), band),
        Ok(Sign::Positive)
    ) {
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
    match super::contain::curved_face_placement(y, face, foot, band) {
        Ok(CurvedPlacement::Trim(Some(FaceContainment::Out))) => Ok(true),
        Ok(_)
        | Err(
            ContainError::EmptyLoop(_)
            | ContainError::LoopUnreadable(_)
            | ContainError::Curved(_)
            | ContainError::RayExhausted
            | ContainError::Uncrossable(_),
        ) => Ok(false),
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
/// is decided against `radius`, per carrier: a segment's own distance;
/// for an arc, the distance to its whole circle; for an elliptic arc,
/// the offset from its plane together with the in-plane gap to the
/// annulus between its semi-axes, where the whole ellipse lies. Any
/// other carrier is not clear.
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
    // The offset of `at` from the plane through `center` normal to
    // `axis`, and its distance from `center` within that plane.
    let split = |center: Point3<T>, axis: Vec3<T>| {
        let w = at - center;
        let n = axis / axis.norm();
        let h = w.dot(n);
        (h, (w - n * h).norm())
    };
    let gap = match *reduce::certified(y.get_curve_geom(e.curve))?.carrier() {
        geom::Curve3::Line { .. } => {
            let a = boxes::edge_end_point(y, edge, e.he_plus, "he_plus");
            let b = boxes::edge_end_point(y, edge, e.he_minus, "he_minus");
            let (d, w) = (b - a, at - a);
            let s = (w.dot(d) / d.norm_squared()).max(T::zero()).min(T::one());
            (w - d * s).norm()
        }
        geom::Curve3::Circle {
            center,
            axis,
            radius: r,
            ..
        } => {
            let (h, rho) = split(center, axis);
            ((rho - r).powi(2) + h.powi(2)).sqrt()
        }
        geom::Curve3::Ellipse {
            center,
            axis,
            major,
            minor,
            ..
        } => {
            // The annulus between the semi-axis magnitudes, whichever is
            // stored first.
            let (lo, hi) = (major.abs().min(minor.abs()), major.abs().max(minor.abs()));
            let (h, rho) = split(center, axis);
            let g = (rho - hi).max(lo - rho).max(T::zero());
            (g.powi(2) + h.powi(2)).sqrt()
        }
        _ => return Ok(false),
    };
    Ok(clear(gap, radius, band))
}

#[cfg(test)]
#[path = "carrier_touch_rows.rs"]
mod carrier_touch_rows;
