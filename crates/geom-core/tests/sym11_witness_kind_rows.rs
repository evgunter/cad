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
//! The EXACT half of every row here is `sym11_witness_kind_interval_rows`,
//! which shares this file's residual builders and its witness pin, and
//! the certified twin of the two residuals is `sym_rule_f_interval_rows`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::{Band, Margin, Sign};
use geom_core::sym::{SymRegistration, with_session_rules};
use geom_core::tolerance::Tol;
use geom_core::{Decide, ParamSymbol, Real, Sym, SymBudget, SymCounts, SymRules, Witness};

pub(crate) fn budget() -> SymBudget {
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

/// **R2's adversary, over parameters the CALLER built** — a point at
/// the inexact lanes, a box at the certified one, so the two halves of
/// this claim are one residual and not two copies of it.
///
/// `copysign(1, E) − 1` with
/// `E = (x + 1)² − x² − 2x − 1 + 1e-30·(1 + y²)`, which is the
/// polynomial `1e-30·(1 + y²)` as a FORM — manifestly positive, so
/// rule F folds the `copysign` to `1` and the residual is the zero
/// form. At `x ≈ 1e8` a point channel's first four terms are roundoff
/// of order one and come out NEGATIVE, and the margin is then a
/// definite `−2`.
pub(crate) fn adversary_of<T: Real>(x: Sym<T>, y: Sym<T>) -> Sym<T> {
    let one = || Sym::<T>::from_f64(1.0);
    let e = (x + one()).powi(2) - x.powi(2) - Sym::from_f64(2.0) * x - one()
        + Sym::from_f64(1.0e-30) * (one() + y.powi(2));
    one().copysign(e) - one()
}

/// The adversary at a POINT, which is the only shape an inexact lane
/// has.
fn adversary<T: Real>(x0: f64) -> Sym<T> {
    adversary_of(
        Sym::param(ParamSymbol::of("x"), T::from_f64(x0)),
        Sym::param(ParamSymbol::of("y"), T::from_f64(0.5)),
    )
}

/// **R1's residue, over a parameter the CALLER built**:
/// `copysign(1, 1/(t − 1)²) − 1`, whose argument is a positive constant
/// over a perfect square — manifestly positive to rule F, and with no
/// value at all at `t = 1`.
pub(crate) fn pole_of<T: Real>(t: Sym<T>) -> Sym<T> {
    let one = || Sym::<T>::from_f64(1.0);
    let x = one() / (t - one()).powi(2);
    one().copysign(x) - one()
}

/// The pole at a POINT. `1/0` is `+inf` at a point scalar and
/// `copysign(1, +inf)` is `1`, so the margin is a finite `0` where the
/// function has no value — the point channel has no clause 1 to refuse
/// the pole with, which is what the certified lane supplies there.
fn pole<T: Real>(t0: f64) -> Sym<T> {
    pole_of(Sym::param(ParamSymbol::of("t"), T::from_f64(t0)))
}

/// The margin AT THE POINT, read without asking for a decision: the
/// value channel's own number, which is what a fired assertion's
/// message cannot show.
fn margin_at<T: Real>(build: impl FnOnce() -> Sym<T>) -> T {
    let (v, _) = with_session_rules(budget(), SymRules::shipped(), || build().value);
    v
}

/// How the tier answered.
///
/// `> 0`, never `== 1`: a row that discharged TWICE would read
/// `numeric Zero` under an equality test and slip past every
/// `assert_ne!` that reads this label, which is the one direction it
/// may not fail in.
pub(crate) fn label(
    out: Result<Sign, geom_core::predicate::Indeterminate>,
    counts: SymCounts,
) -> String {
    match out {
        Ok(Sign::Zero) if counts.symbolic_zero > 0 => "theorem".to_owned(),
        Ok(Sign::Zero) if counts.registered > 0 => "registered".to_owned(),
        Ok(Sign::Zero) if counts.sign_gated > 0 => "sign_gated".to_owned(),
        Ok(s) => format!("numeric {s:?}"),
        Err(e) => format!("refused {:?}", e.margin),
    }
}

/// One decision under `rules`, ON ITS OWN THREAD: the label, the
/// session's counts and the numeric channel's own sign — or the panic's
/// own message, when the contradiction assertion fired. A panic inside
/// `with_session_rules` leaves that thread's session installed, and the
/// next case would refuse to nest
/// ([`test_utils::own_thread`], which also keeps the message).
fn decide_under<T>(
    rules: SymRules,
    build: impl FnOnce() -> Sym<T> + Send + 'static,
) -> Result<(String, SymCounts, f64), String>
where
    T: Real + Decide + Send + 'static,
{
    test_utils::own_thread::caught(move || {
        let ((out, probe), counts) = with_session_rules(budget(), rules, || {
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
    })
}

/// [`decide_under`] at the shipped set.
fn decide_on_a_thread<T>(
    build: impl FnOnce() -> Sym<T> + Send + 'static,
) -> Result<(String, SymCounts, f64), String>
where
    T: Real + Decide + Send + 'static,
{
    decide_under(SymRules::shipped(), build)
}

/// **THE TWO CONTRACTS, PINNED PER SCALAR AND NOT PER LITERAL.**
/// [`Real::WITNESS`] and [`Real::register_equal`]'s refusal arm are one
/// claim about one channel, spelled twice; this is the predicate that
/// holds them together, and every `impl Real` in the tree is passed
/// through it by NAME rather than by a hand-written row of literals.
///
/// A table of expected answers is the shape this pin failed in before:
/// a scalar whose const says `Inexact` while its `register_equal`
/// answers `Contradicted` is caught here only if someone remembered to
/// write it down, and both reviews planted exactly that and walked
/// through. Generic, a new `impl Real` is added to the LIST of
/// instantiations below and its answer is derived, so the only way past
/// is to not call this at all — which the roster rows' own count
/// catches.
///
/// **The pair is `0` against `1`**, which every scalar separates: at
/// `Interval` the two point enclosures are disjoint (a proof), at a
/// point scalar they are a whole unit apart against a slack of ε
/// relative to the larger magnitude (a measurement that cannot tell
/// which). It runs inside a session so that the `Sym` lifts reach their
/// registry, and nothing is recorded: the witness refuses first.
pub(crate) fn witness_agrees<T: Real>(name: &str) {
    let tol = Tol::witness();
    let (arm, _) = with_session_rules(budget(), SymRules::shipped(), || {
        T::from_f64(0.0).register_equal(T::from_f64(1.0), tol)
    });
    println!("  {name}: WITNESS {:?} refuses {arm:?}", T::WITNESS);
    let expected = match T::WITNESS {
        Witness::Exact => SymRegistration::Contradicted,
        Witness::Inexact => SymRegistration::Disputed,
    };
    assert_eq!(
        arm,
        expected,
        "{name} declares a {:?} witness and refuses a separated pair with {arm:?}. The two \
         are one claim about one channel: an EXACT witness PROVES two values differ and \
         answers `Contradicted`, an INEXACT one compared at a slack and can only answer \
         `Disputed` — and `Sym<T>::sign_within` charges a theorem-vs-numeric contradiction \
         by the same const, so a scalar that disagrees with itself here panics on a \
         contradiction it cannot prove, or counts one it can",
        T::WITNESS
    );
}

/// **The INEXACT roster**: every `impl Real` in the tree whose witness
/// is a point comparison, each named once. The interval half is
/// `sym11_witness_kind_interval_rows`, and `Probe`'s is the row below
/// this one, because that scalar is behind its own feature.
#[test]
fn sym11_every_inexact_scalar_agrees_with_its_own_refusal_arm() {
    witness_agrees::<f64>("f64");
    witness_agrees::<geom_core::Dual<f64>>("Dual<f64>");
    witness_agrees::<Sym<f64>>("Sym<f64>");
    witness_agrees::<Sym<geom_core::Dual<f64>>>("Sym<Dual<f64>>");
}

/// `Probe` and its `Sym` lift, behind the feature that mints them.
#[cfg(feature = "probe")]
#[test]
fn sym11_the_recording_scalar_agrees_with_its_own_refusal_arm() {
    witness_agrees::<geom_core::Probe>("Probe");
    witness_agrees::<Sym<geom_core::Probe>>("Sym<Probe>");
}

/// **MECHANISM 2 — rule F's sign amplification, at `Sym<f64>`.**
/// `E > 0` for every real `x`, `y`, so the tier's discharge is CORRECT
/// and the lift's answer is not: the lift is not an enclosure and has
/// no clause 1 to refuse with.
///
/// What rule F adds is the AMPLIFICATION — it is the first rule that
/// turns a one-ulp error in a SIGN argument into a whole `2.0` at the
/// margin, because `copysign`'s output is `±1` however small its
/// argument's error was. Any later rule that folds a sign decision
/// inherits it.
///
/// Measured at every sampled `x` and ε-independent (this file's band is
/// its own): the margin is `−2`, the tier keeps the numeric answer, and
/// the dispute is on the receipt.
///
/// Gating, and nothing panics: before SYM-11 this row fired the
/// contradiction `debug_assert!` at 6 of 6 points and had to be
/// `#[ignore]`d.
#[test]
fn sym11_the_adversary_is_a_counted_dispute_at_sym_f64() {
    for x0 in ADVERSARY_X {
        let m = margin_at(|| adversary::<f64>(x0));
        let (l, c, n) = decide_on_a_thread(move || adversary::<f64>(x0))
            .expect("an inexact witness never panics on a contradiction");
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
            c.numeric, 1,
            "x = {x0:e}: the decision itself is the numeric channel's, counted where every \
             undischarged decision is counted — a dispute is a second fact about that \
             decision and not a fourth column for one to land in: {c:?}"
        );
    }
}

/// The same adversary at `Sym<Probe>`, whose value channel IS an
/// `f64`: the same rounding reaches the same door, and the K sample the
/// funnel records stays the numeric channel's own `Definite` at a
/// point — the dispute is visible on the receipt and not in the K token
/// vocabulary.
#[cfg(feature = "probe")]
#[test]
fn sym11_the_adversary_is_a_counted_dispute_at_sym_probe() {
    use geom_core::Probe;
    for x0 in ADVERSARY_X {
        let (l, c, n) = decide_on_a_thread(move || adversary::<Probe>(x0))
            .expect("an inexact witness never panics on a contradiction");
        println!("  x = {x0:e}: {l} | numeric sign {n} | {c:?}");
        assert_eq!(l, "numeric Negative", "x = {x0:e}");
        assert_eq!(c.theorems_disputed, 1, "x = {x0:e}: {c:?}");
        assert_eq!(c.numeric, 1, "x = {x0:e}: {c:?}");
    }
}

/// **THE GATED KIND IS IN THE SAME COLUMN, AND HERE IT IS.** The charge
/// matches `Theorem | SignGated`, and
/// [`SymCounts::theorems_disputed`](geom_core::SymCounts::theorems_disputed)
/// counts both: at an inexact witness rule C's sign read came from the
/// SAME point channel the margin did, so a gated fold's premise is
/// exactly as suspect as a plain theorem's, and splitting the column
/// would report one suspicion as two kinds of fact.
///
/// The residual: `sqrt(((r + d) − d)²) − r` with `r` bracketed strictly
/// positive and `d = 1e9`. As a FORM `(r + d) − d` is `r`, so rule C
/// folds `sqrt(r²) → r` over the bracket and the residual is zero —
/// GATED, because the fold rests on the sign read. At `f64` the value
/// `r + d` rounds at an ulp of `1e9` (~1.2e-7), so what comes back out
/// of the subtraction is `r` plus that residue and the margin is the
/// residue itself: ~1e-7, a DEFINITE positive an order of magnitude
/// past this file's escalate threshold of `1e-8` — mechanism 1 at a
/// form mechanism 2 could not reach.
///
/// **Rule C is dial-OFF in the shipped set** (`SymRules::shipped`), so
/// no gated dispute can arise in a shipped run at all; this row drives
/// `SymRules::all()` to produce one, and asserts the shipped set's
/// silence beside it so that the dial is what the difference rests on.
#[test]
fn sym11_a_gated_theorem_disputes_into_the_same_column() {
    let resid = || {
        let r = Sym::<f64>::param_over(ParamSymbol::of("r"), 1.25e-3, 1.0e-3, 2.0e-3);
        let d = Sym::<f64>::param(ParamSymbol::of("d"), 1.0e9);
        (((r + d) - d) * ((r + d) - d)).sqrt() - r
    };
    let (l, c, n) = decide_under(SymRules::all(), resid)
        .expect("an inexact witness never panics on a contradiction");
    println!("  rule C on: {l} | numeric sign {n} | {c:?}");
    assert_eq!(
        l, "numeric Positive",
        "the margin at the point is the far placement's rounding residue, which the band \
         cannot absorb"
    );
    assert_eq!(
        c.theorems_disputed, 1,
        "a GATED zero under a definite numeric sign is the same contradiction the plain \
         kind is, and the same column says so: {c:?}"
    );
    assert_eq!(
        c.sign_gated, 0,
        "and it is not a discharge — the fold did not answer, the numeric channel did: {c:?}"
    );

    let (l, c, _) = decide_under(SymRules::shipped(), resid)
        .expect("an inexact witness never panics on a contradiction");
    println!("  shipped (rule C off): {l} | {c:?}");
    assert_eq!(
        c.theorems_disputed, 0,
        "with rule C off the `sqrt` atom stays opaque, the form is not zero, and there is \
         nothing to contradict — which is why no SHIPPED run can produce a gated \
         dispute: {c:?}"
    );
}

/// **THE POLE (R1's residue), both arms.** At `t = 1` the tier answers
/// the form's THEOREM on a box where the function has no value at all.
/// Nothing false is reported and nothing is disputed — the numeric
/// channel never answered a definite non-zero sign, so the charge never
/// arises.
///
/// It is the reason the inexact channel cannot simply be GIVEN the
/// form's answer either: at `t` clear of the pole the form is a theorem
/// and the value channel agrees, and AT the pole the point channel has
/// no clause 1 of its own to refuse with — only the certified lift does
/// (`sym_rule_f_interval_rows::a_manifestly_positive_form_undefined_inside_the_box`,
/// where the box straddling the pole answers `refused Invalid`). So
/// what an inexact witness keeps is the NUMERIC answer, which is the
/// one thing that is true of this channel.
#[test]
fn sym11_the_pole_at_sym_f64_answers_the_form_and_disputes_nothing() {
    for (what, t0) in [("at the pole", 1.0), ("clear of it", 0.4)] {
        let (l, c, n) = decide_on_a_thread(move || pole::<f64>(t0))
            .expect("the pole does not panic at an inexact witness");
        println!("  [{what}] t = {t0}: {l} | numeric sign {n} | {c:?}");
        assert_eq!(l, "theorem", "[{what}] t = {t0}");
        assert_eq!(
            c.theorems_disputed, 0,
            "[{what}] t = {t0}: the numeric channel answered no definite sign, so there is \
             nothing for the form to contradict: {c:?}"
        );
    }
}
