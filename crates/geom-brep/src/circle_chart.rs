//! **A circle against a cylinder, cone or torus**: a closed-form bound
//! on the chart's canonical form along a circle over the whole circle,
//! the incidence term of a circle's projected row on those charts
//! (`pcurve_cache::projected`). The sphere's is
//! [`crate::sphere_circle`].
//!
//! # The form along the circle
//!
//! In the chart's frame (origin at its origin, apex or centre, `z`
//! along the axis) the carrier is `C(t) = c + a·cos t + b·sin t`, so each
//! coordinate is a trigonometric polynomial of degree 1 in `t`, and the
//! chart's canonical form `f` (a polynomial in the coordinates) composed
//! along it is a trigonometric polynomial
//! `k₀ + Σₙ (pₙ·cos nt + qₙ·sin nt)`, exactly, built here by the
//! product-to-sum identities:
//!
//! | chart | `f` | degree in `t` |
//! |---|---|---|
//! | cylinder | `x² + y² − R²` | 2 |
//! | cone | `(x² + y²)·cos²α − z²·sin²α` | 2 |
//! | torus | `(x² + y² + z² + R² − r²)² − 4R²·(x² + y²)` | 4 |
//!
//! Each harmonic pair is read at its exact sup `√(pₙ² + qₙ²)`
//! ([`crate::harmonic`]), so `K = |k₀| + Σₙ √(pₙ² + qₙ²)` bounds `|f|`
//! at every `t` ([`form_sup`]). The projected row converts it to metres
//! by the same conversion its net path reads (`projected::Meters`).
//! Every operation is a `Real` one, so at `Interval` each coefficient's
//! enclosure holds the exact coefficient and `K`'s upper end bounds
//! `|f|` for every circle the inputs' enclosures stand for.

use geom_core::{Real, Vec3};

use crate::harmonic::harmonic_sup_scalar;

/// A trigonometric polynomial `k₀ + Σₙ (pₙ·cos nt + qₙ·sin nt)`, as
/// `[(k₀, 0), (p₁, q₁), …]`.
#[derive(Clone, Debug)]
struct Trig<T>(Vec<(T, T)>);

impl<T: Real> Trig<T> {
    /// `c + a·cos t + b·sin t`.
    fn linear(c: T, a: T, b: T) -> Self {
        Self(vec![(c, T::zero()), (a, b)])
    }

    fn constant(c: T) -> Self {
        Self(vec![(c, T::zero())])
    }

    fn add(&self, other: &Self) -> Self {
        let n = self.0.len().max(other.0.len());
        let at = |p: &Self, i: usize| p.0.get(i).copied().unwrap_or((T::zero(), T::zero()));
        Self(
            (0..n)
                .map(|i| {
                    let ((p, q), (r, s)) = (at(self, i), at(other, i));
                    (p + r, q + s)
                })
                .collect(),
        )
    }

    fn scale(&self, k: T) -> Self {
        Self(self.0.iter().map(|&(p, q)| (p * k, q * k)).collect())
    }

    /// The product, by `cos j·cos k = (cos(j−k) + cos(j+k))/2`,
    /// `sin j·sin k = (cos(j−k) − cos(j+k))/2`,
    /// `cos j·sin k = (sin(j+k) − sin(j−k))/2` and
    /// `sin j·cos k = (sin(j+k) + sin(j−k))/2`; the constant term is
    /// `(k₀, 0)`, so the same identities read it.
    fn mul(&self, other: &Self) -> Self {
        let half = T::from_f64(0.5);
        let mut out = vec![(T::zero(), T::zero()); self.0.len() + other.0.len() - 1];
        for (j, &(pj, qj)) in self.0.iter().enumerate() {
            for (k, &(pk, qk)) in other.0.iter().enumerate() {
                let (cc, ss, cs, sc) = (pj * pk, qj * qk, pj * qk, qj * pk);
                let sum = j + k;
                out[sum].0 = out[sum].0 + (cc - ss) * half;
                out[sum].1 = out[sum].1 + (cs + sc) * half;
                // `sin(j − k)`, signed: `sin(−m) = −sin m`; at `j = k`
                // it vanishes.
                let (diff, sign) = if j >= k {
                    (j - k, T::one())
                } else {
                    (k - j, T::zero() - T::one())
                };
                out[diff].0 = out[diff].0 + (cc + ss) * half;
                if diff > 0 {
                    out[diff].1 = out[diff].1 + (sc - cs) * half * sign;
                }
            }
        }
        Self(out)
    }

    /// `|k₀| + Σₙ √(pₙ² + qₙ²)`: a bound on `|·|` at every `t`.
    fn sup(&self) -> T {
        self.0
            .iter()
            .enumerate()
            .fold(T::zero(), |sum, (n, &(p, q))| {
                sum + if n == 0 {
                    p.abs()
                } else {
                    harmonic_sup_scalar(p, q)
                }
            })
    }
}

/// The analytic chart a circle's form is read against (module docs).
#[derive(Clone, Copy, Debug)]
pub(crate) enum ChartForm<T> {
    Cylinder { radius: T },
    Cone { sin: T, cos: T },
    Torus { major: T, minor: T },
}

/// A bound on `|f(C(t))|` over every `t`, for the chart-frame circle
/// `c + a·cos t + b·sin t` (module docs).
pub(crate) fn form_sup<T: Real>(chart: ChartForm<T>, c: Vec3<T>, a: Vec3<T>, b: Vec3<T>) -> T {
    let (x, y, z) = (
        Trig::linear(c.x, a.x, b.x),
        Trig::linear(c.y, a.y, b.y),
        Trig::linear(c.z, a.z, b.z),
    );
    let rho_sq = x.mul(&x).add(&y.mul(&y));
    let form = match chart {
        ChartForm::Cylinder { radius } => rho_sq.add(&Trig::constant(T::zero() - radius.powi(2))),
        ChartForm::Cone { sin, cos } => rho_sq
            .scale(cos.powi(2))
            .add(&z.mul(&z).scale(T::zero() - sin.powi(2))),
        ChartForm::Torus { major, minor } => {
            let g = rho_sq
                .add(&z.mul(&z))
                .add(&Trig::constant(major.powi(2) - minor.powi(2)));
            g.mul(&g)
                .add(&rho_sq.scale(T::zero() - T::from_f64(4.0) * major.powi(2)))
        }
    };
    form.sup()
}

/// A lower bound on `ρ = √(x² + y²)` over every `t` of the chart-frame
/// circle `c + a·cos t + b·sin t`: `ρ²` is a trigonometric polynomial
/// of degree 2, at least `k₀ − Σₙ √(pₙ² + qₙ²)`, floored at `0`.
pub(crate) fn rho_floor<T: Real>(c: Vec3<T>, a: Vec3<T>, b: Vec3<T>) -> T {
    let (x, y) = (Trig::linear(c.x, a.x, b.x), Trig::linear(c.y, a.y, b.y));
    let rho_sq = x.mul(&x).add(&y.mul(&y));
    let (k0, _) = rho_sq.0[0];
    let harmonics = Trig(
        core::iter::once((T::zero(), T::zero()))
            .chain(rho_sq.0[1..].iter().copied())
            .collect(),
    );
    (k0 - harmonics.sup()).max(T::zero()).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The product identities against direct evaluation, on random
    /// polynomials of degree up to 2: the expansion is exact, so the two
    /// agree to rounding.
    #[test]
    fn the_product_is_the_pointwise_product() {
        let eval = |p: &Trig<f64>, t: f64| {
            p.0.iter().enumerate().fold(0.0, |s, (n, &(c, d))| {
                let nt = n as f64 * t;
                s + c * nt.cos() + d * nt.sin()
            })
        };
        let mut seed = 0x9e37_79b9_7f4a_7c15_u64;
        let mut next = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            (seed >> 11) as f64 / (1u64 << 53) as f64 * 2.0 - 1.0
        };
        for _ in 0..64 {
            let p = Trig(
                (0..3)
                    .map(|n| (next(), if n == 0 { 0.0 } else { next() }))
                    .collect(),
            );
            let q = Trig(
                (0..2)
                    .map(|n| (next(), if n == 0 { 0.0 } else { next() }))
                    .collect(),
            );
            let pq = p.mul(&q);
            for i in 0..32 {
                let t = f64::from(i) * 0.2 - 3.1;
                let (want, got) = (eval(&p, t) * eval(&q, t), eval(&pq, t));
                assert!((want - got).abs() < 1e-13, "t {t}: {want} vs {got}");
            }
        }
    }

    /// The bound covers the form's sampled magnitude on each chart, and
    /// is exact to rounding on a circle of the chart (a torus parallel,
    /// a cone rim, a cylinder section).
    #[test]
    fn the_bound_covers_the_sampled_form() {
        type Row = (ChartForm<f64>, Vec3<f64>, Vec3<f64>, Vec3<f64>, bool);
        let rows: [Row; 6] = [
            (
                ChartForm::Cylinder { radius: 1.5 },
                Vec3::new(0.0, 0.0, 2.0),
                Vec3::new(1.5, 0.0, 0.0),
                Vec3::new(0.0, 1.5, 0.0),
                true,
            ),
            (
                ChartForm::Cylinder { radius: 1.5 },
                Vec3::new(0.1, -0.2, 2.0),
                Vec3::new(1.3, 0.2, 0.4),
                Vec3::new(-0.1, 1.4, 0.3),
                false,
            ),
            (
                ChartForm::Cone {
                    sin: 0.3_f64.sin(),
                    cos: 0.3_f64.cos(),
                },
                Vec3::new(0.0, 0.0, 2.0),
                Vec3::new(2.0 * 0.3_f64.tan(), 0.0, 0.0),
                Vec3::new(0.0, 2.0 * 0.3_f64.tan(), 0.0),
                true,
            ),
            (
                ChartForm::Cone {
                    sin: 0.3_f64.sin(),
                    cos: 0.3_f64.cos(),
                },
                Vec3::new(0.2, 0.0, 2.0),
                Vec3::new(0.5, 0.1, 0.2),
                Vec3::new(0.0, 0.6, -0.1),
                false,
            ),
            (
                ChartForm::Torus {
                    major: 2.0,
                    minor: 0.5,
                },
                Vec3::new(0.0, 0.0, 0.3),
                Vec3::new(2.4, 0.0, 0.0),
                Vec3::new(0.0, 2.4, 0.0),
                true,
            ),
            (
                ChartForm::Torus {
                    major: 2.0,
                    minor: 0.5,
                },
                Vec3::new(0.1, 0.0, 0.5),
                Vec3::new(2.0, 0.0, 0.1),
                Vec3::new(0.0, 2.0, -0.05),
                false,
            ),
        ];
        for (chart, c, a, b, on) in rows {
            let form = |p: Vec3<f64>| match chart {
                ChartForm::Cylinder { radius } => p.x * p.x + p.y * p.y - radius * radius,
                ChartForm::Cone { sin, cos } => {
                    (p.x * p.x + p.y * p.y) * cos * cos - p.z * p.z * sin * sin
                }
                ChartForm::Torus { major, minor } => {
                    let g = p.norm_squared() + major * major - minor * minor;
                    g * g - 4.0 * major * major * (p.x * p.x + p.y * p.y)
                }
            };
            let sampled = (0..4096)
                .map(|i| {
                    let t = f64::from(i) * core::f64::consts::TAU / 4096.0;
                    form(c + a * t.cos() + b * t.sin()).abs()
                })
                .fold(0.0, f64::max);
            let bound = form_sup(chart, c, a, b);
            assert!(
                bound >= sampled - 1e-12,
                "{chart:?}: {bound:e} under {sampled:e}"
            );
            let rho = (0..4096)
                .map(|i| {
                    let t = f64::from(i) * core::f64::consts::TAU / 4096.0;
                    let p = c + a * t.cos() + b * t.sin();
                    p.x.hypot(p.y)
                })
                .fold(f64::INFINITY, f64::min);
            assert!(
                rho_floor(c, a, b) <= rho + 1e-14,
                "{chart:?}: the radial floor is under the sampled least radius {rho:e}"
            );
            if on {
                assert!(bound < 1e-13, "{chart:?}: a circle of the chart, {bound:e}");
            }
        }
    }
}
