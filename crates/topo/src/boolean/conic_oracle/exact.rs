//! **The conic rows' exact oracle**: a conic carrier's true crossings with
//! a sphere, a cylinder wall or a torus, and how deep its shallowest
//! extreme sits, in double-double arithmetic on the carrier and surface
//! AS STORED (`v̂ = n̂ × û` taken exactly, the surface's axis normalized
//! exactly), so a crossing whose `f64` residual cannot place it is placed
//! here to about `1e-30` of the carrier's scale.
//!
//! Along `C(θ) = C₀ + A cos θ + B sin θ` (`A = a·û`, `B = b·v̂`), with
//! `θ = φ₀ + 2·atan t`, the offset from the surface's anchor is
//! `q̃(t)/(1 + t²)` with `q̃` quadratic in `t`, so the surface's implicit
//! `F` times `(1 + t²)ᵏ` is a polynomial `G(t)` of degree `2k` (`k = 2`
//! on a quadric, `4` on a torus) that shares `F`'s sign. Its real roots
//! with a sign change are isolated between consecutive roots of its
//! derivative, recursively, and bisected; `F`'s extremes in `θ` are the
//! roots of `G′(1 + t²) − 2k·t·G` found the same way. The anchor `φ₀` is
//! the quarter turn whose opposite point is farthest from the surface,
//! so no crossing sits near the substitution's pole.

use core::f64::consts::{FRAC_PI_2, TAU};

use geom_core::{Point3, Vec3};

/// A double-double: the unevaluated sum `hi + lo`, `|lo| ≤ ulp(hi)/2`.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Dd {
    hi: f64,
    lo: f64,
}

fn two_sum(a: f64, b: f64) -> Dd {
    let s = a + b;
    let v = s - a;
    Dd {
        hi: s,
        lo: (a - (s - v)) + (b - v),
    }
}

fn quick_two_sum(a: f64, b: f64) -> Dd {
    let s = a + b;
    Dd {
        hi: s,
        lo: b - (s - a),
    }
}

impl Dd {
    const ZERO: Self = Self { hi: 0.0, lo: 0.0 };

    fn of(x: f64) -> Self {
        Self { hi: x, lo: 0.0 }
    }

    fn add(self, o: Self) -> Self {
        let s = two_sum(self.hi, o.hi);
        let t = two_sum(self.lo, o.lo);
        let s = quick_two_sum(s.hi, s.lo + t.hi);
        quick_two_sum(s.hi, s.lo + t.lo)
    }

    fn neg(self) -> Self {
        Self {
            hi: -self.hi,
            lo: -self.lo,
        }
    }

    fn sub(self, o: Self) -> Self {
        self.add(o.neg())
    }

    fn mul(self, o: Self) -> Self {
        let p = self.hi * o.hi;
        let e = self.hi.mul_add(o.hi, -p);
        quick_two_sum(p, e + (self.hi * o.lo + self.lo * o.hi))
    }

    fn scale(self, k: f64) -> Self {
        self.mul(Self::of(k))
    }

    fn div(self, o: Self) -> Self {
        let q1 = self.hi / o.hi;
        let r = self.sub(o.scale(q1));
        let q2 = r.hi / o.hi;
        let r = r.sub(o.scale(q2));
        let q3 = r.hi / o.hi;
        quick_two_sum(q1, q2).add(Self::of(q3))
    }

    fn sqrt(self) -> Self {
        if self.hi <= 0.0 {
            return Self::ZERO;
        }
        let s = self.hi.sqrt();
        let r = self.sub(Self::of(s).mul(Self::of(s)));
        Self::of(s).add(Self::of(r.hi / (2.0 * s)))
    }

    fn signum(self) -> i8 {
        if self.hi > 0.0 {
            1
        } else if self.hi < 0.0 {
            -1
        } else {
            0
        }
    }

    fn to_f64(self) -> f64 {
        self.hi + self.lo
    }
}

type V3 = [Dd; 3];

fn dd3(v: Vec3<f64>) -> V3 {
    [Dd::of(v.x), Dd::of(v.y), Dd::of(v.z)]
}

/// `p − o` exactly, each component as a double-double.
fn offset(p: Point3<f64>, o: Point3<f64>) -> V3 {
    [two_sum(p.x, -o.x), two_sum(p.y, -o.y), two_sum(p.z, -o.z)]
}

fn dot(a: V3, b: V3) -> Dd {
    a[0].mul(b[0]).add(a[1].mul(b[1])).add(a[2].mul(b[2]))
}

fn cross(a: V3, b: V3) -> V3 {
    [
        a[1].mul(b[2]).sub(a[2].mul(b[1])),
        a[2].mul(b[0]).sub(a[0].mul(b[2])),
        a[0].mul(b[1]).sub(a[1].mul(b[0])),
    ]
}

fn scaled(v: V3, k: Dd) -> V3 {
    [v[0].mul(k), v[1].mul(k), v[2].mul(k)]
}

fn unit(v: V3) -> V3 {
    let n = dot(v, v).sqrt();
    [v[0].div(n), v[1].div(n), v[2].div(n)]
}

/// A polynomial in `t`, lowest coefficient first.
type Poly = Vec<Dd>;

fn p_add(a: &[Dd], b: &[Dd]) -> Poly {
    (0..a.len().max(b.len()))
        .map(|i| {
            let x = a.get(i).copied().unwrap_or(Dd::ZERO);
            x.add(b.get(i).copied().unwrap_or(Dd::ZERO))
        })
        .collect()
}

fn p_scale(a: &[Dd], k: Dd) -> Poly {
    a.iter().map(|x| x.mul(k)).collect()
}

fn p_mul(a: &[Dd], b: &[Dd]) -> Poly {
    let mut out = vec![Dd::ZERO; a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            out[i + j] = out[i + j].add(x.mul(*y));
        }
    }
    out
}

fn p_derivative(a: &[Dd]) -> Poly {
    a.iter()
        .enumerate()
        .skip(1)
        .map(|(i, x)| x.scale(i as f64))
        .collect()
}

fn p_eval(a: &[Dd], t: Dd) -> Dd {
    a.iter().rev().fold(Dd::ZERO, |acc, x| acc.mul(t).add(*x))
}

fn trimmed(mut a: Poly) -> Poly {
    while a.last().is_some_and(|x| x.hi == 0.0) {
        a.pop();
    }
    a
}

/// The real roots of `p` at which it changes sign, ascending: isolated
/// between consecutive such roots of `p′` (between which `p` is
/// monotone) and the Cauchy bound, then bisected to the double-double's
/// resolution.
fn sign_change_roots(p: &[Dd]) -> Vec<Dd> {
    let p = trimmed(p.to_vec());
    match p.len() {
        0 | 1 => return Vec::new(),
        2 => return vec![p[0].neg().div(p[1])],
        _ => {}
    }
    let lead = p[p.len() - 1].to_f64().abs();
    let bound = 1.0
        + p[..p.len() - 1]
            .iter()
            .map(|x| x.to_f64().abs() / lead)
            .fold(0.0, f64::max);
    let mut ends = vec![Dd::of(-bound)];
    ends.extend(sign_change_roots(&p_derivative(&p)));
    ends.push(Dd::of(bound));
    let mut out = Vec::new();
    for w in ends.windows(2) {
        let (mut a, mut b) = (w[0], w[1]);
        let (sa, sb) = (p_eval(&p, a).signum(), p_eval(&p, b).signum());
        if sa * sb >= 0 {
            continue;
        }
        for _ in 0..400 {
            let m = a.add(b).scale(0.5);
            if m == a || m == b {
                break;
            }
            if p_eval(&p, m).signum() == sa {
                a = m;
            } else {
                b = m;
            }
        }
        out.push(a.add(b).scale(0.5));
    }
    out
}

/// The surfaces the oracle reads, each from its stored anchor.
enum Form {
    Sphere { r: Dd },
    Wall { axis: V3, r: Dd },
    Torus { axis: V3, big: Dd, small: Dd },
}

/// A conic's true crossings with a surface ([`truth`]).
#[derive(Debug)]
pub(in crate::boolean) struct Truth {
    /// Each crossing's parameter, in `[0, 2π)`, ascending.
    pub(in crate::boolean) roots: Vec<f64>,
    /// The least `|d|` of the true signed distance over `F`'s extremes
    /// along the carrier (`∞` with none): a pose is outside a band `K·ε`
    /// deep when this is at least `K·ε`.
    pub(in crate::boolean) depth: f64,
}

/// The true crossings of `carrier`, a circle or an ellipse, with `surface`,
/// a sphere, a cylinder wall or a torus, over the whole turn (module docs).
///
/// # Panics
///
/// On any other carrier or surface: the oracle reads these kinds only.
#[allow(clippy::panic)]
pub(in crate::boolean) fn truth(
    carrier: &geom::Curve3<f64>,
    surface: &geom::Surface<f64>,
) -> Truth {
    let (center, axis, (a, b), u_ref) = match *carrier {
        geom::Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } => (center, axis, (radius, radius), u_ref),
        geom::Curve3::Ellipse {
            center,
            axis,
            major,
            minor,
            u_ref,
        } => (center, axis, (major, minor), u_ref),
        _ => panic!("the exact oracle reads circles and ellipses"),
    };
    let (anchor, form) = match *surface {
        geom::Surface::Sphere { center, radius, .. } => {
            (center, Form::Sphere { r: Dd::of(radius) })
        }
        geom::Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => (
            origin,
            Form::Wall {
                axis: unit(dd3(axis)),
                r: Dd::of(radius),
            },
        ),
        geom::Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => (
            center,
            Form::Torus {
                axis: unit(dd3(axis)),
                big: Dd::of(major_radius),
                small: Dd::of(minor_radius),
            },
        ),
        _ => panic!("the exact oracle reads spheres, walls and tori"),
    };
    let u = dd3(u_ref);
    let big_a = scaled(u, Dd::of(a));
    let big_b = scaled(cross(dd3(axis), u), Dd::of(b));
    let w = offset(center, anchor);
    let one_plus = vec![Dd::of(1.0), Dd::ZERO, Dd::of(1.0)];
    // `q̃(t)` about the anchor `φ₀ = k·π/2`: the quarter turns carry
    // `(A, B)` to `(B, −A)`, exactly.
    let frame = |k: usize| match k {
        0 => (big_a, big_b),
        1 => (big_b, scaled(big_a, Dd::of(-1.0))),
        2 => (scaled(big_a, Dd::of(-1.0)), scaled(big_b, Dd::of(-1.0))),
        _ => (scaled(big_b, Dd::of(-1.0)), big_a),
    };
    let q_tilde = |k: usize| -> [Poly; 3] {
        let (fa, fb) = frame(k);
        core::array::from_fn(|i| vec![w[i].add(fa[i]), fb[i].scale(2.0), w[i].sub(fa[i])])
    };
    // `G = F·(1 + t²)ᵏ` and `k`, for the anchor's `q̃`.
    let g_of = |q: &[Poly; 3]| -> (Poly, usize) {
        let s = p_add(
            &p_add(&p_mul(&q[0], &q[0]), &p_mul(&q[1], &q[1])),
            &p_mul(&q[2], &q[2]),
        );
        let p2 = p_mul(&one_plus, &one_plus);
        let along = |ax: V3| {
            p_add(
                &p_add(&p_scale(&q[0], ax[0]), &p_scale(&q[1], ax[1])),
                &p_scale(&q[2], ax[2]),
            )
        };
        match form {
            Form::Sphere { r } => (p_add(&s, &p_scale(&p2, r.mul(r).neg())), 2),
            Form::Wall { axis, r } => {
                let h = along(axis);
                let perp = p_add(&s, &p_scale(&p_mul(&h, &h), Dd::of(-1.0)));
                (p_add(&perp, &p_scale(&p2, r.mul(r).neg())), 2)
            }
            Form::Torus { axis, big, small } => {
                let h = along(axis);
                let k = big.mul(big).sub(small.mul(small));
                let first = p_add(&s, &p_scale(&p2, k));
                let perp = p_add(&s, &p_scale(&p_mul(&h, &h), Dd::of(-1.0)));
                let g = p_add(
                    &p_mul(&first, &first),
                    &p_scale(&p_mul(&perp, &p2), big.mul(big).scale(-4.0)),
                );
                (g, 4)
            }
        }
    };
    // The anchor whose pole `φ₀ + π` (the leading coefficient) reads
    // farthest from zero.
    let (k, (g, half)) = (0..4)
        .map(|k| (k, g_of(&q_tilde(k))))
        .max_by(|x, y| {
            let lead = |g: &Poly| g.last().map_or(0.0, |c| c.to_f64().abs());
            lead(&x.1.0).total_cmp(&lead(&y.1.0))
        })
        .unwrap_or_else(|| unreachable!());
    let phi0 = FRAC_PI_2 * k as f64;
    let theta = |t: Dd| (phi0 + 2.0 * t.to_f64().atan()).rem_euclid(TAU);
    let mut roots: Vec<f64> = sign_change_roots(&g).into_iter().map(theta).collect();
    roots.sort_by(f64::total_cmp);
    // `F`'s extremes in `θ`: `dF/dθ ∝ G′(1 + t²) − 2k·t·G`, read about
    // every anchor, so an extreme at one anchor's pole is found at
    // another's.
    let distance = |q: &[Poly; 3], t: Dd| {
        let den = Dd::of(1.0).add(t.mul(t));
        let p: V3 = core::array::from_fn(|i| p_eval(&q[i], t).div(den));
        let s = dot(p, p);
        match form {
            Form::Sphere { r } => s.sqrt().sub(r),
            Form::Wall { axis, r } => {
                let h = dot(p, axis);
                s.sub(h.mul(h)).sqrt().sub(r)
            }
            Form::Torus { axis, big, small } => {
                let h = dot(p, axis);
                let rho = s.sub(h.mul(h)).sqrt().sub(big);
                rho.mul(rho).add(h.mul(h)).sqrt().sub(small)
            }
        }
        .to_f64()
    };
    let depth = (0..4)
        .flat_map(|k| {
            let q = q_tilde(k);
            let (g, _) = g_of(&q);
            let slope = p_add(
                &p_mul(&p_derivative(&g), &one_plus),
                &p_mul(&[Dd::ZERO, Dd::of(-2.0 * half as f64)], &g),
            );
            sign_change_roots(&slope)
                .into_iter()
                .map(|t| distance(&q, t).abs())
                .collect::<Vec<_>>()
        })
        .fold(f64::INFINITY, f64::min);
    Truth { roots, depth }
}
