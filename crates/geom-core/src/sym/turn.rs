//! **Rule H — angle sums, exact** behind
//! [`SymRules::angle_sum`](super::SymRules::angle_sum): every `sin` and
//! `cos` atom of the early walk is minted over a CANONICAL argument,
//! and a product of two such atoms is rewritten as `sin`/`cos` of the
//! sum and difference of their arguments. Nothing here reads a value:
//! every identity below is a theorem about the reals, and each rewrite
//! is an equality of forms.
//!
//! # What it is for
//!
//! A ring of `N` members turned by `k · turn/N` meets its neighbour
//! where member `k`'s face, turned by `cos(2πk/N)` and `sin(2πk/N)`,
//! lies on member `k + 1`'s, written through `cos(2π(k+1)/N)` and
//! `sin(2π(k+1)/N)`. The residual between the two is a polynomial in
//! those atoms that vanishes by angle addition, and held opaque the
//! atoms of `2πk/N`, `2π/N` and `2π(k+1)/N` are unrelated
//! indeterminates. Linearised, both sides are `sin`/`cos` of the one
//! angle `2π(k+1)/N`, and the residual is the zero form.
//!
//! # The canonical argument
//!
//! An argument whose denominator is a constant is split into its
//! RATIONAL-TURN part `q·π` (the coefficient of the monomial `π` alone)
//! and the rest `ω`, with `ω`'s denominator divided in so that two
//! spellings of one rest are one form. Then
//!
//! ```text
//! sin(ω + qπ) = sin ω · cos qπ + cos ω · sin qπ,
//! cos(ω + qπ) = cos ω · cos qπ − sin ω · sin qπ,
//! ```
//!
//! so every atom this rule mints is of one of two CLASSES: a
//! **constant** atom, of `qπ` alone, and a **symbolic** atom, of an `ω`
//! with no rational-turn part (a rational number of radians, `sin 1`,
//! among them).
//!
//! - A constant angle is reduced to the first octant: `qπ` modulo `2π`,
//!   then `x − π` (both negate), `π − x` (the cosine negates), and
//!   `π/2 − x` (the two swap), which leaves `q ∈ [0, 1/4]`, with the
//!   sine at `π/4` spelled as its cosine. At `q = 0` the value folds
//!   (`0`, `1`); otherwise the atom is minted over the form `q·π`
//!   itself, `sin(π/6)` included (below). Every constant atom is
//!   therefore keyed by its angle in the first octant, as
//!   [`Turns`](crate::Turns) reads an exact turn, and two spellings of
//!   one value are one atom.
//! - A symbolic angle's sign is normalised: of `ω` and `−ω` the one
//!   with the smaller digest is the argument, by `sin(−ω) = −sin ω` and
//!   `cos(−ω) = cos ω`. Rule D's closed form of `q · atan X` is asked
//!   of `ω` before its atom is minted, as at a node.
//!
//! An argument with a polynomial denominator is its own `ω`, its sign
//! normalised the same way: its rational-turn part is not read.
//!
//! # The product rewrite
//!
//! Per node in the early walk (as rules A/B run), every term carrying
//! two `sin`/`cos` atoms of ONE class has the pair replaced by
//!
//! ```text
//! cos A cos B = ½ cos(A − B) + ½ cos(A + B),
//! sin A sin B = ½ cos(A − B) − ½ cos(A + B),
//! sin A cos B = ½ sin(A + B) + ½ sin(A − B),
//! ```
//!
//! each new atom minted through the canonical argument above, until no
//! term carries two atoms of one class. A pair of one constant and one
//! symbolic atom is the split above and is left as it is, so the
//! rewrite cannot undo the split. It terminates: a step lowers the
//! symbolic atoms' degree in a term by one (a sum of two `ω`s carries
//! no rational-turn part, so its atom is one symbolic atom), or keeps
//! it and lowers the constant atoms' degree by one; a step cap and the
//! size budget stand behind that argument. Rule B's `sin² = 1 − cos²`
//! is this rewrite's square case and meets it: both reach `½ − ½ cos 2A`.
//!
//! A product of `m` atoms of one class becomes up to `2^(m−1)` terms.
//! The budget is what bounds that, and a term the rewrite cannot fit
//! keeps its atoms, which is the conservative direction.
//!
//! # The deferred fold
//!
//! The rewrite is exact over the angles as group elements, each
//! constant atom the real or imaginary part of its own `ζ = e^{iqπ}`,
//! and folding
//! `sin(π/6)`, the one first-octant atom whose value is rational, to
//! `1/2` mid-walk would impose `ζ₆`'s relation on that element alone:
//! `cos(2π/9) · cos(2π/3)` would be `−½ cos(2π/9)`, never the
//! `½ cos(8π/9) + ½ cos(4π/9)` its neighbour's product meets, and a
//! ring of 9 would leave the steps beside its members at a third of a
//! turn. So `sin(π/6)` stays an atom, [`Turns`](crate::Turns) reads a
//! twelfth's sine as that atom (`Real::sin_of_a_twelfth`), and [`settle`](crate::sym::turn::settle)
//! folds it to `1/2` where the walk's form is read: its output, and
//! every argument but a ring op's (an atom's, a reciprocal's, a
//! decision's), so a value read there is the value the fold gives.
//!
//! # What it does not reach
//!
//! The constant atoms are linearly independent of each other only up to
//! the algebraic relations among the `cos(qπ)` (`cos(π/9) − cos(2π/9) =
//! sin(π/18)`); a residual that vanishes only through such a relation
//! keeps its atoms. A ring's step is an angle addition and needs none.

use std::collections::HashMap;
use std::sync::Arc;

use super::form::{Form, Mono, Poly, mono_mul};
use super::rational::{Int, Rat};
use super::{AtomInfo, INDET_PI, Session, SymOp, indet_atom, mint_atom, unary_at_zero};

/// The most pair rewrites one node's form takes before it is left as it
/// was.
const STEPS: usize = 4096;

/// A rational as the integer fraction `p / s`, `s > 0`, where both fit a
/// machine word.
fn fraction(q: &Rat) -> Option<(i128, i128)> {
    let (Int::Small(num), Int::Small(den)) = (&q.num, &q.den) else {
        return None;
    };
    let shift = q.exp2.unsigned_abs();
    // `checked_shl` refuses a shift past the width only, and `1 << 127`
    // is negative: a shift past 126 is no machine fraction.
    if shift > 126 {
        return None;
    }
    if q.exp2 >= 0 {
        Some((num.checked_mul(1i128.checked_shl(shift)?)?, *den))
    } else {
        Some((*num, den.checked_mul(1i128.checked_shl(shift)?)?))
    }
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    (a, b) = (a.abs(), b.abs());
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// `sin` or `cos` of `(p/s)·π` read off the first octant: the op, the
/// reduced angle `p'/s' ∈ [0, 1/4]`, and whether the value negates.
fn first_octant(op: SymOp, p: i128, s: i128) -> Option<(SymOp, i128, i128, bool)> {
    let (mut op, mut neg) = (op, false);
    // `r/s ∈ [0, 2)`: the angle modulo a turn.
    let mut r = p.rem_euclid(s.checked_mul(2)?);
    let mut s = s;
    if r >= s {
        // `x − π`: the sine and the cosine both negate.
        r -= s;
        neg = !neg;
    }
    if r.checked_mul(2)? > s {
        // `π − x`: the sine keeps, the cosine negates.
        r = s - r;
        if op == SymOp::Cos {
            neg = !neg;
        }
    }
    let quarter = r.checked_mul(4)?;
    if quarter > s || (quarter == s && op == SymOp::Sin) {
        // `π/2 − x`: the two swap.
        (r, s) = (s.checked_sub(r.checked_mul(2)?)?, s.checked_mul(2)?);
        op = if op == SymOp::Sin {
            SymOp::Cos
        } else {
            SymOp::Sin
        };
    }
    let g = gcd(r, s).max(1);
    Some((op, r / g, s / g, neg))
}

/// The form `c · x` for a rational `c`.
fn times(c: Rat, x: u128) -> Form {
    Form::poly(Poly::term(vec![(x, 1)], c))
}

/// One atom over `arg`, keyed as a node's would be and recorded with
/// its argument, so the reads that enclose an atom reach it.
fn atom(sess: &mut Session, op: SymOp, arg: Form) -> Form {
    let id = indet_atom(op.tag(), 0, &[arg.digest()]);
    mint_atom(sess, id, true, || AtomInfo {
        op,
        payload: 0,
        args: [Some(Arc::new(arg)), None, None],
    });
    Form::poly(Poly::indet(id))
}

/// `sin`/`cos` of the constant angle `(p/s)·π`: `0` or `1` at the
/// angle `0`, else `±` its atom, `sin(π/6)` among them (module docs).
fn constant(sess: &mut Session, op: SymOp, p: i128, s: i128) -> Option<Form> {
    let (op, p, s, neg) = first_octant(op, p, s)?;
    let f = if p == 0 {
        unary_at_zero(op)?
    } else {
        atom(sess, op, times(Rat::new(p, s, 0)?, INDET_PI))
    };
    if neg { f.neg() } else { Some(f) }
}

/// The key of the atom `sin(π/6)`, the one constant atom whose value is
/// rational.
fn sixth() -> Option<u128> {
    static KEY: std::sync::OnceLock<Option<u128>> = std::sync::OnceLock::new();
    *KEY.get_or_init(|| {
        let arg = times(Rat::new(1, 6, 0)?, INDET_PI);
        Some(indet_atom(SymOp::Sin.tag(), 0, &[arg.digest()]))
    })
}

/// `num / den`, or poison where `den` is the zero polynomial: a
/// denominator the rewrite or the fold shows to vanish is a real zero,
/// and the form has no value. The walk settles a reciprocal's argument
/// where it is made, so no document is known to reach this zero; it is
/// the backstop behind that settle.
fn over(num: Poly, den: Poly) -> Form {
    if den.is_zero() {
        Form::poison()
    } else {
        Form::quotient(num, den)
    }
}

/// `p` with `sin(π/6)` read as `1/2`.
fn settle_poly(p: &Poly, id: u128) -> Option<Poly> {
    let mut out = Poly::zero();
    for (mono, c) in p.terms() {
        let e = super::form::exp_of(mono, id);
        if e == 0 {
            out.insert(mono.clone(), c.clone())?;
            continue;
        }
        let half = Rat::new(1, 1, -i32::try_from(e).ok()?)?;
        let rest: Mono = mono.iter().copied().filter(|&(x, _)| x != id).collect();
        out.insert(rest, c.mul(&half)?)?;
    }
    Some(out)
}

/// **The fold rule H defers** (module docs): `f` with `sin(π/6)` read
/// as its value `1/2` ([`over`]), `None` where `f` does not carry it
/// or the fold does not fit the ring, and `f` stands.
pub(super) fn settle(f: &Form, sess: &Session) -> Option<Form> {
    let id = sixth()?;
    if f.poisoned || !sess.atoms.contains_key(&id) {
        return None;
    }
    let carries = |p: &Poly| p.monos().any(|m| super::form::exp_of(m, id) > 0);
    if !carries(&f.num) && !carries(&f.den) {
        return None;
    }
    Some(over(settle_poly(&f.num, id)?, settle_poly(&f.den, id)?))
}

/// `sin`/`cos` of a symbolic angle `ω` (no rational-turn part), its
/// sign normalised, rule D's closed form asked first.
fn symbolic(sess: &mut Session, op: SymOp, omega: &Form) -> Option<Form> {
    if omega.is_zero() {
        return unary_at_zero(op);
    }
    let flipped = omega.neg()?;
    let (arg, neg) = if flipped.digest() < omega.digest() {
        (flipped, op == SymOp::Sin)
    } else {
        (omega.clone(), false)
    };
    let f = if sess.rules.trig_of_atan
        && let Some(f) = super::trig::fold(op, &arg, sess)
    {
        f
    } else {
        atom(sess, op, arg)
    };
    if neg { f.neg() } else { Some(f) }
}

/// **`sin` or `cos` of `arg`, over the canonical argument** (module
/// docs). `None` where the rule does not fit — a rational-turn part past
/// a machine fraction, a product past the budget — and the caller mints
/// the node as it would without the rule.
pub(super) fn mint(op: SymOp, arg: &Form, sess: &mut Session) -> Option<Form> {
    debug_assert!(matches!(op, SymOp::Sin | SymOp::Cos));
    if arg.poisoned {
        return Some(Form::poison());
    }
    let budget = sess.budget;
    let Some(den) = arg.den.as_constant() else {
        return symbolic(sess, op, arg);
    };
    let inv = den.recip()?;
    let mut rest = Poly::zero();
    let mut q = Rat::zero();
    for (mono, c) in arg.num.terms() {
        let c = c.mul(&inv)?;
        if mono.as_slice() == [(INDET_PI, 1)] {
            q = c;
        } else {
            rest.insert(mono.clone(), c)?;
        }
    }
    let omega = Form::poly(rest);
    let (p, s) = fraction(&q)?;
    if p == 0 {
        return symbolic(sess, op, &omega);
    }
    if omega.is_zero() {
        return constant(sess, op, p, s);
    }
    let (c, sn) = (
        constant(sess, SymOp::Cos, p, s)?,
        constant(sess, SymOp::Sin, p, s)?,
    );
    let (so, co) = (
        symbolic(sess, SymOp::Sin, &omega)?,
        symbolic(sess, SymOp::Cos, &omega)?,
    );
    match op {
        SymOp::Sin => so.mul(&c, budget)?.add(&co.mul(&sn, budget)?, budget),
        _ => co
            .mul(&c, budget)?
            .add(&so.mul(&sn, budget)?.neg()?, budget),
    }
}

/// The class of a `sin`/`cos` atom: `Some(true)` for a constant one (its
/// argument reads π alone), `Some(false)` for a symbolic one, `None` for
/// an indeterminate that is no `sin`/`cos` atom.
fn class(sess: &Session, id: u128) -> Option<bool> {
    let info = sess.atoms.get(&id)?;
    if !matches!(info.op, SymOp::Sin | SymOp::Cos) {
        return None;
    }
    let arg = info.args[0].as_ref()?;
    // `q·π` alone: an angle with a rational term (`sin 1`) is symbolic,
    // or a product of it with a constant atom would be split back into
    // the pair it came from.
    let on_pi = matches!(arg.num.terms(), [(m, _)] if m.as_slice() == [(INDET_PI, 1)]);
    Some(on_pi && arg.den.as_constant().is_some())
}

/// The first two factors of one class in `mono` (one atom twice where
/// its power is past one), or `None` where no class has two.
fn pair(sess: &Session, mono: &Mono) -> Option<(u128, u128)> {
    let mut first: [Option<u128>; 2] = [None, None];
    for &(id, e) in mono {
        let Some(c) = class(sess, id) else { continue };
        let slot = &mut first[usize::from(c)];
        match *slot {
            Some(x) => return Some((x, id)),
            None if e >= 2 => return Some((id, id)),
            None => *slot = Some(id),
        }
    }
    None
}

/// `mono` with one power of `x` and one of `y` taken out.
fn without(mono: &Mono, x: u128, y: u128) -> Mono {
    let mut out = mono.clone();
    for id in [x, y] {
        if let Some(at) = out.iter().position(|&(i, _)| i == id) {
            if out[at].1 == 1 {
                out.remove(at);
            } else {
                out[at].1 -= 1;
            }
        }
    }
    out
}

/// A form with a constant denominator as a polynomial.
fn as_poly(f: &Form) -> Option<Poly> {
    if f.poisoned {
        return None;
    }
    f.num.scaled(&f.den.as_constant()?.recip()?)
}

/// The product of the atoms `x` and `y` as a sum of single atoms
/// (module docs), or `None` where it is no polynomial.
fn product(sess: &mut Session, x: u128, y: u128) -> Option<Poly> {
    let budget = sess.budget;
    let read = |id: u128| {
        let info = sess.atoms.get(&id)?;
        Some((info.op, (**info.args[0].as_ref()?).clone()))
    };
    let ((ox, a), (oy, b)) = (read(x)?, read(y)?);
    let sum = a.add(&b, budget)?;
    let diff = a.add(&b.neg()?, budget)?;
    let half = Rat::new(1, 1, -1)?;
    // `½ (σ · f(A + B) + δ · f(A − B))`, `f` the sum's op.
    let (op, sigma, delta) = match (ox, oy) {
        (SymOp::Cos, SymOp::Cos) => (SymOp::Cos, 1, 1),
        (SymOp::Sin, SymOp::Sin) => (SymOp::Cos, -1, 1),
        (SymOp::Sin, SymOp::Cos) => (SymOp::Sin, 1, 1),
        _ => (SymOp::Sin, 1, -1),
    };
    let signed = |p: Poly, sign: i32| if sign < 0 { p.neg() } else { Some(p) };
    let s = signed(as_poly(&mint(op, &sum, sess)?)?, sigma)?;
    let d = signed(as_poly(&mint(op, &diff, sess)?)?, delta)?;
    s.add(&d)?.scaled(&half)
}

/// One polynomial with every pair of one class rewritten, or `None`
/// past the step cap or the budget.
fn linear_poly(
    p: &Poly,
    sess: &mut Session,
    memo: &mut HashMap<(u128, u128), Option<Poly>>,
    steps: &mut usize,
) -> Option<Poly> {
    let budget = sess.budget;
    let mut out = Poly::zero();
    let mut work: Vec<(Mono, Rat)> = p.terms().to_vec();
    while let Some((mono, c)) = work.pop() {
        let Some((x, y)) = pair(sess, &mono) else {
            out.insert(mono, c)?;
            continue;
        };
        let key = (x.min(y), x.max(y));
        let sum = match memo.get(&key) {
            Some(held) => held.clone(),
            None => {
                let made = product(sess, key.0, key.1);
                memo.insert(key, made.clone());
                made
            }
        };
        let Some(sum) = sum else {
            out.insert(mono, c)?;
            continue;
        };
        *steps += 1;
        if *steps > STEPS || work.len() + sum.terms().len() > 4 * budget.max_terms {
            return None;
        }
        let rest = without(&mono, x, y);
        for (m, k) in sum.terms() {
            work.push((mono_mul(&rest, m)?, c.mul(k)?));
        }
        if out.terms().len() > budget.max_terms {
            return None;
        }
    }
    Some(out)
}

/// **The product rewrite over one form** (module docs): `Some` of the
/// rewritten form ([`over`]), the form itself where no term has a
/// pair, `None` where the rewrite does not fit and the caller keeps the
/// form.
pub(super) fn linearise(f: &Form, sess: &mut Session) -> Option<Form> {
    if f.poisoned {
        return Some(f.clone());
    }
    let has_pair = |p: &Poly, sess: &Session| p.monos().any(|m| pair(sess, m).is_some());
    if !has_pair(&f.num, sess) && !has_pair(&f.den, sess) {
        return Some(f.clone());
    }
    let mut memo = HashMap::new();
    let mut steps = 0;
    let num = linear_poly(&f.num, sess, &mut memo, &mut steps)?;
    let den = linear_poly(&f.den, sess, &mut memo, &mut steps)?;
    Some(over(num, den))
}
