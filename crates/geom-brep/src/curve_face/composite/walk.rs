//! **The ring composite's walk at the scalar** ([`method`]): the module
//! docs' reading of `f ∘ C` (`super`), every quantity computed in the
//! scalar's own arithmetic and every verdict a [`Decide`] sign.
//!
//! The carrier's net is written in the surface's chart frame and cut to
//! its Bézier pieces by Boehm's insertion at the scalar
//! ([`crate::pcurve_cache::spline_incidence::refined`]), each held as
//! its parameter's ends and the composite's numerator and denominator
//! rows ([`CanonicalSurface::piece_rows`], `geom_core::spline::canonical_rows`). At `f64` the rows are real
//! numbers, with no rounding charged or padded (DESIGN
//! `[simple.conditioned]`); at an interval scalar they are enclosures.
//! Every cut is at a constant share of a piece (a halving, or one of
//! [`SPLITS`]), entered as a constant: a cut's parameter is computed
//! from the knots, never re-entered from an `f64` place. Each piece
//! carries an `f64` shadow of its parameter ends, which orders and
//! floors the walk and decides nothing.
//!
//! The verdicts, on a piece whose weight row is shown positive (so the
//! composite lies in the hull of its coefficient ratios `Nᵢ/Dᵢ`):
//!
//! - **clear** where every ratio, metered, is decided past the band on
//!   one side (`spline_composite_past_the_band`): the carrier is
//!   definitely apart from the surface over the piece;
//! - **monotone** where every coefficient of `N′D − ND′` is proved one
//!   side of zero (`spline_composite_slope`, [`prove`]): the cut's own
//!   inequality, which no tolerance governs;
//! - a crossing of a monotone piece whose ends read past the band on
//!   opposite sides is halved on `f`'s exact sign at the halving
//!   (`spline_composite_side`), and on the surface's residual at the
//!   point where that sign is not proved, until the point reads ON
//!   (`spline_composite_root_on`).
//!
//! **A span end within the band** is read once, here, on the piece that
//! holds it: where `f` there is proved opposite the piece's far side,
//! the crossing between is located and is a crossing like any other;
//! otherwise the end's own point is read against the surface, and is
//! the meeting (a root at the end) where it reads ON. Where an interval
//! scalar cannot prove `f`'s sign there, the end's stretch (to where
//! `f` is proved on the far side, so every meeting lies on it) is a
//! meeting at the end where its chord is within the band, and a window
//! otherwise. Every root found is placed
//! against the span's ends by the table's own placement
//! ([`super::super::cross::place`]), so no end is decided twice.

use geom::{NurbsCurve3, Surface};
use geom_core::bisect::{Budget, PIECE_BUDGET, Step, bisect_within};
use geom_core::k_stats::{decide, prove};
use geom_core::spline::compose::CanonicalSurface;
use geom_core::{Band, Decide, Indeterminate, InfSpeed, Margin, Point3, Sign, SupSpeed, Vec3};

use super::CompositeRefusal;
use crate::curve_face::cross::Walked;
use crate::curve_pair::{MeetFault, Span};
use crate::half_angle::SPLITS;
use crate::ladder::{Answering, Rung, climb};
use crate::pcurve_cache::spline_incidence::{Control, refined};
use crate::roots::{Unsettled, touch_or_tangency};

/// A coefficient ratio, metered, against the band.
const PAST_THE_BAND: &str = "spline_composite_past_the_band";
/// A coefficient of `N′D − ND′`, proved one side of zero.
const SLOPE: &str = "spline_composite_slope";
/// `f` at a halving or an end, proved one side of zero.
const SIDE: &str = "spline_composite_side";
/// A weight coefficient, proved positive.
const WEIGHT: &str = "spline_composite_weight";
/// The surface's residual at a located crossing or an end.
const ROOT_ON: &str = "spline_composite_root_on";
/// A meeting's point against a crease's.
const AT_CREASE: &str = "spline_composite_root_at_crease";
/// A near end's stretch, by its chord.
const STRETCH: &str = "spline_composite_end_stretch";

/// The shortest piece cut, as a share of the span (the module docs of
/// `super`).
const FLOOR_SHARE: f64 = 1.0 / 1_099_511_627_776.0;

/// The halvings that close on a crossing in its monotone piece.
const BISECTIONS: u32 = 64;

/// The span's least and greatest speed ([`Span::speeds`]).
type Speeds<T> = (InfSpeed<T>, SupSpeed<T>);

/// One piece of the composite: its parameter's ends at the scalar, their
/// `f64` shadow, and the numerator's and denominator's Bernstein rows.
#[derive(Clone)]
struct Piece<T> {
    a: T,
    b: T,
    shadow: (f64, f64),
    num: Vec<T>,
    den: Vec<T>,
}

/// An end of a piece, read: definitely past the band on a side, or not,
/// with the band's escalation where the reading fell in its gap.
#[derive(Clone, Copy)]
enum Reading {
    Past(Sign),
    Near(Option<Indeterminate>),
}

impl Reading {
    /// The refusal of a reading the walk needed past the band.
    fn refused(self) -> CompositeRefusal {
        match self {
            Self::Near(gap) => touch_or_tangency(gap, CompositeRefusal::Escalated),
            Self::Past(_) => Unsettled::Tangency.into(),
        }
    }
}

/// What the walk found, before it is placed.
enum Meeting<T> {
    /// A crossing located at the parameter, to be read ON.
    Crossing(T),
    /// A touch at a knot, and the refusal the band's reading there makes.
    Touch(T, CompositeRefusal),
    /// A span end within the band: its meeting, where its point reads ON.
    End(T),
    /// A span end's stretch, from the end to where `f` reads past the
    /// band, that no reading places, and why.
    Stretch((T, T), MeetFault),
    /// A near end whose meeting lies past the end, beyond the band: no
    /// meeting of the span.
    Past,
}

/// **The ring composite's method** for the curve × face table: the
/// span's crossings of `surface`, each read ON it, and a root at each
/// end the band reads there; a refusal is one window over the span,
/// its fault [`CompositeRefusal::fault`].
pub(in crate::curve_face) fn method<T: Decide>(
    span: &Span<'_, T>,
    surface: &Surface<T>,
    band: Band,
) -> Walked<T> {
    let params = span.params();
    let geom::Curve3::Nurbs(net) = span.carrier() else {
        return Walked::whole(params, MeetFault::Desync);
    };
    read(
        net,
        crate::curve_face::ascending(params),
        span.speeds(),
        surface,
        band,
    )
    .unwrap_or_else(|refusal| Walked::whole(params, refusal.fault()))
}

/// The walk over `[t0, t1]` (the module docs).
fn read<T: Decide>(
    net: &NurbsCurve3<T>,
    (t0, t1): (T, T),
    speeds: Speeds<T>,
    surface: &Surface<T>,
    band: Band,
) -> Result<Walked<T>, CompositeRefusal> {
    let shadow = (t0.value_channel(), t1.value_channel());
    // `!(a < b)` is deliberate: a NaN end refuses.
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    if !(shadow.0 < shadow.1) {
        return Err(CompositeRefusal::Unread(
            "the span's ends do not read forward",
        ));
    }
    let (pieces, divisor) = pieces(net, (t0, t1), shadow, surface)?;
    let floor = (shadow.1 - shadow.0) * FLOOR_SHARE;
    let read = |v: T| match decide(PAST_THE_BAND, Margin::of(v / divisor), band) {
        Ok(s @ (Sign::Positive | Sign::Negative)) => Reading::Past(s),
        Ok(Sign::Zero) => Reading::Near(None),
        Err(diag) => Reading::Near(Some(diag)),
    };
    let off = |t: T| -> (Point3<T>, Margin<T>) {
        let p = net.eval(t);
        (p, Margin::of(crate::implicit_residual(surface, p)))
    };
    // The knot at which a piece ended within the band, that piece, and
    // its other end's side: read with the piece that starts there.
    let mut open: Option<(f64, Piece<T>, Sign)> = None;
    let mut meetings: Vec<Meeting<T>> = Vec::new();
    let mut budget = Budget::new(PIECE_BUDGET.saturating_add(pieces.len()));
    let walked = bisect_within(pieces, &mut budget, |piece: Piece<T>| {
        let visit = || -> Result<Step<Piece<T>, CompositeRefusal>, CompositeRefusal> {
            if clear(&piece, &read) {
                return Ok(Step::Settled);
            }
            if one_signed(&slope_row(&piece.num, &piece.den)).is_none() {
                return cut(piece, floor, &read).map(Step::Halve);
            }
            let (ra, rb) = (
                read(end_value(&piece, true)),
                read(end_value(&piece, false)),
            );
            match (ra, rb) {
                (Reading::Past(l), Reading::Past(r)) if l == r => {}
                (Reading::Past(l), Reading::Past(_)) => {
                    meetings.push(Meeting::Crossing(bisect_row(&piece, l, &off, band)?));
                }
                (Reading::Near(_), Reading::Near(_)) => {
                    return cut(piece, floor, &read).map(Step::Halve);
                }
                (Reading::Near(_), Reading::Past(far)) if piece.shadow.0 == shadow.0 => {
                    meetings.push(near_end(&piece, true, far, (&off, speeds), net, band)?);
                }
                (Reading::Past(far), Reading::Near(_)) if piece.shadow.1 == shadow.1 => {
                    meetings.push(near_end(&piece, false, far, (&off, speeds), net, band)?);
                }
                (Reading::Past(far), Reading::Near(_)) => {
                    if open.is_some() {
                        return Err(rb.refused());
                    }
                    open = Some((piece.shadow.1, piece, far));
                }
                (Reading::Near(_), Reading::Past(far)) => {
                    let Some((k, before, l)) = open.take() else {
                        // The piece before this knot read definitely
                        // past the band there, and this one not: they
                        // part only by their rows' rounding.
                        return Err(Unsettled::Tangency.into());
                    };
                    if k != piece.shadow.0 {
                        return Err(ra.refused());
                    }
                    meetings.push(if l == far {
                        Meeting::Touch(piece.a, ra.refused())
                    } else {
                        Meeting::Crossing(knot_crossing(&before, &piece, l, &off, band)?)
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
    settle(net, shadow, band, meetings, &off)
}

/// The carrier's Bézier pieces over `[t0, t1]`, each with its composite
/// rows, and an upper bound on the form's distance divisor over the
/// net's hull (one on the plane).
fn pieces<T: Decide>(
    net: &NurbsCurve3<T>,
    (t0, t1): (T, T),
    (s0, s1): (f64, f64),
    surface: &Surface<T>,
) -> Result<(Vec<Piece<T>>, T), CompositeRefusal> {
    let unread = CompositeRefusal::Unread;
    let degenerate = CompositeRefusal::FaceDegenerate;
    let (origin, [u_ref, _, axis]) = crate::pcurve_cache::projected::chart_frame(surface)
        .ok_or(unread("a spline face has no canonical form"))?;
    let e2 = axis.normalize();
    let e0 = (u_ref - e2 * u_ref.dot(e2)).normalize();
    let e = [e0, e2.cross(e0), e2];
    if e.iter()
        .any(|v| v.x.is_poison() || v.y.is_poison() || v.z.is_poison())
    {
        return Err(degenerate("the face's chart frame does not read"));
    }
    let (form, divisor) = form_and_divisor(surface, net.control(), origin, e)?;
    if !matches!(
        prove(PAST_THE_BAND, Margin::sign_of(divisor)),
        Ok(Sign::Positive)
    ) {
        return Err(degenerate(
            "the form's distance divisor is not shown positive",
        ));
    }
    let row: Vec<Control<T>> = net
        .control()
        .iter()
        .zip(net.weights())
        .map(|(&p, &w)| {
            let g = p - origin;
            (
                [g.dot(e[0]) * w, g.dot(e[1]) * w, g.dot(e[2]) * w, w],
                [0.0; 4],
            )
        })
        .collect();
    let knots = net.knot_values();
    let p = knots.degree();
    let runs: Vec<T> = knots.breakpoints().collect();
    let row = refined(knots, row, false)
        .filter(|row| p > 0 && runs.len() >= 2 && row.len() == (runs.len() - 1) * p + 1)
        .ok_or(unread("the carrier does not cut to its Bézier pieces"))?;
    let mut out = Vec::new();
    for (j, ends) in runs.windows(2).enumerate() {
        let (k0, k1) = (ends[0], ends[1]);
        let (v0, v1) = (k0.value_channel(), k1.value_channel());
        if !(v1 > s0 && v0 < s1) {
            continue;
        }
        let mut rows: [Vec<T>; 4] =
            core::array::from_fn(|c| row[j * p..=j * p + p].iter().map(|h| h.0[c]).collect());
        let (mut a, mut b, mut shadow) = (k0, k1, (v0, v1));
        if v0 < s0 {
            let s = (t0 - a) / (b - a);
            rows = rows.map(|r| split_row(&r, s).1);
            (a, shadow.0) = (t0, s0);
        }
        if v1 > s1 {
            let s = (t1 - a) / (b - a);
            rows = rows.map(|r| split_row(&r, s).0);
            (b, shadow.1) = (t1, s1);
        }
        let [g0, g1, g2, w] = rows;
        if !w
            .iter()
            .all(|&w| matches!(prove(WEIGHT, Margin::sign_of(w)), Ok(Sign::Positive)))
        {
            return Err(unread("a weight of the carrier is not shown positive"));
        }
        let (num, den) = form.piece_rows([&g0, &g1, &g2], &w);
        out.push(Piece {
            a,
            b,
            shadow,
            num,
            den,
        });
    }
    if out.is_empty() {
        return Err(unread("the span holds no piece"));
    }
    Ok((out, divisor))
}

/// The surface's canonical form at the scalar, and an upper bound on its
/// distance divisor (`projected::Meters::divisor`) over the box of the
/// net's points in the chart frame: each coordinate's greatest magnitude
/// `M`, read into the divisor's terms where each only grows with it. On
/// the torus both tube distances are at most `F = (ρ_M + R)² + z_M²`, so
/// the divisor is at most `(√F + r)·(F + r²)`.
fn form_and_divisor<T: Decide>(
    surface: &Surface<T>,
    control: &[Point3<T>],
    origin: Point3<T>,
    e: [Vec3<T>; 3],
) -> Result<(CanonicalSurface<T>, T), CompositeRefusal> {
    let mut m = [T::zero(); 3];
    for &p in control {
        let g = p - origin;
        for (k, axis) in e.iter().enumerate() {
            m[k] = m[k].max(g.dot(*axis).abs());
        }
    }
    let rho = (m[0].powi(2) + m[1].powi(2)).sqrt();
    let norm = (rho.powi(2) + m[2].powi(2)).sqrt();
    Ok(match *surface {
        Surface::Plane { .. } => (CanonicalSurface::Plane, T::one()),
        Surface::Cylinder { radius, .. } => (CanonicalSurface::Cylinder { radius }, rho + radius),
        Surface::Sphere { radius, .. } => (CanonicalSurface::Sphere { radius }, norm + radius),
        Surface::Cone { half_angle, .. } => {
            let (sin, cos) = half_angle.sin_cos();
            (
                CanonicalSurface::Cone {
                    cos2: cos.powi(2),
                    sin2: sin.powi(2),
                },
                rho * cos + m[2] * sin,
            )
        }
        Surface::Torus {
            major_radius,
            minor_radius,
            ..
        } => {
            let outer = (rho + major_radius.abs()).powi(2) + m[2].powi(2);
            (
                CanonicalSurface::Torus {
                    major: major_radius,
                    minor: minor_radius,
                },
                (outer.sqrt() + minor_radius.abs()) * (outer + minor_radius.powi(2)),
            )
        }
        Surface::Nurbs(_) | Surface::Approx(_) => {
            return Err(CompositeRefusal::Unread(
                "a spline face has no canonical form",
            ));
        }
        Surface::Spindle { .. } => {
            return Err(CompositeRefusal::Unread(
                "a spindle sheet's face has no composite lane yet",
            ));
        }
    })
}

/// Whether every coefficient ratio of the piece is decided past the band
/// on one side.
fn clear<T: Decide>(piece: &Piece<T>, read: &impl Fn(T) -> Reading) -> bool {
    let mut side = None;
    for (&n, &d) in piece.num.iter().zip(&piece.den) {
        match read(n / d) {
            Reading::Past(s) if side.is_none_or(|held| held == s) => side = Some(s),
            _ => return false,
        }
    }
    side.is_some()
}

/// The sign every value of `row` is proved to hold: `None` where one is
/// not proved strictly signed, or they differ, or there is none.
fn one_signed<T: Decide>(row: &[T]) -> Option<Sign> {
    let mut sign = None;
    for &v in row {
        let s = strict(v)?;
        if sign.is_some_and(|held| held != s) {
            return None;
        }
        sign = Some(s);
    }
    sign
}

/// `v`'s sign, proved strictly one side of zero.
fn strict<T: Decide>(v: T) -> Option<Sign> {
    match prove(SLOPE, Margin::sign_of(v)) {
        Ok(s @ (Sign::Positive | Sign::Negative)) => Some(s),
        _ => None,
    }
}

/// `f` at the piece's start (`at_start`) or end: the ratio of the rows'
/// end coefficients, which a Bernstein row interpolates.
fn end_value<T: Decide>(piece: &Piece<T>, at_start: bool) -> T {
    let pick = |row: &[T]| {
        if at_start { row.first() } else { row.last() }
            .copied()
            .unwrap_or_else(|| T::from_f64(f64::NAN))
    };
    pick(&piece.num) / pick(&piece.den)
}

/// The derivative's Bernstein row over the piece's own unit parameter
/// (whose positive scale changes no sign): `n·(c[i+1] − c[i])`.
fn derivative<T: Decide>(row: &[T]) -> Vec<T> {
    #[allow(clippy::cast_precision_loss)]
    let n = T::from_f64(row.len().saturating_sub(1) as f64);
    row.windows(2).map(|w| n * (w[1] - w[0])).collect()
}

/// `N′D − ND′`, whose sign is `f′`'s (`D > 0`), as one Bernstein row.
fn slope_row<T: Decide>(num: &[T], den: &[T]) -> Vec<T> {
    use geom_core::spline::canonical_rows::bernstein_product as product;
    let left = product(&derivative(num), den);
    let right = product(num, &derivative(den));
    match (left.is_empty(), right.is_empty()) {
        (false, false) => left.iter().zip(&right).map(|(l, r)| *l - *r).collect(),
        (false, true) => left,
        (true, false) => right.iter().map(|r| -*r).collect(),
        (true, true) => Vec::new(),
    }
}

/// De Casteljau's split of `row` at the share `s` of its piece: the rows
/// over the two parts.
fn split_row<T: Decide>(row: &[T], s: T) -> (Vec<T>, Vec<T>) {
    let n = row.len();
    let mut work = row.to_vec();
    let mut left = Vec::with_capacity(n);
    let mut right = Vec::with_capacity(n);
    let one_minus = T::one() - s;
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

/// The piece split at the constant share `k`: its two parts, in order.
fn split<T: Decide>(piece: &Piece<T>, k: f64) -> [Piece<T>; 2] {
    let s = T::from_f64(k);
    let (num_l, num_r) = split_row(&piece.num, s);
    let (den_l, den_r) = split_row(&piece.den, s);
    let c = at_share(piece, s);
    let (a, b) = piece.shadow;
    let m = a + (b - a) * k;
    [
        Piece {
            a: piece.a,
            b: c,
            shadow: (a, m),
            num: num_l,
            den: den_l,
        },
        Piece {
            a: c,
            b: piece.b,
            shadow: (m, b),
            num: num_r,
            den: den_r,
        },
    ]
}

/// The piece's parameter at the share `s`, as the convex combination
/// `a·(1 − s) + b·s`: at an enclosure scalar each end enters once, so a
/// chain of cuts does not widen the ends it inherits.
fn at_share<T: Decide>(piece: &Piece<T>, s: T) -> T {
    piece.a * (T::one() - s) + piece.b * s
}

/// `f` at the constant share `k` of the piece.
fn value_at<T: Decide>(piece: &Piece<T>, k: f64) -> T {
    let s = T::from_f64(k);
    let at = |row: &[T]| {
        split_row(row, s)
            .0
            .last()
            .copied()
            .unwrap_or_else(|| T::from_f64(f64::NAN))
    };
    at(&piece.num) / at(&piece.den)
}

/// A piece not settled by its readings, cut at the first of [`SPLITS`]
/// where `f` is definitely past the band, its parts in walk order.
fn cut<T: Decide>(
    piece: Piece<T>,
    floor: f64,
    read: &impl Fn(T) -> Reading,
) -> Result<[Piece<T>; 2], CompositeRefusal> {
    let (a, b) = piece.shadow;
    if b - a <= floor {
        return Err(Unsettled::Floor.into());
    }
    // The ladder over the shares: the first share definitely past the
    // band answers, and which one answers is where the piece is cut. A
    // piece within the band at every share runs along the surface here,
    // or crosses it twice within the band.
    let cut_at = climb(Answering::First, SPLITS, |k| {
        let c = a + (b - a) * k;
        if !(a < c && c < b) {
            return Rung::Passes;
        }
        match read(value_at(&piece, k)) {
            Reading::Past(_) => Rung::Answers(k),
            Reading::Near(Some(diag)) => Rung::kept(diag),
            Reading::Near(None) => Rung::Passes,
        }
    })
    .into_reading();
    match cut_at {
        Ok(Some(k)) => Ok(split(&piece, k)),
        Ok(None) => Err(touch_or_tangency(None, CompositeRefusal::Escalated)),
        Err(diag) => Err(touch_or_tangency(Some(diag), CompositeRefusal::Escalated)),
    }
}

/// `f`'s side at the halving of `piece`, proved; where it is not, the
/// surface's residual at the point (`off`): `None` where that reads ON,
/// so the halving is the crossing.
fn side_at_half<T: Decide>(
    piece: &Piece<T>,
    off: &impl Fn(T) -> (Point3<T>, Margin<T>),
    band: Band,
) -> Result<Option<Sign>, CompositeRefusal> {
    match prove(SIDE, Margin::sign_of(value_at(piece, 0.5))) {
        Ok(s @ (Sign::Positive | Sign::Negative)) => Ok(Some(s)),
        Ok(Sign::Zero) => Ok(None),
        Err(_) => {
            let mid = at_share(piece, T::from_f64(0.5));
            match decide(ROOT_ON, off(mid).1, band) {
                Ok(Sign::Zero) => Ok(None),
                Ok(s) => Ok(Some(s)),
                Err(diag) => Err(CompositeRefusal::Escalated(diag)),
            }
        }
    }
}

/// The crossing of a piece over which `f` is monotone and changes sign,
/// `left` its side at the piece's start: halved until the halving reads
/// ON ([`side_at_half`]) or the shadow closes on adjacent `f64`s.
fn bisect_row<T: Decide>(
    piece: &Piece<T>,
    left: Sign,
    off: &impl Fn(T) -> (Point3<T>, Margin<T>),
    band: Band,
) -> Result<T, CompositeRefusal> {
    let mut piece = piece.clone();
    for _ in 0..BISECTIONS {
        let (a, b) = piece.shadow;
        let m = geom::bracket_mid((a, b));
        if !(a < m && m < b) {
            break;
        }
        let Some(side) = side_at_half(&piece, off, band)? else {
            return Ok(split(&piece, 0.5)[0].b);
        };
        let [l, r] = split(&piece, 0.5);
        piece = if side == left { r } else { l };
    }
    Ok(split(&piece, 0.5)[0].b)
}

/// The crossing of a knot's stretch: `before` ends at the knot within
/// the band, its start on the side `left`, and `after` starts there,
/// ending on the other side. It is in whichever piece's ends are proved
/// opposite, or at the knot itself.
fn knot_crossing<T: Decide>(
    before: &Piece<T>,
    after: &Piece<T>,
    left: Sign,
    off: &impl Fn(T) -> (Point3<T>, Margin<T>),
    band: Band,
) -> Result<T, CompositeRefusal> {
    match prove(SIDE, Margin::sign_of(end_value(after, true))) {
        Ok(s) if s == left => bisect_row(after, s, off, band),
        Ok(Sign::Positive | Sign::Negative) => bisect_row(before, left, off, band),
        _ => Ok(after.a),
    }
}

/// **A span end within the band**, read on the monotone `piece` that
/// starts (`at_start`) or ends at it, its far end past the band on the
/// side `far` (the module docs). Where `f` at the end is proved on the
/// far side too, the meeting is the carrier's past the end: the piece's
/// own polynomial, continued past it ([`continued`]), is read for it.
fn near_end<T: Decide>(
    piece: &Piece<T>,
    at_start: bool,
    far: Sign,
    (off, speeds): (&impl Fn(T) -> (Point3<T>, Margin<T>), Speeds<T>),
    net: &NurbsCurve3<T>,
    band: Band,
) -> Result<Meeting<T>, CompositeRefusal> {
    let end = if at_start { piece.a } else { piece.b };
    match prove(SIDE, Margin::sign_of(end_value(piece, at_start))) {
        Ok(s @ (Sign::Positive | Sign::Negative)) if s != far => {
            let left = if at_start { s } else { far };
            Ok(Meeting::Crossing(bisect_row(piece, left, off, band)?))
        }
        Ok(Sign::Zero) => Ok(Meeting::End(end)),
        Ok(_) => past_the_end(piece, at_start, far, (off, speeds), band),
        Err(_) => {
            let proved = |v: T| matches!(prove(SIDE, Margin::sign_of(v)), Ok(s) if s == far);
            let until = stretch(piece, at_start, proved);
            let window = if at_start { (end, until) } else { (until, end) };
            match decide(
                STRETCH,
                Margin::norm3(net.eval(until) - net.eval(end)),
                band,
            ) {
                Ok(Sign::Zero) => Ok(Meeting::End(end)),
                Ok(_) => Ok(Meeting::Stretch(
                    window,
                    MeetFault::Unsettled(Unsettled::Tangency),
                )),
                Err(diag) => Ok(Meeting::Stretch(window, MeetFault::Escalated(diag))),
            }
        }
    }
}

/// The shares of a piece's width its polynomial is continued past an
/// end by, tried in order until the continuation is proved monotone.
const CONTINUATIONS: [f64; 3] = [1.0, 0.0625, 0.003_906_25];

/// **The meeting past a near end**, `f` there on the side `far` as at the
/// piece's other end: the root of the end piece's own polynomial
/// continued past the end, which is the carrier's continuation there,
/// placed against the end by its parameter's distance metered at the
/// span's speeds (`spline_composite_end_stretch`), as the table places a
/// root by its chord: within the band's zero of the end it is the end's
/// meeting, past the band's escalation no meeting of the span, and in
/// the gap a window at the end. A continuation not proved monotone, or
/// proved monotone with no root on it, refuses at the end.
fn past_the_end<T: Decide>(
    piece: &Piece<T>,
    at_start: bool,
    far: Sign,
    (off, (inf, sup)): (&impl Fn(T) -> (Point3<T>, Margin<T>), Speeds<T>),
    band: Band,
) -> Result<Meeting<T>, CompositeRefusal> {
    let end = if at_start { piece.a } else { piece.b };
    let tangency = Meeting::Stretch((end, end), MeetFault::Unsettled(Unsettled::Tangency));
    let Some(beyond) = CONTINUATIONS
        .iter()
        .map(|&k| continued(piece, at_start, k))
        .find(|c| one_signed(&slope_row(&c.num, &c.den)).is_some())
    else {
        return Ok(tangency);
    };
    // The continuation's far end, proved past zero on the other side.
    match prove(SIDE, Margin::sign_of(end_value(&beyond, at_start))) {
        Ok(s @ (Sign::Positive | Sign::Negative)) if s != far => {}
        _ => return Ok(tangency),
    }
    let left = if at_start { far.flip() } else { far };
    let root = bisect_row(&beyond, left, off, band)?;
    let along = root - end;
    match decide(STRETCH, Margin::metered_sup(along, sup), band) {
        Ok(Sign::Zero) => return Ok(Meeting::End(end)),
        Ok(_) => {}
        Err(diag) => return Ok(Meeting::Stretch((end, end), MeetFault::Escalated(diag))),
    }
    match decide(STRETCH, Margin::metered(along, inf), band) {
        Ok(Sign::Positive | Sign::Negative) => Ok(Meeting::Past),
        Ok(Sign::Zero) => Ok(tangency),
        Err(diag) => Ok(Meeting::Stretch((end, end), MeetFault::Escalated(diag))),
    }
}

/// **The piece's polynomial continued past its start (`at_start`) or its
/// end** by the share `k` of its width: the rows over
/// `[a − k·(b − a), a]` or `[b, b + k·(b − a)]`, by de Casteljau's split
/// at a share outside `[0, 1]`, which the polynomial identity holds at
/// every share.
fn continued<T: Decide>(piece: &Piece<T>, at_start: bool, k: f64) -> Piece<T> {
    let (sa, sb) = piece.shadow;
    let w = sb - sa;
    let width = piece.b - piece.a;
    let kk = T::from_f64(k);
    let rows = |row: &[T]| -> Vec<T> {
        if at_start {
            // Over `[a − k·w, b]`, then its part before `a`.
            let (_, right) = split_row(row, -kk);
            let share = kk / (T::one() + kk);
            split_row(&right, share).0
        } else {
            // Over `[a, b + k·w]`, then its part past `b`.
            let (left, _) = split_row(row, T::one() + kk);
            let share = T::one() / (T::one() + kk);
            split_row(&left, share).1
        }
    };
    let (a, b, shadow) = if at_start {
        (piece.a - width * kk, piece.a, (sa - w * k, sa))
    } else {
        (piece.b, piece.b + width * kk, (sb, sb + w * k))
    };
    Piece {
        a,
        b,
        shadow,
        num: rows(&piece.num),
        den: rows(&piece.den),
    }
}

/// **How far a near end's stretch runs** along the monotone `piece`
/// that starts (`at_start`) or ends at it: the parameter, found by
/// halving, past which `f` is proved on its far side (`proved`). `f` is
/// monotone over the piece, so every meeting of it lies on the stretch.
fn stretch<T: Decide>(piece: &Piece<T>, at_start: bool, proved: impl Fn(T) -> bool) -> T {
    let mut piece = piece.clone();
    for _ in 0..BISECTIONS {
        let (a, b) = piece.shadow;
        let m = geom::bracket_mid((a, b));
        if !(a < m && m < b) {
            break;
        }
        let past = proved(value_at(&piece, 0.5));
        let [l, r] = split(&piece, 0.5);
        piece = if past == at_start { l } else { r };
    }
    if at_start { piece.b } else { piece.a }
}

/// **The meetings found, each read**: against the creases (where the
/// carrier has no tangent), then ON the surface. A crossing must read
/// ON; an end reads ON (a root at the end), off (no meeting), or in the
/// band's gap (a window at the end). The roots are placed against the
/// span's ends and each other by the table ([`Walked`]'s reader).
fn settle<T: Decide>(
    net: &NurbsCurve3<T>,
    (s0, s1): (f64, f64),
    band: Band,
    meetings: Vec<Meeting<T>>,
    off: &impl Fn(T) -> (Point3<T>, Margin<T>),
) -> Result<Walked<T>, CompositeRefusal> {
    let creases: Vec<Point3<T>> = net
        .knot_values()
        .crease_values()
        .into_iter()
        .filter(|u| {
            let v = u.value_channel();
            s0 < v && v < s1
        })
        .map(|u| net.eval(u))
        .collect();
    let mut walked = Walked::found(Vec::new());
    for meeting in meetings {
        let t = match meeting {
            Meeting::Crossing(t) | Meeting::Touch(t, _) | Meeting::End(t) => t,
            Meeting::Stretch(window, fault) => {
                walked.windows.push((window, fault));
                continue;
            }
            Meeting::Past => continue,
        };
        let (p, on) = off(t);
        for &crease in &creases {
            match decide(AT_CREASE, Margin::norm3(p - crease), band) {
                Ok(Sign::Zero) => return Err(Unsettled::Crease.into()),
                Ok(_) => {}
                Err(diag) => return Err(CompositeRefusal::Escalated(diag)),
            }
        }
        if let Meeting::Touch(_, refusal) = meeting {
            return Err(refusal);
        }
        match (decide(ROOT_ON, on, band), &meeting) {
            (Ok(Sign::Zero), _) => walked.roots.push(t),
            (Ok(_), Meeting::End(_)) => {}
            (Ok(_), _) => return Err(Unsettled::Meter { row: ROOT_ON }.into()),
            (Err(diag), Meeting::End(_)) => {
                walked.windows.push(((t, t), MeetFault::Escalated(diag)))
            }
            (Err(diag), _) => return Err(CompositeRefusal::Escalated(diag)),
        }
    }
    Ok(walked)
}
