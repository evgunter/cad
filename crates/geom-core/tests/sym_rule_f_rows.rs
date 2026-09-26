//! **Rule F's rows at the scalar** — what the manifest sign
//! ([`SymRules::manifest_sign`](geom_core::SymRules)) folds, and every
//! shape it must never fold.
//!
//! The rule is `copysign(Y, X) → abs(Y)` and `abs(X) → X` wherever the
//! FORM of `X` is manifestly POSITIVE, and `−abs(Y)`, `−X` wherever it
//! is manifestly NEGATIVE, so the rows that matter are the ones at the
//! boundary of those predicates: a quantity the form shows positive or
//! negative (folds), and a quantity the form shows only NON-negative —
//! a sum of squares, a perfect square, a `sqrt` atom of a bare square —
//! or only non-positive (the same shapes negated), which may be a real
//! zero and must not (`copysign(1, +0.0)` and `copysign(1, −0.0)` are
//! different numbers, so a fold there would be a claim about a
//! spelling).
//!
//! **Rule F's ADVERSARY is not here.** `copysign(1, E) − 1` for
//! `E = (x + 1)² − x² − 2x − 1 + 1e-30·(1 + y²)` — the form rule F
//! calls manifestly positive whose `f64` channel reads negative at
//! `x ≈ 1e8` — is a row about what the DOOR does with a
//! theorem-vs-numeric contradiction, not about what the rule folds, so
//! it lives with the rest of that partition in
//! `sym11_witness_kind_rows` (both inexact lane scalars, gating) and
//! at the certified lift in `sym_rule_f_interval_rows`
//! (`the_adversary_at_the_interval_lift_is_a_plain_theorem`).
//!
//! Every row drives the tier through the same door a document does
//! (`k_stats::decide` over a `Margin`), and every THEOREM is checked
//! against the margin's own value at the point: a claim of "identically
//! zero" that is not numerically zero is the one failure this rule
//! could have, and it would be silent otherwise. This file's shape is
//! `sym_rule_e_rows`'.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::linalg::Vec3;
use geom_core::predicate::{Band, Margin, Sign};
use geom_core::sym::with_session_rules;
use geom_core::{ParamSymbol, Real, Sym, SymBudget, SymRules};

pub(crate) fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

pub(crate) fn band() -> Band {
    Band::new(1.0e-9, 1.0e-8).unwrap()
}

pub(crate) fn p(name: &str, v: f64) -> Sym<f64> {
    Sym::param(ParamSymbol::of(name), v)
}

pub(crate) fn one() -> Sym<f64> {
    Sym::from_f64(1.0)
}

pub(crate) fn label(
    out: Result<Sign, geom_core::predicate::Indeterminate>,
    counts: geom_core::SymCounts,
) -> String {
    // `> 0`, never `== 1`: a row that discharges TWICE would read
    // `numeric Zero` under an equality test and slip past every
    // `assert_ne!` in the file, which is the one direction a label may
    // not fail in (R1 S6).
    match out {
        Ok(Sign::Zero) if counts.symbolic_zero > 0 => "theorem".to_owned(),
        Ok(Sign::Zero) if counts.registered > 0 => "registered".to_owned(),
        Ok(Sign::Zero) if counts.sign_gated > 0 => "sign_gated".to_owned(),
        Ok(s) => format!("numeric {s:?}"),
        Err(e) => format!("refused {:?}", e.margin),
    }
}

/// How the tier answered, and the margin's value at the point.
pub(crate) fn how(rules: SymRules, build: impl FnOnce() -> Sym<f64>) -> (String, f64) {
    let ((out, value), counts) = with_session_rules(budget(), rules, || {
        let m = build();
        (
            geom_core::k_stats::decide("sym_rule_f_row", Margin::of(m), band()),
            m.value,
        )
    });
    (label(out, counts), value)
}

/// [`how`] with the session's counts beside the label and the value,
/// for a row that has to say what STOOD when a fold did not fire.
pub(crate) fn how_counted(
    rules: SymRules,
    build: impl FnOnce() -> Sym<f64>,
) -> (String, f64, geom_core::SymCounts) {
    let ((out, value), counts) = with_session_rules(budget(), rules, || {
        let m = build();
        (
            geom_core::k_stats::decide("sym_rule_f_row", Margin::of(m), band()),
            m.value,
        )
    });
    (label(out, counts), value, counts)
}

/// **The soundness check every row below shares**: a theorem must be
/// numerically zero at the point it was taken at.
///
/// **What it TRUSTS, and therefore cannot catch** (R2): the `f64` value
/// channel at that point. The `f64` lift is not an enclosure, so a
/// residual whose true real value is zero can evaluate non-zero under
/// cancellation — R2's adversary `E = (x+1)² − x² − 2x − 1 + 1e-30(1+y²)`
/// is exactly that shape — and this check would then call a TRUE
/// theorem unsound. It is a guard against the fold claiming zero for a
/// form that is plainly non-zero, not a proof of the identity;
/// `work/sym/sym-f64-far-placement-trips-the-theorem-vs-numeric-assert`
/// carries the class.
pub(crate) fn sound(what: &str, (l, v): (String, f64)) -> String {
    println!("  {what}: {l} value {v:e}");
    assert!(
        l != "theorem" || v.abs() <= 1.0e-12,
        "{what}: SAYS IDENTICALLY ZERO but the value at the point is {v:e} — UNSOUND"
    );
    l
}

/// `1/sqrt(1 + t²)` — an `Inv` of a `sqrt` atom whose argument is a
/// positive constant plus a square: the shape a re-normalised stored
/// unit vector's `z` component arrives in, and the one the predicate is
/// written for.
fn inv_sqrt_positive() -> Sym<f64> {
    one() / (one() + p("t", 0.25).powi(2)).sqrt()
}

/// **The theorem rows** — the simplest forms that reach the tilt-`u`
/// wall. `abs(X) − X` and `copysign(1, X) − 1` for a manifestly
/// POSITIVE `X`; both are theorems under the shipped set and neither is
/// reached with rule F shut, so the rows say which rule took them.
#[test]
fn the_manifest_sign_folds_an_inv_of_a_sqrt_atom() {
    println!("=== abs(1/sqrt(1 + t²)) − 1/sqrt(1 + t²)");
    let abs_resid = || inv_sqrt_positive().abs() - inv_sqrt_positive();
    assert_eq!(
        sound("shipped", how(SymRules::shipped(), abs_resid)),
        "theorem"
    );
    assert_ne!(
        sound("without_rule_f", how(SymRules::without_rule_f(), abs_resid)),
        "theorem",
        "with rule F shut the abs stays an opaque atom"
    );

    println!("=== copysign(1, 1/sqrt(1 + t²)) − 1");
    let cs_resid = || one().copysign(inv_sqrt_positive()) - one();
    assert_eq!(
        sound("shipped", how(SymRules::shipped(), cs_resid)),
        "theorem"
    );
    assert_ne!(
        sound("without_rule_f", how(SymRules::without_rule_f(), cs_resid)),
        "theorem",
        "with rule F shut the copysign stays an opaque atom"
    );
}

/// **A discharge through rule F is a THEOREM, never `sign_gated`** —
/// the rule reads no value, so it may not be counted in rule C's
/// column, and turning rule C on beside it does not change that.
///
/// **This row does NOT pin the F → C ORDER, and used to be cited as if
/// it did** (R1 and R2 both found it, R1 by planting C before F: all
/// four rows of this file passed). Rule C cannot reach this residual at
/// either order, for two independent reasons — `Sym::param` registers
/// no bracket, so `signed::fold` declines on an empty `Session::params`
/// before it looks at anything, and the argument carries a `sqrt` ATOM
/// its `enclosable` test refuses whatever the brackets are. What pins
/// the order is a residual BOTH rules take:
/// [`the_order_against_rule_c_is_pinned_by_a_residual_rule_c_would_take`]
/// and [`a_shape_both_rules_take_is_what_pins_the_order`], which are
/// the two rows the plant reds.
#[test]
fn the_manifest_sign_lands_in_symbolic_zero_and_not_sign_gated() {
    println!("=== abs(1/sqrt(1 + t²)) − 1/sqrt(1 + t²), with rule C on too");
    let with_c = SymRules {
        signed_root: true,
        ..SymRules::shipped()
    };
    assert_eq!(
        sound(
            "shipped + signed_root",
            how(with_c, || inv_sqrt_positive().abs() - inv_sqrt_positive())
        ),
        "theorem"
    );
}

/// **The shapes rule F must NOT fold.** Each is non-negative by its
/// syntax and can be a real ZERO, which is exactly where `copysign`
/// stops being a function of the real value of its argument; `abs`
/// would be sound at the zero and is held to the same predicate, for
/// the reason `manifest`'s header gives.
#[test]
fn the_shapes_the_manifest_sign_must_not_fold() {
    println!("=== shapes rule F must not fold, shipped set");
    let s = SymRules::shipped();

    // A SUM OF SQUARES is zero at the origin, and the sign bit of that
    // zero is the value channel's spelling, not a fact of the form.
    assert_ne!(
        sound(
            "copysign(1, x² + y²) − 1 at (3, 4)",
            how(s, || one()
                .copysign(p("x", 3.0).powi(2) + p("y", 4.0).powi(2))
                - one())
        ),
        "theorem",
        "a sum of squares can be zero"
    );
    assert_ne!(
        sound(
            "abs(x² + y²) − (x² + y²) at (3, 4)",
            how(s, || {
                let q = p("x", 3.0).powi(2) + p("y", 4.0).powi(2);
                q.abs() - q
            })
        ),
        "theorem",
        "the abs arm is held to the same predicate"
    );

    // A PERFECT SQUARE `(t − 1)²` is non-negative by `poly_sqrt` and
    // zero at `t = 1`: the branch the non-negativity predicate has and
    // the positivity predicate deliberately drops.
    assert_ne!(
        sound(
            "copysign(1, (t − 1)²) − 1 at t = 0.25",
            how(s, || {
                let d = p("t", 0.25) - one();
                one().copysign(d.powi(2)) - one()
            })
        ),
        "theorem",
        "a perfect square vanishes at the root of its root"
    );

    // A `sqrt` ATOM OF A BARE SQUARE — the R1 segment boss's
    // `abs((5/8)·sqrt(L²))`, the fold
    // `work/sym/coefficient-ring-width-is-not-monotone-in-reach`
    // measured LOSING ten decisions. Non-negative, not positive.
    assert_ne!(
        sound(
            "abs(sqrt(t²)) − sqrt(t²) at t = 0.25",
            how(s, || {
                let r = p("t", 0.25).powi(2).sqrt();
                r.abs() - r
            })
        ),
        "theorem",
        "sqrt of a bare square is zero wherever its argument is"
    );

    // A PARAMETER of unknown sign.
    assert_ne!(
        sound(
            "abs(t) − t at t = 0.25",
            how(s, || p("t", 0.25).abs() - p("t", 0.25))
        ),
        "theorem",
        "a parameter has no sign the form can read"
    );
    assert_ne!(
        sound(
            "copysign(1, t) − 1 at t = 0.25",
            how(s, || one().copysign(p("t", 0.25)) - one())
        ),
        "theorem",
        "a parameter has no sign the form can read"
    );

    // POISON: the zero vector's normalisation is 0/0 in every
    // component, and a function of an expression with no value has no
    // value either. Clause 1 is what refuses; the fold must not fire
    // over it.
    let poisoned = sound(
        "copysign(1, ‖0̂‖) − 1 (the zero vector)",
        how(s, || {
            let z = Vec3::new(p("a", 0.0), p("b", 0.0), p("c", 0.0))
                .normalize()
                .norm();
            one().copysign(z) - one()
        }),
    );
    assert_ne!(poisoned, "theorem", "poison folds nothing");
}

/// **The mint site, end to end.** `Vec3::orthonormal_basis` is where
/// the atoms come from (`s = 1.copysign(n.z)`, `r = 1/(1 + |n.z|)`),
/// and on a normal whose `z` is an `Inv` of a `sqrt` atom the rule
/// folds both. The residual is `b1 · n`, zero for every unit `n` by the
/// construction's own orthogonality. Measured, the tier reaches it at
/// BOTH dials — so this row does not discriminate the rule; what it
/// pins is that folding the basis's own atoms does not turn that
/// identity into a FALSE zero, which is the failure the shared value
/// check would catch.
#[test]
fn the_orthonormal_bases_own_atoms_fold_at_the_mint_site() {
    println!("=== b1 · n for n = (0, −t, 1)/sqrt(1 + t²)");
    let resid = || {
        let t = p("t", 0.25);
        let n = Vec3::new(Sym::from_f64(0.0), -t, one()).normalize();
        let (b1, _) = n.orthonormal_basis();
        b1.dot(n)
    };
    let shipped = sound("shipped", how(SymRules::shipped(), resid));
    let shut = sound("without_rule_f", how(SymRules::without_rule_f(), resid));
    println!("  shipped {shipped} | without_rule_f {shut}");
}

// ---------------------------------------------------------------- the
// rows the two blinded reviews wrote, adopted here under this suite's
// naming with their credit. The probe branches (`sym/8-review-r1`,
// `sym/8-review-r2`) are not merged; these are their rows.

/// A parameter WITH its bracket recorded in the session — the one door
/// rule C reads through ([`signed`](geom_core::sym)'s `fold` declines
/// outright when `Session::params` is empty).
fn p_over(name: &str, v: f64, lo: f64, hi: f64) -> Sym<f64> {
    Sym::param_over(ParamSymbol::of(name), v, lo, hi)
}

/// Rule C on beside the shipped set.
fn with_c() -> SymRules {
    SymRules {
        signed_root: true,
        ..SymRules::shipped()
    }
}

/// Rule C on, rule F SHUT.
fn c_not_f() -> SymRules {
    SymRules {
        signed_root: true,
        ..SymRules::without_rule_f()
    }
}

/// **THE ORDERING PIN (R2).** F is asked before C at the node, and this
/// is the row that reds when that flips: `abs(1 + t²) − (1 + t²)` with
/// `t` bracketed over `[0.2, 0.3]` is a residual BOTH rules take — rule
/// F because a positive constant plus an even power is manifestly
/// positive, rule C because the argument is enclosable and its sign
/// certifies. Shipped order the answer is a THEOREM; with rule F shut
/// the same residual is rule C's and the answer is `sign_gated`. So the
/// label distinguishes which rule took it, and planting C before F reds
/// the first assertion.
///
/// The row this suite used to name as the ordering pin
/// ([`the_manifest_sign_lands_in_symbolic_zero_and_not_sign_gated`])
/// cannot do it, and the third assertion here says why: rule C never
/// reaches that residual at any order.
#[test]
fn the_order_against_rule_c_is_pinned_by_a_residual_rule_c_would_take() {
    let resid = || {
        let x = one() + p_over("t", 0.25, 0.2, 0.3).powi(2);
        x.abs() - x
    };
    assert_eq!(
        sound("abs(1 + t²) − (1 + t²), C+F", how(with_c(), resid)),
        "theorem",
        "F before C: the value-free rule answers first"
    );
    assert_eq!(
        sound("… F shut, C on", how(c_not_f(), resid)),
        "sign_gated",
        "rule C alone takes this residual, and gates it"
    );
    let prs_row = || inv_sqrt_positive().abs() - inv_sqrt_positive();
    assert_ne!(
        sound(
            "the old ordering row, F shut and C on",
            how(c_not_f(), prs_row)
        ),
        "sign_gated",
        "rule C cannot reach that residual at any order, which is why it pins nothing"
    );
}

/// **THE SAME ORDER, THE OTHER SHAPE (R1).** `abs(2/t²)` over a bracket
/// that excludes zero: a positive constant numerator over an even
/// power, so rule F folds it, and enclosable with a certified sign, so
/// rule C would. Two rows rather than one because they fail differently
/// — this one has no atom in it at all, so it also says the order does
/// not depend on rule C's `enclosable` test declining.
#[test]
fn a_shape_both_rules_take_is_what_pins_the_order() {
    let resid = || {
        let x = Sym::from_f64(2.0) / p_over("t", 0.4, 0.3, 0.5).powi(2);
        x.abs() - x
    };
    assert_eq!(
        sound("abs(2/t²) − 2/t², shipped", how(SymRules::shipped(), resid)),
        "theorem",
        "rule F takes it with no value read"
    );
    assert_eq!(
        sound("abs(2/t²) − 2/t², C only", how(c_not_f(), resid)),
        "sign_gated",
        "rule C takes it by reading the bracket"
    );
    assert_eq!(
        sound("abs(2/t²) − 2/t², C+F", how(with_c(), resid)),
        "theorem",
        "with both on the value-free rule must be asked first"
    );
}

/// **THE EARLIER-TIER DIFFERENTIALS DO NOT CARRY RULE F (R2).**
/// `without_the_algebra` and `without_rule_e` are documented as
/// M10-9's and M10-10's tiers "bit for bit"; neither tier had rule F,
/// and both are spelled `..Self::shipped()`, so each acquired
/// `manifest_sign: true` the day rule F shipped and went on claiming
/// otherwise. This row reds on that head and is the pin that keeps the
/// next early-walk rule from doing it again.
///
/// `shipped_without_the_door` is deliberately NOT in this list. Its
/// live contract is "shipped minus the door and nothing else" —
/// `m10_9_pins_interval`'s census asserts exactly that, and every use
/// in the tree is a door differential — and its old "M10-8's tier
/// exactly" sentence was already false before rule F: it has carried
/// rules A/B and D since M10-10 and rule E since SYM-5. That SENTENCE
/// is the defect and SYM-8's fix pass retired it. M10-8's tier is
/// `a0_alone()` in `m10_8_pins_interval`.
#[test]
fn the_earlier_tier_differentials_shut_rule_f() {
    for (what, r) in [
        ("without_the_algebra", SymRules::without_the_algebra()),
        ("without_rule_e", SymRules::without_rule_e()),
    ] {
        println!("  {what}.manifest_sign = {}", r.manifest_sign);
        assert!(
            !r.manifest_sign,
            "{what} names a tier that had no rule F; carrying one makes it a differential \
             against a tier that never existed"
        );
    }
    assert!(
        SymRules::without_rule_f().common_factor,
        "the rule-F differential keeps rule E on: it is the pair's other half"
    );
    assert!(
        SymRules::shipped_without_the_door().manifest_sign,
        "the DOOR differential shuts the door and nothing else: shutting a fold rule \
         with it would measure two things at once"
    );
}

/// **THE ONE REACHABLE ZERO OF A POSITIVE FORM SPELLS ITSELF `+0.0`
/// (R1, addendum 2).** The header's signed-zero paragraph says a
/// manifestly positive form whose value channel yields `−0.0` would be
/// a failure to enclose and not a break of this rule. The reachable
/// case is UNDERFLOW: `1/(1 + t²)` at `t = 1e200` overflows its
/// denominator to `+inf` and the quotient to zero. This row asserts
/// that zero's SIGN BIT is the one the fold assumes.
#[test]
fn the_underflowed_positive_form_spells_its_zero_positive() {
    let x = || one() / (one() + p("t", 1.0e200).powi(2));
    let v = x().value;
    println!(
        "  1/(1 + t²) at t = 1e200 = {v:e}, sign_negative {}",
        v.is_sign_negative()
    );
    assert!(v == 0.0, "the value underflows");
    assert!(
        !v.is_sign_negative(),
        "a manifestly positive form underflowed to −0.0: copysign would take the other branch"
    );
    sound(
        "copysign(1, X) − 1 at the underflow",
        how(SymRules::shipped(), || one().copysign(x()) - one()),
    );
}

/// **The negative arm's reflection of the row above.** The one
/// reachable zero of a manifestly NEGATIVE form is the same underflow
/// with the sign carried: `−1/(1 + t²)` at `t = 1e200` is `−0.0`, whose
/// sign BIT is the one the negative arm's fold assumes — and the fold
/// reads no value, so `copysign(1, X) + 1` is a theorem there
/// regardless.
#[test]
fn the_underflowed_negative_form_spells_its_zero_negative() {
    let x = || -(one() / (one() + p("t", 1.0e200).powi(2)));
    let v = x().value;
    println!(
        "  −1/(1 + t²) at t = 1e200 = {v:e}, sign_negative {}",
        v.is_sign_negative()
    );
    assert!(v == 0.0, "the value underflows");
    assert!(
        v.is_sign_negative(),
        "the underflowed negative form spells its zero −0.0, which is the sign bit the fold assumes"
    );
    assert_eq!(
        sound(
            "copysign(1, −1/(1 + t²)) + 1 at t = 1e200",
            how(SymRules::shipped(), || one().copysign(x()) + one())
        ),
        "theorem"
    );
}

/// **THE PREDICATE'S POSITIVE BOUNDARY (R2, addendum 4).** Four shapes
/// that DO fold, each checked against the value at the point and each
/// NOT a theorem with rule F shut: a positive constant carrying a
/// `sqrt` atom over a bare square; a `sqrt` atom over a positive
/// argument at an odd power; a product of two positive atoms; and
/// `copysign(Y, X)` with a `Y` of unknown sign, whose magnitude is the
/// `Abs` atom an `abs(Y)` node mints.
#[test]
fn the_positive_boundary_folds_and_every_fold_is_zero_at_the_point() {
    /// One named residual builder, so the array below is a list of
    /// cases rather than a nest of function types.
    type Case = (&'static str, fn() -> Sym<f64>);
    let cases: [Case; 4] = [
        ("abs(1 + sqrt(t²)) − (1 + sqrt(t²))", || {
            let x = Sym::from_f64(1.0) + p("t", 0.25).powi(2).sqrt();
            x.abs() - x
        }),
        ("abs(sqrt(1 + t²)) − sqrt(1 + t²)", || {
            let x = (Sym::from_f64(1.0) + p("t", 0.25).powi(2)).sqrt();
            x.abs() - x
        }),
        ("abs(sqrt(1 + t²)·sqrt(4 + t⁴)) − the product", || {
            let a = (Sym::from_f64(1.0) + p("t", 0.25).powi(2)).sqrt();
            let b = (Sym::from_f64(4.0) + p("t", 0.25).powi(4)).sqrt();
            (a * b).abs() - a * b
        }),
        ("copysign(t, 1/sqrt(1 + t²)) − abs(t)", || {
            let s = Sym::from_f64(1.0) / (Sym::from_f64(1.0) + p("t", 0.25).powi(2)).sqrt();
            p("t", 0.25).copysign(s) - p("t", 0.25).abs()
        }),
    ];
    for (what, build) in cases {
        assert_eq!(
            sound(what, how(SymRules::shipped(), build)),
            "theorem",
            "{what}"
        );
        assert_ne!(
            sound(
                &format!("{what} [F shut]"),
                how(SymRules::without_rule_f(), build)
            ),
            "theorem",
            "{what}: rule F is what takes it"
        );
    }
}

/// **THE NEGATIVE ARM'S THEOREM ROWS (SYM-12).** The START cap of the
/// tilt-`u` cube carries `n.z = −1/sqrt(P(t))`, and `abs(−X) = X` and
/// `copysign(1, −X) = −1` are identities of reals for a manifestly
/// positive `X` exactly as the folded ones are. SYM-8's positive arm
/// declined them (a negative coefficient was refused outright) and this
/// row, written by its review R2, used to pin that decline; the arm
/// SYM-12 measured and took folds them, and the row now says which
/// rule does: a theorem under the shipped set, not reached with rule F
/// shut.
#[test]
fn the_negative_arm_folds_the_start_caps_atoms() {
    let neg = || -(one() / (one() + p("t", 0.25).powi(2)).sqrt());
    println!("=== abs(−1/sqrt(1 + t²)) + 1/sqrt(1 + t²)");
    let abs_resid = || neg().abs() + neg();
    assert_eq!(
        sound("shipped", how(SymRules::shipped(), abs_resid)),
        "theorem",
        "abs(X) = −X for a manifestly negative X"
    );
    assert_ne!(
        sound("without_rule_f", how(SymRules::without_rule_f(), abs_resid)),
        "theorem",
        "with rule F shut the abs stays an opaque atom"
    );
    println!("=== copysign(1, −1/sqrt(1 + t²)) + 1");
    let cs_resid = || one().copysign(neg()) + one();
    assert_eq!(
        sound("shipped", how(SymRules::shipped(), cs_resid)),
        "theorem",
        "copysign(1, X) = −1 for a manifestly negative X"
    );
    // With rule F shut what stands is said in full, not only "not a
    // theorem": the atom is minted (no symbolic discharge of any
    // kind), the VALUE channel is what answers, and the value it reads
    // at the point is exactly zero — so the shipped theorem above is
    // the fold's and not the value channel's.
    let (shut_label, shut_value, shut_counts) = how_counted(SymRules::without_rule_f(), cs_resid);
    println!("  without_rule_f: {shut_label} value {shut_value:e} counts {shut_counts:?}");
    assert_eq!(shut_label, "numeric Zero", "the value channel answers");
    assert_eq!(shut_counts.symbolic_zero, 0, "no theorem was reached");
    assert_eq!(shut_counts.sign_gated, 0, "and no value was read by a rule");
    assert_eq!(shut_counts.registered, 0, "and no axiom was stated");
    assert!(shut_counts.numeric >= 1, "the numeric channel decided it");
    assert!(shut_value == 0.0, "−1 + 1 is exactly zero at the point");
    // And a NON-constant magnitude: `copysign(Y, X) = −|Y|` mints the
    // `Abs` atom over `Y` negated, the same indeterminate `abs(Y)`
    // mints, so `copysign(t, X) + abs(t)` is a theorem too.
    println!("=== copysign(t, −1/sqrt(1 + t²)) + abs(t)");
    assert_eq!(
        sound(
            "shipped",
            how(SymRules::shipped(), || {
                p("t", 0.25).copysign(neg()) + p("t", 0.25).abs()
            })
        ),
        "theorem",
        "the negated magnitude is the same Abs atom an abs node mints"
    );
}

/// **THE TWO SPELLINGS OF A NEGATIVE MAGNITUDE MEET.** A
/// `copysign(Y, X)` node's `|Y|` is `manifest::magnitude`'s, an
/// `abs(Y)` node's is `fold_abs`'s, and for a manifestly NEGATIVE `Y`
/// the two parted when the negative arm was first cut into `fold_abs`
/// alone: `abs(Y)` folded to `−Y` while `magnitude` still minted the
/// `Abs` atom, so `copysign(Y, X) − abs(Y)` — a theorem before the arm,
/// both spellings one atom — read `numeric` on a residual the plain
/// walk cannot close. `magnitude` now reads `fold_abs` first, and this
/// row pins that: the bare pair, the arm's own identity
/// `copysign(Y, X) + Y`, and the pair through a factor only rule E's
/// `Q/Q → 1` clears, all theorems at the shipped set; the positive `Y`
/// beside them, which never parted.
#[test]
fn the_two_spellings_of_a_negative_magnitude_meet() {
    let y = || -(one() + p("t", 0.25).powi(2));
    let x = || one() + p("s", 0.5).powi(2);
    assert_eq!(
        sound(
            "copysign(Y, X) − abs(Y), Y = −(1 + t²), X = 1 + s²",
            how(SymRules::shipped(), || y().copysign(x()) - y().abs())
        ),
        "theorem"
    );
    assert_eq!(
        sound(
            "copysign(Y, X) + Y — the arm's own identity",
            how(SymRules::shipped(), || y().copysign(x()) + y())
        ),
        "theorem"
    );
    let q = || one() + p("t", 0.25).powi(2);
    let through_e = || {
        let unit = (q() / q()).sqrt();
        y().copysign(x()) * unit - y().abs()
    };
    assert_eq!(
        sound(
            "copysign(Y, X)·sqrt(Q/Q) − abs(Y): the pair the plain walk cannot close",
            how(SymRules::shipped(), through_e)
        ),
        "theorem",
        "the two spellings of |Y| for a manifestly negative Y must fold to one form"
    );
    assert_ne!(
        sound(
            "… with rule F shut",
            how(SymRules::without_rule_f(), through_e)
        ),
        "theorem",
        "and it is rule F that closes it"
    );
    let yp = || one() + p("t", 0.25).powi(2);
    assert_eq!(
        sound(
            "the same shape with Y = 1 + t² (the positive arm's, unchanged)",
            how(SymRules::shipped(), || {
                let unit = (q() / q()).sqrt();
                yp().copysign(x()) * unit - yp().abs()
            })
        ),
        "theorem"
    );
}

/// **The shapes the NEGATIVE arm must NOT fold** — the positive arm's
/// negatives reflected: each is non-positive by its syntax and can be a
/// real ZERO, or carries a term whose sign the syntax cannot read, and
/// the arm must decline it exactly as the positive arm declines its
/// mirror image.
///
/// By construction this row is every `assert_ne!` and cannot tell the
/// arm EXISTS — it is green with `negative` returning `false` — so it
/// is read together with [`the_negative_arm_folds_the_start_caps_atoms`],
/// the existence pin, which reds on that plant.
#[test]
fn the_shapes_the_negative_arm_must_not_fold() {
    println!("=== shapes the negative arm must not fold, shipped set");
    let s = SymRules::shipped();

    // A NEGATED SUM OF SQUARES is zero at the origin.
    assert_ne!(
        sound(
            "copysign(1, −(x² + y²)) + 1 at (3, 4)",
            how(s, || one()
                .copysign(-(p("x", 3.0).powi(2) + p("y", 4.0).powi(2)))
                + one())
        ),
        "theorem",
        "a negated sum of squares can be zero"
    );
    assert_ne!(
        sound(
            "abs(−(x² + y²)) − (x² + y²) at (3, 4)",
            how(s, || {
                let q = p("x", 3.0).powi(2) + p("y", 4.0).powi(2);
                (-q).abs() - q
            })
        ),
        "theorem",
        "the abs arm is held to the same predicate"
    );

    // A NEGATED PERFECT SQUARE `−(t − 1)²` is zero at `t = 1`.
    assert_ne!(
        sound(
            "copysign(1, −(t − 1)²) + 1 at t = 0.25",
            how(s, || {
                let d = p("t", 0.25) - one();
                one().copysign(-d.powi(2)) + one()
            })
        ),
        "theorem",
        "a negated perfect square vanishes at the root of its root"
    );

    // A NON-NEGATIVE TERM beside the negative ones: `t² − 1/sqrt(1 + t²)`
    // is negative at the point and has no sign the syntax can read.
    assert_ne!(
        sound(
            "copysign(1, t² − 1/sqrt(1 + t²)) + 1 at t = 0.25",
            how(s, || {
                let x = p("t", 0.25).powi(2) - inv_sqrt_positive();
                one().copysign(x) + one()
            })
        ),
        "theorem",
        "a form with a non-negative term beside its negative ones has no manifest sign"
    );

    // A NEGATED PARAMETER of unknown sign.
    assert_ne!(
        sound(
            "abs(−t) − t at t = 0.25",
            how(s, || (-p("t", 0.25)).abs() - p("t", 0.25))
        ),
        "theorem",
        "a negated parameter has no sign the form can read"
    );
    assert_ne!(
        sound(
            "copysign(1, −t) + 1 at t = 0.25",
            how(s, || one().copysign(-p("t", 0.25)) + one())
        ),
        "theorem",
        "a negated parameter has no sign the form can read"
    );

    // A NEGATED `sqrt` atom of a bare square — the ring row's atom,
    // reflected — under both spellings.
    assert_ne!(
        sound(
            "abs(−sqrt(t²)) − sqrt(t²) at t = 0.25",
            how(s, || {
                let r = p("t", 0.25).powi(2).sqrt();
                (-r).abs() - r
            })
        ),
        "theorem",
        "a negated sqrt of a bare square is zero wherever its argument is"
    );
    assert_ne!(
        sound(
            "copysign(1, −sqrt(t²)) + 1 at t = 0.25",
            how(s, || one().copysign(-p("t", 0.25).powi(2).sqrt()) + one())
        ),
        "theorem",
        "the copysign spelling is held to the same predicate"
    );

    // POISON, negated: `−‖0̂‖` is a function of an expression with no
    // value, and `negative`'s poison guard is what keeps the arm off
    // it — this is the row that reds if that guard goes.
    let poisoned = sound(
        "copysign(1, −‖0̂‖) + 1 (the zero vector, negated)",
        how(s, || {
            let z = Vec3::new(p("a", 0.0), p("b", 0.0), p("c", 0.0))
                .normalize()
                .norm();
            one().copysign(-z) + one()
        }),
    );
    assert_ne!(poisoned, "theorem", "poison folds nothing, negated or not");
}

/// **THE NEGATIVE ARM'S ORDER AGAINST RULE C is pinned the same way**
/// as the positive arm's, by residuals BOTH rules take: `abs(−(1 + t²))`
/// and `abs(−2/t²)` over brackets that exclude zero. Rule F folds each
/// because the argument is manifestly negative (a negative constant
/// times an even power, or over one); rule C would fold each because
/// the argument is enclosable with a certified NEGATIVE sign (`abs(R) →
/// −R`). Shipped order, a THEOREM; with rule F shut, rule C's
/// `sign_gated`; so planting C before F reds both, as it reds the
/// positive arm's two rows above.
#[test]
fn the_negative_arms_order_against_rule_c_is_pinned_the_same_way() {
    let resid_a = || {
        let x = -(one() + p_over("t", 0.25, 0.2, 0.3).powi(2));
        x.abs() + x
    };
    assert_eq!(
        sound("abs(−(1 + t²)) − (1 + t²), C+F", how(with_c(), resid_a)),
        "theorem",
        "F before C: the value-free rule answers first"
    );
    assert_eq!(
        sound("… F shut, C on", how(c_not_f(), resid_a)),
        "sign_gated",
        "rule C alone takes this residual, and gates it"
    );
    let resid_b = || {
        let x = -(Sym::from_f64(2.0) / p_over("t", 0.4, 0.3, 0.5).powi(2));
        x.abs() + x
    };
    assert_eq!(
        sound("abs(−2/t²) − 2/t², C+F", how(with_c(), resid_b)),
        "theorem",
        "with both on the value-free rule must be asked first"
    );
    assert_eq!(
        sound("abs(−2/t²) − 2/t², C only", how(c_not_f(), resid_b)),
        "sign_gated",
        "rule C takes it by reading the bracket"
    );
}

/// **THE `copysign` MINT SITES THE TREE HOLDS, counted by the source
/// and not by prose.** `manifest.rs`'s header lists the sites outside
/// the tier that mint a `copysign` atom; a list in a doc-comment is a
/// claim, and SYM-12's own first cut of it was wrong twice (a site
/// dropped, four never named). This row is the register: every
/// `.copysign(` call in the shipped sources of every crate under
/// `crates/`, outside `geom-core/src/sym/` (the tier, where the atom is
/// consumed) and outside the files that DEFINE `fn copysign(` (the
/// scalar impls, which forward it), read over
/// [`test_utils::source::code_only`] so prose and literals do not
/// count and with each file cut at its in-file `#[cfg(test)]` module,
/// since a test's own `copysign` mints nothing shipped. The expected
/// table is per file with its count; a site that appears or goes reds
/// here, and the fix is to re-ask the census on the measured
/// documents and then move BOTH the table below and the header's
/// list. The shape is `flagged_census`'s (the `decide_flagged` site
/// register), which is this repo's precedent for a source census.
#[test]
fn the_copysign_mint_sites_the_tree_holds_are_these() {
    let root = test_utils::source::repo_root(env!("CARGO_MANIFEST_DIR"));
    let crates = root.join("crates");
    let mut found: std::collections::BTreeMap<String, usize> = Default::default();
    for entry in std::fs::read_dir(&crates).expect("the crates directory lists") {
        let src = entry.expect("a directory entry").path().join("src");
        if !src.is_dir() {
            continue;
        }
        for path in test_utils::source::rust_sources(&src) {
            let rel = path
                .strip_prefix(&root)
                .expect("a source under the root")
                .to_string_lossy()
                .replace('\\', "/");
            if rel.starts_with("crates/geom-core/src/sym/") || rel == "crates/geom-core/src/sym.rs"
            {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("a source file reads");
            let code = test_utils::source::code_only(&text);
            // The shipped part of the file: everything above its
            // in-file test module.
            let shipped = code
                .lines()
                .position(|l| l.trim() == "#[cfg(test)]")
                .map_or(code.as_str(), |n| {
                    let at: usize = code.lines().take(n).map(|l| l.len() + 1).sum();
                    &code[..at]
                });
            if shipped.contains("fn copysign(") {
                continue;
            }
            let n = shipped.matches(".copysign(").count();
            if n > 0 {
                found.insert(rel, n);
            }
        }
    }
    let expected: std::collections::BTreeMap<String, usize> = [
        ("crates/geom-brep/src/implicit.rs", 1),
        ("crates/geom-brep/src/props/curved.rs", 1),
        ("crates/geom-brep/src/tangent.rs", 1),
        ("crates/geom-core/src/linalg/svd.rs", 1),
        ("crates/geom-core/src/linalg/vec.rs", 1),
        ("crates/profile/src/path.rs", 1),
        ("crates/profile/src/sugar.rs", 2),
        ("crates/sweep/src/blend/arms.rs", 1),
        ("crates/sweep/src/revolve/axis.rs", 1),
        ("crates/topo/src/boolean/solid_contain.rs", 2),
    ]
    .into_iter()
    .map(|(f, n)| (f.to_owned(), n))
    .collect();
    for (f, n) in &found {
        println!("  {f}: {n}");
    }
    assert_eq!(
        found, expected,
        "the copysign mint sites moved: a site appeared or went. Re-ask the census \
         (`m10_10_evidence_interval`'s `sym12_the_copysign_census_at_the_nominal`) on the \
         measured documents, then move this table and `manifest.rs`'s header list together."
    );
}
