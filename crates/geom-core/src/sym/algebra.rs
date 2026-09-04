//! **The atom algebra** behind [`SymRules`](super::SymRules): the
//! rule-A/B reduction of the top RESIDUAL a decide site tests. Every
//! routine here is exact rational arithmetic on the parent module's
//! [`Poly`]/[`Form`] and answers `None` where the parent would freeze —
//! an overflow, a budget, or a shape the rule does not reach — so a rule
//! can only ever FAIL TO FIND a cancellation, never claim one.
//!
//! # The two buildable rules are one rewrite
//!
//! Both replace an EVEN power of an atom by a power of a form the atom
//! squared is equal to: **A** `sqrt(X)² → X` and **B** `sin(θ)² → 1 −
//! cos(θ)²`. `reduce` applies that rewrite to the residual until no
//! such power remains. It terminates because every substituted form was
//! built strictly before the atom it replaces — the argument of a
//! `sqrt` is a descendant of the `sqrt` node — so each step trades a
//! square for squares of strictly older atoms, and the DAG is finite; a
//! step cap and the size budget stand behind that argument.
//!
//! Neither rule reads a value: rule C (`sqrt(Q²)=Q` by a certified sign,
//! clause 3) is FILED UNBUILT ([`SymRules`](super::SymRules)'s docs), so
//! nothing here evaluates a form at the lane scalar.
//!
//! # Only the top residual
//!
//! `reduce` runs ONCE per near-zero margin, over the residual the decide
//! site tests — never per DAG node. It reaches atoms that appear IN the
//! residual, not atoms nested inside another atom's argument or inside a
//! frozen subform; those it leaves opaque, which is the conservative
//! direction. That reach is why the shipped default files the rules: on
//! curved geometry the arc-family subforms freeze before the residual
//! reaches them (M10-8's §1 measurement).

use super::{
    AtomInfo, Form, IndetMap, Mono, Poly, Rat, SymBudget, SymOp, SymRules, indet_atom, powi_form,
    within,
};

/// One reduction the atom algebra can apply: `id² → x`, an even power of
/// the atom replaced by a power of `x` (rule A's `x` is the `sqrt`'s
/// argument; rule B's is `1 − cos²`), one factor of the atom left where
/// the power is odd.
struct Square {
    id: u128,
    x: Form,
}

/// The `1 − cos(θ)²` of one argument, as a form — rule B's substitution
/// for `sin(θ)²`. The `cos` twin's id is `indet_atom(Cos, 0,
/// [arg.digest()])`, the same key a `cos(θ)` node mints, so a `sin` and
/// a `cos` of one argument reduce into one indeterminate and cancel.
fn one_minus_cos_squared(arg: &Form) -> Option<Form> {
    let cos = indet_atom(SymOp::Cos.tag(), 0, &[arg.digest()]);
    let mut cos2 = Poly::zero();
    cos2.insert(vec![(cos, 2)], Rat::new(-1, 1, 0)?)?;
    Some(Form::poly(Poly::one().add(&cos2)?))
}

/// The first reduction any enabled rule can apply to `f` — a `sqrt`
/// atom (rule A) or a `sin` atom (rule B) appearing to an EVEN power.
/// `None` when no rule reaches an even-power atom of `f`.
fn find_square(f: &Form, rules: SymRules, atoms: &IndetMap<AtomInfo>) -> Option<Square> {
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
                        return Some(Square {
                            id,
                            x: (**arg).clone(),
                        });
                    }
                    SymOp::Sin if rules.pythagoras => {
                        return Some(Square {
                            id,
                            x: one_minus_cos_squared(arg)?,
                        });
                    }
                    _ => {}
                }
            }
        }
    }
    None
}

/// Substitutes `id²` by `repl` in one polynomial: `id^e →
/// repl^(e/2)·id^(e%2)`, folding each term into the quotient `Form` the
/// substitution produces (a `repl` that is itself a quotient makes the
/// result one).
fn poly_subst_square(poly: &Poly, id: u128, repl: &Form, budget: SymBudget) -> Option<Form> {
    let mut acc = Form::zero();
    for (mono, coeff) in &poly.terms {
        let e = mono.iter().find(|(i, _)| *i == id).map_or(0, |(_, e)| *e);
        let rest: Mono = mono.iter().filter(|(i, _)| *i != id).copied().collect();
        let mut rp = Poly::zero();
        rp.insert(rest, *coeff)?;
        let mut term = Form::poly(rp);
        if e > 0 {
            let mut factor = powi_form(repl, e / 2, budget)?;
            if e % 2 == 1 {
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

/// Applies one square reduction: substitute in numerator and
/// denominator, re-form the quotient.
fn apply(f: &Form, sq: &Square, budget: SymBudget) -> Option<Form> {
    let num = poly_subst_square(&f.num, sq.id, &sq.x, budget)?;
    let den = poly_subst_square(&f.den, sq.id, &sq.x, budget)?;
    num.mul(&den.recip()?, budget)
}

/// The most substitutions `reduce` takes before it FREEZES. Each step
/// removes one atom occurrence (lowers its power by two and can only
/// reintroduce strictly-older atoms), so a real residual reduces in a
/// handful; a form that needs more than this is a pathological product
/// the budget would freeze anyway, and freezing early bounds the cost.
const REDUCE_STEPS: usize = 256;

/// **The atom algebra over one residual** (module docs): apply rules A
/// and B — as chosen by `rules` — to `f` until no rule reaches an
/// even-power atom, and answer the reduced form. `None` is a FREEZE (the
/// reduction ran past the step or size budget); the untouched form
/// otherwise.
pub(super) fn reduce(
    f: &Form,
    rules: SymRules,
    budget: SymBudget,
    atoms: &IndetMap<AtomInfo>,
) -> Option<Form> {
    if f.poisoned || rules == SymRules::none() {
        return Some(f.clone());
    }
    let mut cur = f.clone();
    for _ in 0..REDUCE_STEPS {
        let Some(sq) = find_square(&cur, rules, atoms) else {
            return Some(cur);
        };
        cur = apply(&cur, &sq, budget)?;
        if cur.poisoned {
            return Some(cur);
        }
        if !within(budget, &cur) {
            return None;
        }
    }
    None
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

    /// `id² → x` on a two-term residual: `a·id² + b → a·x + b`, one
    /// factor of the atom kept where the power is odd.
    #[test]
    fn a_square_substitution_lowers_the_power() {
        // Residual `id² − x` for a sqrt atom whose argument form is `x`.
        let x = Poly::indet(7);
        let atom = indet_atom(SymOp::Sqrt.tag(), 0, &[Form::poly(x.clone()).digest()]);
        let mut atoms: IndetMap<AtomInfo> = IndetMap::default();
        atoms.insert(
            atom,
            AtomInfo {
                op: SymOp::Sqrt,
                args: [Some(std::rc::Rc::new(Form::poly(x.clone()))), None],
            },
        );
        let mut resid = Poly::zero();
        resid
            .insert(vec![(atom, 2)], Rat::new(1, 1, 0).unwrap())
            .unwrap();
        let f = Form::poly(resid)
            .add(&Form::poly(x).neg().unwrap(), budget())
            .unwrap();
        let out = reduce(&f, SymRules::all(), budget(), &atoms).unwrap();
        assert!(out.is_zero(), "sqrt(x)² − x reduces to zero");
    }
}
