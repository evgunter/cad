//! **R1's DECIDE-3 probes, round 2**: uniformity across mint sites,
//! the context-dependence of the side condition's source 3, and the
//! two reads over a real parameter BOX.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::panic, clippy::float_cmp)]

use geom_core::interval::Interval;
use geom_core::predicate::{Band, Sign};
use geom_core::real::Real;
use geom_core::sym::{SymCounts, SymRegistration, with_session_rules};
use geom_core::{Decide, ParamSymbol, Sym, SymBudget, SymRules, Tol};

fn budget() -> SymBudget {
    SymBudget { max_terms: 4096, max_degree: 128 }
}
fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}
/// A parameter WITH a bracket, so the reads have a box.
fn pb(name: &str, lo: f64, hi: f64) -> Sym<Interval> {
    Sym::param_over(ParamSymbol::of(name), Interval::from_bounds(lo, hi), lo, hi)
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
        Err(_) => "REFUSED".to_string(),
    };
    println!(
        "  {name:<48} {v:<8} sym0={} gated={} reg={} numeric={}",
        r.1.symbolic_zero, r.1.sign_gated, r.1.registered, r.1.numeric
    );
}

/// **Source 3 is CONTEXTUAL, not just ordered.** `sqrt(9/X)` and
/// `3·sqrt(1/X)` are one real. Whether they meet depends on whether
/// some OTHER node of the document happens to hold `sqrt(X)`.
#[test]
fn r1_source_three_makes_the_key_depend_on_the_rest_of_the_document() {
    let alone = probe(SymRules::shipped(), || {
        let x = pb("x", 1.0, 2.0);
        (k(9.0) / x).sqrt() - k(3.0) * (k(1.0) / x).sqrt()
    });
    let with_root = probe(SymRules::shipped(), || {
        let x = pb("x", 1.0, 2.0);
        let s = x.sqrt();
        (k(9.0) / x).sqrt() - k(3.0) * (k(1.0) / x).sqrt() + (s - s)
    });
    line("sqrt(9/X) - 3*sqrt(1/X)   alone          ", &alone);
    line("sqrt(9/X) - 3*sqrt(1/X)   + (sqrt X-sqrt X)", &with_root);
}

/// **Uniformity across mint sites**: rule D's hand-built `sqrt(1+X²)`
/// against the walk's own root over the same argument.
#[test]
fn r1_the_trig_site_and_the_walk_site_key_one_atom() {
    let r = probe(SymRules::shipped(), || {
        let x = pb("x", 1.0, 2.0);
        let s = (k(1.0) + x * x).sqrt();
        x.atan().cos() * s - k(1.0)
    });
    line("cos(atan X)*sqrt(1+X^2) - 1               ", &r);
    let off = probe(SymRules::without_canonical_root(), || {
        let x = pb("x", 1.0, 2.0);
        let s = (k(1.0) + x * x).sqrt();
        x.atan().cos() * s - k(1.0)
    });
    line("   same, without_canonical_root           ", &off);
}

/// **The registry's forms meet the walk's.** A registrant states
/// `sqrt(4·X) = 2·sqrt(X)`-shaped identity; the walk mints the other
/// spelling. With one door the residual is a THEOREM, not an axiom.
#[test]
fn r1_the_registrant_and_the_walk_mint_one_atom() {
    let (out, counts) = with_session_rules(budget(), SymRules::shipped(), || {
        let x = pb("x", 1.0, 2.0);
        let lhs = (k(4.0) * x).sqrt();
        let rhs = k(2.0) * x.sqrt();
        let reg = lhs.register_equal(rhs, Tol::witness());
        let s = (lhs - rhs).sign_within(band()).map_err(|e| format!("{e:?}"));
        (reg, s)
    });
    println!("  registration {:?} residual {:?} counts sym0={} reg={}",
        out.0, out.1, counts.symbolic_zero, counts.registered);
    assert_eq!(out.0, SymRegistration::Recorded);
}

/// **The two reads over a real box**: what fires, what is counted, and
/// whether the read DECLINES when the certification degrades (Q3).
#[test]
fn r1_the_reads_over_a_box() {
    let rows: Vec<(&str, fn() -> Sym<Interval>)> = vec![
        ("max(X,1) - X        X in [3,4]  certified", || {
            let x = pb("x", 3.0, 4.0);
            x.max(k(1.0)) - x
        }),
        ("max(X,1) - 1        X in [0,.5] certified", || {
            let x = pb("x", 0.0, 0.5);
            x.max(k(1.0)) - k(1.0)
        }),
        ("max(X,1) - X        X in [.5,4] straddles", || {
            let x = pb("x", 0.5, 4.0);
            x.max(k(1.0)) - x
        }),
        ("min(X,Y) - X        X<Y certified        ", || {
            let x = pb("x", 1.0, 2.0);
            let y = pb("y", 3.0, 4.0);
            x.min(y) - x
        }),
        ("max(1, 1/4) - 1     two CONSTANTS        ", || {
            k(1.0).max(k(0.25)) - k(1.0)
        }),
        ("select(X,7,9) - 9   X in [3,4]           ", || {
            let x = pb("x", 3.0, 4.0);
            Real::select_le_zero(x, k(7.0), k(9.0)) - k(9.0)
        }),
        ("select(X,7,9) - 7   X in [-1,0]  TIE at 0", || {
            let x = pb("x", -1.0, 0.0);
            Real::select_le_zero(x, k(7.0), k(9.0)) - k(7.0)
        }),
        ("select(X,7,9) - 7   X in [-1,1] straddles", || {
            let x = pb("x", -1.0, 1.0);
            Real::select_le_zero(x, k(7.0), k(9.0)) - k(7.0)
        }),
        // A decision dressed in a manifestly positive factor: the
        // content strip is what makes this readable.
        ("select(sqrt(1+X^2)*(X-5),7,9) - 7  X<5   ", || {
            let x = pb("x", 1.0, 2.0);
            let d = (k(1.0) + x * x).sqrt() * (x - k(5.0));
            Real::select_le_zero(d, k(7.0), k(9.0)) - k(7.0)
        }),
    ];
    println!("the reads (shipped):");
    for (n, f) in &rows {
        line(n, &probe(SymRules::shipped(), *f));
    }
    println!("the reads (without_the_reads):");
    for (n, f) in &rows {
        line(n, &probe(SymRules::without_the_reads(), *f));
    }
}
