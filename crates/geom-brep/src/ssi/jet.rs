//! Truncated third-order Taylor arithmetic along a polynomial path —
//! the marcher's exact source of the Hoffmann Eq. 6.1 right-hand sides
//! (M5 PR 7, C3).
//!
//! # What this computes and why it is shaped this way
//!
//! Hoffmann's third-order approximant needs, at each step, the
//! quantities `b_{f,m}` in `∇f·r⁽ᵐ⁾ = b_{f,m}` — that is, *everything
//! in `dᵐ/dsᵐ f(r(s))` except the term carrying `r⁽ᵐ⁾` itself*. Those
//! are directional-derivative tensors of `f` (second and third), which
//! the kernel has no general machinery for: [`geom_core::Dual`] is
//! forward-mode **first** order and does not nest (its `KinkJacobian`
//! helper is not implemented for `Dual` itself), and a full
//! `Real`-implementing jet type would be a large new scalar surface
//! that §7 of the PR spec puts out of scope.
//!
//! The observation that makes this small: let
//!
//! ```text
//! r̃(s) = p + s·d₁ + (s²/2)·d₂          (the approximant with r‴ dropped)
//! h(s) = f(r̃(s))
//! ```
//!
//! Then `h″(0) = D²f(d₁,d₁) + ∇f·d₂` and
//! `h‴(0) = D³f(d₁,d₁,d₁) + 3·D²f(d₁,d₂)` — *exactly* the two
//! right-hand sides, with **no** tensor ever formed:
//!
//! - `b₂ = −h″(0)` evaluated with `d₂ = 0`;
//! - `b₃ = −h‴(0)` evaluated with the solved `d₂`.
//!
//! So the whole requirement is: evaluate `f` at an argument that is a
//! **degree-3 truncated polynomial in `s`**. The analytic implicit
//! forms of `geom_brep::implicit` are built from `+`, `−`, `×`, `÷`,
//! `sqrt` and one sign-stable `abs`, so a four-coefficient [`Poly3`]
//! with those five operations is all the arithmetic needed — about
//! eighty lines instead of a scalar.
//!
//! # Discipline
//!
//! `f64`-only, like the rest of the marcher (C6's selection lane): the
//! stepper is an untrusted candidate generator and the C2 certificate,
//! which runs at every `T`, is the only gate. Fixed operation order
//! throughout (D9); `sqrt` is the single libm call. Poison in, poison
//! out — a non-finite coefficient propagates to a NaN right-hand side,
//! which poisons the SVD solve, which fails the step's band. Nothing
//! here decides anything.

use geom::Surface;
use geom_core::{Point3, Vec3};

/// A Taylor polynomial in `s`, truncated after `s³`, in the **monomial**
/// basis: `c[0] + c[1]·s + c[2]·s² + c[3]·s³`.
///
/// Monomial rather than Taylor-coefficient form so the products are the
/// obvious convolutions; the derivative readings
/// ([`Poly3::d2`], [`Poly3::d3`]) carry the factorials.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Poly3 {
    c: [f64; 4],
}

impl Poly3 {
    /// The constant series `x`.
    pub(crate) fn constant(x: f64) -> Self {
        Self {
            c: [x, 0.0, 0.0, 0.0],
        }
    }

    /// The series `c0 + c1·s + c2·s² + c3·s³`.
    pub(crate) fn new(c0: f64, c1: f64, c2: f64, c3: f64) -> Self {
        Self {
            c: [c0, c1, c2, c3],
        }
    }

    /// The value at `s = 0`. Read by the tests, which pin it against
    /// `implicit_residual` — the production path only ever wants the
    /// higher coefficients, since the value it already has.
    #[cfg(test)]
    pub(crate) fn value(self) -> f64 {
        self.c[0]
    }

    /// `h′(0)` = the `s` coefficient. Read by the tests (the
    /// production path gets the first-order data from the Jacobian,
    /// which is where the SVD needs it anyway).
    #[cfg(test)]
    pub(crate) fn d1(self) -> f64 {
        self.c[1]
    }

    /// `h″(0)` = `2·` the `s²` coefficient.
    pub(crate) fn d2(self) -> f64 {
        2.0 * self.c[2]
    }

    /// `h‴(0)` = `6·` the `s³` coefficient.
    pub(crate) fn d3(self) -> f64 {
        6.0 * self.c[3]
    }

    fn add(self, o: Self) -> Self {
        Self {
            c: [
                self.c[0] + o.c[0],
                self.c[1] + o.c[1],
                self.c[2] + o.c[2],
                self.c[3] + o.c[3],
            ],
        }
    }

    fn sub(self, o: Self) -> Self {
        Self {
            c: [
                self.c[0] - o.c[0],
                self.c[1] - o.c[1],
                self.c[2] - o.c[2],
                self.c[3] - o.c[3],
            ],
        }
    }

    /// Truncated product (ascending convolution, D9).
    fn mul(self, o: Self) -> Self {
        let a = self.c;
        let b = o.c;
        Self {
            c: [
                a[0] * b[0],
                a[0] * b[1] + a[1] * b[0],
                a[0] * b[2] + a[1] * b[1] + a[2] * b[0],
                a[0] * b[3] + a[1] * b[2] + a[2] * b[1] + a[3] * b[0],
            ],
        }
    }

    fn scale(self, k: f64) -> Self {
        Self {
            c: [self.c[0] * k, self.c[1] * k, self.c[2] * k, self.c[3] * k],
        }
    }

    /// Truncated `sqrt`, from `Q² = P` solved coefficient by
    /// coefficient in ascending order. **Poison for `c₀ ≤ 0`**: the
    /// square root of a series whose constant term is not strictly
    /// positive is not analytic at `s = 0`, and a marcher step that
    /// silently substituted something there would be exactly the kind
    /// of lie the untrusted-stepper discipline exists to prevent.
    fn sqrt(self) -> Self {
        let c = self.c;
        if c[0].is_nan() || c[0] <= 0.0 {
            return Self { c: [f64::NAN; 4] };
        }
        let q0 = c[0].sqrt();
        let q1 = c[1] / (2.0 * q0);
        let q2 = (c[2] - q1 * q1) / (2.0 * q0);
        let q3 = (c[3] - 2.0 * q1 * q2) / (2.0 * q0);
        Self {
            c: [q0, q1, q2, q3],
        }
    }

    /// `|P|` where the sign is taken from the **constant term** —
    /// legitimate only because the implicit forms that use `abs` (the
    /// cone's axial coordinate) are sign-stable along a short step.
    /// Poison at `c₀ = 0`, where `|·|` is not differentiable and the
    /// approximant would be a fiction.
    fn abs_stable(self) -> Self {
        if self.c[0] > 0.0 {
            self
        } else if self.c[0] < 0.0 {
            self.scale(-1.0)
        } else {
            Self { c: [f64::NAN; 4] }
        }
    }
}

/// A 3-vector whose components are [`Poly3`] series — the path and its
/// derived quantities.
#[derive(Clone, Copy, Debug)]
struct Vec3Poly {
    x: Poly3,
    y: Poly3,
    z: Poly3,
}

impl Vec3Poly {
    fn dot(self, o: Self) -> Poly3 {
        // Fixed ascending association (D9), matching `Vec3::dot`.
        self.x.mul(o.x).add(self.y.mul(o.y)).add(self.z.mul(o.z))
    }

    fn sub_const(self, p: Point3<f64>) -> Self {
        Self {
            x: self.x.sub(Poly3::constant(p.x)),
            y: self.y.sub(Poly3::constant(p.y)),
            z: self.z.sub(Poly3::constant(p.z)),
        }
    }

    fn scaled_const(v: Vec3<f64>, k: Poly3) -> Self {
        Self {
            x: k.scale(v.x),
            y: k.scale(v.y),
            z: k.scale(v.z),
        }
    }

    fn sub(self, o: Self) -> Self {
        Self {
            x: self.x.sub(o.x),
            y: self.y.sub(o.y),
            z: self.z.sub(o.z),
        }
    }
}

/// The path `r̃(s) = p + s·d₁ + (s²/2)·d₂` as a [`Vec3Poly`].
fn path(p: Point3<f64>, d1: Vec3<f64>, d2: Vec3<f64>) -> Vec3Poly {
    Vec3Poly {
        x: Poly3::new(p.x, d1.x, 0.5 * d2.x, 0.0),
        y: Poly3::new(p.y, d1.y, 0.5 * d2.y, 0.0),
        z: Poly3::new(p.z, d1.z, 0.5 * d2.z, 0.0),
    }
}

/// `h(s) = f(p + s·d₁ + (s²/2)·d₂)` for the **linearized implicit
/// residual** `f` of `surface` (`geom_brep::implicit`'s meters form),
/// as a truncated third-order series.
///
/// The five analytic kinds are written out against the same algebra as
/// [`crate::implicit::implicit_residual`], term for term and in the
/// same order, so the two agree on the constant coefficient (pinned by
/// test). [`Surface::Nurbs`] has no implicit form and yields poison —
/// the ℝ³ implicit-pair trace is never routed a NURBS operand (the
/// C5 table's rung-3 arms name their trace shape at compile time), so
/// this is the honest report of an unreachable request, not a fallback.
pub(crate) fn implicit_path_jet(
    surface: &Surface<f64>,
    p: Point3<f64>,
    d1: Vec3<f64>,
    d2: Vec3<f64>,
) -> Poly3 {
    let r = path(p, d1, d2);
    match *surface {
        Surface::Plane { origin, normal, .. } => {
            let q = r.sub_const(origin);
            q.dot(Vec3Poly {
                x: Poly3::constant(normal.x),
                y: Poly3::constant(normal.y),
                z: Poly3::constant(normal.z),
            })
        }
        Surface::Sphere { center, radius, .. } => {
            let q = r.sub_const(center);
            let d2q = q.dot(q);
            d2q.sub(Poly3::constant(radius * radius))
                .scale(1.0 / (2.0 * radius))
        }
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => {
            let (_, w) = axial_radial(r, origin, axis);
            let w2 = w.dot(w);
            w2.sub(Poly3::constant(radius * radius))
                .scale(1.0 / (2.0 * radius))
        }
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => {
            // `Real::sin_cos`, not the inherent `f64::sin_cos` route:
            // `implicit_residual` takes the former, and a second route
            // to the same constant is a latent value fork the moment
            // the two disagree in the last bit (they can — libm vs std).
            // The cone arm refuses at both the enclosure and the
            // certificate today, so nothing depends on it yet; that is
            // exactly when a fork is cheap to remove.
            let (s_a, c_a) = geom_core::Real::sin_cos(half_angle);
            let (h, w) = axial_radial(r, apex, axis);
            let rho = w.dot(w).sqrt();
            rho.scale(c_a).sub(h.abs_stable().scale(s_a))
        }
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => {
            let (h, w) = axial_radial(r, center, axis);
            let rho = w.dot(w).sqrt();
            let d = rho.sub(Poly3::constant(major_radius));
            d.mul(d)
                .add(h.mul(h))
                .sub(Poly3::constant(minor_radius * minor_radius))
                .scale(1.0 / (2.0 * minor_radius))
        }
        // No implicit form (module docs).
        // No implicit form for a spline stand-in, so no univariate
        // restriction of one — `Approx` yields the same poison.
        Surface::Nurbs(_) | Surface::Approx(_) => Poly3 { c: [f64::NAN; 4] },
    }
}

/// The axial/radial decomposition of a path against an anchor and unit
/// axis — [`crate::implicit`]'s `axial_radial`, lifted to series.
fn axial_radial(r: Vec3Poly, anchor: Point3<f64>, axis: Vec3<f64>) -> (Poly3, Vec3Poly) {
    let q = r.sub_const(anchor);
    let h = q.dot(Vec3Poly {
        x: Poly3::constant(axis.x),
        y: Poly3::constant(axis.y),
        z: Poly3::constant(axis.z),
    });
    let w = q.sub(Vec3Poly::scaled_const(axis, h));
    (h, w)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::implicit::implicit_residual;

    fn sphere() -> Surface<f64> {
        Surface::Sphere {
            center: Point3::new(0.3, -0.2, 0.5),
            radius: 1.25,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    fn cylinder() -> Surface<f64> {
        Surface::Cylinder {
            origin: Point3::new(-0.1, 0.4, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 0.8,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    fn torus() -> Surface<f64> {
        Surface::Torus {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            major_radius: 1.0,
            minor_radius: 0.3,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    fn cone() -> Surface<f64> {
        Surface::Cone {
            apex: Point3::new(0.0, 0.0, 2.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            half_angle: 0.4,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    /// The series' constant term must be the implicit residual itself:
    /// a fork between the two would make the marcher chase a locus the
    /// certificate does not check.
    #[test]
    fn constant_term_is_the_implicit_residual() {
        let p = Point3::new(0.9, 0.35, 0.6);
        let d1 = Vec3::new(0.4, -0.7, 0.2);
        let d2 = Vec3::new(-0.1, 0.25, 0.05);
        for s in [sphere(), cylinder(), torus(), cone()] {
            let j = implicit_path_jet(&s, p, d1, d2);
            let f = implicit_residual(&s, p);
            assert!(
                (j.value() - f).abs() <= 1e-15 * (1.0 + f.abs()),
                "{:?}: series {} vs residual {f}",
                crate::SurfaceKind::of(&s),
                j.value()
            );
        }
    }

    /// The whole series, against Richardson-extrapolated differences of
    /// `implicit_residual` along the same path — an independent
    /// reference sharing no code with the series algebra.
    #[test]
    fn series_matches_numerical_derivatives_along_the_path() {
        let p = Point3::new(0.9, 0.35, 0.6);
        let d1 = Vec3::new(0.4, -0.7, 0.2);
        let d2 = Vec3::new(-0.1, 0.25, 0.05);
        let h = 1.0e-3;
        for s in [sphere(), cylinder(), torus(), cone()] {
            let j = implicit_path_jet(&s, p, d1, d2);
            let at = |t: f64| {
                let q = p + d1 * t + d2 * (0.5 * t * t);
                implicit_residual(&s, q)
            };
            // Central differences, Richardson-extrapolated to O(h⁴).
            let d1n = {
                let a = (at(h) - at(-h)) / (2.0 * h);
                let b = (at(2.0 * h) - at(-2.0 * h)) / (4.0 * h);
                (4.0 * a - b) / 3.0
            };
            let d2n = {
                let a = (at(h) - 2.0 * at(0.0) + at(-h)) / (h * h);
                let b = (at(2.0 * h) - 2.0 * at(0.0) + at(-2.0 * h)) / (4.0 * h * h);
                (4.0 * a - b) / 3.0
            };
            let d3n = {
                let a =
                    (at(2.0 * h) - 2.0 * at(h) + 2.0 * at(-h) - at(-2.0 * h)) / (2.0 * h * h * h);
                let b = (at(4.0 * h) - 2.0 * at(2.0 * h) + 2.0 * at(-2.0 * h) - at(-4.0 * h))
                    / (16.0 * h * h * h);
                (4.0 * a - b) / 3.0
            };
            let kind = crate::SurfaceKind::of(&s);
            assert!(
                (j.d1() - d1n).abs() < 1e-8,
                "{kind:?} h′: {} vs {d1n}",
                j.d1()
            );
            assert!(
                (j.d2() - d2n).abs() < 1e-6,
                "{kind:?} h″: {} vs {d2n}",
                j.d2()
            );
            assert!(
                (j.d3() - d3n).abs() < 1e-4,
                "{kind:?} h‴: {} vs {d3n}",
                j.d3()
            );
        }
    }

    #[test]
    fn a_nurbs_operand_yields_poison_never_a_number() {
        let s = Surface::nurbs_placeholder();
        let j = implicit_path_jet(&s, Point3::new(0.0, 0.0, 0.0), Vec3::zero(), Vec3::zero());
        assert!(j.value().is_nan() && j.d3().is_nan());
    }

    #[test]
    fn sqrt_of_a_non_positive_constant_term_poisons() {
        let p = Poly3::new(0.0, 1.0, 0.0, 0.0).sqrt();
        assert!(p.value().is_nan());
        let n = Poly3::new(-1.0, 1.0, 0.0, 0.0).sqrt();
        assert!(n.value().is_nan());
    }

    #[test]
    fn poly3_products_and_sqrt_are_exact_on_a_known_series() {
        // (1 + s)² = 1 + 2s + s²
        let a = Poly3::new(1.0, 1.0, 0.0, 0.0);
        let sq = a.mul(a);
        assert_eq!(sq.c, [1.0, 2.0, 1.0, 0.0]);
        // sqrt(1 + 2s + s²) = 1 + s (truncated; the s³ coefficient of
        // the exact root is zero).
        let r = sq.sqrt();
        assert!((r.c[0] - 1.0).abs() < 1e-15);
        assert!((r.c[1] - 1.0).abs() < 1e-15);
        assert!(r.c[2].abs() < 1e-15);
        assert!(r.c[3].abs() < 1e-15);
    }
}
