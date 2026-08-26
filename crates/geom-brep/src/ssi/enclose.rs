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
//!
//! # The M6-2 seam: generic scalars in, ring out
//!
//! [`Box3`] stays a **C9-ring object** — its fields are
//! [`RingInterval`]s, and `RingInterval` deliberately does not implement
//! `Real` (`geom_core::real`'s `Enclosure` rationale), so there is no
//! "`Box3<T>`" to want. What M6-2 lifted is the **seam**: every
//! constructor and entry point here now takes the caller's own scalar
//! and crosses into the ring through its bracket
//! ([`geom_core::Bounds`]), instead of demanding `f64` operands and
//! walling the whole certificate off the interval lane (M5-LOG PR 9c
//! deviation 2).
//!
//! The bound is the sole bound [`geom_core::CertifiedBounds`] — the pair
//! of bracket doors, named, so certification code that reads both writes
//! one bound rather than a compound one. Nothing here DECIDES; the
//! discipline's compound-`Bounds` rule is about a parameter that decides
//! *and* brackets, and this file has no decide half. An operand enters
//! as `[lo, hi]` and every
//! subsequent operation is ring arithmetic, so a widened operand widens
//! the enclosure — which can only cost a refusal. At `f64` the bracket
//! is the value, so this lane's numbers are what they were, up to the
//! ring's outward rounding of the pad itself.

use geom::{NurbsSurface, Surface, SurfaceWindow};
use geom_core::{CertifiedBounds, CertifiedEnclosure, Point3, RingInterval, Vec3};

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
    /// The box `[cx−r, cx+r] × …` around `c` — the caller's scalar
    /// crosses into the ring here (module docs' seam).
    pub(crate) fn around<T: CertifiedBounds>(c: Point3<T>, r: T) -> Self {
        let g = pad_interval(r);
        Self {
            x: ring(c.x) + g,
            y: ring(c.y) + g,
            z: ring(c.z) + g,
        }
    }

    /// The box spanned by two corners (componentwise hull).
    pub(crate) fn between<T: CertifiedBounds>(a: Point3<T>, b: Point3<T>) -> Self {
        Self {
            x: RingInterval::hull(ring(a.x), ring(b.x)),
            y: RingInterval::hull(ring(a.y), ring(b.y)),
            z: RingInterval::hull(ring(a.z), ring(b.z)),
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
    pub(crate) fn pad<T: CertifiedBounds>(self, r: T) -> Self {
        let g = pad_interval(r);
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

/// **The seam** (module docs): one scalar's bracket, as a ring
/// enclosure. At `f64` this is `RingInterval::point`; at the interval
/// scalar it carries the whole enclosure in, so nothing downstream can
/// narrow what the caller could not.
/// The seam itself: the caller's scalar crossing into the ring.
///
/// The bound is [`CertifiedEnclosure`], not `Bounds`. Everything built
/// from this is a *certificate*, and the ring has two states and no
/// decorations — so an operand that carries a sound bracket but whose
/// computation left a domain has no way to say so once it is across.
/// A refusing operand becomes ring poison, which fails every downstream
/// test rather than shrinking a box.
///
/// The body is [`RingInterval::from_certified`]; this wrapper exists for
/// the name at the call sites below, not for a second spelling of the
/// door.
fn ring<T: CertifiedEnclosure>(v: T) -> RingInterval {
    RingInterval::from_certified(v)
}

/// A symmetric pad of magnitude `r` — `[−r⁺, r⁺]` taken at the
/// bracket's UPPER end, because a pad is a widening and the sound
/// direction for a widening is the largest value the operand could
/// stand for.
///
/// An operand that cannot certify is refused at the door and yields
/// poison here; a bracket whose upper end is negative is admitted by the
/// door and yields poison at [`RingInterval::from_bounds`] (`−hi ≤ hi`
/// fails). Either way the pad fails every downstream test rather than
/// shrinking a box, and the two refusals stay distinct because only one
/// of them is about the operand's right to certify anything. Stated
/// precisely because the weaker claim is the true one: a bracket that
/// merely STRADDLES zero has `hi ≥ 0` and pads by its upper end, which
/// is sound — it is only an entirely-negative radius that poisons.
fn pad_interval<T: CertifiedEnclosure>(r: T) -> RingInterval {
    match r.certified_bracket() {
        Some((_, hi)) => RingInterval::from_bounds(-hi, hi),
        None => RingInterval::poison(),
    }
}

fn dot3(a: [RingInterval; 3], b: [RingInterval; 3]) -> RingInterval {
    // Ascending association (D9), matching `Vec3::dot`.
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// (Component-for-component `offset_meters::cross`, which is the
/// borrow-shaped twin one crate module over; noted at both sites so
/// the duplication is a decision. A third consumer collapses them.)
fn cross3(a: [RingInterval; 3], b: [RingInterval; 3]) -> [RingInterval; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn constv<T: CertifiedBounds>(v: Vec3<T>) -> [RingInterval; 3] {
    [ring(v.x), ring(v.y), ring(v.z)]
}

fn subp<T: CertifiedBounds>(b: Box3, p: Point3<T>) -> [RingInterval; 3] {
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
pub(crate) fn implicit_enclosure<T: CertifiedBounds>(
    surface: &Surface<T>,
    b: Box3,
) -> RingInterval {
    let two = RingInterval::point(2.0);
    match *surface {
        Surface::Plane { origin, normal, .. } => dot3(subp(b, origin), constv(normal)),
        Surface::Sphere { center, radius, .. } => {
            // `sqr`, never `ring(radius) * ring(radius)`: the radius
            // enclosure is one operand, and squaring it as two
            // independent ones widens it (the interval-square rule).
            (norm_sq(subp(b, center)) - ring(radius).sqr()) / (two * ring(radius))
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
            (norm_sq(w) - ring(radius).sqr()) / (two * ring(radius))
        }
        // `Approx` with the no-enclosure group: the implicit forms this
        // module encloses do not exist for a spline stand-in.
        Surface::Cone { .. }
        | Surface::Torus { .. }
        | Surface::Nurbs(_)
        | Surface::Approx(_) => RingInterval::poison(),
    }
}

/// The enclosure of `∇f` ([`crate::implicit::implicit_gradient`]) over
/// `b`. Same kind coverage and same reasons as
/// [`implicit_enclosure`].
pub(crate) fn implicit_gradient_enclosure<T: CertifiedBounds>(
    surface: &Surface<T>,
    b: Box3,
) -> [RingInterval; 3] {
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
        // As the residual enclosure above: no implicit form, no
        // gradient enclosure.
        Surface::Cone { .. } | Surface::Torus { .. } | Surface::Nurbs(_) | Surface::Approx(_) => {
            poison
        }
    }
}

/// `(∇f₁ × ∇f₂)·e` over `b` — **the uniqueness-tube quantity** (module
/// docs). An enclosure excluding zero proves the solution set inside
/// `b` is a graph over the `e` axis: one arc, and therefore exactly one
/// component to select.
pub(crate) fn graph_margin<T: CertifiedBounds>(
    s1: &Surface<T>,
    s2: &Surface<T>,
    b: Box3,
    e: Vec3<T>,
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
pub(crate) struct NurbsBoxes<'a, T: CertifiedBounds> {
    surface: &'a NurbsSurface<T>,
}

impl<'a, T: CertifiedBounds> NurbsBoxes<'a, T> {
    /// Wrap a surface.
    pub(crate) fn new(surface: &'a NurbsSurface<T>) -> Self {
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
        // `span_range` answers in validated spans, but a cell RANGE is
        // what this returns: the interior of the rectangle is neither
        // end, so the two ends' proofs do not cover it. The callers
        // re-derive each cell's window with `NurbsSurface::window`,
        // which is also what skips the empty spans in between — hence
        // indices here, and the pair read back out.
        let (u_lo, u_hi) = ku.span_range(cu.0, cu.1);
        let (v_lo, v_hi) = kv.span_range(cv.0, cv.1);
        ((u_lo.index(), u_hi.index()), (v_lo.index(), v_hi.index()))
    }

    /// The hull of the Cartesian control block of one span cell.
    ///
    /// The cell is a [`SurfaceWindow`], so the `su < pu || sv < pv`
    /// refusal this used to open with is gone: the window's two `Span`s
    /// carry `index ≥ degree` from their construction. What remains is
    /// the length check the window cannot make — `ctl.get`, below,
    /// against the caller's control array.
    fn cell_point_box(&self, win: SurfaceWindow) -> Box3 {
        let ctl = self.surface.control();
        let mut out: Option<Box3> = None;
        // Same ascending walk as the open-coded `(su − pu)..=su` pair
        // (D9): `row(i) + j` IS `iu·nv + iv` over the same indices.
        for i in 0..=win.span_u().degree() {
            let row = win.row(i);
            for j in 0..=win.span_v().degree() {
                let Some(p) = ctl.get(row + j) else {
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
    ///
    /// Both the range refusal (`su < pu`) and the degree-0 one this
    /// used to open with are gone with the [`SurfaceWindow`]: the
    /// former is the `Span` invariant, and `KnotVector::clamped`
    /// refuses degree 0 outright, so `pu == 0 || pv == 0` named a state
    /// no constructible window can reach. The remaining `.get`
    /// refusals — against `ctl`, `wts` and the raw knot slices — are
    /// the ones a window cannot make.
    fn cell_homogeneous_deriv(
        &self,
        win: SurfaceWindow,
        along_u: bool,
    ) -> (Box3, RingInterval, RingInterval) {
        let s = self.surface;
        let (pu, pv) = (win.span_u().degree(), win.span_v().degree());
        let nv = win.stride();
        let ctl = s.control();
        let wts = s.weights();
        let (ku, kv) = (s.knots_u().knots(), s.knots_v().knots());
        let mut abox: Option<Box3> = None;
        let mut wd: Option<RingInterval> = None;
        let mut w: Option<RingInterval> = None;
        // The derivative spline in direction d has degree p−1 and its
        // local block on this cell is the divided differences over the
        // Cartesian block shifted by one index in d.
        // Differencing drops the window's top index in the direction it
        // acts on, which is exactly `Span::first_derived_window` — so
        // the `su − pu` / `su − 1` pairs are not subtractions here any
        // more, and the pair that is NOT differenced keeps the full
        // window.
        let (uw, vw) = if along_u {
            (win.span_u().first_derived_window(), win.span_v().window())
        } else {
            (win.span_u().window(), win.span_v().first_derived_window())
        };
        for iu in uw {
            for iv in vw.clone() {
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
                // Homogeneous coefficients A = w·P. The weight is `f64`
                // structure and the control point is the caller's
                // scalar, so the product is formed IN THE RING — the
                // seam, not a collapse.
                let (rw0, rw1) = (ring(w0), ring(w1));
                let a0 = [rw0 * ring(p0.x), rw0 * ring(p0.y), rw0 * ring(p0.z)];
                let a1 = [rw1 * ring(p1.x), rw1 * ring(p1.y), rw1 * ring(p1.z)];
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
                // An EMPTY span cell covers no parameters, so its
                // control block is not part of what this box must
                // contain — skipping it is sound and strictly tighter.
                // The corner cell is always nonempty (`cells` locates
                // its ends with `span_at`), so the hull is never left
                // unseeded by this skip.
                let Some(win) = self.surface.window(su, sv) else {
                    continue;
                };
                let b = self.cell_point_box(win);
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
                // Empty cells contribute nothing here either — see
                // [`NurbsBoxes::point_box`]'s note.
                let Some(win) = self.surface.window(su, sv) else {
                    continue;
                };
                let (ad, wd, w) = self.cell_homogeneous_deriv(win, along_u);
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
        // The midpoint evaluation is the only EVALUATION in this
        // module, and it happens at the caller's scalar and then
        // crosses the seam — so a widened control net widens the box
        // rather than being silently collapsed.
        // `eval_in_span` at the midpoint's own span rather than `eval`:
        // the parameter is a thin `f64` structure value, so its span is
        // unique and no `SpanLocate` hull is needed.
        let c = self.surface.eval_in_span(
            self.surface.window_at(um, vm),
            T::from_f64(um),
            T::from_f64(vm),
        );
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
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
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

    /// A rational surface whose u direction carries an interior knot of
    /// **multiplicity 2**, so `[0.5, 0.5]` is an empty span sitting
    /// between two nonempty ones — the cell the box loops now skip.
    fn multiplicity_2_patch() -> NurbsSurface<f64> {
        let ku =
            geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2)
                .expect("clamped quadratic");
        let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1)
            .expect("clamped linear");
        let (nu, nv) = (ku.control_count(), kv.control_count());
        let mut control = Vec::with_capacity(nu * nv);
        let mut weights = Vec::with_capacity(nu * nv);
        for iu in 0..nu {
            for iv in 0..nv {
                let (a, b) = (iu as f64, iv as f64);
                control.push(Point3::new(a * 0.4, b * 1.1, 0.3 * a * b - 0.2 * a.powi(2)));
                weights.push(0.6 + 0.35 * ((iu * 3 + iv) % 5) as f64);
            }
        }
        NurbsSurface::new(ku, kv, control, weights).expect("valid patch")
    }

    /// The box loops now SKIP an empty span cell instead of hulling its
    /// control block. This pins that the skip loses nothing: the box is
    /// exactly the hull of the control points the surviving (nonempty)
    /// cells name — computed here from the two `Span` windows directly,
    /// never from the loop under test.
    ///
    /// Exact equality is available because a hull is min/max over `f64`
    /// points: order-independent, no rounding. That is deliberately a
    /// stronger and cheaper claim than sampling would give — sampled
    /// containment against this box is NOT exact, because `ders`' f64
    /// evaluation can land an ulp outside the true control hull at a
    /// domain edge (measured on this fixture: `S(0.55, 1).y` exceeds the
    /// largest control `y` by 3e-16). The box bounds the real surface;
    /// consumers pad by the certified tube radius before comparing an
    /// evaluated point to it, and this row tests the window, not that
    /// pad.
    #[test]
    fn a_skipped_empty_span_cell_removes_nothing_from_the_box() {
        let s = multiplicity_2_patch();
        let (ku, kv) = (s.knots_u(), s.knots_v());
        // Anti-slack: the fixture must really present an empty cell, or
        // this row covers nothing.
        let empty: Vec<usize> = (ku.first_span()..=ku.last_span())
            .filter(|i| ku.span(*i).is_none())
            .collect();
        assert_eq!(
            empty,
            vec![3],
            "the fixture must carry exactly one empty span"
        );
        let boxes = NurbsBoxes::new(&s);
        let (u0, u1, v0, v1) = (0.2, 0.8, 0.0, 1.0);
        let ((su0, su1), (sv0, sv1)) = boxes.cells(u0, u1, v0, v1);
        assert!(
            (su0..=su1).contains(&3),
            "the rectangle must straddle the empty cell, got {su0}..={su1}"
        );
        // The expected hull, taken off the `Span` windows themselves.
        let nv = kv.control_count();
        let mut expect: Option<Box3> = None;
        for su in su0..=su1 {
            for sv in sv0..=sv1 {
                let (Some(a), Some(b)) = (ku.span(su), kv.span(sv)) else {
                    continue;
                };
                for iu in a.window() {
                    for iv in b.window() {
                        let p = s.control()[iu * nv + iv];
                        let one = Box3::between(p, p);
                        expect = Some(match expect {
                            None => one,
                            Some(acc) => acc.hull(one),
                        });
                    }
                }
            }
        }
        let expect = expect.expect("at least one nonempty cell");
        let pb = boxes.point_box(u0, u1, v0, v1);
        for (got, want, axis) in [
            (pb.x, expect.x, "x"),
            (pb.y, expect.y, "y"),
            (pb.z, expect.z, "z"),
        ] {
            assert!(
                got.lo().to_bits() == want.lo().to_bits()
                    && got.hi().to_bits() == want.hi().to_bits(),
                "{axis}: box [{}, {}] is not the surviving cells' hull [{}, {}]",
                got.lo(),
                got.hi(),
                want.lo(),
                want.hi()
            );
        }
        // The derivative boxes survive the same skip — poison here would
        // mean it left their hulls unseeded.
        for along_u in [true, false] {
            let d = boxes.deriv_box(u0, u1, v0, v1, along_u);
            assert!(
                !d.x.is_poison() && !d.y.is_poison() && !d.z.is_poison(),
                "deriv box (along_u = {along_u}) poisoned across the empty cell"
            );
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

    /// **The two crossings agree with the door, whichever way it
    /// answers.** `ring` destructures a refusal to poison and
    /// `pad_interval` reads the bracket's upper end, so the door
    /// beginning to REFUSE a value it used to hand over as a NaN bracket
    /// takes a different branch through both of them. The `f64` lane is
    /// where that is reachable without a feature: NaN is its poison, and
    /// the door stops it here rather than leaving it to
    /// `RingInterval::from_bounds`.
    ///
    /// **An infinite radius is a different case and deliberately not
    /// poison** — ∞ is not `f64` poison (D4's Q1 residue), so the door
    /// hands it over and `[−∞, ∞]` is what a pad of unbounded radius
    /// honestly encloses. It is useless rather than wrong, and what
    /// stops it is its width, downstream. The row pins the boundary
    /// between the two so neither can drift into the other unnoticed.
    #[test]
    fn a_poisoned_f64_refuses_at_the_door_and_still_lands_on_poison() {
        let origin = Point3::new(0.0, 0.0, 0.0);
        let nan_pad = Box3::around(origin, f64::NAN);
        assert!(
            nan_pad.x.is_poison() && nan_pad.y.is_poison() && nan_pad.z.is_poison(),
            "a NaN radius padded to {:?}",
            nan_pad.x
        );
        let inf_pad = Box3::around(origin, f64::INFINITY);
        assert_eq!(
            (inf_pad.x.lo(), inf_pad.x.hi()),
            (f64::NEG_INFINITY, f64::INFINITY),
            "an infinite radius must enclose everything, not refuse"
        );

        let bad_centre = Box3::around(Point3::new(f64::NAN, 0.0, 0.0), 0.5);
        assert!(bad_centre.x.is_poison(), "a NaN centre built a box");
        assert!(
            !bad_centre.y.is_poison(),
            "the healthy coordinates must still build: poison is per-axis here"
        );
        let good = Box3::around(origin, 0.5);
        assert!(
            !good.x.is_poison() && good.x.contains(-0.5) && good.x.contains(0.5),
            "the healthy pad must still build, or the row proves nothing: {:?}",
            good.x
        );
        assert!(
            good.x.width() < 1.5,
            "the healthy pad widened to {:?}",
            good.x
        );
    }

    /// **The M6-2 seam requires the certified door.**
    ///
    /// `ring<T: CertifiedEnclosure>` is the only way an evaluation scalar
    /// enters the C9 ring here, and the ring has two states and no
    /// decorations — so an operand whose bracket is sound but whose
    /// computation left a domain has no way to say so once it is across.
    /// The rows sweep the operand across the domain boundary: the
    /// enclosure must refuse on exactly the side where the decoration
    /// degrades, which neither a laundering nor a uniformly-poisoning
    /// implementation can satisfy.
    #[cfg(feature = "interval")]
    #[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
    mod decoration_seam {
        use geom_core::{Bounds, CertifiedEnclosure, Interval, Real};

        use super::{Box3, Point3, RingInterval, Surface, Vec3, implicit_enclosure};

        /// One named entry point from an evaluation scalar into the ring.
        type Crossing = (&'static str, fn(Interval) -> RingInterval);

        /// `sqrt([a, 4])`: `Trv` with finite endpoints when `a < 0` forces
        /// a clamp, certified otherwise.
        fn operand(a: f64) -> Interval {
            Interval::from_bounds(a, 4.0).sqrt()
        }

        fn h(x: f64) -> Interval {
            Interval::from_f64(x)
        }

        fn unit_box() -> Box3 {
            Box3 {
                x: RingInterval::from_bounds(0.1, 0.9),
                y: RingInterval::from_bounds(-0.4, 0.3),
                z: RingInterval::from_bounds(-0.2, 0.7),
            }
        }

        /// A sphere whose centre carries `c` in x — the operand reaches
        /// `implicit_enclosure` through `subp`, the same `ring` helper
        /// every other entry point uses.
        fn sphere(c: Interval) -> Surface<Interval> {
            Surface::Sphere {
                center: Point3::new(c, h(0.0), h(0.0)),
                radius: h(1.0),
                axis: Vec3::new(h(0.0), h(0.0), h(1.0)),
                u_ref: Vec3::new(h(1.0), h(0.0), h(0.0)),
            }
        }

        fn crossings() -> [Crossing; 3] {
            [
                ("implicit_enclosure", |c| {
                    implicit_enclosure(&sphere(c), unit_box())
                }),
                ("Box3::around", |c| {
                    Box3::around(Point3::new(c, h(0.0), h(0.0)), h(0.5)).x
                }),
                ("Box3::between", |c| {
                    Box3::between(
                        Point3::new(c, h(0.0), h(0.0)),
                        Point3::new(h(3.0), h(0.0), h(0.0)),
                    )
                    .x
                }),
            ]
        }

        /// Each entry point, swept. Both mutation directions are caught:
        /// laundering certifies the whole sweep, uniform poisoning refuses
        /// the whole sweep, and the non-vacuity assertions reject both.
        #[test]
        fn every_ring_crossing_refuses_exactly_where_the_decoration_degrades() {
            for (name, cross) in crossings() {
                let (mut certified, mut refused) = (0, 0);
                for a in [-4.0, -1.0, -0.25, -1e-300, 0.0, 1e-300, 0.25, 1.0] {
                    let c = operand(a);
                    let e = cross(c);
                    assert_eq!(
                        e.is_poison(),
                        !c.is_certified(),
                        "{name}: sqrt([{a}, 4]) is {} but crossed as {e:?}",
                        if c.is_certified() {
                            "certified"
                        } else {
                            "domain-violated"
                        }
                    );
                    if c.is_certified() {
                        certified += 1;
                    } else {
                        refused += 1;
                    }
                }
                assert!(certified > 0, "{name}: sweep never certified — vacuous");
                assert!(refused > 0, "{name}: sweep never refused — vacuous");
            }
        }

        /// **The crossings must follow the certified door, not the
        /// bracket door** — the row that goes red if any of them is ever
        /// re-bounded to plain `Bounds`.
        ///
        /// The fixture's two doors disagree visibly: the bracket is a
        /// sound, finite `[0, 2]`, and the certified door refuses. So the
        /// crossing's output says which one it consulted, and a
        /// `Bounds`-bounded crossing is caught by name here rather than
        /// as an anonymous boolean.
        #[test]
        fn no_crossing_may_be_rebounded_to_the_bracket_door() {
            let c = operand(-1.0);
            let bracket = (Bounds::lo(c), Bounds::hi(c));
            assert_eq!(bracket, (0.0, 2.0), "fixture drifted");
            assert!(c.certified_bracket().is_none(), "fixture drifted");
            for (name, cross) in crossings() {
                let e = cross(c);
                assert!(
                    e.is_poison(),
                    "{name} consumed the BRACKET answer {bracket:?} and produced \
                     {e:?}. It is reading `Bounds`, not `CertifiedEnclosure` — the \
                     crossing's bound must require the certified door."
                );
            }
        }

        /// A radius is read at its UPPER end only (`pad_interval`), so it
        /// is the one crossing where forwarding a single endpoint could
        /// have been thought safe. It is not: the pad is a widening
        /// derived from a quantity whose computation left its domain.
        #[test]
        fn a_violated_radius_poisons_the_pad() {
            let c = Point3::new(h(0.0), h(0.0), h(0.0));
            let bad = Box3::around(c, operand(-1.0));
            assert!(bad.x.is_poison() && bad.y.is_poison() && bad.z.is_poison());
            let good = Box3::around(c, operand(1.0));
            assert!(!good.x.is_poison(), "the healthy half must still build");
        }
    }
}
