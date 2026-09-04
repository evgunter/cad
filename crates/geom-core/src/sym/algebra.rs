//! **The atom algebra** behind [`SymRules`](super::SymRules): the exact
//! polynomial square root rule C tests with, the rule A/B reductions
//! that run over every fresh form, and the clause-3 evaluation of a
//! form at the lane scalar. Every routine here is exact rational
//! arithmetic on the parent module's [`Poly`]/[`Form`] and answers
//! `None` where the parent would freeze — an overflow, a budget, or a
//! shape the rule does not reach — so a rule can only ever FAIL TO
//! FIND a cancellation, never claim one.
//!
//! # Rules A and B are one rewrite
//!
//! Both replace an EVEN power of an atom by a power of a form the atom
//! squared is equal to: `sqrt(X)² → X` and `sin(θ)² → 1 − cos(θ)²`.
//! `reduce` applies that rewrite to every term of a form until no
//! such power remains, and it terminates because every substituted
//! form was built strictly before the atom it replaces — the
//! argument of a `sqrt` is a descendant of the `sqrt` node — so each
//! step trades a square for squares of strictly older atoms, and the
//! DAG is finite. A budget-derived step cap stands behind that
//! argument, and exhausting it freezes.
//!
//! Rule B is the reduction modulo the ideal `⟨sin² + cos² − 1⟩` with
//! `sin²` as the leading term, which is a Gröbner basis of itself: any
//! polynomial in one argument's `sin` and `cos` that lies in the ideal
//! reduces to zero, and nothing outside it does. `sin(2θ) − 2 sinθ
//! cosθ` is a different atom of a different argument and stays.
//!
//! # The monomial order
//!
//! `poly_sqrt` needs a genuine MONOMIAL order (one compatible with
//! multiplication), and the parent's `BTreeMap` key order is not one:
//! its sparse `(id, exponent)` vectors compare `x < y` but `x·x > x·y`.
//! `mono_cmp` is graded lexicographic on the dense exponent vectors,
//! which is.

use core::cmp::Ordering;

use super::{
    AtomInfo, Form, INDET_PI, IndetMap, Mono, ParamValue, Poly, Rat, SignOracle, SymBudget, SymOp,
    SymRules, indet_atom, powi_form, within,
};
use crate::predicate::{Band, Sign};
use crate::real::Real;

/// Graded lexicographic order on monomials: total degree first, then
/// the dense exponent vectors compared at the smallest indeterminate
/// id first, a larger exponent ranking higher. A monomial order.
pub(super) fn mono_cmp(a: &Mono, b: &Mono) -> Ordering {
    let deg = |m: &Mono| m.iter().map(|(_, e)| u64::from(*e)).sum::<u64>();
    deg(a).cmp(&deg(b)).then_with(|| {
        let (mut i, mut j) = (0, 0);
        loop {
            match (a.get(i), b.get(j)) {
                (None, None) => return Ordering::Equal,
                (Some(_), None) => return Ordering::Greater,
                (None, Some(_)) => return Ordering::Less,
                (Some(&(ia, ea)), Some(&(ib, eb))) => match ia.cmp(&ib) {
                    // `a` has a positive exponent on an id where `b`
                    // has zero, and that id is the more significant.
                    Ordering::Less => return Ordering::Greater,
                    Ordering::Greater => return Ordering::Less,
                    Ordering::Equal => {
                        if ea != eb {
                            return ea.cmp(&eb);
                        }
                        i += 1;
                        j += 1;
                    }
                },
            }
        }
    })
}

/// The leading term under `mono_cmp`; `None` for the zero polynomial.
fn leading(p: &Poly) -> Option<(Mono, Rat)> {
    p.terms
        .iter()
        .max_by(|(a, _), (b, _)| mono_cmp(a, b))
        .map(|(m, c)| (m.clone(), *c))
}

/// `a / b` on monomials, or `None` when `b` does not divide `a`.
fn mono_div(a: &Mono, b: &Mono) -> Option<Mono> {
    let mut out = Mono::with_capacity(a.len());
    let mut j = 0;
    for &(ia, ea) in a {
        let eb = match b.get(j) {
            Some(&(ib, eb)) if ib == ia => {
                j += 1;
                eb
            }
            Some(&(ib, _)) if ib < ia => return None,
            _ => 0,
        };
        if eb > ea {
            return None;
        }
        if ea > eb {
            out.push((ia, ea - eb));
        }
    }
    (j == b.len()).then_some(out)
}

/// The monomial with every exponent halved, or `None` if any is odd.
fn mono_half(m: &Mono) -> Option<Mono> {
    m.iter()
        .map(|&(id, e)| (e % 2 == 0).then_some((id, e / 2)))
        .collect()
}

/// **The exact square root of a polynomial**, with a positive leading
/// coefficient — `Some(q)` iff `q·q == p` exactly, else `None`.
///
/// Term by term from the leading term down: with `q` the part of the
/// root found so far and `r = p − q²`, the next term of the root is
/// `LT(r) / (2·LT(q))`, which must divide exactly and rank strictly
/// below `LT(q)`; any failure means `p` is not a square. Every product
/// is budget-checked, and the root has at most `max_terms` terms, so
/// the loop is bounded by the budget rather than by trust.
pub(super) fn poly_sqrt(p: &Poly, budget: SymBudget) -> Option<Poly> {
    if p.is_zero() {
        return Some(Poly::zero());
    }
    let (lm, lc) = leading(p)?;
    let (qm, qc) = (mono_half(&lm)?, lc.sqrt()?);
    let two_qc_inv = qc.add(qc)?.recip()?;
    let mut q = Poly::zero();
    q.insert(qm.clone(), qc)?;
    for _ in 0..=budget.max_terms {
        let r = p.add(&q.mul(&q, budget)?.neg()?)?;
        if r.is_zero() {
            return Some(q);
        }
        let (rm, rc) = leading(&r)?;
        let tm = mono_div(&rm, &qm)?;
        if mono_cmp(&tm, &qm) != Ordering::Less {
            return None;
        }
        q.insert(tm, rc.mul(two_qc_inv)?)?;
        if q.terms.len() > budget.max_terms {
            return None;
        }
    }
    None
}

/// The exact square root of a form — both halves of the quotient
/// perfect squares — carrying the argument's flags.
pub(super) fn form_sqrt(f: &Form, budget: SymBudget) -> Option<Form> {
    if f.poisoned {
        return None;
    }
    Some(Form {
        num: poly_sqrt(&f.num, budget)?,
        den: poly_sqrt(&f.den, budget)?,
        poisoned: false,
        gated: f.gated,
    })
}

/// One reduction the atom algebra can apply to a form.
enum Rewrite {
    /// `id² → x` (rules A and B): an even power of the atom becomes a
    /// power of `x`, one factor left where the power is odd. Reads no
    /// value.
    Square { id: u128, x: Form },
    /// `id → q` (rule C): the atom equals `q` outright, over EVERY
    /// power, on the strength of a certified sign. The result is
    /// gated.
    Whole { id: u128, q: Form },
}

/// The `cos²` of the same argument, as a form: `1 − cos(θ)²` — rule B's
/// substitution for `sin(θ)²`. The `cos` twin's id is
/// `indet_atom(Cos, 0, [arg.digest()])`, the same key a `cos(θ)` node
/// mints, so a `sin` and a `cos` of one argument reduce into one
/// indeterminate and cancel.
fn one_minus_cos_squared(arg: &Form) -> Option<Form> {
    let cos = indet_atom(SymOp::Cos.tag(), 0, &[arg.digest()]);
    let mut cos2 = Poly::zero();
    cos2.insert(vec![(cos, 2)], Rat::new(-1, 1, 0)?)?;
    Some(Form::poly(Poly::one().add(&cos2)?))
}

/// The first reduction any rule can apply to `f`, tried A and B before
/// C so an unconditional rewrite is always preferred to the sign fold.
/// `None` when no rule reaches any atom of `f`.
fn find_rewrite(
    f: &Form,
    rules: SymRules,
    budget: SymBudget,
    atoms: &IndetMap<AtomInfo>,
    oracle: &dyn SignOracle,
    params: &IndetMap<ParamValue>,
    band: Band,
) -> Option<Rewrite> {
    // A and B act on EVEN powers, so scan the monomials for one.
    if rules.sqrt_square || rules.pythagoras {
        for poly in [&f.num, &f.den] {
            for mono in poly.terms.keys() {
                for &(id, e) in mono {
                    if e < 2 {
                        continue;
                    }
                    let Some(info) = atoms.get(&id) else { continue };
                    let Some(arg) = info.args[0].as_ref() else {
                        continue;
                    };
                    match info.op {
                        SymOp::Sqrt if rules.sqrt_square => {
                            return Some(Rewrite::Square {
                                id,
                                x: (**arg).clone(),
                            });
                        }
                        SymOp::Sin if rules.pythagoras => {
                            return Some(Rewrite::Square {
                                id,
                                x: one_minus_cos_squared(arg)?,
                            });
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    // C acts at ANY power, and reads the sign, so it is last.
    if rules.signed_root {
        for poly in [&f.num, &f.den] {
            for mono in poly.terms.keys() {
                for &(id, _e) in mono {
                    let Some(info) = atoms.get(&id) else { continue };
                    let Some(arg) = info.args[0].as_ref() else {
                        continue;
                    };
                    // `sqrt(Q²) = |Q|`, then `|Q| = ±Q` by the sign;
                    // `|X| = ±X` directly. The candidate `Q` (or `X`)
                    // must be a rational function the oracle can read.
                    let candidate = match info.op {
                        SymOp::Sqrt => form_sqrt(arg, budget)?,
                        SymOp::Abs => (**arg).clone(),
                        _ => continue,
                    };
                    if candidate.poisoned || candidate.is_zero() {
                        continue;
                    }
                    match oracle.sign_of(&candidate, params, band) {
                        Some(Sign::Positive) => return Some(Rewrite::Whole { id, q: candidate }),
                        Some(Sign::Negative) => {
                            return Some(Rewrite::Whole {
                                id,
                                q: candidate.neg()?,
                            });
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    None
}

/// Substitutes `id` in one polynomial by `repl` — as a SQUARE
/// (`id^e → repl^(e/2)·id^(e%2)`) or WHOLE (`id^e → repl^e`) — folding
/// each term into the quotient `Form` the substitution produces (a
/// `repl` that is itself a quotient makes the result one).
fn poly_subst(poly: &Poly, id: u128, repl: &Form, square: bool, budget: SymBudget) -> Option<Form> {
    let mut acc = Form::zero();
    for (mono, coeff) in &poly.terms {
        let e = mono.iter().find(|(i, _)| *i == id).map_or(0, |(_, e)| *e);
        let rest: Mono = mono.iter().filter(|(i, _)| *i != id).copied().collect();
        let mut rp = Poly::zero();
        rp.insert(rest, *coeff)?;
        let mut term = Form::poly(rp);
        if e > 0 {
            let mut factor = powi_form(repl, if square { e / 2 } else { e }, budget)?;
            if square && e % 2 == 1 {
                let mut idp = Poly::zero();
                idp.insert(vec![(id, 1)], Rat::new(1, 1, 0)?)?;
                factor = factor.mul(&Form::poly(idp), budget)?;
            }
            term = term.mul(&factor, budget)?;
        }
        acc = acc.add(&term, budget)?;
    }
    Some(acc)
}

/// Applies one [`Rewrite`] to a form: substitute in numerator and
/// denominator, re-form the quotient, and carry the gated flag (a
/// `Whole` rewrite is rule C's, so its result is gated).
fn apply(f: &Form, rw: &Rewrite, budget: SymBudget) -> Option<Form> {
    let (id, repl, square, gated) = match rw {
        Rewrite::Square { id, x } => (*id, x, true, false),
        Rewrite::Whole { id, q } => (*id, q, false, true),
    };
    let num = poly_subst(&f.num, id, repl, square, budget)?;
    let den = poly_subst(&f.den, id, repl, square, budget)?;
    let mut out = num.mul(&den.recip()?, budget)?;
    out.gated = f.gated || gated || repl.gated;
    Some(out)
}

/// The most substitutions `reduce` takes before it FREEZES. Each step
/// removes one atom occurrence (a `Whole` erases the atom; a `Square`
/// lowers its power by two and can only reintroduce strictly-older
/// atoms), so a real residual reduces in a handful; a form that needs
/// more than this is a pathological product the budget would freeze
/// anyway, and freezing early bounds the cost.
const REDUCE_STEPS: usize = 256;

/// **The atom algebra over one residual** (module docs): apply rules
/// A, B and C — as chosen by `rules` — to `f` until no rule reaches an
/// atom, and answer the reduced form. Runs ONCE per near-zero margin,
/// over the top residual only, never per DAG node — so its cost is
/// bounded and paid only where the plain form did not already decide.
///
/// `None` is a FREEZE (the reduction ran past the step or size budget);
/// the untouched form otherwise. The `gated` flag on the result says
/// whether a clause-3 fold participated, which is what separates a
/// sign-gated discharge from an unconditional one.
pub(super) fn reduce(
    f: &Form,
    rules: SymRules,
    budget: SymBudget,
    atoms: &IndetMap<AtomInfo>,
    oracle: &dyn SignOracle,
    params: &IndetMap<ParamValue>,
    band: Band,
) -> Option<Form> {
    if f.poisoned || rules == SymRules::none() {
        return Some(f.clone());
    }
    let mut cur = f.clone();
    for _ in 0..REDUCE_STEPS {
        let Some(rw) = find_rewrite(&cur, rules, budget, atoms, oracle, params, band) else {
            return Some(cur);
        };
        cur = apply(&cur, &rw, budget)?;
        if cur.poisoned {
            return Some(cur);
        }
        if !within(budget, &cur) {
            return None;
        }
    }
    None
}

/// **Clause 3's evaluation**: the value of `q` at the lane scalar `T`,
/// over the session's parameter values — `None` where `q` mentions an
/// indeterminate that is not a parameter or π, or a coefficient the
/// scalar cannot take exactly.
pub(super) fn eval_form<T: Real>(q: &Form, params: &IndetMap<ParamValue>) -> Option<T> {
    if q.poisoned {
        return None;
    }
    let num = eval_poly::<T>(&q.num, params)?;
    let den = eval_poly::<T>(&q.den, params)?;
    Some(num / den)
}

fn eval_poly<T: Real>(p: &Poly, params: &IndetMap<ParamValue>) -> Option<T> {
    let mut acc = T::zero();
    for (m, c) in &p.terms {
        let mut term = coefficient::<T>(*c)?;
        for &(id, e) in m {
            let v: T = if id == INDET_PI {
                T::pi()
            } else {
                *params.get(&id)?.downcast_ref::<T>()?
            };
            term = term * v.powi(i32::try_from(e).ok()?);
        }
        acc = acc + term;
    }
    Some(acc)
}

/// An exact rational as the scalar: `num / den · 2^exp2` with every
/// factor representable, else `None`. The scalar's own division and
/// scaling do the rest — at an interval that is outward-rounded, which
/// is what makes the evaluation an ENCLOSURE.
fn coefficient<T: Real>(c: Rat) -> Option<T> {
    const EXACT: i128 = 1 << 53;
    if c.num.abs() >= EXACT || c.den >= EXACT || !(-1000..=1000).contains(&c.exp2) {
        return None;
    }
    let scale = T::from_f64(2f64.powi(c.exp2));
    Some(T::from_f64(c.num as f64) / T::from_f64(c.den as f64) * scale)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    fn budget() -> SymBudget {
        SymBudget {
            max_terms: 4096,
            max_degree: 128,
        }
    }

    fn var(id: u128) -> Poly {
        Poly::indet(id)
    }

    fn scaled(p: &Poly, k: i128) -> Poly {
        let mut out = Poly::zero();
        for (m, c) in &p.terms {
            out.insert(m.clone(), c.mul(Rat::new(k, 1, 0).unwrap()).unwrap())
                .unwrap();
        }
        out
    }

    /// `(2x + 3y + 5)²` has the root it was built from, with a positive
    /// leading coefficient; its negation does not.
    #[test]
    fn a_perfect_square_recovers_its_root() {
        let (x, y) = (var(7), var(11));
        let q = scaled(&x, 2)
            .add(&scaled(&y, 3))
            .unwrap()
            .add(&scaled(&Poly::one(), 5))
            .unwrap();
        let p = q.mul(&q, budget()).unwrap();
        assert_eq!(poly_sqrt(&p, budget()).unwrap(), q);
        let neg = scaled(&q, -1);
        assert_eq!(
            poly_sqrt(&neg.mul(&neg, budget()).unwrap(), budget()).unwrap(),
            q
        );
        assert!(poly_sqrt(&scaled(&p, -1), budget()).is_none());
    }

    /// `x² + 1`, `2x²` and `x² + 2xy` are not squares over the
    /// rationals; `4x²·y⁴` is.
    #[test]
    fn a_non_square_is_refused() {
        let (x, y) = (var(7), var(11));
        let x2 = x.mul(&x, budget()).unwrap();
        assert!(poly_sqrt(&x2.add(&Poly::one()).unwrap(), budget()).is_none());
        assert!(poly_sqrt(&scaled(&x2, 2), budget()).is_none());
        let xy2 = scaled(&x.mul(&y, budget()).unwrap(), 2);
        assert!(poly_sqrt(&x2.add(&xy2).unwrap(), budget()).is_none());
        let y2 = y.mul(&y, budget()).unwrap();
        let sq = scaled(
            &x2.mul(&y2, budget()).unwrap().mul(&y2, budget()).unwrap(),
            4,
        );
        let root = poly_sqrt(&sq, budget()).unwrap();
        assert_eq!(root.mul(&root, budget()).unwrap(), sq);
    }

    /// The order is a monomial order: `a < b` implies `a·m < b·m`, on
    /// the pair the map's own key order gets wrong.
    #[test]
    fn the_monomial_order_respects_multiplication() {
        let (x, y): (Mono, Mono) = (vec![(7, 1)], vec![(11, 1)]);
        assert_eq!(mono_cmp(&x, &y), Ordering::Greater);
        let xx = super::super::mono_mul(&x, &x).unwrap();
        let xy = super::super::mono_mul(&x, &y).unwrap();
        assert_eq!(mono_cmp(&xx, &xy), Ordering::Greater);
        assert_eq!(mono_cmp(&xx, &x), Ordering::Greater, "degree first");
    }

    #[test]
    fn the_integer_square_root_is_exact() {
        for v in [0u128, 1, 4, 9, 1 << 100, (1u128 << 63) * (1u128 << 63)] {
            let r = super::super::exact_isqrt(v).unwrap();
            assert_eq!(r * r, v);
        }
        for v in [2u128, 3, 5, 8, (1 << 100) + 1] {
            assert!(super::super::exact_isqrt(v).is_none(), "{v}");
        }
    }
}
