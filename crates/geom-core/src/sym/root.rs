//! **Rule G — the CANONICAL square root**: a `Sqrt` atom's key is a
//! function of its argument's VALUE CLASS, not of the spelling the walk
//! happened to arrive with ([`super::SymRules::canonical_root`]).
//!
//! # The defect it closes
//!
//! Two spellings of one real that the tier keys as two indeterminates
//! cannot cancel, and the tier has no way back: an atom is an opaque
//! indeterminate, so `S · sqrt(1/X)` — the number one, where `X` is
//! `S`'s own argument — stands in a residual forever. That is what a
//! candidate norm on a tilted frame is made of: the normal's own root
//! `S = sqrt(P)` beside `‖v‖ = sqrt(1/S²)`, which rule A turns into
//! `sqrt(1/X)` and rule E's scale step then spells as
//! `sqrt((16/17)/(1 + 8t/17 + 16t²/17))` — a root over a QUOTIENT,
//! keyed on a form nothing else is keyed on.
//!
//! # The canonical form
//!
//! For an argument `N/D`, wherever the root has a value:
//!
//! 1. `sqrt(N/D) = sqrt(N)/sqrt(D)`, under the side condition below;
//! 2. each half's rational CONTENT split out — `p = c·p'` with `c > 0`
//!    the content and `p'` the primitive integer polynomial, then
//!    `c = s²·f` with `s` taken out exactly and `sqrt(f)` a constant
//!    atom — so `sqrt(p) = s · sqrt(f) · sqrt(p')`;
//! 3. `sqrt(R²) = |R|` where the primitive part is a perfect square
//!    ([`signed::poly_sqrt`]), read by rule F ([`manifest::fold_abs`]),
//!    by rule C ([`signed::fold`], gated, under its own dial) or as the
//!    `Abs` ATOM — which is the same indeterminate an `abs` node over
//!    the same form mints.
//!
//! Step 2 is what makes the key a function of the value class: two
//! spellings of one real polynomial differ by a positive rational
//! factor at most, and they have the same primitive part, so they mint
//! ONE atom. Steps 1 and 3 are identities of reals; step 2's `c > 0`
//! holds by construction (the content is a gcd of magnitudes), so
//! `p ≥ 0` and `p' ≥ 0` are the same statement and no sign is read.
//!
//! # The side condition, argued once: `D ≥ 0`
//!
//! `sqrt(N/D) = sqrt(N)/sqrt(D)` needs `D > 0` where it is used, not
//! merely `N/D ≥ 0`: at `N ≤ 0, D < 0` the left side is real and
//! neither root on the right is. `D ≠ 0` is rule E's four-source
//! denominator argument ([`quotient`]) — a denominator the form carries
//! has a value, so it is non-zero wherever the expression has one.
//! Non-negativity comes from exactly these, in this order:
//!
//! 1. **`D` is a positive rational constant.** Read off the form; no
//!    value of any parameter is involved. This is the case rule E's
//!    scale step leaves behind most often.
//! 2. **`D` is manifestly non-negative** ([`manifest::nonneg`]): rule
//!    F's own predicate — non-negative coefficients over monomials that
//!    are even powers or `Sqrt`/`Abs` atoms, or a perfect square. A
//!    fact about the FORM; it reads no value.
//! 3. **The walk has already minted `sqrt(D')` for `D`'s primitive
//!    part `D'`.** A `Sqrt` atom exists only because some node of this
//!    session's DAG computes that root, and a real square root has a
//!    value only where its argument is non-negative; `D` and `D'` differ
//!    by the positive content, so `D ≥ 0` exactly where `D' ≥ 0`. The
//!    claim is therefore "wherever both nodes have values", which is
//!    the scope every clause of the tier already speaks in.
//! 4. **A certified read** — [`signed`]'s bracket over the leaf's box
//!    says `D > 0`. This reads a VALUE, so it is rule C's shape and
//!    rides rule C's dial ([`super::SymRules::signed_root`]); the form
//!    it returns is `gated` and the discharge it reaches is counted
//!    `sign_gated`, not a theorem.
//!
//! `N ≥ 0` then follows from `N/D ≥ 0` and `D > 0` and is never tested
//! on its own; a numerator that is a NEGATIVE constant declines
//! instead, because `N/D ≥ 0` would make `D` negative and source 1
//! has already refused that.
//!
//! **Source 3 depends on minting order, and that is stated rather than
//! hidden.** Whether `sqrt(D')` is in the session when this root is
//! minted is a fact about the walk's traversal, which is deterministic
//! for a given DAG but not a property of the argument alone. What is a
//! property of the argument alone is the KEY: an atom this module mints
//! is keyed on the primitive polynomial and nothing else, so two sites
//! that both split mint one atom whichever went first. What order can
//! change is whether a given root splits at all — never which atom it
//! splits into.
//!
//! # One door
//!
//! [`mint`] is the only place a `Sqrt` atom is built. The walk reaches
//! it from `combine`'s `Sqrt` arm, rule D from [`trig`]'s hand-built
//! roots, and the registered-identity door from the same early walk the
//! registrant's own forms are built in — so a registrant's `‖q − c‖`
//! and the walk's meet by construction rather than by coincidence.

use std::sync::Arc;

use super::form::{Form, Poly};
use super::rational::Rat;
use super::{AtomInfo, Session, SymOp, indet_atom, manifest, mint_atom, signed};

/// `p = c · p'` with `c > 0` the rational CONTENT and `p'` the
/// primitive integer polynomial — the coefficients of `p` divided by
/// the gcd of their magnitudes, so the sign of every coefficient, and
/// hence the sign of `p` at every point, is carried by `p'` alone.
/// `None` for the zero polynomial and wherever the ring refuses.
fn content_split(p: &Poly) -> Option<(Rat, Poly)> {
    let mut content: Option<Rat> = None;
    for (_, k) in p.terms() {
        content = Some(match content {
            None => k.abs(),
            Some(g) => g.content_gcd(k)?,
        });
    }
    let content = content?;
    let primitive = p.scaled(&content.recip()?)?;
    Some((content, primitive))
}

/// The opaque `Sqrt` atom over `arg` — the indeterminate every site
/// mints when the rule declines, keyed on the argument form's digest
/// exactly as `combine` keys a unary atom.
fn atom(op: SymOp, arg: Form, sess: &mut Session, early: bool) -> Form {
    let id = indet_atom(op.tag(), 0, &[arg.digest()]);
    mint_atom(sess, id, early, || AtomInfo {
        op,
        payload: 0,
        args: [Some(Arc::new(arg)), None, None],
    });
    Form::poly(Poly::indet(id))
}

/// `sqrt(c)` for a rational `c ≥ 0`: exact where the rational is a
/// square, else `s · sqrt(f)` for the square-part split `c = s²·f`,
/// with `sqrt(f)` a CONSTANT atom two spellings of `c` share.
fn sqrt_rational(c: &Rat, sess: &mut Session, early: bool) -> Option<Form> {
    if let Some(r) = c.sqrt_exact() {
        return Some(Form::poly(Poly::constant(r)));
    }
    let (s, f) = c.split_square()?;
    let k = atom(SymOp::Sqrt, Form::poly(Poly::constant(f)), sess, early);
    k.mul(&Form::poly(Poly::constant(s)), sess.budget)
}

/// `|R|` for the exact polynomial root `R` of a perfect square.
///
/// Through [`manifest::magnitude`] — rule F's own door — so that `|R|`
/// is `R` itself wherever the FORM already shows `R` non-negative, and
/// the `Abs` ATOM it mints otherwise is the same indeterminate an
/// `abs(R)` node elsewhere in the DAG mints. **Non-negativity, not
/// strict positivity**, is the right test here: `abs` reads a value,
/// not a sign bit, so `|R| = R` holds at `R = 0` too — and it is
/// load-bearing, because `sqrt(|X|²)` must come back as `|X|` and not
/// as a second `abs` wrapped around the first.
///
/// The predicate is read whatever rule F's own dial says: it is a fact
/// about the form, and what the dial governs is rule F's folds at
/// `copysign` and `abs` NODES, not whether the fact is true.
fn magnitude_of_root(r: Poly, sess: &mut Session, early: bool) -> Form {
    let f = Form::poly(r);
    if sess.rules.signed_root
        && !manifest::nonneg(&f, sess)
        && let Some(g) = signed::fold(SymOp::Abs, &f, &sess.params, sess.budget)
    {
        return g;
    }
    match manifest::magnitude(&f, sess) {
        Some(m) => m,
        None => atom(SymOp::Abs, f, sess, early),
    }
}

/// `sqrt(p)` in canonical form — the content split of the module
/// header's step 2 with step 3 on the primitive part. `None` where the
/// polynomial is a negative constant (no real root) or the ring
/// refuses a product.
fn sqrt_poly(p: &Poly, sess: &mut Session, early: bool) -> Option<Form> {
    if p.is_zero() {
        return Some(Form::zero());
    }
    if let Some(c) = p.as_constant() {
        if c.is_negative() {
            return None;
        }
        return sqrt_rational(&c, sess, early);
    }
    let (content, primitive) = content_split(p)?;
    let (s, f) = content.split_square()?;
    let base = match signed::poly_sqrt(&primitive, sess.budget) {
        Some(r) => magnitude_of_root(r, sess, early),
        None => atom(SymOp::Sqrt, Form::poly(primitive), sess, early),
    };
    let mut out = base.mul(&Form::poly(Poly::constant(s)), sess.budget)?;
    if f != Rat::one() {
        let k = atom(SymOp::Sqrt, Form::poly(Poly::constant(f)), sess, early);
        out = out.mul(&k, sess.budget)?;
    }
    Some(out)
}

/// Whether the session already holds `sqrt` of `d`'s primitive part —
/// the module header's side-condition source 3.
fn primitive_root_is_an_atom(d: &Poly, sess: &Session) -> bool {
    content_split(d).is_some_and(|(_, primitive)| {
        let id = indet_atom(SymOp::Sqrt.tag(), 0, &[Form::poly(primitive).digest()]);
        sess.atoms.contains_key(&id)
    })
}

/// Whether the denominator `d` is non-negative wherever the root has a
/// value, and whether saying so read a value (the second half is the
/// `gated` bit the caller must carry). `None` where no source reaches
/// it and the split may not be taken.
fn denominator_nonneg(d: &Poly, sess: &Session) -> Option<bool> {
    if let Some(c) = d.as_constant() {
        return (!c.is_negative()).then_some(false);
    }
    if manifest::nonneg(&Form::poly(d.clone()), sess) || primitive_root_is_an_atom(d, sess) {
        return Some(false);
    }
    if sess.rules.signed_root && signed::enclose_poly(d, sess).is_some_and(|r| r.lo() > 0.0) {
        return Some(true);
    }
    None
}

/// The canonical form of `sqrt(arg)`, or `None` where the rule
/// declines and the caller keeps the opaque atom.
pub(super) fn canonical(arg: &Form, sess: &mut Session, early: bool) -> Option<Form> {
    if arg.poisoned || arg.is_zero() {
        return None;
    }
    if arg.den.as_constant().is_some_and(|c| c == Rat::one()) {
        let mut out = sqrt_poly(&arg.num, sess, early)?;
        out.gated |= arg.gated;
        return Some(out);
    }
    let read = denominator_nonneg(&arg.den, sess)?;
    let num = sqrt_poly(&arg.num, sess, early)?;
    let den = sqrt_poly(&arg.den, sess, early)?;
    let mut out = num.mul(&den.recip()?, sess.budget)?;
    out.gated |= arg.gated || read;
    Some(out)
}

/// **The one door every `Sqrt` atom is minted through**: the canonical
/// form under [`super::SymRules::canonical_root`], the opaque atom
/// otherwise. `early` is the walk the caller is in, which decides
/// whether the mint is noted for the drive memo ([`mint_atom`]).
pub(super) fn mint(arg: Form, sess: &mut Session, early: bool) -> Form {
    if early
        && sess.rules.canonical_root
        && let Some(f) = canonical(&arg, sess, early)
    {
        return f;
    }
    let gated = arg.gated;
    let mut out = atom(SymOp::Sqrt, arg, sess, early);
    out.gated |= gated;
    out
}
