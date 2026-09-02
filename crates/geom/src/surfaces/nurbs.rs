//! The tensor-product NURBS surface — the [`crate::surfaces::Surface::Nurbs`]
//! payload (M5 PR 3).
//!
//! Data model, evaluation contract, and fixed-association rules are
//! the curve module's, lifted to the tensor product (see
//! [`crate::curves::nurbs`]; the conventions are
//! stated once there and once here — clamped-v1 per direction, f64
//! structure, positive weights, span contract with documented
//! polynomial-extension garbage-out). A [`SurfaceWindow`] pairs two
//! validated spans with the layout that flattens them, but it carries
//! no borrow of the surface it was minted from, so each `*_in_span`
//! core asks [`NurbsSurface::admits`] and answers a window this
//! surface does not admit with all-poison rather than an
//! out-of-bounds index.
//!
//! # Grid layout (binding)
//!
//! Control points and weights are **row-major over `u` then `v`**:
//! `index = iu · nv + iv` with `nu = knots_u.control_count()`,
//! `nv = knots_v.control_count()`. Combination is a double ascending
//! pass — outer `iu`, inner `iv`, exactly as written in the evaluator
//! bodies. Inside a span window that layout is [`SurfaceWindow`]'s
//! `base`/`stride` rather than prose: the evaluators walk
//! `row(i) + j`.
//!
//! # Direction-mapped knot algebra
//!
//! The u-direction operations apply the shared curve plans
//! (`geom_core::spline::algebra`) **per v-column** (each column has
//! its own weights, hence its own projective λs; the knot schedule is
//! shared). The v-direction operations are the u-direction ones
//! conjugated by [`NurbsSurface::transposed`] — one implementation,
//! both directions, deterministic.

use geom_core::spline::{self, KnotAlgebraError, KnotVector, Span, SpanLocate, SplineError};
use geom_core::{Point3, Real, Vec3};

use crate::net;

/// The point and every partial with `k + l ≤ 2` at one parameter pair.
///
/// This is [`crate::surfaces::Surface::jet`]'s return type for **all
/// six** variants, so read it as a surface jet, not as a NURBS
/// evaluation artifact: the analytic arms fill it directly, with no
/// span and no basis pass involved. It lives in this module because
/// [`NurbsSurface::ders`] — which does fill it from a single
/// span-restricted pass — is where it was born and is still its only
/// producer here.
#[derive(Clone, Copy, Debug)]
pub struct SurfaceJet<T: Real> {
    /// The surface point `S(u, v)`.
    pub point: Point3<T>,
    /// `∂S/∂u`.
    pub du: Vec3<T>,
    /// `∂S/∂v`.
    pub dv: Vec3<T>,
    /// `∂²S/∂u²`.
    pub duu: Vec3<T>,
    /// `∂²S/∂u∂v`.
    pub duv: Vec3<T>,
    /// `∂²S/∂v²`.
    pub dvv: Vec3<T>,
}

/// [`SurfaceJet`] extended to **third** order — the ten partials with
/// `k + l ≤ 3` (M5 PR 7).
///
/// # Why third order exists
///
/// Hoffmann's SSI stepper (§6.2, §6.3.2) is a **third-order** Taylor
/// approximant of the local parameterization, so the ℝ⁴
/// parametric×parametric trace needs `d³/ds³ G(u(s), v(s))`, whose
/// chain rule reaches `∂³S`. Nothing else in the kernel does: this jet
/// is the marcher's substrate, computed once per step.
///
/// The `k + l ≤ 2` entries are computed by **exactly the expressions
/// [`NurbsSurface::ders_in_span`] uses**, in the same order, so
/// [`NurbsSurface::ders3_in_span`] and [`NurbsSurface::ders_in_span`]
/// agree **bit for bit** on their common fields (pinned by test) — a
/// second implementation of the same quantity would otherwise be a
/// silent D9 fork.
#[derive(Clone, Copy, Debug)]
pub struct SurfaceJet3<T: Real> {
    /// The second-order jet: point and all partials with `k + l ≤ 2`.
    pub jet: SurfaceJet<T>,
    /// `∂³S/∂u³`.
    pub duuu: Vec3<T>,
    /// `∂³S/∂u²∂v`.
    pub duuv: Vec3<T>,
    /// `∂³S/∂u∂v²`.
    pub duvv: Vec3<T>,
    /// `∂³S/∂v³`.
    pub dvvv: Vec3<T>,
}

/// The all-poison vector — the refusal payload the three `_in_span`
/// doors return for a window this surface does not admit
/// ([`NurbsSurface::admits`]), matching the curve doors' shape.
fn poison_vec<T: Real>() -> Vec3<T> {
    let nan = T::from_f64(f64::NAN);
    Vec3::new(nan, nan, nan)
}

/// The all-poison second-order jet.
fn poison_jet<T: Real>() -> SurfaceJet<T> {
    let d = poison_vec();
    SurfaceJet {
        point: net::poison_point::<T, Point3<T>>(),
        du: d,
        dv: d,
        duu: d,
        duv: d,
        dvv: d,
    }
}

/// The all-poison third-order jet.
fn poison_jet3<T: Real>() -> SurfaceJet3<T> {
    let d = poison_vec();
    SurfaceJet3 {
        jet: poison_jet(),
        duuu: d,
        duuv: d,
        duvv: d,
        dvvv: d,
    }
}

/// The tensor-product control window a span PAIR selects, together
/// with the row-major layout that flattens it — `geom_core`'s
/// [`Span`] one dimension up.
///
/// A `Span` is a proof about one knot vector: in range, nonempty,
/// carrying `index − degree`. What a surface evaluator additionally
/// needs is the *layout*: the `(iu, iv) ↦ iu·nv + iv` stride that
/// turns the two windows into flat indices of `control`/`weights`.
/// That stride was prose at seven separate sites; here it is a field,
/// derived once from `knots_v.control_count()` and never passed in by
/// a caller.
///
/// So the window carries three facts that used to be re-derived per
/// evaluation, per basis term:
///
/// - `span_u − pu` and `span_v − pv`, subtracted once at construction
///   (the `Span` invariant — no use site can underflow them);
/// - `base = (span_u − pu)·nv + (span_v − pv)`, the flat index of the
///   window's corner;
/// - `stride = nv`, so a row step is one addition.
///
/// Evaluation then reads `base + i·stride + j` for
/// `(i, j) ∈ [0, pu] × [0, pv]`, which is exactly `[0, nu) × [0, nv)`
/// flattened — for the surface the window was minted from. That last
/// clause is the whole of [`NurbsSurface::admits`]'s job, and it is
/// why the per-basis-term arithmetic below is free of guards while the
/// door above it has one.
///
/// `Copy`, four `usize`s wide, allocation-free, built once per
/// evaluation. That is deliberate: PR #447 measured a 2.4–2.8×
/// regression from a window abstraction that allocated per basis
/// term, and this one is shaped so it cannot.
///
/// **Not branded to its surface — the pairing is checked, not
/// structural**, exactly as [`Span`]'s is. A window is a plain value
/// with no borrow of the surface it was minted from, so one built from
/// surface A is a representable argument to surface B's evaluators.
/// Each of the three therefore asks [`NurbsSurface::admits`] first and
/// answers a window this surface does not admit with **all-poison**,
/// never with an out-of-bounds index into `control`/`weights` (D9: the
/// kernel never panics on any input).
///
/// [`NurbsSurface::admits`] is [`geom_core::spline::KnotVector::admits`]
/// in both directions plus `stride == knots_v.control_count()`, and
/// what those three tests establish is exactly this: an admitted
/// window is **bit-identical to the one this surface would have minted
/// for the same span pair**, so every `row(i) + j` the evaluators read
/// is inside this net.
///
/// **What they do not establish**, stated rather than implied away:
/// they relate the window to this surface's *shape*, never to its knot
/// values. A window from a different surface whose two degrees, two
/// control counts and two span indices are all nonempty-and-in-range
/// here is admitted, and evaluation against it is a **wrong answer
/// rather than a refusal**. That is the same species of residue
/// [`geom_core::spline::KnotVector::admits`] leaves one dimension
/// down, and closing it wants the invariant-lifetime brand `Span`
/// deliberately does not pay for either. Every consumer
/// in this workspace still builds the window from the surface it
/// evaluates, through [`NurbsSurface::window`] or
/// [`NurbsSurface::window_at`] — the two public mints, both of which
/// take indices or parameters rather than spans.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SurfaceWindow {
    span_u: Span,
    span_v: Span,
    base: usize,
    stride: usize,
}

impl SurfaceWindow {
    /// The u-direction span.
    pub fn span_u(self) -> Span {
        self.span_u
    }

    /// The v-direction span.
    pub fn span_v(self) -> Span {
        self.span_v
    }

    /// The row-major stride — the v control count of the surface this
    /// window was built from.
    pub fn stride(self) -> usize {
        self.stride
    }

    /// The flat index of the window's corner control point,
    /// `(span_u − pu)·stride + (span_v − pv)`.
    pub fn base(self) -> usize {
        self.base
    }

    /// The flat index at which window row `i` starts —
    /// `base + i·stride`. Evaluation hoists this out of its inner
    /// loop, so the inner loop is `row(i) + j`: one addition, no
    /// multiply, and no subtraction anywhere.
    pub fn row(self, i: usize) -> usize {
        self.base + i * self.stride
    }

    /// Whether control point `(iu, iv)` (grid coordinates) is one of
    /// the `(pu + 1)·(pv + 1)` the window names.
    pub fn contains(self, iu: usize, iv: usize) -> bool {
        self.span_u.window().contains(&iu) && self.span_v.window().contains(&iv)
    }
}

/// A validated tensor-product NURBS surface (module docs; immutable
/// after construction — every knot-algebra operation returns a new
/// surface).
#[derive(Clone, Debug)]
pub struct NurbsSurface<T: Real> {
    knots_u: KnotVector,
    knots_v: KnotVector,
    control: Vec<Point3<T>>,
    weights: Vec<f64>,
}

impl<T: Real> NurbsSurface<T> {
    /// Validated construction: `control.len()` must equal
    /// `knots_u.control_count() · knots_v.control_count()` (row-major
    /// layout, module docs), weights match and are strictly positive
    /// and finite.
    ///
    /// # Errors
    ///
    /// [`SplineError`] naming the exact violation.
    pub fn new(
        knots_u: KnotVector,
        knots_v: KnotVector,
        control: Vec<Point3<T>>,
        weights: Vec<f64>,
    ) -> Result<Self, SplineError> {
        let expected = knots_u.control_count() * knots_v.control_count();
        net::validate_counts(expected, control.len(), &weights)?;
        Ok(Self {
            knots_u,
            knots_v,
            control,
            weights,
        })
    }

    /// The "no description yet" placeholder payload for
    /// [`crate::surfaces::Surface::Nurbs`]: structurally valid (bilinear on
    /// `[0,1]²`, unit weights) with all-poison control points, so
    /// every evaluation is all-poison — bit-for-bit the former unit
    /// placeholder's totality behavior (D4 ¶2: fails every downstream
    /// certification loudly).
    pub fn placeholder() -> Self {
        let p = net::poison_point::<T, Point3<T>>();
        Self {
            knots_u: KnotVector::unit_segment(1),
            knots_v: KnotVector::unit_segment(1),
            control: vec![p; 4],
            weights: vec![1.0; 4],
        }
    }

    /// Is this payload the [`NurbsSurface::placeholder`] — the "no
    /// description yet" state — rather than a described surface?
    ///
    /// The discriminator and the reason it is `all` and not
    /// `any` are the crate docs' totality-and-poison section;
    /// the surface and curve halves answer it identically.
    pub fn is_placeholder(&self) -> bool {
        net::is_placeholder(&self.control)
    }

    /// The u-direction knot vector.
    pub fn knots_u(&self) -> &KnotVector {
        &self.knots_u
    }

    /// The v-direction knot vector.
    pub fn knots_v(&self) -> &KnotVector {
        &self.knots_v
    }

    /// The control net, row-major (`iu · nv + iv` — module docs).
    pub fn control(&self) -> &[Point3<T>] {
        &self.control
    }

    /// The weights, same layout as [`NurbsSurface::control`].
    pub fn weights(&self) -> &[f64] {
        &self.weights
    }

    /// `(nu, nv)` — control counts per direction.
    pub fn control_counts(&self) -> (usize, usize) {
        (self.knots_u.control_count(), self.knots_v.control_count())
    }

    /// Construction from parts whose invariants are ALREADY
    /// established — the door a structural map takes instead of
    /// [`Self::new`], and the one place that says why `new`'s check is
    /// redundant for it: both knot vectors and the weights are a
    /// validated surface's own, carried verbatim, and `control` is that
    /// surface's net mapped POINTWISE, which cannot change its length —
    /// so `control.len()` still equals `nu · nv`, `weights.len()` still
    /// equals `control.len()`, and every weight is still the positive
    /// finite value `new` admitted. The `debug_assert` re-derives the
    /// count agreement (D2 addendum row 5).
    fn from_validated_parts(
        knots_u: KnotVector,
        knots_v: KnotVector,
        control: Vec<Point3<T>>,
        weights: Vec<f64>,
    ) -> Self {
        debug_assert!(
            control.len() == knots_u.control_count() * knots_v.control_count()
                && weights.len() == control.len(),
            "from_validated_parts: a pointwise map changed a count \
             (control {}, knots want {}, weights {})",
            control.len(),
            knots_u.control_count() * knots_v.control_count(),
            weights.len()
        );
        Self {
            knots_u,
            knots_v,
            control,
            weights,
        }
    }

    /// The same surface read at another scalar: `f` applied to every
    /// control coordinate, both knot vectors and the weights carried
    /// over verbatim — `f64` structure at every scalar. Construction
    /// goes through [`Self::from_validated_parts`], which states why no
    /// re-validation is run. The contract is
    /// [`NurbsCurve3::map_scalar`]'s one dimension up: exact whenever
    /// `f` is; what the placeholder and a poisoned net lift to is
    /// argued once, in `crate::scalar_lift`'s module docs.
    ///
    /// [`NurbsCurve3::map_scalar`]: crate::curves::NurbsCurve3::map_scalar
    #[must_use]
    pub fn map_scalar<U: Real>(&self, f: impl Fn(T) -> U) -> NurbsSurface<U> {
        NurbsSurface::from_validated_parts(
            self.knots_u.clone(),
            self.knots_v.clone(),
            self.control.iter().map(|p| p.map(&f)).collect(),
            self.weights.clone(),
        )
    }

    /// The [`SurfaceWindow`] for a span pair already validated against
    /// THIS surface's own knot vectors — the one primitive
    /// constructor, behind [`Self::window`] and [`Self::window_at`].
    ///
    /// The stride is taken from THIS surface, never from the caller, so
    /// a window minted here can never disagree with the net it indexes.
    /// A window minted on a DIFFERENT surface is a separate question,
    /// and it is [`NurbsSurface::admits`]'s, asked at each of the three
    /// doors rather than here.
    ///
    /// The **argument order is load-bearing** and nothing checks it: a
    /// `Span` carries no direction, so the two arguments are
    /// interchangeable to the type system and a swap builds a window
    /// that is wrong rather than refused. That obligation is not one a
    /// caller can be handed, which is why this is private: it is
    /// discharged here, at each mint, against the vector each span was
    /// drawn from.
    fn window_of(&self, span_u: Span, span_v: Span) -> SurfaceWindow {
        let stride = self.knots_v.control_count();
        SurfaceWindow {
            span_u,
            span_v,
            base: span_u.first_control() * stride + span_v.first_control(),
            stride,
        }
    }

    /// Whether `win` may be evaluated against **this** surface: both
    /// of its spans are admitted by this surface's own knot vectors
    /// ([`geom_core::spline::KnotVector::admits`]) and its row-major
    /// stride is this surface's v control count.
    ///
    /// Three tests — two [`geom_core::spline::KnotVector::admits`] calls
    /// and one integer compare, seven integer compares in all — and
    /// they are exactly what the tensor indexing needs. The three
    /// `_in_span` doors read
    /// `base + i·stride + j` for `(i, j) ∈ [0, pu] × [0, pv]`, where
    /// `pu`/`pv` are THIS surface's degrees (the basis rows are sized
    /// from `self.knots_*`, never from the window), so the highest
    /// index read is
    /// `(span_u.first_control() + pu)·stride + span_v.first_control() + pv`.
    /// Degree agreement in each direction turns each `first_control + p`
    /// back into that direction's `span.index()`; `index <= last_span()`
    /// bounds them by `nu − 1` and `nv − 1`; and `stride == nv` makes
    /// the whole expression `span_u.index()·nv + span_v.index()`, at
    /// most `nu·nv − 1` — one below `control.len()`, which
    /// [`NurbsSurface::new`] pins at `nu·nv`. The stride compare is the
    /// term with no one-dimensional analogue and it is **not** implied
    /// by the other two: a window whose spans both fit but whose stride
    /// came from a wider net walks past the end of a shorter row.
    /// Nonemptiness rides along inside
    /// [`geom_core::spline::KnotVector::admits`] — it is not needed for
    /// the bound above, and it is what stops an admitted foreign span
    /// dividing by a zero knot difference here. It also **subsumes**
    /// that predicate's index compare for any vector this crate can
    /// build: every index above `last_span()` sits in the trailing run
    /// of `degree + 1` equal knots a clamped vector ends with, so it is
    /// empty. The index compare is what makes the bound argument
    /// legible and what would still carry it if an unclamped
    /// `KnotVector` ever became constructible; it is not a second
    /// independent filter today.
    ///
    /// Equivalently, and this is the whole guarantee: an admitted
    /// window is bit-identical to `self`'s own window for that span
    /// pair, because `base` is a function of the two `first_control`s
    /// and the stride alone.
    ///
    /// **What it does not decide** is which surface the window came
    /// from — see the residue on [`SurfaceWindow`].
    pub fn admits(&self, win: SurfaceWindow) -> bool {
        self.knots_u.admits(win.span_u())
            && self.knots_v.admits(win.span_v())
            && win.stride() == self.knots_v.control_count()
    }

    /// The window at span indices `(span_u, span_v)`, or `None` when
    /// either index is out of range or names an EMPTY span (interior
    /// knot multiplicity). This is the direct replacement for the
    /// `span_is_nonempty` guard followed by an unvalidated index: the
    /// emptiness check and the window construction are one operation.
    pub fn window(&self, span_u: usize, span_v: usize) -> Option<SurfaceWindow> {
        Some(self.window_of(self.knots_u.span(span_u)?, self.knots_v.span(span_v)?))
    }

    /// The window containing parameters `(u, v)` — total on all of
    /// `f64`² for exactly the reasons [`KnotVector::span_at`] is
    /// (out-of-domain clamps to an end span, NaN lands on the first).
    pub fn window_at(&self, u: f64, v: f64) -> SurfaceWindow {
        self.window_of(self.knots_u.span_at(u), self.knots_v.span_at(v))
    }

    /// The point at `(u, v)` in the given control window — the generic
    /// core (span contract and garbage-out as the curve core). Double
    /// ascending pass (outer `iu`, inner `iv`), then one division per
    /// coordinate.
    ///
    /// A window this surface does not admit ([`NurbsSurface::admits`])
    /// yields the **all-poison** point: a `SurfaceWindow` carries no
    /// borrow of the surface it was minted from, so a foreign one is a
    /// representable input here and the three compares are what keep
    /// `row(i) + j` inside this net. Garbage-in on the PARAMETERS still
    /// gives garbage-out (the polynomial extension of the window's
    /// patch), unchanged.
    pub fn eval_in_span(&self, win: SurfaceWindow, u: T, v: T) -> Point3<T> {
        // The pairing check, before any indexing — see `admits` for
        // why these three tests are what the arithmetic below needs.
        if !self.admits(win) {
            return net::poison_point::<T, Point3<T>>();
        }
        let bu = spline::basis::basis_funs(&self.knots_u, win.span_u(), u);
        let bv = spline::basis::basis_funs(&self.knots_v, win.span_v(), v);
        let (mut x, mut y, mut z, mut w) = (T::zero(), T::zero(), T::zero(), T::zero());
        for (i, bui) in bu.iter().enumerate() {
            // Indexed off the window base, deliberately: the basis row
            // length and the window length are two derivations of the
            // same degree — one degree by the check above — and if they
            // ever disagree indexing PANICS where a `zip` would
            // silently drop control points and return a plausible wrong
            // point (D4; PR #447's reverted revision).
            let row = win.row(i);
            for (j, bvj) in bv.iter().enumerate() {
                let idx = row + j;
                let cw = (*bui * *bvj) * T::from_f64(self.weights[idx]);
                let pt = self.control[idx];
                x = x + cw * pt.x;
                y = y + cw * pt.y;
                z = z + cw * pt.z;
                w = w + cw;
            }
        }
        Point3::new(x / w, y / w, z / w)
    }

    /// Point plus first and second partials at `(u, v)` in the given
    /// control window — one homogeneous tensor pass (basis orders 0..=2
    /// in each direction), then the rational corrections exactly as
    /// written: `S = A₀₀/w₀₀`, `S_u = (A₁₀ − S·w₁₀)/w₀₀` (v symmetric),
    /// `S_uu = (A₂₀ − S·w₂₀ − S_u·w₁₀·2)/w₀₀` (v symmetric),
    /// `S_uv = (A₁₁ − S·w₁₁ − S_u·w₀₁ − S_v·w₁₀)/w₀₀`.
    ///
    /// Same totality contract as [`Self::eval_in_span`]: a window this
    /// surface does not admit yields an all-poison jet.
    pub fn ders_in_span(&self, win: SurfaceWindow, u: T, v: T) -> SurfaceJet<T> {
        // The pairing check, as in [`Self::eval_in_span`].
        if !self.admits(win) {
            return poison_jet();
        }
        let du = spline::basis::ders_basis_funs(&self.knots_u, win.span_u(), u, 2);
        let dv = spline::basis::ders_basis_funs(&self.knots_v, win.span_v(), v, 2);
        // Homogeneous partials A_kl for the six (k, l) with k + l ≤ 2,
        // indexed [k][l]; each lane accumulated in the double
        // ascending pass.
        let mut ax = [[T::zero(); 3]; 3];
        let mut ay = [[T::zero(); 3]; 3];
        let mut az = [[T::zero(); 3]; 3];
        let mut aw = [[T::zero(); 3]; 3];
        for (i, _) in du[0].iter().enumerate() {
            // Indexed off the window base — see `eval_in_span`'s note
            // on why this loop is not a `zip`.
            let row = win.row(i);
            for (j, _) in dv[0].iter().enumerate() {
                let idx = row + j;
                let wf = T::from_f64(self.weights[idx]);
                let pt = self.control[idx];
                for k in 0..3usize {
                    for l in 0..3usize {
                        if k + l > 2 {
                            continue;
                        }
                        let cw = (du[k][i] * dv[l][j]) * wf;
                        ax[k][l] = ax[k][l] + cw * pt.x;
                        ay[k][l] = ay[k][l] + cw * pt.y;
                        az[k][l] = az[k][l] + cw * pt.z;
                        aw[k][l] = aw[k][l] + cw;
                    }
                }
            }
        }
        let two = T::from_f64(2.0);
        let w00 = aw[0][0];
        let s = Point3::new(ax[0][0] / w00, ay[0][0] / w00, az[0][0] / w00);
        let sv3 = Vec3::new(s.x, s.y, s.z);
        let s_u = (Vec3::new(ax[1][0], ay[1][0], az[1][0]) - sv3 * aw[1][0]) / w00;
        let s_v = (Vec3::new(ax[0][1], ay[0][1], az[0][1]) - sv3 * aw[0][1]) / w00;
        let s_uu =
            (Vec3::new(ax[2][0], ay[2][0], az[2][0]) - sv3 * aw[2][0] - s_u * (aw[1][0] * two))
                / w00;
        let s_vv =
            (Vec3::new(ax[0][2], ay[0][2], az[0][2]) - sv3 * aw[0][2] - s_v * (aw[0][1] * two))
                / w00;
        let s_uv = (Vec3::new(ax[1][1], ay[1][1], az[1][1])
            - sv3 * aw[1][1]
            - s_u * aw[0][1]
            - s_v * aw[1][0])
            / w00;
        SurfaceJet {
            point: s,
            du: s_u,
            dv: s_v,
            duu: s_uu,
            duv: s_uv,
            dvv: s_vv,
        }
    }

    /// Point plus **all partials with `k + l ≤ 3`** at `(u, v)` in the
    /// given control window — one homogeneous tensor pass (basis orders
    /// `0..=3` in each direction), then the rational corrections.
    ///
    /// The `k + l ≤ 2` block is written **character for character** as
    /// in [`NurbsSurface::ders_in_span`] so the two agree bit for bit
    /// (D9; pinned by test). The four third-order corrections are the
    /// Book's general rational-derivative recursion (Eq. 4.20 /
    /// A4.4) specialized and written out — each subtraction in a fixed
    /// ascending order:
    ///
    /// ```text
    /// S30 = (A30 − 3·w10·S20 − 3·w20·S10 −   w30·S00) / w00
    /// S21 = (A21 − 2·w10·S11 −   w20·S01 −   w01·S20 − 2·w11·S10 − w21·S00) / w00
    /// S12 = (A12 − 2·w01·S11 −   w02·S10 −   w10·S02 − 2·w11·S01 − w12·S00) / w00
    /// S03 = (A03 − 3·w01·S02 − 3·w02·S01 −   w03·S00) / w00
    /// ```
    pub fn ders3_in_span(&self, win: SurfaceWindow, u: T, v: T) -> SurfaceJet3<T> {
        // The pairing check, as in [`Self::eval_in_span`]; the poison
        // jet's `k + l ≤ 2` block is [`Self::ders_in_span`]'s own, so
        // the two agree on the refusal path as they do on every other.
        if !self.admits(win) {
            return poison_jet3();
        }
        let du = spline::basis::ders_basis_funs(&self.knots_u, win.span_u(), u, 3);
        let dv = spline::basis::ders_basis_funs(&self.knots_v, win.span_v(), v, 3);
        // Homogeneous partials A_kl for the ten (k, l) with k + l ≤ 3,
        // indexed [k][l]; each lane accumulated in the double
        // ascending pass (the second-order pass's shape, one order up).
        let mut ax = [[T::zero(); 4]; 4];
        let mut ay = [[T::zero(); 4]; 4];
        let mut az = [[T::zero(); 4]; 4];
        let mut aw = [[T::zero(); 4]; 4];
        for (i, _) in du[0].iter().enumerate() {
            // Indexed off the window base — see `eval_in_span`'s note
            // on why this loop is not a `zip`.
            let row = win.row(i);
            for (j, _) in dv[0].iter().enumerate() {
                let idx = row + j;
                let wf = T::from_f64(self.weights[idx]);
                let pt = self.control[idx];
                for k in 0..4usize {
                    for l in 0..4usize {
                        if k + l > 3 {
                            continue;
                        }
                        let cw = (du[k][i] * dv[l][j]) * wf;
                        ax[k][l] = ax[k][l] + cw * pt.x;
                        ay[k][l] = ay[k][l] + cw * pt.y;
                        az[k][l] = az[k][l] + cw * pt.z;
                        aw[k][l] = aw[k][l] + cw;
                    }
                }
            }
        }
        let two = T::from_f64(2.0);
        let three = T::from_f64(3.0);
        let w00 = aw[0][0];
        // ---- k + l ≤ 2: verbatim `ders_in_span`, for bit-identity ----
        let s = Point3::new(ax[0][0] / w00, ay[0][0] / w00, az[0][0] / w00);
        let sv3 = Vec3::new(s.x, s.y, s.z);
        let s_u = (Vec3::new(ax[1][0], ay[1][0], az[1][0]) - sv3 * aw[1][0]) / w00;
        let s_v = (Vec3::new(ax[0][1], ay[0][1], az[0][1]) - sv3 * aw[0][1]) / w00;
        let s_uu =
            (Vec3::new(ax[2][0], ay[2][0], az[2][0]) - sv3 * aw[2][0] - s_u * (aw[1][0] * two))
                / w00;
        let s_vv =
            (Vec3::new(ax[0][2], ay[0][2], az[0][2]) - sv3 * aw[0][2] - s_v * (aw[0][1] * two))
                / w00;
        let s_uv = (Vec3::new(ax[1][1], ay[1][1], az[1][1])
            - sv3 * aw[1][1]
            - s_u * aw[0][1]
            - s_v * aw[1][0])
            / w00;
        // ---- k + l = 3 ----
        let s_uuu = (Vec3::new(ax[3][0], ay[3][0], az[3][0])
            - s_uu * (aw[1][0] * three)
            - s_u * (aw[2][0] * three)
            - sv3 * aw[3][0])
            / w00;
        let s_uuv = (Vec3::new(ax[2][1], ay[2][1], az[2][1])
            - s_uv * (aw[1][0] * two)
            - s_v * aw[2][0]
            - s_uu * aw[0][1]
            - s_u * (aw[1][1] * two)
            - sv3 * aw[2][1])
            / w00;
        let s_uvv = (Vec3::new(ax[1][2], ay[1][2], az[1][2])
            - s_uv * (aw[0][1] * two)
            - s_u * aw[0][2]
            - s_vv * aw[1][0]
            - s_v * (aw[1][1] * two)
            - sv3 * aw[1][2])
            / w00;
        let s_vvv = (Vec3::new(ax[0][3], ay[0][3], az[0][3])
            - s_vv * (aw[0][1] * three)
            - s_v * (aw[0][2] * three)
            - sv3 * aw[0][3])
            / w00;
        SurfaceJet3 {
            jet: SurfaceJet {
                point: s,
                du: s_u,
                dv: s_v,
                duu: s_uu,
                duv: s_uv,
                dvv: s_vv,
            },
            duuu: s_uuu,
            duuv: s_uuv,
            duvv: s_uvv,
            dvvv: s_vvv,
        }
    }

    /// The transposed surface: `u` and `v` swapped (knot vectors
    /// swapped, grid re-indexed). Involutive; the conjugation that
    /// gives every v-direction knot-algebra op from its u-direction
    /// implementation.
    pub fn transposed(&self) -> Self {
        let (nu, nv) = self.control_counts();
        let mut control = Vec::with_capacity(self.control.len());
        let mut weights = Vec::with_capacity(self.weights.len());
        for iv in 0..nv {
            for iu in 0..nu {
                // Indexing justified: iu < nu, iv < nv (construction).
                control.push(self.control[iu * nv + iv]);
                weights.push(self.weights[iu * nv + iv]);
            }
        }
        Self {
            knots_u: self.knots_v.clone(),
            knots_v: self.knots_u.clone(),
            control,
            weights,
        }
    }

    /// Extracts v-column `iv` as a (points, weights) pair — a curve in
    /// the u direction.
    fn u_column(&self, iv: usize) -> (Vec<Point3<T>>, Vec<f64>) {
        let (nu, nv) = self.control_counts();
        let mut pts = Vec::with_capacity(nu);
        let mut w = Vec::with_capacity(nu);
        for iu in 0..nu {
            pts.push(self.control[iu * nv + iv]);
            w.push(self.weights[iu * nv + iv]);
        }
        (pts, w)
    }

    /// Rebuilds a surface from per-column (points, weights) with a new
    /// u knot vector (columns share it by construction).
    fn from_u_columns(
        knots_u: KnotVector,
        knots_v: KnotVector,
        cols: Vec<(Vec<Point3<T>>, Vec<f64>)>,
    ) -> Self {
        let nu = knots_u.control_count();
        let nv = knots_v.control_count();
        let mut control = Vec::with_capacity(nu * nv);
        let mut weights = Vec::with_capacity(nu * nv);
        for iu in 0..nu {
            for (pts, w) in &cols {
                // Indexing justified: every column has nu entries (the
                // shared plan chain fixes the length).
                control.push(pts[iu]);
                weights.push(w[iu]);
            }
        }
        Self {
            knots_u,
            knots_v,
            control,
            weights,
        }
    }

    /// Applies one shared-schedule plan chain builder per v-column
    /// (module docs: per-column weights ⇒ per-column λs, shared knot
    /// schedule) and reassembles the grid.
    fn map_u_columns(
        &self,
        build: impl Fn(&KnotVector, &[f64]) -> Result<Vec<spline::CurvePlan>, KnotAlgebraError>,
    ) -> Result<Self, KnotAlgebraError> {
        let (_, nv) = self.control_counts();
        let mut cols = Vec::with_capacity(nv);
        let mut new_knots_u = self.knots_u.clone();
        for iv in 0..nv {
            let (mut pts, col_w) = self.u_column(iv);
            let plans = build(&self.knots_u, &col_w)?;
            let mut w = col_w;
            for plan in &plans {
                pts = plan.apply_points(&pts, net::poison_point::<T, Point3<T>>(), |x, y, l| {
                    x.lerp(y, T::from_f64(l))
                });
                w = plan.weights().to_vec();
            }
            if let Some(last) = plans.last() {
                new_knots_u = last.knots().clone();
            }
            cols.push((pts, w));
        }
        Ok(Self::from_u_columns(
            new_knots_u,
            self.knots_v.clone(),
            cols,
        ))
    }

    /// u-direction knot insertion (§5.2, per v-column). Evaluation-
    /// invariant in ℝ.
    ///
    /// # Errors
    ///
    /// As the curve op ([`geom_core::spline::algebra::insert_knot_plan`]).
    pub fn insert_knot_u(&self, u: f64, times: usize) -> Result<Self, KnotAlgebraError> {
        self.map_u_columns(|kv, w| spline::algebra::insert_knot_plan(kv, w, u, times))
    }

    /// v-direction knot insertion — the u op conjugated by transpose.
    ///
    /// # Errors
    ///
    /// As [`NurbsSurface::insert_knot_u`].
    pub fn insert_knot_v(&self, v: f64, times: usize) -> Result<Self, KnotAlgebraError> {
        Ok(self.transposed().insert_knot_u(v, times)?.transposed())
    }

    /// u-direction refinement (§5.3, ascending fold of insertions).
    ///
    /// # Errors
    ///
    /// As [`NurbsSurface::insert_knot_u`], against cumulative structure.
    pub fn refine_knots_u(&self, add: &[f64]) -> Result<Self, KnotAlgebraError> {
        self.map_u_columns(|kv, w| spline::algebra::refine_plan(kv, w, add))
    }

    /// v-direction refinement.
    ///
    /// # Errors
    ///
    /// As [`NurbsSurface::refine_knots_u`].
    pub fn refine_knots_v(&self, add: &[f64]) -> Result<Self, KnotAlgebraError> {
        Ok(self.transposed().refine_knots_u(add)?.transposed())
    }

    /// u-direction degree elevation (§5.5, Bézier route per column).
    ///
    /// # Errors
    ///
    /// As the curve op ([`geom_core::spline::algebra::elevate_plan`]).
    pub fn elevate_degree_u(&self, raise: usize) -> Result<Self, KnotAlgebraError> {
        let mut cur = self.clone();
        for _ in 0..raise {
            cur = cur.map_u_columns(spline::algebra::elevate_plan)?;
        }
        Ok(cur)
    }

    /// v-direction degree elevation.
    ///
    /// # Errors
    ///
    /// As [`NurbsSurface::elevate_degree_u`].
    pub fn elevate_degree_v(&self, raise: usize) -> Result<Self, KnotAlgebraError> {
        Ok(self.transposed().elevate_degree_u(raise)?.transposed())
    }

    /// u-direction bounded knot removal (§5.4): removes `times` copies
    /// of the interior u-knot and returns the surface **with a
    /// sup-norm bound** on `|S − Ŝ|` over the whole domain — the
    /// curve bound's mechanism lifted to the grid (partition of unity
    /// holds in both directions, so the per-pass projected bound uses
    /// grid-wide maxima; passes add). See
    /// [`crate::curves::nurbs::NurbsCurve3::remove_knot`] for the
    /// projective derivation.
    ///
    /// # Errors
    ///
    /// As the curve op (`KnotNotPresent`, multiplicity, weight
    /// collapse, structure).
    pub fn remove_knot_u(&self, u: f64, times: usize) -> Result<(Self, T), KnotAlgebraError> {
        let mut cur = self.clone();
        let mut bound = T::zero();
        for _ in 0..times {
            // One pass per iteration: per-column removal plan plus its
            // exact reinsertion for the perturbation measurement.
            let (_, nv) = cur.control_counts();
            let mut removed_cols = Vec::with_capacity(nv);
            let mut reinserted_cols = Vec::with_capacity(nv);
            let mut new_knots_u = cur.knots_u.clone();
            for iv in 0..nv {
                let (pts, col_w) = cur.u_column(iv);
                let steps = spline::algebra::remove_knot_plan(&cur.knots_u, &col_w, u, 1)?;
                // remove_knot_plan(times = 1) yields exactly one step;
                // the refusal arm is unreachable but keeps this total
                // without an unwrap (D9: no panic path).
                let step = steps
                    .into_iter()
                    .next()
                    .ok_or(KnotAlgebraError::KnotNotPresent { u })?;
                let lerp = |x: Point3<T>, y: Point3<T>, l: f64| x.lerp(y, T::from_f64(l));
                let rem_pts =
                    step.plan
                        .apply_points(&pts, net::poison_point::<T, Point3<T>>(), lerp);
                let re_pts =
                    step.reinsert
                        .apply_points(&rem_pts, net::poison_point::<T, Point3<T>>(), lerp);
                new_knots_u = step.plan.knots().clone();
                removed_cols.push((rem_pts, step.plan.weights().to_vec()));
                reinserted_cols.push((re_pts, step.reinsert.weights().to_vec()));
            }
            let reinserted =
                Self::from_u_columns(cur.knots_u.clone(), cur.knots_v.clone(), reinserted_cols);
            bound = bound
                + net::removal_pass_bound(
                    (&cur.control, &cur.weights),
                    (&reinserted.control, &reinserted.weights),
                );
            cur = Self::from_u_columns(new_knots_u, cur.knots_v.clone(), removed_cols);
        }
        Ok((cur, bound))
    }

    /// v-direction bounded knot removal.
    ///
    /// # Errors
    ///
    /// As [`NurbsSurface::remove_knot_u`].
    pub fn remove_knot_v(&self, v: f64, times: usize) -> Result<(Self, T), KnotAlgebraError> {
        let (s, b) = self.transposed().remove_knot_u(v, times)?;
        Ok((s.transposed(), b))
    }
}

impl<T: SpanLocate> NurbsSurface<T> {
    /// The full jet at `(u, v)`: span selection per direction through
    /// the sealed [`SpanLocate`] seam, the core per overlapped span
    /// cell, channel-independent hulls across cells for
    /// interval-natured scalars (rectangle iteration: ascending u
    /// spans outer, v spans inner).
    pub fn ders(&self, u: T, v: T) -> SurfaceJet<T> {
        let su = u.locate_spans(&self.knots_u);
        let sv = v.locate_spans(&self.knots_v);
        // The seed cell's window comes straight from the located
        // spans, which ARE proofs — no re-validation.
        let (su_first, su_last) = (su.first.index(), su.last.index());
        let (sv_first, sv_last) = (sv.first.index(), sv.last.index());
        let mut acc = self.ders_in_span(self.window_of(su.first, sv.first), u, v);
        for cu in su_first..=su_last {
            for cv in sv_first..=sv_last {
                if cu == su_first && cv == sv_first {
                    continue;
                }
                // Empty spans (interior multiplicity) have no window,
                // so the skip and the validation are one operation:
                // find_span assigns every parameter — a repeated knot
                // value included — to the nonempty span starting at
                // it, which the rectangle always covers, so nothing
                // is discarded (containment preserved); an empty span
                // would only contribute poison (zero denominators).
                let Some(win) = self.window(cu, cv) else {
                    continue;
                };
                let jet = self.ders_in_span(win, u, v);
                acc = SurfaceJet {
                    point: hull_point(acc.point, jet.point),
                    du: hull_vec(acc.du, jet.du),
                    dv: hull_vec(acc.dv, jet.dv),
                    duu: hull_vec(acc.duu, jet.duu),
                    duv: hull_vec(acc.duv, jet.duv),
                    dvv: hull_vec(acc.dvv, jet.dvv),
                };
            }
        }
        acc
    }

    /// The third-order jet at `(u, v)` — [`NurbsSurface::ders`]'s span
    /// selection and channel-independent cell hulling, one order up
    /// (M5 PR 7's ℝ⁴ trace).
    pub fn ders3(&self, u: T, v: T) -> SurfaceJet3<T> {
        let su = u.locate_spans(&self.knots_u);
        let sv = v.locate_spans(&self.knots_v);
        // Seed window and empty-span skip: see [`NurbsSurface::ders`].
        let (su_first, su_last) = (su.first.index(), su.last.index());
        let (sv_first, sv_last) = (sv.first.index(), sv.last.index());
        let mut acc = self.ders3_in_span(self.window_of(su.first, sv.first), u, v);
        for cu in su_first..=su_last {
            for cv in sv_first..=sv_last {
                if cu == su_first && cv == sv_first {
                    continue;
                }
                let Some(win) = self.window(cu, cv) else {
                    continue;
                };
                let j = self.ders3_in_span(win, u, v);
                acc = SurfaceJet3 {
                    jet: SurfaceJet {
                        point: hull_point(acc.jet.point, j.jet.point),
                        du: hull_vec(acc.jet.du, j.jet.du),
                        dv: hull_vec(acc.jet.dv, j.jet.dv),
                        duu: hull_vec(acc.jet.duu, j.jet.duu),
                        duv: hull_vec(acc.jet.duv, j.jet.duv),
                        dvv: hull_vec(acc.jet.dvv, j.jet.dvv),
                    },
                    duuu: hull_vec(acc.duuu, j.duuu),
                    duuv: hull_vec(acc.duuv, j.duuv),
                    duvv: hull_vec(acc.duvv, j.duvv),
                    dvvv: hull_vec(acc.dvvv, j.dvvv),
                };
            }
        }
        acc
    }

    /// The point at `(u, v)` (span selection as [`NurbsSurface::ders`];
    /// point-only pass).
    pub fn eval(&self, u: T, v: T) -> Point3<T> {
        let su = u.locate_spans(&self.knots_u);
        let sv = v.locate_spans(&self.knots_v);
        // Seed window and empty-span skip: see [`NurbsSurface::ders`].
        let (su_first, su_last) = (su.first.index(), su.last.index());
        let (sv_first, sv_last) = (sv.first.index(), sv.last.index());
        let mut acc = self.eval_in_span(self.window_of(su.first, sv.first), u, v);
        for cu in su_first..=su_last {
            for cv in sv_first..=sv_last {
                if cu == su_first && cv == sv_first {
                    continue;
                }
                let Some(win) = self.window(cu, cv) else {
                    continue;
                };
                acc = hull_point(acc, self.eval_in_span(win, u, v));
            }
        }
        acc
    }
}

impl<T: geom_core::CertifiedBounds> NurbsSurface<T> {
    /// The control net lifted to ring points — the data-in shape of
    /// `geom_core::spline::compose::tensor`: channel `d`, control
    /// index `i` in the row-major `iu·nv + iv` layout, as `[x, y, z]`
    /// channels of ring enclosures. Pair with
    /// [`Self::knots_u`]/[`Self::knots_v`]/[`Self::weights`] to build a
    /// `SurfaceRingData` for composite residual bounds. The rank does
    /// not enter the lift, so this is the same body the curves use
    /// (`net::ring_coords`).
    pub fn ring_coords(&self) -> Vec<Vec<geom_core::RingInterval>> {
        net::ring_coords(&self.control)
    }
}

/// Channel-independent point hull (the seam's multi-span combination).
fn hull_point<T: SpanLocate>(a: Point3<T>, b: Point3<T>) -> Point3<T> {
    Point3::new(
        a.x.enclosure_hull(b.x),
        a.y.enclosure_hull(b.y),
        a.z.enclosure_hull(b.z),
    )
}

/// Channel-independent vector hull.
fn hull_vec<T: SpanLocate>(a: Vec3<T>, b: Vec3<T>) -> Vec3<T> {
    Vec3::new(
        a.x.enclosure_hull(b.x),
        a.y.enclosure_hull(b.y),
        a.z.enclosure_hull(b.z),
    )
}
