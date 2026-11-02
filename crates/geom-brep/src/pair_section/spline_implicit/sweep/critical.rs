//! **A critical point of `φ` decided** (C5): the cells the regularity
//! sweep cannot read at the floor, clustered, each cluster isolated as
//! one zero of `∇φ` by the Krawczyk operator and decided by its Hessian
//! and its value.
//!
//! On a box `X` of the wall's chart, `∇φ = (∇f(S)·S_u, ∇f(S)·S_v)` and
//! its Jacobian, `φ`'s Hessian
//! `φ_ab = S_aᵀ ∇²f S_b + ∇f·S_ab`, are enclosed through the wall's jet
//! over `X` in interval arithmetic ([`geom::NurbsSurface::ders`] at the
//! box's parameters) and the partner's form, its gradient and its
//! Hessian over the box of `S(X)` (a plane's `∇f` is its unit normal and
//! its `∇²f` zero). With `m` the box's middle and `Y` the inverse of the
//! Hessian's midpoint,
//! `K = m − Y·∇φ(m) + (I − Y·∇²φ(X))·(X − m)` holds every zero of `∇φ` in
//! `X`, and where `K` lies strictly inside `X` it holds one: **existence**
//! is what containment proves, and with it every matrix in `∇²φ(X)` is
//! invertible, so the zero is the only one (`crate::krawczyk`'s operator,
//! two dimensions down). Without it, a box over a regular stretch where
//! `∇φ` is small but nowhere zero would be read as a critical point that
//! is not there. A box `K` does not fit is cut to `K ∩ X` and read again,
//! a fixed number of times; each cut keeps every zero of the first box,
//! so the one zero found is the first box's only one.
//!
//! The point found is then decided:
//!
//! - **its value**, `φ` over the final box in metres, read on a ceiling
//!   against the band (`spline_critical_value`): decided zero, the point
//!   is a tangency decided Zero; in band or past it, the cluster is not
//!   decided here and the sweep declines;
//! - **its Hessian's determinant** over the final box: positive, a definite
//!   Hessian, a **touch**; negative, an indefinite one, a **node**, the
//!   arcs of the section crossing there; straddling zero, the sweep
//!   declines.
//!
//! A box over a knot reads the jets of every span it touches, hulled, so
//! a cluster at a knot reads a hull the Krawczyk test may not fit, and
//! is then not decided. The caller declines a decided point whose final
//! box is not strictly inside its piece of the rectangle.

use geom::NurbsSurface;
use geom_core::interval::certification::Certification;
use geom_core::k_stats::{Magnitude, decide_magnitude_reported};
use geom_core::{Band, Bounds, Interval, Margin, MarginDiag, Vec3};

use super::Phi;
use crate::ssi::exhaust::UvRect;

/// The most times a box `K` does not fit is cut to `K ∩ X` and read
/// again (D9, a fixed rule).
const KRAWCZYK_ROUNDS: u32 = 12;

/// What a critical point decided Zero is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::pair_section) enum Kind {
    /// A definite Hessian: the surfaces touch at one point.
    Touch,
    /// An indefinite Hessian: the section's arcs cross at the point.
    Node,
}

/// **A critical point of `φ` decided Zero**: its chart point (the last
/// box's middle), the box that holds it, what it is, and the margin of
/// its value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::pair_section) struct Critical {
    /// The chart point.
    pub at: (f64, f64),
    /// The box in which `∇φ` has this one zero.
    pub cell: UvRect,
    /// A touch or a node.
    pub kind: Kind,
    /// The value's margin, decided zero.
    pub margin: MarginDiag,
}

/// The wall's chart, read in interval arithmetic: its jet over a box.
pub(super) struct Jets {
    net: NurbsSurface<Interval>,
}

impl Jets {
    /// The wall's net, its knots, points and weights as point enclosures;
    /// `None` where it does not lift.
    pub(super) fn of(net: &NurbsSurface<f64>) -> Option<Self> {
        let ku = net.knot_values_u().map(Interval::point).ok()?;
        let kv = net.knot_values_v().map(Interval::point).ok()?;
        let control = net
            .control()
            .iter()
            .map(|p| p.map(Interval::point))
            .collect();
        let weights = net.weights().iter().map(|&w| Interval::point(w)).collect();
        NurbsSurface::new(ku, kv, control, weights)
            .ok()
            .map(|net| Self { net })
    }
}

/// `∇φ` and `∇²φ` over the box `(u, v)`, and `φ` there as an enclosure
/// with a ceiling in metres on its distance from the partner.
struct Read {
    gradient: [Interval; 2],
    hessian: [[Interval; 2]; 2],
    value: Interval,
    ceiling: f64,
}

/// Reads `φ` of `phi` over the box `(u, v)` of the wall's chart.
fn read(jets: &Jets, phi: Phi<'_>, u: Interval, v: Interval) -> Read {
    let jet = jets.net.ders(u, v);
    let p = Vec3::new(jet.point.x, jet.point.y, jet.point.z);
    let (g, h, value, ceiling, frame): (Vec3<Interval>, [[Interval; 3]; 3], Interval, f64, _) =
        match phi {
            Phi::Plane(origin, normal) => {
                let (o, n) = plane(origin, normal);
                let value = (p - o).dot(n);
                let ceiling = value.lo().abs().max(value.hi().abs());
                let zero = Interval::point(0.0);
                (n, [[zero; 3]; 3], value, ceiling, None)
            }
            Phi::Curved(curved) => {
                let q = curved.framed(crate::ssi::enclose::Box3 {
                    x: p.x,
                    y: p.y,
                    z: p.z,
                });
                let value = curved.value(q);
                let ceiling = curved.ceiling_metres(value.lo().abs().max(value.hi().abs()), q);
                (
                    curved.frame_gradient(q),
                    curved.frame_hessian(q),
                    value,
                    ceiling,
                    Some(curved),
                )
            }
        };
    // Every vector in the frame `g` and `h` are written in.
    let framed = |d: Vec3<Interval>| frame.map_or(d, |c| c.frame_vector(d));
    let (su, sv) = (framed(jet.du), framed(jet.dv));
    let (suu, suv, svv) = (framed(jet.duu), framed(jet.duv), framed(jet.dvv));
    let quad = |a: Vec3<Interval>, b: Vec3<Interval>| {
        let a = [a.x, a.y, a.z];
        let b = [b.x, b.y, b.z];
        let mut acc = Interval::point(0.0);
        for (i, row) in h.iter().enumerate() {
            for (j, e) in row.iter().enumerate() {
                acc = acc + a[i] * *e * b[j];
            }
        }
        acc
    };
    let huv = quad(su, sv) + g.dot(suv);
    Read {
        gradient: [g.dot(su), g.dot(sv)],
        hessian: [
            [quad(su, su) + g.dot(suu), huv],
            [huv, quad(sv, sv) + g.dot(svv)],
        ],
        value,
        ceiling,
    }
}

/// **`φ` along a crease line**, over the stretch `(u, v)` of it (one of
/// the two a point): `φ` there in metres, a floor on its distance,
/// signed; and the line's slope off the partner, `∇f·S_d` over
/// `sup‖∇f‖·sup‖S_d‖`, `d` the line's own direction (`along_u`), a
/// derivative continuous across the crease. Read through the wall's jet
/// at the stretch, so the derivative is the line's own and not the
/// cell's beside it.
pub(super) fn along_line(
    jets: &Jets,
    phi: Phi<'_>,
    (u, v): (Interval, Interval),
    along_u: bool,
) -> (Interval, Interval) {
    let jet = jets.net.ders(u, v);
    let p = Vec3::new(jet.point.x, jet.point.y, jet.point.z);
    let d = if along_u { jet.du } else { jet.dv };
    let (metres, g, d) = match phi {
        Phi::Plane(origin, normal) => {
            let (o, n) = plane(origin, normal);
            ((p - o).dot(n), n, d)
        }
        Phi::Curved(curved) => {
            let q = curved.framed(crate::ssi::enclose::Box3 {
                x: p.x,
                y: p.y,
                z: p.z,
            });
            (
                curved.floor_metres(curved.value(q), q),
                curved.frame_gradient(q),
                curved.frame_vector(d),
            )
        }
    };
    let norm = |w: Vec3<Interval>| {
        Interval::point(Certification::sqrt(w.x.sqr() + w.y.sqr() + w.z.sqr()).hi())
    };
    (metres, super::sine(g, norm(g), d, norm(d)))
}

/// A plane's origin and its unit normal, as enclosures: its `∇f`, and the
/// point its height is read from.
fn plane(origin: geom_core::Point3<f64>, normal: Vec3<f64>) -> (Vec3<Interval>, Vec3<Interval>) {
    let n = normal.map(Interval::point);
    let length = Certification::sqrt(n.x.sqr() + n.y.sqr() + n.z.sqr());
    (
        Vec3::new(origin.x, origin.y, origin.z).map(Interval::point),
        Vec3::new(n.x / length, n.y / length, n.z / length),
    )
}

/// The inverse of a 2×2 matrix, `None` where it is singular or not
/// finite.
fn inverse2(m: [[f64; 2]; 2]) -> Option<[[f64; 2]; 2]> {
    let det = m[0][0] * m[1][1] - m[0][1] * m[1][0];
    if !det.is_finite() || det == 0.0 {
        return None;
    }
    Some([
        [m[1][1] / det, -m[0][1] / det],
        [-m[1][0] / det, m[0][0] / det],
    ])
}

/// `[lo, hi]` as an interval, or `None` where it is inverted or not
/// finite.
fn range((lo, hi): (f64, f64)) -> Option<Interval> {
    (lo <= hi && lo.is_finite() && hi.is_finite()).then(|| Interval::from_bounds(lo, hi))
}

/// The middle of an interval.
fn mid(x: Interval) -> f64 {
    0.5 * (x.lo() + x.hi())
}

/// **The Krawczyk box of `x`**, `K ∩ x`, and whether `K` lies strictly
/// inside `x` (module docs). `None` where a reading does not certify or
/// the Hessian's midpoint is singular.
fn krawczyk(jets: &Jets, phi: Phi<'_>, x: [Interval; 2]) -> Option<([Interval; 2], bool)> {
    let m = [mid(x[0]), mid(x[1])];
    let at = read(jets, phi, Interval::point(m[0]), Interval::point(m[1]));
    let over = read(jets, phi, x[0], x[1]);
    let certified = |i: &Interval| i.is_certified();
    if !(at.gradient.iter().all(certified) && over.hessian.iter().flatten().all(certified)) {
        return None;
    }
    let y = inverse2(over.hessian.map(|row| row.map(mid)))?;
    let mut k = [Interval::point(0.0); 2];
    for i in 0..2 {
        let mut acc = Interval::point(m[i]);
        for j in 0..2 {
            acc = acc - Interval::point(y[i][j]) * at.gradient[j];
            let mut c = Interval::point(if i == j { 1.0 } else { 0.0 });
            for (l, row) in over.hessian.iter().enumerate() {
                c = c - Interval::point(y[i][l]) * row[j];
            }
            acc = acc + c * (x[j] - Interval::point(m[j]));
        }
        k[i] = acc;
    }
    if !k.iter().all(certified) {
        return None;
    }
    let inside = (0..2).all(|i| x[i].lo() < k[i].lo() && k[i].hi() < x[i].hi());
    let meet = [
        range((x[0].lo().max(k[0].lo()), x[0].hi().min(k[0].hi())))?,
        range((x[1].lo().max(k[1].lo()), x[1].hi().min(k[1].hi())))?,
    ];
    Some((meet, inside))
}

/// **The cluster in `x` decided** (module docs): the critical point it
/// holds decided Zero, or `None` where it is not decided here. `x` lies
/// inside one piece of the rectangle between its creases, over which
/// `∇φ` is continuous.
pub(super) fn decide(jets: &Jets, phi: Phi<'_>, x: UvRect, band: Band) -> Option<Critical> {
    let mut b = [range(x.u)?, range(x.v)?];
    let mut proved = false;
    for _ in 0..KRAWCZYK_ROUNDS {
        let (next, inside) = krawczyk(jets, phi, b)?;
        let narrower = next[0].hi() - next[0].lo() < b[0].hi() - b[0].lo()
            || next[1].hi() - next[1].lo() < b[1].hi() - b[1].lo();
        b = next;
        proved |= inside;
        if !narrower {
            break;
        }
    }
    if !proved {
        return None;
    }
    let at = read(jets, phi, b[0], b[1]);
    let det = at.hessian[0][0] * at.hessian[1][1] - at.hessian[0][1] * at.hessian[1][0];
    if !det.is_certified() {
        return None;
    }
    let kind = if det.lo() > 0.0 {
        Kind::Touch
    } else if det.hi() < 0.0 {
        Kind::Node
    } else {
        return None;
    };
    if !at.value.is_certified() {
        return None;
    }
    match decide_magnitude_reported("spline_critical_value", Margin::of(at.ceiling), band) {
        Ok((Magnitude::Zero, margin)) => Some(Critical {
            at: (mid(b[0]), mid(b[1])),
            cell: UvRect {
                u: (b[0].lo(), b[0].hi()),
                v: (b[1].lo(), b[1].hi()),
            },
            kind,
            margin,
        }),
        Ok((Magnitude::Positive, _)) | Err(_) => None,
    }
}
