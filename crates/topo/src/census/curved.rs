//! The confirm lanes for records on **curved edges**: a `(vertex,
//! edge)` record whose edge is not a line, and an edge-edge record with
//! a curved side. The join writes both (`boolean::edge_join`): a
//! vertex a record named that the join took, or a conventional vertex,
//! reads as the joined edge's interior, whatever its carrier.
//!
//! One reading serves every carrier kind. A point is on a curved edge's
//! interior when it is on the carrier (its distance from the carrier's
//! point at its recovered parameter decided zero) and strictly inside
//! the span at both ends, metered in metres through the carrier's least
//! speed ([`crate::carrier_speed::speed_range`]), or anywhere on a closed
//! edge whose vertex is conventional — the questions
//! [`super::on_edge_interior`] asks of a line. The parameter is
//! [`geom::Curve3::param_near`]'s where the kind inverts in closed form,
//! and otherwise each foot the search below finds.
//!
//! Two edges' interiors meet when one candidate point is on both: each
//! edge's midpoint and the points halfway between one edge's end and
//! the other's ends, one of which an overlap holds; then every point the
//! search finds where the carriers come within the band.
//!
//! **The search** bisects the spans. A piece reaches at most `ℓ` metres
//! from its middle point, `ℓ` its half-length times the carrier's
//! greatest speed, so two pieces hold no common point when the distance
//! between their middle points less both reaches decides positive, and
//! a piece holds no point within the band of `q` when its distance from
//! `q` less its reach does. A piece whose reach decides zero is a point
//! at the band: its middle is polished by Gauss–Newton toward the
//! meeting (a step is kept only while it stays near the piece) and read
//! as a candidate. Every piece the search does not
//! clear ends at such a point, so no meeting is left unread; a search
//! that cuts past [`PIECE_BUDGET`] refuses typed.

use geom::Curve3;
use geom_core::{Band, Decide, InfSpeed, Margin, Point3, Real, Sign, SupSpeed};

use super::{CensusSubject, CensusUnsupportedCause, ValidationError, gap_is_zero};
use crate::body::Body;
use crate::entity::{EdgeKey, EntityId};

/// The most pieces, or pairs of pieces, a search cuts before it refuses:
/// a bound on the walk (D9). A transverse meeting keeps a few pairs open
/// per level of the bisection, down to the band, about `2·log₂(ℓ/ε)`
/// levels below the spans.
const PIECE_BUDGET: usize = 16384;

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
        Margin::metered_sup((self.b - self.a).abs() * T::from_f64(0.5), hi).value()
    }

    /// The two halves, the later first: a stack pops the earlier one.
    fn halves(self) -> [Self; 2] {
        let (m, depth) = (self.mid(), self.depth + 1);
        [
            Self {
                a: m,
                b: self.b,
                depth,
            },
            Self {
                a: self.a,
                b: m,
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

/// `e`'s certified curve, span and speed bounds, or the typed refusal
/// pushed for an edge with none, or whose least speed meters no length
/// over its span (a spline's control-net floor can be zero).
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
    let span = curve.params();
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
    })
}

/// Gauss–Newton from `(s, t)` toward where `a` and `b` meet (or, with
/// `b` a point, toward `a`'s foot of it), each step kept only while it
/// moves the parameters less than `16·slack` metres, `slack` the
/// leaf's gap and reaches: the steps a leaf of a meeting in the band
/// needs, and not a jump to a far branch.
fn polish<T: Decide>(
    (ra, mut s): (&Read<T>, T),
    b: Option<(&Read<T>, T)>,
    q: Point3<T>,
    slack: T,
) -> (T, T) {
    let mut t = b.map_or(T::zero(), |(_, t)| t);
    let bound = slack * T::from_f64(16.0);
    for _ in 0..POLISH_STEPS {
        let (pa, da) = ra.curve.ders1(s);
        let (ds, dt, moved) = match b {
            None => {
                let d = pa - q;
                let ds = T::zero() - d.dot(da) / da.dot(da);
                (ds, T::zero(), ra.hi.to_meters(ds.abs()))
            }
            Some((rb, _)) => {
                let (pb, db) = rb.curve.ders1(t);
                let d = pa - pb;
                let (aa, bb, ab) = (da.dot(da), db.dot(db), da.dot(db));
                let (ad, bd) = (da.dot(d), db.dot(d));
                let det = aa * bb - ab.powi(2);
                let (ds, dt) = ((ab * bd - bb * ad) / det, (aa * bd - ab * ad) / det);
                (
                    ds,
                    dt,
                    ra.hi.to_meters(ds.abs()) + rb.hi.to_meters(dt.abs()),
                )
            }
        };
        let keep = moved - bound;
        s = keep.select_le_zero(s + ds, s);
        t = keep.select_le_zero(t + dt, t);
    }
    (s, t)
}

/// The parameters of `r`'s carrier where `q` may lie on it within the
/// band (module docs, the search): `param_near`'s where it inverts the
/// kind, otherwise each polished foot. `None` past the budget (pushed).
fn feet<T: Decide>(
    r: &Read<T>,
    q: Point3<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Vec<T>> {
    if let Some(t) = r.curve.param_near(q, geom::mid_param(r.span.0, r.span.1)) {
        return Some(vec![t]);
    }
    let mut out = Vec::new();
    let mut stack = vec![Piece::whole(r.span)];
    let mut cut = 0;
    while let Some(p) = stack.pop() {
        cut += 1;
        if cut > PIECE_BUDGET {
            refuse(
                r.edge,
                "the census's foot search on a curved edge ran past its budget",
                errors,
            );
            return None;
        }
        let (m, reach) = (p.mid(), p.reach(r.hi));
        let gap = (r.curve.eval(m) - q).norm();
        if surely(
            "pm_census_ve_curved_clear",
            gap - reach,
            band,
            Sign::Positive,
        ) {
            continue;
        }
        if surely("pm_census_ve_curved_piece", reach, band, Sign::Zero) {
            out.push(polish((r, m), None, q, gap + reach).0);
            continue;
        }
        stack.extend(p.halves());
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

/// [`on_curved_interior`] on an edge already read. A candidate foot off
/// the carrier may escalate where another is on the interior: its
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
    for m in [t - t0, t1 - t] {
        match decided(
            "pm_census_ve_curved_span",
            Margin::metered(m, r.lo),
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

/// Each point where `a`'s and `b`'s carriers may meet over their spans
/// (module docs, the search), polished. `None` past the budget (pushed).
fn meetings<T: Decide>(
    a: &Read<T>,
    b: &Read<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Vec<Point3<T>>> {
    let mut out = Vec::new();
    let mut stack = vec![(Piece::whole(a.span), Piece::whole(b.span))];
    let mut cut = 0;
    while let Some((pa, pb)) = stack.pop() {
        cut += 1;
        if cut > PIECE_BUDGET {
            refuse(
                a.edge,
                "the census's meeting search of two curved edges ran past its budget",
                errors,
            );
            return None;
        }
        let (ma, mb) = (pa.mid(), pb.mid());
        let (reach_a, reach_b) = (pa.reach(a.hi), pb.reach(b.hi));
        let gap = (a.curve.eval(ma) - b.curve.eval(mb)).norm();
        if surely(
            "pm_census_ee_curved_clear",
            gap - reach_a - reach_b,
            band,
            Sign::Positive,
        ) {
            continue;
        }
        let point_a = surely("pm_census_ee_curved_piece", reach_a, band, Sign::Zero);
        let point_b = surely("pm_census_ee_curved_piece", reach_b, band, Sign::Zero);
        if point_a && point_b {
            let slack = gap + reach_a + reach_b;
            let (s, _) = polish((a, ma), Some((b, mb)), Point3::origin(), slack);
            out.push(a.curve.eval(s));
        } else if !point_a && (point_b || pa.depth <= pb.depth) {
            stack.extend(pa.halves().map(|h| (h, pb)));
        } else {
            stack.extend(pb.halves().map(|h| (pa, h)));
        }
    }
    Some(out)
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
    let mid = |r: &Read<T>| r.curve.eval(geom::mid_param(r.span.0, r.span.1));
    let mut points = vec![mid(&ra), mid(&rb)];
    // An overlap that holds neither midpoint holds the point halfway
    // between one edge's end and the other's end on its carrier.
    let mut held = Vec::new();
    for (r, other) in [(&ra, &rb), (&rb, &ra)] {
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
    let on_both = |p: Point3<T>, held: &mut Vec<ValidationError>| {
        on_interior(body, &ra, p, band, held) == Some(true)
            && on_interior(body, &rb, p, band, held) == Some(true)
    };
    if points.into_iter().any(|p| on_both(p, &mut held)) {
        return Some(true);
    }
    if meetings(&ra, &rb, band, errors)?
        .into_iter()
        .any(|p| on_both(p, &mut held))
    {
        return Some(true);
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

    use super::{Read, curved_interiors_meet, feet, meetings};
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
        Read {
            edge: EdgeKey::default(),
            curve,
            span,
            lo,
            hi,
        }
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

    fn line(origin: Point3<f64>, dir: Vec3<f64>) -> Curve3<f64> {
        Curve3::Line { origin, dir }
    }

    /// The rational quadratic arc `shell`'s speed row reads, in `z = 0`.
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

    /// `got` and `want` are the same points to `1e-9`, each counted
    /// once.
    fn same_points(got: &[Point3<f64>], want: &[Point3<f64>]) -> bool {
        let near = |p: &Point3<f64>, q: &Point3<f64>| (*p - *q).norm() < 1e-9;
        got.iter().all(|p| want.iter().any(|q| near(p, q)))
            && want.iter().all(|q| got.iter().any(|p| near(p, q)))
    }

    /// **The search finds every meeting of two carriers, on every
    /// kind**, and clears carriers that keep apart: two coplanar
    /// ellipses at right angles meet at their four `(±a, ±a)`, `a² =
    /// 4/5`; a line across a spiric's oval meets it twice; a line
    /// through a spline's point meets it there; an ellipse and a circle
    /// a metre above it, nowhere.
    #[test]
    fn the_search_finds_every_meeting_on_every_kind() {
        let band = Band::linear(Tol::witness()).unwrap();
        let a = (0.8_f64).sqrt();
        let (r, d) = (0.5, 0.3);
        let h = (r * r - ((d * d + 4.0_f64).sqrt() - 2.0).powi(2)).sqrt();
        let on = spline().eval(0.5);
        let cases = [
            (
                "ellipse × ellipse",
                read(ellipse(2.0, 1.0, Vec3::unit_x()), (0.0, TAU)),
                read(ellipse(2.0, 1.0, Vec3::unit_y()), (0.0, TAU)),
                vec![
                    Point3::new(a, a, 0.0),
                    Point3::new(-a, a, 0.0),
                    Point3::new(-a, -a, 0.0),
                    Point3::new(a, -a, 0.0),
                ],
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
                    (-3.0, 3.0),
                ),
                read(line(Point3::new(d, 2.0, -1.0), Vec3::unit_z()), (0.0, 2.0)),
                vec![Point3::new(d, 2.0, h), Point3::new(d, 2.0, -h)],
            ),
            (
                "spline × line",
                read(spline(), (0.0, 1.0)),
                read(line(on - Vec3::unit_z(), Vec3::unit_z()), (0.0, 2.0)),
                vec![on],
            ),
            (
                "ellipse × a circle above it",
                read(ellipse(2.0, 1.0, Vec3::unit_x()), (0.0, TAU)),
                read(
                    Curve3::Circle {
                        center: Point3::new(0.0, 0.0, 1.0),
                        axis: Vec3::unit_z(),
                        radius: 1.5,
                        u_ref: Vec3::unit_x(),
                    },
                    (0.0, TAU),
                ),
                vec![],
            ),
        ];
        for (what, x, y, want) in cases {
            let mut errors = Vec::new();
            let got = meetings(&x, &y, band, &mut errors).expect(what);
            assert_eq!(errors, vec![], "{what}");
            assert!(same_points(&got, &want), "{what}: {got:?}");
        }
    }

    /// **A spline's foot is found by the search**: a point of it reads
    /// its own parameter, a point a millimetre off it none.
    #[test]
    fn a_point_on_a_spline_finds_its_foot() {
        let band = Band::linear(Tol::witness()).unwrap();
        let r = read(spline(), (0.0, 1.0));
        let q = r.curve.eval(0.3);
        let mut errors = Vec::new();
        let ts = feet(&r, q, band, &mut errors).unwrap();
        assert!(
            !ts.is_empty() && ts.iter().all(|&t| (r.curve.eval(t) - q).norm() < 1e-12),
            "{ts:?}"
        );
        let off = feet(&r, q + Vec3::unit_z() * 1e-3, band, &mut errors).unwrap();
        assert_eq!((off, errors), (vec![], vec![]));
    }
}
