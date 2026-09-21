//! **The theorem-vs-numeric contradiction at an INEXACT witness** —
//! what `Sym<f64>` and `Sym<Probe>` do when the value channel's
//! definite sign and the form's theorem disagree.
//!
//! At `Sym<Interval>` a definite non-zero sign is a certified proof
//! that the margin is not zero, so a form that is the zero polynomial
//! contradicts it and one of the two channels is unsound. At a point
//! scalar it is a comparison of one rounded number: rule F turns a
//! one-ulp error in a SIGN argument into a whole `2.0` at the margin,
//! and at a pole the point channel has no clause 1 to refuse with.
//! Neither is a proof, so neither is an assertion's business.
//!
//! The interval twin of the two residuals below is
//! `sym_rule_f_interval_rows`, where the same forms are plain theorems.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::{Band, Margin, Sign};
use geom_core::sym::with_session_rules;
use geom_core::{Decide, ParamSymbol, Real, Sym, SymBudget, SymCounts, SymRules};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

fn band() -> Band {
    Band::new(1.0e-9, 1.0e-8).unwrap()
}

/// The six sampled `x` of the item's second mechanism.
const ADVERSARY_X: [f64; 6] = [1.0e8, 3.0e8, 5.0e8, 7.0e8, 1.0e9, 1.3e9];

/// R2's adversary at one point: `copysign(1, E) − 1` with
/// `E = (x + 1)² − x² − 2x − 1 + 1e-30·(1 + y²)`, which is the
/// polynomial `1e-30·(1 + y²)` as a FORM — manifestly positive, so
/// rule F folds the `copysign` to `1` and the residual is the zero
/// form. At `x ≈ 1e8` the point channel's first four terms are
/// roundoff of order one and come out NEGATIVE, and the margin is then
/// a definite `−2`.
fn adversary<T: Real>(x0: f64) -> Sym<T> {
    let one = || Sym::<T>::from_f64(1.0);
    let x = Sym::<T>::param(ParamSymbol::of("x"), T::from_f64(x0));
    let y = Sym::<T>::param(ParamSymbol::of("y"), T::from_f64(0.5));
    let e = (x + one()).powi(2) - x.powi(2) - Sym::from_f64(2.0) * x - one()
        + Sym::from_f64(1.0e-30) * (one() + y.powi(2));
    one().copysign(e) - one()
}

/// R1's residue: `copysign(1, 1/(t − 1)²) − 1` at the POLE `t = 1`.
/// `1/0` is `+inf` at a point scalar and `copysign(1, +inf)` is `1`, so
/// the margin is a finite `0` where the function has no value at all —
/// the point channel has no clause 1 to refuse the pole with, which is
/// what the certified lane's `refused Invalid` supplies there.
fn pole<T: Real>(t0: f64) -> Sym<T> {
    let one = || Sym::<T>::from_f64(1.0);
    let t = Sym::<T>::param(ParamSymbol::of("t"), T::from_f64(t0));
    let x = one() / (t - one()).powi(2);
    one().copysign(x) - one()
}

/// The margin AT THE POINT, read without asking for a decision: the
/// value channel's own number, which is what the assertion's own
/// message cannot show once it has fired.
fn margin_at<T: Real>(build: impl FnOnce() -> Sym<T>) -> T {
    let (v, _) = with_session_rules(budget(), SymRules::shipped(), || build().value);
    v
}

/// How the tier answered, by the same vocabulary `sym_rule_f_rows`
/// labels with.
fn label(out: Result<Sign, geom_core::predicate::Indeterminate>, counts: SymCounts) -> String {
    match out {
        Ok(Sign::Zero) if counts.symbolic_zero > 0 => "theorem".to_owned(),
        Ok(Sign::Zero) if counts.registered > 0 => "registered".to_owned(),
        Ok(Sign::Zero) if counts.sign_gated > 0 => "sign_gated".to_owned(),
        Ok(s) => format!("numeric {s:?}"),
        Err(e) => format!("refused {:?}", e.margin),
    }
}

/// One decision, on its OWN THREAD: the label, the session's counts and
/// the margin at the point — or `Err` when the contradiction assertion
/// fired. A panic inside `with_session_rules` leaves that thread's
/// session installed, and the next point would refuse to nest.
fn decide_on_a_thread<T>(
    build: impl FnOnce() -> Sym<T> + Send + 'static,
) -> Result<(String, SymCounts, f64), ()>
where
    T: Real + Decide + Send + 'static,
{
    std::thread::spawn(move || {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let ((out, probe), counts) = with_session_rules(budget(), SymRules::shipped(), || {
                let m = build();
                (
                    geom_core::k_stats::decide("sym11_witness_kind", Margin::of(m), band()),
                    m.value.sign_within(band()),
                )
            });
            let numeric = match probe {
                Ok(Sign::Positive) => 1.0,
                Ok(Sign::Negative) => -1.0,
                Ok(Sign::Zero) => 0.0,
                Err(_) => f64::NAN,
            };
            (label(out, counts), counts, numeric)
        }))
        .map_err(|_| ())
    })
    .join()
    .expect("the probe thread itself joins")
}

/// **MECHANISM 2 AT `Sym<f64>` — rule F's sign amplification.** The
/// adversary at all six sampled `x`, each point on its own thread.
///
/// Phase 1's measurement, at ε = default, 1e-6 and 1e-12 alike (the
/// residual's band is this file's own, so the run's ε does not enter):
/// the contradiction assertion fires at **6 of 6** points, with the
/// numeric channel definite NEGATIVE (margin `−2`) and the form's
/// discharge `Theorem` at every one.
///
/// `#[ignore]`d while the assertion is live: a row that panics on
/// purpose in every CI log is read as a break.
#[test]
#[ignore = "phase 1 evidence: fires Sym<f64>'s contradiction debug_assert by design"]
fn sym11_p1_the_adversary_at_sym_f64_trips_the_assertion() {
    let mut fired = 0;
    for x0 in ADVERSARY_X {
        let m = margin_at(|| adversary::<f64>(x0));
        let got = decide_on_a_thread(move || adversary::<f64>(x0));
        match got {
            Ok((l, c, n)) => {
                println!("  x = {x0:e}: margin {m:e} | {l} | numeric sign {n} | {c:?}")
            }
            Err(()) => {
                println!("  x = {x0:e}: margin {m:e} | the contradiction assertion FIRED");
                fired += 1;
            }
        }
    }
    println!(
        "  [Sym<f64>] the adversary fired at {fired} of {} points",
        ADVERSARY_X.len()
    );
}

/// **THE POLE AT `Sym<f64>` (R1's residue).** `copysign(1, 1/(t−1)²) − 1`
/// at `t = 1`: the point channel evaluates `1/0` to `+inf`, whose
/// `copysign` is `1`, so the margin is a finite `0` and the tier
/// answers the form's THEOREM — on a box where the function has no
/// value. Nothing false is reported and nothing panics; what the row
/// records is that the point channel has no clause 1 of its own, which
/// is why the numeric answer is what an inexact witness keeps.
///
/// Phase 1's measurement: `theorem`, numeric sign `0`.
#[test]
fn sym11_p1_the_pole_at_sym_f64_answers_the_form_where_the_function_is_undefined() {
    let got = decide_on_a_thread(|| pole::<f64>(1.0)).expect("the pole does not panic at f64");
    println!(
        "  [Sym<f64>] the pole at t = 1: {} | numeric sign {}",
        got.0, got.2
    );
    assert_eq!(got.0, "theorem");
}

/// **MECHANISM 2 AT `Sym<Probe>`.** The recording scalar's value
/// channel IS an `f64`, so the adversary reads the same way and the
/// assertion fires the same 6 of 6 times — the K sample the funnel
/// records is the numeric channel's own `Definite(Negative)`.
#[cfg(feature = "probe")]
#[test]
#[ignore = "phase 1 evidence: fires Sym<Probe>'s contradiction debug_assert by design"]
fn sym11_p1_the_adversary_at_sym_probe_trips_the_assertion() {
    use geom_core::Probe;
    let mut fired = 0;
    for x0 in ADVERSARY_X {
        let m = margin_at(|| adversary::<Probe>(x0));
        let got = decide_on_a_thread(move || adversary::<Probe>(x0));
        match got {
            Ok((l, c, n)) => {
                println!("  x = {x0:e}: margin {m:?} | {l} | numeric sign {n} | {c:?}")
            }
            Err(()) => {
                println!("  x = {x0:e}: margin {m:?} | the contradiction assertion FIRED");
                fired += 1;
            }
        }
    }
    println!(
        "  [Sym<Probe>] the adversary fired at {fired} of {} points",
        ADVERSARY_X.len()
    );
}
