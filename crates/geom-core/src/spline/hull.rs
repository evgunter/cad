//! **Control-coefficient hull bounds** — the C2.2 sup-norm mechanism,
//! built on the C9 ring (M5 PR 2). Data in, bounds out: every door
//! here reads coefficient brackets and knot structure through one
//! borrow and returns a [`RingInterval`] enclosing the spline's
//! *values*, with no evaluation and no sampling anywhere.
//!
//! # Why a hull is a bound (the convexity fact)
//!
//! On a span `s` of a degree-`p` clamped knot vector, only the `p + 1`
//! basis functions `N_{s−p,p} … N_{s,p}` are nonzero, they are
//! **nonnegative**, and they **sum to one** (partition of unity). So
//! for a scalar spline `f(t) = Σ_j N_{j,p}(t) · c_j`, every value on
//! the span is a convex combination of `c_{s−p} … c_s`, and therefore
//! lies in their hull:
//!
//! ```text
//! min_{j∈[s−p, s]} c_j  ≤  f(t)  ≤  max_{j∈[s−p, s]} c_j   for t ∈ [u_s, u_{s+1})
//! ```
//!
//! This is the mechanism behind the Book's knot-removal bounds
//! (Eq. 9.81, p. 427) that `crates/geom-brep/README.md` C2.2 cites, and it
//! is what turns a *sampled* max into a *certified* sup-norm: sampling
//! governs the fit, the hull bound certifies it. The bound is not
//! merely sound but **tight in the useful sense**: the enclosure is
//! exactly the hull of the coefficients, so it can never be vacuously
//! wide — refining knots shrinks it (the variation-diminishing /
//! convergence-under-refinement property).
//!
//! # The pairing (SPLINE-DESIGN S1, one level down)
//!
//! A coefficient array is a proof about the knot vector it was fitted
//! or composed against, so it travels with that vector: a
//! [`SplineCoeffs`] borrows both, is minted only by
//! [`KnotVector::coeffs`] and [`KnotVector::coeffs_rational`], and a
//! span of that vector is taken FROM the pair — [`SplineCoeffs::span`]
//! and [`SplineCoeffs::span_at`] mint a [`CoeffWindow`], which holds the
//! pair beside a [`Span`] of ITS vector. "A span of another vector
//! against these coefficients" has no spelling, and no door here takes
//! a coefficient array beside anything.
//!
//! The one relation a mint checks is the count: an array that is not
//! `control_count()` long is refused (`None`) at the mint, once, and
//! that bound is what keeps every window `[index − degree, index]`
//! inside the array — so nothing here can index out of range and no
//! door here panics (D9). The doors themselves carry no refusal for
//! the pairing: the state one would refuse is not representable.
//!
//! # Rational splines and the positive-weight precondition
//!
//! For a NURBS `C(t) = Σ N_j w_j P_j / Σ N_j w_j`, the rational basis
//! `R_j = N_j w_j / Σ_k N_k w_k` is *also* a nonnegative partition of
//! unity — **provided every weight is strictly positive**, which is
//! exactly what makes the denominator positive and each term
//! nonnegative. The hull of the control values is then a bound on the
//! rational curve too, with no extra arithmetic. The precondition is a
//! type invariant upstream (`SplineError::NonPositiveWeight`,
//! `spline::algebra`, `geom::curves::nurbs`), but the rational doors
//! here **re-check it on the span's weights** rather than trust the
//! slice the mint was handed: the check is three `f64` comparisons on
//! structure, and the alternative is an unsound bound. It stays a
//! per-span check rather than a mint-time refusal because it is a
//! *value* precondition of the claim on exactly the weights a window
//! reads — a bad weight poisons the windows that read it and no other
//! — where the count is a *pairing* fact about the whole array, which
//! is the mint's business. A pair minted without weights licenses no
//! rational claim, and its rational doors answer poison for that
//! reason.
//!
//! # What is deliberately not here
//!
//! - **No `f ∘ C` composition.** Turning an implicit residual into
//!   B-spline coefficient form is the fitting side's work (M5 PR 4/7);
//!   building it now would guess its shape. These primitives consume
//!   whatever coefficient brackets that work produces.
//! - **No rational derivative coefficients.** The knot-difference
//!   formula in [`SplineCoeffs::derivative_coeffs`] is the nonrational
//!   one; a rational derivative needs the homogeneous curve plus a
//!   quotient rule, whose enclosure belongs with the consumer that owns
//!   the homogeneous form.
//! - **No comparisons on a generic scalar.** The coefficient type is
//!   read only through [`CertifiedEnclosure`] — one fallible bracket
//!   accessor — so nothing here can accidentally decide anything about an
//!   evaluation scalar.
//!
//! # Poison (fail-loud, D4 ¶2)
//!
//! Every checkable structural error — a non-positive weight, a
//! non-positive knot difference — yields a **poisoned bound**, and a
//! poisoned coefficient poisons every bound it participates in. A
//! poisoned bound fails `residual.hi() <= eps` under every comparison
//! direction, so a structural mistake can never be mistaken for a
//! certificate. A coefficient/weight count mismatch is not a poison
//! route: it is refused at the mint, before there is a door to answer.

use super::knots::{KnotVector, Span};
use crate::real::CertifiedEnclosure;
use crate::ring_interval::RingInterval;

/// A coefficient array **with the knot vector it is a proof about**,
/// and optionally the weights that license a rational claim on it.
///
/// Minted only by [`KnotVector::coeffs`] and
/// [`KnotVector::coeffs_rational`], which are where the count relation
/// (`coeffs.len() == control_count()`) is checked — once. Every door
/// on this type and on the [`CoeffWindow`]s it mints reads the knots,
/// the coefficients and the weights from this one borrow, so there is
/// no second vector or second array for any of them to disagree with.
///
/// `Copy`: two references (three with weights), allocation-free.
///
/// # The three shapes that have no spelling
///
/// These are library doctests: they run under
/// `cargo test -p geom-core --doc`, so the claims below redden if the
/// borrow is undone rather than merely dating a comment. Each shape
/// is one the length-only relation used to answer finitely and
/// wrongly: two vectors of equal control count, so a length check
/// cannot tell them apart, and a span of the OTHER vector hulled over
/// these coefficients.
///
/// **(a) Same degree, different interior knots.** A window is minted
/// from the pair, by index, and there is no parameter through which a
/// span of another vector could arrive:
///
/// ```compile_fail,E0308
/// use geom_core::spline::KnotVector;
/// let mine = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.25, 0.5, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
/// let theirs = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.5, 0.75, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
/// let coeffs = vec![0.0f64; mine.control_count()];
/// let pair = mine.coeffs(&coeffs).unwrap();
/// let _ = pair.span(theirs.span_at(0.3)).unwrap().hull();
/// ```
///
/// Its twin, differing in one respect — the span is asked of the pair,
/// which draws it from the vector the coefficients belong to:
///
/// ```
/// use geom_core::spline::KnotVector;
/// let mine = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.25, 0.5, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
/// let coeffs = vec![0.0f64; mine.control_count()];
/// let pair = mine.coeffs(&coeffs).unwrap();
/// let _ = pair.span_at(0.3).hull();
/// ```
///
/// **(b) A span whose index is EMPTY in the coefficients' vector.**
/// The struct has no public constructor to put a foreign span beside
/// the pair, so the only way to name index 4 is to ask the pair for
/// it — and the pair's vector refuses it as empty:
///
/// ```compile_fail,E0451
/// use geom_core::spline::{KnotVector, hull::CoeffWindow};
/// let mine = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
/// let theirs = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.25, 0.5, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
/// let coeffs = vec![0.0f64; mine.control_count()];
/// let pair = mine.coeffs(&coeffs).unwrap();
/// let win = CoeffWindow { coeffs: pair, span: theirs.span(4).unwrap() };
/// ```
///
/// The twin differs in where the span comes from, and its answer is
/// the refusal a length check could never have given:
///
/// ```
/// use geom_core::spline::KnotVector;
/// let mine = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
/// let coeffs = vec![0.0f64; mine.control_count()];
/// let pair = mine.coeffs(&coeffs).unwrap();
/// assert!(pair.span(4).is_none());
/// ```
///
/// **(c) A span of a DIFFERENT degree** — the sharpest, because the
/// basis row it would pair with is a different length from this
/// vector's. The window doors take no span at all, so there is
/// nothing to hand one to:
///
/// ```compile_fail,E0061
/// use geom_core::spline::KnotVector;
/// let mine = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
/// let quad = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.3, 0.6, 0.8, 1.0, 1.0, 1.0], 2).unwrap();
/// let coeffs = vec![0.0f64; mine.control_count()];
/// let pair = mine.coeffs(&coeffs).unwrap();
/// let _ = pair.span_at(0.7).sup_norm_bound(quad.span_at(0.7));
/// ```
///
/// The twin differs in one respect — the door is called with nothing:
///
/// ```
/// use geom_core::spline::KnotVector;
/// let mine = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
/// let coeffs = vec![0.0f64; mine.control_count()];
/// let pair = mine.coeffs(&coeffs).unwrap();
/// assert!(pair.span_at(0.7).sup_norm_bound().is_finite());
/// ```
///
/// **What these rows do and do not check.** Stable rustdoc checks only
/// that a `compile_fail` block fails to build; the `,E0308` / `,E0451`
/// / `,E0061` annotation beside it is **not** verified there (that is
/// a nightly rustdoc feature), so a row could be red for a typo
/// instead of for its subject. That is what each twin is for: it
/// differs from its block in exactly one respect and it compiles, so
/// a typo shared by both would redden the twin. The codes themselves
/// were read off `rustc` directly on each snippet at the pinned
/// toolchain.
#[derive(Clone, Copy)]
pub struct SplineCoeffs<'a, E: CertifiedEnclosure> {
    knots: &'a KnotVector,
    coeffs: &'a [E],
    weights: Option<&'a [f64]>,
}

/// The borrows are printed as ADDRESSES, never followed: a derived
/// `Debug` would dump the knot vector and the whole coefficient array
/// through the references at every `{:?}`, which is the one cost a
/// borrow-carrying token can impose by accident. The addresses are
/// also what equality reads.
impl<E: CertifiedEnclosure> core::fmt::Debug for SplineCoeffs<'_, E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SplineCoeffs")
            .field("knots", &core::ptr::from_ref(self.knots))
            .field("coeffs", &self.coeffs.as_ptr())
            .field("len", &self.coeffs.len())
            .field("weights", &self.weights.map(<[f64]>::as_ptr))
            .finish()
    }
}

/// Equality is address equality on the vector and on both arrays: a
/// pair is a proof about *those* coefficients against *that* vector,
/// and neither is [`Eq`] by value (the knots are `f64`).
impl<E: CertifiedEnclosure> PartialEq for SplineCoeffs<'_, E> {
    fn eq(&self, other: &Self) -> bool {
        core::ptr::eq(self.knots, other.knots)
            && core::ptr::eq(self.coeffs, other.coeffs)
            && match (self.weights, other.weights) {
                (None, None) => true,
                (Some(a), Some(b)) => core::ptr::eq(a, b),
                _ => false,
            }
    }
}

impl<E: CertifiedEnclosure> Eq for SplineCoeffs<'_, E> {}

/// A [`SplineCoeffs`] beside a [`Span`] of **its** knot vector — the
/// window every span-restricted hull door reads from. Minted only by
/// [`SplineCoeffs::span`] and [`SplineCoeffs::span_at`], so the span
/// and the coefficients name the same vector by construction.
///
/// `Copy`, one pair and one `Span` wide, allocation-free.
#[derive(Clone, Copy)]
pub struct CoeffWindow<'a, E: CertifiedEnclosure> {
    coeffs: SplineCoeffs<'a, E>,
    span: Span<'a>,
}

/// Address-printed like the pair it holds (see [`SplineCoeffs`]).
impl<E: CertifiedEnclosure> core::fmt::Debug for CoeffWindow<'_, E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CoeffWindow")
            .field("coeffs", &self.coeffs)
            .field("span", &self.span)
            .finish()
    }
}

impl<E: CertifiedEnclosure> PartialEq for CoeffWindow<'_, E> {
    fn eq(&self, other: &Self) -> bool {
        self.coeffs == other.coeffs && self.span == other.span
    }
}

impl<E: CertifiedEnclosure> Eq for CoeffWindow<'_, E> {}

impl KnotVector {
    /// `coeffs` as a proof about **this** vector — `None` unless the
    /// array is exactly [`KnotVector::control_count`] long, which is
    /// the one relation a length can state and the bound that keeps
    /// every window of the pair inside the array. This and
    /// [`KnotVector::coeffs_rational`] are the only ways to obtain a
    /// [`SplineCoeffs`].
    pub fn coeffs<'a, E: CertifiedEnclosure>(
        &'a self,
        coeffs: &'a [E],
    ) -> Option<SplineCoeffs<'a, E>> {
        (coeffs.len() == self.control_count()).then_some(SplineCoeffs {
            knots: self,
            coeffs,
            weights: None,
        })
    }

    /// [`KnotVector::coeffs`] with the weights that license a rational
    /// claim carried beside: `None` unless both arrays are
    /// [`KnotVector::control_count`] long. Weight *positivity* is not
    /// checked here — it is the rational doors' per-window precondition
    /// (module docs), and a bad weight poisons the windows that read it.
    pub fn coeffs_rational<'a, E: CertifiedEnclosure>(
        &'a self,
        coeffs: &'a [E],
        weights: &'a [f64],
    ) -> Option<SplineCoeffs<'a, E>> {
        let n = self.control_count();
        (coeffs.len() == n && weights.len() == n).then_some(SplineCoeffs {
            knots: self,
            coeffs,
            weights: Some(weights),
        })
    }
}

impl<'a, E: CertifiedEnclosure> SplineCoeffs<'a, E> {
    /// The [`KnotVector`] these coefficients are a proof about — the
    /// one every door here reads its knots from.
    pub fn knots(self) -> &'a KnotVector {
        self.knots
    }

    /// The coefficient brackets, `control_count()` of them.
    pub fn coeffs(self) -> &'a [E] {
        self.coeffs
    }

    /// The weights, when the pair was minted rationally; `None` says
    /// the pair licenses no rational claim.
    pub fn weights(self) -> Option<&'a [f64]> {
        self.weights
    }

    /// The window of this pair at span `index` — `None` when the index
    /// is out of range or names an **empty** span (interior knot
    /// multiplicity), exactly as [`KnotVector::span`] refuses it. The
    /// emptiness check and the window construction are one operation.
    pub fn span(self, index: usize) -> Option<CoeffWindow<'a, E>> {
        Some(CoeffWindow {
            coeffs: self,
            span: self.knots.span(index)?,
        })
    }

    /// The window containing `t` — total on all of `f64` for exactly
    /// the reasons [`KnotVector::span_at`] is (out-of-domain clamps to
    /// an end span, NaN lands on the first).
    pub fn span_at(self, t: f64) -> CoeffWindow<'a, E> {
        CoeffWindow {
            coeffs: self,
            span: self.knots.span_at(t),
        }
    }

    /// Enclosure of the scalar B-spline's values over its whole domain:
    /// the hull of the per-span enclosures, which (every coefficient
    /// being active on some nonempty span) is the hull of all
    /// coefficients.
    ///
    /// Computed span-wise rather than coefficient-wise on purpose — it
    /// is the same value, and it keeps this door's answer
    /// definitionally equal to the granular form subdivision consumers
    /// use.
    pub fn domain_hull(self) -> RingInterval {
        let mut acc = RingInterval::poison();
        let mut seeded = false;
        // Fixed ascending span order (D9).
        for index in self.knots.first_span()..=self.knots.last_span() {
            // Emptiness check and window construction are one step.
            let Some(win) = self.span(index) else {
                continue;
            };
            let h = win.hull();
            acc = if seeded {
                RingInterval::hull(acc, h)
            } else {
                h
            };
            seeded = true;
        }
        acc
    }

    /// Whole-domain enclosure of the rational scalar spline: the hull
    /// over spans of [`CoeffWindow::hull_rational`]. Poison if any
    /// span's weights fail the precondition, or if the pair carries no
    /// weights.
    pub fn domain_hull_rational(self) -> RingInterval {
        let mut acc = RingInterval::poison();
        let mut seeded = false;
        for index in self.knots.first_span()..=self.knots.last_span() {
            // Emptiness check and window construction are one step.
            let Some(win) = self.span(index) else {
                continue;
            };
            let h = win.hull_rational();
            acc = if seeded {
                RingInterval::hull(acc, h)
            } else {
                h
            };
            seeded = true;
        }
        acc
    }

    /// One derivative coefficient of the nonrational scalar B-spline:
    /// `Q_i = p · (c_{i+1} − c_i) / (u_{i+p+1} − u_{i+1})`, the standard
    /// knot-difference formula (Book Eq. 3.7 / A3.3 shape). `Q_i` are
    /// the degree-`p−1` coefficients of `f'` over the knot vector with
    /// the two outer knots dropped.
    ///
    /// The knot difference is **structure** (both knots are `f64`), but
    /// it is formed *in the ring* rather than at `f64`: an `f64`
    /// subtraction is correctly rounded, not exact, and a point
    /// enclosure of a rounded difference would silently drop that error
    /// into the denominator. (Sterbenz makes the subtraction exact for
    /// most knot pairs; the ring pays one ulp for the cases where it is
    /// not.) A knot difference that is not provably positive poisons
    /// the coefficient — the ring's `Div` refuses a zero-touching
    /// divisor, so this cannot leak.
    ///
    /// Fixed association (D9): `(c_{i+1} − c_i) · p / Δu`, exactly as
    /// parenthesized.
    fn deriv_coeff(self, i: usize) -> RingInterval {
        let p = self.knots.degree();
        let u = self.knots.knots();
        let coeffs = self.coeffs;
        // Indexing justified by the caller's range: i + 1 ≤
        // control_count() − 1 = u.len() − p − 2, so i + p + 1 ≤ u.len() − 1.
        if i + 1 >= coeffs.len() || i + p + 1 >= u.len() {
            return RingInterval::poison();
        }
        let du = RingInterval::point(u[i + p + 1]) - RingInterval::point(u[i + 1]);
        let dc =
            RingInterval::from_certified(coeffs[i + 1]) - RingInterval::from_certified(coeffs[i]);
        #[allow(clippy::cast_precision_loss)]
        let scale = RingInterval::point(p as f64);
        dc * scale / du
    }

    /// Enclosures of **every** derivative coefficient of the nonrational
    /// scalar B-spline: `control_count() − 1` values, index `i` giving
    /// `Q_i`. A bad knot difference or a poisoned coefficient poisons
    /// that entry. The returned length is never zero: a clamped vector
    /// refuses degree 0 and has at least `2(p + 1)` knots, so the pair
    /// has at least two coefficients and one derivative coefficient.
    ///
    /// See the module docs for why the rational case is not here.
    pub fn derivative_coeffs(self) -> Vec<RingInterval> {
        (0..self.coeffs.len() - 1)
            .map(|i| self.deriv_coeff(i))
            .collect()
    }

    /// Enclosure of the derivative over the whole domain: the hull of
    /// all derivative coefficients.
    pub fn derivative_domain_hull(self) -> RingInterval {
        let qs = self.derivative_coeffs();
        let mut acc = RingInterval::poison();
        for (n, q) in qs.iter().enumerate() {
            acc = if n == 0 {
                *q
            } else {
                RingInterval::hull(acc, *q)
            };
        }
        acc
    }

    /// A certified upper bound on `|f|` over the whole domain. See
    /// [`CoeffWindow::sup_norm_bound`].
    pub fn sup_norm_bound(self) -> f64 {
        self.domain_hull().mag()
    }

    /// The rational counterpart of [`SplineCoeffs::sup_norm_bound`]: an
    /// upper bound on `|f|` over the whole domain of the rational scalar
    /// spline, `NaN` unless every weight satisfies the positivity
    /// precondition.
    pub fn sup_norm_bound_rational(self) -> f64 {
        self.domain_hull_rational().mag()
    }
}

impl<'a, E: CertifiedEnclosure> CoeffWindow<'a, E> {
    /// The pair this window is a window of.
    pub fn coeffs(self) -> SplineCoeffs<'a, E> {
        self.coeffs
    }

    /// The knot span this window selects — a span of the pair's own
    /// vector.
    pub fn span(self) -> Span<'a> {
        self.span
    }

    /// The vector both halves name (`SplineCoeffs::knots`).
    pub fn knots(self) -> &'a KnotVector {
        self.coeffs.knots
    }

    /// The span index (`Span::index`).
    pub fn index(self) -> usize {
        self.span.index()
    }

    /// The first coefficient of the window (`Span::first_control`).
    pub fn first_control(self) -> usize {
        self.span.first_control()
    }

    /// The inclusive coefficient window, `Span::window`.
    pub fn window(self) -> core::ops::RangeInclusive<usize> {
        self.span.window()
    }

    /// Enclosure of the scalar B-spline's values over this span, from
    /// the brackets of the `p + 1` coefficients active there (module
    /// docs: the convexity fact).
    ///
    /// Poison for a poisoned coefficient. The result is the
    /// polynomial's bound on `[u_span, u_{span+1}]` only — outside it
    /// the span's polynomial extension is unbounded by anything here.
    ///
    /// The window `[first_control, index]` was computed once at the
    /// span's construction, and `index ≤ last_span() = control_count()
    /// − 1 = coeffs.len() − 1` by the mint, so it indexes in range.
    pub fn hull(self) -> RingInterval {
        let coeffs = self.coeffs.coeffs;
        let (first, last) = (self.span.first_control(), self.span.index());
        let mut acc = RingInterval::poison();
        // Fixed ascending reduction order (D9).
        for (n, j) in (first..=last).enumerate() {
            // Indexing justified: last ≤ control_count() − 1 = coeffs.len() − 1.
            let c = RingInterval::from_certified(coeffs[j]);
            acc = if n == 0 {
                c
            } else {
                RingInterval::hull(acc, c)
            };
        }
        acc
    }

    /// Whether every weight active on this span is strictly positive
    /// and finite — the precondition that licenses the hull bound for
    /// a rational spline (module docs). A pair minted without weights
    /// licenses nothing.
    fn weights_positive(self) -> bool {
        let Some(weights) = self.coeffs.weights else {
            return false;
        };
        let (first, last) = (self.span.first_control(), self.span.index());
        // `w > 0.0` is false for NaN, so a NaN weight refuses (the spline
        // substrate's NaN-catching discipline, in its positive form).
        // Indexing justified: last ≤ control_count() − 1 = weights.len() − 1.
        weights[first..=last]
            .iter()
            .all(|w| *w > 0.0 && w.is_finite())
    }

    /// Enclosure of the **rational** scalar spline's values over this
    /// span, from the brackets of the active control values and their
    /// weights.
    ///
    /// The returned bound is the same hull as [`CoeffWindow::hull`] —
    /// the weights buy no tightness, they buy the *right to make the
    /// claim*: with all active weights strictly positive the rational
    /// basis is a nonnegative partition of unity, so the value is still
    /// a convex combination of the control values. A non-positive or
    /// non-finite weight, or a pair carrying no weights, is poison.
    pub fn hull_rational(self) -> RingInterval {
        if !self.weights_positive() {
            return RingInterval::poison();
        }
        self.hull()
    }

    /// Enclosure of the **derivative** of the nonrational scalar
    /// B-spline over this span of the *original* knot vector.
    ///
    /// Index bookkeeping: dropping the outer knots shifts span indices
    /// by one, so span `s` of the vector is span `s − 1` of the
    /// derivative's knot vector, where the active degree-`(p−1)` basis
    /// indices are `s − p … s − 1`. That is the range of `Q` hulled
    /// here — no derivative [`KnotVector`] needs to be materialised.
    ///
    /// The range is nonempty for every [`Span`]: `first = s − p < s =
    /// last` because `KnotVector::clamped` refuses degree 0. It is the
    /// span's own window minus its top end.
    pub fn derivative_hull(self) -> RingInterval {
        let (first, last) = (self.span.first_control(), self.span.index());
        let mut acc = RingInterval::poison();
        // Fixed ascending reduction order (D9). Range: [span − p, span − 1].
        for (n, i) in (first..last).enumerate() {
            let q = self.coeffs.deriv_coeff(i);
            acc = if n == 0 {
                q
            } else {
                RingInterval::hull(acc, q)
            };
        }
        acc
    }

    /// A certified upper bound on `|f|` over this span — the scalar
    /// sup-norm reading of [`CoeffWindow::hull`], and the shape C2.2's
    /// honesty limb consumes (`window.sup_norm_bound() <= eps`
    /// certifies the span).
    ///
    /// Returns `NaN` for every poison path, which fails that comparison
    /// under every direction (D4 ¶2). The value is an upper bound on
    /// the true supremum, never an approximation of it.
    pub fn sup_norm_bound(self) -> f64 {
        self.hull().mag()
    }
}
