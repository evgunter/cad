//! DECIDE-10 Phase 1 probe (not landed): every shape under the candidate
//! `CAD_DECIDE10` names, read on and read shut.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::{Band, Margin, Sign};
use geom_core::sym::with_session_rules;
use geom_core::{Interval, ParamSymbol, Real, Sym, SymBudget, SymCounts, SymRules, Tol};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}
fn band() -> Band {
    Band::linear(Tol::witness()).expect("linear band")
}
fn lit(v: f64) -> Sym<Interval> {
    Sym::from_f64(v)
}
fn over(name: &str, lo: f64, hi: f64) -> Sym<Interval> {
    Sym::param_over(
        ParamSymbol::new(test_utils::symbol_id(name)),
        Interval::from_bounds(lo, hi),
        lo,
        hi,
    )
}
fn label(out: Result<Sign, geom_core::predicate::Indeterminate>, c: SymCounts) -> String {
    match out {
        Ok(Sign::Zero) if c.symbolic_zero > 0 => "theorem".to_owned(),
        Ok(Sign::Zero) if c.registered > 0 => "registered".to_owned(),
        Ok(Sign::Zero) if c.sign_gated > 0 => "sign_gated".to_owned(),
        Ok(s) => format!("numeric {s:?}"),
        Err(e) => format!("refused {:?}", e.margin),
    }
}
fn how(rules: SymRules, build: fn() -> Sym<Interval>) -> String {
    let (out, counts) = with_session_rules(budget(), rules, || {
        let m = build();
        geom_core::k_stats::decide("decide10", Margin::of(m), band())
    });
    label(out, counts)
}
fn x() -> Sym<Interval> {
    over("x", 1.0, 2.0)
}
fn y() -> Sym<Interval> {
    over("y", 3.0, 4.0)
}
fn z(x: Sym<Interval>) -> Sym<Interval> {
    x.sqrt().powi(2) - x
}

#[test]
#[ignore = "probe"]
fn decide10_shapes() {
    let shapes: Vec<(&str, fn() -> Sym<Interval>)> = vec![
        ("S1 max(x+Z,3)-max(x,3)", || {
            (x() + z(x())).max(lit(3.0)) - x().max(lit(3.0))
        }),
        ("S2 min(x+Z,3)-min(x,3)", || {
            (x() + z(x())).min(lit(3.0)) - x().min(lit(3.0))
        }),
        ("S3 max(x+Z,y)-max(x,y)", || {
            (x() + z(x())).max(y()) - x().max(y())
        }),
        ("S4 sel(x-3+Z,y,y+1)-sel(x-3,y,y+1)", || {
            (x() - lit(3.0) + z(x())).select_le_zero(y(), y() + lit(1.0))
                - (x() - lit(3.0)).select_le_zero(y(), y() + lit(1.0))
        }),
        ("S5 (max(x+Z,3)+1)-(max(x,3)+1)", || {
            ((x() + z(x())).max(lit(3.0)) + lit(1.0)) - (x().max(lit(3.0)) + lit(1.0))
        }),
        ("S6 atan(max(x+Z,3))-atan(max(x,3))", || {
            (x() + z(x())).max(lit(3.0)).atan() - x().max(lit(3.0)).atan()
        }),
        ("S7 max(max(x+Z,3),y)-max(max(x,3),y)", || {
            (x() + z(x())).max(lit(3.0)).max(y()) - x().max(lit(3.0)).max(y())
        }),
        ("S8 atan(max(x+Z,3)-max(x,3)+x)-atan(x)", || {
            ((x() + z(x())).max(lit(3.0)) - x().max(lit(3.0)) + x()).atan() - x().atan()
        }),
        ("S8s sqrt(max(x+Z,3)-max(x,3)+x)-sqrt(x)", || {
            ((x() + z(x())).max(lit(3.0)) - x().max(lit(3.0)) + x()).sqrt() - x().sqrt()
        }),
        ("S10 atan(sel(x-3+Z,y,y+1))-atan(sel(x-3,y,y+1))", || {
            (x() - lit(3.0) + z(x()))
                .select_le_zero(y(), y() + lit(1.0))
                .atan()
                - (x() - lit(3.0)).select_le_zero(y(), y() + lit(1.0)).atan()
        }),
        ("S11 (max(x+Z,3)-max(x,3))*y", || {
            ((x() + z(x())).max(lit(3.0)) - x().max(lit(3.0))) * y()
        }),
        ("C1 max(x,3)-3", || x().max(lit(3.0)) - lit(3.0)),
        ("C2 max(x+Z,3)-3", || {
            (x() + z(x())).max(lit(3.0)) - lit(3.0)
        }),
        ("C3 max(x+Z,3)-max(x,3)+max(x,3)-3", || {
            (x() + z(x())).max(lit(3.0)) - x().max(lit(3.0)) + x().max(lit(3.0)) - lit(3.0)
        }),
        ("C4 min(x,y)-x", || x().min(y()) - x()),
        ("C5 max(sel(x-3,y,y+1),5)-5", || {
            (x() - lit(3.0))
                .select_le_zero(y(), y() + lit(1.0))
                .max(lit(5.0))
                - lit(5.0)
        }),
        ("C6 atan(max(x,3))-atan(3)", || {
            x().max(lit(3.0)).atan() - lit(3.0).atan()
        }),
        ("C7 max(x,3)*y-3y", || {
            x().max(lit(3.0)) * y() - lit(3.0) * y()
        }),
    ];
    println!("candidate {:?}", std::env::var("CAD_DECIDE10").ok());
    for (what, b) in shapes {
        let on = how(SymRules::shipped(), b);
        let off = how(SymRules::without_the_reads(), b);
        println!("ROW | {what} | {on} | {off}");
    }
    let tiny = SymBudget {
        max_terms: 4,
        max_degree: 128,
    };
    for (dial, rules) in [
        ("on", SymRules::shipped()),
        ("shut", SymRules::without_the_reads()),
    ] {
        let (out, counts) = with_session_rules(tiny, rules, || {
            let p = x() + y() + over("w", 3.0, 4.0);
            let q = over("u", 1.0, 2.0) + over("v", 1.0, 2.0);
            let m = (x() + z(x())).max(p) * q - x().max(p) * q;
            geom_core::k_stats::decide("decide10", Margin::of(m), band())
        });
        println!(
            "ROW | S13 [4 terms] max(x+Z,P)Q-max(x,P)Q | {dial} {} {counts:?}",
            label(out, counts)
        );
    }
    for (dial, rules) in [
        ("on", SymRules::shipped()),
        ("shut", SymRules::without_the_reads()),
    ] {
        let (out, counts) = with_session_rules(budget(), rules, || {
            let x = over("x", 0.9, 1.1);
            let sq = x * x;
            let _ = sq.register_equal(x, Tol::witness());
            let m = sq.max(lit(3.0)) - x.max(lit(3.0));
            geom_core::k_stats::decide("decide10", Margin::of(m), band())
        });
        println!(
            "ROW | S9 door max(x*x,3)-max(x,3), x*x=x | {dial} {}",
            label(out, counts)
        );
        let (out, counts) = with_session_rules(budget(), rules, || {
            let x = over("x", 0.9, 1.1);
            let sq = x * x;
            let _ = sq.register_equal(x, Tol::witness());
            let m = (sq.max(lit(3.0)) - x.max(lit(3.0))) + (sq - x);
            geom_core::k_stats::decide("decide10", Margin::of(m), band())
        });
        println!(
            "ROW | S12 door (max(x*x,3)-max(x,3))+(x*x-x), x*x=x | {dial} {}",
            label(out, counts)
        );
    }
}
