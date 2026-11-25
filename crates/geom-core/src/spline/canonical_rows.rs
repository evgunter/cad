//! **The canonical forms' rows at any scalar**: a surface's canonical
//! form ([`CanonicalSurface`]) composed along one Bézier piece of a
//! carrier written in its chart frame, as numerator and denominator
//! Bernstein rows computed in the scalar's own arithmetic (reals at
//! `f64`, enclosures at an interval scalar). The curve × face table's
//! ring-composite arm reads them (`geom_brep::curve_face::composite`).
//! The forms are [`CanonicalSurface`]'s; [`super::compose`] composes
//! the same forms over a curve's every span in certification arithmetic.

use super::compose::CanonicalSurface;
use crate::real::Real;

impl<S: Real> CanonicalSurface<S> {
    /// **The form's numerator and denominator rows on one Bézier piece**,
    /// at the scalar `S`: `g` the piece's weighted coordinate rows
    /// `w·x`, `w·y`, `w·z` in the chart frame and `w` its weight row, all
    /// of one degree. The numerator is the form's polynomial in the
    /// weighted coordinates, homogenized by powers of `w`, and the
    /// denominator that power of `w` (`w` on the plane, `w²` on the
    /// quadrics, `w⁴` on the torus), so the composite is their quotient.
    /// The products are Bernstein products ([`bernstein_product`]),
    /// computed in `S`'s own arithmetic: reals at `f64`, enclosures at an
    /// interval scalar. [`super::compose::canonical_composite`] is the same form over a
    /// curve's every span in certification arithmetic, in its own
    /// association.
    #[must_use]
    pub fn piece_rows(&self, g: [&[S]; 3], w: &[S]) -> (Vec<S>, Vec<S>) {
        let mul = bernstein_product::<S>;
        let add = |a: &[S], b: &[S]| -> Vec<S> { a.iter().zip(b).map(|(x, y)| *x + *y).collect() };
        let sub = |a: &[S], b: &[S]| -> Vec<S> { a.iter().zip(b).map(|(x, y)| *x - *y).collect() };
        let scale = |c: S, a: &[S]| -> Vec<S> { a.iter().map(|x| c * *x).collect() };
        let radial = || add(&mul(g[0], g[0]), &mul(g[1], g[1]));
        let squares = || add(&radial(), &mul(g[2], g[2]));
        let w2 = || mul(w, w);
        match *self {
            Self::Plane => (g[2].to_vec(), w.to_vec()),
            Self::Cylinder { radius } => {
                let w2 = w2();
                (sub(&radial(), &scale(radius.powi(2), &w2)), w2)
            }
            Self::Sphere { radius } => {
                let w2 = w2();
                (sub(&squares(), &scale(radius.powi(2), &w2)), w2)
            }
            Self::Cone { cos2, sin2 } => (
                sub(&scale(cos2, &radial()), &scale(sin2, &mul(g[2], g[2]))),
                w2(),
            ),
            Self::Torus { major, minor } => {
                let w2 = w2();
                let shifted = add(&squares(), &scale(major.powi(2) - minor.powi(2), &w2));
                let four_r2 = S::from_f64(4.0) * major.powi(2);
                (
                    sub(
                        &mul(&shifted, &shifted),
                        &scale(four_r2, &mul(&radial(), &w2)),
                    ),
                    mul(&w2, &w2),
                )
            }
        }
    }
}

/// **The Bernstein product of two rows** at any scalar, of degrees `p`
/// and `q` (from their lengths), as a row of degree `p + q`:
/// `(xy)_k = Σ_{i+j=k} C(p,i)·C(q,j)/C(p+q,k) · x_i·y_j`, each weight an
/// exact quotient of binomials ([`super::pieces::binomial`]) entered as
/// a constant. Empty where either row is.
#[must_use]
pub fn bernstein_product<S: Real>(x: &[S], y: &[S]) -> Vec<S> {
    let (Some(p), Some(q)) = (x.len().checked_sub(1), y.len().checked_sub(1)) else {
        return Vec::new();
    };
    let binomial = super::pieces::binomial;
    (0..=p + q)
        .map(|k| {
            (k.saturating_sub(q)..=k.min(p))
                .map(|i| {
                    let w = S::from_f64(binomial(p, i) * binomial(q, k - i))
                        / S::from_f64(binomial(p + q, k));
                    w * x[i] * y[k - i]
                })
                .fold(S::zero(), |s, t| s + t)
        })
        .collect()
}
