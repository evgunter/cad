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
use geom_core::{Decide, ParamSymbol, Real, Sym, SymBudget, SymCounts, SymRules, Witness};

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

/// **MECHANISM 2 — rule F's sign amplification, at both INEXACT lane
/// scalars.** `E` is `1e-30·(1 + y²)` as a FORM, so rule F folds
/// `copysign(1, E)` to `1` and the residual is the zero form; at
/// `x ≈ 1e8` the point channel's first four terms are roundoff of
/// order one and come out negative, and the margin is a definite `−2`.
/// `E > 0` for every real `x`, `y`, so the tier's discharge is
/// CORRECT and the lift's answer is not: the lift is not an enclosure
/// and has no clause 1 to refuse with.
///
/// What rule F adds is the AMPLIFICATION — it is the first rule that
/// turns a one-ulp error in a SIGN argument into a whole `2.0` at the
/// margin, because `copysign`'s output is `±1` however small its
/// argument's error was. Any later rule that folds a sign decision
/// inherits it.
///
/// Measured at every sampled `x` and at both scalars, ε-independent
/// (this file's band is its own): the margin is `−2`, the tier keeps
/// the numeric answer, and the dispute is on the receipt. The same
/// residual at the certified lift is a plain theorem —
/// `sym_rule_f_interval_rows::the_adversary_at_the_interval_lift_is_a_plain_theorem`
/// is the twin, and the `Interval` twin of THIS count is
/// `sym11_witness_kind_interval_rows`.
///
/// Gating, and nothing panics: before SYM-11 this row fired the
/// contradiction `debug_assert!` at 6 of 6 points and had to be
/// `#[ignore]`d.
#[test]
fn sym11_the_adversary_is_a_counted_dispute_at_sym_f64() {
    for x0 in ADVERSARY_X {
        let m = margin_at(|| adversary::<f64>(x0));
        let (l, c, n) = decide_on_a_thread(move || adversary::<f64>(x0))
            .expect("the inexact witness never panics on a contradiction");
        println!("  x = {x0:e}: margin {m:e} | {l} | numeric sign {n} | {c:?}");
        assert_eq!(m, -2.0, "x = {x0:e}: the adversary's margin at the point");
        assert_eq!(
            l, "numeric Negative",
            "x = {x0:e}: the numeric answer is what a definite sign returns, and the \
             ratified numeric-first order is what returns it"
        );
        assert_eq!(
            c.theorems_disputed, 1,
            "x = {x0:e}: the form is a theorem and the value channel says −2, so the run \
             owes the receipt a dispute: {c:?}"
        );
        assert_eq!(
            c.symbolic_zero, 0,
            "x = {x0:e}: a dispute is NOT a discharge — the decision is `numeric`: {c:?}"
        );
    }
}

/// The same adversary at `Sym<Probe>`, whose value channel IS an
/// `f64`: the same rounding reaches the same door, and the K sample
/// the funnel records stays the numeric channel's own `Definite` at a
/// point — the dispute is visible on the receipt and not in the K
/// token vocabulary.
#[cfg(feature = "probe")]
#[test]
fn sym11_the_adversary_is_a_counted_dispute_at_sym_probe() {
    use geom_core::Probe;
    for x0 in ADVERSARY_X {
        let (l, c, n) = decide_on_a_thread(move || adversary::<Probe>(x0))
            .expect("the inexact witness never panics on a contradiction");
        println!("  x = {x0:e}: {l} | numeric sign {n} | {c:?}");
        assert_eq!(l, "numeric Negative", "x = {x0:e}");
        assert_eq!(c.theorems_disputed, 1, "x = {x0:e}: {c:?}");
    }
}

/// **THE POLE (R1's residue), both arms.** `copysign(1, 1/(t−1)²) − 1`
/// at `t = 1`: the point channel evaluates `1/0` to `+inf`, whose
/// `copysign` is `1`, so the margin is a finite `0` and the tier
/// answers the form's THEOREM on a box where the function has no value
/// at all. Nothing false is reported and nothing is disputed — the
/// numeric channel never answered a definite non-zero sign, so the
/// charge never arises.
///
/// It is the reason the inexact channel cannot simply be GIVEN the
/// form's answer either: at `t` clear of the pole the form is a
/// theorem and the value channel agrees, and AT the pole the point
/// channel has no clause 1 of its own to refuse with — only the
/// certified lift does
/// (`sym_rule_f_interval_rows::a_manifestly_positive_form_undefined_inside_the_box`,
/// where the box straddling the pole answers `refused Invalid`). So
/// what an inexact witness keeps is the NUMERIC answer, which is the
/// one thing that is true of this channel.
#[test]
fn sym11_the_pole_at_sym_f64_answers_the_form_and_disputes_nothing() {
    for (what, t0, want) in [
        ("at the pole", 1.0, "theorem"),
        ("clear of it", 0.4, "theorem"),
    ] {
        let (l, c, n) = decide_on_a_thread(move || pole::<f64>(t0))
            .expect("the pole does not panic at an inexact witness");
        println!("  [{what}] t = {t0}: {l} | numeric sign {n} | {c:?}");
        assert_eq!(l, want, "[{what}] t = {t0}");
        assert_eq!(
            c.theorems_disputed, 0,
            "[{what}] t = {t0}: the numeric channel answered no definite sign, so there is \
             nothing for the form to contradict: {c:?}"
        );
    }
}

/// **THE TWO CONTRACTS CANNOT DRIFT APART.** `Real::WITNESS` and
/// `Real::register_equal`'s refusal arm are the same claim about the
/// same channel, spelled twice, and this row is what holds them
/// together: an EXACT witness never answers `Disputed`, an INEXACT one
/// never answers `Contradicted`.
///
/// The roster is the lane scalars that implement `Real` at a point:
/// `Interval`, `Dual<Interval>`, `Sym<Interval>` and their inexact
/// twins are in `sym11_witness_kind_interval_rows`, which is the same
/// row over the certified half.
#[test]
fn sym11_the_witness_kind_and_the_refusal_arm_are_one_claim() {
    use geom_core::sym::SymRegistration;
    use geom_core::tolerance::Tol;
    let tol = Tol::witness();
    let inexact: [(&str, Witness, SymRegistration); 2] = [
        (
            "f64",
            <f64 as Real>::WITNESS,
            <f64 as Real>::register_equal(0.0, 1.0, tol),
        ),
        (
            "Sym<f64>",
            <Sym<f64> as Real>::WITNESS,
            with_session_rules(budget(), SymRules::shipped(), || {
                Sym::<f64>::from_f64(0.0).register_equal(Sym::<f64>::from_f64(1.0), tol)
            })
            .0,
        ),
    ];
    for (name, witness, arm) in inexact {
        println!("  {name}: {witness:?} refuses {arm:?}");
        assert_eq!(
            witness,
            Witness::Inexact,
            "{name} is in this row's INEXACT roster"
        );
        assert_eq!(
            arm,
            SymRegistration::Disputed,
            "{name} declares an INEXACT witness and refuses {arm:?} — an inexact witness \
             compares at a slack and cannot PROVE two values differ, so `Contradicted` is \
             not an arm it may answer"
        );
    }
}
