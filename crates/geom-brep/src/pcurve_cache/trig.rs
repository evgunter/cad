//! **A closed-form image's miss from a closed-form carrier, as one
//! trigonometric polynomial**: the algebra [`super::closed_bound`] writes
//! `C(t) − S(P(t))` in.
//!
//! A [`Trig`] is `Σₖ Σⱼ sʲ·(cₖⱼ·cos ks + dₖⱼ·sin ks)` in the centred
//! parameter `s = t − t_m`, with whole frequencies `k ≥ 0` and powers
//! `j ≥ 0`. The family is closed under sums and products (the
//! product-to-sum identities), and every analytic chart evaluated on a
//! closed-form image is a polynomial in the image's channels and in the
//! cosine and sine of its angle channels, so the chart's image of a
//! closed form lands in it exactly, coefficient by coefficient.
//!
//! The canonical form keeps one slot per `(k, j)` and no `sin 0·s` slot,
//! and the functions `sʲ·cos ks`, `sʲ·sin ks` (`k > 0`) and `sʲ` are
//! linearly independent. So a residual that vanishes identically has
//! every coefficient zero, whatever spelling its image chose: a phase a
//! whole period away, a mirrored winding read against a mirrored frame,
//! or a sphere point's involution twin all expand to the same function,
//! and their difference to rounding.

use geom_core::{Real, Vec3};

use crate::harmonic::harmonic_sup;

/// The most whole turns per carrier period an angle is read at. A line or
/// a conic's closed-form image turns at most once per period; a stated
/// winding past this is no closed form's, and its image's structure is
/// read wrong.
pub(super) const MAX_WINDING: u32 = 64;

/// One scalar of the family over `s`, dense in `(k, j)`.
#[derive(Clone, Debug)]
pub(super) struct Trig<T> {
    /// One past the highest power of `s`.
    powers: usize,
    /// `(cₖⱼ, dₖⱼ)` at `k·powers + j`; `d₀ⱼ` is zero by construction.
    terms: Vec<(T, T)>,
}

impl<T: Real> Trig<T> {
    fn zero(frequencies: usize, powers: usize) -> Self {
        Self {
            powers,
            terms: vec![(T::zero(), T::zero()); frequencies * powers],
        }
    }

    fn frequencies(&self) -> usize {
        self.terms.len() / self.powers
    }

    /// Adds `c·sʲ·cos(k s) + d·sʲ·sin(k s)`, for a signed `k`, folding a
    /// negative frequency onto its positive twin (`sin` odd, `cos` even)
    /// and dropping `sin 0`.
    fn add_at(&mut self, k: isize, j: usize, (c, d): (T, T)) {
        let (k, d) = if k < 0 {
            (k.unsigned_abs(), -d)
        } else {
            (k.unsigned_abs(), d)
        };
        let slot = &mut self.terms[k * self.powers + j];
        slot.0 = slot.0 + c;
        if k > 0 {
            slot.1 = slot.1 + d;
        }
    }

    /// The constant `x`.
    pub(super) fn constant(x: T) -> Self {
        let mut out = Self::zero(1, 1);
        out.terms[0].0 = x;
        out
    }

    /// The free channel `c + a·cos t + b·sin t + l·t` at `t = t_m + s`:
    /// `cos(t_m + s) = cos t_m·cos s − sin t_m·sin s`, so the cosine pair
    /// turns by `t_m` and the line's constant takes `l·t_m`.
    pub(super) fn free([c, a, b, l]: [T; 4], t_m: T) -> Self {
        let (sin_m, cos_m) = t_m.sin_cos();
        let mut out = Self::zero(2, 2);
        out.add_at(0, 0, (c + l * t_m, T::zero()));
        out.add_at(0, 1, (l, T::zero()));
        out.add_at(1, 0, (a * cos_m + b * sin_m, b * cos_m - a * sin_m));
        out
    }

    /// `(cos θ, sin θ)` of the angle `θ = phase + w·t` at `t = t_m + s`:
    /// `θ = φ + w·s` with `φ = phase + w·t_m`, read as an angle, so a phase
    /// a whole period away is the same pair to rounding. `None` for a
    /// winding past [`MAX_WINDING`], whose frequency the family does not
    /// hold.
    pub(super) fn angle(phase: T, winding: i32, t_m: T) -> Option<(Self, Self)> {
        if winding.unsigned_abs() > MAX_WINDING {
            return None;
        }
        let w: T = super::angle_rate(winding);
        let (sin_p, cos_p) = (phase + w * t_m).sin_cos();
        let k = isize::try_from(winding).ok()?;
        let n = k.unsigned_abs() + 1;
        let (mut cos, mut sin) = (Self::zero(n, 1), Self::zero(n, 1));
        // cos(φ + ks) = cos φ·cos ks − sin φ·sin ks;
        // sin(φ + ks) = sin φ·cos ks + cos φ·sin ks.
        cos.add_at(k, 0, (cos_p, -sin_p));
        sin.add_at(k, 0, (sin_p, cos_p));
        Some((cos, sin))
    }

    /// `self + other`.
    #[must_use]
    pub(super) fn plus(&self, other: &Self) -> Self {
        let mut out = Self::zero(
            self.frequencies().max(other.frequencies()),
            self.powers.max(other.powers),
        );
        for x in [self, other] {
            for (i, &pair) in x.terms.iter().enumerate() {
                let (k, j) = (i / x.powers, i % x.powers);
                out.add_at(k.cast_signed(), j, pair);
            }
        }
        out
    }

    /// `self·x`.
    #[must_use]
    pub(super) fn scaled(&self, x: T) -> Self {
        Self {
            powers: self.powers,
            terms: self.terms.iter().map(|&(c, d)| (c * x, d * x)).collect(),
        }
    }

    /// `self − other`.
    #[must_use]
    pub(super) fn minus(&self, other: &Self) -> Self {
        self.plus(&other.scaled(-T::one()))
    }

    /// `self·other`, by the product-to-sum identities:
    /// `cos a·cos b = ½(cos(a+b) + cos(a−b))`,
    /// `sin a·sin b = ½(cos(a−b) − cos(a+b))`,
    /// `cos a·sin b = ½(sin(a+b) − sin(a−b))`,
    /// `sin a·cos b = ½(sin(a+b) + sin(a−b))`.
    #[must_use]
    pub(super) fn times(&self, other: &Self) -> Self {
        let mut out = Self::zero(
            self.frequencies() + other.frequencies() - 1,
            self.powers + other.powers - 1,
        );
        let half = T::from_f64(0.5);
        for (i, &(c1, d1)) in self.terms.iter().enumerate() {
            let (k, j) = ((i / self.powers).cast_signed(), i % self.powers);
            for (n, &(c2, d2)) in other.terms.iter().enumerate() {
                let (m, l) = ((n / other.powers).cast_signed(), n % other.powers);
                let p = j + l;
                out.add_at(
                    k + m,
                    p,
                    ((c1 * c2 - d1 * d2) * half, (c1 * d2 + d1 * c2) * half),
                );
                out.add_at(
                    k - m,
                    p,
                    ((c1 * c2 + d1 * d2) * half, (d1 * c2 - c1 * d2) * half),
                );
            }
        }
        out
    }
}

/// A point or vector of the family: one [`Trig`] per coordinate.
#[derive(Clone, Debug)]
pub(super) struct Trig3<T>([Trig<T>; 3]);

impl<T: Real> Trig3<T> {
    /// The constant point or vector `p`.
    pub(super) fn constant(p: Vec3<T>) -> Self {
        Self([p.x, p.y, p.z].map(Trig::constant))
    }

    /// The scalar `x` along the vector `e`.
    pub(super) fn along(x: &Trig<T>, e: Vec3<T>) -> Self {
        Self([e.x, e.y, e.z].map(|c| x.scaled(c)))
    }

    /// The harmonic carrier `c + a·cos t + b·sin t + l·t` at `t = t_m + s`.
    pub(super) fn harmonic((c, a, b, l): (Vec3<T>, Vec3<T>, Vec3<T>, Vec3<T>), t_m: T) -> Self {
        Self(core::array::from_fn(|i| {
            let pick = |v: Vec3<T>| [v.x, v.y, v.z][i];
            Trig::free([pick(c), pick(a), pick(b), pick(l)], t_m)
        }))
    }

    /// One past the highest frequency of any coordinate.
    pub(super) fn frequencies(&self) -> usize {
        self.0.iter().map(Trig::frequencies).max().unwrap_or(0)
    }

    /// `self + other`.
    #[must_use]
    pub(super) fn plus(&self, other: &Self) -> Self {
        Self(core::array::from_fn(|i| self.0[i].plus(&other.0[i])))
    }

    /// `self − other`.
    #[must_use]
    pub(super) fn minus(&self, other: &Self) -> Self {
        Self(core::array::from_fn(|i| self.0[i].minus(&other.0[i])))
    }

    /// `x·self`, a scalar of the family times this vector.
    #[must_use]
    pub(super) fn times(&self, x: &Trig<T>) -> Self {
        Self(core::array::from_fn(|i| self.0[i].times(x)))
    }

    /// **An upper bound on `sup |self(s)|` over `|s| ≤ h`**: the triangle
    /// inequality over the slots, each `(k, j)` pair read at its exact sup
    /// `hʲ·sup_s |c·cos ks + d·sin ks|` ([`harmonic_sup`], the largest
    /// singular value of the `3 × 2` pair). A sum of upper bounds: it
    /// locates no point.
    pub(super) fn sup(&self, h: T) -> T {
        let frequencies = self.0.iter().map(Trig::frequencies).max().unwrap_or(0);
        let powers = self.0.iter().map(|x| x.powers).max().unwrap_or(0);
        let slot = |x: &Trig<T>, k: usize, j: usize| {
            if k < x.frequencies() && j < x.powers {
                x.terms[k * x.powers + j]
            } else {
                (T::zero(), T::zero())
            }
        };
        let mut total = T::zero();
        for k in 0..frequencies {
            let mut reach = T::one();
            for j in 0..powers {
                let [x, y, z] = [0, 1, 2].map(|i| slot(&self.0[i], k, j));
                let (c, d) = (Vec3::new(x.0, y.0, z.0), Vec3::new(x.1, y.1, z.1));
                total = total + reach * harmonic_sup(c, d);
                reach = reach * h;
            }
        }
        total
    }
}
