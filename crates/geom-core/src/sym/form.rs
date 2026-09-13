//! **The documented limits.** The normal form is a QUOTIENT of
//! polynomials over the parameter symbols — a field of fractions, not a
//! polynomial ring — so a reciprocal is a first-class part of it and
//! `(x/y)·y − x` DOES decide symbolically: `x/y` is the form `x` over
//! `y`, multiplying by `y` gives `xy/y`, and the difference's numerator
//! is the zero polynomial. (The tier's headline row needs exactly that:
//! an extruded strut's carrier is `origin + (w/‖w‖)·t`, so its endpoint
//! residual is literally `w·(‖w‖·‖w‖⁻¹ − 1)`.) That is the whole of the
//! reciprocal's reach, and it is a NORMAL FORM rather than a rewrite
//! rule: nothing is factored, and no simplification is attempted.
//!
//! What remains outside the PLAIN form: no factoring, and no functional
//! identity of any opaque atom — each atom is an indeterminate keyed by
//! its argument's form, so two occurrences of ONE atom cancel and
//! nothing else about it is known there. The SHIPPED tier layers the
//! atom algebra on top (the M10-8, M10-9 and M10-10 sections of the
//! tier's own docs):
//! `sqrt(x)·sqrt(x) − x` and `sin² + cos² − 1` DO decide as theorems
//! under [`super::SymRules::shipped`] (rules A and B, over the top residual
//! and per node), and `sin`/`cos` of `q · atan X` fold to closed forms
//! (rule D). What still stands with the shipped set is what needs a
//! SIGN: `|x| − x` on a nonnegative `x` is rule C's, and rule C is
//! dial-off. These are limits of the tier and not bugs in it —
//! over-refusal is the safe direction, and every such margin falls to
//! the numeric channel exactly as before.

use std::collections::BTreeMap;

use super::rational::Rat;
use super::{Hash128, SymBudget};

/// A monomial: indeterminate ids with their exponents, sorted by id and
/// carrying no zero exponent. The empty vector is the constant monomial.
pub(super) type Mono = Vec<(u128, u32)>;

/// The polynomial normal form over the parameter symbols, π and the
/// opaque atoms — with exact rational coefficients, and no zero
/// coefficient stored, so **the form is zero iff it has no terms**.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub(super) struct Poly {
    pub(super) terms: BTreeMap<Mono, Rat>,
}

impl Poly {
    pub(super) fn zero() -> Self {
        Self::default()
    }

    pub(super) fn one() -> Self {
        Self::constant(Rat::one())
    }

    pub(super) fn constant(c: Rat) -> Self {
        let mut terms = BTreeMap::new();
        if !c.is_zero() {
            terms.insert(Mono::new(), c);
        }
        Self { terms }
    }

    /// The form of a single indeterminate, coefficient one.
    pub(super) fn indet(id: u128) -> Self {
        let mut terms = BTreeMap::new();
        terms.insert(vec![(id, 1)], Rat::one());
        Self { terms }
    }

    pub(super) fn is_zero(&self) -> bool {
        self.terms.is_empty()
    }

    /// The value of a CONSTANT polynomial (no indeterminate).
    pub(super) fn as_constant(&self) -> Option<Rat> {
        match self.terms.len() {
            0 => Some(Rat::zero()),
            1 => {
                let (m, c) = self.terms.iter().next()?;
                m.is_empty().then(|| c.clone())
            }
            _ => None,
        }
    }

    /// The largest total degree of any term (zero for the zero form).
    pub(super) fn degree(&self) -> u32 {
        self.terms
            .keys()
            .map(|m| m.iter().map(|(_, e)| *e).sum::<u32>())
            .max()
            .unwrap_or(0)
    }

    pub(super) fn insert(&mut self, mono: Mono, c: Rat) -> Option<()> {
        if c.is_zero() {
            return Some(());
        }
        match self.terms.remove(&mono) {
            None => {
                self.terms.insert(mono, c);
            }
            Some(existing) => {
                let sum = existing.add(&c)?;
                if !sum.is_zero() {
                    self.terms.insert(mono, sum);
                }
            }
        }
        Some(())
    }

    pub(super) fn add(&self, other: &Self) -> Option<Self> {
        let mut out = self.clone();
        for (m, c) in &other.terms {
            out.insert(m.clone(), c.clone())?;
        }
        Some(out)
    }

    pub(super) fn neg(&self) -> Option<Self> {
        let mut out = Self::zero();
        for (m, c) in &self.terms {
            out.terms.insert(m.clone(), c.neg()?);
        }
        Some(out)
    }

    /// The product, or `None` for the caller to freeze.
    ///
    /// **Refused BEFORE it is built**, on bounds that cost nothing to
    /// compute: the product has at most `|a|*|b|` terms and degree
    /// exactly `deg(a) + deg(b)`. The first version built the whole
    /// product and let [`within`] reject it afterwards, which is how a
    /// single multiplication came to take 10.8 s on a reviewer's
    /// bracket — the work was done and then thrown away. Freezing is
    /// the same outcome either way; only the bill differs.
    ///
    /// The term bound is an UPPER one (colliding monomials merge), so a
    /// product whose terms would have collided down under the budget is
    /// refused where the old code would have kept it. That is a real
    /// difference and it is measured rather than assumed: on the M10-3
    /// slab and the tour's plate the frozen counts are unchanged, so
    /// nothing the shipped fixtures rely on sat in that gap. The degree
    /// bound is exact for the leading monomial in every case the form
    /// reaches, cancellation of a whole leading term needing coefficient
    /// cancellation that a product of two nonzero polynomials over a
    /// field does not produce.
    pub(super) fn mul(&self, other: &Self, budget: SymBudget) -> Option<Self> {
        if self.terms.len().checked_mul(other.terms.len())? > budget.max_terms
            || self.degree().checked_add(other.degree())? > budget.max_degree
        {
            return None;
        }
        let mut out = Self::zero();
        for (ma, ca) in &self.terms {
            for (mb, cb) in &other.terms {
                out.insert(mono_mul(ma, mb)?, ca.mul(cb)?)?;
            }
        }
        Some(out)
    }

    /// The form's canonical digest — the key an opaque atom is minted
    /// under, so two atoms with equal-form arguments are one
    /// indeterminate.
    pub(super) fn digest(&self) -> u128 {
        let mut h = Hash128::new().word(0x504f_4c59_4e46_524d);
        for (m, c) in &self.terms {
            h = h.word(m.len() as u64);
            for (id, e) in m {
                h = h.wide(*id).word(u64::from(*e));
            }
            h = c.feed(h);
        }
        h.finish()
    }
}

/// The product of two monomials, refusing an exponent overflow.
pub(super) fn mono_mul(a: &Mono, b: &Mono) -> Option<Mono> {
    let mut out: Mono = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    while i < a.len() || j < b.len() {
        match (a.get(i), b.get(j)) {
            (Some(&(ia, ea)), Some(&(ib, eb))) if ia == ib => {
                out.push((ia, ea.checked_add(eb)?));
                i += 1;
                j += 1;
            }
            (Some(&(ia, ea)), Some(&(ib, _))) if ia < ib => {
                out.push((ia, ea));
                i += 1;
            }
            (Some(_), Some(&(ib, eb))) => {
                out.push((ib, eb));
                j += 1;
            }
            (Some(&(ia, ea)), None) => {
                out.push((ia, ea));
                i += 1;
            }
            (None, Some(&(ib, eb))) => {
                out.push((ib, eb));
                j += 1;
            }
            (None, None) => break,
        }
    }
    Some(out)
}

/// **The normal form: a quotient of two polynomials** over the parameter
/// symbols, π and the opaque atoms — with exact rational coefficients,
/// a denominator that is never the zero polynomial, and NO common-factor
/// cancellation.
///
/// # Why a quotient and not a polynomial
///
/// Division is not decoration in this kernel's identities. An extruded
/// strut's carrier is `origin + (w/‖w‖)·t` metered by `t ∈ [0, ‖w‖]`, so
/// the endpoint-pinning residual `carrier.eval(t₁) − end` is literally
/// `w·(‖w‖ · ‖w‖⁻¹ − 1)`: with the reciprocal held opaque the residual is
/// not the zero form, the tier discharges the rest of the identity
/// population and the macroscopic box still refuses. Measured on
/// `m10_3_driver_interval`'s slab at a ±0.05 band: 945 identities
/// discharged, `carrier_endpoint_end` still indeterminate at `[0, 0.21]`.
///
/// A quotient is not a simplification RULE bolted on — it is the normal
/// form of the field of fractions, reached by the same construction the
/// polynomial form is: `a/b + c/d = (ad + cb)/(bd)`, `(a/b)⁻¹ = b/a`.
/// Nothing is factored, `sqrt` and the transcendentals stay opaque
/// atoms, and no identity is asserted about them.
///
/// # Why the zero test stays a theorem
///
/// The form is zero **iff its NUMERATOR is the zero polynomial**. As a
/// rational function that is exactly zero; as a real number at the box's
/// actual parameter point it is `p(x)/q(x) = 0` PROVIDED `q(x) ≠ 0`, and
/// clause 1 of [`Decide::sign_within`]'s test already guarantees that: a
/// division by an enclosure containing zero is undefined there, so the
/// interval decoration drops to `Trv`, `Trv` propagates, and
/// `certified_bracket()` refuses the margin before the identity test is
/// ever asked. The clause was there for `sqrt(-1)`; it covers `1/0` by
/// the same sentence.
///
/// # What is deliberately absent
///
/// No common-factor cancellation, so `x²/x` and `x/1` are DIFFERENT
/// forms and an atom over one does not cancel against an atom over the
/// other. That is the conservative direction — a missed cancellation is
/// a numeric decision, which is what the kernel did before the tier
/// existed — and it keeps the form's cost linear in the expression
/// rather than in a polynomial GCD.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Form {
    pub(super) num: Poly,
    pub(super) den: Poly,
    /// **This form is not a rational function of the parameters** — it
    /// was built through a division by the ZERO polynomial, so it has
    /// no value anywhere and nothing derived from it may be declared
    /// identically zero.
    ///
    /// Why a poison and not a freeze (the reviewer's row
    /// `atan(1/(x-x)) - atan(1/(x-x))`). Freezing turns an
    /// unrepresentable subexpression into an INDETERMINATE keyed by its
    /// node id, and two occurrences of one node share that id — so the
    /// two `atan`s would be one unknown `u`, `u - u` would be the zero
    /// polynomial, and the tier would answer `Zero` for an expression
    /// with no real value. Freezing is the right answer for something
    /// the form cannot REPRESENT; it is the wrong answer for something
    /// that does not EXIST. Poison propagates through every combinator
    /// and makes [`Form::is_zero`] false, so the theorem's clause 2
    /// cannot be satisfied through it.
    ///
    /// This is the form-side half of clause 1. The value-side half
    /// (`MarginDiag::Invalid` from the numeric channel) catches a
    /// domain violation the SCALAR can see — an uncertified interval,
    /// a NaN. It cannot see this one: at `f64` the whole expression
    /// evaluates to a finite `0.0`, because `1/0` is `+inf`,
    /// `atan(+inf)` is `pi/2`, and the difference is an honest zero.
    /// The two halves catch different things and both are needed.
    pub(super) poisoned: bool,
    /// **This form was built through a clause-3 fold** (rule C,
    /// [`signed`]): it is equal to the expression at every point of the
    /// leaf's box rather than identically in the parameters, so a zero
    /// reached through it is `sign_gated`, not `symbolic_zero`. Sticky
    /// through every combinator, like the poison flag.
    pub(super) gated: bool,
}

impl Form {
    pub(super) fn poly(num: Poly) -> Self {
        Self::quotient(num, Poly::one())
    }

    pub(super) fn quotient(num: Poly, den: Poly) -> Self {
        Self {
            num,
            den,
            poisoned: false,
            gated: false,
        }
    }

    pub(super) fn zero() -> Self {
        Self::poly(Poly::zero())
    }

    /// The form of an expression with no value: see [`Form::poisoned`].
    /// Its numerator is deliberately the ONE polynomial, so that a
    /// reader who ignores the flag still never reads it as a zero.
    pub(super) fn poison() -> Self {
        Self {
            num: Poly::one(),
            den: Poly::one(),
            poisoned: true,
            gated: false,
        }
    }

    /// Whether either operand is poison — the propagation rule, in one
    /// place so no combinator can forget it.
    pub(super) fn tainted(&self, other: &Self) -> bool {
        self.poisoned || other.poisoned
    }

    pub(super) fn is_zero(&self) -> bool {
        !self.poisoned && self.num.is_zero()
    }

    pub(super) fn add(&self, other: &Self, budget: SymBudget) -> Option<Self> {
        if self.tainted(other) {
            return Some(Self::poison());
        }
        // A shared denominator adds numerators, which is both cheaper
        // and tighter against the budget than cross-multiplying two
        // copies of the same polynomial.
        if self.den == other.den {
            return Some(Self {
                num: self.num.add(&other.num)?,
                den: self.den.clone(),
                poisoned: false,
                gated: self.gated || other.gated,
            });
        }
        Some(Self {
            num: self
                .num
                .mul(&other.den, budget)?
                .add(&other.num.mul(&self.den, budget)?)?,
            den: self.den.mul(&other.den, budget)?,
            poisoned: false,
            gated: self.gated || other.gated,
        })
    }

    pub(super) fn neg(&self) -> Option<Self> {
        if self.poisoned {
            return Some(Self::poison());
        }
        Some(Self {
            num: self.num.neg()?,
            den: self.den.clone(),
            poisoned: false,
            gated: self.gated,
        })
    }

    pub(super) fn mul(&self, other: &Self, budget: SymBudget) -> Option<Self> {
        if self.tainted(other) {
            return Some(Self::poison());
        }
        Some(Self {
            num: self.num.mul(&other.num, budget)?,
            den: self.den.mul(&other.den, budget)?,
            poisoned: false,
            gated: self.gated || other.gated,
        })
    }

    /// The reciprocal — POISON when the numerator is identically zero,
    /// which is not a rational function and has no value at any point.
    pub(super) fn recip(&self) -> Option<Self> {
        if self.poisoned || self.num.is_zero() {
            return Some(Self::poison());
        }
        Some(Self {
            num: self.den.clone(),
            den: self.num.clone(),
            poisoned: false,
            gated: self.gated,
        })
    }

    /// The form's canonical digest — the key an opaque atom is minted
    /// under, so two atoms with equal-form arguments are one
    /// indeterminate. The poison flag is part of it, so an atom over a
    /// poisoned argument is never keyed as one over a clean argument;
    /// the atom itself is poisoned too, which is the load-bearing half.
    pub(super) fn digest(&self) -> u128 {
        Hash128::new()
            .word(0x464f_524d_5f4e_4652)
            .word(u64::from(self.poisoned) | (u64::from(self.gated) << 1))
            .wide(self.num.digest())
            .wide(self.den.digest())
            .finish()
    }
}

/// Whether a form is inside the session's freezing budget — both halves
/// of the quotient, because a denominator that grows without bound costs
/// exactly what a numerator does.
pub(super) fn within(budget: SymBudget, f: &Form) -> bool {
    let ok = |p: &Poly| p.terms.len() <= budget.max_terms && p.degree() <= budget.max_degree;
    ok(&f.num) && ok(&f.den)
}

/// `base^n` for `n >= 0`, budget-checked at every step so a large
/// exponent freezes rather than allocating its way to the ceiling.
pub(super) fn powi_form(base: &Form, n: u32, budget: SymBudget) -> Option<Form> {
    // `x^0` is 1 only where `x` has a value; a poisoned base has none,
    // so the exponent cannot rescue it.
    if base.poisoned {
        return Some(Form::poison());
    }
    let mut acc = Form::poly(Poly::one());
    for _ in 0..n {
        acc = acc.mul(base, budget)?;
        if !within(budget, &acc) {
            return None;
        }
    }
    Some(acc)
}
