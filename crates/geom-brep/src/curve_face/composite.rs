//! **The spline span × analytic face arm** of the curve × face table
//! ([`super::Door::SplineComposite`]): the face's canonical residual
//! composed along the span's NURBS carrier, the ring composite `f ∘ C`
//! ([`geom_core::spline::compose::canonical_composite`]), read piece by
//! piece.
//!
//! The carrier's net is written in the surface's chart frame in
//! certification arithmetic, so the composite is a rational Bernstein
//! form whose denominator is a power of the carrier's weight function:
//! positive, so the composite has its numerator's zeros and signs, and
//! the ladder reads the numerator's rows alone. **A piece never straddles
//! a knot**: the pieces start as the carrier's knot spans cut at the
//! span's ends, and every later piece is a part of one, so each row is
//! one polynomial and a derivative read on it is the derivative there.
//! On a piece:
//!
//! - **clear**: every coefficient one-signed, so the form is nowhere
//!   zero on it;
//! - **monotone**: every forward difference of the row one-signed, so
//!   the form has at most one zero on it, and one exactly where its ends'
//!   signs differ; that zero is bisected on the row and must then read
//!   ON the surface ([`crate::implicit_residual`]);
//! - otherwise the piece is cut where the row's sign is definite; a piece
//!   too short to cut, or with no cut whose sign reads definite, is a
//!   tangency, refused.
//!
//! A knot where the row's own value holds zero is a meeting at the knot.
//! At a **crease** (an interior knot of multiplicity at least the degree,
//! `KnotStructure::crease_runs`) the carrier has no tangent and the
//! meeting refuses; elsewhere it is a crossing when the pieces either side
//! are monotone with far ends of opposite signs, and a tangency
//! otherwise. A located crossing within the band of a crease refuses the
//! same way. One within the band of an end of the span is that end's
//! meeting, read by the caller, and two within the band of each other
//! cannot be told apart, so they refuse.

use geom::{NurbsCurve3, Surface};
use geom_core::k_stats::decide;
use geom_core::spline::compose::{CanonicalSurface, CurveCertData, canonical_composite};
use geom_core::{Band, Bounds, Decide, Indeterminate, Interval, Margin, Point3, Real, Sign, Vec3};

use crate::roots::Unsettled;

/// Why [`crossings`] gave no answer.
#[derive(Clone, Copy, Debug)]
pub enum CompositeRefusal {
    /// A decision the arm must read definite fell in the band's gap:
    /// whether a located crossing lies ON the surface, or where it stands
    /// against an end of the span, a crease or the crossing before it.
    Escalated(Indeterminate),
    /// The ladder could not settle the span: [`Unsettled::Tangency`] (a
    /// piece too short to cut, or with no cut whose sign reads definite,
    /// and neither clear nor monotone; a zero at a knot with one sign
    /// either side; or two crossings within the band of each other),
    /// [`Unsettled::Crease`], [`Unsettled::SignInBand`] (a located
    /// crossing reads off the surface) or [`Unsettled::Exhausted`].
    Unsettled(Unsettled),
    /// The composite could not be formed: the face has no canonical form
    /// (a spline face), or a control point, a frame scalar or the net's
    /// structure did not certify.
    Unread(&'static str),
}

impl From<Unsettled> for CompositeRefusal {
    fn from(why: Unsettled) -> Self {
        Self::Unsettled(why)
    }
}

/// The most pieces one span is read in before the walk refuses
/// [`Unsettled::Exhausted`].
const PIECE_BUDGET: usize = 4096;

/// The shortest piece cut, as a share of the span: below it a piece
/// neither clear nor monotone is a tangency.
const FLOOR_SHARE: f64 = 1.0 / 1_099_511_627_776.0;

/// The bisection steps that close on a crossing in its monotone piece:
/// enough to reach `f64` resolution from any piece.
const BISECTIONS: u32 = 64;

/// Where a piece is cut, as shares of its width, tried in order until
/// the row's sign there is definite. No answer depends on their values,
/// since every cut is a point whose sign was read definite; none is
/// `1/2`, so a pose symmetric about a piece does not put its tangency on
/// the first cut.
const SPLITS: [f64; 7] = [0.472_135_954_999_579_4, 0.3, 0.618, 0.2, 0.8, 0.4, 0.55];

/// One piece of the composite: the parameter interval `[a, b]` and the
/// numerator's Bernstein row over it.
struct Piece {
    a: f64,
    b: f64,
    row: Vec<Interval>,
}

/// **The crossings of a spline span with an analytic face's carrier**:
/// the parameters strictly inside `params` where the carrier crosses
/// `surface`, ascending, each read ON it (module docs). A meeting at an
/// end of the span is not reported: it is the end vertex's.
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
    let (a, b) = (cross(params.0).lo(), cross(params.1).hi());
    if !(a < b) {
        return Err(CompositeRefusal::Unread(
            "the span's ends do not certify forward",
        ));
    }
    let pieces = numerator_rows(net, surface, (a, b), &cross)?;
    let creases = creases(net, (a, b), &cross);
    let floor = (b - a) * FLOOR_SHARE;
    // Each knot inside the span where a row's value holds zero, and the
    // far signs of the monotone pieces either side, left then right.
    let mut knot_zeros: Vec<(f64, Option<Sign>)> = Vec::new();
    let mut found: Vec<f64> = Vec::new();
    let mut stack: Vec<Piece> = pieces.into_iter().rev().collect();
    let mut read = 0;
    while let Some(piece) = stack.pop() {
        read += 1;
        if read > PIECE_BUDGET {
            return Err(Unsettled::Exhausted.into());
        }
        let row = &piece.row;
        if one_signed(row.iter().copied()).is_some() {
            continue;
        }
        if one_signed(row.windows(2).map(|w| w[1] - w[0])).is_none() {
            stack.extend(cut(piece, floor)?);
            continue;
        }
        let (Some(&first), Some(&last)) = (row.first(), row.last()) else {
            return Err(CompositeRefusal::Unread("a composite row is empty"));
        };
        match (strict_sign(first), strict_sign(last)) {
            (Some(l), Some(r)) if l == r => {}
            (Some(l), Some(_)) => found.push(bisect_row(&piece, l)),
            (None, None) => return Err(Unsettled::Tangency.into()),
            // A zero at one end, so the monotone piece holds no other:
            // the span's end is the end vertex's meeting, a knot's is the
            // knot's (module docs).
            (None, Some(far)) => {
                if piece.a != a {
                    knot_zero(
                        &mut knot_zeros,
                        &mut found,
                        &creases,
                        piece.a,
                        Some(far),
                        false,
                    )?;
                }
            }
            (Some(far), None) => {
                if piece.b != b {
                    knot_zero(
                        &mut knot_zeros,
                        &mut found,
                        &creases,
                        piece.b,
                        Some(far),
                        true,
                    )?;
                }
            }
        }
    }
    if knot_zeros.iter().any(|(_, open)| open.is_some()) {
        // A knot zero whose right-hand piece never read monotone.
        return Err(Unsettled::Tangency.into());
    }
    settle(net, params, surface, band, &creases, found)
}

/// The composite's numerator rows over `[a, b]`, one per knot span.
fn numerator_rows<T: Real>(
    net: &NurbsCurve3<T>,
    surface: &Surface<T>,
    (a, b): (f64, f64),
    cross: &impl Fn(T) -> Interval,
) -> Result<Vec<Piece>, CompositeRefusal> {
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
    let form = match *surface {
        Surface::Plane { .. } => CanonicalSurface::Plane,
        Surface::Cylinder { radius, .. } => CanonicalSurface::Cylinder {
            radius: cross(radius),
        },
        Surface::Sphere { radius, .. } => CanonicalSurface::Sphere {
            radius: cross(radius),
        },
        Surface::Cone { half_angle, .. } => {
            let (sin, cos) = cross(half_angle).sin_cos();
            CanonicalSurface::Cone {
                cos2: cos.powi(2),
                sin2: sin.powi(2),
            }
        }
        Surface::Torus {
            major_radius,
            minor_radius,
            ..
        } => CanonicalSurface::Torus {
            major: cross(major_radius),
            minor: cross(minor_radius),
        },
        Surface::Nurbs(_) | Surface::Approx(_) => {
            return Err(unread("a spline face has no canonical form"));
        }
    };
    let certified = match form {
        CanonicalSurface::Plane => true,
        CanonicalSurface::Cylinder { radius } | CanonicalSurface::Sphere { radius } => {
            radius.is_certified()
        }
        CanonicalSurface::Cone { cos2, sin2 } => cos2.is_certified() && sin2.is_certified(),
        CanonicalSurface::Torus { major, minor } => major.is_certified() && minor.is_certified(),
    };
    if !certified {
        return Err(unread("a scalar of the surface does not certify"));
    }
    let ends = [a, b];
    let (knots, weights) = crate::pcurve_cache::projected::composite_structure(net);
    let data = CurveCertData::new(knots, weights, &coords)
        .map_err(|_| unread("the carrier's structure does not certify"))?
        .with_breaks(&ends);
    let composite =
        canonical_composite(&data, &form).map_err(|_| unread("the composite refused the net"))?;
    if !composite
        .den
        .span_hulls()
        .iter()
        .all(|d| d.is_certified() && d.lo() > 0.0)
    {
        return Err(unread(
            "the composite's denominator is not certified positive",
        ));
    }
    let breaks = composite.num.breaks();
    Ok(breaks
        .windows(2)
        .zip(composite.num.spans())
        .filter(|(w, _)| w[0] >= a && w[1] <= b)
        .map(|(w, row)| Piece {
            a: w[0],
            b: w[1],
            row: row.clone(),
        })
        .collect())
}

/// The carrier's creases strictly inside `(a, b)`, each at its held
/// knot's lower end, which is the knot where it is a point.
fn creases<T: Real>(
    net: &NurbsCurve3<T>,
    (a, b): (f64, f64),
    cross: &impl Fn(T) -> Interval,
) -> Vec<f64> {
    let knots = net.knot_values();
    let values: Vec<f64> = knots.runs().map(|(u, _)| cross(u).lo()).collect();
    knots
        .structure()
        .crease_runs()
        .map(|r| values[r])
        .filter(|&k| a < k && k < b)
        .collect()
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

/// The row's value at the share `s`; `None` for an empty row.
fn value_at(row: &[Interval], s: Interval) -> Option<Interval> {
    let (left, _) = split_row(row, s);
    left.last().copied()
}

/// A piece neither clear nor monotone, cut where its row's sign is
/// definite, its parts in walk order for a stack (right first).
fn cut(piece: Piece, floor: f64) -> Result<[Piece; 2], CompositeRefusal> {
    if piece.b - piece.a <= floor {
        return Err(Unsettled::Tangency.into());
    }
    let Some((c, s)) = SPLITS.iter().find_map(|&k| {
        let c = piece.a + (piece.b - piece.a) * k;
        let s = share(&piece, c);
        (piece.a < c && c < piece.b && value_at(&piece.row, s).and_then(strict_sign).is_some())
            .then_some((c, s))
    }) else {
        // The row is zero to its rounding at every cut: the carrier lies
        // on the surface's form there, which only a tangency or a run
        // along the surface does.
        return Err(Unsettled::Tangency.into());
    };
    let (left, right) = split_row(&piece.row, s);
    Ok([
        Piece {
            a: c,
            b: piece.b,
            row: right,
        },
        Piece {
            a: piece.a,
            b: c,
            row: left,
        },
    ])
}

/// The zero of a monotone piece whose ends' signs differ, `left` its
/// sign at the piece's start: bisected on the row to a point where the
/// row's value holds zero, or to adjacent `f64`s.
fn bisect_row(piece: &Piece, left: Sign) -> f64 {
    let (mut lo, mut hi) = (piece.a, piece.b);
    for _ in 0..BISECTIONS {
        let c = lo + (hi - lo) * 0.5;
        if !(lo < c && c < hi) {
            break;
        }
        match value_at(&piece.row, share(piece, c)).and_then(strict_sign) {
            Some(s) if s == left => lo = c,
            Some(_) => hi = c,
            None => return c,
        }
    }
    lo + (hi - lo) * 0.5
}

/// A row's zero at the knot `k`, read from the monotone piece on one side
/// of it (`before`: the piece ends there), `far` that piece's sign at its
/// other end. The right-hand reading settles it.
fn knot_zero(
    knot_zeros: &mut Vec<(f64, Option<Sign>)>,
    found: &mut Vec<f64>,
    creases: &[f64],
    k: f64,
    far: Option<Sign>,
    before: bool,
) -> Result<(), CompositeRefusal> {
    if creases.contains(&k) {
        return Err(Unsettled::Crease.into());
    }
    if before {
        knot_zeros.push((k, far));
        return Ok(());
    }
    let Some(entry) = knot_zeros.iter_mut().find(|(at, _)| *at == k) else {
        // The piece before the knot read clear or met it with a definite
        // sign, which a row holding zero there cannot do.
        return Err(Unsettled::Tangency.into());
    };
    let left = entry.1.take();
    match (left, far) {
        (Some(l), Some(r)) if l != r => {
            found.push(k);
            Ok(())
        }
        _ => Err(Unsettled::Tangency.into()),
    }
}

/// The located zeros, each read ON the surface and placed against the
/// span's ends, the creases and the crossing before it, in metres.
fn settle<T: Decide>(
    net: &NurbsCurve3<T>,
    (t0, t1): (T, T),
    surface: &Surface<T>,
    band: Band,
    creases: &[f64],
    found: Vec<f64>,
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
    let mut roots: Vec<(T, Point3<T>)> = Vec::with_capacity(found.len());
    for t in found {
        let t = T::from_computed(t);
        let p = net.eval(t);
        match decide(
            "spline_composite_root_on",
            Margin::of(crate::implicit_residual(surface, p)),
            band,
        ) {
            Ok(Sign::Zero) => {}
            Ok(_) => return Err(Unsettled::SignInBand.into()),
            Err(diag) => return Err(CompositeRefusal::Escalated(diag)),
        }
        let mut at_end = false;
        for end in ends {
            at_end |= !apart("spline_composite_root_at_end", p, end)?;
        }
        if at_end {
            continue;
        }
        for &crease in &creases {
            if !apart("spline_composite_root_at_crease", p, crease)? {
                return Err(Unsettled::Crease.into());
            }
        }
        if let Some(&(_, before)) = roots.last()
            && !apart("spline_composite_roots_apart", p, before)?
        {
            return Err(Unsettled::Tangency.into());
        }
        roots.push((t, p));
    }
    Ok(roots.into_iter().map(|(t, _)| t).collect())
}

#[cfg(test)]
mod rows;
