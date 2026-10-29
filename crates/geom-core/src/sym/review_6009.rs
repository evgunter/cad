//! Review probes for PR 6009 (rule H's deferred fold). Not for merge.
use super::*;
use crate::predicate::Margin;
use crate::tolerance::Tol;
use crate::{Band, Sign};

fn budget() -> SymBudget {
    SymBudget { max_terms: 4096, max_degree: 128 }
}
fn band() -> Band {
    Band::linear(Tol::witness()).expect("band")
}
fn p(name: &str, v: f64) -> Sym<f64> {
    Sym::param(ParamSymbol::new(test_utils::symbol_id(name)), v)
}
fn lit(x: f64) -> Sym<f64> {
    Sym::from_f64(x)
}
fn decide(m: Sym<f64>) -> String {
    format!("{:?}", crate::k_stats::decide("rev", Margin::of(m), band()))
}
fn turned(num: i64, den: u64) -> (Sym<f64>, Sym<f64>) {
    crate::Angle::from_turns(crate::Turns::new(num, den).expect("a fraction")).sin_cos()
}
fn symbolic() -> u64 {
    session_counts().map_or(0, |c| c.symbolic as u64)
}
/// `(name, verdict, symbolic proofs it added)`.
fn row(name: &str, m: Sym<f64>) -> (String, String, u64) {
    let before = symbolic();
    let d = decide(m);
    (name.to_string(), d, symbolic() - before)
}

fn digests(m: &Sym<f64>) -> (u128, u128, bool) {
    SESSION.with(|s| {
        let mut slot = s.borrow_mut();
        let sess = slot.as_mut().expect("session");
        let pl = plain_form(sess, m.node);
        let ea = early_form(sess, m.node);
        (pl.digest(), ea.digest(), ea.poisoned)
    })
}

fn exprs() -> Vec<(String, Sym<f64>)> {
    let x = p("x", 0.7);
    let r = p("r", 2.0);
    let mut out = Vec::new();
    for den in [3u64, 4, 6, 8, 9, 12, 24] {
        let (s1, c1) = turned(1, den);
        for k in 0..i64::try_from(den).unwrap() {
            let (s, c) = turned(k, den);
            let (sn, cn) = turned(k + 1, den);
            out.push((format!("s({k}/{den})"), s));
            out.push((format!("c({k}/{den})"), c));
            out.push((format!("x*s({k}/{den})+c"), x * s + c));
            out.push((format!("ring c {k}/{den}"), c1 * (r * c) - s1 * (r * s) - r * cn));
            out.push((format!("ring s {k}/{den}"), s1 * (r * c) + c1 * (r * s) - r * sn));
            out.push((format!("sqrt(1+s) {k}/{den}"), (lit(1.0) + s).sqrt()));
            out.push((format!("1/(x-s) {k}/{den}"), lit(1.0) / (x - s)));
            out.push((format!("sel {k}/{den}"), (s - lit(0.5)).select_le_zero(x, r)));
        }
    }
    out
}

/// Prints each expression's plain and early form digests: diff base vs head.
#[test]
fn review_6009_form_digests() {
    for (label, rules) in [
        ("shipped", SymRules::shipped()),
        ("no_h", SymRules::without_angle_sum()),
        ("none", SymRules::none()),
    ] {
        let ((), _) = with_session_rules(budget(), rules, || {
            for (name, m) in exprs() {
                let (pl, ea, poison) = digests(&m);
                println!("DIGEST {label} {name}: plain={pl:032x} early={ea:032x} poison={poison}");
            }
        });
    }
}

/// Verdicts under shipped and no_h for the false-proof probes.
#[test]
fn review_6009_false_proofs() {
    for (label, rules) in [("shipped", SymRules::shipped()), ("no_h", SymRules::without_angle_sum())] {
        let (rows, counts) = with_session_rules(budget(), rules, || {
            let x = p("x", 0.7);
            let y = p("y", 0.3);
            let (s, _) = turned(1, 12);
            let (_, c3) = turned(1, 3);
            let (s9, c9) = turned(1, 9);
            let half = lit(0.5);
            let eps = lit(1e-9);
            vec![
                row("s-0.5 (T)", s - half),
                row("s-(0.5+1e-9) (F)", s - (half + eps)),
                row("s-0.5-1e-9 (F)", s - half - eps),
                row("s*s-0.25 (T)", s * s - lit(0.25)),
                row("s*s-0.25-1e-9 (F)", s * s - lit(0.25) - eps),
                row("s*s-s/2 (T)", s * s - s * half),
                row("s*s - s*s9/s9? (s - s9) (F)", s * s - s9),
                row("c3+0.5 (T)", c3 + half),
                row("c3+s (T)", c3 + s),
                row("c3-s (F)", c3 - s),
                row("c3*c9 + 0.5*c9 (T)", c3 * c9 + half * c9),
                row("c3*c9 + (0.5+1e-9)*c9 (F)", c3 * c9 + (half + eps) * c9),
                row("c3*c9 + s*c9 (T)", c3 * c9 + s * c9),
                row("sqrt(s-0.5) (T)", (s - half).sqrt()),
                row("sqrt(s-0.5+x*x)-x (T? sqrt(x^2)=|x|)", (s - half + x * x).sqrt() - x),
                row("sqrt(s*s)-s (T)", (s * s).sqrt() - s),
                row("sqrt(s*s)-0.5-1e-9 (F)", (s * s).sqrt() - half - eps),
                row("atan(s)-atan(0.5) (T)", s.atan() - half.atan()),
                row("atan(s)-atan(0.5+1e-9) (F)", s.atan() - (half + eps).atan()),
                row("1/(s-0.5) (poison)", lit(1.0) / (s - half)),
                row("x/(2s-1) (poison)", x / (lit(2.0) * s - lit(1.0))),
                row("(s-0.5)*(1/(s-0.5)) (poison)", (s - half) * (lit(1.0) / (s - half))),
                row("(s-0.5)*(1/(s-0.5)) - 1 (poison)", (s - half) * (lit(1.0) / (s - half)) - lit(1.0)),
                row("powi(s-0.5,-2) (poison)", (s - half).powi(-2)),
                row("powi(s,-1)-2 (T)", s.powi(-1) - lit(2.0)),
                row("powi(s,-1)-2-1e-9 (F)", s.powi(-1) - lit(2.0) - eps),
                row("1/s - 2 (T)", lit(1.0) / s - lit(2.0)),
                row("x/(s+x) - x/(0.5+x) (T)", x / (s + x) - x / (half + x)),
                row("x/(s+x) - x/(0.5+1e-9+x) (F)", x / (s + x) - x / (half + eps + x)),
                row("sel(s-0.5,x,y)-x (T)", (s - half).select_le_zero(x, y) - x),
                row("sel(s-0.5,x,y)-y (F)", (s - half).select_le_zero(x, y) - y),
                row("sel(0.5-s,x,y)-x (T)", (half - s).select_le_zero(x, y) - x),
                row("sel(s-0.5-1e-9,x,y)-x (T)", (s - half - eps).select_le_zero(x, y) - x),
                row("sel(s-0.5+1e-9,x,y)-x (F)", (s - half + eps).select_le_zero(x, y) - x),
                row("sel(s-0.5+1e-9,x,y)-y (T)", (s - half + eps).select_le_zero(x, y) - y),
                row("sel(x,s,0.5)-0.5 (T arms)", x.select_le_zero(s, half) - half),
                row("sel(x,s,0.5)-s (T arms)", x.select_le_zero(s, half) - s),
                row("sel(x,s,y)-0.5 (F)", x.select_le_zero(s, y) - half),
                row("min(s,0.5)-0.5 (T)", s.min(half) - half),
                row("max(s,x)-max(0.5,x) (T)", s.max(x) - half.max(x)),
                row("abs(s-0.5) (T)", (s - half).abs()),
                row("abs(s-0.5-1e-9) (F)", (s - half - eps).abs()),
                row("sin(s)-sin(0.5) (T)", s.sin() - half.sin()),
                row("cos(pi*s)-0 (T: cos(pi/2))", (<Sym<f64> as Real>::pi() * s).cos()),
                row("sin(pi*s)-1 (T)", (<Sym<f64> as Real>::pi() * s).sin() - lit(1.0)),
                row("sin(pi*s)-1-1e-9 (F)", (<Sym<f64> as Real>::pi() * s).sin() - lit(1.0) - eps),
                row("s^3 - 1/8 (T)", s * s * s - lit(0.125)),
                row("s^3 - s/4 (T)", s * s * s - s * lit(0.25)),
                row("s^2*c3 + 1/8 (T)", s * s * c3 + lit(0.125)),
                row("s^2*c3 + 1/8 + 1e-9 (F)", s * s * c3 + lit(0.125) + eps),
            ]
        });
        for (n, d, k) in rows {
            println!("FP {label} {n}: {d} sym+{k}");
        }
        println!("FP {label} counts {counts:?}");
    }
}

/// Select's arms are not settled: does a spelling with s vs 0.5 in an undecided arm still meet?
#[test]
fn review_6009_select_arms() {
    for (label, rules) in [("shipped", SymRules::shipped()), ("no_h", SymRules::without_angle_sum())] {
        let (rows, _) = with_session_rules(budget(), rules, || {
            let x = p("x", 0.7);
            let y = p("y", 0.3);
            let (s, _) = turned(1, 12);
            let half = lit(0.5);
            vec![
                row("sel(x,s,y)-sel(x,0.5,y) (T)", x.select_le_zero(s, y) - x.select_le_zero(half, y)),
                row("sel(x,s*y,y)-sel(x,0.5*y,y) (T)", x.select_le_zero(s * y, y) - x.select_le_zero(half * y, y)),
                row("min(x,s)-min(x,0.5) (T)", x.min(s) - x.min(half)),
            ]
        });
        for (n, d, k) in rows {
            println!("SA {label} {n}: {d} sym+{k}");
        }
    }
}
