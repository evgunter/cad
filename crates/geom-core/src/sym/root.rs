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
//!    by rule C ([`signed::fold`], gated, under its own dial and in the
//!    last rung's walk, [`super::SymRules::signed_root_last`]) or as the
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
//! neither root on the right is. `D ≠ 0` is rule E's five-source
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
//!    rides rule C's dial ([`super::SymRules::signed_root`]) and not
//!    the last rung's, which folds atoms and nothing else; the form
//!    it returns is `gated` and the discharge it reaches is counted
//!    `sign_gated`, not a theorem.
//!
//! `N ≥ 0` then follows from `N/D ≥ 0` and `D > 0` and is never tested
//! on its own; a numerator that is a NEGATIVE constant declines
//! instead, because `N/D ≥ 0` would make `D` negative and the sources
//! above have already settled `D`'s sign.
//!
//! **The manifestly NEGATIVE magnitude is not decided here.** Rule G's
//! magnitude door (`magnitude_of_root`) re-keys only an argument whose sign
//! no form shows; whether `|X|` folds for a manifestly negative `X` is
//! rule F's own question and SYM-12's measured decision, and this
//! module's normalisation is arranged so that rule F's arms see a
//! signed argument exactly as SYM-8 pinned it. The negative case rule
//! G DOES settle is its own: a manifestly non-positive DENOMINATOR,
//! source 3 above, which negates the pair inside the split and touches
//! rule F's predicate not at all.
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
//! # The exact quotient: a polynomial factor the halves share
//!
//! **The invariant.** A root whose argument `N/D` has a denominator
//! that divides its numerator exactly is minted over the polynomial
//! quotient `Q`, `N = Q·D`, before the split above is asked
//! ([`super::SymRules::root_quotient`], read with rule G's dial as
//! `canonical_root && root_quotient`). Where `D` does not divide `N`,
//! or `Q`'s own root declines, the split is asked as before.
//!
//! **Why it is needed.** Rule E ([`quotient`]) divides out the
//! MONOMIAL both halves share, and says of itself that it is not a
//! polynomial GCD: `(x + 1)` dividing both halves stays. A root keyed
//! through the split of such a quotient keys its halves' factors
//! separately — `|R³|/R²`, say, where the value is `|R|` — and on wide
//! coefficients the split declines on the ring and keys the whole
//! quotient. R1's boss at `bulge = 2` carries exactly that at its
//! `arc_span` identity: `2^-59·sqrt(5)·|c + 2^59·h| − sqrt(P/Q)` with
//! `Q = (1 + h/a)⁴`, `a = c·2^-59`, and `P = 5a²(1 + h/a)⁶` — the
//! chord's own polynomial to the fourth power in both halves. With the
//! factor divided out the root is `sqrt(5(a + h)²)`, whose canonical
//! form is the first term's atom, and the residual is zero.
//!
//! **Why it is an equality of reals, with no value read.** The quotient
//! comes from [`Poly::div_exact`], whose loop holds `rest = N − q·D`
//! exactly at every step, so a zero `rest` is the statement `N = Q·D`
//! as polynomials. So `N/D = Q` as rational functions, and at every
//! point where `D ≠ 0` the two denote one real; `sqrt(N/D) = sqrt(Q)`
//! there, on both sides of zero and at a zero of `Q` (both roots are
//! `0`). `D ≠ 0` at every point clause 1 admits is [`quotient`]'s
//! five-source argument, which this step inherits whole and extends by
//! nothing: it mints no denominator (its output has none). Nothing
//! here consults the box, the atom table or a bracket, so a zero
//! reached through it is a THEOREM.
//!
//! **The sign of `D` stops mattering, and that is correct.** The split
//! `sqrt(N/D) = sqrt(N)/sqrt(D)` needs `D > 0`; the quotient does not,
//! because it never separates the halves. A negative `D` with `D | N`
//! gives a `Q` that is the argument's own value, and whether `sqrt(Q)`
//! has a real value is clause 1's question exactly as it was for
//! `sqrt(N/D)`.
//!
//! **The `abs` and `copysign` hazard.** The step rewrites no `abs` and
//! no `copysign` node. Its output does meet rule F's predicate: a
//! perfect-square `Q = R²` goes through step 3 above, `sqrt(R²) = |R|`,
//! and `|R|` through [`manifest::magnitude`], which folds `|R| = R`
//! only for an `R` the form shows non-negative — `sqrt(R²) = R` there
//! is an identity of reals, the zero included — and otherwise keys the
//! `Abs` atom. No `copysign` is minted, so its signed-zero edge does
//! not arise. A sign-carrying `R` stays opaque; `geom-core`'s
//! `decide_4_root_quotient_rows` pins that on boxes where the tier is
//! asked.
//!
//! **What it does not reach, and the meeting it gives up.**
//! - A factor shared by the halves when neither divides the other
//!   (`(x + 1)(x + 2)/((x + 1)(x + 3))`) — that is a GCD, and a GCD is
//!   not taken here. Only `D | N` is tried, not `N | D`.
//! - It runs at the root door only: an `abs` over the same quotient, or
//!   a quotient that is not under a root, is untouched.
//! - **It re-keys a root the split used to meet.** With `D | N` the
//!   root is `sqrt(Q)`, so it no longer meets the split spelling
//!   `sqrt(N)/sqrt(D)` of the same value that the rule mints wherever
//!   `N` and `D` are written apart — `sqrt(N)` for `N = Q·D` is its
//!   own atom, and meeting it needs `sqrt(Q·D) = sqrt(Q)·sqrt(D)`, a
//!   factorisation of `N` this tier does not take. Asking the split
//!   FIRST trades the other way: it keeps that meeting, but it makes
//!   the quotient's reach depend on the ring. Wherever the ring lets
//!   the split through, `sqrt(5p⁶)/sqrt(p⁴)` keys `|p³|/p²` and not
//!   `|p|`. The MEASURED boss is still taken under that order, because
//!   there the split declines on the ring, but the dyadic twin of its
//!   shape is not. Run split-first, `decide_4_root_quotient_rows` reds
//!   four rows: the boss's dyadic shape, a sign-carrying square over a
//!   squared denominator, the signed-denominator meeting, and the trade
//!   rows. Quotient-first keys the root the same way at every ring
//!   width. No measured document moves on the lost shape. `decide_4_root_quotient_rows` pins both
//!   spellings as the trade, and
//!   `work/decide/the-exact-quotient-re-keys-a-root-the-split-met`
//!   carries them with the remedy, a canonical factorisation of a
//!   root's argument.
//!
//! **The division** is [`Poly::div_exact`]'s: leading terms under the
//! graded-lexicographic monomial order (`form::grlex`), a step cap
//! that is the budget's own term cap, and a decline wherever a
//! leading monomial does not divide or the ring refuses — a missed
//! cancellation, never a wrong one.
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

/// Names the branch the running `canonical` call takes
/// (`profile::RootProfile`); nothing without the profile's feature.
macro_rules! note {
    ($branch:literal) => {
        #[cfg(feature = "sym-profile-testing")]
        super::profile::root_note($branch);
    };
}

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

/// The opaque atom over `arg` — the indeterminate every site mints
/// when the rule declines, keyed on the argument form's digest exactly
/// as `combine` keys a unary atom. **Always the EARLY walk**: rule G
/// runs in no other, so `mint_atom`'s plain-walk bookkeeping has
/// nothing to record here.
fn atom(op: SymOp, arg: Form, sess: &mut Session) -> Form {
    let id = indet_atom(op.tag(), 0, &[arg.digest()]);
    mint_atom(sess, id, true, || AtomInfo {
        op,
        payload: 0,
        args: [Some(Arc::new(arg)), None, None],
    });
    Form::poly(Poly::indet(id))
}

/// `sqrt(c)` for a rational `c ≥ 0`: exact where the rational is a
/// square, else `s · sqrt(f)` for the square-part split `c = s²·f`,
/// with `sqrt(f)` a CONSTANT atom two spellings of `c` share.
fn sqrt_rational(c: &Rat, sess: &mut Session) -> Option<Form> {
    if let Some(r) = c.sqrt_exact() {
        return Some(Form::poly(Poly::constant(r)));
    }
    let (s, f) = c.split_square()?;
    let k = atom(SymOp::Sqrt, Form::poly(Poly::constant(f)), sess);
    k.mul(&Form::poly(Poly::constant(s)), sess.budget)
}

/// The sign of a polynomial's LAST term in STORAGE order — `Mono`'s own
/// `Ord`, which is not a monomial order and not `form::leading`'s grlex.
/// Any fixed choice of term serves a KEY convention; this one is the
/// cheapest. `None` for the zero polynomial, which needs no
/// normalisation.
fn last_stored_is_negative(p: &Poly) -> Option<bool> {
    p.terms().last().map(|(_, c)| c.is_negative())
}

/// **`|Y|` and `|−Y|` are one real, so they are one atom.** The
/// representative is the form whose numerator's last stored coefficient is
/// positive; a magnitude minted over `1 − 2x` and one minted over
/// `2x − 1` then key the same indeterminate, and a root of a perfect
/// square meets the `abs` NODE the document spelled whichever way
/// round `poly_sqrt` happened to return its root.
///
/// **The normalisation is a KEY convention and touches no rule's
/// predicate.** Every fold at an `abs` node — A0's, rule F's manifest
/// sign, rule C's certified read — has already been asked by `combine`
/// on the argument AS WRITTEN and declined by the time this runs, so
/// rule F's arms see what SYM-8 pinned whether the argument is
/// manifestly signed or not, and whether `|X|` folds for a manifestly
/// NEGATIVE `X` stays the measured decision it is (SYM-12's). What
/// changes here is only which indeterminate the magnitude that is left
/// is called.
fn sign_normalised(f: &Form) -> Option<Form> {
    if last_stored_is_negative(&f.num)? {
        let mut out = f.neg()?;
        out.gated = f.gated;
        return Some(out);
    }
    Some(f.clone())
}

/// **`|c · R|` and `c · |R|` are one magnitude, so they are one atom.**
/// The rational content comes out of both halves and the primitives
/// are sign-normalised, leaving the key a function of the value class
/// exactly as a root's is: `(k, R')` with `|Y| = k · |R'|`, `k > 0`.
///
/// Without it rule G's own magnitudes — keyed on the primitive, since
/// that is what a root's content split leaves — would not meet the
/// `abs` NODES a document writes, which are keyed on the form as
/// written. A registrant's `|signed_radius|` and a walk's
/// `sqrt(signed_radius²)` are exactly that pair.
fn magnitude_key(f: &Form) -> Option<(Rat, Form)> {
    let (cn, mut n) = content_split(&f.num)?;
    let (cd, mut d) = content_split(&f.den)?;
    if last_stored_is_negative(&n)? {
        n = n.neg()?;
    }
    if last_stored_is_negative(&d)? {
        d = d.neg()?;
    }
    let k = cn.mul(&cd.recip()?)?;
    let mut out = Form::quotient(n, d);
    out.gated = f.gated;
    Some((k, out))
}

/// **The `Abs` atom's door**: the indeterminate an `abs` NODE mints
/// once every fold before it has declined, keyed by [`magnitude_key`]
/// so `|Y|`, `|−Y|`, `|c·Y|` and `c·|Y|` are ONE atom.
///
/// It folds NOTHING. Every fold at an `abs` node — A0's constant, rule
/// F's manifest sign, rule C's certified one — has already been asked
/// by `combine` and declined; what is left for this door is the key,
/// and the key alone. That is the seam with rule F: this module
/// decides how a magnitude is NAMED, and rule F decides when one may
/// be folded away.
pub(super) fn magnitude_atom(arg: &Form, sess: &mut Session) -> Option<Form> {
    #[cfg(feature = "sym-profile-testing")]
    let t0 = super::profile::root_begin();
    let out = magnitude_atom_in(arg, sess);
    #[cfg(feature = "sym-profile-testing")]
    super::profile::root_abs_done(t0);
    out
}

fn magnitude_atom_in(arg: &Form, sess: &mut Session) -> Option<Form> {
    let (k, primitive) = magnitude_key(arg)?;
    let a = atom(SymOp::Abs, primitive, sess);
    a.mul(&Form::poly(Poly::constant(k)), sess.budget)
}

/// `|R|` for the exact polynomial root `R` of a perfect square — the
/// magnitude rule G itself produces, as against the one an `abs` NODE
/// asks for ([`magnitude_atom`]).
///
/// Here `|R| = R` for a merely NON-NEGATIVE `R` is taken, and it is
/// rule G's own step rather than rule F's: the equality being used is
/// `sqrt(R²) = |R| = R`, an identity of reals at every point `R ≥ 0`
/// admits, the zero included. Rule F's `abs` arm asks a different
/// question — whether to fold an `abs` NODE the document wrote — and
/// answers it on a STRICT predicate for the reason `manifest`'s header
/// gives; nothing here moves that. Rule C's certified fold is asked
/// only after the value-free step, which is `combine`'s documented
/// order.
fn magnitude_of_root(r: Poly, sess: &mut Session) -> Option<Form> {
    let f = sign_normalised(&Form::poly(r))?;
    #[cfg(feature = "sym-profile-testing")]
    let t0 = super::profile::clock();
    let m = manifest::magnitude(&f, sess);
    #[cfg(feature = "sym-profile-testing")]
    super::profile::root_part(
        "magnitude door (manifest; hit = |R| = R)",
        t0,
        m.as_ref().is_some_and(|m| *m == f),
    );
    if let Some(m) = m {
        return Some(m);
    }
    if sess.folds_signed_root()
        && let Some(g) = signed::fold(SymOp::Abs, &f, &sess.params, sess.budget)
    {
        return Some(g);
    }
    magnitude_atom_in(&f, sess)
}

/// `sqrt(p)` in canonical form — the content split of the module
/// header's step 2 with step 3 on the primitive part. `None` where the
/// polynomial is a negative constant (no real root) or the ring
/// refuses a product.
fn sqrt_poly(p: &Poly, sess: &mut Session) -> Option<Form> {
    if p.is_zero() {
        return Some(Form::zero());
    }
    if let Some(c) = p.as_constant() {
        if c.is_negative() {
            return None;
        }
        return sqrt_rational(&c, sess);
    }
    #[cfg(feature = "sym-profile-testing")]
    let t0 = super::profile::clock();
    let split = content_split(p).and_then(|(c, pr)| Some((c.split_square()?, pr)));
    #[cfg(feature = "sym-profile-testing")]
    super::profile::root_part("content split", t0, split.is_some());
    let ((s, f), primitive) = split?;
    // `sqrt(R²) = |R|` — step 3, behind its own dial
    // (`SymRules::root_magnitude`), read as its conjunction with rule
    // G's so that it can only take the step AWAY. With the step off the
    // primitive part stays under a `Sqrt` atom and the content split
    // above it is unchanged. It is a dial because it is the half of
    // rule G that hands six of R2's link's theorems to the rim
    // registrant's axiom; shut as a RETRY it recovered nothing on the
    // five documents SYM-9 measured, so no measured ladder shuts it.
    let magnitude = sess.rules.canonical_root && sess.rules.root_magnitude;
    #[cfg(feature = "sym-profile-testing")]
    let t0 = super::profile::clock();
    let root = magnitude
        .then(|| signed::poly_sqrt(&primitive, sess.budget))
        .flatten();
    #[cfg(feature = "sym-profile-testing")]
    if magnitude {
        super::profile::root_part("poly_sqrt", t0, root.is_some());
    }
    let base = match root {
        Some(r) => magnitude_of_root(r, sess)?,
        None => atom(SymOp::Sqrt, Form::poly(primitive), sess),
    };
    let mut out = base.mul(&Form::poly(Poly::constant(s)), sess.budget)?;
    if f != Rat::one() {
        let k = atom(SymOp::Sqrt, Form::poly(Poly::constant(f)), sess);
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
    #[cfg(feature = "sym-profile-testing")]
    let t0 = super::profile::clock();
    let out = denominator_sign_in(d, sess);
    #[cfg(feature = "sym-profile-testing")]
    super::profile::root_part("side condition", t0, out.is_some());
    out
}

fn denominator_sign_in(d: &Poly, sess: &Session) -> Option<Sign> {
    if let Some(c) = d.as_constant() {
        note!("split: D constant");
        return (!c.is_zero()).then_some(Sign {
            negate: c.is_negative(),
            read: false,
        });
    }
    if manifest::nonneg(&Form::poly(d.clone()), sess) {
        note!("split: D manifestly >= 0");
        return Some(Sign {
            negate: false,
            read: false,
        });
    }
    if manifest::nonneg(&Form::poly(d.neg()?), sess) {
        note!("split: D manifestly <= 0");
        return Some(Sign {
            negate: true,
            read: false,
        });
    }
    #[cfg(feature = "sym-profile-testing")]
    let t0 = super::profile::clock();
    let read = if sess.rules.signed_root {
        signed::enclose_poly(d, sess)
    } else {
        None
    };
    #[cfg(feature = "sym-profile-testing")]
    if sess.rules.signed_root {
        super::profile::root_part(
            "side condition: certified read",
            t0,
            read.as_ref().is_some_and(|r| r.lo() > 0.0 || r.hi() < 0.0),
        );
    }
    if let Some(r) = read {
        note!("split: D read");
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
    note!("declined: no sign for D");
    None
}

/// The canonical form of `sqrt(arg)`, or `None` where the rule
/// declines and the caller keeps the opaque atom.
pub(super) fn canonical(arg: &Form, sess: &mut Session) -> Option<Form> {
    #[cfg(feature = "sym-profile-testing")]
    let t0 = super::profile::root_begin();
    let out = canonical_in(arg, sess);
    #[cfg(feature = "sym-profile-testing")]
    if t0.is_some() {
        super::profile::root_canonical_done(t0, arg.digest(), out.as_ref());
    }
    out
}

fn canonical_in(arg: &Form, sess: &mut Session) -> Option<Form> {
    if arg.poisoned || arg.is_zero() {
        note!("declined: poisoned or zero");
        return None;
    }
    exact_quotient_root(arg, sess).or_else(|| split(arg, sess))
}

/// The exact quotient (module header): where the denominator divides
/// the numerator, the root of the polynomial quotient, asked BEFORE the
/// split. `None` where the step is shut, `D` does not divide `N`, or the
/// quotient's own root declines — and then the split IS asked.
fn exact_quotient_root(arg: &Form, sess: &mut Session) -> Option<Form> {
    if !(sess.rules.canonical_root && sess.rules.root_quotient) {
        return None;
    }
    #[cfg(feature = "sym-profile-testing")]
    let t0 = super::profile::clock();
    let q = arg.num.div_exact(&arg.den, sess.budget);
    #[cfg(feature = "sym-profile-testing")]
    super::profile::root_part("div_exact", t0, q.is_some());
    let mut out = sqrt_poly(&q?, sess)?;
    note!("exact quotient");
    // Redundant under `combine`'s `gate()`, and kept: it matches the
    // split's own branch and holds the label wherever else `canonical`
    // is reached from.
    out.gated |= arg.gated;
    Some(out)
}

/// The split of the module header's steps 1–3: a polynomial argument
/// through `sqrt_poly`, a quotient through `D`'s proved sign.
fn split(arg: &Form, sess: &mut Session) -> Option<Form> {
    if arg.den.as_constant().is_some_and(|c| c == Rat::one()) {
        note!("declined: den = 1, the root");
        let mut out = sqrt_poly(&arg.num, sess)?;
        note!("den = 1");
        out.gated |= arg.gated;
        return Some(out);
    }
    let sign = denominator_sign(&arg.den, sess)?;
    let (n, d) = if sign.negate {
        (arg.num.neg()?, arg.den.neg()?)
    } else {
        (arg.num.clone(), arg.den.clone())
    };
    #[cfg(feature = "sym-profile-testing")]
    let branch = super::profile::root_noted();
    note!("declined: a half's root");
    let num = sqrt_poly(&n, sess)?;
    let den = sqrt_poly(&d, sess)?;
    let mut out = num.mul(&den.recip()?, sess.budget)?;
    #[cfg(feature = "sym-profile-testing")]
    super::profile::root_renote(branch);
    out.gated |= arg.gated || sign.read;
    Some(out)
}

/// **The one door every `Sqrt` atom is minted through**: the canonical
/// form under [`super::SymRules::canonical_root`], the opaque atom
/// otherwise. Early-walk only, like every rule of the algebra — the
/// plain form stays M10-7's and no theorem is ever re-labelled.
pub(super) fn mint(arg: Form, sess: &mut Session) -> Form {
    if sess.rules.canonical_root
        && let Some(f) = canonical(&arg, sess)
    {
        return f;
    }
    let gated = arg.gated;
    let mut out = atom(SymOp::Sqrt, arg, sess);
    out.gated |= gated;
    out
}
