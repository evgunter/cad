//! Certified enclosures for the SSI certificate and the exhaustiveness
//! subdivision — **the C9 ring only** (M5 PR 7, C2.2/C2.3, C3).
//!
//! Everything the SSI *proof* obligations need is transcendental-free
//! (C9's whole argument), so it is computed here in
//! [`RingInterval`]: outward-rounded, always compiled, no feature gate,
//! no LGPL. Nothing in this module evaluates at a `Real` scalar and
//! nothing in it decides — it produces enclosures which the named
//! trileans upstairs classify.
//!
//! # What lives here
//!
//! - [`Box3`] — an axis-aligned ring box in ℝ³, the exhaustiveness
//!   cell and the uniqueness tube's link.
//! - [`implicit_enclosure`] — `f(B)` for an analytic surface over a
//!   box: **exclusion** when the enclosure excludes 0.
//! - [`implicit_gradient_enclosure`] — `∇f(B)`, the input to the
//!   transversality/graph enclosure.
//! - [`graph_margin`] — `(∇f₁ × ∇f₂)·e` over a box. This single number
//!   carries the whole uniqueness-tube argument: if its enclosure
//!   excludes zero then, on every slice `e·x = const` meeting the box,
//!   the 2×2 system `(f₁, f₂)` has non-singular Jacobian, so by the
//!   implicit function theorem the solution set inside the box is a
//!   **graph over the `e` axis** — one arc, no branch, no loop, no
//!   second component. Straddling zero at the floor is the genuine
//!   sliver (F6).
//! - [`NurbsBoxes`] — the same three readings for a NURBS chart,
//!   assembled from control-net hulls: the rational surface's point box
//!   is the *Cartesian* control hull over a span cell (positive weights
//!   ⇒ convex combination), and the derivative box comes from the
//!   homogeneous derivative net through the quotient rule
//!   `S_u = (A_u − S·w_u)/w`, all in the ring. (`geom_core::spline::
//!   hull` deliberately has no rational derivative path; this is that
//!   path, assembled at the consumer from the primitives it does have,
//!   which is where the surface-shaped bookkeeping belongs.)
//!
//! # Soundness posture
//!
//! Every function is **conservative or poison**: a widened enclosure
//! costs a refusal, never a wrong answer, and any structural surprise
//! (unsupported kind, malformed net, zero-touching divisor) yields
//! [`RingInterval::poison`], which fails every downstream test. There is
//! no path here that narrows an enclosure on a value branch.

use geom_core::{Point3, RingInterval, Vec3};
use geom_surfaces::{NurbsSurface, Surface};

/// An axis-aligned ring box in ℝ³.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Box3 {
    /// The x extent.
    pub x: RingInterval,
    /// The y extent.
    pub y: RingInterval,
    /// The z extent.
    pub z: RingInterval,
}

impl Box3 {
    /// The box `[cx−r, cx+r] × …` around `c`.
    pub(crate) fn around(c: Point3<f64>, r: f64) -> Self {
        Self {
            x: RingInterval::from_bounds(c.x - r, c.x + r),
            y: RingInterval::from_bounds(c.y - r, c.y + r),
            z: RingInterval::from_bounds(c.z - r, c.z + r),
        }
    }

    /// The box spanned by two corners (componentwise hull).
    pub(crate) fn between(a: Point3<f64>, b: Point3<f64>) -> Self {
        Self {
            x: RingInterval::hull(RingInterval::point(a.x), RingInterval::point(b.x)),
            y: RingInterval::hull(RingInterval::point(a.y), RingInterval::point(b.y)),
            z: RingInterval::hull(RingInterval::point(a.z), RingInterval::point(b.z)),
        }
    }

    /// Componentwise hull.
    pub(crate) fn hull(self, o: Self) -> Self {
        Self {
            x: RingInterval::hull(self.x, o.x),
            y: RingInterval::hull(self.y, o.y),
            z: RingInterval::hull(self.z, o.z),
        }
    }

    /// Grow every side by `r` (the certified tube radius).
    pub(crate) fn pad(self, r: f64) -> Self {
        let g = RingInterval::from_bounds(-r, r);
        Self {
            x: self.x + g,
            y: self.y + g,
            z: self.z + g,
        }
    }

    /// Whether the two boxes definitely do **not** meet — the
    /// exclusion test for the ℝ⁴ image-separation lane. Poison is never
    /// disjoint (poison excludes nothing).
    pub(crate) fn definitely_disjoint(self, o: Self) -> bool {
        let sep = |a: RingInterval, b: RingInterval| {
            !a.is_poison() && !b.is_poison() && (a.hi() < b.lo() || b.hi() < a.lo())
        };
        sep(self.x, o.x) || sep(self.y, o.y) || sep(self.z, o.z)
    }

    /// Whether `self` is contained in `o` — the "accounted" test.
    /// Poison contains nothing and is contained in nothing.
    pub(crate) fn contained_in(self, o: Self) -> bool {
        let inside = |a: RingInterval, b: RingInterval| {
            !a.is_poison() && !b.is_poison() && b.lo() <= a.lo() && a.hi() <= b.hi()
        };
        inside(self.x, o.x) && inside(self.y, o.y) && inside(self.z, o.z)
    }

    /// The largest side length (the cell's size, for the floor test).
    pub(crate) fn width(self) -> f64 {
        self.x.width().max(self.y.width()).max(self.z.width())
    }

    /// The center as an f64 point (a marcher seed, never a claim).
    pub(crate) fn center(self) -> Point3<f64> {
        Point3::new(
            0.5 * (self.x.lo() + self.x.hi()),
            0.5 * (self.y.lo() + self.y.hi()),
            0.5 * (self.z.lo() + self.z.hi()),
        )
    }

    /// Split along the widest axis (fixed tie-break: x, then y, then z
    /// — D9), returning the two halves in ascending order.
    pub(crate) fn split(self) -> (Self, Self) {
        let (wx, wy, wz) = (self.x.width(), self.y.width(), self.z.width());
        let half = |i: RingInterval| {
            let m = 0.5 * (i.lo() + i.hi());
            (
                RingInterval::from_bounds(i.lo(), m),
                RingInterval::from_bounds(m, i.hi()),
            )
        };
        if wx >= wy && wx >= wz {
            let (a, b) = half(self.x);
            (Self { x: a, ..self }, Self { x: b, ..self })
        } else if wy >= wz {
            let (a, b) = half(self.y);
            (Self { y: a, ..self }, Self { y: b, ..self })
        } else {
            let (a, b) = half(self.z);
            (Self { z: a, ..self }, Self { z: b, ..self })
        }
    }
}

fn ring(v: f64) -> RingInterval {
    RingInterval::point(v)
}

fn dot3(a: [RingInterval; 3], b: [RingInterval; 3]) -> RingInterval {
    // Ascending association (D9), matching `Vec3::dot`.
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross3(a: [RingInterval; 3], b: [RingInterval; 3]) -> [RingInterval; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn constv(v: Vec3<f64>) -> [RingInterval; 3] {
    [ring(v.x), ring(v.y), ring(v.z)]
}

fn subp(b: Box3, p: Point3<f64>) -> [RingInterval; 3] {
    [b.x - ring(p.x), b.y - ring(p.y), b.z - ring(p.z)]
}

/// `|q|²` with **tight** squares — [`RingInterval::sqr`], not `q*q`:
/// a straddling coordinate multiplied by itself as two independent
/// operands would report a spurious negative lower bound (the
/// `norm_squared` rationale, M2 PR 3).
fn norm_sq(q: [RingInterval; 3]) -> RingInterval {
    q[0].sqr() + q[1].sqr() + q[2].sqr()
}

/// The enclosure of the **linearized implicit residual in meters**
/// ([`crate::implicit::implicit_residual`]) over `b`.
///
/// Implemented for the kinds whose meters form is a ring expression
/// with no root: plane, sphere, cylinder. **Cone and torus yield
/// poison** — their meters forms carry a `sqrt` the C9 ring
/// deliberately lacks (`√(w·w)`), and converting their polynomial
/// composites back to meters needs a certified reciprocal of a
/// quantity the ring cannot bound tightly enough to be useful. No
/// rung-3 arm implemented in this PR routes them here; an arm that
/// wanted to would have to land that conversion first, which is
/// exactly the per-arm retirement rule (C12.1). [`Surface::Nurbs`] has
/// no implicit form at all.
pub(crate) fn implicit_enclosure(surface: &Surface<f64>, b: Box3) -> RingInterval {
    match *surface {
        Surface::Plane { origin, normal, .. } => dot3(subp(b, origin), constv(normal)),
        Surface::Sphere { center, radius, .. } => {
            (norm_sq(subp(b, center)) - ring(radius * radius)) / ring(2.0 * radius)
        }
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => {
            let q = subp(b, origin);
            let a = constv(axis);
            let h = dot3(q, a);
            // |w|² from the radial vector itself, NOT as |q|² − (q·â)².
            //
            // The algebraic identity is exact in ℝ and catastrophic in
            // interval arithmetic: the two terms share `q`, and
            // subtracting them as independent operands adds twice the
            // axial square to the width. On a box straddling `z ≈ 1`
            // that is a width of ~0.8 m² where the true width is zero,
            // which — divided by 2r for a thin cylinder — makes the
            // enclosure so wide it can never exclude anything, and the
            // exhaustiveness subdivision then refines the entire slab
            // to the floor. Forming `w = q − â(q·â)` first keeps the
            // cancellation inside one expression, and for an
            // axis-aligned cylinder it is exact.
            let w = [q[0] - a[0] * h, q[1] - a[1] * h, q[2] - a[2] * h];
            (norm_sq(w) - ring(radius * radius)) / ring(2.0 * radius)
        }
        Surface::Cone { .. } | Surface::Torus { .. } | Surface::Nurbs(_) => RingInterval::poison(),
    }
}

/// The enclosure of `∇f` ([`crate::implicit::implicit_gradient`]) over
/// `b`. Same kind coverage and same reasons as
/// [`implicit_enclosure`].
pub(crate) fn implicit_gradient_enclosure(surface: &Surface<f64>, b: Box3) -> [RingInterval; 3] {
    let poison = [RingInterval::poison(); 3];
    match *surface {
        Surface::Plane { normal, .. } => constv(normal),
        Surface::Sphere { center, radius, .. } => {
            let q = subp(b, center);
            [
                q[0] / ring(radius),
                q[1] / ring(radius),
                q[2] / ring(radius),
            ]
        }
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => {
            let q = subp(b, origin);
            let h = dot3(q, constv(axis));
            let a = constv(axis);
            [
                (q[0] - a[0] * h) / ring(radius),
                (q[1] - a[1] * h) / ring(radius),
                (q[2] - a[2] * h) / ring(radius),
            ]
        }
        Surface::Cone { .. } | Surface::Torus { .. } | Surface::Nurbs(_) => poison,
    }
}

/// `(∇f₁ × ∇f₂)·e` over `b` — **the uniqueness-tube quantity** (module
/// docs). An enclosure excluding zero proves the solution set inside
/// `b` is a graph over the `e` axis: one arc, and therefore exactly one
/// component to select.
pub(crate) fn graph_margin(
    s1: &Surface<f64>,
    s2: &Surface<f64>,
    b: Box3,
    e: Vec3<f64>,
) -> RingInterval {
    let g1 = implicit_gradient_enclosure(s1, b);
    let g2 = implicit_gradient_enclosure(s2, b);
    dot3(cross3(g1, g2), constv(e))
}

/// Control-net enclosures for a NURBS chart over a parameter rectangle
/// — the ℝ⁴ lane's substrate (module docs).
///
/// Every reading is a hull of control coefficients over the span cells
/// the rectangle touches, so it is a **convexity fact**, never an
/// evaluation: no sampling, no rounding-mode games, and refinement
/// shrinks it. The point box uses the Cartesian net directly (positive
/// weights make `S` a convex combination of the local control points,
/// which is exactly the rational hull property); the derivative box
/// goes through the homogeneous quotient rule.
pub(crate) struct NurbsBoxes<'a> {
    surface: &'a NurbsSurface<f64>,
}

impl<'a> NurbsBoxes<'a> {
    /// Wrap a surface.
    pub(crate) fn new(surface: &'a NurbsSurface<f64>) -> Self {
        Self { surface }
    }

    /// The (span_u, span_v) cell range touched by the rectangle.
    ///
    /// The rectangle is **clamped to the knot domains** first. Callers
    /// pad windows by a tube radius, which routinely pushes them past
    /// the clamped ends; a parameter outside the domain has no span,
    /// and letting that poison the enclosure would make every branch
    /// that reaches a surface edge fail its own uniqueness tube. The
    /// clamp is sound because the objects being enclosed — a pcurve, a
    /// foot point — cannot leave the domain either.
    fn cells(&self, u0: f64, u1: f64, v0: f64, v1: f64) -> ((usize, usize), (usize, usize)) {
        let ku = self.surface.knots_u();
        let kv = self.surface.knots_v();
        let (ud, vd) = (ku.domain(), kv.domain());
        let cu = (u0.clamp(ud.0, ud.1), u1.clamp(ud.0, ud.1));
        let cv = (v0.clamp(vd.0, vd.1), v1.clamp(vd.0, vd.1));
        (ku.span_range(cu.0, cu.1), kv.span_range(cv.0, cv.1))
    }

    /// The hull of the Cartesian control block of one span cell.
    fn cell_point_box(&self, su: usize, sv: usize) -> Box3 {
        let (pu, pv) = (
            self.surface.knots_u().degree(),
            self.surface.knots_v().degree(),
        );
        let nv = self.surface.knots_v().control_count();
        let ctl = self.surface.control();
        if su < pu || sv < pv {
            return poison_box();
        }
        let mut out: Option<Box3> = None;
        for iu in (su - pu)..=su {
            for iv in (sv - pv)..=sv {
                let idx = iu * nv + iv;
                let Some(p) = ctl.get(idx) else {
                    return poison_box();
                };
                let b = Box3::between(*p, *p);
                out = Some(match out {
                    None => b,
                    Some(acc) => acc.hull(b),
                });
            }
        }
        out.unwrap_or_else(poison_box)
    }

    /// The homogeneous derivative-net hull in one direction, plus the
    /// weight and weight-derivative hulls, over one span cell:
    /// `(A_d hull, w_d hull, w hull)`.
    fn cell_homogeneous_deriv(
        &self,
        su: usize,
        sv: usize,
        along_u: bool,
    ) -> (Box3, RingInterval, RingInterval) {
        let s = self.surface;
        let (pu, pv) = (s.knots_u().degree(), s.knots_v().degree());
        let nv = s.knots_v().control_count();
        let ctl = s.control();
        let wts = s.weights();
        if su < pu || sv < pv || pu == 0 || pv == 0 {
            return (poison_box(), RingInterval::poison(), RingInterval::poison());
        }
        let (ku, kv) = (s.knots_u().knots(), s.knots_v().knots());
        let mut abox: Option<Box3> = None;
        let mut wd: Option<RingInterval> = None;
        let mut w: Option<RingInterval> = None;
        // The derivative spline in direction d has degree p−1 and its
        // local block on this cell is the divided differences over the
        // Cartesian block shifted by one index in d.
        let (iu_lo, iu_hi) = if along_u {
            (su - pu, su - 1)
        } else {
            (su - pu, su)
        };
        let (iv_lo, iv_hi) = if along_u {
            (sv - pv, sv)
        } else {
            (sv - pv, sv - 1)
        };
        for iu in iu_lo..=iu_hi {
            for iv in iv_lo..=iv_hi {
                let idx0 = iu * nv + iv;
                let idx1 = if along_u {
                    (iu + 1) * nv + iv
                } else {
                    iu * nv + iv + 1
                };
                let (Some(p0), Some(p1), Some(&w0), Some(&w1)) =
                    (ctl.get(idx0), ctl.get(idx1), wts.get(idx0), wts.get(idx1))
                else {
                    return (poison_box(), RingInterval::poison(), RingInterval::poison());
                };
                // Homogeneous coefficients A = w·P.
                let a0 = [ring(w0 * p0.x), ring(w0 * p0.y), ring(w0 * p0.z)];
                let a1 = [ring(w1 * p1.x), ring(w1 * p1.y), ring(w1 * p1.z)];
                let (deg, span_lo, span_hi) = if along_u {
                    let Some((&lo, &hi)) = ku.get(iu + 1).zip(ku.get(iu + pu + 1)) else {
                        return (poison_box(), RingInterval::poison(), RingInterval::poison());
                    };
                    (pu as f64, lo, hi)
                } else {
                    let Some((&lo, &hi)) = kv.get(iv + 1).zip(kv.get(iv + pv + 1)) else {
                        return (poison_box(), RingInterval::poison(), RingInterval::poison());
                    };
                    (pv as f64, lo, hi)
                };
                let denom = ring(span_hi - span_lo);
                let scale = ring(deg) / denom;
                let d = Box3 {
                    x: (a1[0] - a0[0]) * scale,
                    y: (a1[1] - a0[1]) * scale,
                    z: (a1[2] - a0[2]) * scale,
                };
                abox = Some(match abox {
                    None => d,
                    Some(acc) => acc.hull(d),
                });
                let dw = (ring(w1) - ring(w0)) * scale;
                wd = Some(match wd {
                    None => dw,
                    Some(acc) => RingInterval::hull(acc, dw),
                });
                for wv in [ring(w0), ring(w1)] {
                    w = Some(match w {
                        None => wv,
                        Some(acc) => RingInterval::hull(acc, wv),
                    });
                }
            }
        }
        (
            abox.unwrap_or_else(poison_box),
            wd.unwrap_or_else(RingInterval::poison),
            w.unwrap_or_else(RingInterval::poison),
        )
    }

    /// A certified box for `S` over the parameter rectangle — the hull
    /// of the touched span cells' control blocks.
    pub(crate) fn point_box(&self, u0: f64, u1: f64, v0: f64, v1: f64) -> Box3 {
        let ((su0, su1), (sv0, sv1)) = self.cells(u0, u1, v0, v1);
        let mut out: Option<Box3> = None;
        for su in su0..=su1 {
            for sv in sv0..=sv1 {
                let b = self.cell_point_box(su, sv);
                out = Some(match out {
                    None => b,
                    Some(acc) => acc.hull(b),
                });
            }
        }
        out.unwrap_or_else(poison_box)
    }

    /// A certified box for `∂S/∂u` (or `∂S/∂v`) over the rectangle, via
    /// the quotient rule `S_d = (A_d − S·w_d)/w` evaluated entirely on
    /// hulls. Poison when the weight hull touches zero (the ring
    /// refuses the divisor) or the net is malformed.
    pub(crate) fn deriv_box(&self, u0: f64, u1: f64, v0: f64, v1: f64, along_u: bool) -> Box3 {
        let ((su0, su1), (sv0, sv1)) = self.cells(u0, u1, v0, v1);
        let sbox = self.point_box(u0, u1, v0, v1);
        let mut out: Option<Box3> = None;
        for su in su0..=su1 {
            for sv in sv0..=sv1 {
                let (ad, wd, w) = self.cell_homogeneous_deriv(su, sv, along_u);
                let d = Box3 {
                    x: (ad.x - sbox.x * wd) / w,
                    y: (ad.y - sbox.y * wd) / w,
                    z: (ad.z - sbox.z * wd) / w,
                };
                out = Some(match out {
                    None => d,
                    Some(acc) => acc.hull(d),
                });
            }
        }
        out.unwrap_or_else(poison_box)
    }

    /// A **first-order** certified box for `S` over an arbitrary
    /// sub-rectangle: `S(mid) ⊕ S_u·[−h_u, h_u] ⊕ S_v·[−h_v, h_v]`
    /// with the derivative boxes taken over the whole rectangle.
    ///
    /// Sound by the mean value theorem componentwise, and — unlike
    /// [`NurbsBoxes::point_box`] — it **keeps shrinking below the span
    /// cell**, which is what makes the exhaustiveness subdivision
    /// terminate on a surface with few spans. The tighter of the two is
    /// the intersection, but only containment is load-bearing, so the
    /// caller takes whichever it needs and never both.
    pub(crate) fn rect_box(&self, u0: f64, u1: f64, v0: f64, v1: f64) -> Box3 {
        let (ud, vd) = (
            self.surface.knots_u().domain(),
            self.surface.knots_v().domain(),
        );
        let (u0, u1) = (u0.clamp(ud.0, ud.1), u1.clamp(ud.0, ud.1));
        let (v0, v1) = (v0.clamp(vd.0, vd.1), v1.clamp(vd.0, vd.1));
        let um = 0.5 * (u0 + u1);
        let vm = 0.5 * (v0 + v1);
        let hu = 0.5 * (u1 - u0);
        let hv = 0.5 * (v1 - v0);
        let c = self.surface.eval(um, vm);
        let du = self.deriv_box(u0, u1, v0, v1, true);
        let dv = self.deriv_box(u0, u1, v0, v1, false);
        let ru = RingInterval::from_bounds(-hu, hu);
        let rv = RingInterval::from_bounds(-hv, hv);
        Box3 {
            x: ring(c.x) + du.x * ru + dv.x * rv,
            y: ring(c.y) + du.y * ru + dv.y * rv,
            z: ring(c.z) + du.z * ru + dv.z * rv,
        }
    }
}

fn poison_box() -> Box3 {
    Box3 {
        x: RingInterval::poison(),
        y: RingInterval::poison(),
        z: RingInterval::poison(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::implicit::{implicit_gradient, implicit_residual};

    fn sphere() -> Surface<f64> {
        Surface::Sphere {
            center: Point3::new(0.2, -0.1, 0.0),
            radius: 1.0,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    fn cylinder() -> Surface<f64> {
        Surface::Cylinder {
            origin: Point3::new(0.5, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 0.6,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    #[test]
    fn implicit_enclosure_contains_the_residual_at_every_sampled_point() {
        let b = Box3 {
            x: RingInterval::from_bounds(0.1, 0.9),
            y: RingInterval::from_bounds(-0.4, 0.3),
            z: RingInterval::from_bounds(-0.2, 0.7),
        };
        for s in [sphere(), cylinder()] {
            let e = implicit_enclosure(&s, b);
            assert!(!e.is_poison());
            for i in 0..5 {
                for j in 0..5 {
                    for k in 0..5 {
                        let p = Point3::new(
                            b.x.lo() + (b.x.hi() - b.x.lo()) * (i as f64 / 4.0),
                            b.y.lo() + (b.y.hi() - b.y.lo()) * (j as f64 / 4.0),
                            b.z.lo() + (b.z.hi() - b.z.lo()) * (k as f64 / 4.0),
                        );
                        let f = implicit_residual(&s, p);
                        assert!(e.contains(f), "{f} not in [{}, {}]", e.lo(), e.hi());
                    }
                }
            }
        }
    }

    #[test]
    fn gradient_enclosure_contains_the_gradient() {
        let b = Box3::around(Point3::new(0.8, 0.2, 0.1), 0.05);
        for s in [sphere(), cylinder()] {
            let g = implicit_gradient_enclosure(&s, b);
            let at = implicit_gradient(&s, b.center());
            for (i, v) in [at.x, at.y, at.z].iter().enumerate() {
                assert!(
                    g[i].contains(*v),
                    "{v} not in [{}, {}]",
                    g[i].lo(),
                    g[i].hi()
                );
            }
        }
    }

    #[test]
    fn a_definitely_transverse_box_has_a_zero_free_graph_margin() {
        // A point on the cylinder×sphere locus, well away from tangency.
        let b = Box3::around(Point3::new(1.1, 0.0, 0.5), 0.02);
        let e = Vec3::new(0.0, 1.0, 0.0);
        let m = graph_margin(&sphere(), &cylinder(), b, e);
        assert!(!m.is_poison());
        assert!(
            m.lo() > 0.0 || m.hi() < 0.0,
            "expected a zero-free margin, got [{}, {}]",
            m.lo(),
            m.hi()
        );
    }

    #[test]
    fn unsupported_kinds_poison_rather_than_guess() {
        let torus = Surface::Torus {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            major_radius: 1.0,
            minor_radius: 0.2,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let b = Box3::around(Point3::new(1.0, 0.0, 0.0), 0.1);
        assert!(implicit_enclosure(&torus, b).is_poison());
        assert!(implicit_gradient_enclosure(&torus, b)[0].is_poison());
        assert!(graph_margin(&torus, &sphere(), b, Vec3::new(1.0, 0.0, 0.0)).is_poison());
    }

    #[test]
    fn poison_boxes_are_never_disjoint_and_never_contained() {
        let good = Box3::around(Point3::new(0.0, 0.0, 0.0), 1.0);
        let bad = poison_box();
        assert!(!good.definitely_disjoint(bad));
        assert!(!bad.definitely_disjoint(good));
        assert!(!bad.contained_in(good));
        assert!(!good.contained_in(bad));
    }

    #[test]
    fn splitting_covers_the_parent() {
        let b = Box3 {
            x: RingInterval::from_bounds(0.0, 2.0),
            y: RingInterval::from_bounds(0.0, 1.0),
            z: RingInterval::from_bounds(0.0, 0.5),
        };
        let (l, r) = b.split();
        // Widest axis is x.
        assert!((l.x.hi() - 1.0).abs() < 1e-15 && (r.x.lo() - 1.0).abs() < 1e-15);
        assert!(l.width() < b.width());
    }
}
