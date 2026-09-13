//! **Rule D — trig of `atan`, exact** behind
//! [`SymRules::trig_of_atan`](super::SymRules::trig_of_atan): a `sin`
//! or `cos` node whose argument form is `q · atan(X)`, with `q` a
//! dyadic rational, rewrites to its CLOSED FORM in `X` and the atom
//! `sqrt(1 + X²)`. Nothing here reads a value: every identity below is
//! a theorem about the reals, and the fold is an equality of forms.
//!
//! # Why the two spellings of an arc need it
//!
//! A sketch arc is pushed forward through `sin(s·θ)` and `−2·sin²(s·θ/2)`
//! with `θ = 4·atan(bulge)` (`geom-brep`'s `SketchSegment::eval`), and
//! its carrier is evaluated through `cos t`, `sin t` at
//! `t = (i/8)·4·atan|bulge|` (`Curve3::circle_at` over the certifier's
//! schedule). Held opaque, `sin(½·atan b)` and `cos(atan b)` are two
//! unrelated indeterminates and the residual between the spellings is
//! not the zero form anywhere the trig has not collapsed. Written in
//! closed form both sides are rational functions of `X` and one
//! `sqrt` atom, and rules A/B (`super::algebra`) close the ring.
//!
//! # The identities, and why each is unconditional
//!
//! Let `φ = atan X`. For EVERY real `X`, `φ ∈ (−π/2, π/2)`, so `cos φ >
//! 0` and
//!
//! ```text
//! cos φ = 1 / sqrt(1 + X²),      sin φ = X / sqrt(1 + X²).
//! ```
//!
//! **Halves, on the positive branch.** Every angle this module halves
//! is `φ / 2ʲ` for some `j ≥ 0`, and every such angle lies in
//! `(−π/2, π/2)`; its half lies in `(−π/4, π/4)`, where the cosine is
//! strictly positive. So for such a `θ`
//!
//! ```text
//! cos(θ/2) = + sqrt((1 + cos θ) / 2),    sin(θ/2) = sin θ / (2·cos(θ/2)),
//! ```
//!
//! the first because the half-angle formula's sign is the sign of
//! `cos(θ/2)`, which the RANGE of `atan` fixes — a fact about the
//! function, not about any value of `X` — and the second the
//! double-angle identity `sin θ = 2·sin(θ/2)·cos(θ/2)` divided by a
//! cosine the same range keeps away from zero. Neither reads a sign:
//! the sign of `sin(θ/2)` rides in `sin θ`, i.e. in `X`, as a form.
//!
//! **Integer multiples** by angle addition, `cos((n+1)ψ) = cos(nψ)·cos
//! ψ − sin(nψ)·sin ψ` and `sin((n+1)ψ) = sin(nψ)·cos ψ + cos(nψ)·sin
//! ψ`, and `cos(−kψ) = cos(kψ)`, `sin(−kψ) = −sin(kψ)`.
//!
//! # What folds, and what never does
//!
//! Only an argument whose form is EXACTLY `q · A` — one term, the
//! indeterminate `A` an `atan` atom to the first power, the coefficient
//! `q = k / 2ᵐ` with `|k| ≤ MAX_MULTIPLE` and `m ≤ MAX_HALVINGS`
//! — and nothing else: an `atan2` atom (a different op), `q·atan(X) +
//! c` (two terms), `atan(X)·atan(Y)` (degree two), a non-dyadic `q`
//! (`atan(X)/3` has no closed form this module states), or an atom
//! over a poisoned argument, all stay opaque, which is the conservative
//! direction. The certifier's schedule produces `q = i/2` and `i/4` for
//! `i ∈ 0..=8` (`sample_param` at `θ = 4·atan|b|`, and the pushforward's
//! `s·θ` and `s·θ/2` at `s = i/8`) plus the mid-parameter `2·atan|b|`;
//! the bounds hold those with room and nothing folds past them.
//!
//! The `sqrt` atoms this module mints are recorded like every other
//! atom, so rule A reaches their squares, and they are keyed by their
//! argument's form, so the two spellings of one arc mint ONE atom each.
//!
//! **The second fold: `atan2(0, N) = 0` for an `N` non-negative by its
//! syntax** (`manifestly_nonneg`) — the cylinder chart's phase,
//! `atan2(a_r · v_ref, a_r · u_ref)` with `u_ref` the start's own
//! radial, is `atan2(0, r²/sqrt(r²))`, and the arc exists only where
//! that radial length is positive. Every other `atan2` stays an atom.
//!
//! **The third fold: `sin`/`cos` at an exact half-multiple of π**
//! (`fold_at_half_pi`) — what the branch-stabilized azimuth leaves
//! behind on a definitely-negative frame, where the phase is
//! `atan2(−y, −x) + π` and, once the `atan2` has folded, `π` itself:
//! `cos π = −1`, `sin π = 0`, and the other three quadrants likewise.
//! `π` is the form's own indeterminate, so this is the value of a
//! function at a known constant read off the form; `cos(π/3)` and
//! `π` beside anything else stay atoms.
//!
//! Both A1 folds live under rule D's dial: they are the same rule's
//! posture (a function's range or value at a form the rule can read,
//! no value of any parameter read) and the same measurement's
//! mechanism — the chart's phase is what stood between the plate and
//! its real study, and the two folds together are what takes it.

use std::rc::Rc;

use super::{
    AtomInfo, Form, INDET_PI, Mono, Poly, Rat, Session, SymBudget, SymOp, indet_atom, signed,
    within,
};

/// **`atan2(0, N) = 0` for an `N` that is non-negative BY ITS SYNTAX**
/// — the second fold of rule D, on the same posture as the half-angle
/// branch: a fact about a function's range read off the form, never a
/// value.
///
/// `atan2(z, n)` is the angle of the point `(n, z)`; with `z = 0` it is
/// `0` for `n > 0` and `π` for `n < 0`. A form is manifestly
/// non-negative when its numerator and denominator are each either
/// (a) a polynomial every term of which has a positive coefficient and
/// a monomial whose indeterminates are `sqrt` or `abs` atoms (to any
/// power) or anything else to an EVEN power — every such term is a
/// product of non-negative reals wherever it has a value — or (b) a
/// PERFECT SQUARE of a polynomial (`signed::poly_sqrt`, an exact
/// arithmetic test that reads no value): `(k + δ)²` is one, and the
/// chart phase's `r²/sqrt(r²)` is `(b)` over `(a)`. Such an `N` is
/// `> 0` at every parameter point where it is defined and not zero,
/// so the fold is a theorem there; where `N = 0` the geometry it comes
/// from is degenerate (a radial of length zero), clause 1 — the
/// numeric channel's own domain answer — decides first, and IEEE's
/// `atan2(0, 0) = 0` agrees with the fold wherever a value exists. The
/// zero polynomial itself is refused (nothing is claimed about
/// `atan2(0, 0)` as a form), and so is a poisoned operand.
///
/// What never folds: `atan2(0, X)` for a plain parameter `X` (an odd
/// power, no sign known), `atan2(Y, N)` with `Y` not the zero form
/// (a numeric zero is a coincidence, not a form), and any `N` that is
/// non-negative only in value.
pub(super) fn manifestly_nonneg(n: &Form, sess: &Session) -> bool {
    if n.poisoned || n.num.is_zero() {
        return false;
    }
    let nonneg_mono = |m: &Mono| {
        m.iter().all(|&(id, e)| {
            e % 2 == 0
                || sess
                    .atoms
                    .get(&id)
                    .is_some_and(|a| matches!(a.op, SymOp::Sqrt | SymOp::Abs))
        })
    };
    let nonneg_poly = |p: &Poly| {
        p.terms
            .iter()
            .all(|(m, c)| !c.is_negative() && nonneg_mono(m))
            || signed::poly_sqrt(p, sess.budget).is_some()
    };
    nonneg_poly(&n.num) && nonneg_poly(&n.den)
}

/// The largest `|k|` in `q = k / 2ᵐ` this rule folds.
pub(super) const MAX_MULTIPLE: i128 = 32;

/// The most halvings `m` in `q = k / 2ᵐ` this rule folds: the
/// certifier's schedule needs quarter angles at most (`q = i/2` from
/// the carrier's samples and `i/4` from the pushforward's `s·θ/2`), and
/// past that the ring does NOT close — at `m = 3` the fold of
/// `3/8 · atan X` and the angle-addition spelling `1/4 + 1/8` carry the
/// same three `c₂` atoms but the per-node reduction never meets them
/// (R2's note: `3/8 = 1/4 + 1/8` refuses over every box). A fold that
/// cannot be met by its own algebra is a missed cancellation, never a
/// wrong one, but stating a schedule wider than the algebra closes is
/// a claim the code does not keep, so the cap is the schedule's.
///
/// **What the minted atoms are keyed on.** `sqrt(1 + X²)` is keyed on
/// the form `1 + X·X` exactly as built here — the un-reduced square —
/// and each half-angle `c₂` on the un-cancelled quotient `(D + C)/(2D)`.
/// A hand-spelled `sqrt(1 + x·x)` over the same `x` shares the atom;
/// one spelled with a `sqrt` or a denominator inside `X`, or with the
/// quotient cancelled by hand, mints a different atom and the residual
/// stays NUMERIC (never false: two atoms that are not the same node
/// are two indeterminates).
pub(super) const MAX_HALVINGS: u32 = 2;

/// The argument form read as `(k / 2ᵐ) · atan(X)`: `(k, m, X)`, or
/// `None` where the form is not of that shape.
fn read_argument(arg: &Form, sess: &Session) -> Option<(i128, u32, Rc<Form>)> {
    if arg.poisoned {
        return None;
    }
    let den = arg.den.as_constant()?;
    if den.is_zero() {
        return None;
    }
    if arg.num.terms.len() != 1 {
        return None;
    }
    let (mono, coeff) = arg.num.terms.iter().next()?;
    let [(atom, 1)] = mono.as_slice() else {
        return None;
    };
    let info = sess.atoms.get(atom)?;
    if info.op != SymOp::Atan {
        return None;
    }
    let x = info.args[0].clone()?;
    let q = coeff.mul(&den.recip()?)?;
    // A dyadic rational is `k · 2^e` with an odd `k` and no other
    // denominator (`Rat` keeps its integers odd).
    if !q.den.is_one() {
        return None;
    }
    let super::Int::Small(k) = q.num else {
        return None;
    };
    // The multiple `k · 2^e` is formed by CHECKED multiplication, never
    // by a shift: `i128::checked_shl` refuses only a shift of 128 or
    // more and WRAPS below that, so a coefficient such as `(2¹²³ + 1) ·
    // 2⁵` read through it came back as `32` and folded — a false
    // theorem, unreachable from any document (a 53-bit product is
    // `Int::Big` and refused above) but a false theorem all the same
    // (`work/m10/rule-d-multiple-reader-wraps-on-a-huge-dyadic-coefficient`,
    // R1's `r1_the_multiple_reader_wraps_on_a_huge_dyadic_coefficient`
    // is the pin). An overflow here is a multiple past every cap, and
    // `None` is the answer.
    let (k, m) = if q.exp2 >= 0 {
        let scale = 2_i128.checked_pow(u32::try_from(q.exp2).ok()?)?;
        (k.checked_mul(scale)?, 0)
    } else {
        (k, q.exp2.unsigned_abs())
    };
    if k.unsigned_abs() > MAX_MULTIPLE.unsigned_abs() || m > MAX_HALVINGS {
        return None;
    }
    Some((k, m, x))
}

/// **`sin`/`cos` at an exact half-multiple of π** — the third fold of
/// rule D: an argument form that is exactly `(k/2) · π`, `k` an
/// integer, has `cos = 1, 0, −1, 0` and `sin = 0, 1, 0, −1` by `k mod
/// 4`. The value of a function at a known constant, read off the form
/// (π is the form's own indeterminate, [`INDET_PI`]); no value is
/// read. It is what the branch-stabilized azimuth leaves behind: on a
/// definitely-negative frame the chart's phase is `atan2(−y, −x) + π`,
/// and once the `atan2` has folded to the zero form the phase IS `π`.
/// Any other multiple (`π/3`), or a `π` beside anything else, stays an
/// atom.
///
/// **What this fold depends on.** `INDET_PI` is read as the TRUE π —
/// `cos π = −1` exactly — which is only right because the scalar's
/// `pi()` node is the interval `[fl(π), next_up(fl(π))]`, the 1-ulp
/// enclosure of the real π (`interval.rs`'s constants test pins that
/// `Interval::pi()` has exactly those bounds), so every value the
/// numeric channel ever computes for that node encloses the constant
/// the form folds at. If the node were ever built from the `f64`
/// literal `PI` alone the fold would be a theorem about a number the
/// value never was; `geom-core`'s
/// `tests/m10_10_atan2_interval::the_pi_fold_and_the_interval_pi_enclose_the_same_constant`
/// ties the two.
fn fold_at_half_pi(op: SymOp, arg: &Form) -> Option<Form> {
    if arg.poisoned {
        return None;
    }
    let den = arg.den.as_constant()?;
    if den.is_zero() || arg.num.terms.len() != 1 {
        return None;
    }
    let (mono, coeff) = arg.num.terms.iter().next()?;
    if mono.as_slice() != [(INDET_PI, 1)] {
        return None;
    }
    // `q = k/2`: a dyadic with at most one halving.
    let q = coeff.mul(&den.recip()?)?;
    if !q.den.is_one() || q.exp2 < -1 {
        return None;
    }
    let super::Int::Small(n) = q.num else {
        return None;
    };
    // Only `k mod 4` is read, for `k = n · 2^(e+1)` with `n` ODD (`Rat`
    // keeps its integer odd): `e ≥ 1` makes `k` a multiple of 4, `e =
    // 0` makes it `2 mod 4`, and `e = −1` leaves `n` itself. No shift
    // and no product is formed, so the wrap the multiple reader had
    // (`read_argument`) has no counterpart here — and never did, since
    // a wrapped shift preserves the low two bits.
    let k_mod_4 = match q.exp2 {
        _ if n == 0 => 0,
        e if e >= 1 => 0,
        0 => 2,
        _ => n.rem_euclid(4),
    };
    let (c, s) = match k_mod_4 {
        0 => (1, 0),
        1 => (0, 1),
        2 => (-1, 0),
        _ => (0, -1),
    };
    let v = match op {
        SymOp::Sin => s,
        _ => c,
    };
    let mut out = Form::poly(Poly::constant(Rat::new(v, 1, 0)?));
    out.gated = arg.gated;
    Some(out)
}

/// Mints (or finds) the `sqrt` atom over `arg`, recorded in the session
/// so rule A can look its argument back up, and answers its
/// indeterminate id. Payload zero, which is what every `sqrt` node the
/// scalar mints carries, so an expression that spells `sqrt(1 + X²)`
/// itself shares the atom.
fn sqrt_atom(arg: Form, sess: &mut Session) -> u128 {
    let id = indet_atom(SymOp::Sqrt.tag(), 0, &[arg.digest()]);
    sess.atoms.entry(id).or_insert_with(|| AtomInfo {
        op: SymOp::Sqrt,
        payload: 0,
        args: [Some(Rc::new(arg)), None],
    });
    id
}

/// `c · p` for a small integer `c`.
fn scaled(p: &Poly, c: i128, budget: SymBudget) -> Option<Poly> {
    Poly::constant(Rat::new(c, 1, 0)?).mul(p, budget)
}

/// **The fold**: the closed form of `sin`/`cos` (`op`) at the argument
/// form `arg`, or `None` where the argument is not `q · atan(X)` of a
/// foldable shape or the closed form does not fit the session's budget
/// (the caller then keeps the opaque atom — a missed cancellation, never
/// a wrong one).
///
/// # The representation, and why it is the whole of the cost
///
/// `cos ψ` and `sin ψ` are carried as `C / D` and `S / D` over ONE
/// shared denominator `D`, a monomial in the `sqrt` atoms, and the
/// angle-addition recurrence then stays polynomial: `cos((n+1)ψ) =
/// (Cₙ·C − Sₙ·S) / Dⁿ⁺¹`. Built as two independent quotient forms and
/// combined through the ring's own `add`, the two denominators are
/// cross-multiplied at every step because the normal form cancels no
/// common factor, and the seventh multiple of a quarter angle was past
/// the term budget before it was squared — which froze the very
/// `sin²(s·θ/2)` node the pushforward spells. The half-angle step keeps
/// the shape: with `cos(θ/2) = c₂` the atom, `cos(θ/2) = (1 + cos θ) /
/// (2·c₂)` (the half-angle identity `2·cos²(θ/2) = 1 + cos θ`, divided
/// through by the positive `c₂`) and `sin(θ/2) = sin θ / (2·c₂)`, so
/// both numerators stay polynomial and the denominator gains one atom
/// and a factor of two.
pub(super) fn fold(op: SymOp, arg: &Form, sess: &mut Session) -> Option<Form> {
    debug_assert!(matches!(op, SymOp::Sin | SymOp::Cos));
    if let Some(f) = fold_at_half_pi(op, arg) {
        return Some(f);
    }
    let closed = closed_forms(arg, sess)?;
    let mut out = Form::quotient(
        match op {
            SymOp::Sin => closed.sin.clone(),
            _ => closed.cos.clone(),
        },
        closed.den.clone(),
    );
    out.gated = closed.gated;
    // Each quotient is held to the budget on its own, exactly as the
    // two separate folds were.
    within(sess.budget, &out).then_some(out)
}

/// `(cos kψ, sin kψ)` over their shared denominator for one argument
/// form, built ONCE per session: the recurrence yields both at every
/// step, and `sin` and `cos` of one argument are two nodes that used
/// to run it twice (R2's Q7). A budget refusal is memoized too, so the
/// pair fails once.
pub(super) struct Closed {
    cos: Poly,
    sin: Poly,
    den: Poly,
    gated: bool,
}

fn closed_forms(arg: &Form, sess: &mut Session) -> Option<Rc<Closed>> {
    let key = arg.digest();
    if let Some(hit) = sess.trig_closed.get(&key) {
        return hit.clone();
    }
    let built = build_closed_forms(arg, sess).map(Rc::new);
    sess.trig_closed.insert(key, built.clone());
    built
}

fn build_closed_forms(arg: &Form, sess: &mut Session) -> Option<Closed> {
    let (k, m, x) = read_argument(arg, sess)?;
    let budget = sess.budget;
    let one = Form::poly(Poly::one());
    // φ = atan X, X = N / Dx: cos φ = Dx / (Dx·S), sin φ = N / (Dx·S)
    // with S = sqrt(1 + X²).
    let s = sqrt_atom(one.add(&x.mul(&x, budget)?, budget)?, sess);
    let mut den = x.den.mul(&Poly::indet(s), budget)?;
    let mut cn = x.den.clone();
    let sn = x.num.clone();
    // ψ = φ / 2ᵐ, each halving on the positive branch (module docs):
    // c₂ = sqrt((1 + cos θ) / 2) = sqrt((D + C) / (2·D)), then
    // cos(θ/2) = (D + C) / (2·c₂·D), sin(θ/2) = S / (2·c₂·D).
    for _ in 0..m {
        let lifted = den.add(&cn)?;
        let c2 = sqrt_atom(
            Form::quotient(lifted.clone(), scaled(&den, 2, budget)?),
            sess,
        );
        cn = lifted;
        den = scaled(&den.mul(&Poly::indet(c2), budget)?, 2, budget)?;
    }
    // k·ψ by angle addition from (cos 0, sin 0) = (1, 0), every
    // multiple over Dᵏ.
    let (mut ck, mut sk, mut dk) = (Poly::one(), Poly::zero(), Poly::one());
    for _ in 0..k.unsigned_abs() {
        let next_c = ck.mul(&cn, budget)?.add(&sk.mul(&sn, budget)?.neg()?)?;
        let next_s = sk.mul(&cn, budget)?.add(&ck.mul(&sn, budget)?)?;
        ck = next_c;
        sk = next_s;
        dk = dk.mul(&den, budget)?;
    }
    if k < 0 {
        sk = sk.neg()?;
    }
    Some(Closed {
        cos: ck,
        sin: sk,
        den: dk,
        gated: x.gated,
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::sym::{IdMap, IndetMap, SymBudget, SymRules};

    fn budget() -> SymBudget {
        SymBudget {
            max_terms: 4096,
            max_degree: 128,
        }
    }

    /// A bare session with the atoms `atan(x)` and `atan2(x, x)` in
    /// it, for the reader to look up.
    fn session_with(x: &Form, atan: u128, atan2: u128) -> Session {
        let mut sess = Session {
            budget: budget(),
            rules: SymRules::all(),
            nodes: IdMap::default(),
            forms: IdMap::default(),
            forms_early: IdMap::default(),
            forms_door: IdMap::default(),
            params: IndetMap::default(),
            atoms: IndetMap::default(),
            registry: IdMap::default(),
            trig_closed: IndetMap::default(),
            counts: Default::default(),
        };
        for (id, op) in [(atan, SymOp::Atan), (atan2, SymOp::Atan2)] {
            sess.atoms.insert(
                id,
                AtomInfo {
                    op,
                    payload: 0,
                    args: [Some(Rc::new(x.clone())), Some(Rc::new(x.clone()))],
                },
            );
        }
        sess
    }

    /// The argument reader takes exactly `q · atan(X)` with a dyadic
    /// `q` in range, and nothing else.
    #[test]
    fn the_argument_reader_is_exact() {
        let x = Form::poly(Poly::indet(7));
        let atan = indet_atom(SymOp::Atan.tag(), 0, &[x.digest()]);
        let atan2 = indet_atom(SymOp::Atan2.tag(), 0, &[x.digest(), x.digest()]);
        let sess = session_with(&x, atan, atan2);
        let scaled = |id: u128, c: Rat| {
            let mut p = Poly::zero();
            p.insert(vec![(id, 1)], c).unwrap();
            Form::poly(p)
        };
        let q = |n, d| Rat::new(n, d, 0).unwrap();
        let read = |f: &Form| read_argument(f, &sess).map(|(k, m, _)| (k, m));
        assert_eq!(read(&scaled(atan, q(3, 4))), Some((3, 2)), "3/4 · atan X");
        assert_eq!(read(&scaled(atan, q(2, 1))), Some((2, 0)), "2 · atan X");
        assert_eq!(read(&scaled(atan, q(-1, 2))), Some((-1, 1)), "−½ · atan X");
        // `atan(X) / 2` spelled as a quotient with a constant
        // denominator is the same argument.
        let over_two = scaled(atan, q(1, 1))
            .mul(
                &Form::poly(Poly::constant(q(2, 1))).recip().unwrap(),
                budget(),
            )
            .unwrap();
        assert_eq!(read(&over_two), Some((1, 1)), "atan(X) / 2");
        assert!(
            read(&scaled(atan, q(1, 3))).is_none(),
            "atan(X)/3 is not dyadic and never folds"
        );
        assert!(
            read(&scaled(atan2, q(1, 1))).is_none(),
            "an atan2 atom is not an atan atom"
        );
        let plus_c = scaled(atan, q(1, 2))
            .add(&Form::poly(Poly::one()), budget())
            .unwrap();
        assert!(
            read(&plus_c).is_none(),
            "½·atan(X) + 1 has two terms and never folds"
        );
        let squared = scaled(atan, q(1, 1))
            .mul(&scaled(atan, q(1, 1)), budget())
            .unwrap();
        assert!(
            read(&squared).is_none(),
            "atan(X)² is degree two and never folds"
        );
        let too_many = scaled(
            atan,
            Rat::new(1, 1, -(i32::try_from(MAX_HALVINGS).unwrap() + 1)).unwrap(),
        );
        assert!(read(&too_many).is_none(), "past MAX_HALVINGS nothing folds");
        let too_big = scaled(atan, q(MAX_MULTIPLE + 1, 1));
        assert!(read(&too_big).is_none(), "past MAX_MULTIPLE nothing folds");
    }
}
