//! The two **trace shapes** of M5 PR 7 (spec §2, C3), behind one
//! interface: the underdetermined local system the Hoffmann stepper
//! marches on.
//!
//! An arm's trace shape is a **compile-time decision** (C5: no runtime
//! fallback), so this module offers exactly two concrete systems and no
//! way to pick between them at runtime:
//!
//! | shape | state | equations | SVD | used by |
//! |---|---|---|---|---|
//! | [`ImplicitPairR3`] | `(x, y, z) ∈ ℝ³` | `f₁ = f₂ = 0` | 2×3 | analytic × analytic (cylinder×sphere) |
//! | [`ParametricPairR4`] | `(u₁,v₁,u₂,v₂) ∈ ℝ⁴` | `G₁ − G₂ = 0` | 3×4 | anything with a NURBS operand (plane×NURBS) |
//!
//! # The ℝ⁴ trace and OQ4
//!
//! The ℝ⁴ trace is why the OQ4 checkpoint discharged the way it did:
//! one traced object carries **all three** classical representations on
//! **one parameter** — the 3-D curve through either chart, and both
//! pcurves as coordinate projections `(u₁,v₁)` and `(u₂,v₂)`. That
//! shared parameter is precisely the parameter-identity contract PR 6
//! ratified for pcurve caches, so the projections go through the PR 6
//! doors unchanged and the 3-D carrier stays the certified authority
//! (carrier-primary).
//!
//! # Derivative data
//!
//! Both systems must supply the right-hand sides of Hoffmann Eq. 6.1 at
//! orders 2 and 3. The ℝ³ system gets them from [`super::jet`]'s
//! truncated Taylor evaluation of the implicit forms along the
//! approximant; the ℝ⁴ system gets them from the charts' third-order
//! jets ([`geom::SurfaceJet3`]) through the written-out chain
//! rule. Neither ever forms a derivative tensor.
//!
//! `f64`-only and untrusted throughout — C6's selection lane.

use geom::{NurbsSurface, Surface, SurfaceJet3};
use geom_core::{Point3, Vec3};

use super::jet::implicit_path_jet;

/// The underdetermined local system of a trace: `M` equations in `N`
/// unknowns with `N = M + 1`.
///
/// Every method is **total**: poison in, poison out. A NaN anywhere
/// propagates into the SVD (which poisons wholesale) and then into the
/// step's transversality band, where it refuses.
pub(crate) trait LocalSystem<const M: usize, const N: usize> {
    /// `F(x)` — the residual vector, in meters.
    fn residual(&self, x: &[f64; N]) -> [f64; M];

    /// `∂F/∂x` — the `M × N` Jacobian, row-major.
    fn jacobian(&self, x: &[f64; N]) -> [[f64; N]; M];

    /// `b₂ = −D²F(d₁, d₁)` — the order-2 right-hand side.
    fn rhs2(&self, x: &[f64; N], d1: &[f64; N]) -> [f64; M];

    /// `b₃ = −(D³F(d₁,d₁,d₁) + 3·D²F(d₁,d₂))` — the order-3 right-hand
    /// side (Hoffmann's `b_{f,3}`).
    fn rhs3(&self, x: &[f64; N], d1: &[f64; N], d2: &[f64; N]) -> [f64; M];

    /// The 3-D point a state denotes — the carrier's sample.
    fn point(&self, x: &[f64; N]) -> Point3<f64>;

    /// A per-coordinate scale for the state space, in meters per unit
    /// of that coordinate. The closure and domain trileans need it to
    /// state parameter-space distances **in meters** (D4 ¶1: no
    /// dimensionless margins).
    fn coordinate_scale(&self, x: &[f64; N]) -> [f64; N];

    /// `‖d/ds P(x + s·d)‖` at `s = 0`, in meters per unit of the
    /// state-space parameter — the **exact** speed of the traced point,
    /// which converts a step in state units into a step in meters.
    /// Separate from [`LocalSystem::coordinate_scale`] because the ℝ⁴
    /// state carries two charts and only the carrier-primary one
    /// generates the 3-D curve.
    fn tangent_speed(&self, x: &[f64; N], d: &[f64; N]) -> f64;
}

// ---------------------------------------------------------------------
// ℝ³: an implicit pair
// ---------------------------------------------------------------------

/// The ℝ³ implicit-pair trace: march the point itself on
/// `f₁ = f₂ = 0` (Hoffmann §6.2, Eq. 6.1 at 2×3).
pub(crate) struct ImplicitPairR3<'a> {
    /// The first surface (its implicit form is equation 1).
    pub a: &'a Surface<f64>,
    /// The second surface.
    pub b: &'a Surface<f64>,
}

fn v3(x: &[f64; 3]) -> Vec3<f64> {
    Vec3::new(x[0], x[1], x[2])
}

fn p3(x: &[f64; 3]) -> Point3<f64> {
    Point3::new(x[0], x[1], x[2])
}

impl LocalSystem<2, 3> for ImplicitPairR3<'_> {
    fn residual(&self, x: &[f64; 3]) -> [f64; 2] {
        let p = p3(x);
        [
            crate::implicit::implicit_residual(self.a, p),
            crate::implicit::implicit_residual(self.b, p),
        ]
    }

    fn jacobian(&self, x: &[f64; 3]) -> [[f64; 3]; 2] {
        let p = p3(x);
        let g1 = crate::implicit::implicit_gradient(self.a, p);
        let g2 = crate::implicit::implicit_gradient(self.b, p);
        [[g1.x, g1.y, g1.z], [g2.x, g2.y, g2.z]]
    }

    fn rhs2(&self, x: &[f64; 3], d1: &[f64; 3]) -> [f64; 2] {
        let p = p3(x);
        let d = v3(d1);
        [
            -implicit_path_jet(self.a, p, d, Vec3::zero()).d2(),
            -implicit_path_jet(self.b, p, d, Vec3::zero()).d2(),
        ]
    }

    fn rhs3(&self, x: &[f64; 3], d1: &[f64; 3], d2: &[f64; 3]) -> [f64; 2] {
        let p = p3(x);
        let (u, w) = (v3(d1), v3(d2));
        [
            -implicit_path_jet(self.a, p, u, w).d3(),
            -implicit_path_jet(self.b, p, u, w).d3(),
        ]
    }

    fn point(&self, x: &[f64; 3]) -> Point3<f64> {
        p3(x)
    }

    fn coordinate_scale(&self, _x: &[f64; 3]) -> [f64; 3] {
        // The state already is meters.
        [1.0, 1.0, 1.0]
    }

    fn tangent_speed(&self, _x: &[f64; 3], d: &[f64; 3]) -> f64 {
        // The state IS the point: the speed is the tangent's length.
        v3(d).norm()
    }
}

// ---------------------------------------------------------------------
// ℝ⁴: a parametric pair
// ---------------------------------------------------------------------

/// A parametric chart with third-order derivative data — the ℝ⁴
/// trace's operand.
///
/// Only the two charts the PR's implemented arms need exist here: an
/// analytic plane (whose second and third partials vanish identically,
/// so the cubic term costs nothing) and a NURBS surface. Adding a
/// chart is adding an arm, per C12.1's per-arm retirement rule; there
/// is deliberately no `Surface`-wide constructor that would silently
/// admit a chart whose third-order data nobody has written.
pub(crate) enum Chart<'a> {
    /// `S(u, v) = origin + u·du + v·dv`, on the caller's window.
    Plane {
        /// `S(0, 0)`.
        origin: Point3<f64>,
        /// The (unit) `u` axis.
        du: Vec3<f64>,
        /// The (unit) `v` axis.
        dv: Vec3<f64>,
        /// The `u` window `[lo, hi]` — a plane is unbounded, so the
        /// domain is the caller's named extent, never a guess.
        u_range: (f64, f64),
        /// The `v` window `[lo, hi]`.
        v_range: (f64, f64),
    },
    /// A NURBS surface on its own knot domain.
    Nurbs(&'a NurbsSurface<f64>),
}

impl Chart<'_> {
    /// The plane chart of a [`Surface::Plane`] on a named window.
    /// Returns `None` for any other kind — the caller (an arm) knows
    /// its kinds at compile time and this is the assertion of that.
    pub(crate) fn plane_of(
        s: &Surface<f64>,
        u_range: (f64, f64),
        v_range: (f64, f64),
    ) -> Option<Self> {
        match *s {
            Surface::Plane {
                origin,
                normal,
                u_ref,
            } => Some(Self::Plane {
                origin,
                du: u_ref,
                dv: normal.cross(u_ref),
                u_range,
                v_range,
            }),
            _ => None,
        }
    }

    /// The parameter domain rectangle `((u0, u1), (v0, v1))`.
    pub(crate) fn domain(&self) -> ((f64, f64), (f64, f64)) {
        match self {
            Self::Plane {
                u_range, v_range, ..
            } => (*u_range, *v_range),
            Self::Nurbs(n) => (n.knots_u().domain(), n.knots_v().domain()),
        }
    }

    /// `S(u, v)`.
    pub(crate) fn eval(&self, u: f64, v: f64) -> Point3<f64> {
        match self {
            Self::Plane { origin, du, dv, .. } => *origin + *du * u + *dv * v,
            Self::Nurbs(n) => n.eval(u, v),
        }
    }

    /// The third-order jet at `(u, v)` — all partials with `k + l ≤ 3`.
    pub(crate) fn jet3(&self, u: f64, v: f64) -> SurfaceJet3<f64> {
        match self {
            Self::Plane { origin, du, dv, .. } => SurfaceJet3 {
                jet: geom::SurfaceJet {
                    point: *origin + *du * u + *dv * v,
                    du: *du,
                    dv: *dv,
                    duu: Vec3::zero(),
                    duv: Vec3::zero(),
                    dvv: Vec3::zero(),
                },
                duuu: Vec3::zero(),
                duuv: Vec3::zero(),
                duvv: Vec3::zero(),
                dvvv: Vec3::zero(),
            },
            Self::Nurbs(n) => n.ders3(u, v),
        }
    }
}

/// `d²/ds² G(u + s·a, v + s·b)` — the pure second-order chain term
/// (module docs: no tensor is formed).
fn chart_d2(j: &SurfaceJet3<f64>, a: f64, b: f64) -> Vec3<f64> {
    j.jet.duu * (a * a) + j.jet.duv * (2.0 * a * b) + j.jet.dvv * (b * b)
}

/// `d³/ds³ G(u + s·a + s²p/2, v + s·b + s²q/2)`, the whole of it —
/// the cubic chain terms plus the three mixed second-order terms
/// (derivation in the module docs of [`geom_core::linalg::svd`]; the
/// factor 3 on the mixed block is the one every hand derivation drops).
fn chart_d3(j: &SurfaceJet3<f64>, a: f64, b: f64, p: f64, q: f64) -> Vec3<f64> {
    let cubic = j.duuu * (a * a * a)
        + j.duuv * (3.0 * a * a * b)
        + j.duvv * (3.0 * a * b * b)
        + j.dvvv * (b * b * b);
    let mixed =
        j.jet.duu * (3.0 * a * p) + j.jet.duv * (3.0 * (a * q + b * p)) + j.jet.dvv * (3.0 * b * q);
    cubic + mixed
}

/// The ℝ⁴ parametric×parametric trace (Hoffmann §6.3.2, 3×4): the
/// state is `(u₁, v₁, u₂, v₂)` and the system is `G₁ − G₂ = 0`.
pub(crate) struct ParametricPairR4<'a> {
    /// The first chart — the **carrier-primary** one: `point()` reads
    /// the 3-D curve off this chart, and `(u₁, v₁)` is its pcurve.
    pub a: Chart<'a>,
    /// The second chart; `(u₂, v₂)` is its pcurve.
    pub b: Chart<'a>,
}

impl LocalSystem<3, 4> for ParametricPairR4<'_> {
    fn residual(&self, x: &[f64; 4]) -> [f64; 3] {
        let pa = self.a.eval(x[0], x[1]);
        let pb = self.b.eval(x[2], x[3]);
        let d = pa - pb;
        [d.x, d.y, d.z]
    }

    fn jacobian(&self, x: &[f64; 4]) -> [[f64; 4]; 3] {
        let ja = self.a.jet3(x[0], x[1]);
        let jb = self.b.jet3(x[2], x[3]);
        let (au, av) = (ja.jet.du, ja.jet.dv);
        let (bu, bv) = (jb.jet.du, jb.jet.dv);
        [
            [au.x, av.x, -bu.x, -bv.x],
            [au.y, av.y, -bu.y, -bv.y],
            [au.z, av.z, -bu.z, -bv.z],
        ]
    }

    fn rhs2(&self, x: &[f64; 4], d1: &[f64; 4]) -> [f64; 3] {
        let ja = self.a.jet3(x[0], x[1]);
        let jb = self.b.jet3(x[2], x[3]);
        let s = chart_d2(&ja, d1[0], d1[1]) - chart_d2(&jb, d1[2], d1[3]);
        [-s.x, -s.y, -s.z]
    }

    fn rhs3(&self, x: &[f64; 4], d1: &[f64; 4], d2: &[f64; 4]) -> [f64; 3] {
        let ja = self.a.jet3(x[0], x[1]);
        let jb = self.b.jet3(x[2], x[3]);
        let s =
            chart_d3(&ja, d1[0], d1[1], d2[0], d2[1]) - chart_d3(&jb, d1[2], d1[3], d2[2], d2[3]);
        [-s.x, -s.y, -s.z]
    }

    fn point(&self, x: &[f64; 4]) -> Point3<f64> {
        // Carrier-primary: chart A is the authority for the 3-D curve;
        // the residual limb checks chart B independently.
        self.a.eval(x[0], x[1])
    }

    fn coordinate_scale(&self, x: &[f64; 4]) -> [f64; 4] {
        // Meters per unit parameter: the chart speeds. A collapsed
        // speed yields a zero scale, which makes the margin that uses
        // it collapse and the predicate refuse — the honest behavior at
        // a chart singularity.
        let ja = self.a.jet3(x[0], x[1]);
        let jb = self.b.jet3(x[2], x[3]);
        [
            ja.jet.du.norm(),
            ja.jet.dv.norm(),
            jb.jet.du.norm(),
            jb.jet.dv.norm(),
        ]
    }

    fn tangent_speed(&self, x: &[f64; 4], d: &[f64; 4]) -> f64 {
        // Carrier-primary: the 3-D curve is chart A's image, so its
        // velocity is the one that converts state steps to meters.
        let ja = self.a.jet3(x[0], x[1]);
        (ja.jet.du * d[0] + ja.jet.dv * d[1]).norm()
    }
}

impl super::march::TransversalityData<3> for ImplicitPairR3<'_> {
    fn normals(&self, x: &[f64; 3]) -> super::march::NormalPair {
        let p = p3(x);
        (
            crate::implicit::implicit_gradient(self.a, p),
            crate::implicit::implicit_gradient(self.b, p),
        )
    }

    fn lever_arm(&self, x: &[f64; 3]) -> f64 {
        let p = p3(x);
        crate::implicit::curvature_lever_arm(self.a, p)
            .min(crate::implicit::curvature_lever_arm(self.b, p))
    }
}

impl super::march::TransversalityData<4> for ParametricPairR4<'_> {
    fn normals(&self, x: &[f64; 4]) -> super::march::NormalPair {
        let ja = self.a.jet3(x[0], x[1]);
        let jb = self.b.jet3(x[2], x[3]);
        (ja.jet.du.cross(ja.jet.dv), jb.jet.du.cross(jb.jet.dv))
    }

    fn lever_arm(&self, x: &[f64; 4]) -> f64 {
        // Chart curvature is not bounded in closed form for a NURBS
        // patch, so the honest arm at this shape is the CHART SPEED
        // over the second-derivative magnitude — the local radius of
        // curvature of the two parameter lines, folded min-wins, with
        // `f64::MAX` where the chart is flat (the plane identity, so a
        // plane operand never shrinks the arm).
        let mut arm = f64::MAX;
        for j in [self.a.jet3(x[0], x[1]), self.b.jet3(x[2], x[3])] {
            for (speed, second) in [
                (j.jet.du.norm(), j.jet.duu.norm()),
                (j.jet.dv.norm(), j.jet.dvv.norm()),
            ] {
                if second > 0.0 {
                    arm = arm.min(speed * speed / second);
                }
            }
        }
        arm
    }
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::needless_range_loop
)]
mod tests {
    use super::*;
    use geom_core::spline::KnotVector;

    fn sphere() -> Surface<f64> {
        Surface::Sphere {
            center: Point3::new(0.0, 0.0, 0.0),
            radius: 1.0,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    fn cylinder() -> Surface<f64> {
        Surface::Cylinder {
            origin: Point3::new(0.4, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 0.4,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    fn bilinear() -> NurbsSurface<f64> {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let mut control = Vec::new();
        for iu in 0..3 {
            for iv in 0..2 {
                let x = iu as f64 * 0.5;
                control.push(Point3::new(x, iv as f64, 0.4 * x * (1.0 - x)));
            }
        }
        NurbsSurface::new(ku, kv, control, vec![1.0; 6]).unwrap()
    }

    /// The Jacobian rows must be the implicit gradients — the system is
    /// the same object the certificate later encloses.
    #[test]
    fn r3_jacobian_is_the_gradient_pair() {
        let (a, b) = (sphere(), cylinder());
        let sys = ImplicitPairR3 { a: &a, b: &b };
        let x = [0.8, 0.3, 0.2];
        let j = sys.jacobian(&x);
        let g = crate::implicit::implicit_gradient(&a, p3(&x));
        assert_eq!(j[0][0].to_bits(), g.x.to_bits());
        assert_eq!(j[0][2].to_bits(), g.z.to_bits());
    }

    /// The order-2 and order-3 right-hand sides must be exactly the
    /// non-`r⁽ᵐ⁾` part of `dᵐ/dsᵐ F(r̃(s))`, which is checkable by
    /// differencing `F` along the very path the approximant describes.
    #[test]
    fn r3_taylor_rhs_reproduces_the_composite_derivatives() {
        let (a, b) = (sphere(), cylinder());
        let sys = ImplicitPairR3 { a: &a, b: &b };
        let x = [0.75, 0.35, 0.2];
        let d1 = [0.3, -0.6, 0.5];
        let d2 = [0.1, 0.2, -0.15];
        let h = 1.0e-3;
        let f_at = |t: f64, k: usize| {
            let q = [
                x[0] + t * d1[0] + 0.5 * t * t * d2[0],
                x[1] + t * d1[1] + 0.5 * t * t * d2[1],
                x[2] + t * d1[2] + 0.5 * t * t * d2[2],
            ];
            sys.residual(&q)[k]
        };
        let b3 = sys.rhs3(&x, &d1, &d2);
        for k in 0..2 {
            // Third central difference, Richardson-extrapolated.
            let d3a = (f_at(2.0 * h, k) - 2.0 * f_at(h, k) + 2.0 * f_at(-h, k) - f_at(-2.0 * h, k))
                / (2.0 * h * h * h);
            let d3b = (f_at(4.0 * h, k) - 2.0 * f_at(2.0 * h, k) + 2.0 * f_at(-2.0 * h, k)
                - f_at(-4.0 * h, k))
                / (16.0 * h * h * h);
            let d3 = (4.0 * d3a - d3b) / 3.0;
            assert!(
                (b3[k] + d3).abs() < 1e-4,
                "b3[{k}] = {} vs −h‴ = {}",
                b3[k],
                -d3
            );
        }
    }

    #[test]
    fn r4_jacobian_columns_are_the_two_charts_partials() {
        let n = bilinear();
        let plane = Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.05),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let sys = ParametricPairR4 {
            a: Chart::plane_of(&plane, (-1.0, 1.0), (-1.0, 1.0)).unwrap(),
            b: Chart::Nurbs(&n),
        };
        let x = [0.3, 0.4, 0.5, 0.5];
        let j = sys.jacobian(&x);
        // Plane's u axis is +x, v axis is normal×u_ref = +y.
        assert!((j[0][0] - 1.0).abs() < 1e-15);
        assert!((j[1][1] - 1.0).abs() < 1e-15);
        let nb = n.ders(0.5, 0.5);
        assert!((j[0][2] + nb.du.x).abs() < 1e-15);
        assert!((j[2][3] + nb.dv.z).abs() < 1e-15);
    }

    #[test]
    fn r4_taylor_rhs_reproduces_the_composite_derivatives() {
        let n = bilinear();
        let plane = Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.05),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let sys = ParametricPairR4 {
            a: Chart::plane_of(&plane, (-1.0, 1.0), (-1.0, 1.0)).unwrap(),
            b: Chart::Nurbs(&n),
        };
        let x = [0.3, 0.4, 0.45, 0.55];
        let d1 = [0.2, -0.3, 0.15, 0.25];
        let d2 = [0.05, 0.1, -0.08, 0.02];
        let h = 1.0e-3;
        let f_at = |t: f64, k: usize| {
            let mut q = [0.0f64; 4];
            for i in 0..4 {
                q[i] = x[i] + t * d1[i] + 0.5 * t * t * d2[i];
            }
            sys.residual(&q)[k]
        };
        let b3 = sys.rhs3(&x, &d1, &d2);
        for k in 0..3 {
            let d3a = (f_at(2.0 * h, k) - 2.0 * f_at(h, k) + 2.0 * f_at(-h, k) - f_at(-2.0 * h, k))
                / (2.0 * h * h * h);
            let d3b = (f_at(4.0 * h, k) - 2.0 * f_at(2.0 * h, k) + 2.0 * f_at(-2.0 * h, k)
                - f_at(-4.0 * h, k))
                / (16.0 * h * h * h);
            let d3 = (4.0 * d3a - d3b) / 3.0;
            assert!(
                (b3[k] + d3).abs() < 1e-3,
                "b3[{k}] = {} vs −h‴ = {}",
                b3[k],
                -d3
            );
        }
    }

    #[test]
    fn a_non_plane_surface_has_no_plane_chart() {
        assert!(Chart::plane_of(&sphere(), (0.0, 1.0), (0.0, 1.0)).is_none());
    }
}
