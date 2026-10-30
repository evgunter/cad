//! **The spline span × analytic face arm** of the curve × face table
//! ([`super::Door::SplineComposite`]): the face's canonical residual
//! composed along the span's NURBS carrier, the ring composite `f ∘ C`
//! ([`geom_core::spline::compose::canonical_composite`]), read piece by
//! piece. The boolean dispatches the plane alone (`super::route`); the
//! arm reads a cylinder, cone, sphere or torus the same way, metered
//! through the form's distance divisor.
//!
//! The carrier's net is written in the surface's chart frame in
//! certification arithmetic, so the composite is a rational Bernstein
//! form `f = N / D`, `D` a power of the carrier's weight function and so
//! positive: the carrier's door admits no other weight. An enclosure of
//! `D` that reaches zero is no exception to read: the quotient it
//! divides refuses, its piece reads nothing definite and is cut, and a
//! piece still so at the floor refuses. Every
//! reading is of `f` itself, metered: its enclosure over a part, divided
//! by an upper bound on the form's distance divisor over the net's hull
//! (the plane's is one), bounds the part's distance from the surface from
//! below, and that is what is decided against the band `implicit_residual`
//! is decided against. **A piece never straddles a knot**: the pieces
//! start as the carrier's knot spans cut at the span's ends, and every
//! later piece is a part of one, so each row is one polynomial and a
//! derivative read on it is the derivative there. On a piece:
//!
//! - **clear** where `f`'s enclosure over it is definitely past the band:
//!   the carrier is definitely apart from the surface there. A piece
//!   that is one-signed but not definitely apart is not clear;
//! - **monotone** where `N′D − ND′`, which has `f′`'s sign, is one-signed:
//!   `f` takes its extremes at the piece's ends, so the ends' readings
//!   settle it. Ends definitely past the band on one side are clear; on
//!   opposite sides, one crossing, bisected and read ON the surface
//!   ([`crate::implicit_residual`]). An end within the band at the span's
//!   end is that end's meeting, which the caller reads; one at a knot is
//!   read with the piece on the knot's other side, a crossing where their
//!   far ends lie on opposite sides and a touch otherwise;
//! - otherwise the piece is cut where `f` is definitely past the band. A
//!   piece too short to cut, or with no such cut, refuses: the carrier is
//!   within the band of the surface along it, or crosses it within the
//!   band of another crossing, and no reading parts the two.
//!
//! Every meeting is then placed. One within the band of a **crease** (an
//! interior knot of multiplicity at least the degree,
//! `KnotStructure::crease_runs`), where the carrier has no tangent,
//! refuses, read in metres from the crease's point. A touch refuses. A
//! crossing must read ON the surface; one within the band of an end of
//! the span is that end's meeting, and two within the band of each other
//! cannot be told apart, so they refuse.
//!
//! The walk is `geom_core::bisect::bisect`, with its piece budget.

use geom::{NurbsCurve3, Surface};
use geom_core::bisect::{Step, bisect};
use geom_core::k_stats::decide;
use geom_core::spline::compose::{CanonicalSurface, CurveCertData, canonical_composite};
use geom_core::{Band, Bounds, Decide, Indeterminate, Interval, Margin, Point3, Real, Sign, Vec3};

use crate::half_angle::SPLITS;
use crate::pcurve_cache::projected::Meters;
use crate::roots::{Unsettled, touch_or_tangency};

/// Why [`crossings`] gave no answer.
#[derive(Clone, Copy, Debug)]
pub enum CompositeRefusal {
    /// A decision the arm must read definite fell in the band's gap:
    /// whether a located crossing lies ON the surface, where it stands
    /// against an end of the span, a crease or the crossing before it, or
    /// a piece's end, or every share of a piece, read in the gap where the
    /// carrier comes within the band of the surface.
    Escalated(Indeterminate),
    /// The walk reached a limit, or a reading it has no arm for:
    /// [`Unsettled::Tangency`] (the carrier decided within the zero band
    /// of the surface along a piece, at every share, or at a knot it
    /// leaves the way it came, or two crossings the band reads at one
    /// point), [`Unsettled::Floor`] (a piece at the floor that its
    /// enclosure does not read past the band), [`Unsettled::Crease`],
    /// [`Unsettled::Meter`] (a located crossing reads definitely off the
    /// surface) or [`Unsettled::Exhausted`].
    Unsettled(Unsettled),
    /// The composite could not be formed: the face has no canonical form
    /// (a spline face), or a control point, a knot, a frame scalar or the
    /// net's structure did not certify.
    Unread(&'static str),
}

impl From<Unsettled> for CompositeRefusal {
    fn from(why: Unsettled) -> Self {
        Self::Unsettled(why)
    }
}

/// K name: a part of the span read against the surface, metered, as
/// definitely apart from it or not.
const PAST_THE_BAND: &str = "spline_composite_past_the_band";

/// The shortest piece cut, as a share of the span. It is a parameter
/// length and not `geom_core::bisect::tangency_floor`'s metres because
/// the arm holds no bound on the carrier's speed to meter one by. No
/// answer but a refusal rests on it: a piece at the floor that is not
/// clear refuses. `2⁻⁴⁰` leaves a piece about a thousand `f64` steps of
/// its parameter wide on a unit span, so the de Casteljau cuts at its
/// shares stay distinct points.
const FLOOR_SHARE: f64 = 1.0 / 1_099_511_627_776.0;

/// The bisection steps that close on a crossing in its monotone piece:
/// enough to reach `f64` resolution from any piece.
const BISECTIONS: u32 = 64;

/// One piece of the composite: the parameter interval `[a, b]` and the
/// numerator's and denominator's Bernstein rows over it.
struct Piece {
    a: f64,
    b: f64,
    num: Vec<Interval>,
    den: Vec<Interval>,
}

/// What a meeting the walk found is, before it is placed.
enum Meeting {
    /// A crossing at the parameter.
    Crossing(f64),
    /// A touch at a knot: the carrier comes within the band there and
    /// leaves on the side it came from; the refusal the band's reading
    /// there makes.
    Touch(f64, CompositeRefusal),
}

/// An end of a piece, read: definitely past the band on a side, or not,
/// with the band's escalation where the reading fell in its gap.
#[derive(Clone, Copy)]
enum Reading {
    Past(Sign),
    Near(Option<Indeterminate>),
}

impl Reading {
    /// The refusal of a reading the walk needed past the band: the gap's
    /// escalation, or a touch decided in the zero band.
    fn refused(self) -> CompositeRefusal {
        match self {
            Self::Near(gap) => touch_or_tangency(gap, CompositeRefusal::Escalated),
            Self::Past(_) => Unsettled::Tangency.into(),
        }
    }
}

/// **The crossings of a spline span with an analytic face's carrier**:
/// the parameters strictly inside `params` where the carrier crosses
/// `surface`, ascending, each read ON it (module docs). A meeting at an
/// end of the span is not reported: it is the end vertex's. Each is a
/// parameter whose point reads ON the surface, not an enclosure of the
/// true crossing's parameter.
///
/// `cross` takes a scalar of the body into certification arithmetic: it
/// is the door's ([`crate::NurbsLane::surface_crossings`]), so only a
/// scalar that may certify reaches the arm.
///
/// # Errors
///
/// [`CompositeRefusal`], as its variants say.
// `!(a < b)` is deliberate: a NaN end refuses.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
pub fn crossings<T: Decide>(
    net: &NurbsCurve3<T>,
    params: (T, T),
    surface: &Surface<T>,
    band: Band,
    cross: impl Fn(T) -> Interval,
) -> Result<Vec<T>, CompositeRefusal> {
    let (start, end) = (cross(params.0), cross(params.1));
    if !(start.is_certified() && end.is_certified()) {
        return Err(CompositeRefusal::Unread(
            "an end of the span does not certify",
        ));
    }
    let (a, b) = (start.lo(), end.hi());
    if !(a < b) {
        return Err(CompositeRefusal::Unread(
            "the span's ends do not certify forward",
        ));
    }
    let (pieces, divisor) = composite_rows(net, surface, (a, b), &cross)?;
    let floor = (b - a) * FLOOR_SHARE;
    let read = |f: Interval| match decide(PAST_THE_BAND, Margin::of(f / divisor), band) {
        Ok(s @ (Sign::Positive | Sign::Negative)) => Reading::Past(s),
        Ok(Sign::Zero) => Reading::Near(None),
        Err(diag) => Reading::Near(Some(diag)),
    };
    let residual = |t: f64| -> Result<Sign, CompositeRefusal> {
        let p = net.eval(T::from_computed(t));
        decide(
            "spline_composite_root_on",
            Margin::of(crate::implicit_residual(surface, p)),
            band,
        )
        .map_err(CompositeRefusal::Escalated)
    };
    // The knot at which a piece ended within the band, that piece, and
    // its other end's side: read with the piece that starts there.
    let mut open: Option<(f64, Piece, Sign)> = None;
    let mut meetings: Vec<Meeting> = Vec::new();
    let walked = bisect(pieces, |piece: Piece| {
        let visit = || -> Result<Step<Piece, CompositeRefusal>, CompositeRefusal> {
            if matches!(read(hull(&piece.num) / hull(&piece.den)), Reading::Past(_)) {
                return Ok(Step::Settled);
            }
            if one_signed(slope_row(&piece.num, &piece.den).into_iter()).is_none() {
                return cut(piece, floor, &read).map(Step::Halve);
            }
            let (fa, fb) = (end_value(&piece, true), end_value(&piece, false));
            let (ra, rb) = (read(fa), read(fb));
            match (ra, rb) {
                (Reading::Past(l), Reading::Past(r)) if l == r => {}
                (Reading::Past(l), Reading::Past(_)) => {
                    meetings.push(Meeting::Crossing(bisect_row(&piece, l, &residual)?));
                }
                (Reading::Near(_), Reading::Near(_)) => {
                    return cut(piece, floor, &read).map(Step::Halve);
                }
                // Within the band at the span's own end: that end's
                // meeting, `f` being monotone over the piece.
                (Reading::Near(_), Reading::Past(_)) if piece.a == a => {}
                (Reading::Past(_), Reading::Near(_)) if piece.b == b => {}
                (Reading::Past(far), Reading::Near(_)) => {
                    if open.is_some() {
                        return Err(rb.refused());
                    }
                    open = Some((piece.b, piece, far));
                }
                (Reading::Near(_), Reading::Past(far)) => {
                    let Some((k, before, l)) = open.take() else {
                        // The piece before this knot read definitely
                        // past the band there, and this one not: they
                        // part only by their enclosures' width.
                        return Err(Unsettled::Tangency.into());
                    };
                    if k != piece.a {
                        return Err(ra.refused());
                    }
                    meetings.push(if l == far {
                        Meeting::Touch(k, ra.refused())
                    } else {
                        Meeting::Crossing(knot_crossing(&before, &piece, l, &residual)?)
                    });
                }
            }
            Ok(Step::Settled)
        };
        visit().unwrap_or_else(Step::Stop)
    });
    match walked {
        Err(geom_core::bisect::Exhausted) => return Err(Unsettled::Exhausted.into()),
        Ok(Some(refusal)) => return Err(refusal),
        Ok(None) => {}
    }
    if let Some((_, piece, _)) = open {
        return Err(read(end_value(&piece, false)).refused());
    }
    let creases = creases(net, (a, b), &cross)?;
    settle(net, params, band, &creases, meetings, &residual)
}

/// The composite's rows over `[a, b]`, one piece per knot span, and an
/// upper bound on the form's distance divisor over the net's hull
/// (`projected::Meters::divisor`; one for the plane).
fn composite_rows<T: Real>(
    net: &NurbsCurve3<T>,
    surface: &Surface<T>,
    (a, b): (f64, f64),
    cross: &impl Fn(T) -> Interval,
) -> Result<(Vec<Piece>, Interval), CompositeRefusal> {
    let unread = CompositeRefusal::Unread;
    let (origin, frame) = crate::pcurve_cache::projected::chart_frame(surface)
        .ok_or(unread("a spline face has no canonical form"))?;
    let point = |p: Point3<T>| Vec3::new(cross(p.x), cross(p.y), cross(p.z));
    let vector = |v: Vec3<T>| Vec3::new(cross(v.x), cross(v.y), cross(v.z));
    let o = point(origin);
    let e = frame.map(vector);
    let framed: Vec<Vec3<Interval>> = net
        .control()
        .iter()
        .map(|&p| {
            let g = point(p) - o;
            Vec3::new(g.dot(e[0]), g.dot(e[1]), g.dot(e[2]))
        })
        .collect();
    if !framed
        .iter()
        .all(|g| g.x.is_certified() && g.y.is_certified() && g.z.is_certified())
    {
        return Err(unread(
            "a control point or the chart frame does not certify",
        ));
    }
    let coords = [
        framed.iter().map(|g| g.x).collect::<Vec<_>>(),
        framed.iter().map(|g| g.y).collect(),
        framed.iter().map(|g| g.z).collect(),
    ];
    let (form, meters) = match *surface {
        Surface::Plane { .. } => (CanonicalSurface::Plane, Meters::Plane),
        Surface::Cylinder { radius, .. } => {
            let radius = cross(radius);
            (
                CanonicalSurface::Cylinder { radius },
                Meters::Cylinder { radius },
            )
        }
        Surface::Sphere { radius, .. } => {
            let radius = cross(radius);
            (
                CanonicalSurface::Sphere { radius },
                Meters::Sphere { radius },
            )
        }
        Surface::Cone { half_angle, .. } => {
            let (sin, cos) = cross(half_angle).sin_cos();
            (
                CanonicalSurface::Cone {
                    cos2: cos.powi(2),
                    sin2: sin.powi(2),
                },
                // The divisor is the double cone's and reads no nappe.
                Meters::Cone {
                    sin,
                    cos,
                    nappe: crate::offset::Nappe::Opening,
                },
            )
        }
        Surface::Torus {
            major_radius,
            minor_radius,
            ..
        } => {
            let (major, minor) = (cross(major_radius), cross(minor_radius));
            (
                CanonicalSurface::Torus { major, minor },
                Meters::Torus { major, minor },
            )
        }
        Surface::Nurbs(_) | Surface::Approx(_) => {
            return Err(unread("a spline face has no canonical form"));
        }
    };
    // The net's hull in the chart frame bounds every point of the span,
    // so the divisor read over it bounds every point's.
    let divisor = meters
        .divisor(Vec3::new(
            hull(&coords[0]),
            hull(&coords[1]),
            hull(&coords[2]),
        ))
        .abs();
    if !(divisor.is_certified() && divisor.lo() > 0.0) {
        return Err(unread("a scalar of the surface does not certify"));
    }
    let divisor = Interval::from_bounds(divisor.hi(), divisor.hi());
    let ends = [a, b];
    let data = CurveCertData::new(net.knots_f64_interim(), net.weights_f64_interim(), &coords)
        .map_err(|_| unread("the carrier's structure does not certify"))?
        .with_breaks(&ends);
    let composite =
        canonical_composite(&data, &form).map_err(|_| unread("the composite refused the net"))?;
    let breaks = composite.num.breaks();
    let pieces = breaks
        .windows(2)
        .zip(composite.num.spans().iter().zip(composite.den.spans()))
        .filter(|(w, _)| w[0] >= a && w[1] <= b)
        .map(|(w, (num, den))| Piece {
            a: w[0],
            b: w[1],
            num: num.clone(),
            den: den.clone(),
        })
        .collect();
    Ok((pieces, divisor))
}

/// The carrier's creases strictly inside `(a, b)`, each at its held
/// knot's lower end, which is the knot where it is a point.
fn creases<T: Real>(
    net: &NurbsCurve3<T>,
    (a, b): (f64, f64),
    cross: &impl Fn(T) -> Interval,
) -> Result<Vec<f64>, CompositeRefusal> {
    let mut out = Vec::new();
    for u in net.knot_values().crease_values() {
        let u = cross(u);
        if !u.is_certified() {
            return Err(CompositeRefusal::Unread(
                "a knot of the carrier does not certify",
            ));
        }
        if a < u.lo() && u.lo() < b {
            out.push(u.lo());
        }
    }
    Ok(out)
}

/// The hull of a row's coefficients: by the convex hull property, an
/// enclosure of its polynomial over the piece. Refused (NaN) for an
/// empty row or a coefficient that does not certify.
fn hull(row: &[Interval]) -> Interval {
    if row.is_empty() || !row.iter().all(|v| v.is_certified()) {
        return Interval::from_bounds(f64::NAN, f64::NAN);
    }
    let lo = row.iter().map(|v| v.lo()).fold(f64::INFINITY, f64::min);
    let hi = row.iter().map(|v| v.hi()).fold(f64::NEG_INFINITY, f64::max);
    Interval::from_bounds(lo, hi)
}

/// `f` at the piece's start (`at_start`) or end: the ratio of the rows'
/// end coefficients, which a Bernstein row interpolates.
fn end_value(piece: &Piece, at_start: bool) -> Interval {
    let pick = |row: &[Interval]| {
        if at_start { row.first() } else { row.last() }
            .copied()
            .unwrap_or_else(|| Interval::from_bounds(f64::NAN, f64::NAN))
    };
    pick(&piece.num) / pick(&piece.den)
}

/// The sign every value of `values` holds, by its enclosure alone: `None`
/// where one holds zero, does not certify, or there is none.
fn one_signed(values: impl Iterator<Item = Interval>) -> Option<Sign> {
    let mut sign = None;
    for v in values {
        let s = strict_sign(v)?;
        if sign.is_some_and(|held| held != s) {
            return None;
        }
        sign = Some(s);
    }
    sign
}

/// The sign of a certified enclosure wholly on one side of zero.
fn strict_sign(v: Interval) -> Option<Sign> {
    if !v.is_certified() {
        None
    } else if v.lo() > 0.0 {
        Some(Sign::Positive)
    } else if v.hi() < 0.0 {
        Some(Sign::Negative)
    } else {
        None
    }
}

/// An exact small integer as a point enclosure.
fn exact(k: usize) -> Interval {
    #[allow(clippy::cast_precision_loss)]
    let k = k as f64;
    Interval::from_bounds(k, k)
}

/// `C(n, k)`, exact for the degrees a carrier reaches.
fn binomial(n: usize, k: usize) -> usize {
    (0..k).fold(1, |acc, i| acc * (n - i) / (i + 1))
}

/// The derivative's Bernstein row (over the piece's own unit parameter,
/// whose positive scale changes no sign): `n·(c[i+1] − c[i])`.
fn derivative(row: &[Interval]) -> Vec<Interval> {
    let n = exact(row.len().saturating_sub(1));
    row.windows(2).map(|w| n * (w[1] - w[0])).collect()
}

/// The product of two Bernstein rows, of degrees `p` and `q`, as a row
/// of degree `p + q`, each weight a quotient of exact binomials taken in
/// certification arithmetic.
fn product(x: &[Interval], y: &[Interval]) -> Vec<Interval> {
    let (Some(p), Some(q)) = (x.len().checked_sub(1), y.len().checked_sub(1)) else {
        return Vec::new();
    };
    (0..=p + q)
        .map(|k| {
            let lo = k.saturating_sub(q);
            (lo..=k.min(p))
                .map(|i| {
                    let w = exact(binomial(p, i) * binomial(q, k - i)) / exact(binomial(p + q, k));
                    w * x[i] * y[k - i]
                })
                .reduce(|s, t| s + t)
                .unwrap_or_else(|| Interval::from_bounds(0.0, 0.0))
        })
        .collect()
}

/// `N′D − ND′`, whose sign is `f′`'s (`D > 0`), as one Bernstein row of
/// degree `n + m − 1`.
fn slope_row(num: &[Interval], den: &[Interval]) -> Vec<Interval> {
    let (dn, dd) = (derivative(num), derivative(den));
    let left = product(&dn, den);
    let right = product(num, &dd);
    match (left.is_empty(), right.is_empty()) {
        (false, false) => left.iter().zip(&right).map(|(l, r)| *l - *r).collect(),
        (false, true) => left,
        (true, false) => right
            .iter()
            .map(|r| Interval::from_bounds(0.0, 0.0) - *r)
            .collect(),
        (true, true) => Vec::new(),
    }
}

/// The share of `[a, b]` at which `c` lies, enclosed.
fn share(piece: &Piece, c: f64) -> Interval {
    let point = |x: f64| Interval::from_bounds(x, x);
    let (a, b) = (point(piece.a), point(piece.b));
    (point(c) - a) / (b - a)
}

/// De Casteljau's split of `row` at the enclosed share `s`: the rows over
/// the two parts, each enclosing the true rows at every share `s` holds.
fn split_row(row: &[Interval], s: Interval) -> (Vec<Interval>, Vec<Interval>) {
    let n = row.len();
    let mut work = row.to_vec();
    let mut left = Vec::with_capacity(n);
    let mut right = Vec::with_capacity(n);
    let one_minus = Interval::one() - s;
    for level in 0..n {
        left.push(work[0]);
        right.push(work[n - 1 - level]);
        for i in 0..n - 1 - level {
            work[i] = work[i] * one_minus + work[i + 1] * s;
        }
    }
    right.reverse();
    (left, right)
}

/// `f` at the share `s` of the piece.
fn value_at(piece: &Piece, s: Interval) -> Interval {
    let at = |row: &[Interval]| {
        split_row(row, s)
            .0
            .last()
            .copied()
            .unwrap_or_else(|| Interval::from_bounds(f64::NAN, f64::NAN))
    };
    at(&piece.num) / at(&piece.den)
}

/// A piece not settled by its readings, cut at the first of
/// `half_angle`'s shares (`SPLITS`) where `f` is definitely past the
/// band, its parts in walk order. Every cut is such a point, so no
/// crossing found depends on the shares' values; where a piece reaches
/// the floor, and so whether a near pass refuses or parts from a
/// crossing, can.
fn cut(
    piece: Piece,
    floor: f64,
    read: &impl Fn(Interval) -> Reading,
) -> Result<[Piece; 2], CompositeRefusal> {
    if piece.b - piece.a <= floor {
        return Err(Unsettled::Floor.into());
    }
    // The first share read in the band's gap, which a piece that reads in
    // band at every share refuses on.
    let mut gap = None;
    let Some((c, s)) = SPLITS.iter().find_map(|&k| {
        let c = piece.a + (piece.b - piece.a) * k;
        let s = share(&piece, c);
        if !(piece.a < c && c < piece.b) {
            return None;
        }
        match read(value_at(&piece, s)) {
            Reading::Past(_) => Some((c, s)),
            Reading::Near(Some(diag)) => {
                gap.get_or_insert(diag);
                None
            }
            Reading::Near(None) => None,
        }
    }) else {
        // Within the band at every share: the carrier runs along the
        // surface here, or crosses it twice within the band.
        return Err(touch_or_tangency(gap, CompositeRefusal::Escalated));
    };
    let (num_l, num_r) = split_row(&piece.num, s);
    let (den_l, den_r) = split_row(&piece.den, s);
    Ok([
        Piece {
            a: piece.a,
            b: c,
            num: num_l,
            den: den_l,
        },
        Piece {
            a: c,
            b: piece.b,
            num: num_r,
            den: den_r,
        },
    ])
}

/// The crossing of a piece over which `f` is monotone and changes sign,
/// `left` its side at the piece's start: bisected on `f` while its
/// enclosure reads a sign, and on the surface's residual at the point
/// where it does not, until the point reads ON (`residual` deciding
/// Zero) or the bracket closes on adjacent `f64`s.
fn bisect_row(
    piece: &Piece,
    left: Sign,
    residual: &impl Fn(f64) -> Result<Sign, CompositeRefusal>,
) -> Result<f64, CompositeRefusal> {
    let (mut lo, mut hi) = (piece.a, piece.b);
    for _ in 0..BISECTIONS {
        let c = lo + (hi - lo) * 0.5;
        if !(lo < c && c < hi) {
            break;
        }
        let side = match strict_sign(value_at(piece, share(piece, c))) {
            Some(s) => s,
            None => match residual(c)? {
                Sign::Zero => return Ok(c),
                s => s,
            },
        };
        if side == left {
            lo = c;
        } else {
            hi = c;
        }
    }
    Ok(lo + (hi - lo) * 0.5)
}

/// The crossing of a knot's stretch: `before` ends at the knot within
/// the band, its start on the side `left`, and `after` starts there,
/// ending on the other side. It is in whichever piece's ends read
/// opposite strict signs, or at the knot itself.
fn knot_crossing(
    before: &Piece,
    after: &Piece,
    left: Sign,
    residual: &impl Fn(f64) -> Result<Sign, CompositeRefusal>,
) -> Result<f64, CompositeRefusal> {
    match strict_sign(end_value(after, true)) {
        Some(s) if s == left => bisect_row(after, s, residual),
        Some(_) => bisect_row(before, left, residual),
        None => Ok(after.a),
    }
}

/// The meetings found, each placed in metres: against the creases, then
/// on the surface, then against the span's ends and the crossing before
/// it.
fn settle<T: Decide>(
    net: &NurbsCurve3<T>,
    (t0, t1): (T, T),
    band: Band,
    creases: &[f64],
    meetings: Vec<Meeting>,
    residual: &impl Fn(f64) -> Result<Sign, CompositeRefusal>,
) -> Result<Vec<T>, CompositeRefusal> {
    let apart =
        |row, p: Point3<T>, q: Point3<T>| match decide(row, Margin::of((p - q).norm()), band) {
            Ok(Sign::Zero) => Ok(false),
            Ok(_) => Ok(true),
            Err(diag) => Err(CompositeRefusal::Escalated(diag)),
        };
    let ends = [net.eval(t0), net.eval(t1)];
    let creases: Vec<Point3<T>> = creases
        .iter()
        .map(|&k| net.eval(T::from_computed(k)))
        .collect();
    let mut roots: Vec<(T, Point3<T>)> = Vec::with_capacity(meetings.len());
    for meeting in meetings {
        let (Meeting::Crossing(t) | Meeting::Touch(t, _)) = meeting;
        let p = net.eval(T::from_computed(t));
        for &crease in &creases {
            if !apart("spline_composite_root_at_crease", p, crease)? {
                return Err(Unsettled::Crease.into());
            }
        }
        if let Meeting::Touch(_, refusal) = meeting {
            return Err(refusal);
        }
        // Definitely off at the located crossing: the residual's step
        // between two adjacent scalars is past the band.
        if residual(t)? != Sign::Zero {
            return Err(Unsettled::Meter.into());
        }
        let mut at_end = false;
        for end in ends {
            at_end |= !apart("spline_composite_root_at_end", p, end)?;
        }
        if at_end {
            continue;
        }
        if let Some(&(_, before)) = roots.last()
            && !apart("spline_composite_roots_apart", p, before)?
        {
            return Err(Unsettled::Tangency.into());
        }
        roots.push((T::from_computed(t), p));
    }
    Ok(roots.into_iter().map(|(t, _)| t).collect())
}

#[cfg(test)]
mod rows;
