//! **The value reads, and the one door the value comes through.**
//! Rule C — the clause-3 fold behind [`SymRules::signed_root`]:
//! `sqrt(X) → R` where `X = R²` as forms and `R` has a CERTIFIED sign
//! over the leaf's box, and `abs(R) → ±R` likewise. Beside it, behind
//! its own dial ([`SymRules::decision_read`]), the DECISION READ: the
//! arm a `Select` takes where its decision's sign is certified over
//! the box, and the arm `min`/`max` takes, which is the same read
//! (`max(A, B)` IS `select(B − A, A, B)`). Rule G's side condition
//! asks for a third (`enclose_poly`), under rule C's dial. Three
//! reads; ONE enclosure, and this module is the whole of how a value
//! is read.
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

use super::form::{Form, Mono, Poly, leading, mono_div, trailing};
use super::rational::Rat;
use super::{AtomInfo, INDET_PI, IndetMap, Session, SymBudget, SymOp, manifest, quotient};
use crate::ring_interval::RingInterval;

/// The most terms a candidate root may grow to before `poly_sqrt` gives
/// up: a real residual's root is a handful of terms, and the bound keeps
/// a non-square polynomial from being chased term by term.
const ROOT_TERMS: usize = 64;

/// Every indeterminate id of `p`, sorted.
fn ids_of(p: &Poly) -> Vec<u128> {
    let mut ids: Vec<u128> = p.monos().flat_map(|m| m.iter().map(|(i, _)| *i)).collect();
    ids.sort_unstable();
    ids.dedup();
    ids
}

/// The monomial whose square is `m`, if every exponent is even.
fn mono_sqrt(m: &Mono) -> Option<Mono> {
    m.iter()
        .map(|&(i, e)| (e % 2 == 0).then_some((i, e / 2)))
        .collect()
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
    let (lm, lc) = leading(x)?;
    let r0m = mono_sqrt(lm)?;
    let r0c = lc.sqrt_exact()?;
    // The trailing term of a square is the square of the root's
    // trailing term: an odd exponent or a non-square coefficient there
    // settles it without building anything.
    let (tm, tc) = trailing(x)?;
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
        let (tm, tc) = leading(&rem)?;
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

/// The enclosure of `p` over the parameter brackets alone — no atom
/// entered, which is rule C's own reach ([`fold`] declines an argument
/// carrying anything else). ONE walker with the others: the depth cap
/// is what distinguishes the two reaches, so a shallow read is the
/// deep one asked at its floor.
fn enclose(p: &Poly, params: &IndetMap<(f64, f64)>) -> Option<RingInterval> {
    enclose_deep(p, params, &IndetMap::default(), ENCLOSE_DEPTH)
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
// The DECISION READ (DECIDE-3): rule C's certified read, extended from
// `sqrt`/`abs` to the decision door and to `min`/`max`.
// ------------------------------------------------------------------

/// π to the ring's own rounding — the one spelling, read by every
/// enclosure this module builds.
fn pi_bracket() -> RingInterval {
    RingInterval::from_bounds(PI.next_down(), PI.next_up())
}

/// How many atom levels the deep enclosure descends before it declines.
/// A frame's conditioning floor nests a `min` over a `max` over an
/// `abs` over a `sqrt` over the normal's own root — four — and the
/// candidate norms of a tilted frame add two more; past that the cost
/// of one node's read is the cost of a sub-tree, and declining is the
/// conservative direction.
const ENCLOSE_DEPTH: usize = 8;

fn ring_sqrt(x: RingInterval) -> RingInterval {
    if x.is_poison() || x.hi() < 0.0 {
        return RingInterval::poison();
    }
    let lo = if x.lo() <= 0.0 {
        0.0
    } else {
        x.lo().sqrt().next_down()
    };
    RingInterval::from_bounds(lo, x.hi().sqrt().next_up())
}

fn ring_abs(x: RingInterval) -> RingInterval {
    if x.is_poison() || x.lo() >= 0.0 {
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

/// The enclosure of one indeterminate: a parameter's bracket, `π`, or
/// a `sqrt`/`abs`/`min`/`max` ATOM over arguments this function can
/// enclose in turn. `None` for anything else — an opaque real, a
/// frozen node, a `select` or a trig atom — because an indeterminate
/// with no bracket has no enclosure, and a guess would be a value the
/// tier is not entitled to.
fn enclose_indet(
    id: u128,
    params: &IndetMap<(f64, f64)>,
    atoms: &IndetMap<AtomInfo>,
    depth: usize,
) -> Option<RingInterval> {
    if id == INDET_PI {
        return Some(pi_bracket());
    }
    if let Some(&(lo, hi)) = params.get(&id) {
        return Some(RingInterval::from_bounds(lo, hi));
    }
    if depth >= ENCLOSE_DEPTH {
        #[cfg(feature = "sym-profile-testing")]
        super::profile::read_note(|| "depth exhausted".into());
        return None;
    }
    let Some(atom) = atoms.get(&id) else {
        #[cfg(feature = "sym-profile-testing")]
        super::profile::read_note(|| format!("unbracketed@{depth} opaque"));
        return None;
    };
    #[cfg(feature = "sym-profile-testing")]
    super::profile::read_entered(depth + 1);
    let arg = |k: usize| {
        let Some(f) = atom.args[k].as_deref() else {
            #[cfg(feature = "sym-profile-testing")]
            super::profile::read_note(|| format!("unbracketed@{depth} {:?} arity", atom.op));
            return None;
        };
        enclose_form_deep(f, params, atoms, depth + 1)
    };
    let out = match atom.op {
        SymOp::Sqrt => ring_sqrt(arg(0)?),
        SymOp::Abs => ring_abs(arg(0)?),
        SymOp::Min => ring_min(arg(0)?, arg(1)?),
        SymOp::Max => ring_max(arg(0)?, arg(1)?),
        _ => {
            #[cfg(feature = "sym-profile-testing")]
            super::profile::read_note(|| format!("unbracketed@{depth} {:?}", atom.op));
            return None;
        }
    };
    #[cfg(feature = "sym-profile-testing")]
    if out.is_poison() {
        super::profile::read_note(|| format!("poison@{depth}"));
    }
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
            let Ok(e) = i32::try_from(e) else {
                #[cfg(feature = "sym-profile-testing")]
                super::profile::read_note(|| format!("exponent@{depth}"));
                return None;
            };
            term = term * x.powi(e);
        }
        acc = acc + term;
    }
    #[cfg(feature = "sym-profile-testing")]
    if acc.is_poison() {
        super::profile::read_note(|| format!("poison@{depth}"));
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
        #[cfg(feature = "sym-profile-testing")]
        super::profile::read_note(|| format!("poisoned argument@{depth}"));
        return None;
    }
    let q =
        enclose_deep(&f.num, params, atoms, depth)? / enclose_deep(&f.den, params, atoms, depth)?;
    #[cfg(feature = "sym-profile-testing")]
    if q.is_poison() {
        super::profile::read_note(|| format!("poison@{depth}"));
    }
    (!q.is_poison()).then_some(q)
}

/// The deep enclosure of one polynomial over the session's brackets —
/// the door rule G's side-condition source 4 reads (`super::root`).
pub(super) fn enclose_poly(p: &Poly, sess: &Session) -> Option<RingInterval> {
    if sess.params.is_empty() {
        return None;
    }
    enclose_deep(p, &sess.params, &sess.atoms, 0)
}

/// `p` with every manifestly POSITIVE indeterminate of its content
/// divided out. A factor that is `> 0` wherever it has a value moves
/// neither the sign of the product nor its zero set, and dividing it
/// out is what lets a decision whose halves are dressed in norms be
/// read from the polynomial underneath.
fn strip_positive_content(p: &Poly, sess: &Session) -> Poly {
    // Rule E's own content split, asked rather than re-derived: the
    // monomial every term is divisible by, then the division. What is
    // this rule's own is the middle line — keeping only the factors
    // whose sign the form settles.
    let mut common = quotient::content(p);
    common.retain(|&(id, _)| manifest::indet_positive(id, sess));
    if common.is_empty() {
        return p.clone();
    }
    quotient::divide(p, &common).unwrap_or_else(|| p.clone())
}

/// **The decision read** for `Select(d, when_le, when_gt)`:
/// `Some(true)` where `d ≤ 0` is CERTIFIED at every point of the box,
/// `Some(false)` where `d > 0` is, `None` otherwise (straddling,
/// poisoned, or not enclosable).
///
/// **`d ≤ 0` is read as `d < 0` in practice, and that is a property of
/// the instrument, not of the rule.** The enclosure is outwardly
/// rounded and a rational coefficient is padded to an `f64` bracket
/// ([`rat_enclosure`]), so an exact `d = 0` at the tie comes back as a
/// bracket that touches zero from both sides and the read declines.
/// The door's own comparison is `≤`, and a tie the read declined is
/// answered by the numeric channel exactly as it was before the read
/// existed: a missed discharge, never a wrong arm. Sharpening it would
/// take an exact rational enclosure at the tie, which is a different
/// instrument. Manifestly positive factors are
/// stripped from both halves first: the sign of `P/Q` with `Q > 0` is
/// the sign of `P`, and so is its zero set.
///
/// A fold through this is equal to the atom AT EVERY POINT OF THE BOX
/// and not identically in the parameters, exactly as rule C's is, so
/// the caller marks the form `gated` and the discharge is counted
/// `sign_gated`.
pub(super) fn decision(d: &Form, sess: &Session) -> Option<bool> {
    #[cfg(feature = "sym-profile-testing")]
    let (t0, mark) = (super::profile::clock(), super::profile::read_enclose_mark());
    let out = read_decision(d, sess);
    #[cfg(feature = "sym-profile-testing")]
    if let Some(t0) = t0 {
        let spent = t0.elapsed();
        let class = instrument::classify(d, sess, out.is_some());
        super::profile::read_done(spent, mark, d.digest(), &class);
    }
    out
}

fn read_decision(d: &Form, sess: &Session) -> Option<bool> {
    if d.poisoned {
        return None;
    }
    let enclose = |p: &Poly| {
        #[cfg(feature = "sym-profile-testing")]
        let t0 = super::profile::clock();
        let stripped = strip_positive_content(p, sess);
        #[cfg(feature = "sym-profile-testing")]
        super::profile::read_strip(t0);
        #[cfg(feature = "sym-profile-testing")]
        let t0 = super::profile::clock();
        let e = enclose_deep(&stripped, &sess.params, &sess.atoms, 0);
        #[cfg(feature = "sym-profile-testing")]
        super::profile::read_enclose(t0);
        e
    };
    let den = enclose(&d.den)?;
    let den_positive = if manifest::positive(&Form::poly(d.den.clone()), sess) || den.lo() > 0.0 {
        true
    } else if den.hi() < 0.0 {
        false
    } else {
        return None;
    };
    let num = enclose(&d.num)?;
    if (num.hi() <= 0.0 && den_positive) || (num.lo() >= 0.0 && !den_positive) {
        return Some(true);
    }
    if (num.lo() > 0.0 && den_positive) || (num.hi() < 0.0 && !den_positive) {
        return Some(false);
    }
    None
}

/// **The certified ORDER read** at `min`/`max` — `max(A, B)` IS
/// `select(B − A, A, B)`, so the arm is the same read: the one the
/// comparison `A ≤ B` picks wherever that comparison is certified over
/// the box. `None` where it is not.
pub(super) fn order(
    op: SymOp,
    a: &Form,
    b: &Form,
    sess: &Session,
    budget: SymBudget,
) -> Option<Form> {
    #[cfg(feature = "sym-profile-testing")]
    let t0 = super::profile::clock();
    let diff = b.neg().and_then(|nb| a.add(&nb, budget));
    #[cfg(feature = "sym-profile-testing")]
    super::profile::read_order(t0, diff.as_ref().is_none_or(|d| d.poisoned));
    let diff = diff?;
    if diff.poisoned {
        return None;
    }
    #[cfg(feature = "sym-profile-testing")]
    super::profile::read_in_order(true);
    let le = decision(&diff, sess);
    #[cfg(feature = "sym-profile-testing")]
    super::profile::read_in_order(false);
    let mut out = match (op, le?) {
        (SymOp::Max, true) | (SymOp::Min, false) => b.clone(),
        (SymOp::Max, false) | (SymOp::Min, true) => a.clone(),
        _ => return None,
    };
    out.gated = true;
    Some(out)
}

/// The read's INSTRUMENT (`sym-profile-testing` only): why one call of
/// [`decision`] declined, whether an id-walk over its two halves could
/// have said so before enclosing, and how deep its enclosure reaches.
/// It re-encloses what the read enclosed, reads the refusal the
/// enclosure noted at its own arms (`profile::read_note`), and decides
/// nothing.
#[cfg(feature = "sym-profile-testing")]
mod instrument {
    use super::super::profile::{ReadClass, read_classifying, read_noted};
    use super::super::{Session, manifest};
    use super::{Form, Poly, RingInterval, enclose_deep, strip_positive_content};

    /// One half re-enclosed: the enclosure, the refusal it noted, and
    /// the deepest atom level it entered.
    fn half(p: &Poly, sess: &Session) -> (Option<RingInterval>, Option<String>, usize) {
        read_classifying(true);
        let e = enclose_deep(p, &sess.params, &sess.atoms, 0);
        let (note, deepest) = read_noted();
        read_classifying(false);
        (e, note, deepest)
    }

    /// Whether a refusal is one an id-walk sees without an interval.
    fn structural(note: Option<&String>) -> bool {
        note.is_some_and(|c| {
            c.starts_with("unbracketed") || c.starts_with("depth") || c.starts_with("poisoned")
        })
    }

    pub(in super::super) fn classify(d: &Form, sess: &Session, settled: bool) -> ReadClass {
        if d.poisoned {
            return ReadClass {
                cause: Some("poisoned form".into()),
                prepass: false,
                depth: None,
            };
        }
        let (de, dn, dd) = half(&strip_positive_content(&d.den, sess), sess);
        let (ne, nn, nd) = half(&strip_positive_content(&d.num, sess), sess);
        let prepass = structural(dn.as_ref()) || structural(nn.as_ref());
        let depth = (de.is_some() && ne.is_some()).then_some(dd.max(nd));
        if settled {
            return ReadClass {
                cause: None,
                prepass,
                depth,
            };
        }
        // The read's own order: the denominator's half first, and its
        // sign before the numerator is looked at.
        let unnoted = || "unnoted".to_owned();
        let cause = match de {
            None => dn.unwrap_or_else(unnoted),
            Some(dv) => {
                let signed = manifest::positive(&Form::poly(d.den.clone()), sess)
                    || dv.lo() > 0.0
                    || dv.hi() < 0.0;
                if !signed {
                    "straddle (den)".into()
                } else if ne.is_none() {
                    nn.unwrap_or_else(unnoted)
                } else {
                    "straddle (num)".into()
                }
            }
        };
        ReadClass {
            cause: Some(cause),
            prepass,
            depth,
        }
    }

    /// **Every refusal of the enclosure is noted, and named as the
    /// enclosure made it** — at the shapes a decision form carries: an
    /// opaque id at the top, a root chain either side of the cap over a
    /// bracketed or an opaque leaf, `abs`, both arguments of
    /// `min`/`max`, a missing argument, an unbracketed id in an
    /// argument's DENOMINATOR, a poisoned argument, a root of a
    /// negative bracket, and an atom no enclosure reaches. A refusal
    /// arm without its note reds here as `None`.
    #[cfg(test)]
    #[allow(clippy::unwrap_used)]
    mod tests {
        use std::sync::Arc;

        use super::super::super::profile::{read_classifying, read_noted};
        use super::super::super::{AtomInfo, IndetMap, SymOp};
        use super::super::{ENCLOSE_DEPTH, Form, Poly, enclose_deep};

        const PARAM: u128 = 1;
        const NEG: u128 = 2;
        const OPAQUE: u128 = 999;
        const TOP: u128 = 7;

        fn f(id: u128) -> Option<Arc<Form>> {
            Some(Arc::new(Form::poly(Poly::indet(id))))
        }

        /// `k` nested `sqrt` atoms over `leaf`, ids `100 ..`; answers
        /// the outermost.
        fn chain(atoms: &mut IndetMap<AtomInfo>, k: usize, leaf: u128) -> u128 {
            let mut inner = leaf;
            for i in 0..k {
                let id = 100 + i as u128;
                atoms.insert(
                    id,
                    AtomInfo {
                        op: SymOp::Sqrt,
                        payload: 0,
                        args: [f(inner), None, None],
                    },
                );
                inner = id;
            }
            inner
        }

        fn noted(
            p: &Poly,
            params: &IndetMap<(f64, f64)>,
            atoms: &IndetMap<AtomInfo>,
        ) -> (bool, Option<String>, usize) {
            read_classifying(true);
            let e = enclose_deep(p, params, atoms, 0);
            let (note, deepest) = read_noted();
            read_classifying(false);
            (e.is_some(), note, deepest)
        }

        #[test]
        fn every_refusal_of_the_enclosure_is_noted_where_it_is_made() {
            let mut params = IndetMap::default();
            params.insert(PARAM, (1.0, 2.0));
            params.insert(NEG, (-2.0, -1.0));
            // Root chains, behind a bracketed term the enclosure meets
            // first: `(depth entered or the note)`.
            let chains: [(usize, u128, Result<usize, &str>); 6] = [
                (0, PARAM, Ok(0)),
                (0, OPAQUE, Err("unbracketed@0 opaque")),
                (2, PARAM, Ok(2)),
                (2, OPAQUE, Err("unbracketed@2 opaque")),
                (ENCLOSE_DEPTH, PARAM, Ok(ENCLOSE_DEPTH)),
                (ENCLOSE_DEPTH, OPAQUE, Err("depth exhausted")),
            ];
            for (k, leaf, want) in chains {
                let mut atoms = IndetMap::default();
                let top = chain(&mut atoms, k, leaf);
                let p = Poly::indet(PARAM).add(&Poly::indet(top)).unwrap();
                let (encloses, note, deepest) = noted(&p, &params, &atoms);
                match want {
                    Ok(d) => {
                        assert!(encloses && note.is_none(), "{k} over {leaf}: {note:?}");
                        assert_eq!(deepest, d, "{k} over {leaf}: the depth entered");
                    }
                    Err(c) => {
                        assert!(!encloses, "{k} over {leaf}");
                        assert_eq!(note.as_deref(), Some(c), "{k} over {leaf}");
                    }
                }
            }
            // One atom at the top, by op and arguments.
            let x = f(PARAM);
            let o = f(OPAQUE);
            let over = Some(Arc::new(Form::quotient(
                Poly::indet(PARAM),
                Poly::indet(OPAQUE),
            )));
            let atoms_at: [(&str, SymOp, [Option<Arc<Form>>; 3], Option<&str>); 9] = [
                (
                    "abs of the parameter",
                    SymOp::Abs,
                    [x.clone(), None, None],
                    None,
                ),
                ("max of two", SymOp::Max, [x.clone(), x.clone(), None], None),
                (
                    "max, opaque second",
                    SymOp::Max,
                    [x.clone(), o.clone(), None],
                    Some("unbracketed@1 opaque"),
                ),
                (
                    "min, opaque second",
                    SymOp::Min,
                    [x.clone(), o, None],
                    Some("unbracketed@1 opaque"),
                ),
                (
                    "min, second missing",
                    SymOp::Min,
                    [x.clone(), None, None],
                    Some("unbracketed@0 Min arity"),
                ),
                (
                    "sqrt over x / opaque",
                    SymOp::Sqrt,
                    [over, None, None],
                    Some("unbracketed@1 opaque"),
                ),
                (
                    "sqrt over poison",
                    SymOp::Sqrt,
                    [Some(Arc::new(Form::poison())), None, None],
                    Some("poisoned argument@1"),
                ),
                (
                    "sqrt of a negative bracket",
                    SymOp::Sqrt,
                    [f(NEG), None, None],
                    Some("poison@0"),
                ),
                (
                    "select over parameters",
                    SymOp::Select,
                    [x.clone(), x.clone(), x],
                    Some("unbracketed@0 Select"),
                ),
            ];
            for (what, op, args, want) in atoms_at {
                let mut atoms = IndetMap::default();
                atoms.insert(
                    TOP,
                    AtomInfo {
                        op,
                        payload: 0,
                        args,
                    },
                );
                let (encloses, note, _) = noted(&Poly::indet(TOP), &params, &atoms);
                assert_eq!(encloses, want.is_none(), "{what}: the enclosure");
                assert_eq!(note.as_deref(), want, "{what}: the note");
            }
        }
    }
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
