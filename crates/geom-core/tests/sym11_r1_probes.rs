//! SYM-11 review probes (r1): documents the unit did not measure, driven
//! through the public doors at `Sym<f64>`, `Sym<Probe>` and
//! `Sym<Interval>`, with the receipt printed beside each answer.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::{Band, Margin, Sign};
use geom_core::sym::with_session_rules;
use geom_core::tolerance::Tol;
use geom_core::{Decide, ParamSymbol, Real, Sym, SymBudget, SymCounts, SymRules};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// Mechanism 1 without a body: `(x + d) − d − x` is the zero form, and at
/// `d = 1e9` the point channel's rounding of `x + d` is ~1e-7.
fn far<T: Real>(d0: f64, x0: f64) -> Sym<T> {
    let d = Sym::<T>::param(ParamSymbol::of("d"), T::from_f64(d0));
    let x = Sym::<T>::param(ParamSymbol::of("x"), T::from_f64(x0));
    (x + d) - d - x
}

/// My own rule-F form: `copysign(1, (x + y)² − x² − 2xy − y² + 1e-40·(1 + z²)) − 1`.
fn my_adversary<T: Real>(x0: f64, y0: f64) -> Sym<T> {
    let one = || Sym::<T>::from_f64(1.0);
    let x = Sym::<T>::param(ParamSymbol::of("x"), T::from_f64(x0));
    let y = Sym::<T>::param(ParamSymbol::of("y"), T::from_f64(y0));
    let z = Sym::<T>::param(ParamSymbol::of("z"), T::from_f64(0.25));
    let e = (x + y).powi(2) - x.powi(2) - Sym::from_f64(2.0) * x * y - y.powi(2)
        + Sym::from_f64(1.0e-40) * (one() + z.powi(2));
    one().copysign(e) - one()
}

fn run<T: Real + Decide>(
    name: &'static str,
    build: impl FnOnce() -> Sym<T>,
) -> (Result<Sign, geom_core::predicate::Indeterminate>, SymCounts) {
    with_session_rules(budget(), SymRules::shipped(), || {
        geom_core::k_stats::decide(name, Margin::of(build()), band())
    })
}

#[test]
fn r1_the_far_placement_as_one_line_at_sym_f64() {
    let mut disputed = 0;
    for (d0, x0) in [
        (0.0, 0.3),
        (1.0e6, 0.3),
        (3.0e7, 0.3),
        (2.0e8, 0.3),
        (1.0e9, 0.3),
    ] {
        let (out, c) = run("r1_far", || far::<f64>(d0, x0));
        let v = with_session_rules(budget(), SymRules::shipped(), || far::<f64>(d0, x0).value).0;
        println!("  d={d0:e} x={x0}: value {v:e} -> {out:?} | {c:?}");
        assert!(
            c.theorems_disputed <= c.numeric,
            "a dispute is a numeric decision: {c:?}"
        );
        if c.theorems_disputed > 0 {
            assert_eq!(c.symbolic_zero, 0, "{c:?}");
            assert!(
                matches!(out, Ok(Sign::Positive | Sign::Negative)),
                "{out:?}"
            );
            disputed += 1;
        }
    }
    println!(
        "  disputed at {disputed} of 5 placements at eps={:e}",
        Tol::witness().eps()
    );
}

#[test]
fn r1_my_own_rule_f_form_at_sym_f64() {
    let mut disputed = 0;
    for (x0, y0) in [
        (1.0e8, 7.0e7),
        (3.0e8, 1.0e8),
        (5.0e8, 2.0e8),
        (1.0e9, 3.0e8),
        (2.0, 3.0),
    ] {
        let (out, c) = run("r1_rule_f", || my_adversary::<f64>(x0, y0));
        println!("  x={x0:e} y={y0:e}: {out:?} | {c:?}");
        if c.theorems_disputed > 0 {
            disputed += 1;
            assert_eq!(out, Ok(Sign::Negative), "{out:?}");
        } else {
            assert_eq!(out, Ok(Sign::Zero), "{out:?}");
            assert!(c.symbolic_zero > 0, "{c:?}");
        }
    }
    println!("  disputed at {disputed} of 5 points");
}

#[cfg(feature = "probe")]
#[test]
fn r1_the_k_sample_at_sym_probe_is_the_numeric_channels_own() {
    use geom_core::Probe;
    use geom_core::k_stats::{start_recording, take_samples};
    start_recording();
    let (out, c) = run("r1_probe_far", || far::<Probe>(1.0e9, 0.3));
    let samples = take_samples();
    println!("  {out:?} | {c:?}");
    for s in &samples {
        println!("  sample: {s:?}");
    }
    assert_eq!(samples.len(), 1, "one decision, one sample");
    assert_eq!(samples[0].predicate, "r1_probe_far");
    if c.theorems_disputed > 0 {
        assert_ne!(
            samples[0].margin, 0.0,
            "the sample is the point channel's own margin"
        );
    }
}

#[cfg(feature = "interval")]
#[test]
fn r1_the_same_forms_at_sym_interval_dispute_nothing() {
    use geom_core::Interval;
    let over = |name: &str, lo: f64, hi: f64| {
        Sym::param_over(ParamSymbol::of(name), Interval::from_bounds(lo, hi), lo, hi)
    };
    let cases: [(&str, &dyn Fn() -> Sym<Interval>); 3] = [
        ("far d=1e9 point, x over [0.29,0.31]", &|| {
            let d = Sym::<Interval>::param(ParamSymbol::of("d"), Interval::from_f64(1.0e9));
            let x = over("x", 0.29, 0.31);
            (x + d) - d - x
        }),
        ("far d=1e9 point, x=0.3 point", &|| {
            let d = Sym::<Interval>::param(ParamSymbol::of("d"), Interval::from_f64(1.0e9));
            let x = Sym::<Interval>::param(ParamSymbol::of("x"), Interval::from_f64(0.3));
            (x + d) - d - x
        }),
        ("my rule-F form over x ∈ [1e8 ∓ 1]", &|| {
            let one = || Sym::<Interval>::from_f64(1.0);
            let x = over("x", 1.0e8 - 1.0, 1.0e8 + 1.0);
            let y = over("y", 7.0e7 - 1.0, 7.0e7 + 1.0);
            let z = over("z", 0.2, 0.3);
            let e = (x + y).powi(2) - x.powi(2) - Sym::from_f64(2.0) * x * y - y.powi(2)
                + Sym::from_f64(1.0e-40) * (one() + z.powi(2));
            one().copysign(e) - one()
        }),
    ];
    for (what, build) in cases {
        let (out, c) = run("r1_interval", build);
        println!("  {what}: {out:?} | {c:?}");
        assert_eq!(c.theorems_disputed, 0, "{what}: {c:?}");
    }
}
