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

fn label(
    out: Result<Sign, geom_core::predicate::Indeterminate>,
    counts: geom_core::SymCounts,
) -> String {
    match out {
        Ok(Sign::Zero) if counts.symbolic_zero == 1 => "theorem".to_owned(),
        Ok(Sign::Zero) if counts.registered == 1 => "registered".to_owned(),
        Ok(Sign::Zero) if counts.sign_gated == 1 => "sign_gated".to_owned(),
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
/// column. The row above already asserts the label; this one says why
/// it is the assertion that matters, by taking the same residual with
/// rule C ON as well: the answer must still be `theorem`, because the
/// value-free rule is asked first.
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
