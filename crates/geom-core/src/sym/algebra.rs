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
//! [`reduce`] applies that rewrite to every term of a form until no
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
//! [`poly_sqrt`] needs a genuine MONOMIAL order (one compatible with
//! multiplication), and the parent's `BTreeMap` key order is not one:
//! its sparse `(id, exponent)` vectors compare `x < y` but `x·x > x·y`.
//! [`mono_cmp`] is graded lexicographic on the dense exponent vectors,
//! which is.

use core::cmp::Ordering;
use std::rc::Rc;

use super::{
    AtomInfo, Form, INDET_PI, IndetMap, Mono, ParamValue, Poly, Rat, Session, SymBudget, SymOp,
    indet_atom, powi_form, within,
};
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

/// The leading term under [`mono_cmp`]; `None` for the zero polynomial.
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

/// What the square of atom `id` equals, under the session's dials, or
/// `None` when no rule reaches it. The `cos` twin rule B introduces is
/// recorded as an atom so the shape report can render it.
fn square_of(id: u128, sess: &mut Session) -> Option<Form> {
    let rules = sess.rules;
    let (op, arg) = {
        let info = sess.atoms.get(&id)?;
        (info.op, info.args[0].clone()?)
    };
    match op {
        SymOp::Sqrt if rules.sqrt_square => Some((*arg).clone()),
        SymOp::Sin if rules.pythagoras => {
            // `sin` and `cos` nodes carry a zero payload, so the twin's
            // key is the same digest under the other tag.
            let cos = indet_atom(SymOp::Cos.tag(), 0, &[arg.digest()]);
            sess.atoms.entry(cos).or_insert_with(|| AtomInfo {
                op: SymOp::Cos,
                args: [Some(Rc::clone(&arg)), None],
            });
            let mut cos2 = Poly::zero();
            cos2.insert(vec![(cos, 2)], Rat::new(-1, 1, 0)?)?;
            Some(Form::poly(Poly::one().add(&cos2)?).gated(arg.gated))
        }
        _ => None,
    }
}

/// One term of `p` carrying an even-or-higher power of a reducible
/// atom: `(monomial, coefficient, atom id, exponent, what the square
/// equals)`.
fn find_square(p: &Poly, sess: &mut Session) -> Option<(Mono, Rat, u128, u32, Form)> {
    for (m, c) in &p.terms {
        for &(id, e) in m {
            if e >= 2
                && let Some(sq) = square_of(id, sess)
            {
                return Some((m.clone(), *c, id, e, sq));
            }
        }
    }
    None
}

/// **Rules A and B** over one form (module docs): every even power of
/// a `sqrt` or `sin` atom is substituted out, in both halves of the
/// quotient, until none remains. Answers the input untouched when no
/// rule is on or none applies, and `None` — a freeze — when the
/// substitution runs past the budget.
pub(super) fn reduce(mut f: Form, sess: &mut Session) -> Option<Form> {
    let rules = sess.rules;
    if !(rules.sqrt_square || rules.pythagoras) || f.poisoned {
        return Some(f);
    }
    let budget = sess.budget;
    // Every step removes one square; a form within budget holds at
    // most `max_terms` terms of degree at most `max_degree`, so this is
    // the most squares one could ever hold. Standing behind the
    // termination argument, not replacing it.
    let cap = budget.max_terms.saturating_mul(budget.max_degree as usize);
    for _ in 0..=cap {
        let in_den;
        let found = if let Some(x) = find_square(&f.num, sess) {
            in_den = false;
            x
        } else if let Some(x) = find_square(&f.den, sess) {
            in_den = true;
            x
        } else {
            return Some(f);
        };
        let (mono, coeff, id, e, sq) = found;
        let target = if in_den { &f.den } else { &f.num };
        let mut rest = target.clone();
        rest.terms.remove(&mono);
        // The term with the square stripped out: an odd exponent keeps
        // one factor of the atom, in its sorted slot.
        let m: Mono = mono
            .iter()
            .filter_map(|&(i, k)| {
                if i != id {
                    Some((i, k))
                } else {
                    (e % 2 == 1).then_some((i, 1))
                }
            })
            .collect();
        let mut t = Poly::zero();
        t.insert(m, coeff)?;
        let t = Form::poly(t).mul(&powi_form(&sq, e / 2, budget)?, budget)?;
        let replaced = Form::poly(rest).add(&t, budget)?;
        f = if in_den {
            Form::poly(f.num.clone())
                .gated(f.gated)
                .mul(&replaced.recip()?, budget)?
        } else {
            replaced
                .gated(f.gated)
                .mul(&Form::poly(f.den.clone()).recip()?, budget)?
        };
        if f.poisoned {
            return Some(f);
        }
        if !within(budget, &f) {
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
