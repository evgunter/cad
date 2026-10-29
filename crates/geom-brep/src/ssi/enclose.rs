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
//! - [`graph_margin`] — `(∇f₁ × ∇f₂)·e` over a box. This number carries
//!   the graph half of the uniqueness-tube argument: if its enclosure
//!   excludes zero then, on every slice `e·x = const` meeting the box,
//!   the 2×2 system `(f₁, f₂)` has non-singular Jacobian, so by the
//!   implicit function theorem the solution set inside the box is a
//!   **graph over the `e` axis**: at most one solution on each slice,
//!   no branch, no loop. A second arc beside the first along `e` is a
//!   graph too; that it is absent is `super::one_arc`'s proof.
//!   Straddling zero escalates: either a genuine sliver (F6) or the
//!   enclosure's remaining slack.
//! - [`NurbsBoxes`] — the same three readings for a NURBS chart,
//!   assembled from control-net hulls: the rational surface's point box
//!   is the *Cartesian* control hull over a span cell (positive weights
//!   ⇒ convex combination), and the derivative box comes from the
//!   quotient bound `S_u = N_u/W²` (`geom_core::spline::derivative`),
//!   over each cell cut to the rectangle's part of it, all in
//!   certification arithmetic.
//!
//! # Soundness posture
//!
//! Every function is **conservative or refused**: a widened enclosure
//! costs a refusal, never a wrong answer, and any structural surprise
//! (unsupported kind, malformed net, zero-touching divisor) yields
//! [`Interval::refused`], which fails every downstream test. An
//! enclosure is narrowed only by meeting it with another enclosure of
//! the same quantity (the torus arm's mean-value and direct forms of
//! one `g`), and a branch on a value only ever refuses (the torus's
//! `A` not shown positive).
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

use core::cell::OnceCell;

use geom::{NurbsSurface, Surface, SurfaceWindow};
use geom_core::Bounds;
use geom_core::interval::certification::Certification;
use geom_core::interval::{cross, div_down, dot, max_bound, min_bound, norm_sq, norm_sup};
use geom_core::spline::algebra::certainly_positive;
use geom_core::spline::{
    AxisCut, CellAxis, CellBlock, CellFirstQuotients, IntervalPieces, KnotVector, Knots,
    ParamRange, RingRefinement, Span, TensorNet, basis::basis_funs_in, cell_first_quotients,
};
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

    /// The componentwise intersection where it has interior on every
    /// axis, `None` where it has none or a side is refused: the strict
    /// sibling of [`Box3::meet`], for a reader that needs a box rather
    /// than a reach.
    pub(crate) fn intersection(self, o: Self) -> Option<Self> {
        let side = |a: Interval, b: Interval| {
            if !(a.is_certified() && b.is_certified()) {
                return None;
            }
            let (lo, hi) = (a.lo().max(b.lo()), a.hi().min(b.hi()));
            (lo < hi).then(|| Interval::from_bounds(lo, hi))
        };
        Some(Self {
            x: side(self.x, o.x)?,
            y: side(self.y, o.y)?,
            z: side(self.z, o.z)?,
        })
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

/// The enclosure over `b` of each kind's implicit residual in metres.
///
/// On the plane, sphere and cylinder that is the linearized residual
/// itself ([`crate::implicit::implicit_residual`]), a ring expression.
/// On the torus it is the root-free `g = F/(4rA)` ([`Quartic`]): the
/// quartic `F = A² − 4R²ρ²` over `4r` times `A = |q|² + R² − r²`, which
/// shares the meters residual's zero set and sign and agrees with it to
/// first order at the surface. Its box enclosure is the mean-value form
/// about the box's centre, met with the direct one: the direct form
/// treats `A²` and `4R²ρ²` as independent and is about `2R/r` wider.
/// Refused where `A`'s enclosure is not positive (a spindle or horn
/// torus near its axis). The cone has no arm; [`Surface::Nurbs`] has no
/// implicit form at all.
pub(crate) fn implicit_enclosure<T: CertifiedBounds>(surface: &Surface<T>, b: Box3) -> Interval {
    let two = Interval::point(2.0);
    match *surface {
        Surface::Plane { origin, normal, .. } => dot(&subp(b, origin), &constv(normal)),
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
            let h = dot(&q, &a);
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
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => {
            let at = |b: Box3| Quartic::over(b, center, axis, major_radius, minor_radius);
            let c = b.center();
            let mid = Box3::between(c, c);
            let over = at(b);
            let grad = over.gradient();
            let d = [b.x - mid.x, b.y - mid.y, b.z - mid.z];
            (at(mid).residual() + dot(&grad, &d)).meet(over.residual())
        }
        // `Approx` with the no-enclosure group: the implicit forms this
        // module encloses do not exist for a spline stand-in.
        Surface::Cone { .. } | Surface::Nurbs(_) | Surface::Approx(_) => Interval::refused(),
    }
}

/// The enclosure over `b` of the gradient of [`implicit_enclosure`]'s
/// own function, which Krawczyk's test (`super::one_arc`) reads as its
/// Jacobian: [`crate::implicit::implicit_gradient`] on the plane, sphere
/// and cylinder, `∇g` on the torus ([`Quartic::gradient`]), unit on the
/// surface in every case. Same kind coverage as [`implicit_enclosure`].
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
            // The exact derivative of [`implicit_enclosure`]'s own form
            // `(|w|² − r²)/2r`, `w = q − â h`, `h = q·â`:
            // `(q − â h (2 − |â|²))/r`. An `f64` unit axis is not exactly
            // unit, and Krawczyk's uniqueness reads this as that form's
            // Jacobian, so `|â|²` is kept; it is one thin constant, so
            // the enclosure is as tight as the unit form's.
            let q = subp(b, origin);
            let a = constv(axis);
            let h = dot(&q, &a) * (Interval::from_bounds(2.0, 2.0) - norm_sq(&a));
            let r = Interval::from_certified(radius);
            [
                (q[0] - a[0] * h) / r,
                (q[1] - a[1] * h) / r,
                (q[2] - a[2] * h) / r,
            ]
        }
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => Quartic::over(b, center, axis, major_radius, minor_radius).gradient(),
        Surface::Cone { .. } | Surface::Nurbs(_) | Surface::Approx(_) => refused,
    }
}

/// The torus's quartic over a box, with the parts its gradient reads:
/// `q = p − c`, `h = q·â`, the radial vector `w = q − âh` formed
/// itself (the cylinder arm's reason), `A = |q|² + R² − r²` and
/// `F = A² − 4R²|w|²`.
///
/// `ρ² = |q − âh|²` reads the stored `f64` axis as unit, as limb 1's
/// `implicit_residual` does. Limb 2's composite
/// (`geom_core::spline::compose`'s torus arm) spells it
/// `|q|² − (q·a)²/|a|²`, the axis normalized exactly in the ring. The
/// two must differ: over a box, `|q|² − h²` subtracts two enclosures of
/// one `q` and adds twice the axial square to the width, while a
/// composite's coefficients are thin, so there the identity costs only
/// rounding. They agree
/// up to `(1 − |â|²)·h²`, a few ulps of `h²` for an axis normalized in
/// `f64`, as the cylinder's two spellings do.
struct Quartic {
    h: Interval,
    w: [Interval; 3],
    axis: [Interval; 3],
    a: Interval,
    f: Interval,
    big_sq: Interval,
    small: Interval,
}

impl Quartic {
    fn over<T: CertifiedBounds>(
        b: Box3,
        center: Point3<T>,
        axis: Vec3<T>,
        major: T,
        minor: T,
    ) -> Self {
        let q = subp(b, center);
        let axis = constv(axis);
        let h = dot(&q, &axis);
        let w = [q[0] - axis[0] * h, q[1] - axis[1] * h, q[2] - axis[2] * h];
        let big_sq = Interval::from_certified(major).sqr();
        let small = Interval::from_certified(minor);
        let a = norm_sq(&q) + (big_sq - small.sqr());
        let f = a.sqr() - Interval::point(4.0) * big_sq * norm_sq(&w);
        Self {
            h,
            w,
            axis,
            a,
            f,
            big_sq,
            small,
        }
    }

    /// `A`, refused unless its enclosure is positive: `g` and `∇g`
    /// divide by it.
    fn a_positive(&self) -> Interval {
        if self.a.is_certified() && self.a.lo() > 0.0 {
            self.a
        } else {
            Interval::refused()
        }
    }

    /// `g = F/(4rA)`.
    fn residual(&self) -> Interval {
        self.f / (Interval::point(4.0) * self.small * self.a_positive())
    }

    /// `∇g = (k·q − l·½∇|w|²)/r` with `k = 1 − F/(2A²)` and `l = 2R²/A`
    /// (`k·q` and `−l·w` alone are `∇F/(4rA)`), grouped by the radial
    /// vector and the axis: `((k − l)·w + â·h·(k + l·(1 − |â|²)))/r`,
    /// exact for an `f64` axis that is not exactly unit. Each term is
    /// one scalar enclosure times one vector: `k·q − l·w` read
    /// componentwise sums two `O(1)` widths into an `O(r)` difference,
    /// which over a box wide along the carrier straddles where the
    /// gradient does not.
    fn gradient(&self) -> [Interval; 3] {
        let a = self.a_positive();
        let one = Interval::point(1.0);
        let k = one - self.f / (Interval::point(2.0) * a.sqr());
        let l = Interval::point(2.0) * self.big_sq / a;
        let radial = k - l;
        let along = self.h * (k + l * (one - norm_sq(&self.axis)));
        [0, 1, 2].map(|i| (radial * self.w[i] + self.axis[i] * along) / self.small)
    }
}

/// Whether [`implicit_enclosure`] and [`implicit_gradient_enclosure`]
/// have an arm for `kind`. Off it both refuse every box, so a tube over
/// such an operand reads a refused margin in every box at every rung:
/// no verdict on the pair, whatever the carrier.
pub(crate) const fn has_implicit_enclosure(kind: geom::SurfaceKind) -> bool {
    use geom::SurfaceKind as K;
    matches!(kind, K::Plane | K::Sphere | K::Cylinder | K::Torus)
}

/// Whether the tube can enclose `surface` at all: its kind is in
/// [`has_implicit_enclosure`]'s roster and, for a torus, it is shown
/// inside its ring convention, since a spindle or horn torus's `A`
/// reaches zero and both enclosures refuse there.
pub(crate) fn tube_encloses<T: geom_core::Decide>(
    surface: &Surface<T>,
    band: geom_core::Band,
) -> bool {
    match *surface {
        Surface::Torus {
            major_radius,
            minor_radius,
            ..
        } => {
            crate::torus_convention::TorusRefusal::require(major_radius, minor_radius, band).is_ok()
        }
        _ => has_implicit_enclosure(surface.kind()),
    }
}

/// `(∇f₁ × ∇f₂)·e` over `b` — **the uniqueness-tube quantity** (module
/// docs). An enclosure excluding zero proves the solution set inside
/// `b` is a graph over the `e` axis: at most one solution on each
/// slice. One arc in `b` is `super::one_arc`'s proof, not this one.
pub(crate) fn graph_margin<T: CertifiedBounds>(
    s1: &Surface<T>,
    s2: &Surface<T>,
    b: Box3,
    e: Vec3<T>,
) -> Interval {
    let g1 = implicit_gradient_enclosure(s1, b);
    let g2 = implicit_gradient_enclosure(s2, b);
    dot(&cross(&g1, &g2), &constv(e))
}

/// The plane × NURBS chart probe's transversality margin over one
/// chart window `rect`: `∇φ = (n·S_u, n·S_v)` read
/// along the chart direction transverse to the pcurve's tangent, divided
/// by the chart's stretch along that direction. `n` is the plane normal
/// already crossed into certification arithmetic; `tangent` is
/// `(t.x, t.y, ‖t‖)`, the tangent's bracket tops and a positive finite
/// norm, which select the direction (structure, not a bound). Both
/// readings come from the derivative numerators' vector terms
/// ([`CellNet::transverse_readings`]), so a rigid map that carries `n`
/// with the wall moves the margin by its rounding width. `Ok(None)`
/// when the stretch has no finite bound; `NaN` where `∇φ`'s enclosure
/// is refused ([`tube_margin`]), which measured nothing.
///
/// # Errors
///
/// [`TubeDegeneracy::WallConstantAcrossLocus`] when the stretch is
/// zero: the chart is constant along e⊥ over this window, and a narrower
/// window (a smaller rung) lies inside it, so it cannot cure that.
pub(super) fn chart_transverse_margin<T: CertifiedBounds>(
    boxes: &NurbsBoxes<'_, T>,
    n: [Interval; 3],
    rect: UvWindow,
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
    Ok(Some(
        tube_margin(phi).map_or(f64::NAN, |m| div_down(m, stretch)),
    ))
}

/// A uniqueness tube's reading of one transversality enclosure: its
/// certified distance from zero ([`zero_free_lower_bound`]), or `None`
/// where the enclosure is refused. A refused enclosure measured
/// nothing, so its rung is not probed; read as `0` it would be a
/// sliver's margin.
pub(crate) fn tube_margin(i: Interval) -> Option<f64> {
    i.is_certified().then(|| zero_free_lower_bound(i))
}

/// The certified distance of an enclosure from zero: `0` when it
/// straddles or is refused, the vacuous lower bound. A reading that
/// must tell a refusal from a straddle asks [`tube_margin`].
/// (The mignitude. `offset_meters::mig` is the same arithmetic read
/// as a coefficient-hull assembly term rather than a decision; noted
/// at both sites.)
pub(crate) fn zero_free_lower_bound(i: Interval) -> f64 {
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

/// An enclosure in units of which `per` make one metre, crossed to
/// metres by one outward division by the exact `per`.
pub(super) fn in_meters(i: Interval, per: f64) -> Interval {
    i / Interval::point(per)
}

/// A certified upper bound on the magnitude of [`in_meters`]'s
/// enclosure, `NaN` where it is refused.
pub(super) fn mag_in_meters(i: Interval, per: f64) -> f64 {
    in_meters(i, per).mag()
}

/// The torus's `R² − r²`: the shift of its `A = |q|² + R² − r²` from
/// the squared distance to its centre, and a lower bound on `A`.
pub(super) fn torus_a_shift(major: f64, minor: f64) -> Interval {
    Interval::point(major).sqr() - Interval::point(minor).sqr()
}

/// An upper bound in meters on the torus residual `F/(2r·(A + 2Rρ))`
/// where its quartic `F = A² − 4R²ρ²` and its `A` are enclosed by `f`
/// and `a`: `sup|F|` over `2r` times a lower bound on `A + 2Rρ`, which
/// is `inf A` (floored by [`torus_a_shift`]) plus `2Rρ = √(A² − F) ≥
/// √(inf A² − sup F)`. On the surface `2Rρ = A`, so the bound is tight
/// to first order where `inf A` alone would be twice the residual.
/// `NaN` where `f` is refused or the floor is not positive.
pub(super) fn torus_meters_sup(f: Interval, a: Interval, major: f64, minor: f64) -> f64 {
    let floor = torus_a_shift(major, minor).lo();
    let inf = if a.is_certified() {
        a.lo().max(floor)
    } else {
        floor
    };
    if !(f.is_certified() && inf > 0.0) {
        return f64::NAN;
    }
    let inf = Interval::point(inf);
    let radicand = (inf.sqr() - Interval::point(f.hi())).lo();
    let two_big_rho = if radicand > 0.0 {
        Interval::point(radicand).sqrt().lo()
    } else {
        0.0
    };
    let lever = (inf + Interval::point(two_big_rho)).lo();
    (f / (Interval::point(2.0 * minor) * Interval::point(lever))).mag()
}

/// A lower bound in meters on the same residual's magnitude where `A`
/// is enclosed positive: `|F| / (2r·(2A + r²))`, since
/// `0 < A + 2Rρ ≤ 2A + r²` (`2Rρ ≤ R² + ρ² ≤ A + r²`). `0` where nothing
/// is certified, and where `A` may be zero or negative (a spindle torus
/// near its centre), since `A + 2Rρ` may then be negative and larger in
/// magnitude than `2A + r²`.
pub(super) fn torus_meters_floor(f: Interval, a: Interval, minor: f64) -> f64 {
    if !(a.is_certified() && a.lo() > 0.0) {
        return 0.0;
    }
    let r = Interval::point(minor);
    zero_free_lower_bound(f / (Interval::point(2.0) * r * (Interval::point(2.0) * a + r.sqr())))
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

/// A parameter window `u × v` on a chart: two [`ParamRange`]s, so it has
/// no NaN or inverted side. Every [`NurbsBoxes`] reading takes one. It
/// is minted before anything clamps it to the domain, because a clamp
/// turns an inverted window past a domain end into an ordered point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct UvWindow {
    pub(crate) u: ParamRange,
    pub(crate) v: ParamRange,
}

impl UvWindow {
    /// `[u0, u1] × [v0, v1]`, or `None` when either pair has a NaN end
    /// or is inverted: such a pair names no region.
    pub(crate) fn new(u0: f64, u1: f64, v0: f64, v1: f64) -> Option<Self> {
        Some(Self {
            u: ParamRange::new(u0, u1)?,
            v: ParamRange::new(v0, v1)?,
        })
    }

    /// The window two chart points span, in either order, or `None`
    /// when a coordinate is NaN ([`ParamRange::spanning`]).
    pub(crate) fn spanning(a: (f64, f64), b: (f64, f64)) -> Option<Self> {
        Some(Self {
            u: ParamRange::spanning(a.0, b.0)?,
            v: ParamRange::spanning(a.1, b.1)?,
        })
    }
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
/// is the hull of the quotient bound's coefficient ratios over each
/// cell's net cut to the rectangle ([`CellNet::cut`]), so it shrinks with the
/// rectangle below a span cell too.
///
/// The cells are the surface's own, or a [`CellGrid`]'s: the same
/// surface's net refined in certification arithmetic, whose finer cells
/// read tighter hulls. Evaluation ([`NurbsBoxes::rect_box`]'s midpoint)
/// and the boundary rows always read the surface itself.
#[derive(Debug)]
pub(crate) struct NurbsBoxes<'a, T: CertifiedBounds> {
    surface: &'a NurbsSurface<T>,
    grid: Option<&'a CellGrid>,
    /// Each span cell's whole net, row-major by `(su, sv)` over the cell
    /// knots' span indices, built on its first read: a function of the
    /// cell alone, which every rectangle touching the cell reads whole.
    nets: CellNets,
    /// The surface's knot values lifted to certification arithmetic,
    /// for [`Self::rect_box`]'s midpoint, on the first read.
    knots: OnceCell<(Knots<Interval>, Knots<Interval>)>,
    /// The same knot values as `f64`s where every one is a point, for
    /// the midpoint's double-word reading ([`super::compensated`]).
    point_knots: OnceCell<Option<(Vec<f64>, Vec<f64>)>>,
}

/// [`NurbsBoxes`]'s whole-cell nets, `v`-span count wide.
#[derive(Debug)]
struct CellNets {
    nv: usize,
    cells: Vec<OnceCell<Option<CellNet>>>,
}

impl CellNets {
    fn over((ku, kv): (&KnotVector, &KnotVector)) -> Self {
        let (nu, nv) = (ku.knots().len(), kv.knots().len());
        Self {
            nv,
            cells: (0..nu * nv).map(|_| OnceCell::new()).collect(),
        }
    }

    /// Cell `(su, sv)`'s net, built by `build` on the first read.
    fn get(
        &self,
        (su, sv): (usize, usize),
        build: impl FnOnce() -> Option<CellNet>,
    ) -> Option<&CellNet> {
        if sv >= self.nv {
            return None;
        }
        self.cells
            .get(su * self.nv + sv)?
            .get_or_init(|| {
                #[cfg(test)]
                tests::WHOLE_NETS.with(|n| n.set(n.get() + 1));
                build()
            })
            .as_ref()
    }

    /// How many cells' nets have been built.
    #[cfg(test)]
    fn built(&self) -> usize {
        self.cells.iter().filter(|c| c.get().is_some()).count()
    }
}

impl<'a, T: CertifiedBounds> NurbsBoxes<'a, T> {
    /// Wrap a surface, reading its own cells.
    pub(crate) fn new(surface: &'a NurbsSurface<T>) -> Self {
        Self {
            surface,
            grid: None,
            nets: CellNets::over((surface.knots_u_f64_interim(), surface.knots_v_f64_interim())),
            knots: OnceCell::new(),
            point_knots: OnceCell::new(),
        }
    }

    /// Wrap a surface, reading the cells of `grid`, a refinement of this
    /// surface's net ([`CellGrid::refined`]).
    pub(crate) fn on_grid(surface: &'a NurbsSurface<T>, grid: &'a CellGrid) -> Self {
        Self {
            surface,
            grid: Some(grid),
            nets: CellNets::over((&grid.ku, &grid.kv)),
            knots: OnceCell::new(),
            point_knots: OnceCell::new(),
        }
    }

    /// The `u` and `v` vectors the cells are spans of.
    fn cell_knots(&self) -> (&'a KnotVector, &'a KnotVector) {
        match self.grid {
            Some(g) => (&g.ku, &g.kv),
            None => (
                self.surface.knots_u_f64_interim(),
                self.surface.knots_v_f64_interim(),
            ),
        }
    }

    /// The control block of the cell at spans `(su, sv)`, with its two
    /// spans; `None` for an empty span (it covers no parameters) or a
    /// read that leaves the net.
    fn cell(&self, su: usize, sv: usize) -> Option<(&CellNet, (Span<'a>, Span<'a>))> {
        let (ku, kv) = self.cell_knots();
        let spans = (ku.span(su)?, kv.span(sv)?);
        let net = self.nets.get((su, sv), || match self.grid {
            Some(g) => CellNet::of_spans(spans, kv.control_count(), |k| {
                let (p, w, h) = g.entry(k)?;
                Some((p, w, Some(h)))
            }),
            None => CellNet::of_cell(self.surface.window(su, sv)?),
        })?;
        Some((net, spans))
    }

    /// The surface boxed.
    pub(crate) fn surface(&self) -> &'a NurbsSurface<T> {
        self.surface
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
        let domain = UvWindow {
            u: self.surface.knots_u_f64_interim().domain_range(),
            v: self.surface.knots_v_f64_interim().domain_range(),
        };
        let speed = |axis: ChartAxis| {
            let m = self.speed_sup(domain, axis == ChartAxis::U);
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

    /// The (span_u, span_v) cell range touched by the window, with the
    /// window as clamped.
    ///
    /// The rectangle is **clamped to the knot domains** first. Callers
    /// pad windows by a tube radius, which routinely pushes them past
    /// the clamped ends; a parameter outside the domain has no span,
    /// and letting that refuse the enclosure would make every branch
    /// that reaches a surface edge fail its own uniqueness tube. The
    /// clamp is sound because the objects being enclosed — a pcurve, a
    /// foot point — cannot leave the domain either.
    fn cells(&self, w: UvWindow) -> CellRange {
        let (ku, kv) = self.cell_knots();
        let clamped = self.clamped(w);
        let (cu, cv) = (clamped.u, clamped.v);
        // `span_range` answers in validated spans, but a cell RANGE is
        // what this returns: the interior of the rectangle is neither
        // end, so the two ends' proofs do not cover it. The callers
        // re-derive each cell's window with `NurbsSurface::window`,
        // which is also what skips the empty spans in between — hence
        // indices here, and the pair read back out.
        let (u_lo, u_hi) = ku.span_range(cu);
        let (v_lo, v_hi) = kv.span_range(cv);
        CellRange {
            u: (u_lo.index(), u_hi.index()),
            v: (v_lo.index(), v_hi.index()),
            clamped,
        }
    }

    /// The window clamped to the knot domains.
    fn clamped(&self, w: UvWindow) -> UvWindow {
        UvWindow {
            u: w.u
                .clamp_to(self.surface.knots_u_f64_interim().domain_range()),
            v: w.v
                .clamp_to(self.surface.knots_v_f64_interim().domain_range()),
        }
    }

    /// A certified box for `∂S/∂u` (or `∂S/∂v`) over the rectangle: per
    /// span cell, the hull of the quotient `S_d = N_d/W²`'s coefficient
    /// ratios ([`CellNet::derivative_box`]). Each cell is also cut to the
    /// part of the rectangle inside it ([`CellNet::cut`]) and the two
    /// boxes met, so the box shrinks with the rectangle below a span
    /// cell and is never wider than the whole cell's. Refused when a
    /// weight-power coefficient touches zero (interval arithmetic
    /// refuses the divisor) or the net is malformed. The boundary pass reads a
    /// directional partial off
    /// it; the chart readings take norms from the same cut
    /// ([`NurbsBoxes::speed_sup`]).
    pub(crate) fn deriv_box(&self, w: UvWindow, along_u: bool) -> Box3 {
        self.deriv_hull(w, along_u, true)
    }

    /// [`NurbsBoxes::deriv_box`] read off every touched cell's whole net,
    /// without the cut: the exhaustiveness sweep's derivative box, whose
    /// slack the sweep's seeding is pinned against
    /// (`work/ssiedge/the-chart-sweeps-first-order-box-reads-its-derivative-off-the-whole-span-cell.md`).
    pub(crate) fn cell_deriv_box(&self, w: UvWindow, along_u: bool) -> Box3 {
        self.deriv_hull(w, along_u, false)
    }

    /// [`NurbsBoxes::deriv_box`] over the CLOSED rectangle, each part read
    /// through the cells it has width in: a cell the rectangle meets only
    /// along an edge, in an axis where the rectangle has width, is not
    /// read, so a rectangle ending on a knot is read through its own
    /// span's polynomial up to and including that knot. `∂S` is that
    /// polynomial's derivative there (the one-sided derivative from
    /// inside the rectangle), which is what a continuity argument over
    /// the closed rectangle needs; the neighbouring span's derivative at
    /// a kink is not a value `∂S` takes on it. An axis of zero width
    /// keeps every cell it touches.
    pub(crate) fn deriv_box_closed(&self, w: UvWindow, along_u: bool) -> Box3 {
        let clamped = self.clamped(w);
        let (ku, kv) = self.cell_knots();
        let meets = |k: &KnotVector, span: Span<'_>, r: ParamRange| {
            let (lo, hi) = (r.start(), r.end());
            let t = k.knots();
            let ends = t.get(span.index()).zip(t.get(span.index() + 1));
            lo == hi || ends.is_some_and(|(&ta, &tb)| lo.max(ta) < hi.min(tb))
        };
        self.fold_cells_where(
            w,
            Some((along_u, !along_u)),
            |(su, sv)| meets(ku, su, clamped.u) && meets(kv, sv, clamped.v),
            |net| net.derivative_box(along_u),
            Box3::meet_enclosure,
            Box3::hull,
        )
        .unwrap_or_else(refused_box)
    }

    /// Whether the surface's first partials are continuous inside the
    /// window: no crease ([`Knots::crease_values`], an interior knot of
    /// multiplicity at least the degree; a spline, rational or not, is
    /// `C^(p−m)` across a knot of multiplicity `m`) may lie strictly
    /// inside it, on either axis.
    pub(crate) fn first_order_continuous_in(&self, w: UvWindow) -> bool {
        let smooth = |k: &Knots<T>, r: ParamRange| {
            k.crease_values().iter().all(|c| {
                c.certified_bracket()
                    .is_some_and(|(lo, hi)| hi <= r.start() || lo >= r.end())
            })
        };
        smooth(self.surface.knot_values_u(), w.u) && smooth(self.surface.knot_values_v(), w.v)
    }

    /// The derivative box over the rectangle, each touched cell's net cut
    /// to it or read whole.
    fn deriv_hull(&self, w: UvWindow, along_u: bool, cut: bool) -> Box3 {
        // Both boxes enclose `S_d` over the window's part of the cell,
        // so their meet does, and it is never wider than the whole
        // cell's: a zero the whole cell reads exactly stays exact.
        self.fold_cells(
            w,
            cut.then_some((along_u, !along_u)),
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
    /// [`NurbsBoxes::deriv_box`] reads between 1× and √3× it depending on
    /// how the field sits against the axes. Per net it is never above
    /// that box's norm in real arithmetic; the box meets the cut and
    /// whole readings per axis, though, and the norm of a per-axis meet
    /// can sit below both nets' norms, so over a cut window it is not
    /// bounded by the box's. `NaN` when refused.
    pub(crate) fn speed_sup(&self, w: UvWindow, along_u: bool) -> f64 {
        self.fold_cells(
            w,
            Some((along_u, !along_u)),
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
        w: UvWindow,
        n: [Interval; 3],
        ex: Interval,
        ey: Interval,
    ) -> (Interval, f64) {
        self.fold_cells(
            w,
            Some((true, true)),
            |net| net.transverse_readings(n, ex, ey),
            |(a, s), (b, t)| (a.meet(b), min_bound(s, t)),
            |(a, s), (b, t)| (Interval::hull(a, b), max_bound(s, t)),
        )
        .unwrap_or((Interval::refused(), f64::NAN))
    }

    /// `read` over every nonempty span cell the rectangle touches,
    /// `join`ed across cells. With `cut` (the axes `read` reads), each
    /// cell's net is also cut to the rectangle ([`CellNet::cut`]) and
    /// the two readings `meet`:
    /// both hold over the rectangle's part of the cell. `None` for a
    /// malformed net.
    fn fold_cells<R>(
        &self,
        w: UvWindow,
        cut: Option<(bool, bool)>,
        read: impl Fn(&CellNet) -> R,
        meet: impl Fn(R, R) -> R,
        join: impl Fn(R, R) -> R,
    ) -> Option<R> {
        self.fold_cells_where(w, cut, |_| true, read, meet, join)
    }

    /// [`NurbsBoxes::fold_cells`] over the touched cells whose spans
    /// `keep` admits; `None` when it admits none.
    fn fold_cells_where<R>(
        &self,
        w: UvWindow,
        cut: Option<(bool, bool)>,
        keep: impl Fn((Span<'a>, Span<'a>)) -> bool,
        read: impl Fn(&CellNet) -> R,
        meet: impl Fn(R, R) -> R,
        join: impl Fn(R, R) -> R,
    ) -> Option<R> {
        let range = self.cells(w);
        let mut out: Option<R> = None;
        for su in range.u.0..=range.u.1 {
            for sv in range.v.0..=range.v.1 {
                // An EMPTY span cell covers no parameters, so skipping
                // it is sound and strictly tighter. The corner cell is
                // always nonempty (`cells` locates its ends with
                // `span_at`), so the fold is never left unseeded.
                let (ku, kv) = self.cell_knots();
                if !(ku.span_is_nonempty(su) && kv.span_is_nonempty(sv)) {
                    continue;
                }
                let (whole, spans) = self.cell(su, sv)?;
                if !keep(spans) {
                    continue;
                }
                let r = read(whole);
                let r = match cut.and_then(|axes| whole.cut(spans, range.clamped, axes)) {
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
    pub(crate) fn rect_box(&self, w: UvWindow) -> Box3 {
        let w = self.clamped(w);
        let (u0, u1, v0, v1) = (w.u.start(), w.u.end(), w.v.start(), w.v.end());
        let um = 0.5 * (u0 + u1);
        let vm = 0.5 * (v0 + v1);
        // The midpoint evaluation is the only EVALUATION in this
        // module, and it is an ENCLOSURE on every lane: the window's
        // net and knots crossed at the seam ([`Interval::from_certified`])
        // and evaluated in certification arithmetic, so a widened
        // control net widens the box rather than being silently
        // collapsed, and an `f64` net's point is not read as a rounded
        // `f64` evaluation that may sit off the surface.
        // Clamped into a finite domain, so the midpoint is a number.
        let Some(win) = self.surface.window_at(um, vm) else {
            unreachable!("a clamped window's midpoint is a number")
        };
        let c = self.enclosed_point(win, um, vm);
        let du = self.cell_deriv_box(w, true);
        let dv = self.cell_deriv_box(w, false);
        // The window's offsets from its midpoint, each end's difference
        // rounded outward: `um` is itself rounded, and `±(u1 − u0)/2`
        // formed in `f64` can fall short of an end by an ulp, which
        // leaves `S` at the window's own corner outside its box.
        let offsets = |lo: f64, mid: f64, hi: f64| {
            let mid = Interval::point(mid);
            Interval::hull(Interval::point(lo) - mid, Interval::point(hi) - mid)
        };
        let ru = offsets(u0, um, u1);
        let rv = offsets(v0, vm, v1);
        Box3 {
            x: c.x + du.x * ru + dv.x * rv,
            y: c.y + du.y * ru + dv.y * rv,
            z: c.z + du.z * ru + dv.z * rv,
        }
    }

    /// An enclosure of `S(u, v)` in `win`: [`SurfaceWindow::eval_in_span`]'s
    /// sum, term for term in its order, over the window's net and knot
    /// values crossed into certification arithmetic. At `Interval` it
    /// is that evaluation's own bounds; at `f64` it encloses the stored
    /// net's exact point, which the lane's own evaluation only rounds
    /// to.
    fn enclosed_point(&self, win: SurfaceWindow<'_, T>, u: f64, v: f64) -> Point3<Interval> {
        let (ku, kv) = self.knots.get_or_init(|| {
            let lift = |x: T| Interval::from_certified(x);
            (
                self.surface.knot_values_u().lift(lift),
                self.surface.knot_values_v().lift(lift),
            )
        });
        let (Some(su), Some(sv)) = (ku.span(win.span_u().index()), kv.span(win.span_v().index()))
        else {
            unreachable!("the window's spans are spans of the surface's own knots")
        };
        let bu = basis_funs_in(su, Interval::point(u));
        let bv = basis_funs_in(sv, Interval::point(v));
        let (control, weights) = (self.surface.control(), self.surface.weights());
        // Read twice, about the origin and about the window's corner
        // control point `o` (a point), and met: both enclose the one
        // exact point. `S − o = Σ bw·(P − o) / Σ bw` in ℝ, and each
        // `P − o` is a difference of nearby numbers, so that sum's
        // rounding scales with the window's extent rather than with its
        // distance from the origin, at the cost of one rounding at `o`'s
        // scale when it is added back; about the origin there is no
        // such rounding, which is the tighter where `|S|` is small next
        // to `|o|`.
        let corner = control[win.row(0)];
        let o = [corner.x, corner.y, corner.z].map(|c| Interval::point(Bounds::lo(c)));
        let zero = Interval::point(0.0);
        let (mut at, mut about, mut w) = ([zero; 3], [zero; 3], zero);
        for (i, bui) in bu.iter().enumerate() {
            let row = win.row(i);
            for (j, bvj) in bv.iter().enumerate() {
                let idx = row + j;
                let cw = (*bui * *bvj) * Interval::from_certified(weights[idx]);
                let pt = control[idx];
                let p = [pt.x, pt.y, pt.z].map(Interval::from_certified);
                for c in 0..3 {
                    at[c] = at[c] + cw * p[c];
                    about[c] = about[c] + cw * (p[c] - o[c]);
                }
                w = w + cw;
            }
        }
        let [x, y, z] = core::array::from_fn(|c| (at[c] / w).meet(o[c] + about[c] / w));
        // Where every input is a point, the double-word reading too: a
        // few ulps of the point (or `u²` of its terms where they
        // cancel), where the interval sums are as wide as their terms'
        // rounding.
        match self.point_reading(win, (u, v)) {
            Some([dx, dy, dz]) => Point3::new(x.meet(dx), y.meet(dy), z.meet(dz)),
            None => Point3::new(x, y, z),
        }
    }

    /// [`super::compensated::nurbs_point`] on `win` at `(u, v)`, where
    /// the window's net and weights and the surface's knots are all
    /// points; `None` elsewhere, or where its bound is not trusted.
    fn point_reading(
        &self,
        win: SurfaceWindow<'_, T>,
        (u, v): (f64, f64),
    ) -> Option<[Interval; 3]> {
        let point = |x: T| (Bounds::lo(x) == Bounds::hi(x)).then(|| Bounds::lo(x));
        let (ku, kv) = self
            .point_knots
            .get_or_init(|| {
                let all = |k: &Knots<T>| {
                    k.values()
                        .iter()
                        .map(|&x| point(x))
                        .collect::<Option<Vec<f64>>>()
                };
                Some((
                    all(self.surface.knot_values_u())?,
                    all(self.surface.knot_values_v())?,
                ))
            })
            .as_ref()?;
        let (su, sv) = (win.span_u(), win.span_v());
        let (control, weights) = (self.surface.control(), self.surface.weights());
        let window = (0..=su.degree())
            .flat_map(|i| (0..=sv.degree()).map(move |j| win.row(i) + j))
            .map(|idx| {
                let p = control[idx];
                Some((
                    [point(p.x)?, point(p.y)?, point(p.z)?],
                    point(weights[idx])?,
                ))
            })
            .collect::<Option<Vec<_>>>()?;
        super::compensated::nurbs_point(
            &super::compensated::PointCell {
                knots: (ku, kv),
                spans: (su.index(), sv.index()),
                degrees: (su.degree(), sv.degree()),
                window: &window,
            },
            (u, v),
        )
    }
}

/// The span cells a rectangle touches, and the rectangle clamped to
/// the knot domains ([`NurbsBoxes::cells`]).
struct CellRange {
    u: (usize, usize),
    v: (usize, usize),
    clamped: UvWindow,
}

/// A wall's control net refined in certification arithmetic, for
/// [`NurbsBoxes::on_grid`]: the knot pair it is a net over, and each
/// entry's Cartesian point and weight, row-major as
/// `NurbsSurface::control()` is.
///
/// A refinement made inside a certificate chooses nothing (C6), so it is
/// held only as enclosures: the homogeneous channels `(w·P, w)` are
/// refined in the ring ([`RingRefinement`]), both ratios taken from the
/// knots, so each entry encloses the described surface's refined
/// coefficient. The grid keeps them homogeneous; a cell reads its
/// points as the quotient and cuts from the channels themselves
/// ([`CellNet::cut`]). Knot insertion is exact in ℝ, so the grid
/// describes the same surface over the same parameters.
#[derive(Clone, Debug)]
pub(crate) struct CellGrid {
    ku: KnotVector,
    kv: KnotVector,
    hom: Vec<[Interval; 4]>,
}

impl CellGrid {
    /// `surface`'s net with `new_u` inserted along `u` and `new_v` along
    /// `v`.
    ///
    /// `None` when an insertion chain refuses, or a refined weight is not
    /// certainly positive ([`certainly_positive`]): the quotient would
    /// enclose no point, and the caller reads the surface's own cells
    /// instead, coarser and sound.
    pub(crate) fn refined<T: CertifiedBounds>(
        surface: &NurbsSurface<T>,
        new_u: &[f64],
        new_v: &[f64],
    ) -> Option<Self> {
        let (ku, kv) = (surface.knots_u_f64_interim(), surface.knots_v_f64_interim());
        let ref_u = RingRefinement::new(ku, new_u).ok()?;
        let ref_v = RingRefinement::new(kv, new_v).ok()?;
        let homogeneous: Vec<[Interval; 4]> = surface
            .control()
            .iter()
            .zip(surface.weights())
            .map(|(p, &w)| {
                let w = Interval::from_certified(w);
                let [x, y, z] = [p.x, p.y, p.z].map(|x| w * Interval::from_certified(x));
                [x, y, z, w]
            })
            .collect();
        let (nu, nv) = (ku.control_count(), kv.control_count());
        let nets: [TensorNet; 4] = core::array::from_fn(|c| {
            TensorNet::from_flat(nu, nv, homogeneous.iter().map(|h| h[c]).collect())
                .refine_u(&ref_u)
                .refine_v(&ref_v)
        });
        let [x, y, z, w] = nets.each_ref().map(TensorNet::as_flat);
        if !certainly_positive(w) {
            return None;
        }
        let hom = x
            .iter()
            .zip(y)
            .zip(z)
            .zip(w)
            .map(|(((x, y), z), w)| [*x, *y, *z, *w])
            .collect();
        Some(Self {
            ku: ref_u.knots().clone(),
            kv: ref_v.knots().clone(),
            hom,
        })
    }

    /// Entry `k`'s point (the quotient), weight and homogeneous
    /// channels.
    fn entry(&self, k: usize) -> Option<([Interval; 3], Interval, [Interval; 4])> {
        let h = *self.hom.get(k)?;
        Some(([h[0] / h[3], h[1] / h[3], h[2] / h[3]], h[3], h))
    }

    /// The refined `u` vector.
    #[cfg(test)]
    pub(crate) fn knots_u(&self) -> &KnotVector {
        &self.ku
    }

    /// The refined `v` vector.
    #[cfg(test)]
    pub(crate) fn knots_v(&self) -> &KnotVector {
        &self.kv
    }

    /// The refined net's points, row-major: the quotients.
    #[cfg(test)]
    pub(crate) fn points(&self) -> Vec<[Interval; 3]> {
        (0..self.hom.len())
            .filter_map(|k| Some(self.entry(k)?.0))
            .collect()
    }

    /// The refined net's weights, row-major.
    #[cfg(test)]
    pub(crate) fn weights(&self) -> Vec<Interval> {
        self.hom.iter().map(|h| h[3]).collect()
    }
}

/// One span cell's first-order quotients `S_u = N_u/W²` and
/// `S_v = N_v/W²` in certification arithmetic, over the cell or cut to a
/// sub-rectangle of it ([`geom_core::spline::CellFirstQuotients`]),
/// formed once from the cell's control block in a frame `c` at its first
/// point: off the points and weights for the surface's own cells, off
/// the homogeneous channels `(w·(P − c), w)` for a grid's. Every reading
/// is a coefficient ratio of `N_u` or `N_v` over `W²`; `N_d` is
/// translation-free (`A ↦ A + t·W` leaves `A_d·W − A·W_d` fixed) and
/// carried by a rotation to its image, so a rigid map moves a reading by
/// its rounding width alone.
#[derive(Debug)]
struct CellNet {
    /// The quotients over the whole cell, which a cut restricts; `None`
    /// on a cut net, and where the statement refused.
    raised: Option<CellFirstQuotients<Interval>>,
    /// `S_u` on each of the block's [`BLOCK_PIECE_SPLITS`]² pieces,
    /// `None` where the statement refused (every reading of it then
    /// refuses) or, on a cut net, where its reader did not ask for it.
    u: Option<IntervalPieces>,
    /// `S_v` likewise, piece for piece in `u`'s order.
    v: Option<IntervalPieces>,
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
        Self::of_spans((win.span_u(), win.span_v()), win.stride(), |k| {
            let (p, &w) = ctl.get(k).zip(wts.get(k))?;
            Some((
                [
                    Interval::from_certified(p.x),
                    Interval::from_certified(p.y),
                    Interval::from_certified(p.z),
                ],
                Interval::from_certified(w),
                None,
            ))
        })
    }

    /// The control block of the cell at `(spu, spv)` of a row-major net
    /// `stride` columns wide, each entry's point, weight and (from a
    /// grid) homogeneous channels read through `at` by flat index.
    /// `None` when a read leaves the net or the knots.
    fn of_spans(
        (spu, spv): (Span<'_>, Span<'_>),
        stride: usize,
        at: impl Fn(usize) -> Option<([Interval; 3], Interval, Option<[Interval; 4]>)>,
    ) -> Option<Self> {
        let (pu, pv) = (spu.degree(), spv.degree());
        let mut pts = Vec::with_capacity((pu + 1) * (pv + 1));
        let mut ws = Vec::with_capacity((pu + 1) * (pv + 1));
        let mut hom = Some(Vec::with_capacity((pu + 1) * (pv + 1)));
        for i in 0..=pu {
            let row = (spu.first_control() + i) * stride + spv.first_control();
            for j in 0..=pv {
                let (p, w, h) = at(row + j)?;
                pts.push(p);
                ws.push(w);
                hom = hom.zip(h).map(|(mut v, h)| {
                    v.push(h);
                    v
                });
            }
        }
        let (c, h) = translate(&pts, &ws, hom.as_deref())?;
        let (held_u, held_v) = (held(spu), held(spv));
        let axes = (CellAxis::Span(spu, &held_u), CellAxis::Span(spv, &held_v));
        let (a, w): (Vec<[Interval; 3]>, Vec<Interval>) =
            h.iter().map(|c| ([c[0], c[1], c[2]], c[3])).unzip();
        let raised = if hom.is_some() {
            cell_first_quotients(axes, CellBlock::Homogeneous { a: &a, w: &w })
        } else {
            // Exact points: the pair form reads equal points with equal
            // weights as an exact zero, which differencing their
            // translated channels would not.
            cell_first_quotients(
                axes,
                CellBlock::Points {
                    points: &pts,
                    weights: &ws,
                    origin: c,
                },
            )
        };
        // Both axes or neither: a refusal of either refuses the cell.
        let splits = (BLOCK_PIECE_SPLITS, BLOCK_PIECE_SPLITS);
        let (u, v) = raised
            .as_ref()
            .and_then(|r| Some((r.axis_pieces(true, splits)?, r.axis_pieces(false, splits)?)))
            .unzip();
        Some(Self { raised, u, v })
    }

    /// This cell's quotients cut to `wu × wv`: along each axis whose
    /// window falls strictly inside the cell's span with positive width,
    /// the whole cell's quotients restricted to the window's part of the
    /// span ([`CellFirstQuotients::restrict`]), so its readings enclose
    /// that part alone. `None` when no axis is cut (the window covers
    /// the span whole, or meets it in at most a point), or on a net with
    /// no whole-cell quotients: the whole cell is the caller's answer
    /// either way, and a sound one.
    ///
    /// Only the `axes` its reader reads are restricted (`(S_u, S_v)`): a
    /// reader of one axis's quotient restricts its numerator and `W²`
    /// alone, bit for bit the pieces restricting both would give it.
    fn cut(
        &self,
        (span_u, span_v): (Span<'_>, Span<'_>),
        w: UvWindow,
        axes: (bool, bool),
    ) -> Option<Self> {
        let (cut_u, cut_v) = (inside(span_u, w.u), inside(span_v, w.v));
        if cut_u.is_none() && cut_v.is_none() {
            return None;
        }
        let (u, v) = self.raised.as_ref()?.cut_pieces(
            (cut_u, cut_v),
            axes,
            (BLOCK_PIECE_SPLITS, BLOCK_PIECE_SPLITS),
        );
        Some(Self { raised: None, u, v })
    }

    /// The coefficient ratios `a_k/b_k` of `S_u` (`along_u`) or `S_v`
    /// on every piece; `None` where the statement refused.
    fn ratios(&self, along_u: bool) -> Option<Vec<[Interval; 3]>> {
        Some(
            if along_u { &self.u } else { &self.v }
                .as_ref()?
                .iter()
                .flat_map(|q| {
                    q.numer()
                        .iter()
                        .zip(q.denom())
                        .map(|(a, b)| a.map(|x| x / *b))
                })
                .collect(),
        )
    }

    /// The box for `S_d` over the block's region: the hull of its pieces'
    /// coefficient ratios `a_k/b_k`, of which `S_d` is a convex
    /// combination on each piece (`geom_core::spline::derivative`'s
    /// module docs).
    fn derivative_box(&self, along_u: bool) -> Box3 {
        self.ratios(along_u)
            .and_then(|r| {
                r.into_iter()
                    .map(|[x, y, z]| Box3 { x, y, z })
                    .reduce(Box3::hull)
            })
            .unwrap_or_else(refused_box)
    }

    /// An upper bound on `‖S_d‖` over the block's region: the largest
    /// `‖a_k/b_k‖`, rounded outward ([`norm_sup`]); `NaN` when refused.
    /// The norm is convex, so its max over a piece's ratio hull is at a
    /// ratio, and a rigid map carries every ratio to its image. In real
    /// arithmetic it is never above the norm of the box
    /// [`CellNet::derivative_box`] reads, which holds every ratio.
    fn derivative_norm_sup(&self, along_u: bool) -> f64 {
        self.ratios(along_u)
            .and_then(|r| r.iter().map(norm_sup).reduce(max_bound))
            .unwrap_or(f64::NAN)
    }

    /// The chart probe's two readings along the chart direction
    /// `e = (ex, ey)` over the block's region:
    /// `(n·(S_u·ex + S_v·ey), sup ‖S_u·ex + S_v·ey‖)`, the first an
    /// enclosure and the second an upper bound rounded outward (`NaN`
    /// when refused).
    ///
    /// On each piece `S_u` and `S_v` are quotients over the one weight
    /// power `W²` at one degree, so `S_u·ex + S_v·ey` is the quotient of
    /// `ex·N_u + ey·N_v` over `W²`: a convex combination of the ratios
    /// `(ex·a_k + ey·a′_k)/b_k`, whose `n·` hull encloses it and whose
    /// largest norm bounds it.
    fn transverse_readings(&self, n: [Interval; 3], ex: Interval, ey: Interval) -> (Interval, f64) {
        let (Some(us), Some(vs)) = (self.u.as_ref(), self.v.as_ref()) else {
            return (Interval::refused(), f64::NAN);
        };
        let mut ratios: Vec<[Interval; 3]> = Vec::new();
        for (qu, qv) in us.iter().zip(vs) {
            if qu.denom().len() != qv.denom().len() {
                return (Interval::refused(), f64::NAN);
            }
            ratios.extend(
                qu.numer()
                    .iter()
                    .zip(qv.numer())
                    .zip(qu.denom())
                    .map(|((a, b), w)| core::array::from_fn(|k| (a[k] * ex + b[k] * ey) / *w)),
            );
        }
        let along = ratios
            .iter()
            .map(|r| dot(&n, r))
            .reduce(Interval::hull)
            .unwrap_or_else(Interval::refused);
        let sup = ratios
            .iter()
            .map(norm_sup)
            .reduce(max_bound)
            .unwrap_or(f64::NAN);
        (along, sup)
    }
}

/// The block reading's piece schedule per direction (D9: structure,
/// never a decision): every block, whole or cut, is read on this many
/// exact dyadic pieces along each axis, as `geom::NurbsSurface`'s
/// stretch bounds read a cell. With one piece a polynomial wall's
/// derivative box keeps its degree-elevated hull, which on
/// `m5_pr7_ssi`'s bent-path flat wall reads the slope across the
/// `v = 0` side at two thirds of its true least (`5.3e-9` against
/// `8e-9`, inside the band at `ε = 1e-9`); two pieces read it exactly
/// (`tests::a_block_is_read_on_its_pieces` re-takes the two-piece
/// reading; the one-piece `5.3e-9` is this measurement's alone).
const BLOCK_PIECE_SPLITS: usize = 2;

/// A span's knot vector held as points.
fn held(span: Span<'_>) -> Vec<Interval> {
    span.knots_f64_interim()
        .knots()
        .iter()
        .map(|&k| Interval::point(k))
        .collect()
}

/// The part of `span`'s knot interval inside `w`, with the span's own
/// interval, when it is a proper part of positive width; `None` when the
/// window covers the span whole or meets it in at most a point.
fn inside(span: Span<'_>, w: ParamRange) -> Option<AxisCut> {
    let (lo, hi) = (w.start(), w.end());
    let t = span.knots_f64_interim().knots();
    let (&ta, &tb) = t.get(span.index()).zip(t.get(span.index() + 1))?;
    let (a, b) = (lo.max(ta), hi.min(tb));
    (a < b && (a > ta || b < tb)).then_some(((ta, tb), (a, b)))
}

/// A block's channels in the frame of its first point's lower bracket
/// end `c`: `(w·(P − c), w)`, from the homogeneous channels `hom` where
/// the block holds them, and `c`.
fn translate(
    pts: &[[Interval; 3]],
    wts: &[Interval],
    hom: Option<&[[Interval; 4]]>,
) -> Option<([Interval; 3], Vec<[Interval; 4]>)> {
    let c = pts.first()?.map(Interval::lo).map(Interval::point);
    let h = match hom {
        Some(hom) => hom
            .iter()
            .map(|h| {
                let at = |k: usize| h[k] - c[k] * h[3];
                [at(0), at(1), at(2), h[3]]
            })
            .collect(),
        None => pts
            .iter()
            .zip(wts)
            .map(|(p, &w)| {
                let at = |k: usize| w * (p[k] - c[k]);
                [at(0), at(1), at(2), w]
            })
            .collect(),
    };
    Some((c, h))
}

/// The box every reading over a window that names no region answers.
pub(crate) fn refused_box() -> Box3 {
    Box3 {
        x: Interval::refused(),
        y: Interval::refused(),
        z: Interval::refused(),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
pub(super) mod tests {
    use super::*;

    std::thread_local! {
        /// Whole-cell nets built on this thread ([`CellNets::get`]).
        pub(in crate::ssi) static WHOLE_NETS: core::cell::Cell<usize> =
            const { core::cell::Cell::new(0) };
    }

    /// A test's window, ordered by construction.
    fn win((u0, u1, v0, v1): (f64, f64, f64, f64)) -> UvWindow {
        UvWindow::new(u0, u1, v0, v1).expect("an ordered window")
    }
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

    /// **A span cell's whole net is built once per [`NurbsBoxes`]**,
    /// however many reads touch it: the point box, both derivative
    /// boxes and a second pass all read the one build, and read it
    /// alike.
    #[test]
    fn a_cells_whole_net_is_built_once_however_often_it_is_read() {
        let s = multiplicity_2_patch();
        let boxes = NurbsBoxes::new(&s);
        let w = win((0.2, 0.8, 0.0, 1.0));
        assert_eq!(boxes.nets.built(), 0, "a net was built before any read");
        let first = format!("{:?}", boxes.rect_box(w));
        assert_eq!(
            boxes.nets.built(),
            2,
            "the window touches the patch's two nonempty cells, each built once"
        );
        let _ = (boxes.deriv_box(w, true), boxes.deriv_box(w, false));
        assert_eq!(
            format!("{:?}", boxes.rect_box(w)),
            first,
            "a second read of the window boxed it differently"
        );
        assert_eq!(boxes.nets.built(), 2, "a later read built a net again");
        let (a, _) = boxes.cell(2, 1).expect("the first cell");
        let (b, _) = boxes.cell(2, 1).expect("the first cell");
        assert!(
            core::ptr::eq(a, b),
            "a second read of cell (2, 1) rebuilt its net"
        );
    }

    /// **A window with a NaN or inverted end names no region: it mints
    /// no `UvWindow`, so no box is read over it.** A NaN window read as
    /// the first span's control block would be a certified box that is
    /// a strict subset of this two-span patch's. The ordered window
    /// beside them mints and certifies, so the rows are about the window
    /// and not the patch.
    #[test]
    fn a_nan_or_inverted_window_is_not_a_window() {
        let s = multiplicity_2_patch();
        let boxes = NurbsBoxes::new(&s);
        let certified = |b: Box3| b.x.is_certified() && b.y.is_certified() && b.z.is_certified();
        let ok = win((0.2, 0.8, 0.0, 1.0));
        for (which, b) in [
            ("deriv_box u", boxes.deriv_box(ok, true)),
            ("deriv_box v", boxes.deriv_box(ok, false)),
            ("rect_box", boxes.rect_box(ok)),
        ] {
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
            assert!(
                UvWindow::new(w.0, w.1, w.2, w.3).is_none(),
                "{name}: minted a window"
            );
        }
        // An unordered corner pair orders itself, and a NaN corner
        // refuses where `min`/`max` would have kept the other corner.
        assert_eq!(
            UvWindow::spanning((0.8, 1.0), (0.2, 0.0)),
            Some(win((0.2, 0.8, 0.0, 1.0)))
        );
        assert!(UvWindow::spanning((nan, 1.0), (0.2, 0.0)).is_none());
        assert!(UvWindow::spanning((0.8, 1.0), (0.2, nan)).is_none());
    }

    /// An inverted window past a domain end would clamp to an ordered
    /// point, so it is refused where it is minted, before any clamp; an
    /// ordered one past the end clamps to the end's point and boxes.
    #[test]
    fn an_inverted_window_past_the_domain_is_not_a_window() {
        let s = multiplicity_2_patch();
        let b = NurbsBoxes::new(&s);
        assert!(
            UvWindow::new(1.5, 1.2, 0.0, 1.0).is_none(),
            "inverted past hi"
        );
        assert!(
            UvWindow::new(-0.2, -0.5, 0.0, 1.0).is_none(),
            "inverted past lo"
        );
        let x = b.rect_box(win((1.2, 1.5, 0.0, 1.0)));
        assert!(
            x.x.is_certified() && x.y.is_certified() && x.z.is_certified(),
            "an ordered window past the end refused: {x:?}"
        );
    }

    /// The derivative box SKIPS an empty span cell, and the skip never
    /// leaves its hull unseeded: a rectangle straddling the empty cell
    /// still reads a certified box in both directions.
    #[test]
    fn a_skipped_empty_span_cell_leaves_the_derivative_box_seeded() {
        let s = multiplicity_2_patch();
        let ku = s.knots_u_f64_interim();
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
        let (su0, su1) = boxes.cells(win((u0, u1, v0, v1))).u;
        assert!(
            (su0..=su1).contains(&3),
            "the rectangle must straddle the empty cell, got {su0}..={su1}"
        );
        for along_u in [true, false] {
            let d = boxes.deriv_box(win((u0, u1, v0, v1)), along_u);
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
    const MARGIN_WINDOWS: [(f64, f64, f64, f64); 3] = [
        (0.0, 1.0, 0.0, 1.0),
        (0.0, 0.34, 0.2, 0.55),
        INSIDE_ONE_CELL,
    ];

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
                chart_transverse_margin(&boxes, n, win(rect), tangent)
                    .expect("no window is degenerate")
                    .expect("every stretch is finite")
            })
            .collect();
        ChartReadings {
            speeds: [speeds.u.get(), speeds.v.get()],
            pads: [pu, pv],
            floors: [1e-16, 1e-15, 1e-13, 1e-12, 1e-9, 1e-3]
                .map(|m| SweepFloor::chart(root, m, speeds.max(), FloorKind::Accounting).is_ok()),
            margins,
        }
    }

    /// Whether a rigid map moved a seated reading `a` to `b` by its
    /// rounding width alone: some ulps of the coordinates' `reach`,
    /// relative to the reading's `magnitude`, times the window's
    /// `scale` — 256 for a speed or pad, 32 for a margin (its measured
    /// worst, 9.44 at `CAD_FUZZ_EFFORT=64`, with about three times
    /// over).
    ///
    /// The magnitude is what the reading's rounding is relative to. A
    /// speed or pad rounds relative to itself
    /// ([`Magnitude::Own`]). A transverse margin does not:
    /// it is `mig(n·d_k) / stretch`, where each `d_k` is no longer than
    /// the stretch, so every coefficient it reads, and the rounding of
    /// each, sits at the stretch's scale, and the margin is that
    /// rounding divided by the stretch. It rounds at magnitude `1`, the
    /// most it can read, whatever small value it lands on
    /// ([`Magnitude::Unit`]).
    fn within_rounding(a: f64, b: f64, reach: f64, scale: f64, magnitude: Magnitude) -> bool {
        let (ulps, m) = match magnitude {
            Magnitude::Own => (256.0, a.abs().max(b.abs())),
            Magnitude::Unit => (32.0, 1.0),
        };
        (a - b).abs() <= ulps * f64::EPSILON * (1.0 + reach) * scale * m
    }

    /// What a reading's rounding is relative to ([`within_rounding`]).
    #[derive(Clone, Copy, Debug)]
    enum Magnitude {
        /// The reading itself: a speed, a pad.
        Own,
        /// One: a margin, a fraction of the stretch its coefficients
        /// are no longer than.
        Unit,
    }

    /// A rigid map of the rotation row's kind: a rotation by `angle`
    /// about the unit `axis` through `pivot`, with the reach its
    /// [`within_rounding`] reads.
    fn rigid_map(
        axis: Vec3<f64>,
        pivot: Point3<f64>,
        angle: f64,
    ) -> (geom_core::Affine3<f64>, f64) {
        let map = geom_core::Affine3::rotation_about_axis(pivot, axis.normalize(), angle);
        (map, 2.0 * (pivot - Point3::origin()).norm() + 4.0)
    }

    /// **A rigid map moves a wall's chart readings by their rounding
    /// width alone, and every floor and tube decision minted from them
    /// stands as it does seated.** Each reading is a norm or a dot with
    /// the plane normal of a vector built from differences of control
    /// points, and a rigid map carries each such vector to its image, so
    /// only the rounding of the image's coordinates enters: a few hundred
    /// ulps of the coordinates' reach, relative, scaled up on a window by
    /// the cut's `1 / width` (a cut net's differences are divided by the
    /// window's width, and its coordinates round at the reach). Relative
    /// to what is [`within_rounding`]'s magnitude: a speed or pad to
    /// itself, a margin to one, since a small margin is a cancellation
    /// among coefficients at the stretch's scale (sampled over 4000 maps,
    /// the worst margin moved 8.1 of these units at magnitude one and 257
    /// at its own 0.0027; 9.44 at magnitude one is the worst measured, at
    /// `CAD_FUZZ_EFFORT=64`). A norm
    /// read off a
    /// per-coordinate box would move by up to √3 instead (5.88 to 8.20
    /// over 32 maps of the 1.8 / 0.7 wall's `u` speed).
    #[test]
    fn a_rigidly_mapped_wall_reads_the_seated_chart_speeds_floors_and_tube() {
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
        let window_scale = MARGIN_WINDOWS.map(|(u0, u1, v0, v1)| 1.0 / (u1 - u0).min(v1 - v0));
        for _ in 0..fuzz::scaled(32) {
            let axis = Vec3::new(
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
            );
            let pivot = Point3::new(
                rng.range(-10.0, 10.0),
                rng.range(-10.0, 10.0),
                rng.range(-10.0, 10.0),
            );
            let angle = rng.range(0.0, core::f64::consts::TAU);
            let (map, reach) = rigid_map(axis, pivot, angle);
            for ((name, wall), at) in walls.iter().zip(&seated) {
                let image = chart_readings(
                    &wall.map_points(|p| map.transform_point(p)),
                    map.linear * n0,
                );
                let pairs = at
                    .speeds
                    .iter()
                    .zip(&image.speeds)
                    .chain(at.pads.iter().zip(&image.pads))
                    .map(|(&a, &b)| (a, b, 1.0, Magnitude::Own))
                    .chain(
                        at.margins
                            .iter()
                            .zip(&image.margins)
                            .zip(window_scale)
                            .map(|((&a, &b), k)| (a, b, k, Magnitude::Unit)),
                    );
                for (a, b, scale, magnitude) in pairs {
                    assert!(
                        within_rounding(a, b, reach, scale, magnitude),
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

    /// **The fuzzer's recorded maps, as fixed rows.** Four seeds of
    /// [`a_rigidly_mapped_wall_reads_the_seated_chart_speeds_floors_and_tube`]
    /// went red on the two-span patch's second-window margin, which sits
    /// at 0.0027 of its stretch: each map moved it by about 4e-15, past
    /// a few hundred ulps of 0.0027 and within a few dozen of one
    /// (`work/ssiarith/rigid-map-chart-readings-fuzz-reds-past-its-rounding-bound.md`).
    /// Each map, written down: every speed and pad stays within its
    /// rounding at its own value, every margin within its rounding at
    /// one, and every floor decides as seated.
    #[test]
    fn the_recorded_rigid_maps_move_a_small_margin_by_its_stretchs_rounding() {
        // (seed, axis, pivot, angle), as each failure printed them.
        let maps = [
            (
                "0x8371abdecebb95f9 (effort 8)",
                Vec3::new(0.8892767327884497, -0.3111456490169011, 0.529547310454898),
                Point3::new(-0.7856759962338078, 0.5412358116783427, 1.931254384335574),
                3.30562505731778,
            ),
            (
                "0x166d62361818555e",
                Vec3::new(
                    -0.9254990197806239,
                    -0.1674496796624283,
                    -0.38154305676260414,
                ),
                Point3::new(
                    1.1971011302698784,
                    -0.13768816972372022,
                    -0.5807137925187895,
                ),
                0.5675790275065047,
            ),
            (
                "0xdfa19917eaceb89e",
                Vec3::new(0.2896241433614475, -0.1361927893414525, 0.6967084815427922),
                Point3::new(0.40140272923285636, 0.4976481229205518, -0.6405201135113732),
                3.6398997930903314,
            ),
            (
                "0x1d0a3b646ab0444b",
                Vec3::new(-0.15680770128407673, 0.9181613401775988, 0.3246244410860559),
                Point3::new(0.4461639737775851, 0.612123104391344, -0.458107698741939),
                2.91710539156148,
            ),
        ];
        let wall = multiplicity_2_patch();
        let n0 = Vec3::new(0.2, -0.5, 0.84).normalize();
        let at = chart_readings(&wall, n0);
        let window_scale = MARGIN_WINDOWS.map(|(u0, u1, v0, v1)| 1.0 / (u1 - u0).min(v1 - v0));
        for (seed, axis, pivot, angle) in maps {
            let (map, reach) = rigid_map(axis, pivot, angle);
            let image = chart_readings(
                &wall.map_points(|p| map.transform_point(p)),
                map.linear * n0,
            );
            for (&a, &b) in at
                .speeds
                .iter()
                .zip(&image.speeds)
                .chain(at.pads.iter().zip(&image.pads))
            {
                assert!(
                    within_rounding(a, b, reach, 1.0, Magnitude::Own),
                    "{seed}: a speed or pad moved {a} to {b}"
                );
            }
            for ((&a, &b), k) in at.margins.iter().zip(&image.margins).zip(window_scale) {
                assert!(
                    within_rounding(a, b, reach, k, Magnitude::Unit),
                    "{seed}: a margin moved {a} to {b}"
                );
            }
            assert_eq!(
                image.floors, at.floors,
                "{seed}: a floor decides differently"
            );
        }
    }

    /// **The rotation row goes red on a per-coordinate fold.** The
    /// mutation of the reading the row pins: the outward norm of the
    /// derivative box (and of the transverse stretch's box), the fold
    /// the chart readings took before. Under two written-down rotations
    /// of the 1.8 / 0.7 wall the fold moves far past
    /// [`within_rounding`] while [`NurbsBoxes::speed_sup`] and the
    /// transverse stretch stay inside it, so the row's comparator tells
    /// the two apart on these maps alone. The `v` fold stays too: that
    /// field keeps one direction, and a box of one direction scaled by
    /// an interval reads the same norm in any frame.
    #[test]
    fn the_rotation_row_reads_red_on_a_per_coordinate_fold() {
        let tangent: (f64, f64) = (1.0, 0.3);
        let tn = tangent.0.hypot(tangent.1);
        let (ex, ey) = (
            Interval::point(-tangent.1 / tn),
            Interval::point(tangent.0 / tn),
        );
        let n = Vec3::new(0.2, -0.5, 0.84).normalize();
        let whole = (0.0, 1.0, 0.0, 1.0);
        // `[fold u, fold v, fold stretch, speed u, speed v, stretch]`.
        let readings = |wall: &NurbsSurface<f64>, n: Vec3<f64>| {
            let boxes = NurbsBoxes::new(wall);
            let fold = |b: Box3| norm_sup(&[b.x, b.y, b.z]);
            let (du, dv) = (
                boxes.deriv_box(win((0.0, 1.0, 0.0, 1.0)), true),
                boxes.deriv_box(win((0.0, 1.0, 0.0, 1.0)), false),
            );
            let fold_stretch = fold(Box3 {
                x: du.x * ex + dv.x * ey,
                y: du.y * ex + dv.y * ey,
                z: du.z * ex + dv.z * ey,
            });
            let n = [n.x, n.y, n.z].map(Interval::point);
            [
                fold(du),
                fold(dv),
                fold_stretch,
                boxes.speed_sup(win((0.0, 1.0, 0.0, 1.0)), true),
                boxes.speed_sup(win((0.0, 1.0, 0.0, 1.0)), false),
                boxes.transverse_readings(win(whole), n, ex, ey).1,
            ]
        };
        let wall = rational_wall(0.0);
        let seated = readings(&wall, n);
        for (axis, pivot, angle) in [
            (
                Vec3::new(0.0, 0.0, 1.0),
                Point3::origin(),
                core::f64::consts::FRAC_PI_4,
            ),
            (Vec3::new(1.0, 1.0, 1.0), Point3::new(3.0, -2.0, 5.0), 1.0),
        ] {
            let (map, reach) = rigid_map(axis, pivot, angle);
            let image = readings(&wall.map_points(|p| map.transform_point(p)), map.linear * n);
            let moved: Vec<bool> = seated
                .iter()
                .zip(&image)
                .map(|(&a, &b)| !within_rounding(a, b, reach, 1.0, Magnitude::Own))
                .collect();
            assert_eq!(
                moved,
                [true, false, true, false, false, false],
                "about {axis:?} by {angle}: which readings moved past the rounding width \
                 (fold u, fold v, fold stretch, speed u, speed v, stretch); seated {seated:?}, \
                 mapped {image:?}"
            );
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
                let step = (((3 * iu + 2 * iv) % 5) as f64 - 2.0) / 2.0;
                weights.push(if spread < 1.0 {
                    1.0 + spread * step
                } else {
                    spread.powf(step)
                });
            }
        }
        NurbsSurface::new(ku, kv, control, weights).expect("valid net")
    }

    /// The nets the dominance rows read beyond the rational wall and
    /// the two-span patch: polynomial and near-polynomial weights,
    /// non-uniform knots, and higher degree, where the quotient's own
    /// slack is small. A `spread` below one reads the weights as
    /// `1 ± spread`, at one and above as `spread^{±1}`
    /// ([`wide_weight_nets`]).
    fn varied_nets() -> Vec<(&'static str, NurbsSurface<f64>)> {
        let cubic_nonuniform = vec![0.0, 0.0, 0.0, 0.0, 0.3, 0.45, 1.0, 1.0, 1.0, 1.0];
        let quadratic_nonuniform = vec![0.0, 0.0, 0.0, 0.6, 1.0, 1.0, 1.0];
        let quartic = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.35, 1.0, 1.0, 1.0, 1.0, 1.0];
        let linear = vec![0.0, 0.0, 1.0, 1.0];
        vec![
            (
                "cubic × quadratic, non-uniform, polynomial",
                varied_net(
                    3,
                    cubic_nonuniform.clone(),
                    2,
                    quadratic_nonuniform.clone(),
                    0.0,
                ),
            ),
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

    /// Nets with weights spread over decades, for the dominance rows: a
    /// cut's restriction and halving run there on weight powers up to
    /// eighteen orders apart (`W³` at `10^{±3}`).
    fn wide_weight_nets() -> Vec<(&'static str, NurbsSurface<f64>)> {
        vec![
            (
                "cubic × quadratic, non-uniform, weights 1e-3 to 1e3",
                varied_net(
                    3,
                    vec![0.0, 0.0, 0.0, 0.0, 0.3, 0.45, 1.0, 1.0, 1.0, 1.0],
                    2,
                    vec![0.0, 0.0, 0.0, 0.6, 1.0, 1.0, 1.0],
                    1e3,
                ),
            ),
            (
                "quartic × linear, weights 1e-2 to 1e2",
                varied_net(
                    4,
                    vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.35, 1.0, 1.0, 1.0, 1.0, 1.0],
                    1,
                    vec![0.0, 0.0, 1.0, 1.0],
                    1e2,
                ),
            ),
        ]
    }

    /// **A block is read on its pieces.** The flat wall `z = 1e-6·x` of
    /// `m5_pr7_ssi`'s bent chart path (`x` quadratic in `u`, its `v`
    /// slope `α·x_v` running from `−1.6e-8` to `−8e-9` across the
    /// `v = 0` side): over the strip `v ∈ [0, 10⁻³]` the box's `z` is the
    /// true range to rounding. Read as one piece the strip keeps the
    /// degree-elevated hull of `x_v`'s `u` coefficients and its upper end
    /// sits at `−5.3e-9`, which the boundary pass reads as a slope inside
    /// the band at `ε = 1e-9`.
    #[test]
    fn a_block_is_read_on_its_pieces() {
        let (alpha, width, kappa, mu) = (1e-6, 0.01, 0.8, 0.2);
        let (lin, square, hump) = ([-0.5, 0.0, 0.5], [0.25, -0.25, 0.25], [0.0, 2.0, 0.0]);
        let k = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let mut control = Vec::new();
        for i in 0..3 {
            for (j, h) in hump.iter().enumerate() {
                let x = width * (lin[i] - kappa * square[i] * h - mu * h);
                control.push(Point3::new(x, 0.25 * j as f64, alpha * x));
            }
        }
        let s = NurbsSurface::new(k.clone(), k, control, vec![1.0; 9]).unwrap();
        let z = NurbsBoxes::new(&s)
            .deriv_box(win((0.0, 1.0, 0.0, 1e-3)), false)
            .z;
        // The true range: `α·width·4·(1 − 2v)·(−κ(u − ½)² − μ)` at
        // `v ∈ [0, 10⁻³]`.
        let (lo, hi) = (-1.6e-8, -7.984e-9);
        assert!(
            z.lo() <= lo && hi <= z.hi() && z.lo() >= lo - 1e-22 && z.hi() <= hi + 1e-20,
            "the strip's z box [{:e}, {:e}] against the true [{lo:e}, {hi:e}]",
            z.lo(),
            z.hi()
        );
    }

    /// **Each cell's derivative box holds the true derivative.** The box
    /// is the hull of the cell's coefficient ratios `a_k/b_k`, of which
    /// `S_d` is a convex combination; it must contain `S_d` read from
    /// `ders` at a dense grid of the cell's parameters (ends included).
    /// The slack is the f64 rounding of `ders`.
    #[test]
    fn each_cells_derivative_box_holds_the_sampled_derivative() {
        let mut nets = varied_nets();
        nets.extend(wide_weight_nets());
        nets.push(("rational wall", rational_wall(0.0)));
        nets.push(("two-span patch", multiplicity_2_patch()));
        let n = 16;
        for (name, s) in &nets {
            let (ku, kv) = (s.knots_u_f64_interim(), s.knots_v_f64_interim());
            for su in ku.first_span()..=ku.last_span() {
                for sv in kv.first_span()..=kv.last_span() {
                    let Some(win) = s.window(su, sv) else {
                        continue;
                    };
                    let (ua, ub) = (ku.knots()[su], ku.knots()[su + 1]);
                    let (va, vb) = (kv.knots()[sv], kv.knots()[sv + 1]);
                    let net = CellNet::of_cell(win).expect("the cell's block");
                    for along_u in [true, false] {
                        let b = net.derivative_box(along_u);
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
                                let d = if along_u { jet.du } else { jet.dv };
                                for (c, x) in [(b.x, d.x), (b.y, d.y), (b.z, d.z)] {
                                    let slack = 32.0 * f64::EPSILON * x.abs().max(1.0);
                                    assert!(
                                        c.lo() - slack <= x && x <= c.hi() + slack,
                                        "{name}: S_d (along_u = {along_u}) {x} at ({u}, {v}) \
                                         outside [{}, {}] on cell ({su}, {sv})",
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

    /// **The derivative box dominates the true derivative, sampled
    /// densely**, over the whole domain and over sub-rectangles, on the
    /// rational wall and the two-span patch at every translation, and on
    /// [`varied_nets`]: each
    /// sampled component lies in the box, and the chart speed and the
    /// window's [`NurbsBoxes::speed_sup`] are at least every sampled
    /// speed. The chart probe's readings along a chart direction `e`
    /// dominate too: `n·(S_u·e.x + S_v·e.y)` lies in their enclosure and
    /// `‖S_u·e.x + S_v·e.y‖` is at most their stretch. The true derivative is evaluated on
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
        for (name, net) in varied_nets().into_iter().chain(wide_weight_nets()) {
            cases.push((name.to_string(), net.clone(), net, 0.0));
        }
        let n = 16;
        let plane_n = [0.2, -0.5, 0.84];
        let e = (-0.3 / 1.0_f64.hypot(0.3), 1.0 / 1.0_f64.hypot(0.3));
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
                let du = boxes.deriv_box(win((u0, u1, v0, v1)), true);
                let dv = boxes.deriv_box(win((u0, u1, v0, v1)), false);
                let sup =
                    [true, false].map(|along_u| boxes.speed_sup(win((u0, u1, v0, v1)), along_u));
                let (phi, stretch) = boxes.transverse_readings(
                    win((u0, u1, v0, v1)),
                    plane_n.map(Interval::point),
                    Interval::point(e.0),
                    Interval::point(e.1),
                );
                for i in 0..=n {
                    for j in 0..=n {
                        let u = u0 + (u1 - u0) * f64::from(i) / f64::from(n);
                        let v = v0 + (v1 - v0) * f64::from(j) / f64::from(n);
                        let jet = origin.ders(u, v);
                        let along = jet.du * e.0 + jet.dv * e.1;
                        let dot =
                            plane_n[0] * along.x + plane_n[1] * along.y + plane_n[2] * along.z;
                        assert!(
                            phi.lo() - slack(dot) <= dot && dot <= phi.hi() + slack(dot),
                            "{name}: n·(S_u·e.x + S_v·e.y)({u}, {v}) = {dot} outside [{}, {}] \
                             over [{u0}, {u1}]×[{v0}, {v1}]",
                            phi.lo(),
                            phi.hi()
                        );
                        assert!(
                            along.norm() <= stretch + slack(along.norm()),
                            "{name}: |S_u·e.x + S_v·e.y|({u}, {v}) = {} above the stretch \
                             {stretch} over [{u0}, {u1}]×[{v0}, {v1}]",
                            along.norm()
                        );
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
                            let window = sup[usize::from(which == "v")];
                            assert!(
                                speed <= s.get().min(window) + slack(speed),
                                "{name}: |S_{which}({u}, {v})| = {speed} above the chart \
                                 speed {} or the window's {window} over \
                                 [{u0}, {u1}]×[{v0}, {v1}]",
                                s.get()
                            );
                        }
                    }
                }
            }
        }
    }

    /// **A refused plane normal measures no chart margin.** Over the
    /// dome, a window's transverse margin against a plane is a finite
    /// distance from zero. With the normal refused, `∇φ` is refused while
    /// the chart's stretch stays finite, and the margin reads `NaN`
    /// (measured nothing), never the `0` of a straddle.
    #[test]
    fn a_refused_normal_measures_no_chart_margin() {
        let k = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2)
            .expect("clamped quadratic");
        let mut control = Vec::with_capacity(9);
        for i in 0..3u8 {
            for j in 0..3u8 {
                let y = if i == 1 && j == 1 { -1.5 } else { 0.0 };
                control.push(Point3::new(f64::from(i) / 2.0, y, f64::from(j) / 2.0));
            }
        }
        let dome = NurbsSurface::new(k.clone(), k, control, vec![1.0; 9]).expect("the dome");
        let boxes = NurbsBoxes::new(&dome);
        let (tx, ty) = (1.0_f64, 0.3_f64);
        let tangent = (tx, ty, tx.hypot(ty));
        let read = |n| {
            chart_transverse_margin(&boxes, n, win(INSIDE_ONE_CELL), tangent)
                .expect("the window is not degenerate")
                .expect("the stretch is finite")
        };
        let measured = read([0.0, 1.0, 0.0].map(Interval::point));
        assert!(measured.is_finite() && measured > 0.0, "{measured}");
        let refused = read([Interval::refused(); 3]);
        assert!(refused.is_nan(), "a refused normal read {refused}");
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
        let du = boxes.deriv_box(win((u0, u1, v0, v1)), true);
        let dv = boxes.deriv_box(win((u0, u1, v0, v1)), false);
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
                    let cut = boxes.deriv_box(win((u0, u1, v0, v1)), along_u);
                    let whole = boxes.cell_deriv_box(win((u0, u1, v0, v1)), along_u);
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
        let cone = Surface::Cone {
            apex: Point3::new(0.0, 0.0, -2.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            half_angle: 0.4,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let b = Box3::around(Point3::new(0.8, 0.0, 0.0), 0.1);
        assert!(!implicit_enclosure(&cone, b).is_certified());
        assert!(!implicit_gradient_enclosure(&cone, b)[0].is_certified());
        assert!(!graph_margin(&cone, &sphere(), b, Vec3::new(1.0, 0.0, 0.0)).is_certified());
    }

    /// A torus `(centre, axis, R, r)`, its axis normalized in `f64` and so
    /// not exactly unit.
    pub(in crate::ssi) fn torus(c: [f64; 3], a: [f64; 3], big: f64, small: f64) -> Surface<f64> {
        let axis = Vec3::new(a[0], a[1], a[2]).normalize();
        let u_ref = axis.cross(Vec3::new(0.3, -0.7, 0.2)).normalize();
        Surface::Torus {
            center: Point3::new(c[0], c[1], c[2]),
            axis,
            major_radius: big,
            minor_radius: small,
            u_ref,
        }
    }

    /// The torus point at `(u, v)`: `u` about the axis, `v` about the tube.
    pub(in crate::ssi) fn on_torus(s: &Surface<f64>, u: f64, v: f64) -> Point3<f64> {
        let Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        } = *s
        else {
            unreachable!("a torus fixture")
        };
        let e2 = axis.cross(u_ref);
        let radial = u_ref * u.cos() + e2 * u.sin();
        center + radial * (major_radius + minor_radius * v.cos()) + axis * (minor_radius * v.sin())
    }

    /// `g = F/(4rA)` at a point, in `f64`.
    fn torus_g(s: &Surface<f64>, p: Point3<f64>) -> f64 {
        let Surface::Torus {
            center,
            axis,
            major_radius: big,
            minor_radius: small,
            ..
        } = *s
        else {
            unreachable!("a torus fixture")
        };
        let q = p - center;
        let h = q.dot(axis);
        let w = q - axis * h;
        let a = q.norm_squared() + big * big - small * small;
        (a * a - 4.0 * big * big * w.norm_squared()) / (4.0 * small * a)
    }

    /// **The torus enclosures hold every point they claim**, counterexample
    /// search over thin, fat, horn and spindle tori on tilted axes and
    /// boxes from 1e-4 to a third of the tube (a ring torus's enclosures
    /// never refuse; a spindle's may, and are read only where they do
    /// not): at sampled points of each box the residual's
    /// point enclosure meets the box's (an unsound mean-value term or a
    /// gradient that is not `g`'s own misses it), a box holding a surface
    /// point keeps zero (exclusion there would lose a section), and the
    /// gradient is `g`'s derivative, by central differences.
    #[test]
    fn a_torus_enclosure_holds_every_point_of_its_box() {
        use test_utils::fuzz;
        let mut rng = fuzz::start("torus enclosure");
        for _ in 0..fuzz::scaled(200) {
            let small = rng.log_uniform(1e-3, 1.0);
            let big = small * rng.range(0.5, 30.0);
            let ring = big > 1.01 * small;
            let c = [
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
            ];
            let a = [rng.range(-1.0, 1.0), rng.range(-1.0, 1.0), 0.5];
            let t = torus(c, a, big, small);
            let on = on_torus(&t, rng.range(0.0, 6.3), rng.range(0.0, 6.3));
            let half = small * rng.log_uniform(1e-4, 0.3);
            let off = |rng: &mut fuzz::Rng| rng.range(-half, half);
            let centre = on + Vec3::new(off(&mut rng), off(&mut rng), off(&mut rng));
            let b = Box3::around(centre, half);
            let enc = implicit_enclosure(&t, b);
            let grad = implicit_gradient_enclosure(&t, b);
            let ctx = format!("{t:?} over {b:?}: {enc:?}; {}", fuzz::replay());
            let certified = enc.is_certified() && grad.iter().all(|g| g.is_certified());
            assert!(certified || !ring, "a ring torus never refuses: {ctx}");
            if !certified {
                continue;
            }
            if b.x.contains(on.x) && b.y.contains(on.y) && b.z.contains(on.z) {
                assert!(enc.contains(0.0), "a box on the surface keeps zero: {ctx}");
            }
            for _ in 0..8 {
                let p = Point3::new(
                    rng.range(b.x.lo(), b.x.hi()),
                    rng.range(b.y.lo(), b.y.hi()),
                    rng.range(b.z.lo(), b.z.hi()),
                );
                let at = implicit_enclosure(&t, Box3::between(p, p));
                assert!(
                    at.lo() <= enc.hi() && enc.lo() <= at.hi(),
                    "g at {p:?} is {at:?}, outside {ctx}"
                );
                if !ring {
                    continue;
                }
                let step = 1e-6 * small;
                let g_at = implicit_gradient_enclosure(&t, Box3::between(p, p));
                for (k, e) in [Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()]
                    .into_iter()
                    .enumerate()
                {
                    let fd = (torus_g(&t, p + e * step) - torus_g(&t, p - e * step)) / (2.0 * step);
                    let mid = 0.5 * (g_at[k].lo() + g_at[k].hi());
                    assert!(
                        (fd - mid).abs() <= 1e-4 * (1.0 + mid.abs()),
                        "∂g/∂x{k} at {p:?}: differences {fd}, enclosure {:?}: {ctx}",
                        g_at[k]
                    );
                    assert!(
                        grad[k].lo() <= g_at[k].hi() && g_at[k].lo() <= grad[k].hi(),
                        "∇g at {p:?} outside the box's: {ctx}"
                    );
                }
            }
        }
    }

    /// **Limb 2's torus conversion is sound at a point, and tight on a
    /// ring torus.** With `F` and `A` enclosed at one point near a torus
    /// (thin to fat, horn and spindle included, tilted, within a tenth of
    /// the tube), [`super::torus_meters_sup`] never falls under the meters
    /// residual and [`super::torus_meters_floor`] never rises over it; on
    /// a ring torus `R ≥ 1.2r` the bound is the residual to within 10⁻⁶ and
    /// [`super::torus_meters_floor`] lies between 0.4 of it and it.
    /// Dividing by `A` alone would read twice the residual, and the
    /// global `2r(R² − r²)` up to `4R/(R − r)` times it.
    #[test]
    fn the_torus_meters_conversion_is_the_residual_at_a_point() {
        use test_utils::fuzz;
        let mut rng = fuzz::start("torus meters conversion");
        for _ in 0..fuzz::scaled(500) {
            let small = rng.log_uniform(1e-3, 1.0);
            let big = small * rng.range(0.5, 30.0);
            let c = [
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
            ];
            let t = torus(c, [rng.range(-1.0, 1.0), 0.4, 0.7], big, small);
            let on = on_torus(&t, rng.range(0.0, 6.3), rng.range(0.0, 6.3));
            let lift = small * rng.log_uniform(1e-6, 0.1);
            let p =
                on + implicit_gradient(&t, on) * (lift * if rng.unit() < 0.5 { -1.0 } else { 1.0 });
            let Surface::Torus { center, axis, .. } = t else {
                unreachable!("a torus")
            };
            let q = p - center;
            let rho2 = (q - axis * q.dot(axis)).norm_squared();
            let a = Interval::point(q.norm_squared()) + super::torus_a_shift(big, small);
            let f = a.sqr() - Interval::point(4.0 * big * big) * Interval::point(rho2);
            let m = implicit_residual(&t, p).abs();
            let sup = super::torus_meters_sup(f, a, big, small);
            let floor = super::torus_meters_floor(f, a, small);
            let ctx = format!("{t:?} at {p:?}: residual {m:e}; {}", fuzz::replay());
            assert!(
                sup.is_nan() || sup >= m * (1.0 - 1e-6),
                "bound {sup:e} under the residual: {ctx}"
            );
            assert!(
                floor <= m * (1.0 + 1e-6),
                "floor {floor:e} over the residual: {ctx}"
            );
            if big >= 1.2 * small {
                assert!(sup <= m * (1.0 + 1e-6), "bound {sup:e} loose: {ctx}");
                assert!(floor >= 0.4 * m, "floor {floor:e} loose: {ctx}");
            }
        }
    }

    /// **A spindle torus's floor stays under its residual where `A` is
    /// negative**, at its centre: `R = 0.75`, `r = 1`, `A = R² − r² < 0`
    /// while `2A + r² > 0`.
    #[test]
    fn a_spindle_toruss_floor_stays_under_its_residual() {
        let t = torus([0.0; 3], [0.0, 0.0, 1.0], 0.75, 1.0);
        let a = Interval::point(0.0) + super::torus_a_shift(0.75, 1.0);
        let floor = super::torus_meters_floor(a.sqr(), a, 1.0);
        let m = implicit_residual(&t, Point3::origin()).abs();
        assert!(floor <= m, "floor {floor} over the residual {m}");
    }

    /// **The torus enclosures hold adversarial boxes**, counterexample
    /// search on ring tori from barely ring (`R/r` within 5 % of 1) to
    /// thin (`R/r` up to 60), boxes from 10⁻⁷ of the torus to twice it,
    /// placed at the centre, along the axis, at the inner equator and
    /// anywhere about: each corner's point enclosures meet the box's.
    #[test]
    fn a_torus_enclosure_holds_adversarial_boxes() {
        use test_utils::fuzz;
        let mut rng = fuzz::start("torus adversarial boxes");
        for it in 0..fuzz::scaled(500) {
            let small = rng.log_uniform(1e-3, 3.0);
            let big = small
                * match it % 3 {
                    0 => rng.range(1.0005, 1.05),
                    1 => rng.range(20.0, 60.0),
                    _ => rng.range(1.001, 30.0),
                };
            let t = torus(
                [
                    rng.range(-2.0, 2.0),
                    rng.range(-2.0, 2.0),
                    rng.range(-2.0, 2.0),
                ],
                [
                    rng.range(-1.0, 1.0),
                    rng.range(-1.0, 1.0),
                    rng.range(-1.0, 1.0),
                ],
                big,
                small,
            );
            let Surface::Torus { center, axis, .. } = t else {
                unreachable!("a torus")
            };
            let e = axis.cross(Vec3::new(0.3, -0.7, 0.2)).normalize();
            let jitter = |rng: &mut fuzz::Rng, s: f64| {
                Vec3::new(rng.range(-s, s), rng.range(-s, s), rng.range(-s, s))
            };
            let ctr = match it % 4 {
                0 => center + jitter(&mut rng, 0.01 * small),
                1 => center + axis * rng.range(-2.0 * small, 2.0 * small),
                2 => center + e * (big - small) + jitter(&mut rng, 0.1 * small),
                _ => center + jitter(&mut rng, 1.3 * (big + small)),
            };
            let b = Box3::around(ctr, (big + small) * rng.log_uniform(1e-7, 2.0));
            let (enc, grad) = (
                implicit_enclosure(&t, b),
                implicit_gradient_enclosure(&t, b),
            );
            let ctx = format!("{t:?} over {b:?}; {}", fuzz::replay());
            assert!(enc.is_certified(), "a ring torus never refuses: {ctx}");
            for k in 0..8 {
                let p = Point3::new(
                    if k & 1 == 0 { b.x.lo() } else { b.x.hi() },
                    if k & 2 == 0 { b.y.lo() } else { b.y.hi() },
                    if k & 4 == 0 { b.z.lo() } else { b.z.hi() },
                );
                let at = implicit_enclosure(&t, Box3::between(p, p));
                let ga = implicit_gradient_enclosure(&t, Box3::between(p, p));
                assert!(
                    at.lo() <= enc.hi() && enc.lo() <= at.hi(),
                    "at {p:?}: {ctx}"
                );
                for j in 0..3 {
                    assert!(
                        ga[j].lo() <= grad[j].hi() && grad[j].lo() <= ga[j].hi(),
                        "∇g axis {j} at {p:?}: {ctx}"
                    );
                }
            }
        }
    }

    /// **On the surface the torus gradient is the unit normal**, the
    /// implicit layer's own: the uniqueness tube reads it as a sine.
    #[test]
    fn the_torus_gradient_is_the_unit_normal_on_the_surface() {
        let t = torus([0.1, -0.2, 0.3], [0.2, 0.3, 0.9], 1.0, 0.1);
        for (u, v) in [(0.0, 0.0), (1.0, 2.0), (2.5, 3.1), (4.0, 4.7), (5.5, 1.6)] {
            let p = on_torus(&t, u, v);
            let g = implicit_gradient_enclosure(&t, Box3::between(p, p));
            let n = implicit_gradient(&t, p);
            for (k, nk) in [n.x, n.y, n.z].into_iter().enumerate() {
                let mid = 0.5 * (g[k].lo() + g[k].hi());
                assert!(
                    (mid - nk).abs() < 1e-9,
                    "({u}, {v}) axis {k}: {g:?} vs {n:?}"
                );
            }
            let r = implicit_enclosure(&t, Box3::between(p, p));
            assert!(r.contains(0.0) || r.mag() < 1e-15, "({u}, {v}): {r:?}");
            assert!(
                implicit_residual(&t, p).abs() < 1e-15,
                "the fixture is on the torus"
            );
        }
    }

    /// **A spindle torus near its axis refuses**: `A` straddles zero
    /// there, and `g` and `∇g` divide by it.
    #[test]
    fn a_torus_enclosure_refuses_where_a_touches_zero() {
        let spindle = torus([0.0; 3], [0.0, 0.0, 1.0], 0.2, 0.5);
        let b = Box3::around(Point3::new(0.0, 0.0, 0.0), 0.05);
        assert!(!implicit_enclosure(&spindle, b).is_certified());
        assert!(
            implicit_gradient_enclosure(&spindle, b)
                .iter()
                .all(|g| !g.is_certified())
        );
        let clear = Box3::around(Point3::new(0.0, 0.0, 0.0), 0.05);
        let ring = torus([0.0; 3], [0.0, 0.0, 1.0], 0.5, 0.2);
        assert!(
            implicit_enclosure(&ring, clear).is_certified(),
            "a ring torus's A > 0"
        );
    }

    /// **[`super::has_implicit_enclosure`] answers what the enclosures
    /// do**, kind by kind, on a box clear of every operand's degenerate
    /// set. It asks both: the tube's one-arc proof (`one_arc_r3`)
    /// reads the residual enclosure as well as the gradient, so an arm
    /// that encloses the gradient alone does not admit a kind.
    #[test]
    fn the_enclosure_roster_is_the_enclosures_own() {
        let plane = Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let cone = Surface::Cone {
            apex: Point3::new(0.0, 0.0, -2.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            half_angle: 0.4,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let torus = Surface::Torus {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            major_radius: 1.0,
            minor_radius: 0.2,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let b = Box3::around(Point3::new(0.8, 0.2, 0.1), 0.05);
        let mut seen = Vec::new();
        for s in [plane, cylinder(), cone, sphere(), torus] {
            let kind = s.kind();
            seen.push(kind);
            let encloses = implicit_enclosure(&s, b).is_certified()
                && implicit_gradient_enclosure(&s, b)
                    .iter()
                    .all(|g| g.is_certified());
            assert_eq!(super::has_implicit_enclosure(kind), encloses, "{kind:?}");
        }
        for kind in geom::SurfaceKind::ALL {
            assert!(
                seen.contains(&kind) || !super::has_implicit_enclosure(kind),
                "{kind:?} is claimed and not measured here"
            );
        }
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
