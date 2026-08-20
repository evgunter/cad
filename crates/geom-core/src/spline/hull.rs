//! **Control-coefficient hull bounds** — the C2.2 sup-norm mechanism,
//! built on the C9 ring (M5 PR 2). Data in, bounds out: every function
//! here takes coefficient brackets plus knot structure and returns a
//! [`RingInterval`] enclosing the spline's *values*, with no evaluation
//! and no sampling anywhere.
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
//! (Eq. 9.81, p. 427) that `docs/CURVED-DESIGN.md` C2.2 cites, and it
//! is what turns a *sampled* max into a *certified* sup-norm: sampling
//! governs the fit, the hull bound certifies it. The bound is not
//! merely sound but **tight in the useful sense**: the enclosure is
//! exactly the hull of the coefficients, so it can never be vacuously
//! wide — refining knots shrinks it (the variation-diminishing /
//! convergence-under-refinement property).
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
//! `spline::algebra`, `geom::curves::nurbs`), but the rational entry
//! points here **re-check it on the span's weights** rather than trust
//! a slice handed in by a caller: the check is three `f64` comparisons
//! on structure, and the alternative is an unsound bound.
//!
//! # What is deliberately not here
//!
//! - **No `f ∘ C` composition.** Turning an implicit residual into
//!   B-spline coefficient form is the fitting side's work (M5 PR 4/7);
//!   building it now would guess its shape. These primitives consume
//!   whatever coefficient brackets that work produces.
//! - **No rational derivative coefficients.** The knot-difference
//!   formula in [`derivative_coeffs`] is the nonrational one; a
//!   rational derivative needs the homogeneous curve plus a quotient
//!   rule, whose enclosure belongs with the consumer that owns the
//!   homogeneous form.
//! - **No comparisons on a generic scalar.** The coefficient type is
//!   read only through [`CertifiedEnclosure`] — one fallible bracket
//!   accessor — so nothing here can accidentally decide anything about an
//!   evaluation scalar.
//!
//! # Poison (fail-loud, D4 ¶2)
//!
//! Every checkable structural error — coefficient/weight count
//! mismatch, a non-positive weight, a non-positive knot difference —
//! yields a **poisoned bound**, and a poisoned coefficient poisons
//! every bound it participates in. A poisoned bound fails
//! `residual.hi() <= eps` under every comparison direction, so a
//! structural mistake can never be mistaken for a certificate.
//!
//! A span this `kv` does not admit is on that list too, and it is the
//! **only** span refusal left. A [`Span`] carries "in range and
//! nonempty" for the vector it was drawn from, but it carries no borrow
//! of that vector, so a span of a *different* one is a representable
//! input: [`span_indices`] therefore asks [`KnotVector::admits`]
//! alongside the coefficient-length check, and either failing is
//! poison. Emptiness is not re-checked — an admitted span that is empty
//! in this `kv` poisons through its own zero knot difference.
//!
//! # The two length-only relations, and the one that is a residue
//!
//! Both remaining checks are relations of *length*, not of identity.
//! `coeffs.len() == kv.control_count()` says nothing about which curve
//! the coefficients came from, and [`KnotVector::admits`] says nothing
//! about which vector the span came from: a same-degree, same-control-count
//! vector admits the span, and a same-length `coeffs` passes the count.
//! In both cases the bound is then computed over the wrong data and is
//! **wrong rather than refused** — a silent answer, and a deliberate
//! residue.
//!
//! What is closed is the worse failure: neither can index out of bounds
//! any more, so no public door here panics (D9). Closing the residue
//! means the `Span` and the coefficients carrying their vector — a
//! borrow, or an invariant-lifetime brand — which is a design change
//! and not paid for here.

use super::knots::{KnotVector, Span};
use crate::real::CertifiedEnclosure;
use crate::ring_interval::RingInterval;

/// Reads a coefficient's bracket into the ring.
///
/// The bound is [`CertifiedEnclosure`], not [`crate::Enclosure`]: a hull bound
/// is a *certificate*, so a coefficient that merely carries a bracket is
/// not enough — the computation behind it must also have been defined on
/// the whole input box. A coefficient that refuses becomes ring poison,
/// exactly as a NaN or inverted bracket does, and poison propagates
/// through every bound it participates in (module docs).
///
/// The body is [`RingInterval::from_certified`]; this wrapper exists for
/// the name at the call sites below, not for a second spelling of the
/// door.
fn bracket<E: CertifiedEnclosure>(c: E) -> RingInterval {
    RingInterval::from_certified(c)
}

/// The coefficient index range whose basis functions are nonzero on
/// `span` — the [`Span`]'s own window, `[span − p, span]`, computed
/// once at its construction rather than re-derived here.
///
/// `None` — the poison route for every caller below — on either of the
/// two disagreements a `Span` cannot rule out by itself: a `coeffs`
/// array that is not the length this `kv` requires, or a span this `kv`
/// does not admit ([`KnotVector::admits`]). With both discharged,
/// `last = span.index() <= kv.last_span() = control_count() − 1 =
/// coeff_len − 1` and `first = last − degree >= 0`, so the whole window
/// indexes `coeffs` in range.
fn span_indices(kv: &KnotVector, coeff_len: usize, span: Span) -> Option<(usize, usize)> {
    if coeff_len != kv.control_count() || !kv.admits(span) {
        return None;
    }
    Some((span.first_control(), span.index()))
}

/// Enclosure of a scalar B-spline's values over one span, from the
/// brackets of the `p + 1` coefficients active there (module docs: the
/// convexity fact).
///
/// Poison for a coefficient-count mismatch, a span this `kv` does not
/// admit, or a poisoned coefficient. The result is the polynomial's
/// bound on
/// `[u_span, u_{span+1}]` only — outside it the span's polynomial
/// extension is unbounded by anything here.
pub fn span_hull<E: CertifiedEnclosure>(kv: &KnotVector, coeffs: &[E], span: Span) -> RingInterval {
    let Some((first, last)) = span_indices(kv, coeffs.len(), span) else {
        return RingInterval::poison();
    };
    let mut acc = RingInterval::poison();
    // Fixed ascending reduction order (D9).
    for (n, j) in (first..=last).enumerate() {
        // Indexing justified: last ≤ control_count() − 1 = coeffs.len() − 1.
        let c = bracket(coeffs[j]);
        acc = if n == 0 {
            c
        } else {
            RingInterval::hull(acc, c)
        };
    }
    acc
}

/// Enclosure of a scalar B-spline's values over its whole domain: the
/// hull of the per-span enclosures, which (every coefficient being
/// active on some nonempty span) is the hull of all coefficients.
///
/// Computed span-wise rather than coefficient-wise on purpose — it is
/// the same value, and it keeps this function's answer definitionally
/// equal to the granular form subdivision consumers use.
pub fn domain_hull<E: CertifiedEnclosure>(kv: &KnotVector, coeffs: &[E]) -> RingInterval {
    if coeffs.len() != kv.control_count() {
        return RingInterval::poison();
    }
    let mut acc = RingInterval::poison();
    let mut seeded = false;
    // Fixed ascending span order (D9).
    for index in kv.first_span()..=kv.last_span() {
        // Emptiness check and span validation are one step.
        let Some(span) = kv.span(index) else { continue };
        let h = span_hull(kv, coeffs, span);
        acc = if seeded {
            RingInterval::hull(acc, h)
        } else {
            h
        };
        seeded = true;
    }
    acc
}

/// Whether every weight active on `span` is strictly positive and
/// finite — the precondition that licenses the hull bound for a
/// rational spline (module docs).
fn span_weights_positive(kv: &KnotVector, weights: &[f64], span: Span) -> bool {
    let Some((first, last)) = span_indices(kv, weights.len(), span) else {
        return false;
    };
    // `w > 0.0` is false for NaN, so a NaN weight refuses (the spline
    // substrate's NaN-catching discipline, in its positive form).
    // Indexing justified: last ≤ control_count() − 1 = weights.len() − 1.
    weights[first..=last]
        .iter()
        .all(|w| *w > 0.0 && w.is_finite())
}

/// Enclosure of a **rational** scalar spline's values over one span,
/// from the brackets of the active control values and their weights.
///
/// The returned bound is the same hull as [`span_hull`] — the weights
/// buy no tightness, they buy the *right to make the claim*: with all
/// active weights strictly positive the rational basis is a nonnegative
/// partition of unity, so the value is still a convex combination of
/// the control values. A non-positive, non-finite, or miscounted weight
/// is poison.
pub fn span_hull_rational<E: CertifiedEnclosure>(
    kv: &KnotVector,
    coeffs: &[E],
    weights: &[f64],
    span: Span,
) -> RingInterval {
    if !span_weights_positive(kv, weights, span) {
        return RingInterval::poison();
    }
    span_hull(kv, coeffs, span)
}

/// Whole-domain enclosure of a rational scalar spline: the hull over
/// spans of [`span_hull_rational`]. Poison if any span's weights fail
/// the precondition.
pub fn domain_hull_rational<E: CertifiedEnclosure>(
    kv: &KnotVector,
    coeffs: &[E],
    weights: &[f64],
) -> RingInterval {
    if coeffs.len() != kv.control_count() || weights.len() != kv.control_count() {
        return RingInterval::poison();
    }
    let mut acc = RingInterval::poison();
    let mut seeded = false;
    for index in kv.first_span()..=kv.last_span() {
        // Emptiness check and span validation are one step.
        let Some(span) = kv.span(index) else { continue };
        let h = span_hull_rational(kv, coeffs, weights, span);
        acc = if seeded {
            RingInterval::hull(acc, h)
        } else {
            h
        };
        seeded = true;
    }
    acc
}

/// One derivative coefficient of a nonrational scalar B-spline:
/// `Q_i = p · (c_{i+1} − c_i) / (u_{i+p+1} − u_{i+1})`, the standard
/// knot-difference formula (Book Eq. 3.7 / A3.3 shape). `Q_i` are the
/// degree-`p−1` coefficients of `f'` over the knot vector with the two
/// outer knots dropped.
///
/// The knot difference is **structure** (both knots are `f64`), but it
/// is formed *in the ring* rather than at `f64`: an `f64` subtraction is
/// correctly rounded, not exact, and a point enclosure of a rounded
/// difference would silently drop that error into the denominator.
/// (Sterbenz makes the subtraction exact for most knot pairs; the ring
/// pays one ulp for the cases where it is not.) A knot difference that
/// is not provably positive poisons the coefficient — the ring's `Div`
/// refuses a zero-touching divisor, so this cannot leak.
///
/// Fixed association (D9): `(c_{i+1} − c_i) · p / Δu`, exactly as
/// parenthesized.
fn deriv_coeff<E: CertifiedEnclosure>(kv: &KnotVector, coeffs: &[E], i: usize) -> RingInterval {
    let p = kv.degree();
    let u = kv.knots();
    // Indexing justified by the caller's range check: i + 1 ≤
    // control_count() − 1 = u.len() − p − 2, so i + p + 1 ≤ u.len() − 1.
    if i + 1 >= coeffs.len() || i + p + 1 >= u.len() {
        return RingInterval::poison();
    }
    let du = RingInterval::point(u[i + p + 1]) - RingInterval::point(u[i + 1]);
    let dc = bracket(coeffs[i + 1]) - bracket(coeffs[i]);
    #[allow(clippy::cast_precision_loss)]
    let scale = RingInterval::point(p as f64);
    dc * scale / du
}

/// Enclosures of **every** derivative coefficient of a nonrational
/// scalar B-spline: `control_count() − 1` values, index `i` giving
/// `Q_i`. A bad knot difference or a poisoned coefficient poisons that
/// entry. A coefficient-count mismatch returns a **one-element poison
/// vector** rather than an empty one, so a caller cannot read "no
/// coefficients" as "nothing to bound" — the returned length is never
/// zero, and hulling it yields poison.
///
/// See the module docs for why the rational case is not here.
pub fn derivative_coeffs<E: CertifiedEnclosure>(
    kv: &KnotVector,
    coeffs: &[E],
) -> Vec<RingInterval> {
    if coeffs.len() != kv.control_count() || coeffs.len() < 2 {
        return vec![RingInterval::poison()];
    }
    (0..coeffs.len() - 1)
        .map(|i| deriv_coeff(kv, coeffs, i))
        .collect()
}

/// Enclosure of the **derivative** of a nonrational scalar B-spline over
/// one span of the *original* knot vector.
///
/// Index bookkeeping: dropping the outer knots shifts span indices by
/// one, so span `s` of `kv` is span `s − 1` of the derivative's knot
/// vector, where the active degree-`(p−1)` basis indices are
/// `s − p … s − 1`. That is the range of `Q` hulled here — no derivative
/// [`KnotVector`] needs to be materialised.
///
/// The range is nonempty for every [`Span`]: `first = s − p < s = last`
/// because `KnotVector::clamped` refuses degree 0. It is the `Span`'s
/// own window minus its top end.
pub fn derivative_span_hull<E: CertifiedEnclosure>(
    kv: &KnotVector,
    coeffs: &[E],
    span: Span,
) -> RingInterval {
    let Some((first, last)) = span_indices(kv, coeffs.len(), span) else {
        return RingInterval::poison();
    };
    let mut acc = RingInterval::poison();
    // Fixed ascending reduction order (D9). Range: [span − p, span − 1].
    for (n, i) in (first..last).enumerate() {
        let q = deriv_coeff(kv, coeffs, i);
        acc = if n == 0 {
            q
        } else {
            RingInterval::hull(acc, q)
        };
    }
    acc
}

/// Enclosure of the derivative over the whole domain: the hull of all
/// derivative coefficients.
pub fn derivative_domain_hull<E: CertifiedEnclosure>(
    kv: &KnotVector,
    coeffs: &[E],
) -> RingInterval {
    let qs = derivative_coeffs(kv, coeffs);
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

/// A certified upper bound on `|f|` over one span — the scalar sup-norm
/// reading of [`span_hull`], and the shape C2.2's honesty limb consumes
/// (`sup_norm_bound_span(..) <= eps` certifies the span).
///
/// Returns `NaN` for every poison path, which fails that comparison
/// under every direction (D4 ¶2). The value is an upper bound on the
/// true supremum, never an approximation of it.
pub fn sup_norm_bound_span<E: CertifiedEnclosure>(
    kv: &KnotVector,
    coeffs: &[E],
    span: Span,
) -> f64 {
    span_hull(kv, coeffs, span).mag()
}

/// A certified upper bound on `|f|` over the whole domain. See
/// [`sup_norm_bound_span`].
pub fn sup_norm_bound<E: CertifiedEnclosure>(kv: &KnotVector, coeffs: &[E]) -> f64 {
    domain_hull(kv, coeffs).mag()
}

/// The rational counterpart of [`sup_norm_bound`]: an upper bound on
/// `|f|` over the whole domain of a rational scalar spline, `NaN` unless
/// every weight satisfies the positivity precondition.
pub fn sup_norm_bound_rational<E: CertifiedEnclosure>(
    kv: &KnotVector,
    coeffs: &[E],
    weights: &[f64],
) -> f64 {
    domain_hull_rational(kv, coeffs, weights).mag()
}
