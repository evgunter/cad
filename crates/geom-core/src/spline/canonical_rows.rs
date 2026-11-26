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
    /// The products are Bernstein products
    /// ([`super::derivative::row_product`], the generic scalar's one
    /// home), computed in `S`'s own arithmetic: reals at `f64`,
    /// enclosures at an interval scalar. `None` where a product passes
    /// the exact binomials' degree ([`super::derivative::QUOTIENT_MAX_DEGREE`]).
    ///
    /// [`super::compose::canonical_composite`] is the same form over a
    /// curve's every span in certification arithmetic, and is not built
    /// on these rows: its products are `compose`'s own Bernstein product
    /// (`bern_mul_row`, a memoized outward quotient weight per term, the
    /// tighter association for certification rows, with its bit-identity
    /// rows pinned), and its squares `compose`'s symmetric pass. The two
    /// homes are by precision
    /// (`work/nurbs/binomial-bernstein-product-and-blossom-have-two-homes-each.md`).
    #[must_use]
    pub fn piece_rows(&self, g: [&[S]; 3], w: &[S]) -> Option<(Vec<S>, Vec<S>)> {
        let mul = |a: &[S], b: &[S]| super::derivative::row_product(a, b);
        let add = |a: &[S], b: &[S]| -> Vec<S> { a.iter().zip(b).map(|(x, y)| *x + *y).collect() };
        let sub = |a: &[S], b: &[S]| -> Vec<S> { a.iter().zip(b).map(|(x, y)| *x - *y).collect() };
        let scale = |c: S, a: &[S]| -> Vec<S> { a.iter().map(|x| c * *x).collect() };
        let radial = || Some(add(&mul(g[0], g[0])?, &mul(g[1], g[1])?));
        let squares = || Some(add(&radial()?, &mul(g[2], g[2])?));
        let w2 = || mul(w, w);
        Some(match *self {
            Self::Plane => (g[2].to_vec(), w.to_vec()),
            Self::Cylinder { radius } => {
                let w2 = w2()?;
                (sub(&radial()?, &scale(radius.powi(2), &w2)), w2)
            }
            Self::Sphere { radius } => {
                let w2 = w2()?;
                (sub(&squares()?, &scale(radius.powi(2), &w2)), w2)
            }
            Self::Cone { cos2, sin2 } => (
                sub(&scale(cos2, &radial()?), &scale(sin2, &mul(g[2], g[2])?)),
                w2()?,
            ),
            Self::Torus { major, minor } => {
                let w2 = w2()?;
                let shifted = add(&squares()?, &scale(major.powi(2) - minor.powi(2), &w2));
                let four_r2 = S::from_f64(4.0) * major.powi(2);
                (
                    sub(
                        &mul(&shifted, &shifted)?,
                        &scale(four_r2, &mul(&radial()?, &w2)?),
                    ),
                    mul(&w2, &w2)?,
                )
            }
        })
    }
}
