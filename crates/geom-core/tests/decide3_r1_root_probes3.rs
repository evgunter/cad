//! **R1's DECIDE-3 probes, round 3**: the tie side of the decision
//! read, and what the read does to a comparison of two CONSTANTS in a
//! document that has a bracketed parameter somewhere (the slab shape).

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::panic, clippy::float_cmp)]

use geom_core::interval::Interval;
use geom_core::predicate::{Band, Sign};
use geom_core::real::Real;
use geom_core::sym::{SymCounts, with_session_rules};
use geom_core::{Decide, ParamSymbol, Sym, SymBudget, SymRules, Tol};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}
fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}
fn pb(n: &str, lo: f64, hi: f64) -> Sym<Interval> {
    Sym::param_over(ParamSymbol::of(n), Interval::from_bounds(lo, hi), lo, hi)
}
fn k(v: f64) -> Sym<Interval> {
    Sym::from_f64(v)
}
fn probe(rules: SymRules, f: impl FnOnce() -> Sym<Interval>) -> (Result<Sign, String>, SymCounts) {
    let (s, c) = with_session_rules(budget(), rules, || {
        f().sign_within(band()).map_err(|e| format!("{e:?}"))
    });
    (s, c)
}
fn line(name: &str, r: &(Result<Sign, String>, SymCounts)) {
    let v = match &r.0 {
        Ok(s) => format!("{s:?}"),
        Err(_) => "REFUSED".into(),
    };
    println!(
        "  {name:<50} {v:<8} sym0={} gated={} numeric={}",
        r.1.symbolic_zero, r.1.sign_gated, r.1.numeric
    );
}

#[test]
fn r1_the_tie_side_and_the_constant_comparison() {
    let rows: Vec<(&str, fn() -> Sym<Interval>)> = vec![
        // The arms are parameters, so the numeric channel cannot settle
        // the residual and the tier is actually asked.
        ("select(X,Y,Z)-Y  X in [-1,0] tie INCLUDED  ", || {
            let x = pb("x", -1.0, 0.0);
            let (y, z) = (pb("y", 1.0, 2.0), pb("z", 5.0, 6.0));
            Real::select_le_zero(x, y, z) - y
        }),
        ("select(X,Y,Z)-Z  X in [0,1]  tie at the LO ", || {
            let x = pb("x", 0.0, 1.0);
            let (y, z) = (pb("y", 1.0, 2.0), pb("z", 5.0, 6.0));
            Real::select_le_zero(x, y, z) - z
        }),
        ("select(X,Y,Z)-Z  X in [.5,1] strictly > 0  ", || {
            let x = pb("x", 0.5, 1.0);
            let (y, z) = (pb("y", 1.0, 2.0), pb("z", 5.0, 6.0));
            Real::select_le_zero(x, y, z) - z
        }),
        // Two CONSTANTS compared, in a document that has a bracketed
        // parameter elsewhere — the M10-3 slab's shape.
        ("max(1,1/4)*Y - Y   (a param is in the box)  ", || {
            let y = pb("y", 1.0, 2.0);
            k(1.0).max(k(0.25)) * y - y
        }),
        ("min(1,1/4)*Y - Y/4 (a param is in the box)  ", || {
            let y = pb("y", 1.0, 2.0);
            k(1.0).min(k(0.25)) * y - y * k(0.25)
        }),
        // The same two constants with NO bracketed parameter anywhere.
        ("max(1,1/4)*Y - Y   (param has NO bracket)   ", || {
            let y = Sym::param(ParamSymbol::of("y"), Interval::from_bounds(1.0, 2.0));
            k(1.0).max(k(0.25)) * y - y
        }),
    ];
    println!("shipped:");
    for (n, f) in &rows {
        line(n, &probe(SymRules::shipped(), *f));
    }
    println!("without_the_reads:");
    for (n, f) in &rows {
        line(n, &probe(SymRules::without_the_reads(), *f));
    }
}
