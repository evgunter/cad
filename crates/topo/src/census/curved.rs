//! The confirm lanes for records on **curved edges**: a `(vertex,
//! edge)` record whose edge is not a line, and an edge-edge record with
//! a curved side. The join writes both (`boolean::edge_join`): a
//! vertex a record named that the join took, or a conventional vertex,
//! reads as the joined edge's interior, whatever its carrier.
//!
//! One reading serves every carrier kind. A point is on a curved edge's
//! interior when [`geom_brep::curve_pair::Span::contact`], the one
//! point-on-span reading, places it `On` the edge's span at its foot
//! ([`geom::Curve3::foot`], the one foot reading): on the carrier and
//! clear of both ends, or anywhere on a closed edge whose vertex is
//! conventional, which reads as a span with no ends.
//!
//! Two edges' interiors meet when [`geom_brep::curve_pair::meet`], the
//! one reading of two carrier spans' meetings, places a meeting on both
//! interiors (C5, `crates/geom-brep/README.md`). A closed edge whose
//! vertex is conventional reads as a span with no ends. A meeting at an
//! end of either edge is not one, and a refusal past the band's own
//! escalation is pushed typed on both edges. Tier 3′'s pass 5′ sweeps
//! with the same reading ([`interiors_meeting`]). A meeting at an end
//! takes its whole stretch, so the in-band points about a vertex event
//! are no meeting of their own.

use geom::Curve3;
use geom_brep::curve_pair::{Hit, MeetFault, Meeting, Span, meet};
use geom_brep::roots::Unsettled;
use geom_core::{Band, Decide, Margin, MarginDiag, Point3, Real, Sign};

use super::{CensusSubject, CensusUnsupportedCause, ValidationError};
use crate::body::Body;
use crate::entity::{EdgeKey, EntityId};
use crate::validate::Departure;

/// An edge's certified curve as these lanes read it.
struct Read<T: Real> {
    edge: EdgeKey,
    curve: Curve3<T>,
    span: (T, T),
}

fn refuse(edge: EdgeKey, what: &'static str, errors: &mut Vec<ValidationError>) {
    errors.push(ValidationError::CensusUnsupported {
        subject: CensusSubject::Entity(EntityId::Edge(edge)),
        cause: CensusUnsupportedCause::ContactLane(
            crate::contact::ContactRefusal::NotCertifiable { what },
        ),
    });
}

/// `margin`'s sign, or `None` with the escalation pushed.
pub(super) fn decided<T: Decide>(
    name: &'static str,
    margin: Margin<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Sign> {
    geom_core::k_stats::decide(name, margin, band)
        .map_err(|cause| errors.push(ValidationError::CensusEscalated { cause }))
        .ok()
}

/// `e`'s certified curve and span, or the typed refusal
/// pushed for an edge with none; whose speed bound meters no reach over
/// its span (a span within the band, or a bound that is poison — which
/// certification refuses to mint, so the refusal guards a door closed
/// upstream today); or
/// whose least speed meters no length over its span (a spline's
/// control-net floor can be zero: a single-span rational arc whose
/// derivative net turns past a right angle has none, so such an edge
/// refuses here whole).
fn read<T: Decide>(
    body: &Body<T>,
    edge: EdgeKey,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Read<T>> {
    let Some(curve) = body
        .edges
        .get(edge)
        .and_then(|d| body.curves.get(d.curve))
        .and_then(crate::null::CurveGeom::certified)
    else {
        refuse(edge, "a curved edge with no certified curve", errors);
        return None;
    };
    let carrier = curve.carrier().clone();
    let (lo, hi) = geom::carrier_speed::speed_range(&carrier);
    let span = curve.params();
    let span_reach = Margin::metered_sup((span.1 - span.0).abs(), hi);
    if decided("pm_census_curved_speed_hi", span_reach, band, errors)? != Sign::Positive {
        refuse(
            edge,
            "a curved edge whose speed bound meters no reach over its span",
            errors,
        );
        return None;
    }
    let length = Margin::metered((span.1 - span.0).abs(), lo);
    if decided("pm_census_curved_speed_floor", length, band, errors)? != Sign::Positive {
        refuse(
            edge,
            "a curved edge whose least speed meters no length over its span",
            errors,
        );
        return None;
    }
    Some(Read {
        edge,
        curve: carrier,
        span,
    })
}

/// `e`'s certified carrier, or `None` with the typed refusal pushed
/// ([`read`]).
pub(super) fn carrier<T: Decide>(
    body: &Body<T>,
    e: EdgeKey,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Curve3<T>> {
    read(body, e, band, errors).map(|r| r.curve)
}

/// **Whether `q` may rest on `e`'s span** — pass 2′'s skip, a consumer of
/// the point-on-span reading ([`Span::contact`] at `q`'s foot): `false`
/// only where it decides `q` off the span, so a vertex the sweep passes by
/// holds no touch the reading would have found; `true` where it places
/// `q` on the span or refuses, or for an edge with no certified curve,
/// leaving the question to the reading.
pub(super) fn may_rest_on<T: Decide>(body: &Body<T>, e: EdgeKey, q: Point3<T>, band: Band) -> bool {
    let Some(curve) = body
        .edges
        .get(e)
        .and_then(|d| body.curves.get(d.curve))
        .and_then(crate::null::CurveGeom::certified)
    else {
        return true;
    };
    let (t0, t1) = curve.params();
    !matches!(
        Span::new(curve.carrier(), curve.params()).contact(q, geom::mid_param(t0, t1), band),
        Ok(None)
    )
}

/// Whether `q` lies on `e`'s interior (module docs). `None` where a
/// decision escalated or the edge is not read (pushed).
pub(super) fn on_curved_interior<T: Decide>(
    body: &Body<T>,
    e: EdgeKey,
    q: Point3<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<bool> {
    on_curved_interior_reading(body, e, q, band, errors).map(|on| on.is_some())
}

/// [`on_curved_interior`], keeping the reading: `Some(Some(margin))` on
/// the interior, by the margin its gap decided Zero on; `Some(None)` not.
pub(super) fn on_curved_interior_reading<T: Decide>(
    body: &Body<T>,
    e: EdgeKey,
    q: Point3<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Option<MarginDiag>> {
    let r = read(body, e, band, errors)?;
    on_interior_reading(body, &r, q, band, errors)
}

/// [`on_curved_interior_reading`] on an edge already read: `q` read
/// against the edge's span ([`span`]) at its foot by
/// [`geom_brep::curve_pair::Span::contact`], the one point-on-span
/// reading. A refusal is pushed typed: an escalation as itself, a
/// spline's walk past its budget or a spline with no speed bound as the
/// edge's.
fn on_interior_reading<T: Decide>(
    body: &Body<T>,
    r: &Read<T>,
    q: Point3<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Option<MarginDiag>> {
    on_interior_at(body, r, q, band, errors).map(|on| on.map(|(_, margin)| margin))
}

/// **`e`'s tangent where `q` lies on its interior**, along its plus
/// half and scaled to the edge's reach from `q`
/// ([`geom_brep::edge_reach_bounds`]): the corner it bounds is read by
/// the wedge, which acts on every sign, so its lever is the region's own
/// measure from where it is read, and where that is known only from both
/// sides (an ellipse, spiric or spline edge, whose bound from above can
/// pass it many times over) the tangent carries both and the wedge acts
/// only on a sign both read. `Some(None)` where `q` is not on the
/// interior; `None` where a decision escalated or the edge is not read
/// (pushed).
pub(super) fn tangent_on_interior<T: Decide>(
    body: &Body<T>,
    e: EdgeKey,
    q: Point3<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Option<Departure<T>>> {
    let r = read(body, e, band, errors)?;
    let (t0, t1) = r.span;
    let lever = geom_brep::edge_reach_bounds(&r.curve, t0, t1, q);
    Some(
        on_interior_at(body, &r, q, band, errors)?
            .map(|(t, _)| Departure::along(r.curve.deriv(t).normalize(), lever)),
    )
}

/// [`on_interior_reading`], keeping the parameter `q`'s foot is at.
fn on_interior_at<T: Decide>(
    body: &Body<T>,
    r: &Read<T>,
    q: Point3<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Option<(T, MarginDiag)>> {
    let span = span(body, r);
    match span.contact(q, geom::mid_param(r.span.0, r.span.1), band) {
        Ok(Some(Hit::On { t, margin })) => Some(Some((t, margin))),
        Ok(Some(Hit::End(_)) | None) => Some(None),
        Err(geom::CarrierRefusal::Escalated(cause) | geom::CarrierRefusal::Straddle(cause)) => {
            errors.push(ValidationError::CensusEscalated { cause });
            None
        }
        Err(geom::CarrierRefusal::Exhausted) => {
            refuse(
                r.edge,
                "the census's reading of a point on a spline edge ran past its budget",
                errors,
            );
            None
        }
        Err(geom::CarrierRefusal::Unbounded) => {
            refuse(
                r.edge,
                "a spline edge whose speed bound is zero or poison",
                errors,
            );
            None
        }
    }
}

/// Whether `a`'s and `b`'s interiors meet, one of them curved (module
/// docs). `None` where a decision escalated or an edge is not read
/// (pushed).
pub(super) fn curved_interiors_meet<T: Decide>(
    body: &Body<T>,
    a: EdgeKey,
    b: EdgeKey,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<bool> {
    let ra = read(body, a, band, errors)?;
    let rb = read(body, b, band, errors)?;
    reads_meet(body, &ra, &rb, band, errors)
}

/// **An edge's span as these lanes read it**: its certified curve over
/// its span, with no ends where it is closed on a conventional vertex
/// ([`span`]). The ray's edge read (`boolean::solid_contain`) reads a
/// point against an edge through it, so the census and the ray read one
/// span.
pub(crate) struct EdgeSpan<T: Real> {
    read: Read<T>,
    endless: bool,
}

impl<T: Decide> EdgeSpan<T> {
    /// The edge's carrier.
    pub(crate) fn carrier(&self) -> &Curve3<T> {
        &self.read.curve
    }

    /// Its span's middle parameter.
    pub(crate) fn middle(&self) -> T {
        geom::mid_param(self.read.span.0, self.read.span.1)
    }

    /// Its span, read as the census reads it.
    pub(crate) fn span(&self) -> Span<'_, T> {
        let span = Span::new(&self.read.curve, self.read.span);
        if self.endless { span.endless() } else { span }
    }
}

/// `edge`'s [`EdgeSpan`], `None` where the census's read of it refuses
/// (no certified curve, or a speed bound that meters no length).
pub(crate) fn edge_span<T: Decide>(
    body: &Body<T>,
    edge: EdgeKey,
    band: Band,
) -> Option<EdgeSpan<T>> {
    let read = read(body, edge, band, &mut Vec::new())?;
    let endless = on_conventional_vertex(body, &read);
    Some(EdgeSpan { read, endless })
}

/// `r`'s span, with no ends where its edge is closed on a conventional
/// vertex: that vertex has no identity of its own, so its point is the
/// edge's interior too.
fn span<'r, T: Decide>(body: &Body<T>, r: &'r Read<T>) -> Span<'r, T> {
    let span = Span::new(&r.curve, r.span);
    if on_conventional_vertex(body, r) {
        span.endless()
    } else {
        span
    }
}

/// Whether `r`'s edge starts at a conventional vertex: a closed edge's
/// vertex with no identity of its own, so its point is the edge's
/// interior too.
fn on_conventional_vertex<T: Decide>(body: &Body<T>, r: &Read<T>) -> bool {
    body.edges
        .get(r.edge)
        .and_then(|d| body.half_edges.get(d.he_plus))
        .is_some_and(|h| crate::boolean::is_conventional_vertex(body, h.start))
}

/// [`curved_interiors_meet`] on two edges already read: whether
/// [`geom_brep::curve_pair::meet`] places a meeting on both interiors
/// (module docs). A meeting at an end of either is not one.
fn reads_meet<T: Decide>(
    body: &Body<T>,
    ra: &Read<T>,
    rb: &Read<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<bool> {
    interior_meeting(body, ra, rb, band, errors).map(|m| m.is_some())
}

/// A meeting on two edges' interiors, as pass 5′ reports it.
pub(super) struct Interior<T: Real> {
    /// The meeting's point.
    pub(super) point: Point3<T>,
    /// The margin the first edge's reading put it on that edge by.
    pub(super) margin: MarginDiag,
    /// Whether the carriers are one curve: the edges then overlap
    /// along a run of it, rather than crossing or touching at a point.
    pub(super) overlap: bool,
}

/// **The first meeting [`geom_brep::curve_pair::meet`] places on both
/// of `a`'s and `b`'s interiors** — pass 5′'s reading
/// (`census::sweep_curved_edge_edge`). `Some(None)` where none is placed;
/// `None` where a decision escalated or an edge is not read (pushed).
pub(super) fn interiors_meeting<T: Decide>(
    body: &Body<T>,
    a: EdgeKey,
    b: EdgeKey,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Option<Interior<T>>> {
    let ra = read(body, a, band, errors)?;
    let rb = read(body, b, band, errors)?;
    interior_meeting(body, &ra, &rb, band, errors)
}

/// The first meeting [`meet`] places on both
/// interiors, a decided one standing whatever refusal followed it; a
/// refusal with none such pushed typed on both edges.
fn interior_meeting<T: Decide>(
    body: &Body<T>,
    ra: &Read<T>,
    rb: &Read<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Option<Interior<T>>> {
    let interior = |points: &[Meeting<T>], shared: Option<_>| {
        points.iter().find_map(|m| match (m.on_a, m.on_b) {
            (Hit::On { margin, .. }, Hit::On { .. }) => Some(Interior {
                point: m.point,
                margin,
                overlap: shared.is_some(),
            }),
            _ => None,
        })
    };
    let refusal = match meet(&span(body, ra), &span(body, rb), band) {
        Ok(met) => return Some(interior(&met.points, met.shared)),
        Err(refusal) => match interior(&refusal.found, refusal.shared) {
            Some(found) => return Some(Some(found)),
            None => refusal,
        },
    };
    let what = match refusal.fault {
        MeetFault::Escalated(cause) | MeetFault::Touch(cause) => {
            errors.push(ValidationError::CensusEscalated { cause });
            return None;
        }
        MeetFault::Unsettled(Unsettled::Exhausted) => {
            "the meeting reading of two curved edges ran past its budget"
        }
        MeetFault::Unsettled(_) => {
            "the meeting reading of two curved edges could not settle a count"
        }
        MeetFault::Unsupported { pose } => pose,
        MeetFault::Desync => "the meeting reading of two curved edges misread a carrier's kind",
        MeetFault::Contradiction { readings } => readings,
    };
    refuse(ra.edge, what, errors);
    refuse(rb.edge, what, errors);
    None
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use core::f64::consts::TAU;

    use geom::Curve3;
    use geom_core::{Band, Point3, Tol, Vec3};

    use super::{Read, curved_interiors_meet, interior_meeting, reads_meet};
    use crate::EdgeCurveSpec;
    use crate::body::Body;
    use crate::construction::MevSite;
    use crate::entity::EdgeKey;

    /// Arcs of one unit circle, from `t0` to `t1` degrees, each its own
    /// solid in one arena.
    fn arcs(spans: [(f64, f64); 2]) -> (Body<f64>, [EdgeKey; 2]) {
        let tol = Tol::witness();
        let c = geom::Curve3::Circle {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        };
        let mut body = Body::<f64>::new();
        let edges = spans.map(|(t0, t1)| {
            let (t0, t1) = (t0.to_radians(), t1.to_radians());
            let born = body.mvfs(c.eval(t0), true).unwrap();
            body.mev(
                MevSite::Lone {
                    r#loop: born.r#loop,
                },
                c.eval(t1),
                EdgeCurveSpec::arc_of_circle(c.clone(), t0, t1).unwrap(),
                tol,
            )
            .unwrap()
            .edge
        });
        (body, edges)
    }

    /// **Two arcs of one circle meet where they overlap**, though the
    /// overlap holds neither arc's midpoint; arcs a gap apart do not.
    #[test]
    fn overlapping_arcs_meet_off_both_midpoints() {
        let band = Band::linear(Tol::witness()).unwrap();
        for (spans, meet) in [
            ([(0.0, 100.0), (90.0, 190.0)], true),
            ([(90.0, 190.0), (0.0, 100.0)], true),
            ([(0.0, 80.0), (90.0, 190.0)], false),
        ] {
            let (body, [a, b]) = arcs(spans);
            let mut errors = Vec::new();
            assert_eq!(
                curved_interiors_meet(&body, a, b, band, &mut errors),
                Some(meet),
                "{spans:?}: {errors:?}"
            );
        }
    }

    fn read(curve: Curve3<f64>, span: (f64, f64)) -> Read<f64> {
        Read {
            edge: EdgeKey::default(),
            curve,
            span,
        }
    }

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn ellipse(major: f64, minor: f64, u_ref: Vec3<f64>) -> Curve3<f64> {
        Curve3::Ellipse {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::unit_z(),
            semi_u: major,
            semi_v: minor,
            u_ref,
        }
    }

    fn circle(center: Point3<f64>, radius: f64) -> Curve3<f64> {
        Curve3::Circle {
            center,
            axis: Vec3::unit_z(),
            radius,
            u_ref: Vec3::unit_x(),
        }
    }

    fn line(origin: Point3<f64>, dir: Vec3<f64>) -> Curve3<f64> {
        Curve3::Line { origin, dir }
    }

    /// The rational quadratic arc from `(0, 0)` to `(2, 0)` through
    /// `(1, 0.25)`-ish, in `z = 0`.
    fn spline() -> Curve3<f64> {
        Curve3::Nurbs(std::sync::Arc::new(
            geom::NurbsCurve3::new(
                geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2)
                    .unwrap(),
                vec![
                    Point3::new(0.0, 0.0, 0.0),
                    Point3::new(1.0, 0.5, 0.0),
                    Point3::new(2.0, 0.0, 0.0),
                ],
                vec![1.0, 2.0, 1.0],
            )
            .unwrap(),
        ))
    }

    /// Whether the two carriers' spans meet, read without a body: no
    /// vertex of theirs is conventional.
    fn meet(a: &Read<f64>, b: &Read<f64>) -> (Option<bool>, Vec<crate::ValidationError>) {
        let mut errors = Vec::new();
        let met = reads_meet(&Body::new(), a, b, band(), &mut errors);
        (met, errors)
    }

    /// **The reading answers a meeting on every kind**, and clears
    /// carriers that keep apart: a line across a spiric's oval meets it;
    /// a line through a spline's point meets it there; an ellipse and a
    /// circle a metre above it nowhere; two coplanar ellipses at right
    /// angles meet near `(a, a)`, `a² = 4/5`. Every pair is decided, with
    /// nothing pushed.
    #[test]
    fn the_reading_answers_a_meeting_on_every_kind() {
        let a = 0.8_f64.sqrt();
        let theta = (a / 2.0).acos();
        let (r, d) = (0.5, 0.3);
        let on = spline().eval(0.5);
        let cases = [
            (
                "ellipse × ellipse",
                read(
                    ellipse(2.0, 1.0, Vec3::unit_x()),
                    (theta - 0.3, theta + 0.1),
                ),
                read(
                    ellipse(2.0, 1.0, Vec3::unit_y()),
                    (-theta - 0.2, -theta + 0.3),
                ),
                Some(true),
            ),
            (
                "spiric × line",
                read(
                    Curve3::Spiric {
                        center: Point3::new(0.0, 0.0, 0.0),
                        axis: Vec3::unit_z(),
                        u_ref: Vec3::unit_x(),
                        major_radius: 2.0,
                        minor_radius: r,
                        offset: d,
                    },
                    (-3.0, 0.3),
                ),
                read(line(Point3::new(d, 2.0, -1.0), Vec3::unit_z()), (0.0, 2.0)),
                Some(true),
            ),
            (
                "spline × line",
                read(spline(), (0.0, 1.0)),
                read(line(on - Vec3::unit_z() * 0.3, Vec3::unit_z()), (0.0, 1.0)),
                Some(true),
            ),
            (
                "ellipse × a circle above it",
                read(ellipse(2.0, 1.0, Vec3::unit_x()), (0.0, TAU)),
                read(circle(Point3::new(0.0, 0.0, 1.0), 1.5), (0.0, TAU)),
                Some(false),
            ),
        ];
        for (what, x, y, want) in cases {
            let (met, errors) = meet(&x, &y);
            assert_eq!(met, want, "{what}: {errors:?}");
            assert!(
                errors.is_empty(),
                "{what}: decided, nothing pushed: {errors:?}"
            );
        }
    }

    /// **A tangency and a shallow crossing are decided.** A line tangent
    /// to a circle, two coplanar circles touching from outside, and a
    /// line crossing a circle at 0.01 rad each meet; the line moved
    /// `1e-4` off the circle does not. An arc ending where a circle
    /// touches it from outside meets it at the arc's end alone: the arc
    /// stays within ε of the circle over its last few hundredths of a
    /// millimetre, but that stretch's one meeting is the touch at the end,
    /// and it takes the whole stretch. Red when a point of that stretch is
    /// read as an interior meeting of its own.
    #[test]
    fn a_tangency_and_a_shallow_crossing_are_decided() {
        let o = Point3::new(0.0, 0.0, 0.0);
        let unit = (0.3, 2.9);
        let to_top = (0.3, core::f64::consts::FRAC_PI_2);
        let shallow = Vec3::new(0.01_f64.cos(), 0.01_f64.sin(), 0.0);
        let cases = [
            (
                "a line tangent to a circle",
                unit,
                read(
                    line(Point3::new(-1.0, 1.0, 0.0), Vec3::unit_x()),
                    (0.0, 2.3),
                ),
                Some(true),
            ),
            (
                "a circle touching it from outside",
                unit,
                read(circle(Point3::new(0.0, 2.0, 0.0), 1.0), (-2.9, -0.3)),
                Some(true),
            ),
            (
                "a line crossing it at 0.01 rad",
                unit,
                read(
                    line(Point3::new(0.0, 1.0, 0.0) - shallow * 0.2, shallow),
                    (0.0, 0.4),
                ),
                Some(true),
            ),
            (
                "a circle touching it from outside at the end of an arc of it",
                to_top,
                read(circle(Point3::new(0.0, 2.0, 0.0), 1.0), (-2.9, -0.3)),
                Some(false),
            ),
            (
                "a line 1e-4 off it",
                unit,
                read(
                    line(Point3::new(-1.0, 1.0 + 1e-4, 0.0), Vec3::unit_x()),
                    (0.0, 2.3),
                ),
                Some(false),
            ),
        ];
        for (what, arc, other, want) in cases {
            let arc = read(circle(o, 1.0), arc);
            assert_eq!(meet(&arc, &other), (want, vec![]), "{what}");
        }
    }

    /// **A point past the band from an ellipse arc's end is inside
    /// it**, though its parameter distance metered by the least speed
    /// reads zero. The ellipse `1 × 0.05` runs at speed 1 from `π/2`, so
    /// a point 15ε along it is 15ε from the end, past the band's
    /// escalation; its parameter gap times the minor semi-axis is
    /// 0.75ε. Red when that metered gap decides the end.
    #[test]
    fn a_point_past_the_band_from_an_ellipse_end_is_inside() {
        let eps = Tol::witness().eps();
        assert!(band().escalate() < 15.0 * eps, "the row needs K < 15");
        let t0 = core::f64::consts::FRAC_PI_2;
        let r = read(ellipse(1.0, 0.05, Vec3::unit_x()), (t0, t0 + 1.0));
        let t = t0 + 15.0 * eps;
        let q = r.curve.eval(t);
        let inside = super::span(&Body::new(), &r)
            .contact(q, t, band())
            .map(|c| matches!(c, Some(geom_brep::curve_pair::Hit::On { .. })));
        assert_eq!(inside.ok(), Some(true));
    }

    /// **A point past the band outside an ellipse arc's start is not on
    /// it, and nor is a line crossing there.** The mirror of
    /// [`a_point_past_the_band_from_an_ellipse_end_is_inside`]: the point
    /// 15ε before `π/2` on the `1 × 0.05` ellipse reads a parameter gap
    /// of 0.75ε at the minor semi-axis, zero, and its distance from the
    /// end's point is 15ε, past the band, but it lies outside the span.
    /// A line crossing the `1 × 0.02` ellipse 30ε before the start reads
    /// the same way (0.6ε metered, 30ε away), with the overlap
    /// candidates halfway to the start 15ε out, past the band too. Red
    /// when the distance from the end's point decides without the side.
    #[test]
    fn a_point_past_the_band_outside_an_ellipse_start_is_off_it() {
        let eps = Tol::witness().eps();
        let t0 = core::f64::consts::FRAC_PI_2;
        let r = read(ellipse(1.0, 0.05, Vec3::unit_x()), (t0, t0 + 1.0));
        let q = r.curve.eval(t0 - 15.0 * eps);
        let mut errors = Vec::new();
        let on = super::on_interior_reading(&Body::new(), &r, q, band(), &mut errors)
            .map(|on| on.is_some());
        assert_eq!((on, errors), (Some(false), vec![]), "the vertex reading");
        let thin = read(ellipse(1.0, 0.02, Vec3::unit_x()), (t0, t0 + 1.0));
        let (q, d) = thin.curve.ders1(t0 - 30.0 * eps);
        let across = Vec3::unit_z().cross(d).normalize();
        let crossing = read(line(q - across * 0.1, across), (0.0, 0.2));
        assert_eq!(
            meet(&thin, &crossing),
            (Some(false), vec![]),
            "the crossing reading"
        );
    }

    /// **Near a fast end, a point's side and distance decide where the
    /// metered span reads in its band.** On the `1 × 0.05` ellipse from
    /// `π/2`, a point `k·ε` along the curve from the end, `k` from 20 to
    /// 200, meters `0.05·k·ε` by the least speed: from ε to 10ε, the
    /// span decision's band. Its side (speed 1 there) and its distance
    /// from the end's point are `k·ε`, past the band: inside it is on
    /// the arc, outside it is not, and a line crossing the ellipse there
    /// meets the arc or does not alike. Outside, the crossing starts at
    /// 50ε: nearer, the candidate halfway to the end lies within the
    /// band of it and escalates. Red when the span decision's band
    /// escalates rather than falling through to the side.
    #[test]
    fn near_a_fast_end_the_side_decides_where_the_span_reads_in_band() {
        let eps = Tol::witness().eps();
        assert!(band().escalate() < 20.0 * eps, "the row needs K < 20");
        let t0 = core::f64::consts::FRAC_PI_2;
        let r = read(ellipse(1.0, 0.05, Vec3::unit_x()), (t0, t0 + 1.0));
        for k in [20.0, 50.0, 100.0, 200.0] {
            for (sign, inside) in [(1.0, true), (-1.0, false)] {
                let (q, d) = r.curve.ders1(t0 + sign * k * eps);
                let mut errors = Vec::new();
                let on = super::on_interior_reading(&Body::new(), &r, q, band(), &mut errors)
                    .map(|on| on.is_some());
                assert_eq!(
                    (on, errors),
                    (Some(inside), vec![]),
                    "the vertex {sign}·{k}ε"
                );
                if !inside && k < 50.0 {
                    continue;
                }
                let across = Vec3::unit_z().cross(d).normalize();
                let crossing = read(line(q - across * 0.1, across), (0.0, 0.2));
                assert_eq!(
                    meet(&r, &crossing),
                    (Some(inside), vec![]),
                    "the crossing {sign}·{k}ε"
                );
            }
        }
    }

    /// **Near-concentric arcs are decided**: two coplanar concentric arcs
    /// `Δr` apart meet where `Δr` is half the band's coincidence threshold
    /// (one curve, whose overlap meets), and nowhere at thirty times the
    /// escalation band and at `1e-3` (two circles on one centre, their
    /// radii definitely apart): the circle pair's closed form, as
    /// `geom_brep::curve_pair`'s `near_concentric_arcs_are_decided` pins.
    #[test]
    fn near_concentric_arcs_are_decided() {
        let o = Point3::new(0.0, 0.0, 0.0);
        let pair = |dr: f64| {
            meet(
                &read(circle(o, 1.0), (0.3, 2.9)),
                &read(circle(o, 1.0 + dr), (0.5, 3.1)),
            )
        };
        assert_eq!(
            pair(band().zero() / 2.0),
            (Some(true), vec![]),
            "coincident"
        );
        for dr in [30.0 * band().escalate(), 1e-3] {
            assert_eq!(pair(dr), (Some(false), vec![]), "{dr:e} apart");
        }
    }

    /// **A point within the band of a long thin ellipse is on it.** The
    /// ellipse `1 × 0.02` and a point half an ε off it along its normal:
    /// `param_near`'s eccentric anomaly reads it about 25 times farther
    /// off, past the band, and its polished foot reads it on. Red when
    /// the foot is not polished.
    #[test]
    fn a_point_in_the_band_of_a_thin_ellipse_is_on_it() {
        let eps = Tol::witness().eps();
        let r = read(ellipse(1.0, 0.02, Vec3::unit_x()), (0.0, 1.0));
        let (p, d) = r.curve.ders1(0.3);
        let normal = Vec3::unit_z().cross(d).normalize();
        let q = p + normal * (0.5 * eps);
        let mut errors = Vec::new();
        let on = super::on_interior_reading(&Body::new(), &r, q, band(), &mut errors)
            .map(|on| on.is_some());
        assert_eq!((on, errors), (Some(true), vec![]));
    }

    /// **Two circles kissing in their plane meet at the point**: the
    /// unit circle's arc over ±30° against an arc over 150..210° of a
    /// unit circle outside it or a half-unit one inside it, each touching
    /// it at (1, 0, 0) (the point touch C3 names for two edges).
    /// Pushed 0.1 m apart they do not meet; pushed 3ε, whether they
    /// meet is undecided and escalates. A circle inside it whose centre
    /// is `1000ε` off and whose radius is `1000ε + g` short touches it
    /// within the band at `g = ε/2` and is apart at `g = 30ε`: the gap is
    /// read in metres, not magnified by the centres' offset.
    #[test]
    fn kissing_circles_meet_at_their_touch_point() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let eps = tol.eps();
        for (what, (cx, r, t), meet) in [
            ("outside", (2.0, 1.0, 180.0), Some(true)),
            ("inside", (0.5, 0.5, 0.0), Some(true)),
            ("outside, 0.1 m apart", (2.1, 1.0, 180.0), Some(false)),
            ("inside, 0.1 m apart", (0.4, 0.5, 0.0), Some(false)),
            ("outside, 3ε apart", (2.0 + 3.0 * eps, 1.0, 180.0), None),
            (
                "inside, nearly equal radii, ε/2 apart",
                (1e3 * eps, 1.0 - 1e3 * eps - eps / 2.0, 0.0),
                Some(true),
            ),
            (
                "inside, nearly equal radii, 30ε apart",
                (1e3 * eps, 1.0 - 1e3 * eps - 30.0 * eps, 0.0),
                Some(false),
            ),
        ] {
            let mut body = Body::<f64>::new();
            let mut arc = |center: Point3<f64>, radius: f64, (t0, t1): (f64, f64)| {
                let c = geom::Curve3::Circle {
                    center,
                    axis: Vec3::unit_z(),
                    radius,
                    u_ref: Vec3::unit_x(),
                };
                let (t0, t1) = (t0.to_radians(), t1.to_radians());
                let born = body.mvfs(c.eval(t0), true).unwrap();
                body.mev(
                    MevSite::Lone {
                        r#loop: born.r#loop,
                    },
                    c.eval(t1),
                    EdgeCurveSpec::arc_of_circle(c.clone(), t0, t1).unwrap(),
                    tol,
                )
                .unwrap()
                .edge
            };
            let a = arc(Point3::new(0.0, 0.0, 0.0), 1.0, (-30.0, 30.0));
            let b = arc(Point3::new(cx, 0.0, 0.0), r, (t - 30.0, t + 30.0));
            let mut errors = Vec::new();
            assert_eq!(
                curved_interiors_meet(&body, a, b, band, &mut errors),
                meet,
                "{what}: {errors:?}"
            );
            assert_eq!(
                meet.is_none(),
                !errors.is_empty(),
                "{what}: an escalation is pushed exactly where undecided: {errors:?}"
            );
        }
    }

    /// **An arc ending tangent on a line's interior meets it at no
    /// interior point**, nor does a line leaving the arc's end along its
    /// tangent (the slot's joint). Near the arc's end the two stay within
    /// the band of each other a short way, inside both; that stretch's one
    /// meeting is at the arc's end, a vertex on the line, which pass 2
    /// answers by its own record. A line crossing the arc nearby is a
    /// meeting all the same. Red when a point of that stretch is read as a
    /// meeting of its own, which would owe an edge-edge record beside the
    /// `(vertex, edge)` one.
    #[test]
    fn a_tangent_touch_at_an_arcs_end_holds_no_interior_meeting_for_pass_five_prime() {
        let half_pi = core::f64::consts::FRAC_PI_2;
        let arc = read(circle(Point3::new(0.0, 0.0, 0.0), 1.0), (0.0, half_pi));
        let pass = |other: &Read<f64>| {
            let mut errors = Vec::new();
            let met = interior_meeting(&Body::new(), &arc, other, band(), &mut errors)
                .map(|m| m.map(|m| m.overlap));
            (met, errors)
        };
        let joint = read(
            line(Point3::new(0.0, 1.0, 0.0), -Vec3::unit_x()),
            (0.0, 1.0),
        );
        assert_eq!(pass(&joint), (Some(None), vec![]), "the joint");
        let through = read(
            line(Point3::new(1.0, 1.0, 0.0), -Vec3::unit_x()),
            (0.0, 2.0),
        );
        assert_eq!(
            pass(&through),
            (Some(None), vec![]),
            "the arc's end on the line"
        );
        let radial = arc.curve.eval(0.5) - Point3::new(0.0, 0.0, 0.0);
        let across = read(
            line(Point3::new(0.0, 0.0, 0.0) + radial * 0.5, radial),
            (0.0, 1.0),
        );
        assert_eq!(pass(&across), (Some(Some(false)), vec![]), "a crossing");
    }
}
