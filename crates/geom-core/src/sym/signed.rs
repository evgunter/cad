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
//! outward-rounded [`RingInterval`](crate::ring_interval::RingInterval). No type is punned, no feature is
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
//! Per node, in the EARLY memo only (`super::early_form`) — never in
//! the plain form — so a plain theorem is never re-labelled as gated,
//! and the fold reaches atoms nested inside other atoms' arguments,
//! which is where the arc family's `sqrt` of a perfect square sits
//! (`‖q − c‖ = r` has `(a + 2r)²` under its root on the plate).

use core::f64::consts::PI;

use super::form::{Form, Mono, Poly, exp_of};
use super::rational::Rat;
use super::{AtomInfo, INDET_PI, IndetMap, Session, SymBudget, SymOp, manifest};
use crate::ring_interval::RingInterval;

/// The most terms a candidate root may grow to before `poly_sqrt` gives
/// up: a real residual's root is a handful of terms, and the bound keeps
/// a non-square polynomial from being chased term by term.
const ROOT_TERMS: usize = 64;

/// The exponent of `id` in `m` (zero where absent).
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
    let mut ids: Vec<u128> = p.monos().flat_map(|m| m.iter().map(|(i, _)| *i)).collect();
    ids.sort_unstable();
    ids.dedup();
    ids
}

/// The leading term of `p` under the graded-lex order over `ids`.
fn lead<'a>(p: &'a Poly, ids: &[u128]) -> Option<(&'a Mono, Rat)> {
    p.terms()
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
    let cap = x.terms().len().min(ROOT_TERMS);
    while !rem.is_zero() {
        if root.terms().len() >= cap {
            return None;
        }
        let (tm, tc) = lead(&rem, &ids)?;
        let nm = mono_div(tm, &r0m)?;
        let nc = tc.mul(&twice.recip()?)?;
        // rem -= 2·root·t + t²
        let t = Poly::term(nm.clone(), nc.clone());
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
    for (m, c) in x.terms() {
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
    p.terms()
        .iter()
        .min_by(|(a, _), (b, _)| cmp_mono(a, b, ids))
        .map(|(m, c)| (m, c.clone()))
}

/// The monomial `m^e` as a polynomial with coefficient one.
fn mono_poly(m: &Mono, e: u32) -> Poly {
    Poly::term(m.iter().map(|&(i, k)| (i, k * e)).collect(), Rat::one())
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
    for (m, c) in p.terms() {
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
        p.monos().all(|m| {
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


// ------------------------------------------------------------------
// SYM-10 measurement plants (Phase 1): the decision read over a DEEP
// enclosure, and the certified order read at min/max.
// ------------------------------------------------------------------

/// How many atom levels the deep enclosure descends before declining.
const ENCLOSE_DEPTH: usize = 8;

fn ring_sqrt(x: RingInterval) -> RingInterval {
    if x.is_poison() || x.hi() < 0.0 {
        return RingInterval::poison();
    }
    let lo = if x.lo() <= 0.0 { 0.0 } else { x.lo().sqrt().next_down() };
    let hi = x.hi().sqrt().next_up();
    RingInterval::from_bounds(lo, hi)
}

fn ring_abs(x: RingInterval) -> RingInterval {
    if x.is_poison() {
        return x;
    }
    if x.lo() >= 0.0 {
        x
    } else if x.hi() <= 0.0 {
        -x
    } else {
        RingInterval::from_bounds(0.0, x.hi().max(-x.lo()))
    }
}

fn ring_min(a: RingInterval, b: RingInterval) -> RingInterval {
    if a.is_poison() || b.is_poison() {
        return RingInterval::poison();
    }
    RingInterval::from_bounds(a.lo().min(b.lo()), a.hi().min(b.hi()))
}

fn ring_max(a: RingInterval, b: RingInterval) -> RingInterval {
    if a.is_poison() || b.is_poison() {
        return RingInterval::poison();
    }
    RingInterval::from_bounds(a.lo().max(b.lo()), a.hi().max(b.hi()))
}

/// The enclosure of one indeterminate: a parameter's bracket, π, or a
/// `sqrt`/`abs`/`min`/`max` atom over arguments this function can
/// enclose in turn. `None` for anything else.
fn enclose_indet(
    id: u128,
    params: &IndetMap<(f64, f64)>,
    atoms: &IndetMap<AtomInfo>,
    depth: usize,
) -> Option<RingInterval> {
    if id == INDET_PI {
        return Some(RingInterval::from_bounds(PI.next_down(), PI.next_up()));
    }
    if let Some(&(lo, hi)) = params.get(&id) {
        return Some(RingInterval::from_bounds(lo, hi));
    }
    if depth >= ENCLOSE_DEPTH {
        return None;
    }
    let atom = atoms.get(&id)?;
    let arg = |k: usize| -> Option<RingInterval> {
        enclose_form_deep(atom.args[k].as_deref()?, params, atoms, depth + 1)
    };
    let out = match atom.op {
        SymOp::Sqrt => ring_sqrt(arg(0)?),
        SymOp::Abs => ring_abs(arg(0)?),
        SymOp::Min => ring_min(arg(0)?, arg(1)?),
        SymOp::Max => ring_max(arg(0)?, arg(1)?),
        _ => return None,
    };
    (!out.is_poison()).then_some(out)
}

fn enclose_deep(
    p: &Poly,
    params: &IndetMap<(f64, f64)>,
    atoms: &IndetMap<AtomInfo>,
    depth: usize,
) -> Option<RingInterval> {
    let mut acc = RingInterval::zero();
    for (m, c) in p.terms() {
        let mut term = rat_enclosure(c);
        for &(id, e) in m {
            let x = enclose_indet(id, params, atoms, depth)?;
            term = term * x.powi(i32::try_from(e).ok()?);
        }
        acc = acc + term;
    }
    (!acc.is_poison()).then_some(acc)
}

fn enclose_form_deep(
    f: &Form,
    params: &IndetMap<(f64, f64)>,
    atoms: &IndetMap<AtomInfo>,
    depth: usize,
) -> Option<RingInterval> {
    if f.poisoned {
        return None;
    }
    let n = enclose_deep(&f.num, params, atoms, depth)?;
    let d = enclose_deep(&f.den, params, atoms, depth)?;
    let q = n / d;
    (!q.is_poison()).then_some(q)
}

/// The sign class of an enclosure: `Some(true)` for `> 0`, `Some(false)`
/// for `< 0`, `None` where it touches or straddles zero.
fn strict_sign(r: RingInterval) -> Option<bool> {
    if r.is_poison() {
        None
    } else if r.lo() > 0.0 {
        Some(true)
    } else if r.hi() < 0.0 {
        Some(false)
    } else {
        None
    }
}

/// `p` with every manifestly POSITIVE indeterminate of its content
/// divided out — a factor that is `> 0` wherever it has a value does
/// not move the sign of the product, or its zero set.
fn strip_positive_content(p: &Poly, sess: &Session) -> Poly {
    let Some(first) = p.monos().next() else {
        return p.clone();
    };
    let mut g: Mono = first.clone();
    for m in p.monos() {
        g.retain(|&(id, _)| m.iter().any(|&(j, _)| j == id));
        for (id, e) in &mut g {
            *e = (*e).min(exp_of(m, *id));
        }
        if g.is_empty() {
            return p.clone();
        }
    }
    g.retain(|&(id, _)| manifest::positive_indet_pub(id, sess));
    if g.is_empty() {
        return p.clone();
    }
    let mut terms: Vec<(Mono, Rat)> = Vec::with_capacity(p.terms().len());
    for (m, c) in p.terms() {
        let mut rest = Mono::with_capacity(m.len());
        for &(id, e) in m {
            let d = exp_of(&g, id);
            if e > d {
                rest.push((id, e - d));
            }
        }
        terms.push((rest, c.clone()));
    }
    terms.sort_by(|(a, _), (b, _)| a.cmp(b));
    Poly::from_sorted_terms(terms).unwrap_or_else(|| p.clone())
}

/// The certified sign of a polynomial over the box, after stripping:
/// `Some(Some(true))` strictly positive, `Some(Some(false))` strictly
/// negative, `Some(None)` where the enclosure's upper end is `≤ 0` or
/// lower end `≥ 0` is reported through `weak`; `None` not enclosable.
fn signed_poly(p: &Poly, sess: &Session) -> Option<(RingInterval, bool)> {
    let stripped = strip_positive_content(p, sess);
    let r = enclose_deep(&stripped, &sess.params, &sess.atoms, 0)?;
    Some((r, stripped.terms().len() != p.terms().len() || stripped != *p))
}

/// **The decision read** for `Select(d, when_le, when_gt)`: `Some(true)`
/// where `d ≤ 0` is CERTIFIED at every point of the box, `Some(false)`
/// where `d > 0` is, `None` otherwise (straddling, poisoned, not
/// enclosable). Manifestly positive factors are stripped from both
/// halves before the enclosure: the sign of `P/Q` with `Q > 0` is the
/// sign of `P`, and so is its zero set.
pub(super) fn decision(d: &Form, sess: &Session) -> Option<bool> {
    if d.poisoned || sess.params.is_empty() {
        return None;
    }
    // The denominator: manifestly positive, or certified one-signed.
    let den_positive = if manifest::positive(&Form::poly(d.den.clone()), sess) {
        true
    } else {
        let (r, _) = signed_poly(&d.den, sess)?;
        strict_sign(r)?
    };
    let (n, _) = signed_poly(&d.num, sess)?;
    if n.is_poison() {
        return None;
    }
    // d ≤ 0 everywhere: num ≤ 0 with den > 0, or num ≥ 0 with den < 0.
    if (n.hi() <= 0.0 && den_positive) || (n.lo() >= 0.0 && !den_positive) {
        return Some(true);
    }
    // d > 0 everywhere: num > 0 with den > 0, or num < 0 with den < 0.
    if (n.lo() > 0.0 && den_positive) || (n.hi() < 0.0 && !den_positive) {
        return Some(false);
    }
    None
}

/// **The certified order read** at `min`/`max`: the arm the comparison
/// `a ≤ b` selects wherever that comparison is certified over the box.
pub(super) fn order(op: SymOp, a: &Form, b: &Form, sess: &Session, budget: SymBudget) -> Option<Form> {
    let diff = a.add(&b.neg()?, budget)?;
    if diff.poisoned {
        return None;
    }
    let a_le_b = decision(&diff, sess)?;
    let mut out = match (op, a_le_b) {
        (SymOp::Max, true) | (SymOp::Min, false) => b.clone(),
        (SymOp::Max, false) | (SymOp::Min, true) => a.clone(),
        _ => return None,
    };
    out.gated = true;
    Some(out)
}

// ------------------------------------------------------------------
// SYM-10 measurement plant `q` (Phase 1): the CANONICAL square root —
// the fourth piece the render names.
// ------------------------------------------------------------------

use std::sync::Arc;

/// `p = c · p'` with `c > 0` the rational content and `p'` the
/// primitive integer polynomial (its coefficients share no factor),
/// the sign left on `p'`.
fn content_split(p: &Poly) -> Option<(Rat, Poly)> {
    let mut c: Option<Rat> = None;
    for (_, k) in p.terms() {
        c = Some(match c {
            None => k.abs(),
            Some(g) => g.content_gcd(k)?,
        });
    }
    let c = c?;
    let prim = p.scaled(&c.recip()?)?;
    Some((c, prim))
}

fn mint(sess: &mut Session, op: SymOp, arg: Form, early: bool) -> Form {
    let id = super::indet_atom(op.tag(), 0, &[arg.digest()]);
    super::mint_atom(sess, id, early, || AtomInfo {
        op,
        payload: 0,
        args: [Some(Arc::new(arg)), None, None],
    });
    Form::poly(Poly::indet(id))
}

/// `sqrt(p)` in canonical form: `s · sqrt(f) · sqrt(p')` for
/// `p = s²·f·p'` (content split, square part out exactly, the rest a
/// constant atom); `sqrt(R²) = |R|` read by rule F (manifest), rule C
/// (certified, gated) or the `abs` atom.
fn sqrt_poly_canon(p: &Poly, sess: &mut Session, early: bool) -> Option<Form> {
    let budget = sess.budget;
    if p.is_zero() {
        return Some(Form::zero());
    }
    if let Some(c) = p.as_constant() {
        if c.is_negative() {
            return None;
        }
        if let Some(r) = c.sqrt_exact() {
            return Some(Form::poly(Poly::constant(r)));
        }
        let (s, f) = c.split_square()?;
        let k = mint(sess, SymOp::Sqrt, Form::poly(Poly::constant(f)), early);
        return k.mul(&Form::poly(Poly::constant(s)), budget);
    }
    let (c, prim) = content_split(p)?;
    let (s, f) = c.split_square()?;
    let base = if let Some(r) = poly_sqrt(&prim, budget) {
        let rf = Form::poly(r);
        if let Some(m) = manifest::fold_abs(&rf, sess) {
            m
        } else if let Some(g) = fold(SymOp::Abs, &rf, &sess.params, budget) {
            g
        } else {
            mint(sess, SymOp::Abs, rf, early)
        }
    } else {
        mint(sess, SymOp::Sqrt, Form::poly(prim), early)
    };
    let mut out = base.mul(&Form::poly(Poly::constant(s)), budget)?;
    if f != Rat::one() {
        let k = mint(sess, SymOp::Sqrt, Form::poly(Poly::constant(f)), early);
        out = out.mul(&k, budget)?;
    }
    Some(out)
}

/// Whether `sqrt(prim(d))` is already an atom of the session — then
/// `d ≥ 0` wherever the document has a value at all.
fn primitive_sqrt_atom_exists(d: &Poly, sess: &Session) -> bool {
    content_split(d).is_some_and(|(_, prim)| {
        let id = super::indet_atom(SymOp::Sqrt.tag(), 0, &[Form::poly(prim).digest()]);
        sess.atoms.contains_key(&id)
    })
}

/// **Plant `q`**: `sqrt(N/D) → sqrt(N)/sqrt(D)` wherever `D ≥ 0` is
/// known — a positive constant, manifestly non-negative, the argument
/// of an existing `sqrt` atom, or (a READ, gated) certified positive
/// over the box — each half in canonical form.
pub(super) fn sqrt_canon(a: &Form, sess: &mut Session, early: bool) -> Option<Form> {
    if a.poisoned || a.is_zero() {
        return None;
    }
    let budget = sess.budget;
    let manifest_den = a.den.as_constant().is_some_and(|c| !c.is_negative())
        || manifest::nonneg(&Form::poly(a.den.clone()), sess)
        || primitive_sqrt_atom_exists(&a.den, sess);
    let read_den = !manifest_den
        && enclose_deep(&a.den, &sess.params, &sess.atoms, 0).is_some_and(|r| r.lo() > 0.0);
    if !(manifest_den || read_den) {
        return None;
    }
    let n = sqrt_poly_canon(&a.num, sess, early)?;
    let d = sqrt_poly_canon(&a.den, sess, early)?;
    let mut out = n.mul(&d.recip()?, budget)?;
    out.gated |= a.gated || read_den;
    Some(out)
}

/// `sqrt(c · R²) → s · sqrt(f) · |R|` ONLY where the primitive part
/// of `p` is a perfect square (nothing else is re-keyed); `|R|` read
/// by rule F, rule C (gated) or the `abs` atom. `None` otherwise.
fn sqrt_square_content(p: &Poly, sess: &mut Session, early: bool) -> Option<Form> {
    let budget = sess.budget;
    if p.is_zero() || p.as_constant().is_some() {
        return None;
    }
    let (c, prim) = content_split(p)?;
    let r = poly_sqrt(&prim, budget)?;
    let (s, f) = c.split_square()?;
    let rf = Form::poly(r);
    let base = if let Some(m) = manifest::fold_abs(&rf, sess) {
        m
    } else if let Some(g) = fold(SymOp::Abs, &rf, &sess.params, budget) {
        g
    } else {
        mint(sess, SymOp::Abs, rf, early)
    };
    let mut out = base.mul(&Form::poly(Poly::constant(s)), budget)?;
    if f != Rat::one() {
        let k = mint(sess, SymOp::Sqrt, Form::poly(Poly::constant(f)), early);
        out = out.mul(&k, budget)?;
    }
    Some(out)
}

/// `sqrt(λ)` for a positive rational: exact, or `s · sqrt(f)` with a
/// constant atom.
fn sqrt_const(c: &Rat, sess: &mut Session, early: bool) -> Option<Form> {
    if let Some(r) = c.sqrt_exact() {
        return Some(Form::poly(Poly::constant(r)));
    }
    let (s, f) = c.split_square()?;
    let k = mint(sess, SymOp::Sqrt, Form::poly(Poly::constant(f)), early);
    k.mul(&Form::poly(Poly::constant(s)), sess.budget)
}

/// **Plant `r` — the NARROW canonical root**: `sqrt(N/D) →
/// sqrt(N) / (sqrt(λ) · S)` where `S = sqrt(A)` is an atom the session
/// already holds and `D = λ·A` for a positive rational `λ` (the atom's
/// existence is what says `A ≥ 0`, and `D ≠ 0` as a denominator); and
/// `sqrt(c·R²) → s·sqrt(f)·|R|`. No existing atom is re-keyed: the
/// only atoms this mints are constant `sqrt(f)` atoms and `sqrt(N)`
/// over the numerator alone.
pub(super) fn sqrt_narrow(a: &Form, sess: &mut Session, early: bool) -> Option<Form> {
    if a.poisoned || a.is_zero() {
        return None;
    }
    let budget = sess.budget;
    if let Some(c) = a.den.as_constant() {
        if c.is_negative() || c.is_zero() {
            return None;
        }
        let n = sqrt_square_content(&a.num, sess, early)?;
        let d = sqrt_const(&c, sess, early)?;
        return n.mul(&d.recip()?, budget);
    }
    // An existing sqrt atom whose argument is a constant multiple of D.
    let found = sess.atoms.iter().find_map(|(id, at)| {
        if at.op != SymOp::Sqrt {
            return None;
        }
        let arg = at.args[0].as_deref()?;
        if !arg.den.as_constant().is_some_and(|k| k == Rat::one()) {
            return None;
        }
        let lam = super::quotient::constant_ratio(&a.den, &arg.num)?;
        (!lam.is_negative() && !lam.is_zero()).then_some((*id, lam))
    });
    let (sid, lam) = found?;
    let n = if let Some(f) = sqrt_square_content(&a.num, sess, early) {
        f
    } else if let Some(c) = a.num.as_constant() {
        if c.is_negative() {
            return None;
        }
        sqrt_const(&c, sess, early)?
    } else {
        mint(sess, SymOp::Sqrt, Form::poly(a.num.clone()), early)
    };
    let root_lam = sqrt_const(&lam, sess, early)?;
    let d = root_lam.mul(&Form::poly(Poly::indet(sid)), budget)?;
    let mut out = n.mul(&d.recip()?, budget)?;
    out.gated |= a.gated;
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
