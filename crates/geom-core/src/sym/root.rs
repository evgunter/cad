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
//! # The side condition, argued once: `D > 0`, PROVED
//!
//! `sqrt(N/D) = sqrt(N)/sqrt(D)` needs `D > 0` where it is used, not
//! merely `N/D ≥ 0`: at `N ≤ 0, D < 0` the left side is real and
//! neither root on the right is. `D ≠ 0` is rule E's four-source
//! denominator argument ([`quotient`]) — a denominator the form carries
//! has a value, so it is non-zero wherever the expression has one.
//! What is left is a SIGN for `D`, and it is PROVED from the form,
//! never inferred from what else the session happens to hold:
//!
//! 1. **`D` is a rational constant.** Its sign is read off the form; no
//!    value of any parameter is involved. A positive constant splits
//!    as it stands, a negative one through source 3.
//! 2. **`D` is manifestly non-negative** ([`manifest::nonneg`]): rule
//!    F's own predicate — non-negative coefficients over monomials that
//!    are even powers or `Sqrt`/`Abs` atoms, a perfect square, or a
//!    quadratic in one indeterminate with a positive leading
//!    coefficient and no real root. A fact about the FORM; it reads no
//!    value.
//! 3. **`D` is manifestly non-POSITIVE**, i.e. `manifest::nonneg(−D)`:
//!    then `N/D = (−N)/(−D)` and the split is taken over the negated
//!    pair, so `sqrt(x/(−1−y²))` and `sqrt(−x/(1+y²))` key ONE atom
//!    instead of two. Value-free like source 2, because it IS source 2
//!    on the negated denominator.
//! 4. **A certified read** — [`signed`]'s bracket over the leaf's box
//!    says `D > 0`. This reads a VALUE, so it is rule C's shape and
//!    rides rule C's dial ([`super::SymRules::signed_root`]); the form
//!    it returns is `gated` and the discharge it reaches is counted
//!    `sign_gated`, not a theorem.
//!
//! `N ≥ 0` then follows from `N/D ≥ 0` and `D > 0` and is never tested
//! on its own; a numerator that is a NEGATIVE constant declines
//! instead, because `N/D ≥ 0` would make `D` negative and the sources
//! above have already settled `D`'s sign.
//!
//! **What is NOT a source, and why it cannot be.** "The session
//! already holds `sqrt(D')` for `D`'s primitive part, so `D ≥ 0`
//! wherever that node has a value" is FALSE of the DAG this tier
//! walks. The decision door records BOTH arms of every `Select`, and
//! the value of a residual does not depend on the arm the door did not
//! take — so a `sqrt(x)` on a dead arm would license
//! `sqrt(N/x) → sqrt(N)/sqrt(x)` over a box where `x < 0` throughout,
//! and [`manifest`]'s `nonneg` would then read the product of two
//! `Sqrt` atoms as non-negative: a FALSE THEOREM, on a construction
//! (the sign-hull frame) that is made of such arms. The adversary is
//! pinned as a gating row (`sym_root_rows`), and the rule declines
//! rather than asking the atom table. With it goes the old
//! order-dependence disclosure: a proof from the form does not depend
//! on what the walk minted first.
//!
//! # One door
//!
//! [`root::mint`] is the only place a `Sqrt` atom is built. The
//! walk reaches it from `combine`'s `Sqrt` arm, rule D from [`trig`]'s
//! hand-built roots, and the registered-identity door from the same
//! early walk the registrant's own forms are built in — so a
//! registrant's `‖q − c‖` and the walk's meet by construction rather
//! than by coincidence.

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

/// What a denominator's proved sign lets the split do: take the pair
/// as it stands, take it NEGATED (the module header's source 3, which
/// is source 2 on `−D`), and whether saying so READ a value.
struct Sign {
    negate: bool,
    read: bool,
}

/// The proof that `D > 0` where the root has a value, by the module
/// header's sources — or `None`, and then the rule declines and the
/// caller keeps the opaque atom. **No source consults the session's
/// atom table**; the header says why that cannot be one.
fn denominator_sign(d: &Poly, sess: &Session) -> Option<Sign> {
    if let Some(c) = d.as_constant() {
        return (!c.is_zero()).then_some(Sign {
            negate: c.is_negative(),
            read: false,
        });
    }
    if manifest::nonneg(&Form::poly(d.clone()), sess) {
        return Some(Sign {
            negate: false,
            read: false,
        });
    }
    if manifest::nonneg(&Form::poly(d.neg()?), sess) {
        return Some(Sign {
            negate: true,
            read: false,
        });
    }
    if sess.rules.signed_root
        && let Some(r) = signed::enclose_poly(d, sess)
    {
        if r.lo() > 0.0 {
            return Some(Sign {
                negate: false,
                read: true,
            });
        }
        if r.hi() < 0.0 {
            return Some(Sign {
                negate: true,
                read: true,
            });
        }
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
    let sign = denominator_sign(&arg.den, sess)?;
    let (n, d) = if sign.negate {
        (arg.num.neg()?, arg.den.neg()?)
    } else {
        (arg.num.clone(), arg.den.clone())
    };
    let num = sqrt_poly(&n, sess, early)?;
    let den = sqrt_poly(&d, sess, early)?;
    let mut out = num.mul(&den.recip()?, sess.budget)?;
    out.gated |= arg.gated || sign.read;
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
