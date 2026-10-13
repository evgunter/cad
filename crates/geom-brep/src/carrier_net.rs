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
//! spiric whose square root's argument reaches zero over a span) makes
//! the net `None`, and the caller refuses typed.

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
pub(crate) struct EnclosingNet {
    pub(crate) knots: KnotVector,
    pub(crate) coords: Vec<Vec<Interval>>,
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

/// `None` for a spline carrier, whose net is its own.
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
            semi_u: major,
            semi_v: minor,
            u_ref,
        } => {
            let (axis, u_ref) = (lift_v(*axis), lift_v(*u_ref));
            Some(Harmonic {
                p0: lift_p(*center),
                u: u_ref * i(*major),
                v: axis.cross(u_ref) * i(*minor),
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
    let binom = |i: usize, k: usize| -> f64 {
        (0..k).fold(1.0, |acc, j| acc * (i - j) as f64 / (j + 1) as f64)
    };
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

/// The net of `carrier` over `breaks` (strictly increasing, both domain
/// ends included): `None` for a spline carrier, for fewer than two
/// breaks, or where a coefficient does not certify.
pub(crate) fn enclosing_net<T: CertifiedBounds>(
    carrier: &Curve3<T>,
    breaks: &[f64],
) -> Option<EnclosingNet> {
    let (&first, &last) = (breaks.first()?, breaks.last()?);
    if breaks.len() < 2 {
        return None;
    }
    let net = if let Curve3::Line { origin, dir } = carrier {
        let (o, d) = (lift_p(*origin), lift_v(*dir));
        let mut knots = vec![first];
        knots.extend_from_slice(breaks);
        knots.push(last);
        let at = |t: f64| o + d * Interval::from_f64(t);
        EnclosingNet {
            knots: KnotVector::clamped(knots, 1).ok()?,
            coords: vec![
                breaks.iter().map(|&t| at(t).x).collect(),
                breaks.iter().map(|&t| at(t).y).collect(),
                breaks.iter().map(|&t| at(t).z).collect(),
            ],
        }
    } else {
        let h = harmonic(carrier)?;
        let mut knots = vec![first; ORDER + 1];
        for &k in &breaks[1..breaks.len() - 1] {
            knots.extend(core::iter::repeat_n(k, ORDER));
        }
        knots.extend(core::iter::repeat_n(last, ORDER + 1));
        let mut coords: Vec<Vec<Interval>> = vec![Vec::new(), Vec::new(), Vec::new()];
        for pair in breaks.windows(2) {
            let span = span_bernstein(&h, pair[0], pair[1]);
            for (channel, mut row) in coords.iter_mut().zip(span) {
                if let Some(joint) = channel.pop() {
                    row[0] = joint.enclosure_hull(row[0]);
                }
                channel.extend(row);
            }
        }
        EnclosingNet {
            knots: KnotVector::clamped(knots, ORDER).ok()?,
            coords,
        }
    };
    net.coords
        .iter()
        .flatten()
        .all(|c| c.is_certified() && Bounds::lo(*c).is_finite() && Bounds::hi(*c).is_finite())
        .then_some(net)
}

#[cfg(test)]
mod tests {
    use super::enclosing_net;
    use geom::Curve3;
    use geom_core::interval::Interval;
    use geom_core::{Bounds, Point3, Real, Vec3};

    /// The net encloses the carrier at a dense sample of every span,
    /// for each closed-form kind: the Bernstein sum at `s` is enclosed
    /// by its interval evaluation, which must contain `C(t)` at `f64`,
    /// and no wider than `width` where one is asked.
    fn encloses(carrier: &Curve3<f64>, breaks: &[f64], width: Option<f64>) -> Result<(), String> {
        let net = enclosing_net(carrier, breaks).ok_or("no net")?;
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
    /// to the inner radius) has no certified net over a span that holds
    /// the zero: the caller refuses on `None`.
    #[test]
    fn a_square_root_reaching_zero_has_no_net() {
        let carrier = Curve3::Spiric {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
            major_radius: 2.0,
            minor_radius: 0.5,
            offset: 1.5,
        };
        assert!(
            enclosing_net(&carrier, &grid(3.0, 3.3, 2)).is_none(),
            "a span holding t = π has (R − r)² − d² = 0 under its root"
        );
    }

    #[test]
    fn a_spline_carrier_takes_no_net() {
        assert!(enclosing_net(&Curve3::<f64>::nurbs_placeholder(), &[0.0, 1.0]).is_none());
    }
}
