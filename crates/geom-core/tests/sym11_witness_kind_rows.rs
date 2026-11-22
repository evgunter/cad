//! **The theorem-vs-numeric contradiction at an INEXACT witness** —
//! what `Sym<f64>` and `Sym<Probe>` do when the value channel's
//! definite sign and the form's theorem disagree, and what the test
//! instrument (`geom_core::sym::contradiction`) reports of it.
//!
//! A definite sign is the answer at every witness and the decision asks
//! the tier nothing about it. At a point scalar the sign is a
//! comparison of one rounded number: rule F turns a one-ulp error in a
//! SIGN argument into a whole `2.0` at the margin, and at a pole the
//! point channel has no clause 1 to refuse with. The instrument reports
//! such a margin as [`Event::Rounded`]; at `Sym<Interval>` the same
//! report would be [`Event::Enclosure`], a defect in one of the two
//! channels.
//!
//! The EXACT half of every row here is `sym11_witness_kind_interval_rows`,
//! which shares this file's residual builders and its witness pin, and
//! the certified twin of the two residuals is `sym_rule_f_interval_rows`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::{Band, Margin, Sign};
use geom_core::sym::contradiction::{self, Event, Report};
use geom_core::sym::revalue::record;
use geom_core::sym::{Rung, SymRegistration, with_session_rules};
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
        Sym::param(
            ParamSymbol::new(test_utils::symbol_id("x")),
            T::from_f64(x0),
        ),
        Sym::param(
            ParamSymbol::new(test_utils::symbol_id("y")),
            T::from_f64(0.5),
        ),
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
    pole_of(Sym::param(
        ParamSymbol::new(test_utils::symbol_id("t")),
        T::from_f64(t0),
    ))
}

/// The margin AT THE POINT, read without asking for a decision: the
/// value channel's own number, which is what a fired assertion's
/// message cannot show.
fn margin_at<T: Real>(build: impl FnOnce() -> Sym<T>) -> T {
    let (v, _) = with_session_rules(budget(), SymRules::shipped(), || build().value());
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
        Ok(Sign::Zero) if counts.symbolic > 0 => "proven".to_owned(),
        Ok(s) => format!("numeric {s:?}"),
        Err(e) => format!("refused {:?}", e.margin),
    }
}

/// One decision under `budget` and `rules`, ON ITS OWN THREAD: the
/// label, the session's counts, the numeric channel's own sign and what
/// the contradiction instrument read of the decision — or the panic's
/// own message. A panic inside `with_session_rules` leaves that
/// thread's session installed, and the next case would refuse to nest
/// ([`test_utils::own_thread`], which also keeps the message).
fn decide_under<T>(
    budget: SymBudget,
    rules: SymRules,
    build: impl FnOnce() -> Sym<T> + Send + 'static,
) -> Result<(String, SymCounts, f64, Report), String>
where
    T: Real + Decide + Send + 'static,
{
    test_utils::own_thread::caught(move || {
        let ((out, probe, report), counts) = with_session_rules(budget, rules, || {
            let ((out, probe), decisions) = record(|| {
                let m = build();
                (
                    geom_core::k_stats::decide("sym11_witness_kind", Margin::of(m), band()),
                    m.value().sign_within(band()).map(|d| d.sign),
                )
            });
            let report = contradiction::check(&decisions).expect("inside the session");
            (out, probe, report)
        });
        let numeric = match probe {
            Ok(Sign::Positive) => 1.0,
            Ok(Sign::Negative) => -1.0,
            Ok(Sign::Zero) => 0.0,
            Err(_) => f64::NAN,
        };
        (label(out, counts), counts, numeric, report)
    })
}

/// The one contradiction `report` holds: its event, rung and attempt.
fn only(report: &Report) -> Option<(Event, Rung, u8)> {
    match report.contradictions.as_slice() {
        [c] => Some((c.event, c.rung, c.attempt)),
        _ => None,
    }
}

/// [`decide_under`] at the shipped set.
fn decide_on_a_thread<T>(
    build: impl FnOnce() -> Sym<T> + Send + 'static,
) -> Result<(String, SymCounts, f64, Report), String>
where
    T: Real + Decide + Send + 'static,
{
    decide_under(budget(), SymRules::shipped(), build)
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
         `Disputed` — and `Sym<T>::sign_within` checks a symbolic zero against its \
         enclosure by the same const, so a scalar that disagrees with itself here asserts \
         on a reading that proves nothing, or skips one that proves",
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
/// its own): the margin is `−2`, the numeric answer is returned, and the
/// instrument reports the theorem it contradicts as a rounded reading,
/// at the first attempt. Red if `contradiction::check` stops asking the
/// algebra (its `Path::Algebra` ladder answering `None`).
#[test]
fn sym11_the_adversary_is_a_rounded_contradiction_at_sym_f64() {
    for x0 in ADVERSARY_X {
        let m = margin_at(|| adversary::<f64>(x0));
        let (l, c, n, r) = decide_on_a_thread(move || adversary::<f64>(x0))
            .expect("an inexact witness never panics on a contradiction");
        println!("  x = {x0:e}: margin {m:e} | {l} | numeric sign {n} | {c:?} | {r:?}");
        assert_eq!(m, -2.0, "x = {x0:e}: the adversary's margin at the point");
        assert_eq!(
            l, "numeric Negative",
            "x = {x0:e}: the numeric answer is what a definite sign returns, and the \
             ratified numeric-first order is what returns it"
        );
        assert_eq!(c.numeric, 1, "x = {x0:e}: {c:?}");
        assert_eq!(
            (r.checked, only(&r).map(|(e, _, a)| (e, a))),
            (1, Some((Event::Rounded, 0))),
            "x = {x0:e}: the form is a theorem and the value channel says −2: {r:?}"
        );
    }
}

/// **Values built before the session are absent from it.** The
/// adversary built outside any session and decided inside one: its
/// nodes are not in the session's table, so the instrument can ask
/// nothing of the decision and says so (`absent`) rather than reading it
/// as agreement; built inside, it reports the contradiction. Red if
/// `check` walks a node its session does not hold.
#[test]
fn sym11_values_built_outside_the_session_are_absent_to_the_instrument() {
    let x0 = ADVERSARY_X[0];
    let decide = |m: Sym<f64>| {
        let (_, decisions) = record(|| {
            geom_core::k_stats::decide("sym11_witness_kind", Margin::of(m), band()).map(|_| ())
        });
        contradiction::check(&decisions).expect("inside the session")
    };
    let outside = adversary::<f64>(x0);
    let (out, _) = with_session_rules(budget(), SymRules::shipped(), || decide(outside));
    let (inside, _) = with_session_rules(budget(), SymRules::shipped(), || {
        decide(adversary::<f64>(x0))
    });
    for (at, r, want) in [("outside", out, (1, 0, 0)), ("inside", inside, (0, 1, 1))] {
        assert_eq!(
            (r.absent, r.checked, r.contradictions.len()),
            want,
            "built {at} the session: (absent, checked, contradictions): {r:?}"
        );
    }
}

/// The same adversary at `Sym<Probe>`, whose value channel IS an
/// `f64`: the same rounding reaches the same report, and the K sample
/// the funnel records stays the numeric channel's own `Definite` at a
/// point.
#[cfg(feature = "probe")]
#[test]
fn sym11_the_adversary_is_a_rounded_contradiction_at_sym_probe() {
    use geom_core::Probe;
    for x0 in ADVERSARY_X {
        let (l, c, n, r) = decide_on_a_thread(move || adversary::<Probe>(x0))
            .expect("an inexact witness never panics on a contradiction");
        println!("  x = {x0:e}: {l} | numeric sign {n} | {c:?} | {r:?}");
        assert_eq!(l, "numeric Negative", "x = {x0:e}");
        assert_eq!(c.numeric, 1, "x = {x0:e}: {c:?}");
        assert_eq!(
            only(&r).map(|(e, _, _)| e),
            Some(Event::Rounded),
            "x = {x0:e}: {r:?}"
        );
    }
}

/// **A REGISTERED zero under a definite sign is the registration's
/// event, not the algebra's.** `y` is registered as `x` at values that
/// differ by half the witness's slack, so the inexact witness records
/// it; the band is drawn tight enough that the same difference is a
/// DEFINITE positive margin. The instrument reports the contradicted
/// axiom at the door, because its algebra, asked with the door shut,
/// proves nothing: `y − x` is no identity of the forms. Red if `check`
/// asks the door before the algebra declines, or not at all.
#[test]
fn sym11_a_contradicted_registration_is_the_registrations_event() {
    let tol = Tol::witness();
    let x0 = 1.0e3;
    let d = 0.5 * tol.eps() * x0;
    let band = Band::new(1.0e-2 * d, 1.0e-1 * d).unwrap();
    let ((reg, out, r), c) = with_session_rules(budget(), SymRules::shipped(), || {
        let x = Sym::<f64>::param(ParamSymbol::new(test_utils::symbol_id("x")), x0);
        let y = Sym::<f64>::param(ParamSymbol::new(test_utils::symbol_id("y")), x0 + d);
        let reg = y.register_equal(x, tol);
        let (out, decisions) =
            record(|| geom_core::k_stats::decide("sym11_registered", Margin::of(y - x), band));
        (
            reg,
            out,
            contradiction::check(&decisions).expect("inside the session"),
        )
    });
    println!("  {reg:?} {out:?} | {c:?} | {r:?}");
    assert_eq!(
        reg,
        SymRegistration::Recorded,
        "within the inexact witness's slack"
    );
    assert_eq!(out.ok(), Some(Sign::Positive), "the numeric answer stands");
    assert_eq!((c.symbolic, c.numeric), (0, 1), "{c:?}");
    assert_eq!(
        only(&r),
        Some((Event::Registration, Rung::Door, 0)),
        "the registration is what the numbers deny: {r:?}"
    );
}

/// **R1's decision at a point: the exact sign and the rounded one
/// disagree.** `d = (x + 1e17) − 1e17 − 0.25` at `x = 0.5` is exactly
/// `+0.25`, and `−0.25` in `f64`, so the point construction takes
/// `select_le_zero`'s `when_le` arm while the decision read, which signs
/// the EXACT form over the point box, would take `when_gt`. Over a point
/// box `param_over` hands the read a bracket of zero width, which is
/// what made it answer.
///
/// `w` is the gap between the arms (`z + w` against `z`).
fn rounded_decision<T: Real>(w: f64, atan: bool) -> Sym<T> {
    let pt = |name: &str, v: f64| {
        Sym::<T>::param_over(
            ParamSymbol::new(test_utils::symbol_id(name)),
            T::from_f64(v),
            v,
            v,
        )
    };
    let (x, z, w) = (pt("x", 0.5), pt("z", 1.0), pt("w", w));
    let big = Sym::<T>::from_f64(1.0e17);
    let d = (x + big) - big - Sym::from_f64(0.25);
    let s = d.select_le_zero(z + w, z);
    if atan { s.atan() - z.atan() } else { s - z }
}

/// R1's three cases: (a) `select(d, z + w, z) − z` at `w = 1e-15`,
/// (b) the same under `atan` on both sides, and (c) `w = 1e-3`, a gap
/// the band sees.
const ROUNDED_CASES: [(&str, f64, bool); 3] = [
    ("(a) select - z, w = 1e-15", 1.0e-15, false),
    ("(b) atan(select) - atan(z), w = 1e-15", 1.0e-15, true),
    ("(c) select - z, w = 1e-3", 1.0e-3, false),
];

/// **At an inexact witness the decision read declines** (DECIDE-10's
/// fix pass, R1's MINOR-1). The read's arm is the construction's only
/// where the construction's comparison is a certified sign of the real
/// the read encloses — an exact witness. At `Sym<f64>` the construction
/// compared a rounded value and took the other arm, so a zero through
/// the read would be a zero of a function the construction did not
/// evaluate: case (a) was `proven` though its value is `1e-15`, (b) was
/// `proven` once atoms stopped carrying the read's gate, and (c) was a
/// "theorem" the value channel disputed. Each now reads exactly what it
/// reads with the read shut, and the instrument finds no theorem to
/// contradict the numbers.
#[test]
fn sym11_the_decision_read_declines_at_sym_f64() {
    let x = std::hint::black_box(0.5_f64);
    assert!(
        (x + 1.0e17) - 1.0e17 - 0.25 < 0.0,
        "the premise: f64 rounds the decision to the other side of zero"
    );
    for (what, w, atan) in ROUNDED_CASES {
        let on = decide_under(budget(), SymRules::shipped(), move || {
            rounded_decision::<f64>(w, atan)
        })
        .expect("an inexact witness never panics on a contradiction");
        let shut = decide_under(budget(), SymRules::without_the_reads(), move || {
            rounded_decision::<f64>(w, atan)
        })
        .expect("an inexact witness never panics on a contradiction");
        println!("  {what}: read on {on:?} | read shut {shut:?}");
        assert_ne!(on.0, "proven", "{what}: the read answered at a point");
        assert_eq!(
            (on.1.symbolic, on.3.contradictions.len()),
            (0, 0),
            "{what}: {:?} {:?}",
            on.1,
            on.3
        );
        assert_eq!(on.0, shut.0, "{what}: the read shut answers the same");
    }
}

/// [`sym11_the_decision_read_declines_at_sym_f64`] at `Sym<Probe>`,
/// whose value channel is an `f64` too — the scalar of the K replay
/// (`editor_core::drive`'s `Sym<Probe>` replay over the midpoint box),
/// where a read zero would have reached K telemetry as a `symbolic`
/// sample.
#[cfg(feature = "probe")]
#[test]
fn sym11_the_decision_read_declines_at_sym_probe() {
    use geom_core::Probe;
    for (what, w, atan) in ROUNDED_CASES {
        let on = decide_under(budget(), SymRules::shipped(), move || {
            rounded_decision::<Probe>(w, atan)
        })
        .expect("an inexact witness never panics on a contradiction");
        println!("  {what}: {on:?}");
        assert_ne!(on.0, "proven", "{what}: the read answered at a point");
        assert_eq!(
            (on.1.symbolic, on.3.contradictions.len()),
            (0, 0),
            "{what}: {:?} {:?}",
            on.1,
            on.3
        );
    }
}

/// **One session decides at one witness kind, in every profile.** The
/// read above declines at a point because the session knows its
/// witness; a session that decided first at `Interval` and then at
/// `f64` would keep `Exact` and let the read answer at a point. That is
/// a hard `assert!` (`sym.rs`'s `note_witness`), not a `debug_assert!`:
/// `benches/` builds with debug assertions off, and this row is the
/// assertion's own plant.
#[test]
#[should_panic(expected = "two witness kinds")]
fn sym11_one_session_at_two_witness_kinds_panics() {
    use geom_core::Interval;
    let band = Band::linear(Tol::witness()).unwrap();
    let _ = with_session_rules(budget(), SymRules::shipped(), || {
        let xi = Sym::param_over(
            ParamSymbol::new(test_utils::symbol_id("xi")),
            Interval::from_bounds(1.0, 2.0),
            1.0,
            2.0,
        );
        let _ = geom_core::k_stats::decide("sym11_two_kinds", Margin::of(xi - xi), band);
        let x = Sym::<f64>::param(ParamSymbol::new(test_utils::symbol_id("x")), 0.5);
        let _ = geom_core::k_stats::decide("sym11_two_kinds", Margin::of(x - x), band);
    });
}

/// **A zero the registry did not need is not a contradicted
/// registration** (DECIDE-10's fix pass, R1's MINOR-2). With one true,
/// unrelated registration in the session the registry is non-empty, so
/// the instrument asks the door of every definite decision the algebra
/// declines. Case (c) above, R1's probe, reaches no zero at all at a
/// point, so it reports nothing; R2's adversary is a zero by rule F's
/// algebra, which the registry-free rungs prove, so it is the rounded
/// contradiction it is without the registration and NOT a contradicted
/// axiom. Each zero is reported once, as the event its proof rests on.
#[test]
fn sym11_a_zero_that_rests_on_no_registration_is_not_a_contradicted_one() {
    let unrelated = || {
        let q = Sym::<f64>::param(ParamSymbol::new(test_utils::symbol_id("q")), 2.0);
        let reg = (q * Sym::from_f64(1.0)).register_equal(q, Tol::witness());
        assert_eq!(reg, SymRegistration::Recorded, "a true registration");
    };
    let read = decide_on_a_thread(move || {
        unrelated();
        rounded_decision::<f64>(1.0e-3, false)
    })
    .expect("an inexact witness never panics on a contradiction");
    println!("  R1's read zero, registry non-empty: {read:?}");
    assert_eq!(
        (read.3.contradictions.len(), read.1.symbolic),
        (0, 0),
        "R1's probe: {:?} {:?}",
        read.1,
        read.3
    );
    let algebra = decide_on_a_thread(move || {
        unrelated();
        adversary::<f64>(ADVERSARY_X[0])
    })
    .expect("an inexact witness never panics on a contradiction");
    println!("  R2's adversary, registry non-empty: {algebra:?}");
    assert_eq!(algebra.0, "numeric Negative");
    assert_eq!(
        only(&algebra.3).map(|(e, _, _)| e),
        Some(Event::Rounded),
        "rule F's zero is the algebra's, not a contradicted axiom: {:?}",
        algebra.3
    );
}

/// **THE POLE (R1's residue), both arms.** At `t = 1` the tier answers
/// the form's THEOREM on a box where the function has no value at all.
/// Nothing false is reported and nothing contradicted — the numeric
/// channel never answered a definite non-zero sign, so there is nothing
/// for the instrument to ask.
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
fn sym11_the_pole_at_sym_f64_answers_the_form_and_contradicts_nothing() {
    for (what, t0) in [("at the pole", 1.0), ("clear of it", 0.4)] {
        let (l, c, n, r) = decide_on_a_thread(move || pole::<f64>(t0))
            .expect("the pole does not panic at an inexact witness");
        println!("  [{what}] t = {t0}: {l} | numeric sign {n} | {c:?}");
        assert_eq!(l, "proven", "[{what}] t = {t0}");
        assert_eq!(
            (r.checked, r.contradictions.len()),
            (0, 0),
            "[{what}] t = {t0}: the numeric channel answered no definite sign, so there is \
             nothing for the form to contradict: {c:?} {r:?}"
        );
    }
}
