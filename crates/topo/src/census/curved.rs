//! The confirm lanes for records on **curved edges**: a `(vertex,
//! edge)` record whose edge is not a line, and an edge-edge record with
//! a curved side. The join writes both (`boolean::edge_join`): a
//! vertex a record named that the join took, or a conventional vertex,
//! reads as the joined edge's interior, whatever its carrier.
//!
//! One reading serves every carrier kind. A point is on a curved edge's
//! interior when it is on the carrier (its distance from its foot
//! decided zero) and strictly inside the span at both ends, or anywhere
//! on a closed edge whose vertex is conventional — the questions
//! [`super::on_edge_interior`] asks of a line. The foot is polished by
//! Gauss–Newton from [`geom::Curve3::param_near`]'s parameter where the
//! kind inverts in closed form (which is no foot off an ellipse), and
//! otherwise from each piece of the span the foot search keeps. An end
//! is cleared when the parameter's distance from it, metered by the
//! carrier's least speed ([`geom::carrier_speed::speed_range`]), decides
//! positive, and reached when it decides negative; between the two the
//! same distance metered by the greatest speed decides the side, and on
//! the inner side the point's distance from the end's point decides.
//!
//! Two edges' interiors meet when [`geom_brep::curve_pair::meet`], the
//! one reading of two carrier spans' meetings, places a meeting on both
//! interiors (C5, `crates/geom-brep/README.md`). A closed edge whose
//! vertex is conventional reads as a span with no ends. A meeting at an
//! end of either edge is not one, and a refusal past the band's own
//! escalation is pushed typed on both edges.
//!
//! **The foot search** is [`geom_core::bisect`]'s walk over the span. A
//! piece reaches at most `ℓ` metres from its middle point, `ℓ` its
//! half-span times the carrier's greatest speed; a piece whose middle
//! point lies farther than that from the point is cleared, and one whose
//! reach is within the band is kept and its foot polished. A walk past
//! [`geom_core::bisect::PIECE_BUDGET`] refuses typed.

use geom::Curve3;
use geom_brep::curve_pair::{Hit, MeetFault, Meeting, Span, meet};
use geom_brep::roots::Unsettled;
use geom_core::{Band, Decide, InfSpeed, Margin, MarginDiag, Point3, Real, Sign, SupSpeed};

use super::{
    CensusSubject, CensusUnsupportedCause, ValidationError, gap_is_zero, gap_zero_reading,
};
use crate::body::Body;
use crate::entity::{EdgeKey, EntityId};
use geom_core::bisect::{Step, bisect};

/// The Gauss–Newton steps a candidate is polished by.
const POLISH_STEPS: usize = 4;

/// An edge's certified curve as these lanes read it.
struct Read<T: Real> {
    edge: EdgeKey,
    curve: Curve3<T>,
    span: (T, T),
    lo: InfSpeed<T>,
    hi: SupSpeed<T>,
}

/// A run `(a, b)` of a span, cut `depth` times.
#[derive(Clone, Copy)]
struct Piece<T> {
    a: T,
    b: T,
    depth: u32,
}

impl<T: Real> Piece<T> {
    fn whole((a, b): (T, T)) -> Self {
        Self { a, b, depth: 0 }
    }

    fn mid(self) -> T {
        geom::mid_param(self.a, self.b)
    }

    /// The most metres a point of the piece lies from its middle point.
    fn reach(self, hi: SupSpeed<T>) -> T {
        geom_core::bisect::reach(hi, (self.a, self.b))
    }

    /// The two halves, in order.
    fn halves(self) -> [Self; 2] {
        let (m, depth) = (self.mid(), self.depth + 1);
        [
            Self {
                a: self.a,
                b: m,
                depth,
            },
            Self {
                a: m,
                b: self.b,
                depth,
            },
        ]
    }
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

/// Whether a point is in two cells, each reading pushing what stopped
/// it: `Some(false)` where either reads it out, whatever the other
/// pushed (a decided answer drops the other's escalations); `Some(true)`
/// where both read it in; `None` otherwise, with what both pushed.
pub(super) fn in_both(
    a: impl FnOnce(&mut Vec<ValidationError>) -> Option<bool>,
    b: impl FnOnce(&mut Vec<ValidationError>) -> Option<bool>,
    errors: &mut Vec<ValidationError>,
) -> Option<bool> {
    let mut held = Vec::new();
    let on_a = a(&mut held);
    if on_a == Some(false) {
        return Some(false);
    }
    match (on_a, b(&mut held)) {
        (_, Some(false)) => Some(false),
        (Some(true), Some(true)) => Some(true),
        _ => {
            errors.extend(held);
            None
        }
    }
}

/// Whether a search decision definitely came out `sign`; an escalation
/// keeps the piece open.
fn surely<T: Decide>(name: &'static str, margin: T, band: Band, sign: Sign) -> bool {
    geom_core::k_stats::decide(name, Margin::of(margin), band) == Ok(sign)
}

/// `e`'s certified curve, span and rate bounds, or the typed refusal
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
        lo,
        hi,
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

/// Whether a polish step that moves its point `moved` metres is kept:
/// while it moves less than `bound` (the steps a foot in reach needs,
/// and not a jump to a far branch), decided exactly. A singular step —
/// a tangency's — comes out poison and is dropped with the rest, at
/// every scalar.
///
/// **The `16·slack` bound has no row, and needs none.** A polished
/// parameter is read, never trusted: the reading decides the point's gap
/// at it, so a step the bound drops or keeps can lose a foot but never
/// invent one. In [`polish_foot`] the guarded quantity `|C′|·|dt|` is
/// `|(C − q)·C′|/|C′|`, at most the iterate's own gap, while `slack` is
/// the gap at the seed: the bound binds only once the gap has grown past
/// sixteen times the seed's, which is Gauss–Newton diverging.
fn step_kept<T: Decide>(moved: T, bound: T) -> bool {
    crate::splitting::order::exact_band().is_ok_and(|exact| {
        surely(
            "pm_census_curved_polish_step",
            bound - moved,
            exact,
            Sign::Positive,
        )
    })
}

/// Gauss–Newton from `t` toward `r`'s foot of `q`, each step kept
/// ([`step_kept`]) while it moves the point less than `16·slack` metres
/// along the carrier.
fn polish_foot<T: Decide>(r: &Read<T>, q: Point3<T>, mut t: T, slack: T) -> T {
    let bound = slack * T::from_f64(16.0);
    for _ in 0..POLISH_STEPS {
        let (p, d) = r.curve.ders1(t);
        let dt = T::zero() - (p - q).dot(d) / d.dot(d);
        if step_kept(d.norm() * dt.abs(), bound) {
            t = t + dt;
        }
    }
    t
}

/// **The pieces of `curve` over `span` that `q` may rest on**: the
/// span halved, a piece decided clear of `q` when its middle point's
/// distance less its reach ([`geom_core::bisect::reach`] of `hi`) decides
/// positive, and kept when its reach stops at the band ([`stops`]). Every
/// point of the span within the band of `q` lies in a kept piece. The one
/// walk the foot search ([`feet`]) and pass 2′'s skip
/// ([`may_rest_on`]) take; it decides nothing about the carrier's least
/// speed, so it never refuses an edge `read` would.
fn near_pieces<T: Decide>(
    curve: &Curve3<T>,
    span: (T, T),
    hi: SupSpeed<T>,
    q: Point3<T>,
    band: Band,
) -> Result<Vec<Piece<T>>, geom_core::bisect::Exhausted> {
    let mut out = Vec::new();
    bisect(
        [Piece::whole(span)],
        |p| -> Step<Piece<T>, core::convert::Infallible> {
            let reach = p.reach(hi);
            let gap = (curve.eval(p.mid()) - q).norm();
            if surely(
                "pm_census_ve_curved_clear",
                gap - reach,
                band,
                Sign::Positive,
            ) {
                return Step::Settled;
            }
            if stops("pm_census_ve_curved_piece", reach, band) {
                out.push(p);
                return Step::Settled;
            }
            Step::Halve(p.halves())
        },
    )?;
    Ok(out)
}

/// The parameters of `r`'s carrier where `q` may lie on it within the
/// band (module docs): each polished foot — of [`Curve3::param_near`]'s
/// parameter where the kind inverts in closed form, else of each piece
/// [`near_pieces`] keeps. `None` past the budget (pushed).
fn feet<T: Decide>(
    r: &Read<T>,
    q: Point3<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Vec<T>> {
    if let Some(t) = r.curve.param_near(q, geom::mid_param(r.span.0, r.span.1)) {
        let slack = (r.curve.eval(t) - q).norm();
        return Some(vec![polish_foot(r, q, t, slack)]);
    }
    let Ok(pieces) = near_pieces(&r.curve, r.span, r.hi, q, band) else {
        refuse(
            r.edge,
            "the census's foot search on a curved edge ran past its budget",
            errors,
        );
        return None;
    };
    Some(
        pieces
            .into_iter()
            .map(|p| {
                let m = p.mid();
                let slack = (r.curve.eval(m) - q).norm() + p.reach(r.hi);
                polish_foot(r, q, m, slack)
            })
            .collect(),
    )
}

/// **Whether `q` may rest on `e`'s span** — pass 2′'s skip, a consumer of
/// the curved lane's own walk ([`near_pieces`]): `false` only where every
/// piece is decided clear of `q`, so a vertex the sweep passes by holds
/// no touch the reading would have found; `true` where a piece survives,
/// past the budget, or for an edge with no certified curve, leaving the
/// question to the reading.
pub(super) fn may_rest_on<T: Decide>(body: &Body<T>, e: EdgeKey, q: Point3<T>, band: Band) -> bool {
    let Some(curve) = body
        .edges
        .get(e)
        .and_then(|d| body.curves.get(d.curve))
        .and_then(crate::null::CurveGeom::certified)
    else {
        return true;
    };
    let hi = geom::carrier_speed::speed_range(curve.carrier()).1;
    near_pieces(curve.carrier(), curve.params(), hi, q, band).map_or(true, |p| !p.is_empty())
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

/// [`on_curved_interior_reading`] on an edge already read. A foot off
/// the carrier may escalate where another is on the interior: its
/// escalations stand only if no foot decides.
fn on_interior_reading<T: Decide>(
    body: &Body<T>,
    r: &Read<T>,
    q: Point3<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Option<MarginDiag>> {
    let mut held = Vec::new();
    for t in feet(r, q, band, errors)? {
        if let Some(Some(margin)) = interior_reading(body, r, q, t, band, &mut held) {
            return Some(Some(margin));
        }
    }
    if held.is_empty() {
        return Some(None);
    }
    errors.extend(held);
    None
}

/// Whether `q`, read at `t` on `r`'s carrier, is on its interior: the
/// margin its gap decided Zero on, or `None` off it.
fn interior_reading<T: Decide>(
    body: &Body<T>,
    r: &Read<T>,
    q: Point3<T>,
    t: T,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Option<MarginDiag>> {
    let gap = Margin::norm3(q - r.curve.eval(t));
    let Some(margin) = gap_zero_reading("pm_census_ve_curved_gap", gap, band, errors)? else {
        return Some(None);
    };
    if on_conventional_vertex(body, r) {
        return Some(Some(margin));
    }
    let (t0, t1) = r.span;
    let mut interior = Some(Some(margin));
    for (m, end) in [(t - t0, t0), (t1 - t, t1)] {
        match clear_of_end(r, q, m, end, band, errors) {
            Some(true) => {}
            Some(false) => interior = interior.map(|_| None),
            None => interior = None,
        }
    }
    interior
}

/// Whether `q`, at parameter distance `m` inside the span from its end
/// at `end`, is clear of that end (module docs). The parameter distance
/// metered by the least speed understates the arc, so it decides clear
/// and past; where it reads zero or in its band, the same distance
/// metered by the greatest speed decides the side (past, or within the
/// band of the end, is not inside), and on the inner side the point's
/// distance from the end's point decides the rest.
fn clear_of_end<T: Decide>(
    r: &Read<T>,
    q: Point3<T>,
    m: T,
    end: T,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<bool> {
    match geom_core::k_stats::decide("pm_census_ve_curved_span", Margin::metered(m, r.lo), band) {
        Ok(Sign::Positive) => return Some(true),
        Ok(Sign::Negative) => return Some(false),
        Ok(Sign::Zero) | Err(_) => {}
    }
    match decided(
        "pm_census_ve_curved_side",
        Margin::metered_sup(m, r.hi),
        band,
        errors,
    )? {
        Sign::Negative | Sign::Zero => Some(false),
        Sign::Positive => {
            let chord = Margin::norm3(q - r.curve.eval(end));
            gap_is_zero("pm_census_ve_curved_end", chord, band, errors).map(|at| !at)
        }
    }
}

/// Whether a piece of reach `reach` is small enough to stop at, decided
/// on `row`: within the band.
fn stops<T: Decide>(row: &'static str, reach: T, band: Band) -> bool {
    matches!(
        geom_core::k_stats::decide(row, Margin::of(reach), band),
        Ok(Sign::Negative | Sign::Zero)
    )
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
    let interior = |m: &Meeting<T>| matches!((m.on_a, m.on_b), (Hit::On { .. }, Hit::On { .. }));
    let refusal = match meet(&span(body, ra), &span(body, rb), band) {
        Ok(met) => return Some(met.points.iter().any(interior)),
        Err(refusal) if refusal.found.iter().any(interior) => return Some(true),
        Err(refusal) => refusal,
    };
    let what = match refusal.fault {
        MeetFault::Escalated(cause) => {
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

    use super::{Read, curved_interiors_meet, feet, reads_meet};
    use crate::body::Body;
    use crate::entity::EdgeKey;
    use crate::{EdgeCurveSpec, MevSite};

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
        let (lo, hi) = geom::carrier_speed::speed_range(&curve);
        Read {
            edge: EdgeKey::default(),
            curve,
            span,
            lo,
            hi,
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
    /// circle a metre above it nowhere. Two coplanar ellipses at right
    /// angles, which meet near `(a, a)`, `a² = 4/5`, refuse typed on both
    /// edges: the pair's closed form has no method for two ellipses in
    /// one plane (`work/fuse/two-coplanar-ellipses-have-no-meeting-method.md`).
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
                None,
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
            let refused = errors.len() == 2
                && errors
                    .iter()
                    .all(|e| format!("{e:?}").contains("two ellipses in one plane"));
            assert!(
                errors.is_empty() == want.is_some() && (want.is_some() || refused),
                "{what}: refused typed exactly where undecided: {errors:?}"
            );
        }
    }

    /// **A tangency and a shallow crossing are decided.** A line tangent
    /// to a circle, two coplanar circles touching from outside, and a
    /// line crossing a circle at 0.01 rad each meet; the line moved
    /// `1e-4` off the circle does not. An arc ending where a circle
    /// touches it from outside meets it too: the arc stays within ε of
    /// the circle over its last few hundredths of a millimetre, so points
    /// of its interior clear of the end lie within the band of the
    /// circle. Red when a touch read at a span's end does not read the
    /// run about it.
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
                Some(true),
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
        let mut errors = Vec::new();
        let inside =
            super::interior_reading(&Body::new(), &r, r.curve.eval(t), t, band(), &mut errors)
                .map(|on| on.is_some());
        assert_eq!((inside, errors), (Some(true), vec![]));
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

    /// **A spline's foot is found by the search**: a point of it reads
    /// its own parameter, a point a millimetre off it none.
    #[test]
    fn a_point_on_a_spline_finds_its_foot() {
        let r = read(spline(), (0.0, 1.0));
        let q = r.curve.eval(0.3);
        let mut errors = Vec::new();
        let ts = feet(&r, q, band(), &mut errors).unwrap();
        assert!(
            !ts.is_empty() && ts.iter().all(|&t| (r.curve.eval(t) - q).norm() < 1e-12),
            "{ts:?}"
        );
        let off = feet(&r, q + Vec3::unit_z() * 1e-3, band(), &mut errors).unwrap();
        assert_eq!((off, errors), (vec![], vec![]));
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
}
