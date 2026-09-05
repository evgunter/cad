//! **Rule C — the clause-3 fold** behind [`SymRules::signed_root`]:
//! `sqrt(X) → R` where `X = R²` as forms and `R` has a CERTIFIED sign
//! over the leaf's box, and `abs(R) → ±R` likewise. This is the one
//! rule of the atom algebra that reads a value, and this module is the
//! whole of how it reads one.
//!
//! # What is read, and through which door
//!
//! Nothing at the lane scalar. The session records, per document
//! parameter, the `f64` BRACKET the analysis box handed it
//! ([`Sym::param_over`](super::Sym::param_over) — the caller that mints
//! a parameter axis already holds `(lo, hi)` as two `f64`s), and the
//! candidate `R` is enclosed over those brackets in the always-compiled,
//! outward-rounded [`RingInterval`]. No type is punned, no feature is
//! gated, no bound is added: `R` is a polynomial in the parameters and
//! `π`, evaluated in the ring; a form with any other indeterminate (an
//! opaque real, an atom, a frozen node) is not enclosable and the fold
//! declines.
//!
//! # Why it is sound (clause 3 of the theorem)
//!
//! For every point `p` of the box, `sqrt(R(p)²) = |R(p)|`, and a
//! bracket of `R` that is strictly positive (negative) over the box
//! makes `|R(p)| = R(p)` (`−R(p)`) on all of it. So the folded form is
//! equal to the atom AT EVERY POINT OF THE BOX — not identically in the
//! parameters. A zero reached through it is therefore a theorem
//! CONDITIONAL on the sign read, which is why such a discharge is
//! counted `sign_gated` and never `symbolic_zero`: the two claims differ
//! in kind, and the receipt keeps them apart. A bracket that straddles
//! zero, or a poisoned one, folds nothing; the atom stays opaque and the
//! numeric channel answers, which is the conservative direction.
//!
//! # Where it runs
//!
//! Per node, in the EARLY memo only ([`super::early_form`]) — never in
//! the plain form — so a plain theorem is never re-labelled as gated,
//! and the fold reaches atoms nested inside other atoms' arguments,
//! which is where the arc family's `sqrt` of a perfect square sits
//! (`‖q − c‖ = r` has `(a + 2r)²` under its root on the plate).

use core::f64::consts::PI;

use super::{Form, INDET_PI, IndetMap, Mono, Poly, Rat, SymBudget, SymOp};
use crate::ring_interval::RingInterval;

/// The most terms a candidate root may grow to before `poly_sqrt` gives
/// up: a real residual's root is a handful of terms, and the bound keeps
/// a non-square polynomial from being chased term by term.
const ROOT_TERMS: usize = 64;

/// The exponent of `id` in `m` (zero where absent).
fn exp_of(m: &Mono, id: u128) -> u32 {
    m.iter().find(|(i, _)| *i == id).map_or(0, |(_, e)| *e)
}

/// A graded-lexicographic comparison: total degree first, then the
/// exponent vector over `ids` — a monomial order, which the
/// leading-term recurrence below needs (the map's own `Vec` order is
/// not one). Allocation-free: it runs over every term of every
/// argument the early walk offers, and the first cut's per-term key
/// vector was the cost of the whole walk.
fn cmp_mono(a: &Mono, b: &Mono, ids: &[u128]) -> core::cmp::Ordering {
    let deg = |m: &Mono| m.iter().map(|(_, e)| *e).sum::<u32>();
    deg(a).cmp(&deg(b)).then_with(|| {
        for &id in ids {
            let o = exp_of(a, id).cmp(&exp_of(b, id));
            if o != core::cmp::Ordering::Equal {
                return o;
            }
        }
        core::cmp::Ordering::Equal
    })
}

/// Every indeterminate id of `p`, sorted.
fn ids_of(p: &Poly) -> Vec<u128> {
    let mut ids: Vec<u128> = p
        .terms
        .keys()
        .flat_map(|m| m.iter().map(|(i, _)| *i))
        .collect();
    ids.sort_unstable();
    ids.dedup();
    ids
}

/// The leading term of `p` under the graded-lex order over `ids`.
fn lead<'a>(p: &'a Poly, ids: &[u128]) -> Option<(&'a Mono, Rat)> {
    p.terms
        .iter()
        .max_by(|(a, _), (b, _)| cmp_mono(a, b, ids))
        .map(|(m, c)| (m, c.clone()))
}

/// The monomial whose square is `m`, if every exponent is even.
fn mono_sqrt(m: &Mono) -> Option<Mono> {
    m.iter()
        .map(|&(i, e)| (e % 2 == 0).then_some((i, e / 2)))
        .collect()
}

/// `t / r` as monomials, if `r` divides `t`.
fn mono_div(t: &Mono, r: &Mono) -> Option<Mono> {
    let mut out: Mono = Vec::with_capacity(t.len());
    for &(i, e) in t {
        let re = r.iter().find(|(j, _)| *j == i).map_or(0, |(_, e)| *e);
        if re > e {
            return None;
        }
        if e - re > 0 {
            out.push((i, e - re));
        }
    }
    // Every factor of `r` must appear in `t`.
    if r.iter().any(|(j, _)| !t.iter().any(|(i, _)| i == j)) {
        return None;
    }
    Some(out)
}

/// **The exact polynomial square root**: `Some(r)` with `r² == x` as
/// polynomials over the rationals, or `None` where `x` is not a perfect
/// square (or the search runs past its bounds). The classical leading-
/// term recurrence: the root's leading term is the root of `x`'s
/// leading term, and each further term is the leading term of the
/// remainder `x − r²` divided by twice the root's leading term. Under a
/// monomial order the recurrence is unique, so `r² == x` at the end is
/// both necessary and sufficient, and it is checked rather than
/// assumed.
///
/// Cost matters here more than anywhere else in the tier: this runs at
/// EVERY `sqrt` node of the early walk, and nearly every argument is
/// not a square. So the non-squares are turned away cheaply — the
/// trailing term must be a square too (a necessary condition that
/// costs one coefficient root), the root can never carry more terms
/// than `x` (`r²`'s terms `lead(r)·t` are distinct for distinct `t`),
/// and the remainder is updated incrementally (`−2·r·t − t²` per new
/// term `t`) rather than re-squared.
pub(super) fn poly_sqrt(x: &Poly, budget: SymBudget) -> Option<Poly> {
    if x.is_zero() {
        return Some(Poly::zero());
    }
    let ids = ids_of(x);
    let (lm, lc) = lead(x, &ids)?;
    let r0m = mono_sqrt(lm)?;
    let r0c = lc.sqrt_exact()?;
    // The trailing term of a square is the square of the root's
    // trailing term: an odd exponent or a non-square coefficient there
    // settles it without building anything.
    let (tm, tc) = trail(x, &ids)?;
    mono_sqrt(tm)?;
    tc.sqrt_exact()?;
    // And a square polynomial takes a square VALUE at every rational
    // point: one evaluation at small odd integers turns away nearly
    // every non-square before the recurrence is run at all (a
    // non-square that happens to evaluate to a square there is merely
    // handed on to the recurrence, which is exact).
    if !is_square_at_a_point(x, &ids) {
        return None;
    }
    let twice = r0c.add(&r0c)?;
    let mut root = Poly::zero();
    root.insert(r0m.clone(), r0c.clone())?;
    let mut rem = x.add(
        &Poly::constant(r0c.mul(&r0c)?)
            .mul(&mono_poly(&r0m, 2), budget)?
            .neg()?,
    )?;
    let cap = x.terms.len().min(ROOT_TERMS);
    while !rem.is_zero() {
        if root.terms.len() >= cap {
            return None;
        }
        let (tm, tc) = lead(&rem, &ids)?;
        let nm = mono_div(tm, &r0m)?;
        let nc = tc.mul(&twice.recip()?)?;
        // rem -= 2·root·t + t²
        let t = single(nm.clone(), nc.clone())?;
        let two_root_t = root
            .mul(&t, budget)?
            .mul(&Poly::constant(Rat::new(2, 1, 0)?), budget)?;
        rem = rem.add(&two_root_t.neg()?)?;
        rem = rem.add(&t.mul(&t, budget)?.neg()?)?;
        root.insert(nm, nc)?;
    }
    Some(root)
}

/// Whether `x` evaluates to a perfect-square rational at the point
/// `id_k = 2k + 3` — a necessary condition for `x` to be a square
/// polynomial, checked in exact arithmetic.
fn is_square_at_a_point(x: &Poly, ids: &[u128]) -> bool {
    let value_of = |id: u128| -> i128 {
        let k = ids.iter().position(|i| *i == id).unwrap_or(0);
        2 * k as i128 + 3
    };
    let mut acc = Rat::zero();
    for (m, c) in &x.terms {
        let mut term = c.clone();
        for &(id, e) in m {
            let v = value_of(id);
            let mut pw = 1i128;
            for _ in 0..e.min(40) {
                pw = match pw.checked_mul(v) {
                    Some(p) => p,
                    None => return true,
                };
            }
            if e > 40 {
                return true;
            }
            term = match term.mul(&Rat::new(pw, 1, 0).unwrap_or_else(Rat::one)) {
                Some(t) => t,
                None => return true,
            };
        }
        acc = match acc.add(&term) {
            Some(a) => a,
            None => return true,
        };
    }
    acc.sqrt_exact().is_some()
}

/// The trailing term of `p` under the graded-lex order over `ids`.
fn trail<'a>(p: &'a Poly, ids: &[u128]) -> Option<(&'a Mono, Rat)> {
    p.terms
        .iter()
        .min_by(|(a, _), (b, _)| cmp_mono(a, b, ids))
        .map(|(m, c)| (m, c.clone()))
}

/// The one-term polynomial `c · m`.
fn single(m: Mono, c: Rat) -> Option<Poly> {
    let mut p = Poly::zero();
    p.insert(m, c)?;
    Some(p)
}

/// The monomial `m^e` as a polynomial with coefficient one.
fn mono_poly(m: &Mono, e: u32) -> Poly {
    let mut p = Poly::zero();
    let powered: Mono = m.iter().map(|&(i, k)| (i, k * e)).collect();
    p.terms.insert(powered, Rat::one());
    p
}

/// A rational coefficient as a ring enclosure ([`Rat::f64_bracket`]):
/// poison where the value is out of `f64`'s range rather than a flushed
/// zero, which would not be conservative.
fn rat_enclosure(c: &Rat) -> RingInterval {
    match c.f64_bracket() {
        Some((lo, hi)) => RingInterval::from_bounds(lo, hi),
        None => RingInterval::poison(),
    }
}

/// The enclosure of `p` over the parameter brackets, or `None` where
/// `p` carries an indeterminate no bracket is known for.
fn enclose(p: &Poly, params: &IndetMap<(f64, f64)>) -> Option<RingInterval> {
    let mut acc = RingInterval::zero();
    for (m, c) in &p.terms {
        let mut term = rat_enclosure(c);
        for &(id, e) in m {
            let x = if id == INDET_PI {
                RingInterval::from_bounds(PI.next_down(), PI.next_up())
            } else {
                let &(lo, hi) = params.get(&id)?;
                RingInterval::from_bounds(lo, hi)
            };
            term = term * x.powi(i32::try_from(e).ok()?);
        }
        acc = acc + term;
    }
    Some(acc)
}

/// The certified sign of the quotient `num / den` over the brackets:
/// `Some(true)` for strictly positive, `Some(false)` for strictly
/// negative, `None` otherwise (straddling, poisoned, or not
/// enclosable).
fn certified_sign(num: &Poly, den: &Poly, params: &IndetMap<(f64, f64)>) -> Option<bool> {
    let n = enclose(num, params)?;
    let d = enclose(den, params)?;
    let sign = |r: RingInterval| -> Option<bool> {
        if r.is_poison() {
            None
        } else if r.lo() > 0.0 {
            Some(true)
        } else if r.hi() < 0.0 {
            Some(false)
        } else {
            None
        }
    };
    Some(sign(n)? == sign(d)?)
}

/// **The fold**: for `op` applied to the argument form `a`, the form
/// the atom is equal to over the box, GATED — or `None` where the rule
/// does not apply (not a perfect square, no certified sign, not
/// enclosable). Only `Sqrt` and `Abs` are reached.
pub(super) fn fold(
    op: SymOp,
    a: &Form,
    params: &IndetMap<(f64, f64)>,
    budget: SymBudget,
) -> Option<Form> {
    if a.poisoned || params.is_empty() {
        return None;
    }
    // Enclosability first, and cheaply: the root's indeterminates are
    // the argument's, so an argument carrying any id that is not a
    // parameter or π (an atom, an opaque real, a frozen node) can never
    // be signed, whatever else is true of it. This is the test nearly
    // every argument of a real document fails, and it costs one pass
    // over the ids where the polynomial root would cost the recurrence.
    let enclosable = |p: &Poly| {
        p.terms.keys().all(|m| {
            m.iter()
                .all(|&(id, _)| id == INDET_PI || params.contains_key(&id))
        })
    };
    if !(enclosable(&a.num) && enclosable(&a.den)) {
        return None;
    }
    let (num, den) = match op {
        SymOp::Sqrt => (poly_sqrt(&a.num, budget)?, poly_sqrt(&a.den, budget)?),
        SymOp::Abs => (a.num.clone(), a.den.clone()),
        _ => return None,
    };
    if den.is_zero() {
        return None;
    }
    let positive = certified_sign(&num, &den, params)?;
    let mut out = Form::quotient(num, den);
    if !positive {
        out = out.neg()?;
    }
    out.gated = true;
    Some(out)
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

    fn rat(n: i128, d: i128) -> Rat {
        Rat::new(n, d, 0).unwrap()
    }

    /// `(a + 2r)²` as a polynomial over ids 1 (`a`) and 2 (`r`).
    fn square_of_linear() -> (Poly, Poly) {
        let mut root = Poly::zero();
        root.insert(vec![(1, 1)], rat(1, 1)).unwrap();
        root.insert(vec![(2, 1)], rat(2, 1)).unwrap();
        let sq = root.mul(&root, budget()).unwrap();
        (root, sq)
    }

    #[test]
    fn a_perfect_square_polynomial_has_its_root_recovered() {
        let (root, sq) = square_of_linear();
        assert_eq!(poly_sqrt(&sq, budget()).unwrap(), root);
    }

    #[test]
    fn a_non_square_polynomial_has_no_root() {
        let (_, mut sq) = square_of_linear();
        // `(a + 2r)² + 1` is not a square; neither is `a·r`.
        sq.insert(Mono::new(), rat(1, 1)).unwrap();
        assert!(poly_sqrt(&sq, budget()).is_none());
        let mut ar = Poly::zero();
        ar.insert(vec![(1, 1), (2, 1)], rat(1, 1)).unwrap();
        assert!(poly_sqrt(&ar, budget()).is_none());
        // A square coefficient is required too: `2·a²` has no rational root.
        let mut two_a2 = Poly::zero();
        two_a2.insert(vec![(1, 2)], rat(2, 1)).unwrap();
        assert!(poly_sqrt(&two_a2, budget()).is_none());
    }

    fn params(r: (f64, f64)) -> IndetMap<(f64, f64)> {
        let mut m = IndetMap::default();
        m.insert(1, (0.5, 0.5));
        m.insert(2, r);
        m
    }

    #[test]
    fn the_fold_takes_a_certified_positive_root_and_negates_a_negative_one() {
        let (root, sq) = square_of_linear();
        let a = Form::poly(sq);
        // r in [1, 2]: a + 2r > 0 → sqrt folds to the root itself.
        let f = fold(SymOp::Sqrt, &a, &params((1.0, 2.0)), budget()).unwrap();
        assert!(f.gated);
        assert_eq!(f.num, root);
        // r in [-2, -1]: a + 2r < 0 → the fold is the NEGATED root.
        let g = fold(SymOp::Sqrt, &a, &params((-2.0, -1.0)), budget()).unwrap();
        assert!(g.gated);
        assert_eq!(g.num, root.neg().unwrap());
        // abs of the root itself folds the same way.
        let h = fold(
            SymOp::Abs,
            &Form::poly(root.clone()),
            &params((1.0, 2.0)),
            budget(),
        )
        .unwrap();
        assert_eq!(h.num, root);
    }

    #[test]
    fn a_straddling_or_unknown_sign_never_folds() {
        let (root, sq) = square_of_linear();
        let a = Form::poly(sq);
        // r in [-1, 1]: a + 2r straddles zero.
        assert!(fold(SymOp::Sqrt, &a, &params((-1.0, 1.0)), budget()).is_none());
        // An exact zero endpoint is not strictly signed either.
        assert!(fold(SymOp::Sqrt, &a, &params((-0.25, 5.0)), budget()).is_none());
        // No bracket for `r` at all: not enclosable.
        let mut only_a = IndetMap::default();
        only_a.insert(1u128, (0.5, 0.5));
        assert!(fold(SymOp::Abs, &Form::poly(root), &only_a, budget()).is_none());
        // An empty parameter table folds nothing.
        assert!(fold(SymOp::Sqrt, &a, &IndetMap::default(), budget()).is_none());
    }

    #[test]
    fn the_rational_enclosure_is_outward() {
        let c = Rat::new(1, 3, 0).unwrap();
        let e = rat_enclosure(&c);
        assert!(e.lo() < 1.0 / 3.0 && e.hi() > 1.0 / 3.0);
        assert!(rat_enclosure(&Rat::new(1, 1, 2000).unwrap()).is_poison());
    }
}
