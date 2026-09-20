//! **SYM-8 review R2 — probe rows at the scalar for rule F** (the
//! manifest sign, [`SymRules::manifest_sign`](geom_core::SymRules)).
//!
//! What these rows hunt, beyond `sym_rule_f_rows`:
//!
//! - whether the "rule-C-on" row there can pin the order F → C at all
//!   (rule C reads `Session::params`, which only `Sym::param_over`
//!   fills; a row built with `Sym::param` leaves rule C inert), and a
//!   residual rule C WOULD take, with a bracket, so the order is pinned
//!   by a row that reds when it flips;
//! - the adversary the brief asks for: a form the predicate calls
//!   positive whose VALUE channel yields a negative number at the point
//!   (catastrophic cancellation), so `copysign(1, X)` reads `−1` at
//!   `f64` while the tier says the identity is `+1`;
//! - the `D = 0` edge: `abs(1/sqrt(t²)) − 1/sqrt(t²)` over a box that
//!   contains `t = 0`, where the denominator vanishes — clause 1 must
//!   refuse before the fold can claim anything;
//! - the reach's asymmetry: a manifestly NEGATIVE argument
//!   (`−1/sqrt(1 + t²)`, the start cap's normal) is declined by both
//!   arms.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::{Band, Margin, Sign};
use geom_core::sym::with_session_rules;
use geom_core::{Interval, ParamSymbol, Real, Sym, SymBudget, SymRules};

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

/// A parameter WITH its bracket recorded in the session, which is the
/// only door rule C reads through.
fn p_over(name: &str, v: f64, lo: f64, hi: f64) -> Sym<f64> {
    Sym::param_over(ParamSymbol::of(name), v, lo, hi)
}

fn over(name: &str, lo: f64, hi: f64) -> Sym<Interval> {
    Sym::param_over(ParamSymbol::of(name), Interval::from_bounds(lo, hi), lo, hi)
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

fn how(rules: SymRules, build: impl FnOnce() -> Sym<f64>) -> (String, f64) {
    let ((out, value), counts) = with_session_rules(budget(), rules, || {
        let m = build();
        (
            geom_core::k_stats::decide("sym_rule_f_r2_probe", Margin::of(m), band()),
            m.value,
        )
    });
    (label(out, counts), value)
}

fn how_over(rules: SymRules, build: impl FnOnce() -> Sym<Interval>) -> String {
    let (out, counts) = with_session_rules(budget(), rules, || {
        geom_core::k_stats::decide("sym_rule_f_r2_probe_interval", Margin::of(build()), band())
    });
    label(out, counts)
}

fn with_c() -> SymRules {
    SymRules {
        signed_root: true,
        ..SymRules::shipped()
    }
}

/// **The order F → C is pinned only by a residual rule C would take.**
/// `sym_rule_f_rows`' rule-C-on row is `abs(1/sqrt(1 + t²))` built with
/// `Sym::param`: the session has no bracket, `signed::fold` declines on
/// `params.is_empty()` before it looks at anything, and the argument
/// carries a `sqrt` atom rule C could not enclose anyway. So that row
/// answers `theorem` under EITHER order. This row takes
/// `abs(1 + t²) − (1 + t²)` with `t` over `[0.2, 0.3]` and rule C on:
/// rule F folds it (a positive constant plus an even power), rule C
/// would fold it too (enclosable, certified positive) and would count it
/// `sign_gated`. Shipped order → `theorem`; planted C-before-F →
/// `sign_gated`. The first assertion is the pin; the second says the
/// PR's own row is NOT one.
#[test]
fn r2_the_order_against_rule_c_is_pinned_by_a_residual_rule_c_would_take() {
    let resid = || {
        let x = one() + p_over("t", 0.25, 0.2, 0.3).powi(2);
        x.abs() - x
    };
    let (l, v) = how(with_c(), resid);
    println!("  abs(1 + t²) − (1 + t²), t over [0.2, 0.3], rules C+F: {l} value {v:e}");
    assert_eq!(
        l, "theorem",
        "F before C: the value-free rule answers first"
    );

    // With rule F shut the same residual IS rule C's, which is what
    // makes it a discriminating row for the order.
    let (l_c, _) = how(
        SymRules {
            signed_root: true,
            ..SymRules::without_rule_f()
        },
        resid,
    );
    println!("  … with F shut and C on: {l_c}");
    assert_eq!(l_c, "sign_gated", "rule C alone takes this residual, gated");

    // The PR's rule-C-on row, re-taken with rule F SHUT: rule C does not
    // reach it at all, so that row cannot tell F-before-C from
    // C-before-F.
    let prs_row = || {
        let x = one() / (one() + p("t", 0.25).powi(2)).sqrt();
        x.abs() - x
    };
    let (l_prs, _) = how(
        SymRules {
            signed_root: true,
            ..SymRules::without_rule_f()
        },
        prs_row,
    );
    println!("  the PR's rule-C-on residual with F shut and C on: {l_prs}");
    assert_ne!(
        l_prs, "sign_gated",
        "if rule C reached the PR's residual the PR's row would be a real pin"
    );
}

/// **The adversary: a manifestly positive form whose value channel
/// reads negative at the point.** `E = (x + 1)² − x² − 2x − 1 +
/// 1e-30·(1 + y²)` is the polynomial `1e-30 + 1e-30·y²` as a form —
/// a positive constant plus a non-negative term, so rule F folds
/// `copysign(1, E)` to `1` — but at `x ≈ 1e8` the `f64` evaluation of
/// the first four terms is roundoff of order one and can be negative,
/// so the value channel's `copysign(1, E)` is `−1` there and the margin
/// `copysign(1, E) − 1` is a DEFINITE `−2`. The tier's discharge says
/// the same margin is identically zero — CORRECT as an identity of
/// reals (E > 0 everywhere) — and `Sym<f64>`'s `sign_within` then
/// fires its contradiction `debug_assert!` ("the numeric channel proved
/// this margin nonzero and the form says it is identically zero"). At
/// `f64` the value channel is not an enclosure, so the contradiction is
/// the channel's roundoff and not the rule's; what rule F adds is that a
/// one-ulp error in the SIGN argument becomes a whole `2.0` at the
/// margin. At `Sym<Interval>` the enclosure of `E` straddles zero, the
/// value channel cannot decide, and the tier answers `theorem`.
#[test]
fn r2_adversary_a_positive_form_whose_value_channel_reads_negative() {
    use std::panic::{AssertUnwindSafe, catch_unwind};
    let tiny = 1.0e-30;
    let mut contradicted = 0;
    let mut flipped = 0;
    for &x0 in &[1.0e8, 3.0e8, 5.0e8, 7.0e8, 1.0e9, 1.3e9] {
        let resid = move || {
            let x = p("x", x0);
            let y = p("y", 0.5);
            let e = (x + one()).powi(2) - x.powi(2) - Sym::from_f64(2.0) * x - one()
                + Sym::from_f64(tiny) * (one() + y.powi(2));
            one().copysign(e) - one()
        };
        let value = {
            let x = x0;
            let e = (x + 1.0).powi(2) - x.powi(2) - 2.0 * x - 1.0 + tiny * (1.0 + 0.25);
            1.0f64.copysign(e) - 1.0
        };
        // Each point on its own thread: a panic inside `with_session_rules`
        // leaves that thread's session installed, and the next point
        // would then refuse to nest.
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
                    "  x = {x0:e}: the f64 margin is {value:e} (definite) and the form is zero — \
                     Sym<f64>'s contradiction assertion FIRED"
                );
                contradicted += 1;
            }
        }
    }
    println!(
        "  contradiction assertion fired at {contradicted} of 6 points, value ≠ 0 at {flipped}"
    );
    assert!(
        contradicted + flipped > 0,
        "the adversary was meant to make the f64 channel disagree at least once"
    );
    // The same at `Sym<Interval>` over a box around one of those points.
    let l = how_over(SymRules::shipped(), || {
        let x = over("x", 1.0e8 - 1.0, 1.0e8 + 1.0);
        let y = over("y", 0.4, 0.6);
        let one = Sym::from_f64(1.0);
        let e = (x + one).powi(2) - x.powi(2) - Sym::from_f64(2.0) * x - one
            + Sym::from_f64(tiny) * (one + y.powi(2));
        one.copysign(e) - one
    });
    println!("  interval lift over x ∈ [1e8 ∓ 1]: {l}");
    assert_eq!(l, "theorem");
}

/// **The `D = 0` edge.** `abs(1/sqrt(t²)) − 1/sqrt(t²)`: the argument's
/// form is `1 / sqrt(t²)` — numerator a positive constant, denominator a
/// `sqrt` atom (non-negative, NOT positive: `t² = 0` at `t = 0`). The
/// predicate calls the FORM positive on the strength of clause 1
/// refusing every point where `D` vanishes. At `t = 0.25` it folds and
/// the identity holds; over a box that contains `t = 0` the value
/// channel divides by an interval containing zero, the margin is
/// `Invalid`, and the tier must NOT answer `theorem` there.
#[test]
fn r2_the_denominator_that_vanishes_is_refused_by_clause_1_not_folded() {
    let (l, v) = how(SymRules::shipped(), || {
        let x = one() / p("t", 0.25).powi(2).sqrt();
        x.abs() - x
    });
    println!("  abs(1/sqrt(t²)) − 1/sqrt(t²) at t = 0.25: {l} value {v:e}");
    assert_eq!(l, "theorem");
    assert!(v.abs() <= 1.0e-12);

    let l = how_over(SymRules::shipped(), || {
        let one = Sym::from_f64(1.0);
        let x = one / over("t", -0.1, 0.4).powi(2).sqrt();
        x.abs() - x
    });
    println!("  … over t ∈ [−0.1, 0.4] (D = 0 inside): {l}");
    assert_ne!(
        l, "theorem",
        "a box where the value channel divided by zero is clause 1's"
    );

    let l = how_over(SymRules::shipped(), || {
        let one = Sym::from_f64(1.0);
        let x = one / over("t", 0.2, 0.3).powi(2).sqrt();
        x.abs() - x
    });
    println!("  … over t ∈ [0.2, 0.3]: {l}");
    assert_eq!(l, "theorem");
}

/// **The reach is one-sided.** The start cap of the tilt-`u` cube
/// carries `n.z = −1/sqrt(P(t))`; `abs(−X) = X` and
/// `copysign(1, −X) = −1` are identities of reals for a manifestly
/// positive `X` exactly as the folded ones are, and the predicate
/// declines them (a negative coefficient is refused outright). Not a
/// soundness question — a document-class note: a `FaceFrame` on the
/// START cap of the tilt-`u` body is NOT reached by rule F.
#[test]
fn r2_a_manifestly_negative_argument_is_declined_by_both_arms() {
    let neg = || -(one() / (one() + p("t", 0.25).powi(2)).sqrt());
    let (l_abs, v) = how(SymRules::shipped(), || neg().abs() + neg());
    println!("  abs(−1/sqrt(1 + t²)) + 1/sqrt(1 + t²): {l_abs} value {v:e}");
    let (l_cs, v) = how(SymRules::shipped(), || one().copysign(neg()) + one());
    println!("  copysign(1, −1/sqrt(1 + t²)) + 1: {l_cs} value {v:e}");
    assert_ne!(
        l_abs, "theorem",
        "if this folds now, the predicate grew a negative branch"
    );
    assert_ne!(
        l_cs, "theorem",
        "if this folds now, the predicate grew a negative branch"
    );
}

/// **Shapes at the predicate's positive boundary that DO fold**, each
/// checked against the value at the point: a positive constant carrying
/// a `sqrt` atom over a bare square (`1 + sqrt(t²) > 0`); a `sqrt` atom
/// over a positive argument at an ODD power (`sqrt(1 + t²) > 0`); the
/// product of two positive atoms; and `copysign(Y, X)` with a `Y` of
/// unknown sign, whose magnitude is the `Abs` atom an `abs(Y)` node
/// mints (the same indeterminate, so the residual is the zero form).
#[test]
fn r2_the_positive_boundary_folds_and_every_fold_is_zero_at_the_point() {
    let t = || p("t", 0.25);
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
    let _ = t;
    for (what, build) in cases {
        let (l, v) = how(SymRules::shipped(), build);
        println!("  {what}: {l} value {v:e}");
        assert_eq!(l, "theorem", "{what}");
        assert!(v.abs() <= 1.0e-12, "{what}: theorem but value {v:e}");
        let (off, _) = how(SymRules::without_rule_f(), build);
        println!("    without_rule_f: {off}");
        assert_ne!(off, "theorem", "{what}: rule F is what takes it");
    }
}

/// **The differential constructors' contracts, after rule F.**
/// `SymRules::without_the_algebra` is documented as "the tier exactly
/// as M10-9 shipped it, bit for bit", `shipped_without_the_door` as
/// "M10-8's tier exactly" and `without_rule_e` as "M10-10's tier
/// exactly". None of those tiers had rule F, and every one of the three
/// is spelled `..Self::shipped()` with only its own dials shut, so on
/// this head each carries `manifest_sign: true`. This row asserts the
/// DOCUMENTED contract of the first; it reds on the frozen head, which
/// is the finding. (`m10_10_pins_interval`'s "five algebra dials and
/// nothing else" equality stays green only because both of its sides
/// carry rule F.)
#[test]
fn r2_the_algebra_off_differential_is_documented_as_m10_9s_tier() {
    let off = SymRules::without_the_algebra();
    println!(
        "  without_the_algebra().manifest_sign = {} | shipped_without_the_door() = {} | without_rule_e() = {}",
        off.manifest_sign,
        SymRules::shipped_without_the_door().manifest_sign,
        SymRules::without_rule_e().manifest_sign
    );
    assert!(
        !off.manifest_sign,
        "`without_the_algebra` is documented as M10-9's tier bit for bit; M10-9 had no rule F"
    );
}
