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

use core::cmp::Ordering;

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
    /// invariant every constructor and [`Poly::insert`] keep, and
    /// private so that nothing outside this module can break it;
    /// [`Poly::terms`] reads it.
    terms: Vec<(Mono, Rat)>,
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

    /// **A polynomial from terms already sorted by monomial**, with no
    /// zero coefficient and no repeat — the invariant the type keeps,
    /// checked here rather than assumed, because the one caller
    /// ([`quotient::divide`](super::quotient)) builds its order back by
    /// sorting and a wrong order would be a silently wrong polynomial.
    /// `None` when the invariant does not hold.
    pub(super) fn from_sorted_terms(terms: Vec<(Mono, Rat)>) -> Option<Self> {
        let sorted = terms.windows(2).all(|w| w[0].0 < w[1].0);
        (sorted && !terms.iter().any(|(_, c)| c.is_zero())).then_some(Self { terms })
    }

    /// The terms, sorted by monomial.
    pub(super) fn terms(&self) -> &[(Mono, Rat)] {
        &self.terms
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
        let mut terms = Vec::with_capacity(self.terms.len() + other.terms.len());
        merge_sorted(
            &self.terms,
            &other.terms,
            |t| &t.0,
            |(m, ca), (_, cb)| {
                let sum = ca.add(cb)?;
                Some((!sum.is_zero()).then(|| (m.clone(), sum)))
            },
            Clone::clone,
            &mut terms,
        )?;
        Some(Self { terms })
    }

    pub(super) fn neg(&self) -> Option<Self> {
        let terms = self
            .terms
            .iter()
            .map(|(m, c)| Some((m.clone(), c.neg()?)))
            .collect::<Option<Vec<_>>>()?;
        Some(Self { terms })
    }

    /// `k · self` — every coefficient multiplied by one rational, the
    /// monomials untouched. Cheaper than a product by a constant
    /// polynomial (no budget check is needed: neither the term count
    /// nor the degree can move) and the one home for what
    /// [`quotient`](super::quotient)'s scale step and
    /// [`trig`](super::trig)'s integer multiples both do.
    pub(super) fn scaled(&self, k: &Rat) -> Option<Self> {
        let mut out = Self::zero();
        for (m, c) in self.terms() {
            out.insert(m.clone(), c.mul(k)?)?;
        }
        Some(out)
    }

    /// The product, or `None` for the caller to freeze.
    ///
    /// **Refused BEFORE it is built**, on bounds that cost nothing to
    /// compute: at most `|a|·|b|` terms — an UPPER bound, since
    /// colliding monomials merge, so a product that would have merged
    /// down under the budget is refused too, and the frozen counts on
    /// the measured documents say nothing sits in that gap — and
    /// degree exactly `deg(a) + deg(b)`, which is exact for the leading
    /// monomial because a product of two nonzero polynomials over a
    /// field cannot cancel it. Building first and refusing after would
    /// do the work and throw it away.
    ///
    /// Built by inserting each product term in `(a-term, b-term)` order
    /// — the order the coefficient sums are taken in, which the ring's
    /// refusals can see — each by binary search into the sorted vector;
    /// the other spellings measured, and their numbers, are on the
    /// item (`work/sym/symbolic-tier-costs-95-percent-of-the-m10-3-drive`,
    /// `## The change (SYM-4)`).
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
        out.terms.shrink_to_fit();
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

impl Poly {
    /// **`Q` with `self = Q·d` exactly**, for a non-constant `d`; `None`
    /// wherever `d` does not divide `self` or the division declines.
    ///
    /// **Two necessary conditions first, at the cost of four monomials.**
    /// Under the monomial order [`grlex`], `lead(Q·d) = lead(Q)·lead(d)`
    /// and `trail(Q·d) = trail(Q)·trail(d)`, so an exact division needs
    /// `lead(d) | lead(self)` and `trail(d) | trail(self)` as monomials.
    /// Either failing declines before a single step — which is where
    /// nearly every root argument that is not an exact quotient goes.
    ///
    /// **Then division by the leading term.** Each step takes the
    /// remainder's grlex-largest term `r`, the quotient term `t = r /
    /// lead(d)` (declining where the monomial does not divide), and
    /// subtracts `t·d` from the remainder. **The invariant, held exactly
    /// at every step, is `rest = self − q·d`**: it holds at the start
    /// (`rest = self`, `q = 0`), and a step adds `t` to `q` and
    /// subtracts `t·d` from `rest` in the exact ring — `t·lead(d)` IS the
    /// term `r` the step removed (`t`'s coefficient is `r`'s over
    /// `lead(d)`'s, exact), and the rest of `t·d` is merged in term by
    /// term. So a `rest` that reaches zero IS the statement `self = q·d`,
    /// and no product needs checking afterwards. Every arithmetic refusal
    /// answers `None`, never a truncated form.
    ///
    /// **Cost.** The remainder is held in a map ordered by [`grlex`], so a
    /// step pops its leading term in `O(log n)` and merges `|d| − 1`
    /// products in `O(|d|·log n)`: a division that runs is linear in
    /// its steps times `|d|`, and `q` is collected once and sorted at the
    /// end. Under a monomial order every product is smaller than the term
    /// it replaces, so the leading term falls at every step and each `t`
    /// is a new monomial: `q` gains one term per step, and the step cap
    /// is the budget's own term cap — a quotient past `budget.max_terms`
    /// terms is not a form the tier can hold. Measured in release:
    /// - a division the conditions decline — `geom-core`'s
    ///   `decide_4_root_quotient_rows::a_declined_division_costs_nothing_the_budget_allows`,
    ///   the non-divisible `sqrt(x³⁴/(x² − y − z − u − v))`, whose
    ///   trailing term fails — takes the whole DECISION 0.3 ms at a
    ///   4096-term budget and 0.1 ms at 65536, with zero division steps,
    ///   where a loop that rebuilds `q` and the remainder at every step
    ///   spends its whole cap on the same shape: 0.78 s at 4096 and
    ///   1.06 s at 16384;
    /// - a division that passes both conditions and runs to its end
    ///   before declining — `(xⁿ + 1)/(x − 1)`, whose remainder is the
    ///   constant 2 — takes 4001 steps in 1.3 ms at `n = 4000` and
    ///   60001 steps in 20 ms at `n = 60000` (a local timing, not a row):
    ///   linear in the steps.
    ///
    /// A refusal partway (the ring, an exponent) leaves the cost
    /// profile's thread-local note set, although no node froze. It is
    /// harmless: the caller always builds a form (the quotient's root, or
    /// the atom it would have minted), so the node is recorded as built
    /// and the note is never read — `form_in` clears it before the next
    /// `combine`.
    pub(super) fn div_exact(&self, d: &Self, budget: SymBudget) -> Option<Self> {
        use std::collections::BTreeMap;
        use std::collections::btree_map::Entry;

        if self.is_zero() || d.as_constant().is_some() || d.degree() > self.degree() {
            return None;
        }
        let (dm, dc) = leading(d)?;
        mono_div(&leading(self)?.0, dm)?;
        mono_div(&trailing(self)?.0, &trailing(d)?.0)?;
        let inverse = dc.recip()?;
        let mut rest: BTreeMap<GrlexKey, Rat> = self
            .terms()
            .iter()
            .map(|(m, c)| (GrlexKey(m.clone()), c.clone()))
            .collect();
        let mut q: Vec<(Mono, Rat)> = Vec::new();
        while let Some((GrlexKey(rm), rc)) = rest.pop_last() {
            if q.len() >= budget.max_terms {
                return None;
            }
            #[cfg(test)]
            DIV_STEPS.set(DIV_STEPS.get() + 1);
            let tm = mono_div(&rm, dm)?;
            let tc = rc.mul(&inverse)?;
            for (m, c) in d.terms() {
                if m == dm {
                    continue;
                }
                let product = tc.mul(c)?.neg()?;
                match rest.entry(GrlexKey(mono_mul(&tm, m)?)) {
                    Entry::Occupied(mut e) => {
                        let sum = e.get().add(&product)?;
                        if sum.is_zero() {
                            e.remove();
                        } else {
                            *e.get_mut() = sum;
                        }
                    }
                    Entry::Vacant(e) => {
                        e.insert(product);
                    }
                }
            }
            q.push((tm, tc));
        }
        q.sort_by(|a, b| a.0.cmp(&b.0));
        Self::from_sorted_terms(q)
    }
}

/// A monomial ordered by [`grlex`] — the remainder map's key in
/// [`Poly::div_exact`], so the leading term is the map's last.
#[derive(PartialEq, Eq)]
struct GrlexKey(Mono);

impl Ord for GrlexKey {
    fn cmp(&self, other: &Self) -> Ordering {
        grlex(&self.0, &other.0)
    }
}

impl PartialOrd for GrlexKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
thread_local! {
    /// The steps [`Poly::div_exact`] took on this thread — read by the
    /// unit rows that pin a decline as taken BEFORE any step.
    static DIV_STEPS: core::cell::Cell<u64> = const { core::cell::Cell::new(0) };
}

/// **The exponent of `id` in `m`** — zero where the monomial does not
/// carry it. One home: a monomial is a sorted `(id, exponent)` vector,
/// so every rule that asks "to what power does this atom appear" would
/// otherwise spell the same `find` again ([`algebra`](super::algebra),
/// [`quotient`](super::quotient) and [`signed`](super::signed) all ask).
pub(super) fn exp_of(m: &Mono, id: u128) -> u32 {
    m.iter().find(|(i, _)| *i == id).map_or(0, |(_, e)| *e)
}

/// The product of two monomials, refusing an exponent overflow.
pub(super) fn mono_mul(a: &Mono, b: &Mono) -> Option<Mono> {
    let mut out: Mono = Vec::with_capacity(a.len() + b.len());
    merge_sorted(
        a,
        b,
        |t| &t.0,
        |&(id, ea), &(_, eb)| {
            let Some(e) = ea.checked_add(eb) else {
                #[cfg(feature = "sym-profile-testing")]
                profile::note(profile::FreezeCause::Overflow);
                return None;
            };
            Some(Some((id, e)))
        },
        |t| *t,
        &mut out,
    )?;
    Some(out)
}

/// **The graded-lexicographic monomial order** — total degree first,
/// then the exponent at the smallest indeterminate id where the two
/// differ. The one order the tier's leading-term arithmetic runs under
/// ([`leading`], [`trailing`], [`Poly::div_exact`] and `signed`'s
/// polynomial square root): `a > b` implies `a·m > b·m`, and the empty
/// monomial is the least, which is what a division or a root
/// recurrence by leading terms needs. `Mono`'s own `Ord` — the order a
/// `Poly` STORES its terms in — is a vector comparison and is NOT a
/// monomial order, which is the hazard this one home exists for.
/// Allocation-free: one merge over the two sorted vectors.
pub(super) fn grlex(a: &Mono, b: &Mono) -> Ordering {
    let degree = |m: &Mono| m.iter().map(|&(_, e)| u64::from(e)).sum::<u64>();
    degree(a).cmp(&degree(b)).then_with(|| {
        let (mut i, mut j) = (0, 0);
        loop {
            match (a.get(i), b.get(j)) {
                (None, None) => return Ordering::Equal,
                (Some(_), None) => return Ordering::Greater,
                (None, Some(_)) => return Ordering::Less,
                (Some(&(ia, ea)), Some(&(ib, eb))) => match ia.cmp(&ib) {
                    Ordering::Equal if ea == eb => {
                        i += 1;
                        j += 1;
                    }
                    Ordering::Equal => return ea.cmp(&eb),
                    // `a` carries `ia` and `b` does not: `a` is larger.
                    Ordering::Less => return Ordering::Greater,
                    Ordering::Greater => return Ordering::Less,
                },
            }
        }
    })
}

/// **`t / r` as monomials**, where `r` divides `t`; `None` where it
/// does not (an exponent of `r` past `t`'s, or an indeterminate of `r`
/// that `t` lacks). One merge over the two sorted vectors.
pub(super) fn mono_div(t: &Mono, r: &Mono) -> Option<Mono> {
    let mut out: Mono = Vec::with_capacity(t.len());
    let mut j = 0;
    for &(id, e) in t {
        if r.get(j).is_some_and(|&(rid, _)| rid < id) {
            return None;
        }
        let re = match r.get(j) {
            Some(&(rid, re)) if rid == id => {
                j += 1;
                re
            }
            _ => 0,
        };
        if re > e {
            return None;
        }
        if e > re {
            out.push((id, e - re));
        }
    }
    (j == r.len()).then_some(out)
}

/// The LEADING term of `p` under [`grlex`] — not the last term in
/// storage order. `None` for the zero polynomial.
pub(super) fn leading(p: &Poly) -> Option<&(Mono, Rat)> {
    p.terms().iter().max_by(|a, b| grlex(&a.0, &b.0))
}

/// The TRAILING term of `p` under [`grlex`]. `None` for the zero
/// polynomial.
pub(super) fn trailing(p: &Poly) -> Option<&(Mono, Rat)> {
    p.terms().iter().min_by(|a, b| grlex(&a.0, &b.0))
}

/// One walk over two vectors sorted by `key`, in key order, appended
/// to `out`: an element whose key is on one side only goes through
/// `one`; a pair with equal keys goes through `both`, whose `None`
/// refuses the whole merge (the ring's or the exponent's refusal) and
/// whose `Some(None)` drops the pair (a sum that cancelled). The two
/// merges of this module — a polynomial sum and a monomial product —
/// are this with different `both`s.
fn merge_sorted<T, K: Ord + ?Sized>(
    a: &[T],
    b: &[T],
    key: impl Fn(&T) -> &K,
    mut both: impl FnMut(&T, &T) -> Option<Option<T>>,
    mut one: impl FnMut(&T) -> T,
    out: &mut Vec<T>,
) -> Option<()> {
    let (mut i, mut j) = (0, 0);
    while i < a.len() && j < b.len() {
        match key(&a[i]).cmp(key(&b[j])) {
            core::cmp::Ordering::Less => {
                out.push(one(&a[i]));
                i += 1;
            }
            core::cmp::Ordering::Greater => {
                out.push(one(&b[j]));
                j += 1;
            }
            core::cmp::Ordering::Equal => {
                out.extend(both(&a[i], &b[j])?);
                i += 1;
                j += 1;
            }
        }
    }
    out.extend(a[i..].iter().map(&mut one));
    out.extend(b[j..].iter().map(&mut one));
    Some(())
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
/// No common-factor cancellation IN THE PLAIN WALK, so there `x²/x`
/// and `x/1` are DIFFERENT forms and an atom over one does not cancel
/// against an atom over the other. (The EARLY walk does cancel the
/// shared monomial and canonicalise the scale since SYM-5 —
/// [`quotient`](super::quotient), `SymRules::common_factor` — which is
/// why the tier with that dial off is the earlier one bit for bit and
/// what is deliberately absent here is absent from the plain form.) That is the conservative direction — a missed cancellation is
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    //! The sorted-vector polynomial's invariant and its digest, on the
    //! cases a vector can handle differently from a map — a monomial
    //! arriving twice, a sum that cancels to zero, the empty
    //! polynomial, the constant term's position, and the products
    //! built in either operand order. Rows from SYM-4's reviews
    //! (`origin/sym/4-review-r2`, with `origin/sym/4-review-r1`'s
    //! differential run beside them), adopted as ordinary rows.
    use super::*;

    fn budget() -> SymBudget {
        SymBudget {
            max_terms: 4096,
            max_degree: 128,
        }
    }

    fn r(a: i128, b: i128) -> Rat {
        Rat::new(a, b, 0).unwrap()
    }

    /// Sorted strictly by monomial, no zero coefficient.
    fn canonical(p: &Poly) -> bool {
        p.terms().windows(2).all(|w| w[0].0 < w[1].0) && p.terms().iter().all(|(_, c)| !c.is_zero())
    }

    fn poly(terms: &[(&[(u128, u32)], Rat)]) -> Poly {
        let mut p = Poly::zero();
        for (m, c) in terms {
            p.insert(m.to_vec(), c.clone()).unwrap();
        }
        p
    }

    #[test]
    fn a_monomial_arriving_twice_merges_and_a_cancelling_sum_drops_the_term() {
        let x: &[(u128, u32)] = &[(7, 1)];
        let mut p = Poly::zero();
        p.insert(x.to_vec(), r(1, 3)).unwrap();
        p.insert(x.to_vec(), r(2, 3)).unwrap();
        assert_eq!(p.terms().len(), 1);
        assert_eq!(p.terms()[0].1, Rat::one());
        assert!(canonical(&p));
        // The same value inserted in the other order, and via `add`.
        let mut q = Poly::zero();
        q.insert(x.to_vec(), r(2, 3)).unwrap();
        q.insert(x.to_vec(), r(1, 3)).unwrap();
        assert_eq!(p, q);
        assert_eq!(p.digest(), q.digest());
        let s = Poly::term(x.to_vec(), r(1, 3))
            .add(&Poly::term(x.to_vec(), r(2, 3)))
            .unwrap();
        assert_eq!(s, p);
        // Cancel to zero, three ways: insert, add, add of neg.
        p.insert(x.to_vec(), r(-1, 1)).unwrap();
        assert!(p.is_zero() && p.terms().is_empty());
        assert_eq!(p, Poly::zero());
        assert_eq!(p.digest(), Poly::zero().digest());
        let t = q.add(&q.neg().unwrap()).unwrap();
        assert!(t.is_zero());
        assert_eq!(t.digest(), Poly::zero().digest());
        assert_eq!(t.as_constant(), Some(Rat::zero()));
        // A middle term cancelling out of a three-term sum keeps the
        // rest sorted and contiguous.
        let a = poly(&[(&[], r(1, 1)), (&[(3, 1)], r(2, 1)), (&[(5, 2)], r(4, 1))]);
        let b = poly(&[(&[(3, 1)], r(-2, 1)), (&[(9, 1)], r(1, 1))]);
        let c = a.add(&b).unwrap();
        assert_eq!(c.terms().len(), 3);
        assert!(canonical(&c));
        assert_eq!(
            c,
            poly(&[(&[], r(1, 1)), (&[(5, 2)], r(4, 1)), (&[(9, 1)], r(1, 1))])
        );
        assert_eq!(c, b.add(&a).unwrap());
    }

    #[test]
    fn the_constant_term_is_first_and_the_empty_polynomial_is_the_zero() {
        // The empty monomial is the least under `Vec`'s lexicographic
        // `Ord`, so the constant term is the first entry — as it was
        // the first key of the map.
        let p = poly(&[(&[(2, 1)], r(3, 1)), (&[], r(5, 1)), (&[(1, 3)], r(1, 1))]);
        assert!(p.terms()[0].0.is_empty());
        assert_eq!(p.terms()[0].1, r(5, 1));
        assert!(canonical(&p));
        assert_eq!(p.as_constant(), None);
        assert_eq!(Poly::constant(r(5, 1)).as_constant(), Some(r(5, 1)));
        assert_eq!(Poly::constant(Rat::zero()), Poly::zero());
        assert_eq!(Poly::term(vec![(4, 1)], Rat::zero()), Poly::zero());
        assert_eq!(Poly::zero().degree(), 0);
        assert_eq!(Poly::zero().add(&Poly::zero()).unwrap(), Poly::zero());
        assert_eq!(Poly::zero().mul(&p, budget()).unwrap(), Poly::zero());
        assert_eq!(p.mul(&Poly::zero(), budget()).unwrap(), Poly::zero());
        assert_eq!(Poly::zero().neg().unwrap(), Poly::zero());
        assert_eq!(p.degree(), 3);
        assert_eq!(
            Poly::one().add(&Poly::one()).unwrap().as_constant(),
            Some(r(2, 1))
        );
    }

    #[test]
    fn products_in_either_operand_order_and_either_association_are_one_polynomial() {
        // (1 + x + y)(1 - x + y²) and its mirror; then a cube two ways.
        let x: &[(u128, u32)] = &[(11, 1)];
        let y: &[(u128, u32)] = &[(13, 1)];
        let y2: &[(u128, u32)] = &[(13, 2)];
        let a = poly(&[(&[], r(1, 1)), (x, r(1, 1)), (y, r(1, 1))]);
        let b = poly(&[(&[], r(1, 1)), (x, r(-1, 1)), (y2, r(1, 1))]);
        let ab = a.mul(&b, budget()).unwrap();
        let ba = b.mul(&a, budget()).unwrap();
        assert!(canonical(&ab));
        assert_eq!(ab, ba);
        assert_eq!(ab.digest(), ba.digest());
        // 1 + x + y - x - x² - xy + y² + xy² + y³ = 1 + y - x² - xy + y² + xy² + y³
        assert_eq!(ab.terms().len(), 7);
        let abc = ab.mul(&a, budget()).unwrap();
        let bca = b.mul(&a.mul(&a, budget()).unwrap(), budget()).unwrap();
        assert_eq!(abc, bca);
        assert_eq!(abc.digest(), bca.digest());
        assert!(canonical(&abc));
        // Sums associate and commute to the bit.
        let s1 = a.add(&b).unwrap().add(&ab).unwrap();
        let s2 = ab.add(&b.add(&a).unwrap()).unwrap();
        assert_eq!(s1, s2);
        assert_eq!(s1.digest(), s2.digest());
        // Coefficients through the dyadic and the non-dyadic shape
        // agree term by term: (1/3)·(3x) == x == 0.5·(2x).
        let third_x = Poly::term(x.to_vec(), r(1, 3));
        let three = Poly::constant(r(3, 1));
        let half_x = Poly::term(x.to_vec(), Rat::of_f64(0.5).unwrap());
        let two = Poly::constant(Rat::of_f64(2.0).unwrap());
        let p = third_x.mul(&three, budget()).unwrap();
        let q = half_x.mul(&two, budget()).unwrap();
        assert_eq!(p, q);
        assert_eq!(p, Poly::indet(11));
        assert_eq!(p.digest(), Poly::indet(11).digest());
    }

    // ---- the leading-term arithmetic: grlex, mono_div, div_exact ----

    /// Every monomial over ids {3, 5, 7} with exponents 0..=3, in the
    /// type's invariant shape (sorted by id, no zero exponent).
    fn monos() -> Vec<Mono> {
        let mut out = Vec::new();
        for a in 0..=3u32 {
            for b in 0..=3u32 {
                for c in 0..=3u32 {
                    out.push(
                        [(3u128, a), (5, b), (7, c)]
                            .into_iter()
                            .filter(|&(_, e)| e > 0)
                            .collect(),
                    );
                }
            }
        }
        out
    }

    /// **`grlex` is a MONOMIAL order**, exhaustively over 64 monomials:
    /// total, antisymmetric, equal exactly on equal monomials,
    /// transitive, multiplicative (`a > b ⇒ a·m > b·m`), with `1` the
    /// least. A tie-break that is not a monomial order reds the
    /// multiplicative clause.
    #[test]
    fn grlex_is_a_monomial_order() {
        let ms = monos();
        for a in &ms {
            assert_ne!(grlex(a, &Mono::new()), Ordering::Less, "1 is the least");
            for b in &ms {
                let ab = grlex(a, b);
                assert_eq!(ab, grlex(b, a).reverse(), "antisymmetric");
                assert_eq!(ab == Ordering::Equal, a == b, "equal iff equal");
                for m in &ms {
                    let am = mono_mul(a, m).unwrap();
                    let bm = mono_mul(b, m).unwrap();
                    assert_eq!(grlex(&am, &bm), ab, "multiplicative: {a:?} {b:?} by {m:?}");
                    if ab == Ordering::Greater && grlex(b, m) == Ordering::Greater {
                        assert_eq!(grlex(a, m), Ordering::Greater, "transitive");
                    }
                }
            }
        }
    }

    /// `mono_div` inverts `mono_mul` and refuses exactly the non-divisors.
    #[test]
    fn mono_div_inverts_mono_mul() {
        let ms = monos();
        for a in &ms {
            for b in &ms {
                let ab = mono_mul(a, b).unwrap();
                assert_eq!(mono_div(&ab, b).as_ref(), Some(a));
                let divides = b.iter().all(|&(id, e)| exp_of(a, id) >= e);
                assert_eq!(mono_div(a, b).is_some(), divides, "{a:?} / {b:?}");
            }
        }
    }

    /// **The leading term is grlex's maximum, not the last in storage
    /// order**: `x² + y + x·y` (`x` id 3, `y` id 5) stores `y` last —
    /// `Mono`'s vector order puts `[(5, 1)]` after both `[(3, …)]` — while
    /// the grlex leading term is `x²` (degree 2, and the larger `x`
    /// exponent of the tie with `x·y`) and the trailing term is `y`.
    #[test]
    fn leading_is_the_grlex_maximum_not_the_stored_last() {
        let mut p = Poly::term(vec![(3, 2)], Rat::one());
        p.insert(vec![(5, 1)], Rat::one()).unwrap();
        p.insert(vec![(3, 1), (5, 1)], Rat::one()).unwrap();
        assert_ne!(
            p.terms().last().map(|(m, _)| m.clone()),
            Some(vec![(3, 2)]),
            "the row needs a storage order whose last term is not the leading one"
        );
        assert_eq!(leading(&p).map(|(m, _)| m.clone()), Some(vec![(3, 2)]));
        assert_eq!(trailing(&p).map(|(m, _)| m.clone()), Some(vec![(5, 1)]));
    }

    /// `x + k` over one indeterminate.
    fn x_plus(k: Rat) -> Poly {
        let mut p = Poly::indet(3);
        p.insert(Mono::new(), k).unwrap();
        p
    }

    /// `x^n − 1` over one indeterminate.
    fn x_pow_minus_one(n: u32) -> Poly {
        let mut p = Poly::term(vec![(3, n)], Rat::one());
        p.insert(Mono::new(), Rat::new(-1, 1, 0).unwrap()).unwrap();
        p
    }

    /// **The step cap is the budget's term cap.** `(x^n − 1)/(x − 1)`
    /// has an `n`-term quotient: under a 50-term budget `n = 50` divides
    /// and `n = 51` declines although `D | N`, and under the tests'
    /// 4096-term budget `n = 600` divides.
    #[test]
    fn the_division_is_capped_at_the_budgets_term_cap() {
        let d = x_plus(Rat::new(-1, 1, 0).unwrap());
        let small = SymBudget {
            max_terms: 50,
            max_degree: 1024,
        };
        let q = x_pow_minus_one(50)
            .div_exact(&d, small)
            .expect("50 terms fit");
        assert_eq!(q.terms().len(), 50);
        assert!(x_pow_minus_one(51).div_exact(&d, small).is_none());
        let big = SymBudget {
            max_terms: 4096,
            max_degree: 1024,
        };
        let q = x_pow_minus_one(600)
            .div_exact(&d, big)
            .expect("600 terms fit");
        assert_eq!(q.terms().len(), 600);
        assert_eq!(q.mul(&d, big).unwrap(), x_pow_minus_one(600));
    }

    /// **A non-zero remainder is a decline, never a quotient**: `(x² + x
    /// + 1)/(x + 1)` leaves the constant `1`, and a division that
    /// dropped it would answer `x`.
    #[test]
    fn a_non_zero_remainder_declines() {
        let d = x_plus(Rat::one());
        let mut n = Poly::term(vec![(3, 2)], Rat::one());
        n.insert(vec![(3, 1)], Rat::one()).unwrap();
        n.insert(Mono::new(), Rat::one()).unwrap();
        assert!(n.div_exact(&d, budget()).is_none());
        let exact = d.mul(&d, budget()).unwrap();
        assert_eq!(exact.div_exact(&d, budget()), Some(d.clone()));
    }

    /// **The ring refusing partway is a decline**: `x^40/(x + 2^100)`
    /// builds `2^(100·j)` coefficients until the ring refuses.
    #[test]
    fn a_ring_refusal_partway_declines() {
        let d = x_plus(Rat::new(1, 1, 100).unwrap());
        let n = Poly::term(vec![(3, 40)], Rat::one());
        assert!(n.div_exact(&d, budget()).is_none());
    }

    /// **A division that is not exact declines BEFORE any step when a
    /// necessary monomial condition fails**: `x³⁴ / (x² − y − z − u −
    /// v)` has `trail(d) = v` (grlex's least degree-1 term) not
    /// dividing `trail(x³⁴) = x³⁴`, so the division takes zero steps —
    /// the shape a rebuild-per-step loop spent the budget's whole cap on.
    #[test]
    fn a_failed_necessary_condition_declines_before_any_step() {
        let mut d = Poly::term(vec![(3, 2)], Rat::one());
        for id in [5, 7, 9, 11] {
            d.insert(vec![(id, 1)], Rat::new(-1, 1, 0).unwrap())
                .unwrap();
        }
        let n = Poly::term(vec![(3, 34)], Rat::one());
        DIV_STEPS.set(0);
        assert!(n.div_exact(&d, budget()).is_none());
        assert_eq!(
            DIV_STEPS.get(),
            0,
            "declined by the monomial conditions alone"
        );
        // And a division that does run takes one step per quotient term.
        let exact = d.mul(&d, budget()).unwrap();
        DIV_STEPS.set(0);
        assert_eq!(exact.div_exact(&d, budget()), Some(d.clone()));
        assert_eq!(DIV_STEPS.get(), d.terms().len() as u64);
    }
}
