//! **Rule F's rows at the scalar** — what the manifest sign
//! ([`SymRules::manifest_sign`](geom_core::SymRules)) folds, and every
//! shape it must never fold.
//!
//! The rule is `copysign(Y, X) → abs(Y)` and `abs(X) → X` wherever the
//! FORM of `X` is manifestly POSITIVE, so the rows that matter are the
//! ones at the boundary of that predicate: a quantity the form shows
//! positive (folds), and a quantity the form shows only NON-negative —
//! a sum of squares, a perfect square, a `sqrt` atom of a bare square —
//! which may be a real zero and must not (`copysign(1, +0.0)` and
//! `copysign(1, −0.0)` are different numbers, so a fold there would be
//! a claim about a spelling).
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

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

fn band() -> Band {
    Band::new(1.0e-9, 1.0e-8).unwrap()
}

fn p(name: &str, v: f64) -> Sym<f64> {
    Sym::param(ParamSymbol::of(name), v)
}

fn one() -> Sym<f64> {
    Sym::from_f64(1.0)
}

#[cfg(feature = "interval")]
fn one_i() -> Sym<geom_core::Interval> {
    Sym::from_f64(1.0)
}

/// A bracketed parameter at the INTERVAL lift, for the rows that need a
/// box rather than a point (clause 1 answers on a box).
#[cfg(feature = "interval")]
fn over(name: &str, lo: f64, hi: f64) -> Sym<geom_core::Interval> {
    Sym::param_over(
        ParamSymbol::of(name),
        geom_core::Interval::from_bounds(lo, hi),
        lo,
        hi,
    )
}

/// [`how`] at the interval lift: the answer, and the margin's
/// enclosure.
#[cfg(feature = "interval")]
fn how_i(
    rules: SymRules,
    build: impl FnOnce() -> Sym<geom_core::Interval>,
) -> (String, geom_core::Interval) {
    let band = geom_core::predicate::Band::linear(geom_core::Tol::witness())
        .expect("the witness tolerance has a linear band");
    let ((out, value), counts) = with_session_rules(budget(), rules, || {
        let m = build();
        (
            geom_core::k_stats::decide("sym_rule_f_row", Margin::of(m), band),
            m.value,
        )
    });
    (label(out, counts), value)
}

fn label(
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
fn how(rules: SymRules, build: impl FnOnce() -> Sym<f64>) -> (String, f64) {
    let ((out, value), counts) = with_session_rules(budget(), rules, || {
        let m = build();
        (
            geom_core::k_stats::decide("sym_rule_f_row", Margin::of(m), band()),
            m.value,
        )
    });
    (label(out, counts), value)
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
fn sound(what: &str, (l, v): (String, f64)) -> String {
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

/// **THE THREE DIFFERENTIAL CONSTRUCTORS DO NOT CARRY RULE F (R2).**
/// `without_the_algebra`, `shipped_without_the_door` and
/// `without_rule_e` are documented as M10-9's, M10-8's and M10-10's
/// tiers "bit for bit"; none of those tiers had rule F, and all three
/// are spelled `..Self::shipped()`, so each acquired `manifest_sign:
/// true` the day rule F shipped. This row reds on that head and is the
/// pin that keeps the next early-walk rule from doing it again.
#[test]
fn the_earlier_tier_differentials_shut_rule_f() {
    for (what, r) in [
        ("without_the_algebra", SymRules::without_the_algebra()),
        (
            "shipped_without_the_door",
            SymRules::shipped_without_the_door(),
        ),
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
}

/// **THE `D = 0` EDGE (R2, addendum 1).** `abs(1/sqrt(t²)) − 1/sqrt(t²)`
/// — the argument's numerator is a positive constant and its
/// denominator a `sqrt` atom, which is non-negative and NOT positive
/// (`t² = 0` at `t = 0`). The predicate calls the FORM positive anyway,
/// on the strength of `quotient`'s side condition: `D ≠ 0` at every
/// point of a box clause 1 admits. This row is that condition's pin —
/// the fold at a point clear of zero, and no theorem over a box that
/// holds `t = 0`, where the value channel divided by an interval
/// containing zero.
#[test]
#[cfg(feature = "interval")]
fn the_denominator_that_vanishes_is_refused_by_clause_1_not_folded() {
    assert_eq!(
        sound(
            "abs(1/sqrt(t²)) − 1/sqrt(t²) at t = 0.25",
            how(SymRules::shipped(), || {
                let x = one() / p("t", 0.25).powi(2).sqrt();
                x.abs() - x
            })
        ),
        "theorem"
    );
    let over_box = |lo: f64, hi: f64| {
        how_i(SymRules::shipped(), move || {
            let x = one_i() / over("t", lo, hi).powi(2).sqrt();
            x.abs() - x
        })
    };
    let (holds_zero, _) = over_box(-0.1, 0.4);
    println!("  … over t ∈ [−0.1, 0.4] (D = 0 inside): {holds_zero}");
    assert_ne!(
        holds_zero, "theorem",
        "a box the value channel divided by zero on is clause 1's, not the fold's"
    );
    let (clear, _) = over_box(0.2, 0.3);
    println!("  … over t ∈ [0.2, 0.3]: {clear}");
    assert_eq!(clear, "theorem");
}

/// **A MANIFESTLY POSITIVE FORM UNDEFINED INSIDE THE BOX (R1, item G).**
/// `1/(t − 1)²` is a positive constant over a PERFECT SQUARE, which the
/// predicate accepts for a DENOMINATOR, so `manifest::positive` says
/// yes — and at `t = 1` the form has no value at all. The header says
/// clause 1 refuses there first; this row drives a box that contains
/// the pole and one clear of it, under both arms.
#[test]
#[cfg(feature = "interval")]
fn a_manifestly_positive_form_undefined_inside_the_box() {
    for (name, lo, hi, want_theorem) in [
        ("straddles the pole", 0.9, 1.1, false),
        ("clear of it", 0.2, 0.4, true),
    ] {
        for what in ["abs", "copysign"] {
            let (l, v) = how_i(SymRules::shipped(), || {
                let x = one_i() / (over("t", lo, hi) - one_i()).powi(2);
                if what == "abs" {
                    x.abs() - x
                } else {
                    one_i().copysign(x) - one_i()
                }
            });
            println!("  [{name}] {what}: {l} enclosure {v:?}");
            assert_eq!(
                l == "theorem",
                want_theorem,
                "[{name}] {what}: a box holding the pole must be clause 1's"
            );
        }
    }
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

/// **THE HAND-MINTED MAGNITUDE IS THE ATOM AN `abs` NODE MINTS (R1,
/// addendum 3).** `magnitude` folds `copysign(Y, X)` to the `Abs` ATOM
/// over `Y` when `Y` is neither constant nor manifestly non-negative,
/// and the whole value of doing so is that the atom is the SAME
/// indeterminate an `abs(Y)` node elsewhere in the DAG mints. With a
/// compound `Y` that claim is what makes this residual the zero form,
/// and it is the pin of the `mint_atom` door the fold now goes through.
#[test]
#[cfg(feature = "interval")]
fn the_minted_magnitude_is_the_same_indeterminate_an_abs_node_mints() {
    let resid = || {
        let y = over("x", -0.6, 0.6) - over("z", 0.1, 0.2);
        y.copysign(one_i()) - y.abs()
    };
    let (on, _) = how_i(SymRules::shipped(), resid);
    let (off, _) = how_i(SymRules::without_rule_f(), resid);
    println!("  copysign(x − z, 1) − |x − z|: F-on {on} | F-off {off}");
    assert_eq!(on, "theorem", "the minted atom must be the abs node's atom");
    assert_ne!(off, "theorem", "rule F is what takes it");
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
    let cases: [(&str, fn() -> Sym<f64>); 4] = [
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

/// **THE REACH IS ONE-SIDED (R2, addendum 5).** The START cap of the
/// tilt-`u` cube carries `n.z = −1/sqrt(P(t))`, and `abs(−X) = X` and
/// `copysign(1, −X) = −1` are identities of reals for a manifestly
/// positive `X` exactly as the folded ones are — the predicate declines
/// them, because a negative coefficient is refused outright. Not a
/// soundness question, a document-class one: a `FaceFrame` on the start
/// cap of that body is NOT reached by rule F, and a manifest-NEGATIVE
/// arm is the next shape rather than one this unit took.
#[test]
fn a_manifestly_negative_argument_is_declined_by_both_arms() {
    let neg = || -(one() / (one() + p("t", 0.25).powi(2)).sqrt());
    assert_ne!(
        sound(
            "abs(−1/sqrt(1 + t²)) + 1/sqrt(1 + t²)",
            how(SymRules::shipped(), || { neg().abs() + neg() })
        ),
        "theorem",
        "if this folds now, the predicate grew a negative branch"
    );
    assert_ne!(
        sound(
            "copysign(1, −1/sqrt(1 + t²)) + 1",
            how(SymRules::shipped(), || { one().copysign(neg()) + one() })
        ),
        "theorem",
        "if this folds now, the predicate grew a negative branch"
    );
}

/// **THE ADVERSARY: a manifestly positive form whose `f64` channel
/// reads NEGATIVE at the point (R2, item G).**
/// `E = (x + 1)² − x² − 2x − 1 + 1e-30·(1 + y²)` is the polynomial
/// `1e-30 + 1e-30·y²` as a FORM — a positive constant plus a
/// non-negative term — so rule F folds `copysign(1, E)` to `1`. At
/// `x ≈ 1e8` the `f64` evaluation of the first four terms is roundoff
/// of order one and can be negative, so the value channel's
/// `copysign(1, E)` is `−1` there and the margin `copysign(1, E) − 1`
/// is a DEFINITE `−2` while the tier says the same margin is
/// identically zero.
///
/// **That is not an unsoundness of rule F**: `E > 0` for every real
/// `x`, `y`, and the `f64` lift is not an enclosure, so the
/// disagreement is the channel's roundoff. What rule F adds is that a
/// one-ulp error in the SIGN argument becomes a whole `2.0` at the
/// margin — it is the first rule that turns one into the other. At
/// `Sym<Interval>` the enclosure of `E` contains zero, the value
/// channel cannot decide, and the tier answers `theorem`.
///
/// `#[ignore]`d because `Sym<f64>::sign_within`'s contradiction
/// `debug_assert!` FIRES here by design, and a row that panics on
/// purpose in every CI log is read as a break. The class is
/// `work/sym/sym-f64-far-placement-trips-the-theorem-vs-numeric-assert`;
/// changing that assertion is SYM's row and not this unit's.
#[test]
#[ignore = "evidence-only: fires Sym<f64>'s contradiction debug_assert by design (R2's adversary)"]
fn the_adversary_a_positive_form_whose_f64_channel_reads_negative() {
    use std::panic::{AssertUnwindSafe, catch_unwind};
    let tiny = 1.0e-30;
    let (mut contradicted, mut flipped) = (0, 0);
    for &x0 in &[1.0e8, 3.0e8, 5.0e8, 7.0e8, 1.0e9, 1.3e9] {
        let resid = move || {
            let x = p("x", x0);
            let y = p("y", 0.5);
            let e = (x + one()).powi(2) - x.powi(2) - Sym::from_f64(2.0) * x - one()
                + Sym::from_f64(tiny) * (one() + y.powi(2));
            one().copysign(e) - one()
        };
        // Each point on its own thread: a panic inside
        // `with_session_rules` leaves that thread's session installed
        // and the next point would refuse to nest.
        let outcome = std::thread::spawn(move || {
            catch_unwind(AssertUnwindSafe(|| how(SymRules::shipped(), resid)))
        })
        .join()
        .expect("the probe thread itself joins");
        match outcome {
            Ok((l, v)) => {
                println!("  x = {x0:e}: copysign(1, E) − 1 → {l}, value {v:e}");
                assert_eq!(l, "theorem", "E is manifestly positive");
                if v != 0.0 {
                    flipped += 1;
                }
            }
            Err(_) => {
                println!(
                    "  x = {x0:e}: the f64 margin is definite and the form is zero — \
                          Sym<f64>'s contradiction assertion FIRED"
                );
                contradicted += 1;
            }
        }
    }
    println!("  contradiction fired at {contradicted} of 6 points, value ≠ 0 at {flipped}");
    assert!(
        contradicted + flipped > 0,
        "the adversary is meant to make the f64 channel disagree at least once"
    );
}

/// The same adversary at the INTERVAL lift, which is the certified
/// lane: `E`'s enclosure over a box around `x = 1e8` straddles zero,
/// the value channel cannot decide, and the tier answers `theorem` —
/// the identity, correctly. Gating, because nothing here panics.
#[test]
#[cfg(feature = "interval")]
fn the_adversary_at_the_interval_lift_is_a_plain_theorem() {
    let tiny = 1.0e-30;
    let (l, v) = how_i(SymRules::shipped(), || {
        let x = over("x", 1.0e8 - 1.0, 1.0e8 + 1.0);
        let y = over("y", 0.4, 0.6);
        let e = (x + one_i()).powi(2) - x.powi(2) - Sym::from_f64(2.0) * x - one_i()
            + Sym::from_f64(tiny) * (one_i() + y.powi(2));
        one_i().copysign(e) - one_i()
    });
    println!("  interval lift over x ∈ [1e8 ∓ 1]: {l} enclosure {v:?}");
    assert_eq!(l, "theorem");
}
