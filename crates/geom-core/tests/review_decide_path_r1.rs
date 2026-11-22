//! Reviewer probes for the decide-path unit: each row prints what it
//! measured and asserts the reading it found on the reviewed head, so a
//! change of behaviour shows as a red row naming the reading.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::{Band, Margin, Sign};
use geom_core::sym::contradiction::{self, Event};
use geom_core::sym::deferred::{discharge, with_deferred_session};
use geom_core::sym::revalue::record;
use geom_core::sym::{with_session_retry, with_session_rules};
use geom_core::tolerance::Tol;
use geom_core::sym::SymRegistration;
use geom_core::{
    Decide, Interval, ParamSymbol, Real, Sym, SymBudget, SpanLocate, SymCounts, SymRetry,
    SymRules,
};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

fn band() -> Band {
    Band::new(1.0e-9, 1.0e-8).unwrap()
}

fn x_at(v: f64) -> Sym<Interval> {
    Sym::param(
        ParamSymbol::new(test_utils::symbol_id("x")),
        Interval::from_bounds(v, v),
    )
}

fn decide(m: Sym<Interval>) -> Result<geom_core::predicate::Decided, ()> {
    geom_core::k_stats::decide_reported("review_decide_path", Margin::of(m), band()).map_err(|_| ())
}

/// The planted breach of the PR's own panic row (`x(1) − x(1 + 5e-10)`,
/// an in-band enclosure clear of 0 over a zero form), taken in a
/// DEFERRING session at `Sym<Interval>` and discharged by a reader.
#[test]
fn p1_a_deferred_discharge_at_an_exact_witness_passes_no_enclosure_check() {
    let ((sign, proven), counts) =
        with_deferred_session(budget(), SymRules::shipped(), SymRetry::none(), || {
            let d = decide(x_at(1.0) - x_at(1.0 + 5.0e-10)).expect("in band");
            (d.sign, discharge(d.margin))
        });
    println!("p1: sign {sign:?} proven {proven} {counts:?}");
    assert_eq!(sign, Sign::Zero);
    assert!(
        proven,
        "the tier's zero over an enclosure excluding 0 is handed to the reader as proven, \
         and nothing panicked"
    );
}

/// A definite decision whose plain walk freezes at a starved budget:
/// does `check` leave `frozen` (and the rest of the receipt) as a run
/// without it reads?
#[test]
fn p2_the_instrument_and_the_receipts_frozen_column() {
    let starved = SymBudget {
        max_terms: 2,
        max_degree: 2,
    };
    let run = |instrument: bool| -> (SymCounts, Option<contradiction::Report>) {
        let (report, counts) = with_session_rules(starved, SymRules::shipped(), || {
            let (_, decisions) = record(|| {
                let x = x_at(1.0);
                let y = Sym::param(
                    ParamSymbol::new(test_utils::symbol_id("y")),
                    Interval::from_bounds(2.0, 2.0),
                );
                let m = (x + y + <Sym<Interval> as Real>::from_f64(3.0)) * (x - y) * (x + y);
                decide(m).map(|d| d.sign)
            });
            instrument.then(|| contradiction::check(&decisions).expect("in session"))
        });
        (counts, report)
    };
    let (bare, _) = run(false);
    let (with, report) = run(true);
    println!("p2: bare {bare:?}\n    with {with:?}\n    {report:?}");
    assert_eq!(bare.frozen, 0, "a definite sign builds no form");
    assert_eq!(with, bare, "the instrument's walks leave the receipt alone");
}

/// After `check`, a LATER decision in the same session: the session's
/// memos now hold the instrument's forms. Is a later decision's receipt
/// what it would have been?
#[test]
fn p3_a_decision_after_the_instrument_reads_the_instruments_memos() {
    let starved = SymBudget {
        max_terms: 2,
        max_degree: 2,
    };
    let run = |instrument: bool| -> SymCounts {
        let ((), counts) = with_session_rules(starved, SymRules::shipped(), || {
            let x = x_at(1.0);
            let y = Sym::param(
                ParamSymbol::new(test_utils::symbol_id("y")),
                Interval::from_bounds(2.0, 2.0),
            );
            let big = (x + y + <Sym<Interval> as Real>::from_f64(3.0)) * (x - y) * (x + y);
            let (_, decisions) = record(|| decide(big).map(|d| d.sign));
            if instrument {
                let _ = contradiction::check(&decisions);
            }
            // An in-band margin over the same sub-DAG: the tier is asked.
            let tiny = <Sym<Interval> as Real>::from_f64(1.0e-12);
            let _ = decide(big * tiny - big * tiny + tiny * (x - x));
        });
        counts
    };
    let (bare, with) = (run(false), run(true));
    println!("p3: bare {bare:?}\n    with {with:?}");
    assert_eq!(
        (with.symbolic, with.numeric),
        (bare.symbolic, bare.numeric),
        "the later decision decides alike"
    );
    println!(
        "p3: frozen without the instrument {} and after it {}",
        bare.frozen, with.frozen
    );
}

/// A registration that contradicts on a ROUNDED margin (`Sym<f64>`):
/// two values the f64 witness admits as equal, and a margin that
/// amplifies their difference to a definite sign.
#[test]
fn p4_a_registration_contradicted_on_a_rounded_margin_is_reported() {
    let (out, counts) = with_session_rules(budget(), SymRules::shipped(), || {
        let a: Sym<f64> = Sym::param(ParamSymbol::new(test_utils::symbol_id("a")), 1.0);
        let b: Sym<f64> = Sym::param(ParamSymbol::new(test_utils::symbol_id("b")), 1.0 + 1.0e-13);
        let r = b.register_equal(a, Tol::witness());
        let m = (b - a) * <Sym<f64> as Real>::from_f64(1.0e9);
        let (s, decisions) = record(|| m.sign_within(band()).map(|d| d.sign).map_err(|_| ()));
        (r, s, contradiction::check(&decisions).expect("in session"))
    });
    println!("p4: {out:?} {counts:?}");
    assert_eq!(out.0, SymRegistration::Recorded);
    assert_eq!(out.1, Ok(Sign::Positive));
    let found: Vec<_> = out.2.contradictions.iter().map(|c| c.event).collect();
    assert_eq!(found, [Event::Registration], "{:?}", out.2);
}

/// The registration event where only a RETRY's door proves the zero.
#[test]
fn p5_a_registration_only_a_retrys_door_reaches() {
    let retry = SymRetry {
        bits: Some(512),
        ..SymRetry::none()
    };
    let c = || <Sym<Interval> as Real>::from_f64(1.0 / 3.0);
    let chain = |k: usize| (1..k).fold(c(), |acc, _| acc * c());
    let (out, counts) = with_session_retry(budget(), SymRules::shipped(), retry, || {
        let a = Sym::param(
            ParamSymbol::new(test_utils::symbol_id("a")),
            Interval::from_bounds(0.9, 1.1),
        );
        let lo = a - <Sym<Interval> as Real>::from_f64(0.2);
        let hi = a + <Sym<Interval> as Real>::from_f64(0.2);
        let r = (lo.register_equal(a, Tol::witness()), hi.register_equal(a, Tol::witness()));
        let m = (chain(5) - chain(2) * chain(3)) + (lo - hi);
        let (s, decisions) = record(|| m.sign_within(band()).map(|d| d.sign).map_err(|_| ()));
        (r, s, contradiction::check(&decisions).expect("in session"))
    });
    println!("p5: {out:?} {counts:?}");
    let found: Vec<_> = out
        .2
        .contradictions
        .iter()
        .map(|c| (c.event, c.attempt))
        .collect();
    assert_eq!(found, [(Event::Registration, 1)], "{:?}", out.2);
}

/// An opaque value minted BEFORE the session and one minted inside it:
/// the opaque sequence restarts at the session's start, so on a fresh
/// thread both are sequence 0. Two untracked reals, one node id?
#[test]
fn p6_an_opaque_minted_outside_and_one_inside_the_session() {
    let got = std::thread::spawn(|| {
        let outside = Sym::<Interval>::opaque(Interval::from_bounds(1.0, 1.0));
        std::panic::catch_unwind(|| {
            with_session_rules(budget(), SymRules::shipped(), || {
                let inside = Sym::<Interval>::opaque(Interval::from_bounds(3.0, 3.0));
                let same_node = outside.node() == inside.node();
                // Definite: the numbers answer.
                let far = decide(outside - inside).map(|d| (d.sign, d.margin.proven()));
                // In band, clear of 0.
                let near_in = Sym::<Interval>::opaque(Interval::from_bounds(1.0, 1.0));
                let _ = near_in;
                (same_node, far)
            })
        })
        .map_err(|_| "panicked")
    })
    .join()
    .unwrap();
    println!("p6: {got:?}");
}

/// The same collision in band: `outside − inside` with values 5e-10
/// apart.
#[test]
fn p7_the_opaque_collision_in_band() {
    let got = std::thread::spawn(|| {
        let outside = Sym::<Interval>::opaque(Interval::from_bounds(1.0, 1.0));
        std::panic::catch_unwind(|| {
            with_session_rules(budget(), SymRules::shipped(), || {
                let inside = Sym::<Interval>::opaque(Interval::from_bounds(
                    1.0 + 5.0e-10,
                    1.0 + 5.0e-10,
                ));
                (
                    outside.node() == inside.node(),
                    decide(outside - inside).map(|d| (d.sign, d.margin.proven())),
                )
            })
        })
        .map_err(|e| {
            e.downcast_ref::<String>()
                .cloned()
                .unwrap_or_else(|| "panicked".to_owned())
        })
    })
    .join()
    .unwrap();
    println!("p7: {got:?}");
}

/// The same collision at `Sym<f64>`, where nothing is checked: is a
/// false `Zero` returned proven?
#[test]
fn p8_the_opaque_collision_at_sym_f64() {
    let got = std::thread::spawn(|| {
        let outside = Sym::<f64>::opaque(1.0);
        with_session_rules(budget(), SymRules::shipped(), || {
            let inside = Sym::<f64>::opaque(1.0 + 5.0e-10);
            let m = outside - inside;
            (
                outside.node() == inside.node(),
                geom_core::k_stats::decide_reported("review_decide_path", Margin::of(m), band())
                    .map(|d| (d.sign, d.margin.proven()))
                    .map_err(|_| ()),
            )
        })
    })
    .join()
    .unwrap();
    println!("p8: {got:?}");
}

/// A mismatched bracket (`param_over` whose bracket does not hold the
/// value): the decision read certifies an arm over the bracket.
#[test]
fn p9_a_bracket_that_does_not_hold_its_value() {
    let got = std::panic::catch_unwind(|| {
        with_session_rules(budget(), SymRules::shipped(), || {
            let x = Sym::param_over(
                ParamSymbol::new(test_utils::symbol_id("x")),
                Interval::from_bounds(-2.0e-10, -1.0e-10),
                1.0,
                2.0,
            );
            decide(x.abs() - x).map(|d| (d.sign, d.margin.proven()))
        })
    })
    .map_err(|e| {
        e.downcast_ref::<String>()
            .cloned()
            .unwrap_or_else(|| "panicked".to_owned())
    });
    println!("p9: {got:?}");
}

/// A nested scalar: `Sym<Dual<Interval>>`-style lanes are exact
/// witnesses too; does the enclosure check read their value channel?
#[test]
fn p10_the_check_at_a_hull_and_an_opaque_leaf() {
    let ((a, b), counts) = with_session_rules(budget(), SymRules::shipped(), || {
        let h = Sym::<Interval>::from_f64(1.0e-10).enclosure_hull(Sym::from_f64(3.0e-10));
        let o = Sym::<Interval>::opaque(Interval::from_bounds(1.0e-10, 3.0e-10));
        (
            decide(h - h).map(|d| (d.sign, d.margin.proven())),
            decide(o * h - h * o).map(|d| (d.sign, d.margin.proven())),
        )
    });
    println!("p10: hull {a:?} opaque {b:?} {counts:?}");
    assert_eq!(a, Ok((Sign::Zero, true)));
    assert_eq!(b, Ok((Sign::Zero, true)));
}

/// The enclosure check at the REGISTRY DOOR's rung: two registrations
/// the witness admits (touching enclosures) and an in-band margin only
/// the door proves zero, over an enclosure clear of 0. Red if the check
/// is not taken on the door's zeros.
#[test]
#[should_panic(expected = "at the Door rung (attempt 0) and its certified enclosure")]
fn p11_a_doors_zero_whose_enclosure_excludes_zero_panics() {
    let _ = with_session_rules(budget(), SymRules::shipped(), || {
        let a = Sym::param(
            ParamSymbol::new(test_utils::symbol_id("a")),
            Interval::from_bounds(0.9, 1.1),
        );
        let lo = a - <Sym<Interval> as Real>::from_f64(0.2);
        let hi = a + <Sym<Interval> as Real>::from_f64(0.2);
        let _ = lo.register_equal(a, Tol::witness());
        let _ = hi.register_equal(a, Tol::witness());
        decide((lo - hi) * <Sym<Interval> as Real>::from_f64(1.0e-10))
    });
}

/// The planted breach of `a_proven_zero_whose_enclosure_excludes_zero_panics`
/// at ANOTHER exact witness, `Sym<Dual<Interval>>`: the value channel
/// has no `enclosure_probe` there, so is the zero checked?
#[test]
fn p12_the_breach_at_sym_dual_interval() {
    type D = geom_core::Dual<Interval>;
    let x = |v: f64| -> Sym<D> {
        Sym::param(
            ParamSymbol::new(test_utils::symbol_id("x")),
            D::constant(Interval::from_bounds(v, v)),
        )
    };
    let got = std::panic::catch_unwind(|| {
        with_session_rules(budget(), SymRules::shipped(), || {
            geom_core::k_stats::decide_reported(
                "review_decide_path",
                Margin::of(x(1.0) - x(1.0 + 5.0e-10)),
                band(),
            )
            .map(|d| (d.sign, d.margin.proven()))
            .map_err(|_| ())
        })
        .0
    })
    .map_err(|_| "panicked");
    println!(
        "p12: exact witness {} -> {got:?}",
        matches!(<Sym<D> as Real>::WITNESS, geom_core::Witness::Exact)
    );
    assert_eq!(
        got,
        Ok(Ok((Sign::Zero, true))),
        "a proven zero over an enclosure excluding 0, unchecked at this exact scalar"
    );
}
