//! **Rule F — the manifest sign** behind
//! [`SymRules::manifest_sign`](super::SymRules::manifest_sign): in the
//! early walk a `copysign(Y, X)` node becomes `abs(Y)` and an
//! `abs(X)` node becomes `X` wherever the FORM of `X` already shows
//! `X` positive. Nothing here reads a value: the condition is a fact
//! about the form's syntax, and both rewrites are equalities of reals
//! at every point clause 1 admits, so a zero reached through this rule
//! is a THEOREM and lands in `symbolic_zero`.
//!
//! # What mints the atoms this folds
//!
//! [`Vec3::orthonormal_basis`](crate::Vec3::orthonormal_basis) is the
//! branchless Pixar construction, whose first two lines are
//! `s = 1.copysign(n.z)` and `r = 1/(1 + |n.z|)` — a `copysign` and an
//! `abs` of the SAME quantity, the frame normal's `z`. On a
//! `FaceFrame` over a body extruded from a frame tilted about `u`,
//! that `z` is `1/sqrt(P(t))` for a polynomial `P` in the document's
//! parameter: an `Inv` of a `sqrt` atom, which is positive wherever it
//! has a value at all, and which the value channel's `copysign` and
//! `abs` nevertheless carry as two further opaque indeterminates into
//! every term the certification builds. The tier could not see the
//! number `1` in `copysign(1, 1/sqrt(P))` because a `copysign` is
//! opaque; this rule is what lets it.
//!
//! # The predicate: manifestly POSITIVE
//!
//! `nonneg` is this module's older half — the non-negativity rule D's
//! `atan2(0, N)` fold reads (`trig`) — and `positive` sharpens it.
//! The two share the per-term test and differ in what they do with a
//! zero.
//!
//! A polynomial is **manifestly non-negative** when every term has a
//! non-negative coefficient and a monomial each of whose
//! indeterminates is raised to an EVEN power or is a `Sqrt`/`Abs` atom
//! (to any power) — every such term is a product of non-negative reals
//! wherever it has a value — or when it is a PERFECT SQUARE
//! (`signed::poly_sqrt`, exact arithmetic that reads no value).
//!
//! An indeterminate is **manifestly positive** when it is a `Sqrt` or
//! an `Abs` atom whose ARGUMENT form is manifestly positive:
//! `sqrt(X) > 0` and `|X| = X > 0` for `X > 0`. Nothing else is — not a
//! parameter, not `π`, not an opaque real, not a frozen node, and not a
//! `sqrt`/`abs` atom over an argument that is only non-negative, whose
//! value is zero at every zero of the argument.
//!
//! A polynomial is **manifestly positive** when every term has a
//! non-negative coefficient and a non-negative monomial — the term-wise
//! branch above, and NOT the perfect-square branch — and at least ONE
//! term has a strictly positive coefficient and a monomial whose every
//! indeterminate is manifestly positive. The empty monomial qualifies,
//! which is how a positive constant term carries a sum. Then
//! `p = (a positive term) + (non-negative terms) > 0` wherever `p` has
//! a value.
//!
//! The perfect-square branch is deliberately not carried over: `p = r²`
//! is non-negative but vanishes at every real zero of `r`, and its
//! terms may be negative, so the decomposition the positivity argument
//! rests on is unavailable there. `(k + δ)²` is exactly the shape this
//! would get wrong.
//!
//! A FORM `N / D` is manifestly positive when `N` is a manifestly
//! positive polynomial and `D` is a manifestly non-negative one. `D`
//! needs only non-negativity because a point where `D` vanishes is a
//! point the value channel divided by zero at, and clause 1 — the
//! whole-box certification — has already refused there; that is the
//! same side condition `quotient`'s header argues in full, from the
//! four sources a denominator has. So `D > 0` and `N > 0` wherever the
//! form has a value, hence `N/D > 0` there.
//!
//! **What is never folded**: a sum of squares alone (`x² + y²` is zero
//! at the origin); a bare even power; a form carrying a parameter, an
//! opaque real or a frozen node at an odd power, or a negative
//! coefficient on any term; the zero form; poison.
//!
//! # The two identities, and why each is unconditional
//!
//! For a manifestly positive `X`, at every point of the box:
//!
//! ```text
//! abs(X)         = X,
//! copysign(Y, X) = |Y|   (and = 1 when Y is the constant 1).
//! ```
//!
//! The first is `|x| = x` for `x > 0`; the second is `copysign`'s
//! definition, `|y|` with the sign of the second argument, at a second
//! argument whose sign is `+`.
//!
//! **The signed-zero edge, and why strict positivity closes it.**
//! `copysign` reads a SIGN BIT, not a sign: IEEE gives
//! `copysign(1, +0.0) = +1` and `copysign(1, −0.0) = −1`. So at a real
//! ZERO of `X` the node `copysign(Y, X)` does not denote a function of
//! the real value of `X` at all — which of the two it takes is decided
//! by how the value channel happened to spell that zero, and `−0.0`
//! and `+0.0` are one real. A fold under mere NON-negativity would
//! therefore be a claim about a spelling rather than an identity of
//! reals, and on the `−0.0` branch it is false. Strict positivity
//! excludes the point: a manifestly positive `X` is `> 0` at every
//! point of the box where the form has a value, and clause 1 has
//! already refused every point where it does not, so the sign is `+`
//! there unambiguously. The tier's own premise carries the rest —
//! every node denotes a real-valued function of its indeterminates and
//! every operation's value channel encloses that same real — so a
//! value channel that answered `−0.0` for a form this predicate calls
//! positive would be one that failed to enclose, which is a break of
//! clause 1 and not of this rule.
//!
//! `abs(X) = X` alone would be sound under non-negativity, since
//! `|0| = 0` too. It is held to the same predicate here for a reason
//! that is measured rather than logical:
//! `work/sym/coefficient-ring-width-is-not-monotone-in-reach` records
//! `abs(X) = X` on a syntactically non-negative `X` LOSING ten
//! decisions on R1's boss, where the opened atom's products leave the
//! coefficient ring. The boss's atom is `abs((5/8)·sqrt(L²))` over the
//! chord `L`, whose `sqrt(L²)` is an atom of a bare square — non-
//! negative, not manifestly positive — so the positivity predicate
//! declines exactly the fold that row measured losing, and the
//! measurement in that row's third table is what holds it to that.
//!
//! # Where it runs, and in what order
//!
//! At the NODE, in the early walk only (`super::combine`), so the
//! plain form is untouched and a plain theorem is never re-labelled.
//! The atom is not minted at all rather than folded afterwards, which
//! is what keeps it out of every product above it.
//!
//! Against the other rules: A0's exact constant fold is tried FIRST
//! (a constant argument is folded to its exact rational, which is
//! more than this rule would say), then this one, then rule C
//! (`signed`) — the value-free rule before the one that reads a value,
//! so a discharge that could be a theorem is never counted
//! `sign_gated`. Rules A/B per node and rule E run AFTER `combine`
//! returns, on the form this rule left, and the argument this rule
//! tests is the kid's form as those rules already left it — so against
//! THEM the order is structural and not a choice: an atom this rule
//! keeps from being minted is not one a later rule could have folded.
//! What is a choice is the order against A0 and rule C at this node,
//! and `geom-core`'s `sym_rule_f_rows` pins it: with rule C also on,
//! the same residual still answers `theorem` and not `sign_gated`.

use std::sync::Arc;

use super::form::{Form, Mono, Poly};
use super::{AtomInfo, Session, SymBudget, SymOp, indet_atom, signed};
use super::rational::Rat;

/// How many atom arguments deep `positive` looks before it declines.
/// A `sqrt` of a `sqrt` of a `sqrt` is three; the normalisation chains
/// this rule is for are two. The cap is what keeps the predicate a
/// fixed cost per node rather than a walk of the whole atom tree, and
/// declining past it is the conservative direction.
const ATOM_DEPTH: usize = 8;

/// Every indeterminate of `m` is non-negative wherever it has a value:
/// an EVEN power of anything, or a `Sqrt`/`Abs` atom to any power.
fn nonneg_mono(m: &Mono, sess: &Session) -> bool {
    m.iter().all(|&(id, e)| {
        e % 2 == 0
            || sess
                .atoms
                .get(&id)
                .is_some_and(|a| matches!(a.op, SymOp::Sqrt | SymOp::Abs))
    })
}

/// Every term of `p` is non-negative wherever it has a value.
fn termwise_nonneg(p: &Poly, sess: &Session) -> bool {
    p.terms()
        .iter()
        .all(|(m, c)| !c.is_negative() && nonneg_mono(m, sess))
}

/// **A manifestly non-negative POLYNOMIAL** — the term-wise test, or a
/// perfect square.
fn nonneg_poly(p: &Poly, sess: &Session) -> bool {
    termwise_nonneg(p, sess) || signed::poly_sqrt(p, sess.budget).is_some()
}

/// **A manifestly non-negative FORM**: both halves manifestly
/// non-negative polynomials, and not the zero form or a poisoned one.
///
/// This is the predicate rule D's second fold reads — `atan2(0, N) = 0`
/// for an `N` non-negative by its syntax (`trig`) — and the half
/// `positive` sharpens. The module header carries the argument.
pub(super) fn nonneg(f: &Form, sess: &Session) -> bool {
    if f.poisoned || f.num.is_zero() {
        return false;
    }
    nonneg_poly(&f.num, sess) && nonneg_poly(&f.den, sess)
}

/// An indeterminate that is POSITIVE wherever it has a value: a `Sqrt`
/// or an `Abs` atom over a manifestly positive argument.
fn positive_indet(id: u128, sess: &Session, depth: usize) -> bool {
    let Some(atom) = sess.atoms.get(&id) else {
        return false;
    };
    if !matches!(atom.op, SymOp::Sqrt | SymOp::Abs) {
        return false;
    }
    atom.args[0]
        .as_deref()
        .is_some_and(|arg| positive_at(arg, sess, depth + 1))
}

/// Every indeterminate of `m` is positive wherever it has a value —
/// vacuously true of the EMPTY monomial, which is what makes a
/// positive constant term a positive term.
fn positive_mono(m: &Mono, sess: &Session, depth: usize) -> bool {
    m.iter().all(|&(id, _)| positive_indet(id, sess, depth))
}

/// **A manifestly positive POLYNOMIAL**: every term non-negative by
/// the term-wise test, and at least one term strictly positive.
fn positive_poly(p: &Poly, sess: &Session, depth: usize) -> bool {
    termwise_nonneg(p, sess)
        && p.terms()
            .iter()
            .any(|(m, c)| !c.is_negative() && !c.is_zero() && positive_mono(m, sess, depth))
}

fn positive_at(f: &Form, sess: &Session, depth: usize) -> bool {
    if f.poisoned || depth > ATOM_DEPTH {
        return false;
    }
    positive_poly(&f.num, sess, depth) && nonneg_poly(&f.den, sess)
}

/// **A manifestly POSITIVE form**: `> 0` at every point of the box
/// where it has a value. The module header carries the predicate and
/// the argument.
pub(super) fn positive(f: &Form, sess: &Session) -> bool {
    positive_at(f, sess, 0)
}

/// **`abs(X) → X`** for a manifestly positive `X`; `None` otherwise.
pub(super) fn fold_abs(arg: &Form, sess: &Session) -> Option<Form> {
    positive(arg, sess).then(|| arg.clone())
}

/// **`copysign(Y, X) → |Y|`** for a manifestly positive `X`: the
/// magnitude of `Y` as a form.
///
/// A constant `Y` folds to its exact rational magnitude — the case the
/// orthonormal basis mints, `copysign(1, n.z)` — and a `Y` the form
/// already shows non-negative is its own magnitude. Anything else mints
/// the `Abs` ATOM over `Y`, which is the same indeterminate an
/// `abs(Y)` node elsewhere in the DAG mints (same op tag, same zero
/// payload, same argument digest), so the rewrite trades one opaque
/// atom for another the tier may already hold rather than for a new
/// one.
pub(super) fn magnitude(y: &Form, sess: &mut Session) -> Option<Form> {
    if let Some(n) = y.num.as_constant()
        && let Some(d) = y.den.as_constant()
        && let Some(c) = n.mul(&d.recip()?)
    {
        return Some(Form::poly(Poly::constant(c.abs())));
    }
    if nonneg(y, sess) {
        return Some(y.clone());
    }
    let id = indet_atom(SymOp::Abs.tag(), 0, &[y.digest()]);
    sess.atoms.entry(id).or_insert_with(|| AtomInfo {
        op: SymOp::Abs,
        payload: 0,
        args: [Some(Arc::new(y.clone())), None],
    });
    Some(Form::poly(Poly::indet(id)))
}

// ------------------------------------------------------------------
// SYM-10 measurement plants (Phase 1): manifest order and the manifest
// bound.
// ------------------------------------------------------------------

/// A manifestly positive indeterminate, for `signed`'s factor stripping.
pub(super) fn positive_indet_pub(id: u128, sess: &Session) -> bool {
    positive_indet(id, sess, 0)
}

/// **Manifest order**: `max(A, B) → A` and `min(A, B) → B` where the
/// form `A − B` is manifestly non-negative (and the mirror where
/// `B − A` is). Equal forms take either arm.
pub(super) fn fold_order(op: SymOp, a: &Form, b: &Form, sess: &Session, budget: SymBudget) -> Option<Form> {
    let diff = a.add(&b.neg()?, budget)?;
    if diff.poisoned {
        return None;
    }
    let a_ge_b = diff.is_zero() || nonneg(&diff, sess);
    if a_ge_b {
        return Some(if op == SymOp::Max { a.clone() } else { b.clone() });
    }
    if nonneg(&diff.neg()?, sess) {
        return Some(if op == SymOp::Max { b.clone() } else { a.clone() });
    }
    None
}

/// A manifest UPPER bound of `f`, as a rational: a constant; `k·m` for
/// a positive constant `k` and a `min(c, Y)`/`min(Y, c)` atom `m` with
/// `c` a constant (`min(c, Y) ≤ c`); a sum of such terms. Over a
/// positive constant denominator only.
fn upper_bound(f: &Form, sess: &Session) -> Option<Rat> {
    if f.poisoned {
        return None;
    }
    let den = f.den.as_constant()?;
    if den.is_negative() || den.is_zero() {
        return None;
    }
    let mut acc = Rat::zero();
    for (m, c) in f.num.terms() {
        let term_ub = if m.is_empty() {
            c.clone()
        } else if m.len() == 1 && m[0].1 == 1 && !c.is_negative() {
            let atom = sess.atoms.get(&m[0].0)?;
            if atom.op != SymOp::Min {
                return None;
            }
            let cst = |k: usize| -> Option<Rat> {
                let a = atom.args[k].as_deref()?;
                let n = a.num.as_constant()?;
                let d = a.den.as_constant()?;
                n.mul(&d.recip()?)
            };
            let bound = match (cst(0), cst(1)) {
                (Some(x), Some(y)) => if x.add(&y.neg()?)?.is_negative() { x } else { y },
                (Some(x), None) | (None, Some(x)) => x,
                (None, None) => return None,
            };
            c.mul(&bound)?
        } else {
            return None;
        };
        acc = acc.add(&term_ub)?;
    }
    acc.mul(&den.recip()?)
}

/// A manifest LOWER bound of `f`, as a rational: a constant; a form
/// whose numerator is term-wise non-negative apart from its constant
/// term, over a positive constant denominator (`sqrt`/`abs` atoms and
/// even powers are `≥ 0`).
fn lower_bound(f: &Form, sess: &Session) -> Option<Rat> {
    if f.poisoned {
        return None;
    }
    let den = f.den.as_constant()?;
    if den.is_negative() || den.is_zero() {
        return None;
    }
    let mut acc = Rat::zero();
    for (m, c) in f.num.terms() {
        if m.is_empty() {
            acc = acc.add(c)?;
        } else if c.is_negative() || !nonneg_mono(m, sess) {
            return None;
        }
    }
    acc.mul(&den.recip()?)
}

/// **Manifest bound**: `max(A, B) → A` where `B` is manifestly bounded
/// above by a rational that is at most a manifest lower bound of `A`
/// (and the mirror; `min` dually).
pub(super) fn fold_bound(op: SymOp, a: &Form, b: &Form, sess: &Session) -> Option<Form> {
    let b_le_a = |x: &Form, y: &Form| -> bool {
        match (upper_bound(x, sess), lower_bound(y, sess)) {
            (Some(ub), Some(lb)) => ub.neg().and_then(|n| lb.add(&n)).is_some_and(|d| !d.is_negative()),
            _ => false,
        }
    };
    if b_le_a(b, a) {
        return Some(if op == SymOp::Max { a.clone() } else { b.clone() });
    }
    if b_le_a(a, b) {
        return Some(if op == SymOp::Max { b.clone() } else { a.clone() });
    }
    None
}

/// MEASUREMENT ONLY (unsound): the conditioning floor's `max(N, k·m)`
/// with `m` a `min` atom folds to `N` whatever the bound says.
pub(super) fn fold_bound_unsound(op: SymOp, a: &Form, b: &Form, sess: &Session) -> Option<Form> {
    if op != SymOp::Max {
        return None;
    }
    let is_floor = |f: &Form| {
        f.num.terms().len() == 1
            && f.num.terms()[0].0.len() == 1
            && sess
                .atoms
                .get(&f.num.terms()[0].0[0].0)
                .is_some_and(|at| at.op == SymOp::Min)
    };
    if is_floor(b) {
        return Some(a.clone());
    }
    if is_floor(a) {
        return Some(b.clone());
    }
    None
}
