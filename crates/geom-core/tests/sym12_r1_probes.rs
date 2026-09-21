//! **SYM-12 review R1's own scalar-door probe** — forms the unit did
//! not measure, driven at the INTERVAL lift against the manifest-sign
//! rule's NEGATIVE arm. Evidence-only: every row prints what the tier
//! decided and the counts it decided it with, and the rows that must
//! decline assert that they did.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::Margin;
use geom_core::sym::with_session_rules;
use geom_core::{Interval, ParamSymbol, Real, Sym, SymRules};

use crate::sym_rule_f_rows::{budget, label};

fn one_i() -> Sym<Interval> {
    Sym::from_f64(1.0)
}
fn k(v: f64) -> Sym<Interval> {
    Sym::from_f64(v)
}
fn over(name: &str, lo: f64, hi: f64) -> Sym<Interval> {
    Sym::param_over(ParamSymbol::of(name), Interval::from_bounds(lo, hi), lo, hi)
}

/// The answer, the enclosure, and the three non-theorem discharge
/// counts — `sign_gated` is the one that says a VALUE was read.
fn drive(what: &str, rules: SymRules, build: impl FnOnce() -> Sym<Interval>) -> String {
    let band = geom_core::predicate::Band::linear(geom_core::Tol::witness())
        .expect("the witness tolerance has a linear band");
    let ((out, value), counts) = with_session_rules(budget(), rules, || {
        let m = build();
        (
            geom_core::k_stats::decide("sym12_r1_probe", Margin::of(m), band),
            m.value,
        )
    });
    let l = label(out, counts);
    println!(
        "  {what}: {l} | enclosure {:?} | sym0 {} gated {} reg {} numeric {} frozen {}",
        value,
        counts.symbolic_zero,
        counts.sign_gated,
        counts.registered,
        counts.numeric,
        counts.frozen
    );
    l
}

fn t() -> Sym<Interval> {
    over("t", 0.2, 0.3)
}
fn s() -> Sym<Interval> {
    over("s", 0.2, 0.3)
}

#[test]
fn r1_the_negative_arms_strictness_on_forms_the_unit_did_not_measure() {
    let sh = SymRules::shipped();
    println!("=== folds (manifestly negative)");
    // abs(−(1 + t²)) − (1 + t²): the brief's theorem.
    assert_eq!(
        drive("abs(−(1 + t²)) − (1 + t²)", sh, || {
            let x = -(one_i() + t().powi(2));
            x.abs() - (one_i() + t().powi(2))
        }),
        "theorem"
    );
    // −(1 + t²)/(1 + s²): a manifestly non-negative denominator.
    assert_eq!(
        drive(
            "abs(−(1 + t²)/(1 + s²)) − (1 + t²)/(1 + s²)",
            sh,
            || {
                let d = one_i() + s().powi(2);
                let x = -((one_i() + t().powi(2)) / d.clone());
                x.abs() - (one_i() + t().powi(2)) / d
            }
        ),
        "theorem"
    );
    // −(1 + t²)/s²: a denominator that CAN be zero, on a box that
    // excludes it — `quotient`'s four-source argument, reflected.
    assert_eq!(
        drive(
            "abs(−(1 + t²)/s²) − (1 + t²)/s², s ∈ [0.2, 0.3]",
            sh,
            || {
                let d = s().powi(2);
                let x = -((one_i() + t().powi(2)) / d.clone());
                x.abs() - (one_i() + t().powi(2)) / d
            }
        ),
        "theorem"
    );
    // The same over a box that HOLDS s = 0: clause 1's, not the fold's.
    let holds_zero = drive(
        "… the same over s ∈ [−0.1, 0.3] (D = 0 inside)",
        sh,
        || {
            let d = over("s", -0.1, 0.3).powi(2);
            let x = -((one_i() + t().powi(2)) / d.clone());
            x.abs() - (one_i() + t().powi(2)) / d
        },
    );
    assert_ne!(
        holds_zero, "theorem",
        "a box the value channel divided by zero on is clause 1's"
    );

    println!("=== declines (NOT manifestly negative)");
    for (what, l) in [
        (
            "abs(−t²) − t² (a negated square: zero at t = 0)",
            drive("abs(−t²) − t²", sh, || {
                let x = -t().powi(2);
                x.abs() - t().powi(2)
            }),
        ),
        (
            "abs(−(t² + s²)) − (t² + s²) (zero at the origin)",
            drive("abs(−(t² + s²)) − (t² + s²)", sh, || {
                let q = t().powi(2) + s().powi(2);
                (-q.clone()).abs() - q
            }),
        ),
        (
            "abs(−|t|) − |t| (no strictly negative term)",
            drive("abs(−abs(t)) − abs(t)", sh, || {
                let a = t().abs();
                (-a.clone()).abs() - a
            }),
        ),
        (
            "abs(−2 + t²) + (−2 + t²) (a negative constant beside a positive term)",
            drive("abs(−2 + t²) + (−2 + t²)", sh, || {
                let x = k(-2.0) + t().powi(2);
                x.abs() + x
            }),
        ),
    ] {
        assert_ne!(l, "theorem", "{what} must not fold");
    }

    println!("=== copysign");
    assert_eq!(
        drive("copysign(1, −(1 + t²)) + 1", sh, || {
            one_i().copysign(-(one_i() + t().powi(2))) + one_i()
        }),
        "theorem"
    );
    assert_eq!(
        drive("copysign(s, −sqrt(1 + t²)) + abs(s)", sh, || {
            s().copysign(-(one_i() + t().powi(2)).sqrt()) + s().abs()
        }),
        "theorem"
    );
    println!(
        "  copysign(0, −(1 + t²)) − 0 = {}",
        drive("copysign(0, −(1 + t²)) − 0", sh, || {
            k(0.0).copysign(-(one_i() + t().powi(2))) - k(0.0)
        })
    );
    // The sign bit the fold claims, read against the f64 door.
    println!(
        "  f64: copysign(0.0, -1.0) = {} (bit {})",
        0.0f64.copysign(-1.0),
        0.0f64.copysign(-1.0).is_sign_negative()
    );
}

/// The same forms with rule F SHUT: every fold above must stop being a
/// theorem, so the rows above say which rule took them.
#[test]
fn r1_every_fold_above_is_rule_fs_and_no_pair_reads_a_value() {
    let no_f = SymRules::without_rule_f();
    println!("=== rule F shut");
    for l in [
        drive("abs(−(1 + t²)) − (1 + t²)", no_f, || {
            let x = -(one_i() + t().powi(2));
            x.abs() - (one_i() + t().powi(2))
        }),
        drive("copysign(1, −(1 + t²)) + 1", no_f, || {
            one_i().copysign(-(one_i() + t().powi(2))) + one_i()
        }),
        drive("copysign(s, −sqrt(1 + t²)) + abs(s)", no_f, || {
            s().copysign(-(one_i() + t().powi(2)).sqrt()) + s().abs()
        }),
    ] {
        assert_ne!(l, "theorem", "with rule F shut nothing here is a theorem");
    }
}

/// **The two rows SYM-8's reviews forced for the POSITIVE arm, and
/// SYM-12 did not reflect.** (a) R1's addendum 2: the one reachable
/// zero of a positive form spells itself `+0.0` (underflow), which the
/// fold's sign-bit assumption rests on — the negative arm's reflection
/// is that the one reachable zero of a NEGATIVE form spells itself
/// `−0.0`. (b) R2's adversary `E`: a form the predicate calls
/// positive whose `f64` channel reads negative. Its reflection is a
/// form the predicate calls NEGATIVE whose `f64` channel reads
/// positive. Neither reflected row is in the diff; this probe takes
/// both.
#[test]
fn r1_the_positive_arms_two_forced_rows_reflected() {
    println!("=== the negative form that UNDERFLOWS");
    let huge = 1.0e200;
    let x = -1.0f64 / (1.0 + huge * huge);
    println!(
        "  f64: −1/(1 + t²) at t = 1e200 = {x:e}, sign_negative {}",
        x.is_sign_negative()
    );
    assert_eq!(
        drive(
            "copysign(1, −1/(1 + t²)) + 1 at t = 1e200",
            SymRules::shipped(),
            || {
                let t = over("t", huge, huge);
                one_i().copysign(-(one_i() / (one_i() + t.powi(2)))) + one_i()
            }
        ),
        "theorem"
    );
    println!("=== R2's adversary, REFLECTED (manifestly negative, f64 reads positive)");
    let tiny = 1.0e-30;
    let l = drive(
        "copysign(1, E′) + 1 over x ∈ [1e8 ∓ 1]",
        SymRules::shipped(),
        || {
            let x = over("x", 1.0e8 - 1.0, 1.0e8 + 1.0);
            let y = over("y", 0.4, 0.6);
            let e = (x + one_i()).powi(2)
                - x.powi(2)
                - k(2.0) * x
                - one_i()
                - k(tiny) * (one_i() + y.powi(2));
            one_i().copysign(e) + one_i()
        },
    );
    println!("  reflected adversary at the interval lift: {l}");
}
