//! R2's own e2e exercise for SYM-11: a rule-F form the unit did not
//! measure, whose `f64` sign is wrong by construction, driven at all
//! three `Sym` lanes through the public door.
//!
//! `G = (x+1)³ − x³ − 3x² − 3x − 1 + 1e-35·(1+y²)` (a CUBIC cancellation, one degree past the unit's quadratic adversary) is the constant
//! `1e-35` as a FORM (difference of squares), so rule F folds
//! `copysign(1, G)` to `1` and `copysign(1, G) − 1` is the zero form.
//! At `x = 1e9`, `y = x − 1` the two products are `1e18`-scale and
//! their `f64` difference is roundoff of order one.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::{Band, Margin};
use geom_core::sym::with_session_rules;
use geom_core::{Decide, ParamSymbol, Real, Sym, SymBudget, SymCounts, SymRules};

fn budget() -> SymBudget {
    SymBudget { max_terms: 4096, max_degree: 128 }
}

fn cubic_resid<T: Real>(x0: f64, y0: f64) -> Sym<T> {
    let one = || Sym::<T>::from_f64(1.0);
    let x = Sym::<T>::param(ParamSymbol::of("x"), T::from_f64(x0));
    let y = Sym::<T>::param(ParamSymbol::of("y"), T::from_f64(y0));
    let three = || Sym::<T>::from_f64(3.0);
    let g = (x + one()).powi(3) - x.powi(3) - three() * x.powi(2) - three() * x - one()
        + Sym::from_f64(1.0e-35) * (one() + y.powi(2));
    one().copysign(g) - one()
}

fn run<T: Real + Decide + Send + 'static>(
    label: &str,
    build: impl FnOnce() -> Sym<T> + Send + 'static,
) {
    let band = Band::new(1.0e-9, 1.0e-8).unwrap();
    let out = std::thread::spawn(move || {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            with_session_rules(budget(), SymRules::shipped(), || {
                geom_core::k_stats::decide("r2_sym11_e2e", Margin::of(build()), band)
            })
        }))
    })
    .join()
    .expect("the probe thread joins");
    match out {
        Ok((ans, c)) => println!(
            "  [{label}] answer {ans:?} | theorems_disputed {} numeric {} symbolic_zero {} | {c:?}",
            c.theorems_disputed, c.numeric, c.symbolic_zero
        ),
        Err(_) => println!("  [{label}] PANICKED (the contradiction assertion fired)"),
    }
    let _: fn(SymCounts) = |_| ();
}

#[test]
fn r2_e2e_cubic_cancellation_at_the_three_lanes() {
    let (x0, y0) = (1.0e8, 0.5);
    run::<f64>("Sym<f64>", move || cubic_resid::<f64>(x0, y0));
    #[cfg(feature = "probe")]
    run::<geom_core::Probe>("Sym<Probe>", move || {
        cubic_resid::<geom_core::Probe>(x0, y0)
    });
    #[cfg(feature = "interval")]
    {
        use geom_core::Interval;
        let one = || Sym::<Interval>::from_f64(1.0);
        let over = |n: &str, lo: f64, hi: f64| {
            Sym::param_over(ParamSymbol::of(n), Interval::from_bounds(lo, hi), lo, hi)
        };
        run::<Interval>("Sym<Interval>", move || {
            let x = over("x", x0 - 1.0, x0 + 1.0);
            let y = over("y", 0.4, 0.6);
            let three = || Sym::<Interval>::from_f64(3.0);
            let g = (x + one()).powi(3) - x.powi(3) - three() * x.powi(2) - three() * x - one()
                + Sym::from_f64(1.0e-35) * (one() + y.powi(2));
            one().copysign(g) - one()
        });
    }
}
