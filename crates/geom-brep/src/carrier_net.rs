//! **A closed-form carrier as an enclosing net** — a line, circle,
//! ellipse or spiric written as a piecewise Bernstein curve whose
//! interval coefficients enclose the carrier over every span, so the
//! tensor composite (`geom_core::spline::compose::tensor`) can bound
//! `S(P(t)) − C(t)` for it exactly as it does for a spline carrier.
//!
//! A line is its own degree-1 net: the controls enclose the carrier at
//! the breaks, and linear interpolation of them is the line.
//!
//! The curved kinds are `C(t) = P₀ + U·α(t) + V·β(t)` (circle and
//! ellipse: `α = cos`, `β = sin`; spiric: `α = √((R + r·cos t)² − d²)`,
//! `β = sin`). On a span `[a, b]` with node `m` each coordinate is its
//! Taylor polynomial of degree [`ORDER`] at `m` plus the Lagrange
//! remainder, which lies in `c_{n+1}([a, b])·(t − m)^{n+1}` where
//! `c_{n+1}([a, b])` is the `(n+1)`-th Taylor coefficient computed with
//! the span itself as the expansion point: every recurrence below is
//! inclusion-isotone, so that coefficient encloses `C^{(n+1)}(ξ)/(n+1)!`
//! for every `ξ` in the span. The polynomial is re-expressed in the
//! span's Bernstein basis (`t = a + (b − a)·s`) and the remainder added
//! to every coefficient, which a partition of unity carries through.
//! Adjacent spans share their joint control as the hull of the two
//! ends, each of which encloses `C` at the break. Everything is
//! interval arithmetic (C9); a coefficient that does not certify (a
//! spiric whose square root's argument reaches zero over a span)
//! refuses the net, naming the kind and the span
//! ([`CarrierEnclosureRefusal`]).
//!
//! [`carrier_net`] is the door every limb-2 composite reads a carrier
//! through: a spline as its own net, any other kind as its net here.

use geom::Curve3;
use geom_core::interval::Interval;
use geom_core::spline::{KnotVector, SpanLocate};
use geom_core::{Bounds, CertifiedBounds, Point3, Real, Vec3};

/// The Taylor degree of each span's polynomial; the remainder is the
/// next coefficient.
pub(crate) const ORDER: usize = 6;

/// The coefficient count a series carries: `0..=ORDER + 1`.
const TERMS: usize = ORDER + 2;

/// A truncated Taylor series in interval arithmetic.
type Series = [Interval; TERMS];

/// The carrier enclosed: a clamped knot vector over `breaks` and three
/// coordinate channels of interval controls on it.
struct EnclosingNet {
    knots: KnotVector,
    coords: Vec<Vec<Interval>>,
}

/// The carrier `C(t) = P₀ + U·α(t) + V·β(t)` in the scalar's
/// certified enclosures.
struct Harmonic {
    p0: Vec3<Interval>,
    u: Vec3<Interval>,
    v: Vec3<Interval>,
    alpha: Alpha,
}

/// Which function `α` is.
enum Alpha {
    Cos,
    /// `√((major + minor·cos t)² − offset²)`.
    Spiric {
        major: Interval,
        minor: Interval,
        offset: Interval,
    },
}

fn lift_v<T: CertifiedBounds>(v: Vec3<T>) -> Vec3<Interval> {
    Vec3::new(
        Interval::from_certified(v.x),
        Interval::from_certified(v.y),
        Interval::from_certified(v.z),
    )
}

fn lift_p<T: CertifiedBounds>(p: Point3<T>) -> Vec3<Interval> {
    lift_v(p - Point3::origin())
}

/// `None` for a line or a spline carrier, neither of which is a
/// harmonic.
fn harmonic<T: CertifiedBounds>(carrier: &Curve3<T>) -> Option<Harmonic> {
    let i = Interval::from_certified;
    match carrier {
        Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } => {
            let (axis, u_ref, r) = (lift_v(*axis), lift_v(*u_ref), i(*radius));
            Some(Harmonic {
                p0: lift_p(*center),
                u: u_ref * r,
                v: axis.cross(u_ref) * r,
                alpha: Alpha::Cos,
            })
        }
        Curve3::Ellipse {
            center,
            axis,
            semi_u,
            semi_v,
            u_ref,
        } => {
            let (axis, u_ref) = (lift_v(*axis), lift_v(*u_ref));
            Some(Harmonic {
                p0: lift_p(*center),
                u: u_ref * i(*semi_u),
                v: axis.cross(u_ref) * i(*semi_v),
                alpha: Alpha::Cos,
            })
        }
        Curve3::Spiric {
            center,
            axis,
            u_ref,
            major_radius,
            minor_radius,
            offset,
        } => {
            let (axis, u_ref) = (lift_v(*axis), lift_v(*u_ref));
            let (minor, offset) = (i(*minor_radius), i(*offset));
            Some(Harmonic {
                p0: lift_p(*center) + u_ref * offset,
                u: axis.cross(u_ref),
                v: axis * minor,
                alpha: Alpha::Spiric {
                    major: i(*major_radius),
                    minor,
                    offset,
                },
            })
        }
        Curve3::Line { .. } | Curve3::Nurbs(_) => None,
    }
}

fn zero_series() -> Series {
    [Interval::zero(); TERMS]
}

fn factorial(k: usize) -> Interval {
    (1..=k).fold(Interval::one(), |f, j| f * Interval::from_f64(j as f64))
}

/// `(sin, cos)` expanded at `x`: `f⁽ᵏ⁾(x)/k!`.
fn sin_cos_series(x: Interval) -> (Series, Series) {
    let (s, c) = x.sin_cos();
    let (mut sin, mut cos) = (zero_series(), zero_series());
    for k in 0..TERMS {
        let f = factorial(k);
        let (sk, ck) = match k % 4 {
            0 => (s, c),
            1 => (c, -s),
            2 => (-s, -c),
            _ => (-c, s),
        };
        sin[k] = sk / f;
        cos[k] = ck / f;
    }
    (sin, cos)
}

fn mul(a: &Series, b: &Series) -> Series {
    let mut out = zero_series();
    for (k, o) in out.iter_mut().enumerate() {
        *o = (0..=k).fold(Interval::zero(), |acc, j| acc + a[j] * b[k - j]);
    }
    out
}

/// `√a`: `y₀ = √a₀`, `yₖ = (aₖ − Σ_{j=1}^{k−1} yⱼ·y_{k−j}) / 2y₀`.
fn sqrt(a: &Series) -> Series {
    let mut y = zero_series();
    y[0] = a[0].sqrt();
    let two_y0 = y[0] + y[0];
    for k in 1..TERMS {
        let cross = (1..k).fold(Interval::zero(), |acc, j| acc + y[j] * y[k - j]);
        y[k] = (a[k] - cross) / two_y0;
    }
    y
}

/// `(α, β)` expanded at `x`.
fn alpha_beta(alpha: &Alpha, x: Interval) -> (Series, Series) {
    let (sin, cos) = sin_cos_series(x);
    match alpha {
        Alpha::Cos => (cos, sin),
        Alpha::Spiric {
            major,
            minor,
            offset,
        } => {
            let mut rho = cos.map(|c| *minor * c);
            rho[0] = rho[0] + *major;
            let mut g = mul(&rho, &rho);
            g[0] = g[0] - *offset * *offset;
            (sqrt(&g), sin)
        }
    }
}

/// One coordinate's Taylor coefficients: `p₀ + u·α + v·β`.
fn channel(p0: Interval, u: Interval, v: Interval, a: &Series, b: &Series) -> Series {
    let mut out = zero_series();
    for k in 0..TERMS {
        out[k] = u * a[k] + v * b[k];
    }
    out[0] = out[0] + p0;
    out
}

/// The span `[a, b]`'s Bernstein coefficients, degree [`ORDER`], per
/// coordinate, enclosing the carrier over the span.
fn span_bernstein(h: &Harmonic, a: f64, b: f64) -> [Vec<Interval>; 3] {
    let n = ORDER;
    let m = 0.5 * (a + b);
    let (ia, ib, im) = (
        Interval::from_f64(a),
        Interval::from_f64(b),
        Interval::from_f64(m),
    );
    let (am, bm) = (ia - im, ib - im);
    let at_node = alpha_beta(&h.alpha, im);
    let over_span = alpha_beta(&h.alpha, ia.enclosure_hull(ib));
    // `(t − m)^{n+1}` over the span.
    let reach = am
        .enclosure_hull(bm)
        .powi(i32::try_from(n + 1).unwrap_or(i32::MAX));
    // `t − m = (a − m) + (b − a)·s`, so `(t − m)^j`'s coefficients in
    // `s` are the binomial expansion.
    let w = ib - ia;
    let binom = geom_core::spline::pieces::binomial;
    let coords = |p0: Interval, u: Interval, v: Interval| -> Vec<Interval> {
        let node = channel(p0, u, v, &at_node.0, &at_node.1);
        let span = channel(p0, u, v, &over_span.0, &over_span.1);
        let rest = span[n + 1] * reach;
        // The polynomial's coefficients in `s`.
        let mut mono = vec![Interval::zero(); n + 1];
        for (j, cj) in node.iter().take(n + 1).enumerate() {
            for (k, mk) in mono.iter_mut().enumerate().take(j + 1) {
                let term = Interval::from_f64(binom(j, k))
                    * am.powi(i32::try_from(j - k).unwrap_or(i32::MAX))
                    * w.powi(i32::try_from(k).unwrap_or(i32::MAX));
                *mk = *mk + *cj * term;
            }
        }
        // Monomial to Bernstein: `bᵢ = Σ_{k≤i} C(i,k)/C(n,k)·aₖ`.
        (0..=n)
            .map(|i| {
                (0..=i).fold(rest, |acc, k| {
                    acc + mono[k] * Interval::from_f64(binom(i, k))
                        / Interval::from_f64(binom(n, k))
                })
            })
            .collect()
    };
    [
        coords(h.p0.x, h.u.x, h.v.x),
        coords(h.p0.y, h.u.y, h.v.y),
        coords(h.p0.z, h.u.z, h.v.z),
    ]
}

/// Why [`carrier_net`] has no certified net for a carrier: a span of
/// the break list over which the carrier's coefficients do not certify
/// (a spiric whose square root's argument reaches zero there, or a
/// carrier whose data are not finite), named by the carrier's kind and
/// the span. A break list with fewer than two points names the points
/// it has.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CarrierEnclosureRefusal {
    /// The carrier's kind.
    pub kind: geom::CurveKind,
    /// The span, `(start, end)` in the carrier's parameter.
    pub span: (f64, f64),
}

impl core::fmt::Display for CarrierEnclosureRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "the {:?} carrier has no certified enclosure over [{}, {}]",
            self.kind, self.span.0, self.span.1
        )
    }
}

/// A carrier as limb 2's composite reads it: knots, weights and
/// certified controls, with the breaks the composite still inserts.
pub(crate) struct CarrierNet {
    knots: KnotVector,
    weights: Vec<f64>,
    coords: Vec<Vec<Interval>>,
    cuts: Vec<f64>,
}

impl CarrierNet {
    /// The net as the composite's curve operand.
    pub(crate) fn operand(&self) -> (&KnotVector, &[f64], &[Vec<Interval>]) {
        (&self.knots, &self.weights, &self.coords)
    }

    /// The breaks the composite inserts into the net by exact knot
    /// insertion: a spline's round breaks; none for a closed form,
    /// whose knots are its breaks already.
    pub(crate) fn cuts(&self) -> &[f64] {
        &self.cuts
    }
}

/// **The carrier's certified net over `domain`, cut at `breaks`** —
/// the one place a carrier's enclosure for limb 2 is produced, for the
/// fitted row and for the edge certificate alike.
///
/// - A spline is its own net, `domain` unread; `breaks` ride along as
///   [`CarrierNet::cuts`], which the composite inserts exactly, so the
///   net it reads is the spline refined at them.
/// - A line is its exact degree-1 net, and a circle, ellipse or spiric
///   its Taylor-enclosing net (module docs), both over the breaks
///   inside `domain` with `domain`'s ends added.
///
/// # Errors
///
/// [`CarrierEnclosureRefusal`] naming the first span over which a
/// coefficient does not certify.
pub(crate) fn carrier_net<T: CertifiedBounds>(
    carrier: &Curve3<T>,
    (lo, hi): (f64, f64),
    breaks: &[f64],
) -> Result<CarrierNet, CarrierEnclosureRefusal> {
    if let Curve3::Nurbs(net) = carrier {
        return Ok(CarrierNet {
            knots: net.knots_f64_interim().clone(),
            weights: net.weights_f64_interim().to_vec(),
            coords: net.certified_coords(),
            cuts: breaks.to_vec(),
        });
    }
    let mut all: Vec<f64> = breaks
        .iter()
        .copied()
        .filter(|&t| lo < t && t < hi)
        .collect();
    all.extend([lo, hi]);
    all.sort_by(f64::total_cmp);
    all.dedup();
    let net = enclosing_net(carrier, &all)?;
    Ok(CarrierNet {
        weights: vec![1.0; net.knots.control_count()],
        knots: net.knots,
        coords: net.coords,
        cuts: Vec::new(),
    })
}

/// **The interval a carrier's certificate spans**: the `f64` domain
/// the schedules and the composite cut, and its ends as the carrier
/// holds them. A spline spans its own knot domain, which its pcurves
/// are fitted on; any other carrier the outer ends of `params` (its
/// bracket's, so the span covers the edge's interval at every scalar),
/// held there. The edge certificate and the mint's image read this one
/// rule, so the image one certifies is the image the other stores.
pub(crate) fn carrier_span<T: Bounds>(carrier: &Curve3<T>, params: (T, T)) -> ((f64, f64), (T, T)) {
    if let Curve3::Nurbs(net) = carrier {
        return (net.domain(), net.knot_values().domain());
    }
    let (lo, hi) = outer_ends(params);
    ((lo, hi), (T::from_computed(lo), T::from_computed(hi)))
}

/// **An edge interval's outer ends**: the lowest point of either end's
/// bracket and the highest, so `[lo, hi]` covers the interval at every
/// scalar, in either order of its ends. The one spelling of that rule:
/// [`carrier_span`]'s closed-form span, the fitted row's coverage gate
/// and the analytic rung-3 tube's piece all read it here.
pub(crate) fn outer_ends<T: Bounds>((t0, t1): (T, T)) -> (f64, f64) {
    (t0.lo().min(t1.lo()), t0.hi().max(t1.hi()))
}

/// Whether every control of a span is a certified finite enclosure.
fn span_certifies(row: &[Interval]) -> bool {
    row.iter()
        .all(|c| c.is_certified() && Bounds::lo(*c).is_finite() && Bounds::hi(*c).is_finite())
}

/// The net of a closed-form `carrier` over `breaks` (strictly
/// increasing, both domain ends included).
///
/// # Errors
///
/// The first span whose controls do not certify; `breaks` itself where
/// it holds fewer than two points or forms no knot vector.
fn enclosing_net<T: CertifiedBounds>(
    carrier: &Curve3<T>,
    breaks: &[f64],
) -> Result<EnclosingNet, CarrierEnclosureRefusal> {
    let refuse = |span| CarrierEnclosureRefusal {
        kind: carrier.kind(),
        span,
    };
    let whole = (
        breaks.first().copied().unwrap_or(f64::NAN),
        breaks.last().copied().unwrap_or(f64::NAN),
    );
    let (first, last) = whole;
    if breaks.len() < 2 {
        return Err(refuse(whole));
    }
    if let Curve3::Line { origin, dir } = carrier {
        let (o, d) = (lift_p(*origin), lift_v(*dir));
        let mut knots = vec![first];
        knots.extend_from_slice(breaks);
        knots.push(last);
        let at = |t: f64| o + d * Interval::from_f64(t);
        let points: Vec<Vec3<Interval>> = breaks.iter().map(|&t| at(t)).collect();
        for (pair, ends) in breaks.windows(2).zip(points.windows(2)) {
            if !span_certifies(&[
                ends[0].x, ends[0].y, ends[0].z, ends[1].x, ends[1].y, ends[1].z,
            ]) {
                return Err(refuse((pair[0], pair[1])));
            }
        }
        return Ok(EnclosingNet {
            knots: KnotVector::clamped(knots, 1).map_err(|_| refuse(whole))?,
            coords: vec![
                points.iter().map(|p| p.x).collect(),
                points.iter().map(|p| p.y).collect(),
                points.iter().map(|p| p.z).collect(),
            ],
        });
    }
    let Some(h) = harmonic(carrier) else {
        return Err(refuse(whole));
    };
    let mut knots = vec![first; ORDER + 1];
    for &k in &breaks[1..breaks.len() - 1] {
        knots.extend(core::iter::repeat_n(k, ORDER));
    }
    knots.extend(core::iter::repeat_n(last, ORDER + 1));
    let mut coords: Vec<Vec<Interval>> = vec![Vec::new(), Vec::new(), Vec::new()];
    for pair in breaks.windows(2) {
        let span = span_bernstein(&h, pair[0], pair[1]);
        if !span.iter().all(|row| span_certifies(row)) {
            return Err(refuse((pair[0], pair[1])));
        }
        for (channel, mut row) in coords.iter_mut().zip(span) {
            if let Some(joint) = channel.pop() {
                row[0] = joint.enclosure_hull(row[0]);
            }
            channel.extend(row);
        }
    }
    Ok(EnclosingNet {
        knots: KnotVector::clamped(knots, ORDER).map_err(|_| refuse(whole))?,
        coords,
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::{CarrierEnclosureRefusal, carrier_net, enclosing_net};
    use geom::Curve3;
    use geom_core::interval::Interval;
    use geom_core::{Bounds, Point3, Real, Vec3};

    /// The net encloses the carrier at a dense sample of every span,
    /// for each closed-form kind: the Bernstein sum at `s` is enclosed
    /// by its interval evaluation, which must contain `C(t)` at `f64`,
    /// and no wider than `width` where one is asked.
    fn encloses(carrier: &Curve3<f64>, breaks: &[f64], width: Option<f64>) -> Result<(), String> {
        let net = enclosing_net(carrier, breaks).map_err(|e| e.to_string())?;
        let p = net.knots.degree();
        for (k, pair) in breaks.windows(2).enumerate() {
            for j in 0..=40 {
                let s = f64::from(j) / 40.0;
                let t = pair[0] + (pair[1] - pair[0]) * s;
                let c = carrier.eval(t);
                for (d, want) in [c.x, c.y, c.z].into_iter().enumerate() {
                    let ctl = &net.coords[d][k * p..=k * p + p];
                    let si = Interval::from_f64(s);
                    let one = Interval::one();
                    let binom =
                        |i: usize| (0..i).fold(1.0, |acc, q| acc * (p - q) as f64 / (q + 1) as f64);
                    let val = ctl
                        .iter()
                        .enumerate()
                        .fold(Interval::zero(), |acc, (i, b)| {
                            acc + *b
                                * Interval::from_f64(binom(i))
                                * si.powi(i32::try_from(i).unwrap_or(0))
                                * (one - si).powi(i32::try_from(p - i).unwrap_or(0))
                        });
                    // The f64 evaluation's own rounding sits inside a
                    // few ulps of the enclosure.
                    let slack = 1e-12;
                    if !(val.lo() - slack <= want && want <= val.hi() + slack) {
                        return Err(format!(
                            "span {k} s={s} channel {d}: {want} outside [{}, {}]",
                            val.lo(),
                            val.hi()
                        ));
                    }
                    if width.is_some_and(|w| val.hi() - val.lo() > w) {
                        return Err(format!(
                            "span {k} channel {d}: width {}",
                            val.hi() - val.lo()
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    fn grid(a: f64, b: f64, n: u32) -> Vec<f64> {
        (0..=n)
            .map(|i| a + (b - a) * f64::from(i) / f64::from(n))
            .collect()
    }

    #[test]
    fn every_closed_form_kind_is_enclosed_span_by_span() {
        let axis = Vec3::new(0.0, 0.0, 1.0);
        let u_ref = Vec3::new(1.0, 0.0, 0.0);
        let center = Point3::new(0.3, -0.2, 0.5);
        let kinds = [
            (
                "line",
                Curve3::Line {
                    origin: center,
                    dir: Vec3::new(0.2, 0.7, -0.1),
                },
            ),
            (
                "circle",
                Curve3::Circle {
                    center,
                    axis,
                    radius: 0.8,
                    u_ref,
                },
            ),
            (
                "ellipse",
                Curve3::Ellipse {
                    center,
                    axis,
                    semi_u: 1.2,
                    semi_v: 0.4,
                    u_ref,
                },
            ),
            (
                "spiric",
                Curve3::Spiric {
                    center,
                    axis,
                    u_ref,
                    major_radius: 2.0,
                    minor_radius: 0.5,
                    offset: 0.7,
                },
            ),
        ];
        for (name, carrier) in kinds {
            assert_eq!(
                encloses(&carrier, &grid(-0.4, 2.9, 32), Some(1e-9)),
                Ok(()),
                "the {name} carrier's net encloses it tightly on fine spans"
            );
            // Spans two radians wide, where the polynomial alone misses
            // the carrier by far more than the slack: the remainder
            // carries the enclosure.
            assert_eq!(
                encloses(&carrier, &grid(-0.4, 3.6, 2), None),
                Ok(()),
                "the {name} carrier's net encloses it on coarse spans"
            );
        }
    }

    /// On uneven spans the joint control must hold both neighbours' end:
    /// a two-radian span's end control is wide by its remainder, a
    /// thousandth-wide span's start is tight, and either one alone
    /// misses the other span's carrier near the break. The hull holds
    /// both, in either order of widths: a joint that keeps only the
    /// wider span's end, or only the narrower one's, leaves a circle
    /// point outside the net.
    #[test]
    fn r2_uneven_spans_need_the_joint_hull() {
        let circle = Curve3::Circle {
            center: Point3::new(0.3, -0.2, 0.5),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 0.8,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        for breaks in [
            &[-0.4, 1.6, 1.601][..],
            &[-0.4, -0.399, 1.6],
            &[-0.4, 0.0, 2.0, 2.0001, 4.0],
        ] {
            assert_eq!(
                encloses(&circle, breaks, None),
                Ok(()),
                "the circle's net encloses it across the joint of {breaks:?}"
            );
        }
    }

    /// A spiric whose square root's argument reaches zero (offset equal
    /// to the inner radius, so `(R − r)² − d² = 0` at `t = π`) has no
    /// certified net over the span that holds the zero, and the door
    /// names that span and the kind; the same carrier certifies over a
    /// domain clear of it.
    #[test]
    fn the_door_names_the_kind_and_span_it_cannot_enclose() {
        let carrier = Curve3::Spiric {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
            major_radius: 2.0,
            minor_radius: 0.5,
            offset: 1.5,
        };
        assert_eq!(
            carrier_net(&carrier, (2.9, 3.3), &[3.0, 3.1, 3.2]).err(),
            Some(CarrierEnclosureRefusal {
                kind: geom::CurveKind::Spiric,
                span: (3.1, 3.2),
            }),
            "the span holding t = π is named, not the domain"
        );
        assert!(
            carrier_net(&carrier, (2.0, 3.0), &[2.5]).is_ok(),
            "the spiric certifies clear of its root's zero"
        );
    }

    /// The door's per-kind nets: a spline is its own net, the breaks
    /// left for the composite to insert; a line its exact degree-1 net
    /// over the breaks inside the domain and the domain's ends; a circle
    /// its degree-`ORDER` enclosure on the same breaks.
    #[test]
    fn the_door_reads_each_kind_as_its_own_net() {
        let spline = geom::NurbsCurve3::new(
            geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
            vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(1.0, 2.0, 0.0),
                Point3::new(3.0, 0.0, 1.0),
            ],
            vec![1.0, 0.5, 1.0],
        )
        .unwrap();
        let net = carrier_net(
            &Curve3::Nurbs(std::sync::Arc::new(spline.clone())),
            (5.0, 6.0),
            &[0.25, 0.5],
        )
        .unwrap();
        let (knots, weights, coords) = net.operand();
        assert_eq!(
            knots,
            spline.knots_f64_interim(),
            "a spline's knots are its own"
        );
        assert_eq!(weights, spline.weights_f64_interim(), "and its weights");
        let bits = |c: &[Vec<Interval>]| -> Vec<(f64, f64)> {
            c.iter().flatten().map(|i| (i.lo(), i.hi())).collect()
        };
        assert_eq!(
            bits(coords),
            bits(&spline.certified_coords()),
            "and its controls"
        );
        assert_eq!(net.cuts(), &[0.25, 0.5], "its breaks ride as cuts");

        let line = Curve3::Line {
            origin: Point3::new(0.0, 1.0, 0.0),
            dir: Vec3::new(1.0, 0.0, 0.0),
        };
        let net = carrier_net(&line, (0.0, 2.0), &[1.0, 3.0, 1.0]).unwrap();
        let (knots, _, coords) = net.operand();
        assert_eq!(knots.degree(), 1, "a line is degree 1");
        assert_eq!(
            knots.knots(),
            &[0.0, 0.0, 1.0, 2.0, 2.0],
            "on the breaks inside the domain and its ends, once each"
        );
        assert_eq!(
            bits(&coords[..1]),
            vec![(0.0, 0.0), (1.0, 1.0), (2.0, 2.0)],
            "its controls are its points at the breaks"
        );
        assert!(net.cuts().is_empty(), "a closed form owes no cuts");

        let circle = Curve3::Circle {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let net = carrier_net(&circle, (0.0, 1.0), &[0.5]).unwrap();
        assert_eq!(net.operand().0.degree(), super::ORDER, "a circle at ORDER");
        assert_eq!(net.operand().0.control_count(), 2 * super::ORDER + 1);
    }
}
