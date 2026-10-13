//! DECIDE-10 review R1 probes (not for merge).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::{Band, Margin};
use geom_core::sym::with_session_rules;
use geom_core::{ParamSymbol, Real, Sym, SymBudget, SymRules, Tol};

fn budget() -> SymBudget {
    SymBudget { max_terms: 4096, max_degree: 128 }
}

fn pt(name: &str, v: f64) -> Sym<f64> {
    Sym::param_over(ParamSymbol::new(test_utils::symbol_id(name)), v, v, v)
}

thread_local!(static ATAN: std::cell::Cell<bool> = const { std::cell::Cell::new(false) });

/// Claim 2: the read signs the EXACT decision form over the box; at an
/// f64 witness the construction compares a ROUNDED value and can take
/// the other arm. `d = (x + 1e17) − 1e17 − 0.25` at `x = 0.5` is
/// exactly `+0.25` but `−0.25` in f64, so the f64 construction takes
/// `when_le = z + w`; the read takes `when_gt = z` and proves
/// `select(d, z+w, z) − z` zero though it is `w` where the construction
/// took the decision it took.
#[test]
fn r1_a_read_zero_at_an_f64_witness_names_an_arm_the_construction_did_not_take() {
    let band = Band::linear(Tol::witness()).unwrap();
    for (atan, w) in [(false, 1e-15), (false, 1e-3), (true, 1e-15), (true, 1e-3)] {
        ATAN.with(|a| a.set(atan));
        for (what, rules) in [
            ("read on", SymRules::shipped()),
            ("read shut", SymRules::without_the_reads()),
        ] {
            let ((out, value, proven), counts) = with_session_rules(budget(), rules, || {
                let x = pt("x", 0.5);
                let z = pt("z", 1.0);
                let wp = pt("w", w);
                let big = Sym::<f64>::from_f64(1e17);
                let d = (x + big) - big - Sym::from_f64(0.25);
                assert!(d.value <= 0.0, "f64 takes when_le");
                let s = d.select_le_zero(z + wp, z);
                let m = if ATAN.with(|a| a.get()) { s.atan() - z.atan() } else { s - z };
                let r = geom_core::k_stats::decide_reported("r1", Margin::of(m), band);
                let proven = r.as_ref().is_ok_and(|d| d.margin.proven());
                (r.map(|d| d.sign), m.value, proven)
            });
            println!("atan={atan} w={w:e} {what}: {out:?} value={value:e} proven={proven} {counts:?}");
        }
    }
}

/// Claim 4 / rule C: the retired fold's zero is reachable by spelling
/// `|r|` as `max(r, −r)` — the same real function, read as a decision.
#[test]
fn r1_a_sign_of_the_box_proves_through_max() {
    use geom_core::Interval;
    let band = Band::linear(Tol::witness()).unwrap();
    let r = || {
        Sym::param_over(
            ParamSymbol::new(test_utils::symbol_id("r")),
            Interval::from_bounds(1.0e-3, 2.0e-3),
            1.0e-3,
            2.0e-3,
        )
    };
    for (what, rules) in [
        ("shipped", SymRules::shipped()),
        ("all", SymRules::all()),
        ("read shut", SymRules::without_the_reads()),
    ] {
        let (out, counts) = with_session_rules(budget(), rules, || {
            let r = r();
            let d = |m: Sym<Interval>| {
                geom_core::k_stats::decide_reported("r1", Margin::of(m), band)
                    .map(|d| (d.sign, d.margin.proven()))
            };
            (
                d(r.abs() - r),
                d(r.max(-r) - r),
                d((r * r).sqrt() - r.max(-r)),
                d((r * r).sqrt() - r),
            )
        });
        println!("{what}: abs-r {:?} | max(r,-r)-r {:?} | sqrt(r²)-max(r,-r) {:?} | sqrt(r²)-r {:?} | {counts:?}", out.0, out.1, out.2, out.3);
    }
}

/// Claim 5: with ANY registration in the session, a definite-sign
/// margin whose door form is zero through the read alone is counted
/// `registrations_contradicted` — no registration is involved in it.
#[test]
fn r1_a_read_zero_counts_as_a_contradicted_registration() {
    let band = Band::linear(Tol::witness()).unwrap();
    let ((), counts) = with_session_rules(budget(), SymRules::shipped(), || {
        let x = pt("x", 0.5);
        let z = pt("z", 1.0);
        let wp = pt("w", 1e-3);
        // An unrelated, true registration: the registry is non-empty.
        let q = pt("q", 2.0);
        let _ = (q * Sym::from_f64(1.0)).register_equal(q, Tol::witness());
        let big = Sym::<f64>::from_f64(1e17);
        let d = (x + big) - big - Sym::from_f64(0.25);
        let m = d.select_le_zero(z + wp, z) - z;
        let _ = geom_core::k_stats::decide_reported("r1", Margin::of(m), band);
    });
    println!("registry non-empty: {counts:?}");
}
