//! **The shape operator of a parametric surface**, read from its first
//! and second fundamental forms: the one home of the chart-side
//! principal curvatures.
//!
//! [`FundamentalForms`] holds the six coefficients and the metric
//! determinant, generic over the scalar. Two readers take it:
//! [`max_principal_curvature`] at a point jet (the SSI point decisions
//! and the at-rest plane × NURBS check lever `sin θ` by its reciprocal),
//! and `offset_meters::cell_curvature`, which encloses the principal
//! curvatures over a chart cell from the same mean and Gaussian
//! curvatures and the same Weingarten entries.
//!
//! The implicit side's counterpart is
//! [`crate::implicit::implicit_max_normal_curvature`]: the same extremum
//! `|H| + √(H² − K)`, read from an implicit form's Hessian restricted to
//! the tangent plane rather than from a chart. Neither is
//! [`crate::curvature_lever_arm`], which is a chart's own length scale
//! and not a curvature bound on a cone or a torus.

use geom::SurfaceJet;
use geom_core::Real;

/// The first and second fundamental forms of a chart at a point.
///
/// `l`, `m`, `n` are the second derivatives' components along the unit
/// normal; `a = EG − F²`, the metric determinant, is the squared area
/// element `‖S_u × S_v‖²`.
#[derive(Clone, Copy, Debug)]
pub(crate) struct FundamentalForms<T> {
    pub(crate) e: T,
    pub(crate) f: T,
    pub(crate) g: T,
    pub(crate) l: T,
    pub(crate) m: T,
    pub(crate) n: T,
    pub(crate) a: T,
}

impl<T: Real> FundamentalForms<T> {
    /// The forms of a chart jet. A singular chart (`S_u × S_v = 0`)
    /// gives poison.
    pub(crate) fn of_jet(j: &SurfaceJet<T>) -> Self {
        let normal = j.du.cross(j.dv);
        let a = normal.dot(normal);
        let area = a.sqrt();
        Self {
            e: j.du.dot(j.du),
            f: j.du.dot(j.dv),
            g: j.dv.dot(j.dv),
            l: normal.dot(j.duu) / area,
            m: normal.dot(j.duv) / area,
            n: normal.dot(j.dvv) / area,
            a,
        }
    }

    /// The mean curvature `H = (LG − 2MF + NE) / 2A`.
    pub(crate) fn mean(&self) -> T {
        let two = T::from_f64(2.0);
        (self.l * self.g - two * self.m * self.f + self.n * self.e) / (two * self.a)
    }

    /// The Gaussian curvature `K = (LN − M²) / A`.
    pub(crate) fn gauss(&self) -> T {
        (self.l * self.n - self.m.powi(2)) / self.a
    }

    /// The shape operator `W = I⁻¹·II`, row-major, whose eigenvalues are
    /// the principal curvatures.
    pub(crate) fn weingarten(&self) -> [[T; 2]; 2] {
        let (e, f, g, l, m, n, a) = (self.e, self.f, self.g, self.l, self.m, self.n, self.a);
        [
            [(g * l - f * m) / a, (g * m - f * n) / a],
            [(e * m - f * l) / a, (e * n - f * m) / a],
        ]
    }

    /// The larger principal curvature magnitude, `|H| + √(H² − K)`.
    /// `H² − K` is nonnegative in ℝ; the clamp takes its rounding, which
    /// can carry it below zero at an umbilic.
    pub(crate) fn max_principal(&self) -> T {
        let h = self.mean();
        h.abs() + (h.powi(2) - self.gauss()).max(T::zero()).sqrt()
    }
}

/// **The largest principal curvature of a surface at a chart jet**, in
/// reciprocal metres: a property of the surface, so any regular
/// reparameterisation of it reads the same number in ℝ, where a
/// parameter line's own acceleration does not. Zero where the surface
/// is flat at the jet; poison where the jet is, or where the chart is
/// singular. Near a singular chart the rounding of `S_u × S_v` makes it
/// read high, which errs toward refusing.
pub(crate) fn max_principal_curvature<T: Real>(j: &SurfaceJet<T>) -> T {
    FundamentalForms::of_jet(j).max_principal()
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::{FundamentalForms, max_principal_curvature};
    use geom::NurbsSurface;
    use geom_core::Point3;
    use geom_core::spline::KnotVector;

    fn kv(degree: usize) -> KnotVector {
        let mut k = vec![0.0; degree + 1];
        k.extend(vec![1.0; degree + 1]);
        KnotVector::clamped(k, degree).unwrap()
    }

    /// **The saddle `z = xy` reads its analytic curvature everywhere.**
    /// The bilinear chart over `[−1, 1]²` has `F ≠ 0` and `M ≠ 0` off
    /// the axes, and at the origin `H = 0`, `K = −1`: every term of
    /// `|H| + √(H² − K)` carries weight somewhere on the grid. The truth
    /// is `H = −xy/q^{3/2}`, `K = −1/q²`, `q = 1 + x² + y²`.
    #[test]
    fn a_saddle_reads_its_analytic_principal_curvature() {
        let control = vec![
            Point3::new(-1.0, -1.0, 1.0),
            Point3::new(-1.0, 1.0, -1.0),
            Point3::new(1.0, -1.0, -1.0),
            Point3::new(1.0, 1.0, 1.0),
        ];
        let saddle = NurbsSurface::new(kv(1), kv(1), control, vec![1.0; 4]).unwrap();
        let mut skew = false;
        for i in 0..=10 {
            for j in 0..=10 {
                let (u, v) = (f64::from(i) / 10.0, f64::from(j) / 10.0);
                let (x, y) = (2.0 * u - 1.0, 2.0 * v - 1.0);
                let q = 1.0 + x * x + y * y;
                let (h, k) = (-x * y / q.powf(1.5), -1.0 / (q * q));
                let truth = h.abs() + (h * h - k).sqrt();
                let jet = saddle.ders(u, v);
                let forms = FundamentalForms::of_jet(&jet);
                skew |= forms.f.abs() > 0.1 && forms.m.abs() > 0.1;
                let got = max_principal_curvature(&jet);
                assert!(
                    (got / truth - 1.0).abs() <= 1e-12,
                    "the saddle at ({x}, {y}): {got:e}, truth {truth:e}"
                );
            }
        }
        assert!(skew, "FIXTURE: the grid reaches F ≠ 0 and M ≠ 0 together");
    }

    /// A band of the sphere of radius `r` between two latitudes, a
    /// rational biquadratic patch: the quarter turn in longitude along
    /// `u`, its row weights rescaled by `c^i` (a Möbius
    /// reparameterisation of the same patch), the arc in latitude along
    /// `v`.
    fn sphere_band(r: f64, lat: (f64, f64), c: f64) -> NurbsSurface<f64> {
        let half = 0.5 * (lat.1 - lat.0);
        let wm = half.cos();
        let a = (r * lat.0.cos(), r * lat.0.sin());
        let b = (r * lat.1.cos(), r * lat.1.sin());
        let mid = (a.0 + b.0, a.1 + b.1);
        let len = (mid.0 * mid.0 + mid.1 * mid.1).sqrt();
        let m = (mid.0 / len * r / wm, mid.1 / len * r / wm);
        let meridian = [(a.0, a.1, 1.0), (m.0, m.1, wm), (b.0, b.1, 1.0)];
        let mut control = Vec::new();
        let mut weights = Vec::new();
        for (i, scale) in [1.0, c, c * c].into_iter().enumerate() {
            for (x, z, w) in meridian {
                control.push(match i {
                    0 => Point3::new(x, 0.0, z),
                    1 => Point3::new(x, x, z),
                    _ => Point3::new(0.0, x, z),
                });
                let row = if i == 1 {
                    core::f64::consts::FRAC_1_SQRT_2
                } else {
                    1.0
                };
                weights.push(w * row * scale);
            }
        }
        NurbsSurface::new(kv(2), kv(2), control, weights).unwrap()
    }

    /// **A sphere reads `1/r` under two charts, though `H² − K` rounds
    /// below zero.** Every point of a sphere is an umbilic, so `H² − K`
    /// is zero in ℝ and its rounding falls on either side; the clamp
    /// keeps the root real where it falls below. The patch and its
    /// Möbius twin read `1/r` to the root of the rounding.
    #[test]
    fn a_sphere_reads_its_radius_where_the_discriminant_rounds_negative() {
        let r = 0.3;
        let mut below = 0;
        for c in [1.0, 3.0] {
            let band = sphere_band(r, (0.1, 1.0), c);
            for i in 0..=20 {
                for j in 0..=20 {
                    let jet = band.ders(f64::from(i) / 20.0, f64::from(j) / 20.0);
                    let forms = FundamentalForms::of_jet(&jet);
                    if forms.mean().powi(2) - forms.gauss() < 0.0 {
                        below += 1;
                    }
                    let kappa = max_principal_curvature(&jet);
                    assert!(
                        (kappa * r - 1.0).abs() <= 1e-6,
                        "chart c = {c} at ({i}, {j})/20: κ·r = {kappa:e}"
                    );
                }
            }
        }
        assert!(below > 0, "FIXTURE: some H² − K rounds below zero");
    }
}
