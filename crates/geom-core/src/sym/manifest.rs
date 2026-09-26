//! **Rule F — the manifest sign** behind
//! [`SymRules::manifest_sign`](super::SymRules::manifest_sign): in the
//! early walk a `copysign(Y, X)` node becomes `abs(Y)` and an
//! `abs(X)` node becomes `X` wherever the FORM of `X` already shows
//! `X` positive, and `copysign(Y, X)` becomes `−abs(Y)` and `abs(X)`
//! becomes `−X` wherever the form shows `X` NEGATIVE. Nothing here
//! reads a value: the condition is a fact about the form's syntax, and
//! all four rewrites are equalities of reals at every point clause 1
//! admits, so a zero reached through this rule is a THEOREM and lands
//! in `symbolic_zero`. One rule with two arms under one dial: the
//! negative arm is the positive one reflected, term for term, and a
//! reader who wants them apart reads the argument's leading sign.
//!
//! # The invariant, and then where the atoms come from
//!
//! **An `abs` or `copysign` atom over a form the SYNTAX shows signed
//! is not an unknown function: it is the number the form already
//! denotes, or its negation.** That is the whole rule, and it is a
//! fact about the functions `|·|` and `copysign` rather than about any
//! construction that happens to call them. A kernel that spells a sign
//! decision at all mints such an atom, and every one it mints over a
//! signed form is an indeterminate the tier carries into every product
//! above it for no information at all.
//!
//! **Where they come from today.** The measured mint site is
//! [`Vec3::orthonormal_basis`](crate::Vec3::orthonormal_basis), whose
//! `s = 1.copysign(n.z)` and `r = 1/(1 + |n.z|)` take a `copysign` and
//! an `abs` of one quantity, the frame normal's `z`: on a `FaceFrame`
//! over the END cap of a body extruded from a frame tilted about `u`,
//! that `z` is `1/sqrt(P(t))` for a polynomial `P` in the document's
//! parameter — an `Inv` of a `sqrt` atom, positive wherever it has a
//! value at all — and on the same body's START cap, or with the frame's
//! `v` flipped, it is `−1/sqrt(P(t))`, the same atom negated, which is
//! what the negative arm is for. That construction is not permanent:
//! PROPS' sign-hull work replaces it, and the replacement frame's own
//! `|n.z|` is the next `abs` of the same shape, so the rule outlives
//! the spelling that motivated it.
//!
//! **What the census proves, and what the list is.** Two different
//! claims, kept apart. (1) The EMPIRICAL claim, which covers every
//! mint site whether named below or not: on the seven measured
//! documents whose nominal replay the shape report can take (the
//! plate, the annulus, the link, the bracket, R1's segment boss, both
//! D-tabs), no `copysign` atom from ANY site stands in any residual
//! the tier is asked to decide, with rule F on or shut
//! (`m10_10_evidence_interval`'s
//! `sym12_the_copysign_census_at_the_nominal`, and its gating half
//! `sym12_no_copysign_atom_reaches_a_decision_on_the_cheap_documents`);
//! so the reach either arm has MEASURED is the orthonormal basis's
//! atoms alone. `solid_contain.rs`'s second site, the Cardano root's
//! `(A − B).copysign(−Q)`, is measured directly on the plate, the
//! bracket and the annulus (0 atoms each; the run is OOM-killed on the
//! next document on a small box), and covered on the other four by
//! construction: it runs only inside the one-real-root branch of
//! `cubic_largest_real_root`, in the same call as `cbrt`'s own
//! `copysign`, and both flow into the one returned `z`. The census's
//! measurement of those four documents was taken at a head where
//! `cbrt`'s arguments in that branch were the signed forms
//! `−Q/2 ± √(…)`, so every `z` the branch returned carried a `copysign`
//! atom; none stood in a decided residual, so no `z` from that branch
//! does, and this site's atom exists only inside one. (2) The sites the tree holds at this commit, outside
//! this module and the scalar impls that merely forward the function:
//! `linalg/vec.rs`'s basis; `linalg/svd.rs`'s Householder (`f64`
//! only); `geom-brep/src/implicit.rs`'s cone gradient;
//! `geom-brep/src/props/curved.rs`'s sphere-meridian pole margins;
//! `profile/src/sugar.rs`'s arc-leg fillet trims (two);
//! `profile/src/path.rs`'s line×line fillet turn side;
//! `sweep/src/revolve/axis.rs`'s radial extent;
//! `sweep/src/blend/arms.rs`'s cone nappe;
//! `topo/src/boolean/solid_contain.rs`'s `cbrt` and the Cardano
//! root's sign transfer from `Q`. That list is not
//! prose: `sym_rule_f_rows`'s
//! `the_copysign_mint_sites_the_tree_holds_are_these` greps the
//! shipped sources and reds when a site appears or goes, so the day a
//! new site is minted the census is re-asked rather than the sentence
//! silently overclaiming.
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
//! needs only non-negativity because `D ≠ 0` at every point of a box
//! clause 1 admits — and that is `quotient`'s side condition, argued
//! there in full over the FOUR sources a denominator has. It is NOT
//! "a point where `D` vanishes is a point the value channel divided by
//! zero at": that covers only source (i), and `quotient`'s header names
//! it as the mistake its own paragraph replaces; (ii)–(iv) are non-zero
//! for RANGE reasons instead. This rule inherits that argument whole
//! and adds nothing to it, because it mints no new denominator —
//! `fold_abs` hands back the argument it was given and `magnitude`
//! returns a constant, its argument, or an indeterminate. So `D > 0`
//! and `N > 0` wherever the form has a value, hence `N/D > 0` there.
//!
//! **What is never folded by the positive arm**: a sum of squares
//! alone (`x² + y²` is zero at the origin); a bare even power; a form
//! carrying a parameter, an opaque real or a frozen node at an odd
//! power, or a negative coefficient on any term; the zero form;
//! poison.
//!
//! # The predicate reflected: manifestly NEGATIVE
//!
//! A form `N / D` is **manifestly negative** exactly when `(−N) / D`
//! is manifestly positive — `negative` is `positive` of the negated
//! numerator, and that is the whole of its definition, so the two arms
//! cannot drift apart. Spelled out: every term of `N` has a
//! non-positive coefficient (a `Poly` holds no zero-coefficient term,
//! so non-positive is negative there) and a non-negative monomial, at
//! least one term has a strictly negative coefficient and a monomial
//! whose every indeterminate is manifestly positive (the empty
//! monomial included: a negative constant term carries a sum), and `D`
//! is manifestly non-negative — which is enough for the same reason as
//! above, `D ≠ 0` at every point clause 1 admits, and the arm mints no
//! denominator of its own. Then `N = −(a positive term) − (non-negative
//! terms) < 0` and `D > 0` wherever the form has a value, so
//! `N / D < 0` there. The perfect-square branch is not carried into
//! `N` for the reflected reason: `−r²` vanishes at every zero of `r`.
//!
//! **What the negative arm never folds**: a negated sum of squares
//! (`−(x² + y²)` is zero at the origin); a form with any non-negative
//! term beside its negative ones (`t² − 1/sqrt(…)` has no sign the
//! syntax can read); a negated parameter, opaque real or frozen node
//! at an odd power; a negated perfect square; the zero form; poison.
//!
//! # The four identities, and why each is unconditional
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
//! argument whose sign is `+`. For a manifestly NEGATIVE `X`:
//!
//! ```text
//! abs(X)         = −X,
//! copysign(Y, X) = −|Y|  (and = −1 when Y is the constant 1).
//! ```
//!
//! The first is `|x| = −x` for `x < 0`; the second is the same
//! definition at a second argument whose sign is `−`. `−|Y|` is
//! `magnitude`'s form negated — a constant, `Y` itself, or the `Abs`
//! atom over `Y` — and the negation of a form is exact, so the arm
//! adds no coefficient the positive one would not.
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
//! there unambiguously. The negative arm closes the same edge the same
//! way: a manifestly negative `X` is `< 0` at every such point, so the
//! sign is `−` there, and the real zero — the one point where the two
//! spellings of zero part — is on neither side of either predicate.
//! The tier's own premise carries the rest — every node denotes a
//! real-valued function of its indeterminates and
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
//!
//! What IS a choice is the order against A0 and rule C at this node,
//! and it is pinned by a residual BOTH F and C take: `abs(1 + t²)` over
//! a recorded bracket, and `abs(2/t²)` likewise
//! (`geom-core`'s `sym_rule_f_rows`,
//! `the_order_against_rule_c_is_pinned_by_a_residual_rule_c_would_take`
//! and `a_shape_both_rules_take_is_what_pins_the_order`), and for the
//! negative arm `abs(−(1 + t²))` and `abs(−2/t²)` over the same
//! brackets
//! (`the_negative_arms_order_against_rule_c_is_pinned_the_same_way`).
//! Each is a `theorem` at the shipped order and `sign_gated` with rule
//! F shut, and planting C before F reds every one of them. A residual
//! rule C cannot reach — one built with `Sym::param`, which records no
//! bracket, or one whose
//! argument carries a `sqrt` atom `signed::fold` will not enclose — is
//! green under either order and pins nothing.

use std::sync::Arc;

use super::form::{Form, Mono, Poly};
use super::{AtomInfo, Session, SymOp, indet_atom, signed};

/// How many atom arguments deep `positive` looks before it declines.
/// The normalisation chains this rule is for are two deep — a `sqrt`
/// atom over a polynomial, under an `Inv` — and `sqrt` of `sqrt` of
/// `sqrt` is three; the cap is set well clear of both at four.
///
/// **What it bounds, honestly**: the DEPTH of the atom-argument
/// recursion, and nothing else. It does not bound BREADTH — a term
/// with many atom indeterminates is walked in full at each level — and
/// there is no memo, so an atom appearing twice is tested twice. The
/// predicate is therefore linear in the sub-tree it reaches, not
/// constant per node; what the cap buys is a bound on that sub-tree's
/// depth, so a pathological chain cannot make one node's test walk the
/// whole session. Declining past it is the conservative direction.
const ATOM_DEPTH: usize = 4;

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
    // `!c.is_negative()` alone says STRICTLY positive here: `Poly` holds
    // no zero-coefficient term (`Poly::insert` drops one), so a term
    // that survives has a non-zero coefficient. The zero test that used
    // to sit beside this one was dead by that invariant and said so
    // nowhere.
    termwise_nonneg(p, sess)
        && p.terms()
            .iter()
            .any(|(m, c)| !c.is_negative() && positive_mono(m, sess, depth))
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

/// **A manifestly NEGATIVE form**: `< 0` at every point of the box
/// where it has a value. The reflection of [`positive`], stated once:
/// `N / D` is manifestly negative exactly when `(−N) / D` is manifestly
/// positive. The module header carries the spelled-out predicate and
/// the argument; nothing here adds to either.
pub(super) fn negative(f: &Form, sess: &Session) -> bool {
    if f.poisoned || f.num.is_zero() {
        return false;
    }
    // Test before allocating. `positive(−N / D)` needs every coefficient
    // of `−N` non-negative, so every coefficient of `N` must be negative
    // — a scan that declines the common case, an argument the positive
    // arm has just declined, without materialising `−N`; only a
    // numerator that passes it is negated and read by the predicate.
    if !f.num.terms().iter().all(|(_, c)| c.is_negative()) {
        return false;
    }
    f.neg().is_some_and(|n| positive(&n, sess))
}

/// **`abs(X) → X`** for a manifestly positive `X` and **`abs(X) → −X`**
/// for a manifestly negative one; `None` otherwise. The positive arm
/// is asked first; a form is never both, so the order between the two
/// arms decides nothing.
///
/// **The one home of a signed magnitude.** [`magnitude`] — what a
/// `copysign(Y, X)` node's `|Y|` becomes — reads this first and mints
/// its `Abs` atom only where this declines, so the two spellings of
/// `|Y|` the DAG can hold, an `abs(Y)` node and a `copysign(Y, X)`
/// node's magnitude, fold to the same form or to the same atom and
/// cannot drift apart (they did, for a manifestly negative `Y`, when
/// the negative arm was first cut into this function alone:
/// `sym_rule_f_rows`'s
/// `the_two_spellings_of_a_negative_magnitude_meet`).
pub(super) fn fold_abs(arg: &Form, sess: &Session) -> Option<Form> {
    if positive(arg, sess) {
        return Some(arg.clone());
    }
    if negative(arg, sess) {
        return arg.neg();
    }
    None
}

/// **`copysign(Y, X) → |Y|`** for a manifestly positive `X`, and the
/// negative arm's `−|Y|` is this negated: the magnitude of `Y` as a
/// form.
///
/// A constant `Y` folds to its exact rational magnitude — the case the
/// orthonormal basis mints, `copysign(1, n.z)` — then [`fold_abs`]
/// answers a manifestly signed `Y` (`Y` or `−Y`), and a `Y` the form
/// shows only non-negative is its own magnitude. Anything else mints
/// the `Abs` ATOM over `Y`, which is the same indeterminate an
/// `abs(Y)` node elsewhere in the DAG mints (same op tag, same zero
/// payload, same argument digest) — and only where [`fold_abs`] would
/// have left that node an atom too, so the rewrite trades one opaque
/// atom for another the tier may already hold rather than for a new
/// one.
pub(super) fn magnitude(y: &Form, sess: &mut Session) -> Option<Form> {
    if let Some(n) = y.num.as_constant()
        && let Some(d) = y.den.as_constant()
        && let Some(c) = n.mul(&d.recip()?)
    {
        return Some(Form::poly(Poly::constant(c.abs())));
    }
    if let Some(m) = fold_abs(y, sess) {
        return Some(m);
    }
    if nonneg(y, sess) {
        return Some(y.clone());
    }
    let id = indet_atom(SymOp::Abs.tag(), 0, &[y.digest()]);
    // Through `mint_atom`, not `sess.atoms.entry` by hand: the door
    // keeps the plain walk's `plain_atoms` bookkeeping, and a hand
    // mint would skip it silently the day a rule of this shape ran
    // anywhere but the early walk. `early = true` is this rule's own
    // contract (`SymRules::manifest_sign` needs `early`).
    super::mint_atom(sess, id, true, || AtomInfo {
        op: SymOp::Abs,
        payload: 0,
        args: [Some(Arc::new(y.clone())), None],
    });
    Some(Form::poly(Poly::indet(id)))
}
