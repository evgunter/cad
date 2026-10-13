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
//! otherwise from each piece of the span the search below keeps. An end
//! is cleared when the parameter's distance from it, metered by the
//! carrier's least speed ([`crate::carrier_speed::speed_range`]), decides
//! positive, and reached when it decides negative; between the two the
//! same distance metered by the greatest speed decides the side, and on
//! the inner side the point's distance from the end's point decides.
//!
//! Two edges' interiors meet when one candidate is on both: each edge's
//! midpoint and the points halfway between one edge's end and the
//! other's ends, one of which an overlap holds; then every meeting the
//! search finds, each read on both carriers at its own parameters.
//!
//! **The search** is [`crate::bisect`]'s walk over the spans. A piece
//! reaches at most `ℓ` metres from its middle point, `ℓ` its half-span
//! times the carrier's greatest speed. Two pieces are cleared when the
//! distance between their middle points less both reaches decides
//! positive, or, where both carriers bound their acceleration, the
//! distance's component along a unit `n` less each piece's run along `n`
//! and its bend (`|n·C′|·h + |C″|·h²/2`), the second-order bound
//! `boolean::carrier_touch` takes, read along the gap and across each
//! piece's tangent. Pieces stop at [`crate::bisect::tangency_floor`] of
//! `1/κ`, `κ` the two carriers' summed curvature bound
//! ([`crate::carrier_speed::curvature_hi`]). A spline bounds neither
//! acceleration nor curvature here, so its pieces stop at the band
//! itself and a tangency of a spline runs past the budget and refuses
//! typed. A pair that stops is polished toward the meeting and read.
//! Every piece the search does not clear ends at such a pair, so no
//! meeting is left unread; a walk past [`crate::bisect::PIECE_BUDGET`]
//! refuses typed, naming both edges.

use geom::Curve3;
use geom_core::{Band, Decide, InfSpeed, Margin, Point3, Real, Sign, SupSpeed, Vec3};

use super::{CensusSubject, CensusUnsupportedCause, ValidationError, gap_is_zero};
use crate::bisect::{Step, bisect};
use crate::body::Body;
use crate::entity::{EdgeKey, EntityId};

/// The Gauss–Newton steps a candidate is polished by.
const POLISH_STEPS: usize = 4;

/// An edge's certified curve as these lanes read it.
struct Read<T: Real> {
    edge: EdgeKey,
    curve: Curve3<T>,
    span: (T, T),
    lo: InfSpeed<T>,
    hi: SupSpeed<T>,
    /// A bound on `|C″|`, `None` on a spline.
    accel: Option<T>,
    /// A bound on the curvature, `None` on a spline.
    curvature: Option<T>,
}

/// The meeting search's answer: the two interiors meet.
struct Met;

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

    /// Half the piece's parameter run.
    fn half(self) -> T {
        (self.b - self.a).abs() * T::from_f64(0.5)
    }

    /// The most metres a point of the piece lies from its middle point.
    fn reach(self, hi: SupSpeed<T>) -> T {
        crate::bisect::reach(hi, (self.a, self.b))
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
        refuse(edge, "a record on an edge with no certified curve", errors);
        return None;
    };
    let carrier = curve.carrier().clone();
    let (lo, hi) = crate::carrier_speed::speed_range(&carrier);
    let accel = crate::carrier_speed::accel_hi(&carrier);
    let curvature = crate::carrier_speed::curvature_hi(&carrier);
    let span = curve.params();
    let span_reach = Margin::metered_sup((span.1 - span.0).abs(), hi);
    if decided("pm_census_curved_speed_hi", span_reach, band, errors)? != Sign::Positive {
        refuse(
            edge,
            "a record on a curved edge whose speed bound meters no reach",
            errors,
        );
        return None;
    }
    let length = Margin::metered((span.1 - span.0).abs(), lo);
    if decided("pm_census_curved_speed_floor", length, band, errors)? != Sign::Positive {
        refuse(
            edge,
            "a record on a curved edge whose least speed meters no length over its span",
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
        accel,
        curvature,
    })
}

/// Whether a polish step that moves its point `moved` metres is kept:
/// while it moves less than `bound` (the steps a foot in reach needs,
/// and not a jump to a far branch), decided exactly. A singular step —
/// a tangency's — comes out poison and is dropped with the rest, at
/// every scalar.
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

/// Gauss–Newton from `(s, t)` toward where `a` and `b` meet, kept as
/// [`polish_foot`]'s steps are; at a tangency the step is singular, and
/// the feet that follow it in [`meet_at`] still land.
fn polish_pair<T: Decide>(a: &Read<T>, b: &Read<T>, (mut s, mut t): (T, T), slack: T) -> (T, T) {
    let bound = slack * T::from_f64(16.0);
    for _ in 0..POLISH_STEPS {
        let (pa, da) = a.curve.ders1(s);
        let (pb, db) = b.curve.ders1(t);
        let d = pa - pb;
        let (aa, bb, ab) = (da.dot(da), db.dot(db), da.dot(db));
        let (ad, bd) = (da.dot(d), db.dot(d));
        let det = aa * bb - ab.powi(2);
        let (ds, dt) = ((ab * bd - bb * ad) / det, (aa * bd - ab * ad) / det);
        if step_kept(da.norm() * ds.abs() + db.norm() * dt.abs(), bound) {
            s = s + ds;
            t = t + dt;
        }
    }
    (s, t)
}

/// The parameters of `r`'s carrier where `q` may lie on it within the
/// band (module docs): each polished foot. `None` past the budget
/// (pushed).
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
    let mut out = Vec::new();
    let walk = bisect(
        [Piece::whole(r.span)],
        |p| -> Step<Piece<T>, core::convert::Infallible> {
            let (m, reach) = (p.mid(), p.reach(r.hi));
            let gap = (r.curve.eval(m) - q).norm();
            if surely(
                "pm_census_ve_curved_clear",
                gap - reach,
                band,
                Sign::Positive,
            ) {
                return Step::Settled;
            }
            if surely("pm_census_ve_curved_piece", reach, band, Sign::Zero) {
                out.push(polish_foot(r, q, m, gap + reach));
                return Step::Settled;
            }
            Step::Halve(p.halves())
        },
    );
    if walk.is_err() {
        refuse(
            r.edge,
            "the census's foot search on a curved edge ran past its budget",
            errors,
        );
        return None;
    }
    Some(out)
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
    let r = read(body, e, band, errors)?;
    on_interior(body, &r, q, band, errors)
}

/// [`on_curved_interior`] on an edge already read. A foot off the
/// carrier may escalate where another is on the interior: its
/// escalations stand only if no foot decides.
fn on_interior<T: Decide>(
    body: &Body<T>,
    r: &Read<T>,
    q: Point3<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<bool> {
    let mut held = Vec::new();
    for t in feet(r, q, band, errors)? {
        if interior_at(body, r, q, t, band, &mut held) == Some(true) {
            return Some(true);
        }
    }
    if held.is_empty() {
        return Some(false);
    }
    errors.extend(held);
    None
}

/// Whether `q`, read at `t` on `r`'s carrier, is on its interior.
fn interior_at<T: Decide>(
    body: &Body<T>,
    r: &Read<T>,
    q: Point3<T>,
    t: T,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<bool> {
    let gap = Margin::norm3(q - r.curve.eval(t));
    if !gap_is_zero("pm_census_ve_curved_gap", gap, band, errors)? {
        return Some(false);
    }
    // A closed edge's conventional vertex has no identity of its own:
    // its point is the edge's interior too.
    let start = body
        .edges
        .get(r.edge)
        .and_then(|d| body.half_edges.get(d.he_plus))
        .map(|h| h.start);
    if start.is_some_and(|v| crate::boolean::is_conventional_vertex(body, v)) {
        return Some(true);
    }
    let (t0, t1) = r.span;
    let mut interior = Some(true);
    for (m, end) in [(t - t0, t0), (t1 - t, t1)] {
        match clear_of_end(r, q, m, end, band, errors) {
            Some(true) => {}
            Some(false) => interior = interior.map(|_| false),
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

/// Whether the pieces `pa` of `a` and `pb` of `b` are definitely apart
/// (module docs, the search).
fn pieces_apart<T: Decide>(
    (a, pa): (&Read<T>, Piece<T>),
    (b, pb): (&Read<T>, Piece<T>),
    band: Band,
) -> bool {
    let ((qa, da), (qb, db)) = (a.curve.ders1(pa.mid()), b.curve.ders1(pb.mid()));
    let gap = qa - qb;
    let dist = gap.norm();
    let clear = |bound: T| surely("pm_census_ee_curved_clear", bound, band, Sign::Positive);
    if clear(dist - pa.reach(a.hi) - pb.reach(b.hi)) {
        return true;
    }
    let (Some(acc_a), Some(acc_b)) = (a.accel, b.accel) else {
        return false;
    };
    let (ha, hb) = (pa.half(), pb.half());
    let bend = (acc_a * ha.powi(2) + acc_b * hb.powi(2)) * T::from_f64(0.5);
    // `|D| ≥ |n·D|` for every unit `n`: along the gap, and across each
    // piece's tangent, where two near-parallel pieces read their
    // separation rather than their offset along it. A direction that
    // comes out poison decides nothing.
    let across = |d: Vec3<T>| gap - d * (gap.dot(d) / d.dot(d));
    [gap, across(da), across(db)].into_iter().any(|n| {
        let n = n / n.norm();
        clear(n.dot(gap).abs() - n.dot(da).abs() * ha - n.dot(db).abs() * hb - bend)
    })
}

/// The tangency floor of `a` against `b` in metres (module docs), or
/// `None` where a carrier bounds no curvature.
fn floor<T: Decide>(a: &Read<T>, b: &Read<T>, band: Band) -> Option<T> {
    let kappa = a.curvature? + b.curvature?;
    Some(crate::bisect::tangency_floor(
        T::one() / kappa,
        band.escalate(),
    ))
}

/// Whether a piece of reach `reach` is small enough to stop at: within
/// the floor, decided — an enclosure of the floor is read at its lower
/// end, so a wide one halves on rather than stopping a long piece. A
/// floor that is not a finite length (two straight carriers' `κ = 0`) is
/// no floor: pieces stop at the band.
fn stops<T: Decide>(reach: T, floor: Option<T>, band: Band) -> bool {
    let within = |margin: T| {
        matches!(
            geom_core::k_stats::decide("pm_census_ee_curved_piece", Margin::of(margin), band),
            Ok(Sign::Negative | Sign::Zero)
        )
    };
    match floor {
        Some(f) if geom_core::is_finite_length(f) => within(reach - f),
        Some(_) | None => within(reach),
    }
}

/// Whether `a` and `b` meet on both interiors at the pieces `pa` and
/// `pb`, which stopped: polished toward the meeting, then each carrier
/// read at its own foot of the other's point.
fn meet_at<T: Decide>(
    body: &Body<T>,
    (a, pa): (&Read<T>, Piece<T>),
    (b, pb): (&Read<T>, Piece<T>),
    band: Band,
    held: &mut Vec<ValidationError>,
) -> bool {
    let (ma, mb) = (pa.mid(), pb.mid());
    let slack = (a.curve.eval(ma) - b.curve.eval(mb)).norm() + pa.reach(a.hi) + pb.reach(b.hi);
    let (s, t) = polish_pair(a, b, (ma, mb), slack);
    let t = polish_foot(b, a.curve.eval(s), t, slack);
    let s = polish_foot(a, b.curve.eval(t), s, slack);
    let p = a.curve.eval(s);
    interior_at(body, a, p, s, band, held) == Some(true)
        && interior_at(body, b, p, t, band, held) == Some(true)
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

/// [`curved_interiors_meet`] on two edges already read.
fn reads_meet<T: Decide>(
    body: &Body<T>,
    ra: &Read<T>,
    rb: &Read<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<bool> {
    let mid = |r: &Read<T>| r.curve.eval(geom::mid_param(r.span.0, r.span.1));
    let mut points = vec![mid(ra), mid(rb)];
    // An overlap that holds neither midpoint holds the point halfway
    // between one edge's end and the other's end on its carrier.
    let mut held = Vec::new();
    for (r, other) in [(ra, rb), (rb, ra)] {
        let (t0, t1) = r.span;
        for q in [
            other.curve.eval(other.span.0),
            other.curve.eval(other.span.1),
        ] {
            for t in feet(r, q, band, &mut held).unwrap_or_default() {
                points.extend([
                    r.curve.eval(geom::mid_param(t0, t)),
                    r.curve.eval(geom::mid_param(t, t1)),
                ]);
            }
        }
    }
    // A candidate off either edge may escalate where another is on
    // both: its escalations stand only if no candidate decides.
    if points.into_iter().any(|p| {
        on_interior(body, ra, p, band, &mut held) == Some(true)
            && on_interior(body, rb, p, band, &mut held) == Some(true)
    }) {
        return Some(true);
    }
    let floor = floor(ra, rb, band);
    let walk = bisect(
        [(Piece::whole(ra.span), Piece::whole(rb.span))],
        |(pa, pb)| {
            if pieces_apart((ra, pa), (rb, pb), band) {
                return Step::Settled;
            }
            let stop_a = stops(pa.reach(ra.hi), floor, band);
            let stop_b = stops(pb.reach(rb.hi), floor, band);
            if stop_a && stop_b {
                return if meet_at(body, (ra, pa), (rb, pb), band, &mut held) {
                    Step::Stop(Met)
                } else {
                    Step::Settled
                };
            }
            if !stop_a && (stop_b || pa.depth <= pb.depth) {
                Step::Halve(pa.halves().map(|h| (h, pb)))
            } else {
                Step::Halve(pb.halves().map(|h| (pa, h)))
            }
        },
    );
    match walk {
        Ok(Some(Met)) => return Some(true),
        Ok(None) => {}
        Err(_) => {
            let what = "the census's meeting search of two curved edges ran past its budget";
            refuse(ra.edge, what, errors);
            refuse(rb.edge, what, errors);
            return None;
        }
    }
    if held.is_empty() {
        return Some(false);
    }
    errors.extend(held);
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
        let (lo, hi) = crate::carrier_speed::speed_range(&curve);
        let accel = crate::carrier_speed::accel_hi(&curve);
        let curvature = crate::carrier_speed::curvature_hi(&curve);
        Read {
            edge: EdgeKey::default(),
            curve,
            span,
            lo,
            hi,
            accel,
            curvature,
        }
    }

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn ellipse(major: f64, minor: f64, u_ref: Vec3<f64>) -> Curve3<f64> {
        Curve3::Ellipse {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::unit_z(),
            major,
            minor,
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

    /// **The search reads every meeting of two carriers, on every
    /// kind**, and clears carriers that keep apart: two coplanar
    /// ellipses at right angles meet near `(a, a)`, `a² = 4/5`, which
    /// holds neither one's midpoint; a line across a spiric's oval meets
    /// it; a line through a spline's point meets it there; an ellipse
    /// and a circle a metre above it nowhere.
    #[test]
    fn the_search_reads_a_meeting_on_every_kind() {
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
            assert_eq!(meet(&x, &y), (want, vec![]), "{what}");
        }
    }

    /// **A tangency and a shallow crossing are decided.** A line tangent
    /// to a circle, two coplanar circles touching from outside, and a
    /// line crossing a circle at 0.01 rad each meet; the line moved
    /// `1e-4` off the circle does not. An arc ending where a circle
    /// touches it from outside meets it too: the arc stays within ε of
    /// the circle over its last few hundredths of a millimetre, and the
    /// pieces beyond the end read their feet past it, so the walk runs
    /// down to the floor before a piece inside the arc decides. Red when
    /// the search clears by the first-order bound alone, and, through
    /// that row, when its pieces stop at the band rather than at the
    /// tangency floor: each runs past the budget, at every ε.
    #[test]
    fn a_tangency_and_a_shallow_crossing_are_decided() {
        let o = Point3::new(0.0, 0.0, 0.0);
        let unit = || read(circle(o, 1.0), (0.3, 2.9));
        let shallow = Vec3::new(0.01_f64.cos(), 0.01_f64.sin(), 0.0);
        let cases = [
            (
                "a line tangent to a circle",
                read(
                    line(Point3::new(-1.0, 1.0, 0.0), Vec3::unit_x()),
                    (0.0, 2.3),
                ),
                Some(true),
            ),
            (
                "a circle touching it from outside",
                read(circle(Point3::new(0.0, 2.0, 0.0), 1.0), (-2.9, -0.3)),
                Some(true),
            ),
            (
                "a line crossing it at 0.01 rad",
                read(
                    line(Point3::new(0.0, 1.0, 0.0) - shallow * 0.2, shallow),
                    (0.0, 0.4),
                ),
                Some(true),
            ),
            (
                "a circle touching it from outside at the end of an arc of it",
                read(circle(Point3::new(0.0, 2.0, 0.0), 1.0), (-2.9, -0.3)),
                Some(true),
            ),
            (
                "a line 1e-4 off it",
                read(
                    line(Point3::new(-1.0, 1.0 + 1e-4, 0.0), Vec3::unit_x()),
                    (0.0, 2.3),
                ),
                Some(false),
            ),
        ];
        for (what, other, want) in cases {
            let arc = if what.contains("end of an arc") {
                read(circle(o, 1.0), (0.3, core::f64::consts::FRAC_PI_2))
            } else {
                unit()
            };
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
        let inside = super::interior_at(&Body::new(), &r, r.curve.eval(t), t, band(), &mut errors);
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
        let on = super::on_interior(&Body::new(), &r, q, band(), &mut errors);
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
                let on = super::on_interior(&Body::new(), &r, q, band(), &mut errors);
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

    /// **A piece stops only within a finite floor's lower end.** A
    /// floor read as the enclosure `[0.01, 1000]` stops no piece of reach
    /// 1 and stops one of reach 0.005; an infinite floor — two straight
    /// carriers' zero curvature — stops a piece only at the band. Red when a piece stops wherever
    /// its reach does not decide past the floor (the enclosure's upper
    /// end, which stops the walk at its root), and when an infinite floor
    /// stops one.
    #[test]
    fn a_piece_stops_only_within_a_finite_floors_lower_end() {
        use geom_core::Interval;
        let at = |x: f64| Interval::from_bounds(x, x);
        let stops = |reach: f64, floor: Interval| super::stops(at(reach), Some(floor), band());
        let wide = Interval::from_bounds(0.01, 1e3);
        assert!(
            !stops(1.0, wide),
            "a long piece against a wide floor halves on"
        );
        assert!(
            stops(0.005, wide),
            "a piece within the floor's lower end stops"
        );
        assert!(stops(1.0, at(2.0)), "a piece within a point floor stops");
        assert!(
            !super::stops(1.0, Some(f64::INFINITY), band()),
            "an infinite floor stops no piece at f64"
        );
        assert!(
            super::stops(band().zero() / 2.0, Some(f64::INFINITY), band()),
            "a piece within the band stops"
        );
    }

    /// **Near-concentric arcs: coincident meet, far apart do not, and
    /// the window between refuses loudly.** Two coplanar concentric
    /// arcs `Δr` apart: at half the band's coincidence threshold they
    /// meet; at `1e-3` they are apart; at `1e-5`, past the band at every
    /// ε row, the search clears near-parallel pieces only once their
    /// bend `h²` fits under `Δr` and runs past the budget, refusing on
    /// both edges (`work/fuse/near-concentric-arcs-run-the-census-meeting-search-past-its-budget.md`).
    /// Red when that window answers silently.
    #[test]
    fn near_concentric_arcs_meet_part_or_refuse_loudly() {
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
        assert_eq!(pair(1e-3), (Some(false), vec![]), "far apart");
        assert!(
            1e-5 > band().escalate(),
            "the window row needs K·ε below 1e-5"
        );
        let (met, errors) = pair(1e-5);
        assert_eq!(met, None, "the near window answers silently: {errors:?}");
        assert!(
            errors.len() == 2
                && errors
                    .iter()
                    .all(|e| format!("{e:?}").contains("ran past its budget")),
            "the near window refuses on the budget, naming both edges: {errors:?}"
        );
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
        let on = super::on_interior(&Body::new(), &r, q, band(), &mut errors);
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
}
