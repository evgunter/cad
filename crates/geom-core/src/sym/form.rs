//! **The documented limits.** The normal form is a QUOTIENT of polynomials
//! over the parameter symbols — a field of fractions, not a polynomial ring
//! — so a reciprocal is a first-class part of it and `(x/y)·y − x` DOES
//! decide symbolically: `x/y` is the form `x` over `y`, multiplying by `y`
//! gives `xy/y`, and the difference's numerator is the zero polynomial.
//! (The tier's headline row needs exactly that: an extruded strut's carrier
//! is `origin + (w/‖w‖)·t`, so its endpoint residual is literally
//! `w·(‖w‖·‖w‖⁻¹ − 1)`; with the reciprocal held opaque that residual is
//! not the zero form, the tier discharges the rest of the identity
//! population and the macroscopic box still refuses — measured on
//! `m10_3_driver_interval`'s slab at a ±0.05 band: 945 identities
//! discharged, `carrier_endpoint_end` still indeterminate at `[0, 0.21]`.)
//! That is the whole of the reciprocal's reach, and it is a NORMAL FORM
//! rather than a simplification RULE bolted on: the field of fractions
//! reached by the same construction the polynomial form is (`a/b + c/d =
//! (ad + cb)/(bd)`, `(a/b)⁻¹ = b/a`), with nothing factored and no
//! simplification attempted.
//!
//! What remains outside the PLAIN form: no factoring, and no functional
//! identity of any opaque atom — each atom is an indeterminate keyed by its
//! argument's form, so two occurrences of ONE atom cancel and nothing else
//! about it is known there.

#[cfg(feature = "sym-profile-testing")]
use super::profile;
use super::rational::Rat;
use super::{Hash128, SymBudget};

/// A monomial: indeterminate ids with their exponents, sorted by id and
/// carrying no zero exponent. The empty vector is the constant monomial.
/// Its `Ord` — the vector's lexicographic order — is the term order of
/// [`Poly`].
pub(super) type Mono = Vec<(u128, u32)>;

/// The polynomial normal form over the parameter symbols, π and the
/// opaque atoms — with exact rational coefficients, and no zero
/// coefficient stored, so **the form is zero iff it has no terms**.
///
/// # What a `Poly` is in memory, and why the order is the map's
///
/// The terms are one **sorted vector** of `(monomial, coefficient)`
/// pairs — sorted by the monomial under [`Mono`]'s own `Ord`, each
/// monomial at most once, no zero coefficient — so a form of ten terms
/// is one allocation for the terms and one per monomial rather than a
/// tree node per term. The order is exactly the order a
/// `BTreeMap<Mono, Rat>` iterates in, and that is not a convenience:
/// [`Poly::digest`] feeds the terms to the hasher in this order, the
/// digest is the key an opaque atom is minted under, and the atom key
/// is what a decision reads. Every operation below keeps the vector
/// sorted by merging or by binary search; nothing sorts after the
/// fact, so no term is ever out of place between two operations.
/// `m10_sym_profile_interval`'s walk ledger pins the digests the tier
/// builds on the slab and the plate at their nominals.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub(super) struct Poly {
    /// Sorted by monomial, no monomial twice, no zero coefficient — the
    /// invariant every constructor and [`Poly::insert`] keep. Read
    /// freely; write through `insert`.
    pub(super) terms: Vec<(Mono, Rat)>,
}

impl Poly {
    pub(super) fn zero() -> Self {
        Self::default()
    }

    pub(super) fn one() -> Self {
        Self::constant(Rat::one())
    }

    pub(super) fn constant(c: Rat) -> Self {
        Self::term(Mono::new(), c)
    }

    /// The form of a single indeterminate, coefficient one.
    pub(super) fn indet(id: u128) -> Self {
        Self::term(vec![(id, 1)], Rat::one())
    }

    /// The one-term polynomial `c · m` — the zero polynomial when `c`
    /// is zero.
    pub(super) fn term(mono: Mono, c: Rat) -> Self {
        let terms = if c.is_zero() {
            Vec::new()
        } else {
            vec![(mono, c)]
        };
        Self { terms }
    }

    pub(super) fn is_zero(&self) -> bool {
        self.terms.is_empty()
    }

    /// The monomials, in the terms' order.
    pub(super) fn monos(&self) -> impl Iterator<Item = &Mono> {
        self.terms.iter().map(|(m, _)| m)
    }

    /// The value of a CONSTANT polynomial (no indeterminate).
    pub(super) fn as_constant(&self) -> Option<Rat> {
        match self.terms.as_slice() {
            [] => Some(Rat::zero()),
            [(m, c)] => m.is_empty().then(|| c.clone()),
            _ => None,
        }
    }

    /// The largest total degree of any term (zero for the zero form).
    pub(super) fn degree(&self) -> u32 {
        self.monos()
            .map(|m| m.iter().map(|(_, e)| *e).sum::<u32>())
            .max()
            .unwrap_or(0)
    }

    /// Adds `c · mono`, merging into the term already there, and drops
    /// the term when the sum is zero. `None` where the ring refuses
    /// the sum.
    pub(super) fn insert(&mut self, mono: Mono, c: Rat) -> Option<()> {
        if c.is_zero() {
            return Some(());
        }
        match self.terms.binary_search_by(|(m, _)| m.cmp(&mono)) {
            Err(i) => self.terms.insert(i, (mono, c)),
            Ok(i) => self.merge_at(i, &c)?,
        }
        Some(())
    }

    /// Adds `c` into the coefficient at `i` (that term's first), and
    /// drops the term when the sum is zero.
    fn merge_at(&mut self, i: usize, c: &Rat) -> Option<()> {
        let sum = self.terms[i].1.add(c)?;
        if sum.is_zero() {
            self.terms.remove(i);
        } else {
            self.terms[i].1 = sum;
        }
        Some(())
    }

    /// The sum: one merge of the two sorted vectors, adding the
    /// coefficients where a monomial is in both (this side's first).
    pub(super) fn add(&self, other: &Self) -> Option<Self> {
        let (a, b) = (&self.terms, &other.terms);
        let mut out = Vec::with_capacity(a.len() + b.len());
        let (mut i, mut j) = (0, 0);
        while i < a.len() && j < b.len() {
            let (ma, ca) = &a[i];
            let (mb, cb) = &b[j];
            match ma.cmp(mb) {
                core::cmp::Ordering::Less => {
                    out.push((ma.clone(), ca.clone()));
                    i += 1;
                }
                core::cmp::Ordering::Greater => {
                    out.push((mb.clone(), cb.clone()));
                    j += 1;
                }
                core::cmp::Ordering::Equal => {
                    let sum = ca.add(cb)?;
                    if !sum.is_zero() {
                        out.push((ma.clone(), sum));
                    }
                    i += 1;
                    j += 1;
                }
            }
        }
        out.extend_from_slice(&a[i..]);
        out.extend_from_slice(&b[j..]);
        Some(Self { terms: out })
    }

    pub(super) fn neg(&self) -> Option<Self> {
        let terms = self
            .terms
            .iter()
            .map(|(m, c)| Some((m.clone(), c.neg()?)))
            .collect::<Option<Vec<_>>>()?;
        Some(Self { terms })
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
    ///
    /// Built by inserting each product term in `(a-term, b-term)` order
    /// — the order the coefficient sums are taken in, which the ring's
    /// refusals can see — each by binary search into the sorted vector.
    /// Measured against collecting the products and sorting them once
    /// (a fifth more instructions on both documents: the products
    /// vector and the sort cost more than the searches they replace)
    /// and against reusing one scratch monomial across the loop (a
    /// wash: a product here is mostly one term by one, so a merge that
    /// would save the allocation is rare).
    pub(super) fn mul(&self, other: &Self, budget: SymBudget) -> Option<Self> {
        let Some(pairs) = self.terms.len().checked_mul(other.terms.len()) else {
            #[cfg(feature = "sym-profile-testing")]
            profile::note(profile::FreezeCause::Terms);
            return None;
        };
        if pairs > budget.max_terms {
            #[cfg(feature = "sym-profile-testing")]
            profile::note(profile::FreezeCause::Terms);
            return None;
        }
        if self
            .degree()
            .checked_add(other.degree())
            .is_none_or(|d| d > budget.max_degree)
        {
            #[cfg(feature = "sym-profile-testing")]
            profile::note(profile::FreezeCause::Degree);
            return None;
        }
        let mut out = Self {
            terms: Vec::with_capacity(pairs),
        };
        for (ma, ca) in &self.terms {
            for (mb, cb) in &other.terms {
                out.insert(mono_mul(ma, mb)?, ca.mul(cb)?)?;
            }
        }
        Some(out)
    }

    /// The form's canonical digest — the key an opaque atom is minted
    /// under, so two atoms with equal-form arguments are one
    /// indeterminate. The terms feed the hasher in the vector's order,
    /// which is the monomial order.
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
                let Some(e) = ea.checked_add(eb) else {
                    #[cfg(feature = "sym-profile-testing")]
                    profile::note(profile::FreezeCause::Overflow);
                    return None;
                };
                out.push((ia, e));
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
/// The module docs carry it, with the row that measured it.
///
/// # Why the zero test stays a theorem
///
/// The form is zero **iff its NUMERATOR is the zero polynomial**. As a
/// rational function that is exactly zero; as a real number at the box's
/// actual parameter point it is `p(x)/q(x) = 0` PROVIDED `q(x) ≠ 0`, and
/// clause 1 of [`crate::predicate::Decide::sign_within`]'s test already
/// guarantees that: a division by an enclosure containing zero is undefined
/// there, so the interval decoration drops to `Trv`, `Trv` propagates, and
/// `certified_bracket()` refuses the margin before the identity test is
/// ever asked. The clause was there for `sqrt(-1)`; it covers `1/0` by the
/// same sentence.
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
    /// [`super::signed`]): it is equal to the expression at every point of
    /// the leaf's box rather than identically in the parameters, so a zero
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
    let inside = ok(&f.num) && ok(&f.den);
    #[cfg(feature = "sym-profile-testing")]
    if !inside {
        profile::note_within(budget, f);
    }
    inside
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
