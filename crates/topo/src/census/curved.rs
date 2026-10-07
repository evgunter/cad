//! The confirm lanes for records on **curved edges**: a `(vertex,
//! edge)` record whose edge is not a line, and an edge-edge record with
//! a curved side. The join writes both (`boolean::edge_join`): a
//! vertex a record named that the join took, or a conventional vertex,
//! reads as the joined edge's interior, curved or not.
//!
//! A point is on a curved edge's interior when it is on the carrier
//! (its distance from the carrier's point at its recovered parameter
//! decided zero) and strictly inside the span at both ends, metered in
//! metres through the carrier's least speed — the questions
//! [`super::on_edge_interior`] asks of a line. Two edges' interiors
//! meet when one candidate point is on both: where a line or circle
//! meets a circle (through the circle's plane), and each edge's
//! midpoint, which an overlap holds. Other carriers refuse typed.

use geom_core::{Band, Decide, Margin, Point3, Real, Sign, Vec3};

use super::{CensusSubject, CensusUnsupportedCause, ValidationError, gap_is_zero};
use crate::body::Body;
use crate::entity::{EdgeKey, EntityId};

/// The carriers these lanes read: a line, or a circle.
#[derive(Clone, Copy)]
enum Carrier<T: Real> {
    Line {
        origin: Point3<T>,
        dir: Vec3<T>,
    },
    Circle {
        center: Point3<T>,
        normal: Vec3<T>,
        radius: T,
    },
}

/// `e`'s certified curve's carrier, span and least speed, or the typed
/// refusal pushed for a carrier outside these lanes.
fn read<T: Decide>(
    body: &Body<T>,
    e: EdgeKey,
    errors: &mut Vec<ValidationError>,
) -> Option<(geom::Curve3<T>, Carrier<T>, (T, T), T)> {
    let curve = body
        .edges
        .get(e)
        .and_then(|d| body.curves.get(d.curve))
        .and_then(crate::null::CurveGeom::certified)?;
    let carrier = curve.carrier().clone();
    let (read, speed) = match carrier {
        geom::Curve3::Line { origin, dir } => (Carrier::Line { origin, dir }, T::one()),
        geom::Curve3::Circle {
            center,
            axis,
            radius,
            ..
        } => (
            Carrier::Circle {
                center,
                normal: axis,
                radius,
            },
            radius,
        ),
        _ => {
            errors.push(ValidationError::CensusUnsupported {
                subject: CensusSubject::Entity(EntityId::Edge(e)),
                cause: CensusUnsupportedCause::ContactLane(
                    crate::contact::ContactRefusal::NotCertifiable {
                        what: "a record on a curved edge is certified on a circle edge only",
                    },
                ),
            });
            return None;
        }
    };
    Some((carrier, read, curve.params(), speed))
}

fn decided<T: Decide>(
    name: &'static str,
    margin: Margin<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Sign> {
    geom_core::k_stats::decide(name, margin, band)
        .map_err(|cause| errors.push(ValidationError::CensusEscalated { cause }))
        .ok()
}

/// Whether `q` lies on `e`'s interior (module docs). `None` where a
/// decision escalated or the carrier is outside these lanes (pushed).
pub(super) fn on_curved_interior<T: Decide>(
    body: &Body<T>,
    e: EdgeKey,
    q: Point3<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<bool> {
    let (carrier, _, (t0, t1), speed) = read(body, e, errors)?;
    // The midpoint anchor keeps the recovered parameter in span for a
    // span of at most one period (`Curve3::param_near`).
    let t = carrier.param_near(q, geom::mid_param(t0, t1))?;
    let gap = Margin::norm3(q - carrier.eval(t));
    if !gap_is_zero("pm_census_ve_curved_gap", gap, band, errors)? {
        return Some(false);
    }
    let mut interior = Some(true);
    for m in [t - t0, t1 - t] {
        match decided(
            "pm_census_ve_curved_span",
            Margin::levered(m, speed),
            band,
            errors,
        ) {
            Some(Sign::Positive) => {}
            Some(_) => interior = interior.map(|_| false),
            None => interior = None,
        }
    }
    interior
}

/// The points where the line `(o, d)` meets the circle `(c, n, r)`:
/// through the circle's plane where the line crosses it, in the plane
/// where it lies in it. `None` where a decision escalated (pushed).
fn line_circle<T: Decide>(
    (o, d): (Point3<T>, Vec3<T>),
    (c, n, r): (Point3<T>, Vec3<T>, T),
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Vec<Point3<T>>> {
    let nd = n.dot(d);
    match decided(
        "pm_census_ee_curved_plane",
        Margin::levered(nd.abs(), r),
        band,
        errors,
    )? {
        Sign::Zero => {
            let w = o - c;
            let b = w.dot(d);
            let disc = b * b - (w.dot(w) - r * r);
            match decided(
                "pm_census_ee_curved_chord",
                Margin::of(disc / r),
                band,
                errors,
            )? {
                Sign::Negative => Some(Vec::new()),
                _ => {
                    let h = disc.max(T::zero()).sqrt();
                    Some(vec![o + d * (h - b), o + d * (T::zero() - h - b)])
                }
            }
        }
        _ => Some(vec![o + d * (n.dot(c - o) / nd)]),
    }
}

/// The points where two circles' carriers can meet: on the line their
/// planes share, or for coplanar circles on their radical line.
fn circle_circle<T: Decide>(
    (c1, n1, r1): (Point3<T>, Vec3<T>, T),
    (c2, n2, r2): (Point3<T>, Vec3<T>, T),
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Vec<Point3<T>>> {
    let m = n1.cross(n2);
    let arm = r1.min(r2);
    if decided(
        "pm_census_ee_curved_planes",
        Margin::levered(m.norm(), arm),
        band,
        errors,
    )? == Sign::Positive
    {
        let (h1, h2) = (n1.dot(c1 - Point3::origin()), n2.dot(c2 - Point3::origin()));
        let p0 = Point3::origin() + (n2.cross(m) * h1 + m.cross(n1) * h2) / m.dot(m);
        return line_circle((p0, m.normalize()), (c1, n1, r1), band, errors);
    }
    let gap = Margin::of(n1.dot(c2 - c1).abs());
    if !gap_is_zero("pm_census_ee_curved_coplanar", gap, band, errors)? {
        return Some(Vec::new());
    }
    let v = c2 - c1;
    let dist = v.norm();
    if gap_is_zero(
        "pm_census_ee_curved_concentric",
        Margin::of(dist),
        band,
        errors,
    )? {
        return Some(Vec::new()); // concentric: an overlap's midpoints are the candidates
    }
    let u = v / dist;
    let a = (dist * dist + r1 * r1 - r2 * r2) / (dist + dist);
    line_circle((c1 + u * a, n1.cross(u)), (c1, n1, r1), band, errors)
}

/// Whether `a`'s and `b`'s interiors meet, one of them curved (module
/// docs). `None` where a decision escalated or a carrier is outside
/// these lanes (pushed).
pub(super) fn curved_interiors_meet<T: Decide>(
    body: &Body<T>,
    a: EdgeKey,
    b: EdgeKey,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<bool> {
    let (ca, ra, (a0, a1), _) = read(body, a, errors)?;
    let (cb, rb, (b0, b1), _) = read(body, b, errors)?;
    let mut points = vec![
        ca.eval(geom::mid_param(a0, a1)),
        cb.eval(geom::mid_param(b0, b1)),
    ];
    let crossings = match (ra, rb) {
        (
            Carrier::Line { origin, dir },
            Carrier::Circle {
                center,
                normal,
                radius,
            },
        )
        | (
            Carrier::Circle {
                center,
                normal,
                radius,
            },
            Carrier::Line { origin, dir },
        ) => line_circle((origin, dir), (center, normal, radius), band, errors),
        (
            Carrier::Circle {
                center: c1,
                normal: n1,
                radius: r1,
            },
            Carrier::Circle {
                center: c2,
                normal: n2,
                radius: r2,
            },
        ) => circle_circle((c1, n1, r1), (c2, n2, r2), band, errors),
        (Carrier::Line { .. }, Carrier::Line { .. }) => Some(Vec::new()),
    };
    points.extend(crossings?);
    // A candidate off either edge may escalate where another is on
    // both: its escalations stand only if no candidate decides.
    let mut held = Vec::new();
    for p in points {
        let on_a = on_curved_interior(body, a, p, band, &mut held);
        let on_b = on_curved_interior(body, b, p, band, &mut held);
        if (on_a, on_b) == (Some(true), Some(true)) {
            return Some(true);
        }
    }
    if held.is_empty() {
        return Some(false);
    }
    errors.extend(held);
    None
}
