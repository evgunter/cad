//! Certified enclosures for the SSI certificate and the exhaustiveness
//! subdivision — **certification arithmetic only** (M5 PR 7, C2.2/C2.3, C3).
//!
//! Everything the SSI *proof* obligations need is transcendental-free
//! (C9's whole argument), so it is computed here in
//! [`Interval`]: outward-rounded, always compiled, no feature gate,
//! no LGPL. Nothing in this module evaluates at a `Real` scalar and
//! nothing in it decides — it produces enclosures which the named
//! trileans upstairs classify.
//!
//! # What lives here
//!
//! - [`Box3`] — an axis-aligned enclosure box in ℝ³, the exhaustiveness
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
//!   second component. Straddling zero escalates: either a genuine
//!   sliver (F6) or the enclosure's remaining slack.
//! - [`NurbsBoxes`] — the same three readings for a NURBS chart,
//!   assembled from control-net hulls: the rational surface's point box
//!   is the *Cartesian* control hull over a span cell (positive weights
//!   ⇒ convex combination), and the derivative box comes from the
//!   homogeneous derivative net through the quotient rule
//!   `S_u = (A_u − S·w_u)/w`, over each cell cut to the rectangle's
//!   part of it, all in certification arithmetic. (`geom_core::spline::
//!   hull` deliberately has no rational derivative path; this is that
//!   path, assembled at the consumer from the primitives it does have,
//!   which is where the surface-shaped bookkeeping belongs.)
//!
//! # Soundness posture
//!
//! Every function is **conservative or refused**: a widened enclosure
//! costs a refusal, never a wrong answer, and any structural surprise
//! (unsupported kind, malformed net, zero-touching divisor) yields
//! [`Interval::refused`], which fails every downstream test. There is
//! no path here that narrows an enclosure on a value branch.
//!
//! # The M6-2 seam: generic scalars in, enclosures out
//!
//! [`Box3`] is a **certification object** — its fields are the
//! [`Interval`] enclosures every lane scalar crosses into, whatever
//! scalar the caller evaluates at, so there is no "`Box3<T>`" to
//! want. What M6-2 lifted is the **seam**: every
//! constructor and entry point here now takes the caller's own scalar
//! and crosses into certification arithmetic through its bracket
//! ([`geom_core::Bounds`]), instead of demanding `f64` operands and
//! walling the whole certificate off the interval lane (M5-LOG PR 9c
//! deviation 2). One entry point also takes plain `f64`:
//! `chart_transverse_margin` receives the pcurve's tangent as a
//! `(tx, ty, tn)` triple its caller selected on its own scalar — a
//! direction, which is structure in C6's `f64` lane, not an enclosure.
//!
//! The bound is the sole bound [`geom_core::CertifiedBounds`] — the pair
//! of bracket doors, named, so certification code that reads both writes
//! one bound rather than a compound one. Nothing here DECIDES; the
//! discipline's compound-`Bounds` rule is about a parameter that decides
//! *and* brackets, and this file has no decide half. An operand enters
//! as `[lo, hi]` and every
//! subsequent operation is certification arithmetic, so a widened operand widens
//! the enclosure — which can only cost a refusal. At `f64` the bracket
//! is the value, so this lane's numbers are what they were, up to the
//! ring's outward rounding of the pad itself.

use geom::{NurbsSurface, Surface, SurfaceWindow};
use geom_core::Bounds;
use geom_core::interval::certification::Certification;
use geom_core::interval::{div_down, max_bound, norm_sq, norm_sup};
use geom_core::spline::Span;
use geom_core::{CertifiedBounds, CertifiedEnclosure, Interval, Point3, SupSpeed, Vec3};

use super::{ChartAxis, ChartSpeedRefusal, SsiError, TubeDegeneracy};

/// An axis-aligned enclosure box in ℝ³.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Box3 {
    /// The x extent.
    pub x: Interval,
    /// The y extent.
    pub y: Interval,
    /// The z extent.
    pub z: Interval,
}

impl Box3 {
    /// The box `[cx−r, cx+r] × …` around `c` — the caller's scalar
    /// crosses into certification arithmetic here (module docs' seam).
    pub(crate) fn around<T: CertifiedBounds>(c: Point3<T>, r: T) -> Self {
        let g = pad_interval(r);
        Self {
            x: Interval::from_certified(c.x) + g,
            y: Interval::from_certified(c.y) + g,
            z: Interval::from_certified(c.z) + g,
        }
    }

    /// The box spanned by two corners (componentwise hull).
    pub(crate) fn between<T: CertifiedBounds>(a: Point3<T>, b: Point3<T>) -> Self {
        Self {
            x: Interval::hull(Interval::from_certified(a.x), Interval::from_certified(b.x)),
            y: Interval::hull(Interval::from_certified(a.y), Interval::from_certified(b.y)),
            z: Interval::hull(Interval::from_certified(a.z), Interval::from_certified(b.z)),
        }
    }

    /// Componentwise hull.
    pub(crate) fn hull(self, o: Self) -> Self {
        Self {
            x: Interval::hull(self.x, o.x),
            y: Interval::hull(self.y, o.y),
            z: Interval::hull(self.z, o.z),
        }
    }

    /// The componentwise meet of two enclosures of the same set:
    /// [`Certification::meet`] per axis, so a refused side, or two sides
    /// sharing no point, refuses that axis rather than keeping either.
    pub(crate) fn meet_enclosure(self, o: Self) -> Self {
        Self {
            x: self.x.meet(o.x),
            y: self.y.meet(o.y),
            z: self.z.meet(o.z),
        }
    }

    /// The componentwise intersection, or `self` where the two share no
    /// point or either side refused: a reach for what lies in both, which
    /// an empty meet has nothing of. Not a certified enclosure.
    pub(crate) fn meet(self, o: Self) -> Self {
        let side = |a: Interval, b: Interval| {
            if !(a.is_certified() && b.is_certified()) {
                return None;
            }
            let (lo, hi) = (a.lo().max(b.lo()), a.hi().min(b.hi()));
            (lo <= hi).then(|| Interval::from_bounds(lo, hi))
        };
        match (side(self.x, o.x), side(self.y, o.y), side(self.z, o.z)) {
            (Some(x), Some(y), Some(z)) => Self { x, y, z },
            _ => self,
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
    /// exclusion test for the ℝ⁴ image-separation lane. A refusal is
    /// never disjoint (a refusal excludes nothing).
    pub(crate) fn definitely_disjoint(self, o: Self) -> bool {
        let sep = |a: Interval, b: Interval| {
            a.is_certified() && b.is_certified() && (a.hi() < b.lo() || b.hi() < a.lo())
        };
        sep(self.x, o.x) || sep(self.y, o.y) || sep(self.z, o.z)
    }

    /// Whether `self` is contained in `o` — the "accounted" test.
    /// A refusal contains nothing and is contained in nothing.
    pub(crate) fn contained_in(self, o: Self) -> bool {
        let inside = |a: Interval, b: Interval| {
            a.is_certified() && b.is_certified() && b.lo() <= a.lo() && a.hi() <= b.hi()
        };
        inside(self.x, o.x) && inside(self.y, o.y) && inside(self.z, o.z)
    }

    /// The largest side length (the cell's size, for the floor test);
    /// `NaN` when any side is refused, so a refused axis fails the floor
    /// test rather than dropping out of it.
    pub(crate) fn width(self) -> f64 {
        [self.y.width(), self.z.width()]
            .into_iter()
            .fold(self.x.width(), max_bound)
    }

    /// The center as an f64 point (a marcher seed, never a claim).
    ///
    /// A refused axis has no center, and this says so with `NaN`
    /// rather than with the midpoint of a bracket that stands for
    /// nothing: the refusal is asked by name because interval arithmetic keeps
    /// its refusal in the decoration and a refused axis carries
    /// ordinary endpoints.
    pub(crate) fn center(self) -> Point3<f64> {
        let mid = |i: Interval| {
            if !i.is_certified() {
                f64::NAN
            } else {
                0.5 * (i.lo() + i.hi())
            }
        };
        Point3::new(mid(self.x), mid(self.y), mid(self.z))
    }

    /// Split along the widest axis (fixed tie-break: x, then y, then z
    /// — D9), returning the two halves in ascending order.
    pub(crate) fn split(self) -> (Self, Self) {
        let (wx, wy, wz) = (self.x.width(), self.y.width(), self.z.width());
        // A half of a refused axis is refused. `from_bounds` mints
        // a fresh `Com` out of whatever endpoints it is handed, so
        // re-minting a refused axis through it would launder the
        // refusal away — which is what the certified door exists to
        // prevent.
        let half = |i: Interval| {
            if !i.is_certified() {
                return (Interval::refused(), Interval::refused());
            }
            let m = 0.5 * (i.lo() + i.hi());
            (
                Interval::from_bounds(i.lo(), m),
                Interval::from_bounds(m, i.hi()),
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

/// A symmetric pad of magnitude `r` — `[−r⁺, r⁺]` taken at the
/// bracket's UPPER end, because a pad is a widening and the sound
/// direction for a widening is the largest value the operand could
/// stand for.
///
/// An operand that cannot certify is refused at the door and yields
/// a refusal here; a bracket whose upper end is negative is admitted by the
/// door and yields a refusal at [`Interval::from_bounds`] (`−hi ≤ hi`
/// fails). Either way the pad fails every downstream test rather than
/// shrinking a box, and the two refusals stay distinct because only one
/// of them is about the operand's right to certify anything. Stated
/// precisely because the weaker claim is the true one: a bracket that
/// merely STRADDLES zero has `hi ≥ 0` and pads by its upper end, which
/// is sound — it is only an entirely-negative radius that is refused.
fn pad_interval<T: CertifiedEnclosure>(r: T) -> Interval {
    match r.certified_bracket() {
        Some((_, hi)) => Interval::from_bounds(-hi, hi),
        None => Interval::refused(),
    }
}

fn dot3(a: [Interval; 3], b: [Interval; 3]) -> Interval {
    // Ascending association (D9), matching `Vec3::dot`.
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// (Component-for-component `offset_meters::cross`, which is the
/// borrow-shaped twin one crate module over; noted at both sites so
/// the duplication is a decision. A third consumer collapses them.)
fn cross3(a: [Interval; 3], b: [Interval; 3]) -> [Interval; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn constv<T: CertifiedBounds>(v: Vec3<T>) -> [Interval; 3] {
    [
        Interval::from_certified(v.x),
        Interval::from_certified(v.y),
        Interval::from_certified(v.z),
    ]
}

fn subp<T: CertifiedBounds>(b: Box3, p: Point3<T>) -> [Interval; 3] {
    [
        b.x - Interval::from_certified(p.x),
        b.y - Interval::from_certified(p.y),
        b.z - Interval::from_certified(p.z),
    ]
}

/// The enclosure of the **linearized implicit residual in meters**
/// ([`crate::implicit::implicit_residual`]) over `b`.
///
/// Implemented for the kinds whose meters form is a ring expression
/// with no root: plane, sphere, cylinder. **Cone and torus yield
/// a refusal** — their meters forms carry a `sqrt` certification arithmetic
/// deliberately does not take (`√(w·w)`; C9), and converting their
/// polynomial composites back to meters needs a certified reciprocal of a
/// quantity certification arithmetic cannot bound tightly enough to be useful. No
/// rung-3 arm implemented in this PR routes them here; an arm that
/// wanted to would have to land that conversion first, which is
/// exactly the per-arm retirement rule (C12.1). [`Surface::Nurbs`] has
/// no implicit form at all.
pub(crate) fn implicit_enclosure<T: CertifiedBounds>(surface: &Surface<T>, b: Box3) -> Interval {
    let two = Interval::point(2.0);
    match *surface {
        Surface::Plane { origin, normal, .. } => dot3(subp(b, origin), constv(normal)),
        Surface::Sphere { center, radius, .. } => {
            // `sqr`, never `r * r`: the radius enclosure is one
            // operand, and squaring it as two independent ones widens
            // it (the interval-square rule). One crossing, bound once,
            // for the same reason.
            let r = Interval::from_certified(radius);
            (norm_sq(&subp(b, center)) - r.sqr()) / (two * r)
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
            let r = Interval::from_certified(radius);
            (norm_sq(&w) - r.sqr()) / (two * r)
        }
        // `Approx` with the no-enclosure group: the implicit forms this
        // module encloses do not exist for a spline stand-in.
        Surface::Cone { .. } | Surface::Torus { .. } | Surface::Nurbs(_) | Surface::Approx(_) => {
            Interval::refused()
        }
    }
}

/// The enclosure of `∇f` ([`crate::implicit::implicit_gradient`]) over
/// `b`. Same kind coverage and same reasons as
/// [`implicit_enclosure`].
pub(crate) fn implicit_gradient_enclosure<T: CertifiedBounds>(
    surface: &Surface<T>,
    b: Box3,
) -> [Interval; 3] {
    let refused = [Interval::refused(); 3];
    match *surface {
        Surface::Plane { normal, .. } => constv(normal),
        Surface::Sphere { center, radius, .. } => {
            let q = subp(b, center);
            let r = Interval::from_certified(radius);
            [q[0] / r, q[1] / r, q[2] / r]
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
            let r = Interval::from_certified(radius);
            [
                (q[0] - a[0] * h) / r,
                (q[1] - a[1] * h) / r,
                (q[2] - a[2] * h) / r,
            ]
        }
        // As the residual enclosure above: no implicit form, no
        // gradient enclosure.
        Surface::Cone { .. } | Surface::Torus { .. } | Surface::Nurbs(_) | Surface::Approx(_) => {
            refused
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
) -> Interval {
    let g1 = implicit_gradient_enclosure(s1, b);
    let g2 = implicit_gradient_enclosure(s2, b);
    dot3(cross3(g1, g2), constv(e))
}

/// The plane × NURBS chart probe's transversality margin over one
/// chart window `rect` (`(u0, u1, v0, v1)`): `∇φ = (n·S_u, n·S_v)` read
/// along the chart direction transverse to the pcurve's tangent, divided
/// by the chart's stretch along that direction. `n` is the plane normal
/// already crossed into certification arithmetic; `tangent` is
/// `(t.x, t.y, ‖t‖)`, the tangent's bracket tops and a positive finite
/// norm, which select the direction (structure, not a bound). Both
/// readings come from the derivative numerators' vector terms
/// ([`CellNet::transverse_readings`]), so a rigid map that carries `n`
/// with the wall moves the margin by its rounding width. `Ok(None)`
/// when the stretch has no finite bound.
///
/// # Errors
///
/// [`TubeDegeneracy::WallConstantAcrossLocus`] when the stretch is
/// zero: the chart is constant along e⊥ over this window, and a narrower
/// window (a smaller rung) lies inside it, so it cannot cure that.
pub(super) fn chart_transverse_margin<T: CertifiedBounds>(
    boxes: &NurbsBoxes<'_, T>,
    n: [Interval; 3],
    rect: (f64, f64, f64, f64),
    tangent: (f64, f64, f64),
) -> Result<Option<f64>, SsiError> {
    let (tx, ty, tn) = tangent;
    // e⊥ = (−t.y, t.x)/‖t‖; the transverse derivative of φ.
    let ex = Interval::point(-ty / tn);
    let ey = Interval::point(tx / tn);
    // ∇φ·e⊥ is metres of plane-distance per CHART unit, so it is
    // not yet a margin: multiplying it by a lever arm in metres
    // would give metres² per chart unit (D4 ¶1 forbids exactly
    // that). Dividing by the chart's own stretch along e⊥ —
    // ‖S_u·ex + S_v·ey‖, metres per chart unit — cancels the chart
    // units and leaves the dimensionless sine-like quantity the ℝ³
    // lane's `(∇f₁×∇f₂)·e` already is. An UPPER bound on the
    // stretch is used, which can only shrink the margin: the safe
    // direction.
    let (phi, stretch) = boxes.transverse_readings(rect, n, ex, ey);
    // An admitted `+∞` stretch divides the margin to an exact `0`,
    // which the caller's fold then records as the certificate's worst
    // transversality — a number manufactured from an overflow, not a
    // measurement. A `NaN` stretch is refused the same way, and a
    // finite one is never negative, so `<= 0` is zero.
    if !stretch.is_finite() {
        return Ok(None);
    }
    if stretch <= 0.0 {
        return Err(SsiError::TubeDegenerate(
            TubeDegeneracy::WallConstantAcrossLocus,
        ));
    }
    // A lower bound over an upper bound stays one only rounded down.
    Ok(Some(div_down(zero_free_lower_bound(phi), stretch)))
}

/// The certified distance of an enclosure from zero: `0` when it
/// straddles (or is refused), which is exactly what makes the trilean
/// land in the sliver band.
/// (The mignitude. `offset_meters::mig` is the same arithmetic read
/// as a coefficient-hull assembly term rather than a decision; noted
/// at both sites.)
pub(super) fn zero_free_lower_bound(i: Interval) -> f64 {
    if !i.is_certified() {
        return 0.0;
    }
    if i.lo() > 0.0 {
        i.lo()
    } else if i.hi() < 0.0 {
        -i.hi()
    } else {
        0.0
    }
}

/// A NURBS wall's certified chart speeds over its whole domain, one per
/// axis: each a [`SupSpeed`] (metres per parameter unit) that is
/// positive and finite, which is what [`NurbsBoxes::chart_speeds`]
/// refuses short of.
#[derive(Clone, Copy, Debug)]
pub struct ChartSpeeds {
    pub(crate) u: SupSpeed<f64>,
    pub(crate) v: SupSpeed<f64>,
}

impl ChartSpeeds {
    /// The larger of the two: a sup over both axes, which is what the
    /// sweep's single chart floor divides by.
    pub(crate) fn max(self) -> SupSpeed<f64> {
        self.u.max(self.v)
    }

    /// A length in metres crossed into chart units per axis,
    /// `(r / s_u, r / s_v)`: the tube pad.
    pub(crate) fn pad(self, meters: f64) -> (f64, f64) {
        (self.u.to_param(meters), self.v.to_param(meters))
    }
}

/// A parameter window `[u0, u1] × [v0, v1]` is one when each pair is
/// ordered. A NaN end compares false, so it is refused with an inverted
/// pair. Every box over a window asks this before it clamps the window
/// to the domain: a clamp turns an inverted window past a domain end
/// into an ordered point.
fn ordered_window(u0: f64, u1: f64, v0: f64, v1: f64) -> bool {
    u0 <= u1 && v0 <= v1
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
/// goes through the homogeneous quotient rule over each cell's net cut
/// to the rectangle ([`CellNet::cut`]), so it shrinks with the
/// rectangle below a span cell too.
pub(crate) struct NurbsBoxes<'a, T: CertifiedBounds> {
    surface: &'a NurbsSurface<T>,
}

impl<'a, T: CertifiedBounds> NurbsBoxes<'a, T> {
    /// Wrap a surface.
    pub(crate) fn new(surface: &'a NurbsSurface<T>) -> Self {
        Self { surface }
    }

    /// The wall's `{u, v}` chart speeds over its whole domain, each
    /// [`NurbsBoxes::speed_sup`]: a function of the geometry, which a
    /// rigid map moves by its rounding width.
    ///
    /// # Errors
    ///
    /// [`ChartSpeedRefusal::Zero`] when an axis's speed is zero (the wall
    /// is constant along it) and [`ChartSpeedRefusal::NotFinite`] when it
    /// has no finite bound, `u` before `v`.
    pub(crate) fn chart_speeds(&self) -> Result<ChartSpeeds, SsiError> {
        let (ud, vd) = (
            self.surface.knots_u().domain(),
            self.surface.knots_v().domain(),
        );
        let speed = |axis: ChartAxis| {
            let m = self.speed_sup(ud.0, ud.1, vd.0, vd.1, axis == ChartAxis::U);
            if !m.is_finite() {
                Err(SsiError::ChartSpeed(ChartSpeedRefusal::NotFinite { axis }))
            } else if m <= 0.0 {
                Err(SsiError::ChartSpeed(ChartSpeedRefusal::Zero { axis }))
            } else {
                Ok(SupSpeed::new(m))
            }
        };
        Ok(ChartSpeeds {
            u: speed(ChartAxis::U)?,
            v: speed(ChartAxis::V)?,
        })
    }

    /// The (span_u, span_v) cell range touched by the rectangle, with
    /// the rectangle as clamped, or `None` for a window with a NaN or
    /// inverted end: such a window names no region, and clamping it
    /// would land a NaN end on the first span.
    ///
    /// The rectangle is **clamped to the knot domains** first. Callers
    /// pad windows by a tube radius, which routinely pushes them past
    /// the clamped ends; a parameter outside the domain has no span,
    /// and letting that refuse the enclosure would make every branch
    /// that reaches a surface edge fail its own uniqueness tube. The
    /// clamp is sound because the objects being enclosed — a pcurve, a
    /// foot point — cannot leave the domain either.
    fn cells(&self, u0: f64, u1: f64, v0: f64, v1: f64) -> Option<CellRange> {
        if !ordered_window(u0, u1, v0, v1) {
            return None;
        }
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
        Some(CellRange {
            u: (u_lo.index(), u_hi.index()),
            v: (v_lo.index(), v_hi.index()),
            clamped_u: cu,
            clamped_v: cv,
        })
    }

    /// A certified box for `∂S/∂u` (or `∂S/∂v`) over the rectangle, via
    /// the quotient rule `S_d = (A_d − S·w_d)/w` evaluated entirely on
    /// hulls, per span cell, with `S` in that cell's point box
    /// ([`CellNet::quotient_numerator`] gives the numerator's form).
    /// Each cell is also cut to the part of the rectangle inside it
    /// ([`CellNet::cut`]) and the two boxes met, so the box shrinks with
    /// the rectangle below a span cell and is never wider than the whole
    /// cell's. Refused
    /// when the weight hull touches zero (interval arithmetic refuses
    /// the divisor), the net is malformed, or the window has a NaN or
    /// inverted end. The tests' box reading of a cut cell; the chart
    /// readings take norms from the same cut ([`NurbsBoxes::speed_sup`]).
    #[cfg(test)]
    pub(crate) fn deriv_box(&self, u0: f64, u1: f64, v0: f64, v1: f64, along_u: bool) -> Box3 {
        self.deriv_hull(u0, u1, v0, v1, along_u, true)
    }

    /// [`NurbsBoxes::deriv_box`] read off every touched cell's whole net,
    /// without the cut: the exhaustiveness sweep's derivative box, whose
    /// slack the sweep's seeding is pinned against
    /// (`work/ssi/the-chart-sweeps-first-order-box-reads-its-derivative-off-the-whole-span-cell.md`).
    fn cell_deriv_box(&self, u0: f64, u1: f64, v0: f64, v1: f64, along_u: bool) -> Box3 {
        self.deriv_hull(u0, u1, v0, v1, along_u, false)
    }

    /// The derivative box over the rectangle, each touched cell's net cut
    /// to it or read whole.
    fn deriv_hull(&self, u0: f64, u1: f64, v0: f64, v1: f64, along_u: bool, cut: bool) -> Box3 {
        // Both boxes enclose `S_d` over the window's part of the cell,
        // so their meet does, and it is never wider than the whole
        // cell's: a zero the whole cell reads exactly stays exact.
        self.fold_cells(
            (u0, u1, v0, v1),
            cut,
            |net| net.derivative_box(along_u),
            Box3::meet_enclosure,
            Box3::hull,
        )
        .unwrap_or_else(refused_box)
    }

    /// An upper bound on `‖∂S/∂u‖` (or `‖∂S/∂v‖`) over the rectangle,
    /// read per span cell from the quotient numerator's vector terms
    /// ([`CellNet::derivative_norm_sup`]), each cell cut to the rectangle
    /// as [`NurbsBoxes::deriv_box`] cuts it. A function of the geometry:
    /// a rigid map moves it by its rounding width, where the norm of
    /// [`NurbsBoxes::deriv_box`] reads between 1× and √3× the same
    /// numerator bound depending on how the field sits against the axes,
    /// and is never below it. `NaN` when refused.
    pub(crate) fn speed_sup(&self, u0: f64, u1: f64, v0: f64, v1: f64, along_u: bool) -> f64 {
        self.fold_cells(
            (u0, u1, v0, v1),
            true,
            |net| net.derivative_norm_sup(along_u),
            min_bound,
            max_bound,
        )
        .unwrap_or(f64::NAN)
    }

    /// The chart probe's readings over the rectangle along the chart
    /// direction `(ex, ey)`: an enclosure of `n·(S_u·ex + S_v·ey)` and an
    /// upper bound on `‖S_u·ex + S_v·ey‖` ([`CellNet::transverse_readings`]),
    /// each cell cut to the rectangle and met with its whole net.
    fn transverse_readings(
        &self,
        (u0, u1, v0, v1): (f64, f64, f64, f64),
        n: [Interval; 3],
        ex: Interval,
        ey: Interval,
    ) -> (Interval, f64) {
        self.fold_cells(
            (u0, u1, v0, v1),
            true,
            |net| net.transverse_readings(n, ex, ey),
            |(a, s), (b, t)| (a.meet(b), min_bound(s, t)),
            |(a, s), (b, t)| (Interval::hull(a, b), max_bound(s, t)),
        )
        .unwrap_or((Interval::refused(), f64::NAN))
    }

    /// `read` over every nonempty span cell the rectangle touches,
    /// `join`ed across cells. With `cut`, each cell's net is also cut to
    /// the rectangle ([`CellNet::cut`]) and the two readings `meet`:
    /// both hold over the rectangle's part of the cell. `None` for a
    /// window with a NaN or inverted end or a malformed net.
    fn fold_cells<R>(
        &self,
        (u0, u1, v0, v1): (f64, f64, f64, f64),
        cut: bool,
        read: impl Fn(&CellNet) -> R,
        meet: impl Fn(R, R) -> R,
        join: impl Fn(R, R) -> R,
    ) -> Option<R> {
        let range = self.cells(u0, u1, v0, v1)?;
        let mut out: Option<R> = None;
        for su in range.u.0..=range.u.1 {
            for sv in range.v.0..=range.v.1 {
                // An EMPTY span cell covers no parameters, so skipping
                // it is sound and strictly tighter. The corner cell is
                // always nonempty (`cells` locates its ends with
                // `span_at`), so the fold is never left unseeded.
                let Some(win) = self.surface.window(su, sv) else {
                    continue;
                };
                let whole = CellNet::of_cell(win)?;
                let r = read(&whole);
                let r = match cut
                    .then(|| whole.cut(win, range.clamped_u, range.clamped_v))
                    .flatten()
                {
                    Some(net) => meet(r, read(&net)),
                    None => r,
                };
                out = Some(match out {
                    None => r,
                    Some(acc) => join(acc, r),
                });
            }
        }
        out
    }

    /// A **first-order** certified box for `S` over an arbitrary
    /// sub-rectangle: `S(mid) ⊕ S_u·[−h_u, h_u] ⊕ S_v·[−h_v, h_v]`
    /// with the derivative boxes taken over the whole span cells the
    /// rectangle touches ([`NurbsBoxes::cell_deriv_box`]).
    ///
    /// Sound by the mean value theorem componentwise, and — unlike a
    /// span cell's control hull — it **keeps shrinking below the span
    /// cell**, which is what makes the exhaustiveness subdivision
    /// terminate on a surface with few spans.
    pub(crate) fn rect_box(&self, u0: f64, u1: f64, v0: f64, v1: f64) -> Box3 {
        // The window door, before the clamp below: clamping an inverted
        // window past a domain end makes it an ordered point.
        if !ordered_window(u0, u1, v0, v1) {
            return refused_box();
        }
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
        let c = self
            .surface
            .window_at(um, vm)
            .eval_in_span(T::from_f64(um), T::from_f64(vm));
        let du = self.cell_deriv_box(u0, u1, v0, v1, true);
        let dv = self.cell_deriv_box(u0, u1, v0, v1, false);
        let ru = Interval::from_bounds(-hu, hu);
        let rv = Interval::from_bounds(-hv, hv);
        Box3 {
            x: Interval::from_certified(c.x) + du.x * ru + dv.x * rv,
            y: Interval::from_certified(c.y) + du.y * ru + dv.y * rv,
            z: Interval::from_certified(c.z) + du.z * ru + dv.z * rv,
        }
    }
}

/// The span cells a rectangle touches, and the rectangle clamped to
/// the knot domains ([`NurbsBoxes::cells`]).
struct CellRange {
    u: (usize, usize),
    v: (usize, usize),
    clamped_u: (f64, f64),
    clamped_v: (f64, f64),
}

/// One span cell's control block in certification arithmetic, cut to
/// a sub-rectangle or whole: the Cartesian points and weights,
/// `(pu + 1) × (pv + 1)` row-major, and the derivative scale
/// `degree / (knot span)` of each adjacent pair along `u` and along `v`.
/// The points may sit in a translated frame; every reading of a block
/// is a difference of two of its points or of a point with `S` in its
/// own point box, so the frame drops out.
#[derive(Debug)]
struct CellNet {
    nv: usize,
    pts: Vec<[Interval; 3]>,
    wts: Vec<Interval>,
    scale_u: Vec<Interval>,
    scale_v: Vec<Interval>,
}

impl CellNet {
    /// The cell's own control block. `None` when a read leaves the net
    /// or the knots: the window's invariants (`index ≥ degree`,
    /// `index ≤ last_span()`) put every read in range, so this is the
    /// total spelling (D9: an out-of-range read is a refused box, never
    /// an index panic), not a refusal any input reaches.
    fn of_cell<T: CertifiedBounds>(win: SurfaceWindow<'_, T>) -> Option<Self> {
        let s = win.surface();
        let (ctl, wts) = (s.control(), s.weights());
        let (spu, spv) = (win.span_u(), win.span_v());
        let (pu, pv) = (spu.degree(), spv.degree());
        let mut pts = Vec::with_capacity((pu + 1) * (pv + 1));
        let mut ws = Vec::with_capacity((pu + 1) * (pv + 1));
        for i in 0..=pu {
            let row = win.row(i);
            for j in 0..=pv {
                let (p, &w) = ctl.get(row + j).zip(wts.get(row + j))?;
                pts.push([
                    Interval::from_certified(p.x),
                    Interval::from_certified(p.y),
                    Interval::from_certified(p.z),
                ]);
                ws.push(Interval::from_certified(w));
            }
        }
        Some(Self {
            nv: pv + 1,
            pts,
            wts: ws,
            scale_u: pair_scales(spu)?,
            scale_v: pair_scales(spv)?,
        })
    }

    /// This cell block cut to `wu × wv`: along each axis whose window
    /// falls strictly inside the cell's span with positive width, the
    /// block is replaced by the Bézier block of the same polynomial
    /// piece over the window's part of the span, so its hulls enclose
    /// that part alone. `None` when no axis is cut (the window covers
    /// the span whole, or meets it in at most a point), or when a read
    /// leaves the window, which its invariants rule out: the whole block
    /// is the caller's answer either way, and a sound one.
    ///
    /// The cut is blossoming, `b_k = f(a, …, a, b, …, b)` with `k` of the
    /// `b`s, run by de Boor's recurrence in certification arithmetic on
    /// the homogeneous net `(w·(P − c), w)`, and the Cartesian block is
    /// read back as the quotient. `c` is the block's first point's lower
    /// bracket end, translating the net so the products are formed on
    /// differences; the cut block's points are in that frame.
    ///
    /// The cut block's boxes are not always narrower than the whole
    /// block's. The recurrence's rounding can outgrow what the cut saves:
    /// on a net constant along the cut whose `P − c` does not round
    /// exactly, `d₁ − d₀` of two equal non-point intervals is a
    /// symmetric interval rather than zero, and on a window a few ulps
    /// wide the `degree / (b − a)` scale amplifies every such width. So
    /// the caller meets the two ([`NurbsBoxes::deriv_box`]).
    fn cut<T: CertifiedBounds>(
        &self,
        win: SurfaceWindow<'_, T>,
        wu: (f64, f64),
        wv: (f64, f64),
    ) -> Option<Self> {
        let (cut_u, cut_v) = (inside(win.span_u(), wu), inside(win.span_v(), wv));
        if cut_u.is_none() && cut_v.is_none() {
            return None;
        }
        let nv = self.nv;
        let nu = self.pts.len() / nv;
        let (mut scale_u, mut scale_v) = (self.scale_u.clone(), self.scale_v.clone());
        let origin = self.pts.first()?.map(Interval::lo);
        // Four channels per control point: `w·(P − c)` and `w`.
        let mut h: Vec<[Interval; 4]> = self
            .pts
            .iter()
            .zip(&self.wts)
            .map(|(p, &w)| {
                let at = |k: usize| w * (p[k] - Interval::point(origin[k]));
                [at(0), at(1), at(2), w]
            })
            .collect();
        if let Some(cut) = cut_u {
            for j in 0..nv {
                let line: Vec<[Interval; 4]> = (0..nu).map(|i| h[i * nv + j]).collect();
                for (i, c) in bezier_on(&line, win.span_u(), cut)?.into_iter().enumerate() {
                    h[i * nv + j] = c;
                }
            }
            scale_u = bezier_scales(win.span_u().degree(), cut);
        }
        if let Some(cut) = cut_v {
            for i in 0..nu {
                let line = &h[i * nv..(i + 1) * nv];
                let cut_line = bezier_on(line, win.span_v(), cut)?;
                h[i * nv..(i + 1) * nv].copy_from_slice(&cut_line);
            }
            scale_v = bezier_scales(win.span_v().degree(), cut);
        }
        Some(Self {
            nv,
            pts: h
                .iter()
                .map(|c| [c[0] / c[3], c[1] / c[3], c[2] / c[3]])
                .collect(),
            wts: h.iter().map(|c| c[3]).collect(),
            scale_u,
            scale_v,
        })
    }

    /// The box for `S_d` over the block's region: the quotient numerator
    /// over the block's own point box, divided by its weight hull. Both
    /// are read in the block's own frame, the only frame a cut block's
    /// points are good for.
    fn derivative_box(&self, along_u: bool) -> Box3 {
        let (num, w) = self.quotient_numerator(along_u, self.point_box());
        Box3 {
            x: num.x / w,
            y: num.y / w,
            z: num.z / w,
        }
    }

    /// The hull of the block's Cartesian points: a box for `S` over the
    /// block's region (positive weights make `S` a convex combination of
    /// them, the rational hull property), in the block's frame. That is
    /// the absolute frame for [`CellNet::of_cell`]; for a [`CellNet::cut`]
    /// block it is a box for `S − c`, good only as the `S` of that
    /// block's own pair terms.
    fn point_box(&self) -> Box3 {
        let mut out: Option<Box3> = None;
        for p in &self.pts {
            let b = Box3 {
                x: p[0],
                y: p[1],
                z: p[2],
            };
            out = Some(match out {
                None => b,
                Some(acc) => acc.hull(b),
            });
        }
        out.unwrap_or_else(refused_box)
    }

    /// The quotient rule's numerator `A_d − S·w_d` and the weight hull
    /// over the block, `(numerator hull, w hull)`, with `S` ranging over
    /// `sbox`, which is in the block's frame ([`CellNet::point_box`]):
    /// the hull of every [`PairTerm`] at `sbox`.
    fn quotient_numerator(&self, along_u: bool, sbox: Box3) -> (Box3, Interval) {
        let Some(terms) = self.pair_terms(along_u) else {
            return (refused_box(), Interval::refused());
        };
        let num = terms
            .iter()
            .map(|t| {
                let [x, y, z] = t.at([sbox.x, sbox.y, sbox.z]);
                Box3 { x, y, z }
            })
            .reduce(Box3::hull);
        (
            num.unwrap_or_else(refused_box),
            self.weight_hull().unwrap_or_else(Interval::refused),
        )
    }

    /// The numerator's terms, one per adjacent pair of the net in
    /// direction `d`. `None` when a read leaves the net, which the
    /// block's own construction rules out.
    ///
    /// The numerator is a convex combination, under the degree-`p−1`
    /// basis, of these terms with `S` the surface point itself, so its
    /// hull is theirs with `S` anywhere `S` can be. No term holds an
    /// absolute coordinate: every point enters as a difference with
    /// another, so a translation moves a reading by its rounding width
    /// alone, and a net constant along `d` (equal points, equal weights)
    /// reads an exact zero.
    fn pair_terms(&self, along_u: bool) -> Option<Vec<PairTerm>> {
        let nv = self.nv;
        let nu = self.pts.len() / nv;
        let mut pairs = Vec::new();
        if along_u {
            for (k, &s) in self.scale_u.iter().enumerate() {
                for j in 0..nv {
                    pairs.push((k * nv + j, (k + 1) * nv + j, s));
                }
            }
        } else {
            for i in 0..nu {
                for (k, &s) in self.scale_v.iter().enumerate() {
                    pairs.push((i * nv + k, i * nv + k + 1, s));
                }
            }
        }
        pairs
            .into_iter()
            .map(|(i0, i1, scale)| {
                let (p0, p1) = (*self.pts.get(i0)?, *self.pts.get(i1)?);
                let (w0, w1) = (*self.wts.get(i0)?, *self.wts.get(i1)?);
                Some(PairTerm {
                    scale,
                    lever: core::array::from_fn(|k| w1 * (p1[k] - p0[k])),
                    dw: w1 - w0,
                    p0,
                })
            })
            .collect()
    }

    /// The hull of the block's weights, which holds the weight function
    /// over the block's region; `None` for an empty block.
    fn weight_hull(&self) -> Option<Interval> {
        self.wts.iter().copied().reduce(Interval::hull)
    }

    /// An upper bound on `‖S_d‖` over the block's region, read from the
    /// norms of the quotient numerator's vector terms over the weight
    /// function's lower bound, rounded outward; `NaN` when the weight
    /// hull is not positive or a read refuses.
    ///
    /// Each [`PairTerm`] is affine in `S`, and `S` lies in the convex
    /// hull of the block's points, so a term's norm, being convex in
    /// `S`, is largest at one of those points: `max ‖term(P_j)‖` over
    /// the pairs and the points is the numerator's norm bound with no
    /// box in it. A rigid map carries every `term(P_j)` to its image, so
    /// the bound moves by its rounding width alone. It is never above
    /// the norm of the box [`CellNet::derivative_box`] reads, which
    /// holds every `term(P_j)`. Each vector is divided by the weight's
    /// lower bound before its norm is taken: [`norm_sup`] squares, and a
    /// subnormal weight's products would square to nothing a root can
    /// recover.
    fn derivative_norm_sup(&self, along_u: bool) -> f64 {
        let Some((terms, w)) = self.pair_terms(along_u).zip(self.weight_hull()) else {
            return f64::NAN;
        };
        if !(w.is_certified() && w.lo() > 0.0) {
            return f64::NAN;
        }
        let floor = Interval::point(w.lo());
        terms
            .iter()
            .flat_map(|t| {
                self.pts
                    .iter()
                    .map(move |&p| norm_sup(&t.at(p).map(|c| c / floor)))
            })
            .reduce(max_bound)
            .unwrap_or(f64::NAN)
    }

    /// The chart probe's two readings along the chart direction
    /// `e = (ex, ey)` over the block's region:
    /// `(n·(S_u·ex + S_v·ey), sup ‖S_u·ex + S_v·ey‖)`, the first an
    /// enclosure and the second an upper bound rounded outward (`NaN`
    /// when refused).
    ///
    /// `S_u·ex + S_v·ey` is `(ex·N_u + ey·N_v)/w`, and since each
    /// numerator's basis sums to one, `ex·N_u + ey·N_v` is a convex
    /// combination of `ex·a(S) + ey·b(S)` over every `u` pair term `a`
    /// and `v` pair term `b`. Each is affine in `S`, so `n·` of it is
    /// extreme, and its norm largest, at one of the block's points: both
    /// readings are read from the vectors `ex·a(P_j) + ey·b(P_j)` alone,
    /// and a rigid map that carries `n` with the wall moves them by
    /// their rounding width.
    fn transverse_readings(&self, n: [Interval; 3], ex: Interval, ey: Interval) -> (Interval, f64) {
        let (Some(tu), Some(tv), Some(w)) =
            (self.pair_terms(true), self.pair_terms(false), self.weight_hull())
        else {
            return (Interval::refused(), f64::NAN);
        };
        if !(w.is_certified() && w.lo() > 0.0) {
            return (Interval::refused(), f64::NAN);
        }
        // The norms divide first, as `derivative_norm_sup`'s do.
        let floor = Interval::point(w.lo());
        let mut along: Option<Interval> = None;
        let mut norm: Option<f64> = None;
        for &p in &self.pts {
            let av: Vec<[Interval; 3]> = tu.iter().map(|t| t.at(p).map(|c| c * ex)).collect();
            let bv: Vec<[Interval; 3]> = tv.iter().map(|t| t.at(p).map(|c| c * ey)).collect();
            // `n·` is linear, so its range over the pairs `(a, b)` is the
            // sum of its ranges over each side.
            let side = |vs: &[[Interval; 3]]| vs.iter().map(|&v| dot3(n, v)).reduce(Interval::hull);
            if let Some(d) = side(&av).zip(side(&bv)).map(|(a, b)| a + b) {
                along = Some(along.map_or(d, |acc| Interval::hull(acc, d)));
            }
            let per_w = |vs: Vec<[Interval; 3]>| -> Vec<[Interval; 3]> {
                vs.into_iter().map(|v| v.map(|c| c / floor)).collect()
            };
            let (av, bv) = (per_w(av), per_w(bv));
            for a in &av {
                for b in &bv {
                    let m = norm_sup(&core::array::from_fn(|k| a[k] + b[k]));
                    norm = Some(norm.map_or(m, |acc| max_bound(acc, m)));
                }
            }
        }
        (
            along.map_or_else(Interval::refused, |a| a / w),
            norm.unwrap_or(f64::NAN),
        )
    }
}

/// One adjacent pair's term of the quotient numerator `A_d − S·w_d`,
/// `k·(w₁·(P₁ − P₀) + (w₁ − w₀)·(P₀ − S))` as a function of `S`: the
/// scale `k`, the lever `w₁·(P₁ − P₀)`, the weight step `w₁ − w₀` and
/// `P₀`.
#[derive(Clone, Copy, Debug)]
struct PairTerm {
    scale: Interval,
    lever: [Interval; 3],
    dw: Interval,
    p0: [Interval; 3],
}

impl PairTerm {
    /// The term at `S = s`.
    fn at(&self, s: [Interval; 3]) -> [Interval; 3] {
        core::array::from_fn(|k| (self.lever[k] + self.dw * (self.p0[k] - s[k])) * self.scale)
    }
}

/// The derivative scale `degree / (t_{i+degree+1} − t_{i+1})` of each
/// adjacent pair in a span's window, the B-spline derivative's
/// coefficient factor (The NURBS Book Eq. 3.4).
fn pair_scales(span: Span<'_>) -> Option<Vec<Interval>> {
    let (p, t) = (span.degree(), span.knots().knots());
    span.first_derived_window()
        .map(|i| {
            let (&lo, &hi) = t.get(i + 1).zip(t.get(i + p + 1))?;
            let deg = Interval::point(p as f64);
            Some(deg / (Interval::point(hi) - Interval::point(lo)))
        })
        .collect()
}

/// The derivative scale of a Bézier block over `[a, b]`: `degree / (b − a)`
/// for every pair.
fn bezier_scales(degree: usize, (a, b): (f64, f64)) -> Vec<Interval> {
    let deg = Interval::point(degree as f64);
    vec![deg / (Interval::point(b) - Interval::point(a)); degree]
}

/// The part of `span`'s knot interval inside `(lo, hi)`, when it is a
/// proper part of positive width; `None` when the window covers the
/// span whole or meets it in at most a point.
fn inside(span: Span<'_>, (lo, hi): (f64, f64)) -> Option<(f64, f64)> {
    let t = span.knots().knots();
    let (&ta, &tb) = t.get(span.index()).zip(t.get(span.index() + 1))?;
    let (a, b) = (lo.max(ta), hi.min(tb));
    (a < b && (a > ta || b < tb)).then_some((a, b))
}

/// The Bézier coefficients over `[a, b]` of the polynomial piece that
/// `line` (the `degree + 1` coefficients of `span`'s window) defines on
/// that span: the blossom `f(a^{p−k}, b^k)` for each `k`, by de Boor's
/// recurrence with the `r`-th level at its own argument. `None` when the
/// line is not one window long or a knot read leaves the vector.
fn bezier_on(
    line: &[[Interval; 4]],
    span: Span<'_>,
    (a, b): (f64, f64),
) -> Option<Vec<[Interval; 4]>> {
    let (p, s, t) = (span.degree(), span.index(), span.knots().knots());
    if line.len() != p + 1 {
        return None;
    }
    (0..=p)
        .map(|k| {
            let mut d = line.to_vec();
            for r in 1..=p {
                let x = Interval::point(if r <= p - k { a } else { b });
                for i in (r..=p).rev() {
                    // `s ≥ p` (the span's invariant), so `s + i − p` and
                    // `s + i + 1 − r` are in the window's knot range.
                    let (&lo, &hi) = t.get(s + i - p).zip(t.get(s + i + 1 - r))?;
                    let lo = Interval::point(lo);
                    let alpha = (x - lo) / (Interval::point(hi) - lo);
                    let (d0, d1) = (d[i - 1], d[i]);
                    d[i] = core::array::from_fn(|c| d0[c] + alpha * (d1[c] - d0[c]));
                }
            }
            Some(d[p])
        })
        .collect()
}

/// The smaller of two `f64` bounds, `NaN` if either is: the meet of two
/// upper bounds on one quantity, which keeps a refused side refused.
fn min_bound(a: f64, b: f64) -> f64 {
    -max_bound(-a, -b)
}

fn refused_box() -> Box3 {
    Box3 {
        x: Interval::refused(),
        y: Interval::refused(),
        z: Interval::refused(),
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

    /// **A window with a NaN or inverted end names no region, and every
    /// box over it is refused.** Without the window door, `f64::clamp`
    /// passes a NaN end through and `span_range` lands it on the first
    /// span, so a NaN window reads the first span's control block as a
    /// certified box — a strict subset of this two-span patch's. The
    /// ordered window beside them certifies, so the rows are about
    /// the window and not the patch.
    #[test]
    fn a_nan_or_inverted_window_is_refused_rather_than_landed_on_the_first_span() {
        let s = multiplicity_2_patch();
        let boxes = NurbsBoxes::new(&s);
        let certified = |b: Box3| b.x.is_certified() && b.y.is_certified() && b.z.is_certified();
        let all = |(u0, u1, v0, v1): (f64, f64, f64, f64)| {
            [
                ("deriv_box u", boxes.deriv_box(u0, u1, v0, v1, true)),
                ("deriv_box v", boxes.deriv_box(u0, u1, v0, v1, false)),
                ("rect_box", boxes.rect_box(u0, u1, v0, v1)),
            ]
        };
        for (which, b) in all((0.2, 0.8, 0.0, 1.0)) {
            assert!(
                certified(b),
                "CONTROL: {which} over an ordered window refused"
            );
        }
        let nan = f64::NAN;
        for (name, w) in [
            ("NaN window", (nan, nan, nan, nan)),
            ("NaN u start", (nan, 0.8, 0.0, 1.0)),
            ("NaN v end", (0.2, 0.8, 0.0, nan)),
            ("inverted u", (0.8, 0.2, 0.0, 1.0)),
            ("inverted v", (0.2, 0.8, 1.0, 0.0)),
        ] {
            for (which, b) in all(w) {
                assert!(!certified(b), "{name}: {which} certified {b:?}");
            }
        }
    }

    /// An inverted window past a domain end clamps to an ordered point,
    /// so the door has to come before the clamp in every box.
    #[test]
    fn an_inverted_window_past_the_domain_is_refused_by_rect_box() {
        let s = multiplicity_2_patch();
        let b = NurbsBoxes::new(&s);
        let c = |x: Box3| x.x.is_certified() && x.y.is_certified() && x.z.is_certified();
        assert!(
            !c(b.rect_box(1.5, 1.2, 0.0, 1.0)),
            "rect_box clamps an inverted window to a point"
        );
        assert!(!c(b.rect_box(-0.2, -0.5, 0.0, 1.0)));
    }

    /// The derivative box SKIPS an empty span cell, and the skip never
    /// leaves its hull unseeded: a rectangle straddling the empty cell
    /// still reads a certified box in both directions.
    #[test]
    fn a_skipped_empty_span_cell_leaves_the_derivative_box_seeded() {
        let s = multiplicity_2_patch();
        let ku = s.knots_u();
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
        let (su0, su1) = boxes.cells(u0, u1, v0, v1).expect("an ordered window").u;
        assert!(
            (su0..=su1).contains(&3),
            "the rectangle must straddle the empty cell, got {su0}..={su1}"
        );
        for along_u in [true, false] {
            let d = boxes.deriv_box(u0, u1, v0, v1, along_u);
            assert!(
                d.x.is_certified() && d.y.is_certified() && d.z.is_certified(),
                "deriv box (along_u = {along_u}) refused across the empty cell"
            );
        }
    }

    /// A strongly rational cubic × linear wall (weights 1.8 and 0.7 on
    /// the inner columns), its net translated by `t` along `(1, 1, 1)`.
    fn rational_wall(t: f64) -> NurbsSurface<f64> {
        let ku =
            geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3)
                .expect("clamped cubic");
        let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1)
            .expect("clamped linear");
        let mut control = Vec::with_capacity(8);
        for (x, y) in [(0.0, 0.0), (0.35, 0.14), (0.70, 0.24), (1.05, 0.30)] {
            control.push(Point3::new(x + t, y + t, t));
            control.push(Point3::new(x + t, y + t, 0.8 + t));
        }
        let weights = vec![1.0, 1.0, 1.8, 1.8, 0.7, 0.7, 1.0, 1.0];
        NurbsSurface::new(ku, kv, control, weights).expect("valid wall")
    }

    /// A window strictly inside one span cell of every net the rows
    /// below build, in both directions: a cut below the span on each.
    const INSIDE_ONE_CELL: (f64, f64, f64, f64) = (0.41, 0.43, 0.62, 0.635);

    /// The translations the invariance rows read: the origin, a
    /// building's reach and the edge of a site.
    const TRANSLATIONS: [f64; 3] = [0.0, 100.0, 1.0e5];

    /// **A rational wall's chart speeds move under a translation by
    /// their rounding width alone, and every floor minted from them
    /// certifies or refuses as it does at the origin.** The derivative
    /// enclosure reads every control point as a difference with
    /// another, so the coordinates' magnitude enters only through the
    /// rounding of those differences: a few ulps of the translation,
    /// scaled by the weight ratio and the knot scale. Assembled in the
    /// absolute frame instead, this wall read about `t·|w_d|/w` too
    /// fast (1420 at 100 m, 1.4e6 at 1e5 m), and the 1e-12 m floor
    /// refused at 1e5 m on the width alone.
    #[test]
    fn a_translated_rational_wall_reads_the_origins_chart_speed_and_floors() {
        use super::super::exhaust::{FloorKind, SweepFloor, UvRect};

        let at0 = NurbsBoxes::new(&rational_wall(0.0))
            .chart_speeds()
            .expect("the origin wall mints");
        let root = UvRect {
            u: (0.0, 1.0),
            v: (0.0, 1.0),
        };
        let floors = |speed: SupSpeed<f64>| {
            [1e-16, 1e-15, 1e-13, 1e-12, 1e-9, 1e-3]
                .map(|m| SweepFloor::chart(root, m, speed, FloorKind::Accounting).is_ok())
        };
        let origin_floors = floors(at0.max());
        // Static witness, not a search: the scan straddles the floor's
        // boundary at the origin, so "identical" below compares both arms.
        assert!(
            origin_floors.contains(&true) && origin_floors.contains(&false),
            "the meter scan must reach both sides of the floor at the origin: {origin_floors:?}"
        );
        for t in TRANSLATIONS {
            let got = NurbsBoxes::new(&rational_wall(t))
                .chart_speeds()
                .unwrap_or_else(|e| panic!("at {t} m the wall refused its chart speed: {e:?}"));
            // The rounding width: 64 ulps of the coordinates' magnitude
            // per unit of `k·max|Δw|/w_min` (3·1.1/0.7 on this wall).
            let width = 64.0 * f64::EPSILON * (t + 1.05) * (3.0 * 1.1 / 0.7);
            for (axis, s, s0) in [("u", got.u, at0.u), ("v", got.v, at0.v)] {
                assert!(
                    (s.get() - s0.get()).abs() <= width,
                    "at {t} m the {axis} speed reads {} against the origin's {} \
                     (rounding width {width:e})",
                    s.get(),
                    s0.get()
                );
            }
            assert_eq!(
                floors(got.max()),
                origin_floors,
                "at {t} m a chart floor decides differently from the origin's"
            );
        }
    }

    /// The chart readings a rigid map must carry: both speeds, the pads
    /// and floor decisions minted from them, and the transversality
    /// margin over three windows of one chart direction against the
    /// plane normal `n`.
    #[derive(Debug, PartialEq)]
    struct ChartReadings {
        speeds: [f64; 2],
        pads: [f64; 2],
        floors: [bool; 6],
        margins: Vec<f64>,
    }

    /// The windows [`chart_readings`] reads the margin over: the whole
    /// domain, a sub-rectangle across span cells, and one cut inside a
    /// cell.
    const MARGIN_WINDOWS: [(f64, f64, f64, f64); 3] =
        [(0.0, 1.0, 0.0, 1.0), (0.0, 0.34, 0.2, 0.55), INSIDE_ONE_CELL];

    fn chart_readings(wall: &NurbsSurface<f64>, n: Vec3<f64>) -> ChartReadings {
        use super::super::exhaust::{FloorKind, SweepFloor, UvRect};
        let boxes = NurbsBoxes::new(wall);
        let speeds = boxes.chart_speeds().expect("the wall mints");
        let root = UvRect {
            u: (0.0, 1.0),
            v: (0.0, 1.0),
        };
        let (pu, pv) = speeds.pad(1e-3);
        let n = [n.x, n.y, n.z].map(Interval::point);
        let (tx, ty) = (1.0_f64, 0.3_f64);
        let tangent = (tx, ty, tx.hypot(ty));
        let margins = MARGIN_WINDOWS
            .into_iter()
            .map(|rect| {
                chart_transverse_margin(&boxes, n, rect, tangent)
                    .expect("no window is degenerate")
                    .expect("every stretch is finite")
            })
            .collect();
        ChartReadings {
            speeds: [speeds.u.get(), speeds.v.get()],
            pads: [pu, pv],
            floors: [1e-16, 1e-15, 1e-13, 1e-12, 1e-9, 1e-3].map(|m| {
                SweepFloor::chart(root, m, speeds.max(), FloorKind::Accounting).is_ok()
            }),
            margins,
        }
    }

    /// **A rigid map moves a wall's chart readings by their rounding
    /// width alone, and every floor and tube decision minted from them
    /// stands as it does seated.** Each reading is a norm or a dot with
    /// the plane normal of a vector built from differences of control
    /// points, and a rigid map carries each such vector to its image, so
    /// only the rounding of the image's coordinates enters: a few hundred
    /// ulps of the coordinates' reach, relative, scaled up on a window by
    /// the cut's `1 / width` (a cut net's differences are divided by the
    /// window's width, and its coordinates round at the reach). A norm
    /// read off a
    /// per-coordinate box would move by up to √3 instead (5.88 to 8.20
    /// over 32 maps of the 1.8 / 0.7 wall's `u` speed).
    #[test]
    fn a_rigidly_mapped_wall_reads_the_seated_chart_speeds_floors_and_tube() {
        use geom_core::Affine3;
        use test_utils::fuzz;
        let mut rng = fuzz::start("enclose::rigid_map_chart_readings");
        let mut walls = vec![
            ("rational wall", rational_wall(0.0)),
            ("two-span patch", multiplicity_2_patch()),
        ];
        walls.extend(varied_nets());
        let n0 = Vec3::new(0.2, -0.5, 0.84).normalize();
        let seated: Vec<ChartReadings> = walls.iter().map(|(_, w)| chart_readings(w, n0)).collect();
        // Static witness, not a search: the margin rows compare a
        // nonzero reading on some seated wall.
        assert!(
            seated.iter().any(|r| r.margins.iter().any(|&m| m > 0.0)),
            "some seated margin must be zero-free: {seated:?}"
        );
        let close = |a: f64, b: f64, reach: f64, scale: f64| {
            (a - b).abs() <= 256.0 * f64::EPSILON * (1.0 + reach) * scale * a.abs().max(b.abs())
        };
        let window_scale = MARGIN_WINDOWS.map(|(u0, u1, v0, v1)| 1.0 / (u1 - u0).min(v1 - v0));
        for _ in 0..fuzz::scaled(32) {
            let axis = Vec3::new(rng.range(-1.0, 1.0), rng.range(-1.0, 1.0), rng.range(-1.0, 1.0));
            let pivot = Point3::new(rng.range(-10.0, 10.0), rng.range(-10.0, 10.0), rng.range(-10.0, 10.0));
            let angle = rng.range(0.0, core::f64::consts::TAU);
            let map = Affine3::rotation_about_axis(pivot, axis.normalize(), angle);
            let reach = 2.0 * (pivot - Point3::origin()).norm() + 4.0;
            for ((name, wall), at) in walls.iter().zip(&seated) {
                let image = chart_readings(&wall.map_points(|p| map.transform_point(p)), map.linear * n0);
                let pairs = at
                    .speeds
                    .iter()
                    .zip(&image.speeds)
                    .chain(at.pads.iter().zip(&image.pads))
                    .map(|(&a, &b)| (a, b, 1.0))
                    .chain(
                        at.margins
                            .iter()
                            .zip(&image.margins)
                            .zip(window_scale)
                            .map(|((&a, &b), k)| (a, b, k)),
                    );
                for (a, b, scale) in pairs {
                    assert!(
                        close(a, b, reach, scale),
                        "{name}: a rigid map about {axis:?} through {pivot:?} by {angle} moved \
                         a chart reading {a} to {b}: seated {at:?}, mapped {image:?} — {}",
                        fuzz::replay()
                    );
                }
                assert_eq!(
                    image.floors,
                    at.floors,
                    "{name}: a chart floor decides differently under the map about {axis:?} \
                     by {angle} — {}",
                    fuzz::replay()
                );
            }
        }
    }

    /// A net of degree `pu × pv` over the given clamped knots: a curved
    /// sheet, with weights `1 + spread·((3i + 2j) mod 5 − 2)/2`, so
    /// `spread` near 0 is nearly polynomial and larger is strongly
    /// rational.
    fn varied_net(
        pu: usize,
        ku: Vec<f64>,
        pv: usize,
        kv: Vec<f64>,
        spread: f64,
    ) -> NurbsSurface<f64> {
        let ku = geom_core::spline::KnotVector::clamped(ku, pu).expect("clamped u");
        let kv = geom_core::spline::KnotVector::clamped(kv, pv).expect("clamped v");
        let (nu, nv) = (ku.control_count(), kv.control_count());
        let mut control = Vec::with_capacity(nu * nv);
        let mut weights = Vec::with_capacity(nu * nv);
        for iu in 0..nu {
            for iv in 0..nv {
                let (a, b) = (iu as f64, iv as f64);
                control.push(Point3::new(
                    0.4 * a + 0.05 * b * b,
                    0.5 * b + 0.1 * a * a,
                    0.2 * ((iu * 7 + iv * 3) % 5) as f64 - 0.3 * a,
                ));
                weights.push(1.0 + spread * (((3 * iu + 2 * iv) % 5) as f64 - 2.0) / 2.0);
            }
        }
        NurbsSurface::new(ku, kv, control, weights).expect("valid net")
    }

    /// The nets the dominance rows read beyond the rational wall and
    /// the two-span patch: near-polynomial weights, non-uniform knots,
    /// and higher degree, where the quotient's own slack is small.
    fn varied_nets() -> Vec<(&'static str, NurbsSurface<f64>)> {
        let cubic_nonuniform = vec![0.0, 0.0, 0.0, 0.0, 0.3, 0.45, 1.0, 1.0, 1.0, 1.0];
        let quadratic_nonuniform = vec![0.0, 0.0, 0.0, 0.6, 1.0, 1.0, 1.0];
        let quartic = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.35, 1.0, 1.0, 1.0, 1.0, 1.0];
        let linear = vec![0.0, 0.0, 1.0, 1.0];
        vec![
            (
                "cubic × quadratic, non-uniform, weights within 2% of 1",
                varied_net(
                    3,
                    cubic_nonuniform.clone(),
                    2,
                    quadratic_nonuniform.clone(),
                    0.02,
                ),
            ),
            (
                "cubic × quadratic, non-uniform, weights 0.7 to 1.3",
                varied_net(3, cubic_nonuniform, 2, quadratic_nonuniform, 0.3),
            ),
            (
                "quartic × linear, weights within 10% of 1",
                varied_net(4, quartic.clone(), 1, linear.clone(), 0.1),
            ),
            (
                "quartic × linear, weights 0.4 to 1.6",
                varied_net(4, quartic, 1, linear, 0.6),
            ),
        ]
    }

    /// **The quotient numerator box holds the true numerator, with no
    /// quotient slack in between.** `S_d = (A_d − S·w_d)/w`, so the
    /// numerator at a parameter is `S_d·w`, read here from `ders` and
    /// from the weight function evaluated as a polynomial net. Each
    /// span cell's numerator box, assembled over that cell's own point
    /// box, must contain it at a dense grid of the cell's parameters
    /// (ends included, where the numerator is a single pair's term),
    /// and so must the box assembled with `S` pinned to the sample's
    /// own point. The slack is the f64 rounding of `ders` and of the
    /// product.
    #[test]
    fn the_quotient_numerator_box_holds_the_sampled_numerator() {
        let mut nets = varied_nets();
        nets.push(("rational wall", rational_wall(0.0)));
        nets.push(("two-span patch", multiplicity_2_patch()));
        let n = 16;
        for (name, s) in &nets {
            // The weight function as a polynomial net: `x` carries `w`.
            let wnet: Vec<Point3<f64>> = s
                .weights()
                .iter()
                .map(|w| Point3::new(*w, 0.0, 0.0))
                .collect();
            let wsurf = NurbsSurface::new(
                s.knots_u().clone(),
                s.knots_v().clone(),
                wnet,
                vec![1.0; s.weights().len()],
            )
            .expect("the weight net");
            let (ku, kv) = (s.knots_u(), s.knots_v());
            for su in ku.first_span()..=ku.last_span() {
                for sv in kv.first_span()..=kv.last_span() {
                    let Some(win) = s.window(su, sv) else {
                        continue;
                    };
                    let (ua, ub) = (ku.knots()[su], ku.knots()[su + 1]);
                    let (va, vb) = (kv.knots()[sv], kv.knots()[sv + 1]);
                    let net = CellNet::of_cell(win).expect("the cell's block");
                    for along_u in [true, false] {
                        let (num, _) = net.quotient_numerator(along_u, net.point_box());
                        // A cell's far end belongs to the next span (`ders`
                        // reads the span starting there), so it is sampled
                        // only where it is the domain's end.
                        let iu_end = if ub == ku.domain().1 { n } else { n - 1 };
                        let jv_end = if vb == kv.domain().1 { n } else { n - 1 };
                        for i in 0..=iu_end {
                            for j in 0..=jv_end {
                                let u = ua + (ub - ua) * f64::from(i) / f64::from(n);
                                let v = va + (vb - va) * f64::from(j) / f64::from(n);
                                let jet = s.ders(u, v);
                                let w = wsurf.eval(u, v).x;
                                let d = if along_u { jet.du } else { jet.dv };
                                // The same form with `S` pinned to this
                                // parameter's own point: no point-box slack
                                // is left for a wrong pair term to hide in.
                                let (thin, _) = net.quotient_numerator(
                                    along_u,
                                    Box3::between(jet.point, jet.point),
                                );
                                for (which, b) in [("cell box", num), ("S pinned", thin)] {
                                    for (c, x) in [(b.x, d.x * w), (b.y, d.y * w), (b.z, d.z * w)] {
                                        let slack = 32.0 * f64::EPSILON * x.abs().max(1.0);
                                        assert!(
                                            c.lo() - slack <= x && x <= c.hi() + slack,
                                            "{name}: numerator ({which}, along_u = {along_u}) \
                                             {x} at ({u}, {v}) outside [{}, {}] on cell \
                                             ({su}, {sv})",
                                            c.lo(),
                                            c.hi()
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// **The derivative box dominates the true derivative, sampled
    /// densely**, over the whole domain and over sub-rectangles, on the
    /// rational wall and the two-span patch at every translation, and on
    /// [`varied_nets`]: each
    /// sampled component lies in the box, and the chart speed is at
    /// least every sampled speed. The true derivative is evaluated on
    /// the origin wall, so the samples carry no rounding of the far
    /// coordinates. The translated net is not exactly the origin's
    /// moved, though: each control point rounds to an ulp of `t` as it
    /// is built, and a derivative box tight at a cell corner (one cut
    /// below its span) reads that rounding times the knot scale and
    /// the weight ratio. So the slack is `ders`' own f64 rounding at
    /// the origin plus `64·ε_mach·t`.
    #[test]
    fn the_derivative_box_dominates_the_dense_sampled_true_derivative() {
        let patch = multiplicity_2_patch();
        let mut cases: Vec<(String, NurbsSurface<f64>, NurbsSurface<f64>, f64)> = TRANSLATIONS
            .iter()
            .map(|&t| {
                (
                    format!("rational wall at {t} m"),
                    rational_wall(t),
                    rational_wall(0.0),
                    t,
                )
            })
            .collect();
        for t in TRANSLATIONS {
            let shifted = patch.map_points(|p| Point3::new(p.x + t, p.y + t, p.z + t));
            cases.push((
                format!("two-span patch at {t} m"),
                shifted,
                patch.clone(),
                t,
            ));
        }
        for (name, net) in varied_nets() {
            cases.push((name.to_string(), net.clone(), net, 0.0));
        }
        let n = 24;
        for (name, wall, origin, t) in &cases {
            let slack = |x: f64| 8.0 * f64::EPSILON * x.abs().max(1.0) + 64.0 * f64::EPSILON * t;
            let boxes = NurbsBoxes::new(wall);
            let speeds = boxes.chart_speeds().expect("mints");
            // The whole domain, a narrow window inside one span cell,
            // then a 3×3 grid of sub-rectangles.
            let mut rects = vec![(0.0, 1.0, 0.0, 1.0), INSIDE_ONE_CELL];
            for i in 0..3 {
                for j in 0..3 {
                    let (a, b) = (f64::from(i) / 3.0, f64::from(j) / 3.0);
                    rects.push((a, a + 1.0 / 3.0, b, b + 1.0 / 3.0));
                }
            }
            for (u0, u1, v0, v1) in rects {
                let du = boxes.deriv_box(u0, u1, v0, v1, true);
                let dv = boxes.deriv_box(u0, u1, v0, v1, false);
                for i in 0..=n {
                    for j in 0..=n {
                        let u = u0 + (u1 - u0) * f64::from(i) / f64::from(n);
                        let v = v0 + (v1 - v0) * f64::from(j) / f64::from(n);
                        let jet = origin.ders(u, v);
                        for (which, b, d, s) in
                            [("u", du, jet.du, speeds.u), ("v", dv, jet.dv, speeds.v)]
                        {
                            for (c, x) in [(b.x, d.x), (b.y, d.y), (b.z, d.z)] {
                                assert!(
                                    c.lo() - slack(x) <= x && x <= c.hi() + slack(x),
                                    "{name}: S_{which}({u}, {v}) component {x} outside \
                                     [{}, {}] over [{u0}, {u1}]×[{v0}, {v1}]",
                                    c.lo(),
                                    c.hi()
                                );
                            }
                            let speed = d.norm();
                            assert!(
                                speed <= s.get() + slack(speed),
                                "{name}: |S_{which}({u}, {v})| = {speed} above the chart \
                                 speed {}",
                                s.get()
                            );
                        }
                    }
                }
            }
        }
    }

    /// **The derivative box shrinks with its window below a span cell.**
    /// The dome `x = s, z = t, y = −4d·s(1−s)·t(1−t)` is one quadratic
    /// Bézier patch, so every window touches the same single cell, whose
    /// net puts `S_u.y` in `[−2d, 2d]`. Over a window cut to
    /// `[0.41, 0.43] × [0.62, 0.635]`, `S_u.y = −4d(1−2s)·t(1−t)` stays in
    /// about `[−0.17d, −0.13d]`, and `S_v.y = −4d·s(1−s)(1−2t)` in about
    /// `[0.23d, 0.25d]`: the box must hold every sampled value and keep
    /// both zero-free, which a box read off the whole cell cannot.
    #[test]
    fn the_derivative_box_shrinks_with_its_window_below_a_span_cell() {
        let d = 1.5;
        let k = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2)
            .expect("clamped quadratic");
        let mut control = Vec::with_capacity(9);
        for i in 0..3u8 {
            for j in 0..3u8 {
                let y = if i == 1 && j == 1 { -d } else { 0.0 };
                control.push(Point3::new(f64::from(i) / 2.0, y, f64::from(j) / 2.0));
            }
        }
        let dome = NurbsSurface::new(k.clone(), k, control, vec![1.0; 9]).expect("the dome");
        let boxes = NurbsBoxes::new(&dome);
        let (u0, u1, v0, v1) = INSIDE_ONE_CELL;
        let du = boxes.deriv_box(u0, u1, v0, v1, true);
        let dv = boxes.deriv_box(u0, u1, v0, v1, false);
        assert!(
            du.y.hi() < 0.0 && dv.y.lo() > 0.0,
            "S_u.y in [{}, {}] and S_v.y in [{}, {}] must be zero-free over the window",
            du.y.lo(),
            du.y.hi(),
            dv.y.lo(),
            dv.y.hi()
        );
        let n = 8;
        for i in 0..=n {
            for j in 0..=n {
                let u = u0 + (u1 - u0) * f64::from(i) / f64::from(n);
                let v = v0 + (v1 - v0) * f64::from(j) / f64::from(n);
                let jet = dome.ders(u, v);
                for (which, b, x) in [("S_u.y", du.y, jet.du.y), ("S_v.y", dv.y, jet.dv.y)] {
                    assert!(
                        b.contains(x),
                        "{which}({u}, {v}) = {x} outside [{}, {}]",
                        b.lo(),
                        b.hi()
                    );
                }
            }
        }
    }

    /// **The derivative box over a window is never wider than the whole
    /// cells' box.** Random nets (degree 1 to 3 per axis, 1 to 3 spans,
    /// weights 0.5 to 2, every other one constant along `v`) and random
    /// windows from a few ulps to the whole domain, some placed far from
    /// the origin: on every axis the box `deriv_box` reads must lie
    /// inside the one `cell_deriv_box` reads over the same rectangle. The
    /// cut alone is not: its recurrence rounds, and a window a few ulps
    /// wide divides that rounding by its width.
    #[test]
    fn the_derivative_box_is_never_wider_than_the_whole_cells() {
        use test_utils::fuzz;
        let mut rng = fuzz::start("enclose::deriv_box_within_cell_deriv_box");
        let clamped = |rng: &mut fuzz::Rng, p: usize, spans: usize| {
            let mut k = vec![0.0; p + 1];
            let mut inner: Vec<f64> = (1..spans).map(|_| rng.range(0.05, 0.95)).collect();
            inner.sort_by(f64::total_cmp);
            k.extend(inner);
            k.extend(vec![1.0; p + 1]);
            geom_core::spline::KnotVector::clamped(k, p).expect("clamped")
        };
        for case in 0..fuzz::scaled(40) {
            let (pu, pv) = (1 + rng.below(3), 1 + rng.below(3));
            let (su, sv) = (1 + rng.below(3), 1 + rng.below(3));
            let (ku, kv) = (clamped(&mut rng, pu, su), clamped(&mut rng, pv, sv));
            let (nu, nv) = (ku.knots().len() - pu - 1, kv.knots().len() - pv - 1);
            let far = if case % 3 == 0 { 100.0 } else { 0.0 };
            let flat_v = case % 2 == 0;
            let mut control = Vec::with_capacity(nu * nv);
            let mut weights = Vec::with_capacity(nu * nv);
            for _ in 0..nu {
                let row = Point3::new(
                    far + rng.range(-1.0, 1.0),
                    far + rng.range(-1.0, 1.0),
                    far + rng.range(-1.0, 1.0),
                );
                let w = rng.range(0.5, 2.0);
                for _ in 0..nv {
                    let p = if flat_v {
                        row
                    } else {
                        Point3::new(
                            far + rng.range(-1.0, 1.0),
                            far + rng.range(-1.0, 1.0),
                            far + rng.range(-1.0, 1.0),
                        )
                    };
                    control.push(p);
                    weights.push(if flat_v { w } else { rng.range(0.5, 2.0) });
                }
            }
            let net = NurbsSurface::new(ku, kv, control, weights).expect("a valid net");
            let boxes = NurbsBoxes::new(&net);
            for _ in 0..8 {
                let side = |rng: &mut fuzz::Rng| {
                    let width = 10f64.powf(rng.range(-15.0, 0.0));
                    let lo = rng.range(0.0, 1.0 - width);
                    (lo, lo + width)
                };
                let ((u0, u1), (v0, v1)) = (side(&mut rng), side(&mut rng));
                for along_u in [true, false] {
                    let cut = boxes.deriv_box(u0, u1, v0, v1, along_u);
                    let whole = boxes.cell_deriv_box(u0, u1, v0, v1, along_u);
                    for (axis, c, w) in [
                        ("x", cut.x, whole.x),
                        ("y", cut.y, whole.y),
                        ("z", cut.z, whole.z),
                    ] {
                        assert!(
                            c.is_certified()
                                && w.is_certified()
                                && w.lo() <= c.lo()
                                && c.hi() <= w.hi(),
                            "case {case} (degrees {pu}×{pv}, flat along v: {flat_v}, at {far} m), \
                             along_u = {along_u}, axis {axis}: [{}, {}] not inside the whole \
                             cells' [{}, {}] over [{u0}, {u1}]×[{v0}, {v1}]; {}",
                            c.lo(),
                            c.hi(),
                            w.lo(),
                            w.hi(),
                            fuzz::replay()
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn implicit_enclosure_contains_the_residual_at_every_sampled_point() {
        let b = Box3 {
            x: Interval::from_bounds(0.1, 0.9),
            y: Interval::from_bounds(-0.4, 0.3),
            z: Interval::from_bounds(-0.2, 0.7),
        };
        for s in [sphere(), cylinder()] {
            let e = implicit_enclosure(&s, b);
            assert!(e.is_certified());
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
            for (i, v) in at.to_array().iter().enumerate() {
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
        assert!(m.is_certified());
        assert!(
            m.lo() > 0.0 || m.hi() < 0.0,
            "expected a zero-free margin, got [{}, {}]",
            m.lo(),
            m.hi()
        );
    }

    #[test]
    fn unsupported_kinds_refuse_rather_than_guess() {
        let torus = Surface::Torus {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            major_radius: 1.0,
            minor_radius: 0.2,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let b = Box3::around(Point3::new(1.0, 0.0, 0.0), 0.1);
        assert!(!implicit_enclosure(&torus, b).is_certified());
        assert!(!implicit_gradient_enclosure(&torus, b)[0].is_certified());
        assert!(!graph_margin(&torus, &sphere(), b, Vec3::new(1.0, 0.0, 0.0)).is_certified());
    }

    #[test]
    fn refused_boxes_are_never_disjoint_and_never_contained() {
        let good = Box3::around(Point3::new(0.0, 0.0, 0.0), 1.0);
        let bad = refused_box();
        assert!(!good.definitely_disjoint(bad));
        assert!(!bad.definitely_disjoint(good));
        assert!(!bad.contained_in(good));
        assert!(!good.contained_in(bad));
    }

    #[test]
    fn splitting_covers_the_parent() {
        let b = Box3 {
            x: Interval::from_bounds(0.0, 2.0),
            y: Interval::from_bounds(0.0, 1.0),
            z: Interval::from_bounds(0.0, 0.5),
        };
        let (l, r) = b.split();
        // Widest axis is x.
        assert!((l.x.hi() - 1.0).abs() < 1e-15 && (r.x.lo() - 1.0).abs() < 1e-15);
        assert!(l.width() < b.width());
    }

    /// **The two crossings agree with the door, whichever way it
    /// answers.** The bracket crossing destructures a refusal to NaI and
    /// `pad_interval` reads the bracket's upper end, so the door
    /// beginning to REFUSE a value it used to hand over as a NaN bracket
    /// takes a different branch through both of them. The `f64` lane is
    /// where that is reachable without a feature: NaN is its poison, and
    /// the door stops it here rather than leaving it to
    /// `Interval::from_bounds`.
    ///
    /// **An infinite radius is a different case and deliberately not
    /// poison** — ∞ is not `f64` poison (D4's Q1 residue), so the door
    /// hands it over and `[−∞, ∞]` is what a pad of unbounded radius
    /// honestly encloses. It is useless rather than wrong, and what
    /// stops it is its width, downstream. The row pins the boundary
    /// between the two so neither can drift into the other unnoticed.
    #[test]
    fn a_poisoned_f64_refuses_at_the_door_and_still_lands_on_a_refused_box() {
        let origin = Point3::new(0.0, 0.0, 0.0);
        let nan_pad = Box3::around(origin, f64::NAN);
        assert!(
            !nan_pad.x.is_certified() && !nan_pad.y.is_certified() && !nan_pad.z.is_certified(),
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
        assert!(!bad_centre.x.is_certified(), "a NaN centre built a box");
        assert!(
            bad_centre.y.is_certified(),
            "the healthy coordinates must still build: a refusal is per-axis here"
        );
        let good = Box3::around(origin, 0.5);
        assert!(
            good.x.is_certified() && good.x.contains(-0.5) && good.x.contains(0.5),
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
    /// [`Interval::from_certified`] is the only way an evaluation
    /// scalar enters certification arithmetic here, and it is the one place the
    /// operand's own verdict is read — an operand whose bracket is sound
    /// but whose computation left a domain arrives capped at certification's
    /// refusal, and nothing across the seam consults the evaluation
    /// scalar again.
    /// The rows sweep the operand across the domain boundary: the
    /// enclosure must refuse on exactly the side where the decoration
    /// degrades, which neither a laundering nor a uniformly-refusing
    /// implementation can satisfy.
    #[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
    mod decoration_seam {
        use geom_core::{Bounds, CertifiedEnclosure, Interval, Real};

        use super::{Box3, Point3, Surface, Vec3, implicit_enclosure};

        /// One named entry point from an evaluation scalar into certification arithmetic.
        type Crossing = (&'static str, fn(Interval) -> Interval);

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
                x: Interval::from_bounds(0.1, 0.9),
                y: Interval::from_bounds(-0.4, 0.3),
                z: Interval::from_bounds(-0.2, 0.7),
            }
        }

        /// A sphere whose centre carries `c` in x — the operand reaches
        /// `implicit_enclosure` through `subp`, the same crossing door
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
        /// laundering certifies the whole sweep, uniform refusal refuses
        /// the whole sweep, and the non-vacuity assertions reject both.
        #[test]
        fn every_ring_crossing_refuses_exactly_where_the_decoration_degrades() {
            for (name, cross) in crossings() {
                let (mut certified, mut refused) = (0, 0);
                for a in [-4.0, -1.0, -0.25, -1e-300, 0.0, 1e-300, 0.25, 1.0] {
                    let c = operand(a);
                    let e = cross(c);
                    assert_eq!(
                        !e.is_certified(),
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
                    !e.is_certified(),
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
        fn a_violated_radius_refuses_the_pad() {
            let c = Point3::new(h(0.0), h(0.0), h(0.0));
            let bad = Box3::around(c, operand(-1.0));
            assert!(!bad.x.is_certified() && !bad.y.is_certified() && !bad.z.is_certified());
            let good = Box3::around(c, operand(1.0));
            assert!(good.x.is_certified(), "the healthy half must still build");
        }
    }
}
