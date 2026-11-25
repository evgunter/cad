//! **The spline span × analytic face arm** of the curve × face table
//! ([`super::Door::SplineComposite`]): the face's canonical residual
//! composed along the span's NURBS carrier, the ring composite `f ∘ C`
//! ([`geom_core::spline::compose::canonical_composite`]), read piece by
//! piece. It reads a plane, cylinder, cone, sphere or torus, metered
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
//! is decided against. The divisor is a factor of `f` at each point,
//! `|f(p)| ≤ d(p)·divisor(p)` with `d` the distance from the form's zero
//! set (`projected::Meters::divisor`), not a bound along a path to it, so
//! the hull, which holds every point of the span, is the region it is
//! read over. **A piece never straddles a knot**: the pieces
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
//!   opposite sides, one crossing: bracketed between the last parameter
//!   where `f`'s enclosure reads the first side and the first where it
//!   reads the other, proposed inside the bracket where the surface's
//!   residual changes sign, and read ON the surface
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
//! **A crossing's parameter is proposed, and placed for every proposal.**
//! Its bracket holds the composite's one root in the piece, so every
//! parameter the arm could store for the crossing lies in it, and the
//! carrier's length over the bracket is the crossing's reach
//! ([`geom_core::solved::Reach::along`]): a bound from the carrier's own
//! rows, about a hundred rounding steps of the composite over the
//! crossing's sine, and as wide as the form is ill-conditioned (a thin
//! torus, a family of nets). Each placing above is decided with it
//! ([`geom_core::solved::decide_reaching`]): apart only where the nearest
//! point of the reach is, at an end only where the farthest is, and
//! otherwise the band's escalation. The crossing is returned with it.
//!
//! The walk is `geom_core::bisect::bisect_within`, with its piece budget
//! past the knot spans it starts from.

use geom::{NurbsCurve3, Surface};
use geom_core::bisect::{Budget, PIECE_BUDGET, Step, bisect_within};
use geom_core::k_stats::decide;
use geom_core::solved::{Admitted, Checked, Reach, decide_admit, decide_reaching};
use geom_core::spline::compose::{
    CanonicalSurface, CurveCertData, canonical_composite, linear_composite,
};
use geom_core::{Band, Bounds, Indeterminate, Interval, Margin, Point3, Real, Sign, Vec3};

use crate::half_angle::SPLITS;
use crate::ladder::{Answering, Rung, climb};
use crate::pcurve_cache::projected::Meters;
use crate::roots::{Unsettled, touch_or_tangency, unadmitted};

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
    /// The face's surface describes no frame or no positive meter: its
    /// chart frame does not certify (a reference direction along its
    /// axis, a zero or non-finite direction), or the form's distance
    /// divisor has no positive upper bound over the net's hull (a
    /// non-positive or non-finite radius). A surface tier 3 refuses
    /// ([`geom::Surface::representability_margins`]), read here because
    /// the arm does not re-validate its inputs.
    FaceDegenerate(&'static str),
    /// The composite could not be formed: the face has no canonical form
    /// (a spline face), or a control point, a knot or the net's structure
    /// did not certify.
    Unread(&'static str),
}

impl From<Unsettled> for CompositeRefusal {
    fn from(why: Unsettled) -> Self {
        Self::Unsettled(why)
    }
}

/// **The crossings of one span** ([`crossings`]): each parameter with
/// what its check bounded.
pub type Crossings<T> = Vec<Admitted<T, Reach>>;

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

/// The bisection steps that close on an end of a crossing's bracket, or
/// on its proposal: enough to reach `f64` resolution from any piece.
const BISECTIONS: u32 = 64;

/// One piece of the composite: the parameter interval `[a, b]` and the
/// numerator's and denominator's Bernstein rows over it.
struct Piece {
    a: f64,
    b: f64,
    num: Vec<Interval>,
    den: Vec<Interval>,
}

/// The carrier over one knot span `[a, b]`, in the chart frame: each
/// coordinate's weighted Bernstein row and the weight's.
struct Rail {
    a: f64,
    b: f64,
    coords: [Vec<Interval>; 3],
    weight: Vec<Interval>,
}

/// What a meeting the walk found is, before it is placed.
enum Meeting {
    /// A crossing: the parameter proposed for it, and the carrier's
    /// length over its bracket in metres, which every parameter the
    /// crossing could be stored at lies within of this one.
    Crossing(f64, f64),
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
/// parameter whose point reads ON the surface, with how far along the
/// carrier any other the arm could have stored lies from it, not an
/// enclosure of the true crossing's parameter.
///
/// `cross` takes a scalar of the body into certification arithmetic: it
/// is the door's ([`crate::NurbsLane::surface_crossings`]). The routine
/// is certification interior and its bound says so, so a dual cannot
/// instantiate it:
///
/// ```compile_fail,E0277
/// use geom_brep::curve_face::composite::crossings;
/// use geom_core::{Band, Dual64, Interval};
/// fn at_a_dual(
///     net: &geom::NurbsCurve3<Dual64>,
///     params: (Dual64, Dual64),
///     surface: &geom::Surface<Dual64>,
///     band: Band,
///     cross: fn(Dual64) -> Interval,
/// ) {
///     let _ = crossings(net, params, surface, band, cross);
/// }
/// ```
///
/// ```
/// use geom_brep::curve_face::composite::crossings;
/// use geom_core::{Band, Interval};
/// fn at_f64(
///     net: &geom::NurbsCurve3<f64>,
///     params: (f64, f64),
///     surface: &geom::Surface<f64>,
///     band: Band,
///     cross: fn(f64) -> Interval,
/// ) {
///     let _ = crossings(net, params, surface, band, cross);
/// }
/// ```
///
/// # Errors
///
/// [`CompositeRefusal`], as its variants say.
// `!(a < b)` is deliberate: a NaN end refuses.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
pub fn crossings<T: geom_core::Certifies>(
    net: &NurbsCurve3<T>,
    params: (T, T),
    surface: &Surface<T>,
    band: Band,
    cross: impl Fn(T) -> Interval,
) -> Result<Crossings<T>, CompositeRefusal> {
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
    let (pieces, rails, divisor) = composite_rows(net, surface, (a, b), &cross)?;
    let floor = (b - a) * FLOOR_SHARE;
    let read = |f: Interval| match decide(PAST_THE_BAND, Margin::of(f / divisor), band) {
        Ok(s @ (Sign::Positive | Sign::Negative)) => Reading::Past(s),
        Ok(Sign::Zero) => Reading::Near(None),
        Err(diag) => Reading::Near(Some(diag)),
    };
    // The surface's residual at a point of the carrier, read by two
    // askers: the proposal at places it chose (`middle`, the middle of
    // its enclosure, whose sign change it closes on), and the door at the
    // stored crossing (in `settle`).
    let off = |p: Point3<T>| Margin::of(crate::implicit_residual(surface, p));
    let middle = |t: f64| {
        let r = cross(off(net.eval_at(t)).value());
        0.5 * (r.lo() + r.hi())
    };
    let crossing = |bracket: (f64, f64)| {
        Meeting::Crossing(propose(bracket, &middle), length_over(&rails, bracket))
    };
    // The knot at which a piece ended within the band, that piece, and
    // its other end's side: read with the piece that starts there.
    let mut open: Option<(f64, Piece, Sign)> = None;
    let mut meetings: Vec<Meeting> = Vec::new();
    // Every knot span is a root read once, so the budget walls the
    // refinement past them: a fitted carrier holds thousands of spans at
    // a fine band.
    let mut budget = Budget::new(PIECE_BUDGET.saturating_add(pieces.len()));
    let walked = bisect_within(pieces, &mut budget, |piece: Piece| {
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
                    meetings.push(crossing((
                        last_on_side(&piece, l),
                        first_on_side(&piece, l.flip()),
                    )));
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
                        crossing(knot_bracket(&before, &piece, l, far))
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
    settle(net, params, band, &creases, meetings, &off)
}

/// [`framed_form`]'s answer: the points' coordinates in the chart frame,
/// the canonical form and its metres conversion.
pub(crate) type FramedForm = ([Vec<Interval>; 3], CanonicalSurface, Meters<Interval>);

/// A surface's canonical form and its metres conversion, with `points`
/// written in its chart frame in certification arithmetic, coordinate by
/// coordinate: the operands a composite against the surface reads, for
/// a carrier's net or a patch's ([`crate::patch_carrier`]) alike.
///
/// # Errors
///
/// [`CompositeRefusal::Unread`] off an analytic chart or for a point that
/// does not certify; [`CompositeRefusal::FaceDegenerate`] where the chart
/// frame does not.
pub(crate) fn framed_form<T: geom_core::Certifies>(
    surface: &Surface<T>,
    points: &[Point3<T>],
    cross: &impl Fn(T) -> Interval,
) -> Result<FramedForm, CompositeRefusal> {
    let unread = CompositeRefusal::Unread;
    let (origin, frame) = crate::pcurve_cache::projected::chart_frame(surface)
        .ok_or(unread("a spline face has no canonical form"))?;
    let point = |p: Point3<T>| Vec3::new(cross(p.x), cross(p.y), cross(p.z));
    let vector = |v: Vec3<T>| Vec3::new(cross(v.x), cross(v.y), cross(v.z));
    let o = point(origin);
    // The frame formed in certification arithmetic: the axis normalized,
    // and the reference direction made orthogonal to it and normalized.
    // Its enclosures hold an exactly orthonormal frame about the stored
    // axis, which the radial terms read; the stored twin is orthonormal
    // only to its rounding.
    let [u_ref, _, axis] = frame.map(vector);
    let e2 = axis.normalize();
    let radial = u_ref - e2 * u_ref.dot(e2);
    let e0 = radial.normalize();
    let e = [e0, e2.cross(e0), e2];
    if !e
        .iter()
        .all(|v| v.x.is_certified() && v.y.is_certified() && v.z.is_certified())
    {
        return Err(CompositeRefusal::FaceDegenerate(
            "the face's chart frame does not certify",
        ));
    }
    let framed: Vec<Vec3<Interval>> = points
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
        return Err(unread("a control point does not certify"));
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
            let (sin, cos) = crate::implicit::cone_sin_cos(half_angle, cross);
            (
                CanonicalSurface::Cone {
                    cos2: cos.powi(2),
                    sin2: sin.powi(2),
                },
                Meters::Cone {
                    sin,
                    cos,
                    locus: crate::pcurve_cache::projected::Locus::Carrier,
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
        Surface::Spindle { .. } => {
            return Err(unread("a spindle sheet's face has no composite lane yet"));
        }
    };
    Ok((coords, form, meters))
}

/// The composite's rows over `[a, b]`, one piece per knot span, the
/// carrier's own over the same spans, and an upper bound on the form's
/// distance divisor over the net's hull (`projected::Meters::divisor`;
/// one for the plane).
fn composite_rows<T: geom_core::Certifies>(
    net: &NurbsCurve3<T>,
    surface: &Surface<T>,
    (a, b): (f64, f64),
    cross: &impl Fn(T) -> Interval,
) -> Result<(Vec<Piece>, Vec<Rail>, Interval), CompositeRefusal> {
    let unread = CompositeRefusal::Unread;
    let (coords, form, meters) = framed_form(surface, net.control(), cross)?;
    // The net's hull in the chart frame bounds every point of the span,
    // so the divisor read over it bounds every point's.
    let divisor = meters
        .divisor(Vec3::new(
            hull(&coords[0]),
            hull(&coords[1]),
            hull(&coords[2]),
        ))
        .abs();
    if !(divisor.is_certified() && divisor.hi() > 0.0) {
        return Err(CompositeRefusal::FaceDegenerate(
            "the form's distance divisor has no positive upper bound",
        ));
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
    let axis = |k: usize| {
        let mut coeffs = [0.0; 3];
        coeffs[k] = 1.0;
        linear_composite(&data, &coeffs, 0.0).map_err(|_| unread("the composite refused the net"))
    };
    let [x, y, z] = [axis(0)?, axis(1)?, axis(2)?];
    let rails = x
        .num
        .breaks()
        .windows(2)
        .enumerate()
        .filter(|(_, w)| w[0] >= a && w[1] <= b)
        .map(|(i, w)| Rail {
            a: w[0],
            b: w[1],
            coords: [&x, &y, &z].map(|c| c.num.spans()[i].clone()),
            weight: x.den.spans()[i].clone(),
        })
        .collect();
    Ok((pieces, rails, divisor))
}

/// The carrier's creases strictly inside `(a, b)`, each at its held
/// knot's lower end, which is the knot where it is a point.
fn creases<T: geom_core::Certifies>(
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
///
/// A fold over enclosures, not an agreeing ladder
/// (`geom_core::ladder::climb_agreeing`): no value is a band decision,
/// so none is kept in band to answer past, and the sign needs every one.
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

/// The share of the piece at which `c` lies, enclosed.
fn share(piece: &Piece, c: f64) -> Interval {
    share_of((piece.a, piece.b), c)
}

/// The share of `[a, b]` at which `c` lies, enclosed.
fn share_of((a, b): (f64, f64), c: f64) -> Interval {
    let point = |x: f64| Interval::from_bounds(x, x);
    let (a, b) = (point(a), point(b));
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
    // The ladder over the shares ([`climb`]): the first share definitely
    // past the band answers, and which one answers is where the piece is
    // cut (`Answering::First`). A piece within the band at every share
    // runs along the surface here, or crosses it twice within the band:
    // it refuses on the kept share, ranked ([`crate::ladder::Evidence`]),
    // and is a tangency where every share read zero.
    let cut_at = climb(Answering::First, SPLITS, |k| {
        let c = piece.a + (piece.b - piece.a) * k;
        let s = share(&piece, c);
        if !(piece.a < c && c < piece.b) {
            return Rung::Passes;
        }
        match read(value_at(&piece, s)) {
            Reading::Past(_) => Rung::Answers((c, s)),
            Reading::Near(Some(diag)) => Rung::kept(diag),
            Reading::Near(None) => Rung::Passes,
        }
    })
    .into_reading();
    let (c, s) = match cut_at {
        Ok(Some(at)) => at,
        Ok(None) => return Err(touch_or_tangency(None, CompositeRefusal::Escalated)),
        Err(diag) => return Err(touch_or_tangency(Some(diag), CompositeRefusal::Escalated)),
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

/// The greatest parameter a bisection of the piece finds at which `f`'s
/// enclosure reads `side`, the side of its start: `f` is monotone over
/// the piece, so its root there lies past it.
fn last_on_side(piece: &Piece, side: Sign) -> f64 {
    let (mut lo, mut hi) = (piece.a, piece.b);
    for _ in 0..BISECTIONS {
        let c = geom::bracket_mid((lo, hi));
        if !(lo < c && c < hi) {
            break;
        }
        if strict_sign(value_at(piece, share(piece, c))) == Some(side) {
            lo = c;
        } else {
            hi = c;
        }
    }
    lo
}

/// The least parameter a bisection of the piece finds at which `f`'s
/// enclosure reads `side`, the side of its end: its root there lies
/// before it.
fn first_on_side(piece: &Piece, side: Sign) -> f64 {
    let (mut lo, mut hi) = (piece.a, piece.b);
    for _ in 0..BISECTIONS {
        let c = geom::bracket_mid((lo, hi));
        if !(lo < c && c < hi) {
            break;
        }
        if strict_sign(value_at(piece, share(piece, c))) == Some(side) {
            hi = c;
        } else {
            lo = c;
        }
    }
    hi
}

/// The bracket of the crossing of a knot's stretch: `before` ends at the
/// knot within the band, its start on the side `left`, and `after`
/// starts there, ending on the side `far`. The root is in whichever
/// piece's ends read opposite strict signs, or in either where the knot
/// reads none.
fn knot_bracket(before: &Piece, after: &Piece, left: Sign, far: Sign) -> (f64, f64) {
    match strict_sign(end_value(after, true)) {
        Some(s) if s == left => (last_on_side(after, left), first_on_side(after, far)),
        Some(_) => (last_on_side(before, left), first_on_side(before, far)),
        None => (last_on_side(before, left), first_on_side(after, far)),
    }
}

/// **The parameter proposed for a crossing**, inside its bracket: where
/// the middle of the surface's residual changes sign between the
/// bracket's ends, bisected to adjacent `f64`s, or the bracket's middle
/// where it reads no change. A place the search chose: the door admits
/// it or refuses it ([`settle`]), and no answer rests on which parameter
/// of the bracket it is.
fn propose((mut lo, mut hi): (f64, f64), middle: &impl Fn(f64) -> f64) -> f64 {
    let below = middle(lo) < 0.0;
    if (middle(hi) < 0.0) == below {
        return geom::bracket_mid((lo, hi));
    }
    for _ in 0..BISECTIONS {
        let c = geom::bracket_mid((lo, hi));
        if !(lo < c && c < hi) {
            break;
        }
        if (middle(c) < 0.0) == below {
            lo = c;
        } else {
            hi = c;
        }
    }
    geom::bracket_mid((lo, hi))
}

/// **An upper bound on the carrier's length over `[lo, hi]`**, in
/// metres: over each knot span's part of it, the supremum of the
/// carrier's speed there times the part's share of the span. The speed
/// is `(A′W − AW′) / W²` per coordinate, each row cut to the part by de
/// Casteljau and read by its hull. NaN where a row does not certify,
/// which bounds nothing.
// `!(a < b)` is deliberate: a NaN end is no part.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
fn length_over(rails: &[Rail], (lo, hi): (f64, f64)) -> f64 {
    let mut total = Interval::from_bounds(0.0, 0.0);
    for rail in rails {
        let (a, b) = (lo.max(rail.a), hi.min(rail.b));
        if !(a < b) {
            continue;
        }
        let (s0, s1) = (share_of((rail.a, rail.b), a), share_of((rail.a, rail.b), b));
        let over = |row: Vec<Interval>| split_row(&split_row(&row, s1).0, s0 / s1).1;
        let scale = hull(&over(product(&rail.weight, &rail.weight)));
        let [x, y, z] =
            [0, 1, 2].map(|k| hull(&over(slope_row(&rail.coords[k], &rail.weight))) / scale);
        total = total + Vec3::new(x, y, z).norm() * (s1 - s0);
    }
    total.hi()
}

/// The meetings found, each placed in metres for every parameter its
/// crossing could be stored at (module docs): against the creases (at
/// the candidate as a place, deciding only), then on the surface (`off`,
/// the surface's residual at a point of the carrier, through the
/// solved-value door, which hands the crossing back with its reach),
/// then, at the point the door read, against the span's ends and the
/// crossing before it.
fn settle<T: geom_core::Certifies>(
    net: &NurbsCurve3<T>,
    (t0, t1): (T, T),
    band: Band,
    creases: &[f64],
    meetings: Vec<Meeting>,
    off: &impl Fn(Point3<T>) -> Margin<T>,
) -> Result<Crossings<T>, CompositeRefusal> {
    let gap_as = |row, p: Point3<T>, q: Point3<T>, reach: f64| {
        decide_reaching(row, Margin::of((p - q).norm()), reach, band)
            .map_err(CompositeRefusal::Escalated)
    };
    let ends = [net.eval(t0), net.eval(t1)];
    let creases: Vec<Point3<T>> = creases.iter().map(|&k| net.eval_at(k)).collect();
    let mut roots: Vec<(Admitted<T, Reach>, Point3<T>)> = Vec::with_capacity(meetings.len());
    for meeting in meetings {
        let (t, along) = match meeting {
            Meeting::Crossing(t, along) => (t, along),
            Meeting::Touch(t, _) => (t, 0.0),
        };
        let place = net.eval_at(t);
        for &crease in &creases {
            if gap_as("spline_composite_root_at_crease", place, crease, along)? == Sign::Zero {
                return Err(Unsettled::Crease.into());
            }
        }
        if let Meeting::Touch(_, refusal) = meeting {
            return Err(refusal);
        }
        // Definitely off at the proposal: the residual's step between two
        // adjacent scalars is past the band.
        let (at, p) = decide_admit("spline_composite_root_on", t, band, |at| {
            let p = net.eval(at);
            Checked {
                residual: off(p),
                reach: along,
                witness: p,
            }
        })
        .map_err(|refused| unadmitted(refused, CompositeRefusal::Escalated))?;
        let along = at.bound().along();
        let mut at_end = false;
        for end in ends {
            at_end |= gap_as("spline_composite_root_at_end", p, end, along)? == Sign::Zero;
        }
        if at_end {
            continue;
        }
        if let Some(&(before, q)) = roots.last()
            && gap_as(
                "spline_composite_roots_apart",
                p,
                q,
                along + before.bound().along(),
            )? == Sign::Zero
        {
            return Err(Unsettled::Tangency.into());
        }
        roots.push((at, p));
    }
    Ok(roots.into_iter().map(|(t, _)| t).collect())
}

#[cfg(test)]
mod rows;

#[cfg(test)]
mod crossing_parameter_rows;
