//! Reviewer probe (PR 7712): a differential battery over the public door.
//!
//! Each row is one margin decided at `Sym<Interval>` in its own session
//! (shipped rules, the drive's kept-atom ladder). The row prints how it
//! was answered. The file uses nothing the unit adds, so it builds on
//! the merge base too: the two printouts are diffed, and a row `proven`
//! on the base and not on the head is a `Zero` the point reading took
//! away.
//!
//! Run: `cargo nextest run -p geom-core --test review_refute_differential
//! --no-capture --run-ignored all`

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::{Band, Margin, Sign};
use geom_core::sym::with_session_retry;
use geom_core::{Interval, ParamSymbol, Real, Sym, SymBudget, SymRetry, SymRules, Tol};

type S = Sym<Interval>;

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

fn lit(v: f64) -> S {
    Sym::from_f64(v)
}

fn over(name: &str, lo: f64, hi: f64) -> S {
    Sym::param_over(
        ParamSymbol::new(test_utils::symbol_id(name)),
        Interval::from_bounds(lo, hi),
        lo,
        hi,
    )
}

/// The parameters of one box: `x` positive, `y` positive and clear of
/// `x`, `z` and `w` straddling.
fn params(box_: usize) -> [S; 4] {
    match box_ {
        0 => [
            over("x", 0.9, 1.1),
            over("y", 2.0, 3.0),
            over("z", -1.0, 1.0),
            over("w", -0.3, 0.4),
        ],
        1 => [
            over("r", 0.25, 4.0),
            over("h", 5.0, 5.5),
            over("a", -2.0, 0.5),
            over("b", -1e-3, 1e-3),
        ],
        _ => [
            over("p0", 1.0, 1.0 + 1e-9),
            over("p1", 7.0, 7.5),
            over("p2", -1e6, 1e6),
            over("p3", -0.5, 0.25),
        ],
    }
}

type Sub = (&'static str, fn(&[S; 4]) -> S);

/// Sub-expressions of every sign behaviour over the box.
fn pool() -> Vec<Sub> {
    vec![
        ("x", |p| p[0]),
        ("y", |p| p[1]),
        ("z", |p| p[2]),
        ("w", |p| p[3]),
        ("-y", |p| -p[1]),
        ("z-8", |p| p[2] - lit(8.0)),
        ("x*y", |p| p[0] * p[1]),
        ("z*z", |p| p[2] * p[2]),
        ("y-x", |p| p[1] - p[0]),
        ("z*w", |p| p[2] * p[3]),
        ("sin z", |p| p[2].sin_cos().0),
        ("max(z,w)", |p| p[2].max(p[3])),
        ("sel(z,x,y)", |p| p[2].select_le_zero(p[0], p[1])),
        ("|z|", |p| p[2].abs()),
        ("sqrt y", |p| p[1].sqrt()),
        ("x/y", |p| p[0] / p[1]),
        ("z+0.3", |p| p[2] + lit(0.3)),
        ("copysign(x,z)", |p| p[0].copysign(p[2])),
        ("floor(3z)", |p| (p[2] * lit(3.0)).floor()),
        ("atan2(z,w)", |p| p[2].atan2(p[3])),
        ("atan2(w,-x)", |p| p[3].atan2(-p[0])),
        ("sel(w,sqrt(-z),x)", |p| {
            p[3].select_le_zero((-p[2]).sqrt(), p[0])
        }),
        ("min(z,x)", |p| p[2].min(p[0])),
        ("1/(z+w)", |p| lit(1.0) / (p[2] + p[3])),
    ]
}

type Template = (&'static str, fn(S, S) -> S);

fn sin(u: S) -> S {
    u.sin_cos().0
}
fn cos(u: S) -> S {
    u.sin_cos().1
}

/// Margins that are identities on a domain, a branch or everywhere.
fn templates() -> Vec<Template> {
    vec![
        ("(u+v)-v-u", |u, v| (u + v) - v - u),
        ("(u+v)^2-u^2-2uv-v^2", |u, v| {
            (u + v).powi(2) - u.powi(2) - lit(2.0) * u * v - v.powi(2)
        }),
        ("u/u-1", |u, _| u / u - lit(1.0)),
        ("u*(1/u)-1", |u, _| u * (lit(1.0) / u) - lit(1.0)),
        ("1/(1/u)-u", |u, _| lit(1.0) / (lit(1.0) / u) - u),
        ("(u-v)/(v-u)+1", |u, v| (u - v) / (v - u) + lit(1.0)),
        ("sqrt(u)^2-u", |u, _| u.sqrt().powi(2) - u),
        ("sqrt(u)*sqrt(u)-u", |u, _| u.sqrt() * u.sqrt() - u),
        ("sqrt(u^2)-u", |u, _| (u * u).sqrt() - u),
        ("sqrt(u^2)+u", |u, _| (u * u).sqrt() + u),
        ("sqrt(u^2)-|u|", |u, _| (u * u).sqrt() - u.abs()),
        ("|u|^2-u^2", |u, _| u.abs().powi(2) - u * u),
        ("|u|-max(u,-u)", |u, _| u.abs() - u.max(-u)),
        ("max(u,-u)-u", |u, _| u.max(-u) - u),
        ("max(u,-u)+u", |u, _| u.max(-u) + u),
        ("max(u,v)-sel(v-u,u,v)", |u, v| {
            u.max(v) - (v - u).select_le_zero(u, v)
        }),
        ("min+max-u-v", |u, v| u.min(v) + u.max(v) - u - v),
        ("max(u,v)-u", |u, v| u.max(v) - u),
        ("min(u,v)-u", |u, v| u.min(v) - u),
        ("sel(u,v,v)-v", |u, v| u.select_le_zero(v, v) - v),
        ("sel(u,v,u)-v", |u, v| u.select_le_zero(v, u) - v),
        ("sel(u,v,u)-u", |u, v| u.select_le_zero(v, u) - u),
        ("sel(u,u,-u)+|u|", |u, _| u.select_le_zero(u, -u) + u.abs()),
        ("sel(u,-u,u)-sqrt(u^2)", |u, _| {
            u.select_le_zero(-u, u) - (u * u).sqrt()
        }),
        ("sin^2+cos^2-1", |u, _| {
            sin(u).powi(2) + cos(u).powi(2) - lit(1.0)
        }),
        ("sin(2atan u)-2u/(1+u^2)", |u, _| {
            sin(lit(2.0) * u.atan()) - lit(2.0) * u / (lit(1.0) + u * u)
        }),
        ("cos(2atan u)-(1-u^2)/(1+u^2)", |u, _| {
            cos(lit(2.0) * u.atan()) - (lit(1.0) - u * u) / (lit(1.0) + u * u)
        }),
        ("sin(atan u)-u/sqrt(1+u^2)", |u, _| {
            sin(u.atan()) - u / (lit(1.0) + u * u).sqrt()
        }),
        ("cos(atan u)-1/sqrt(1+u^2)", |u, _| {
            cos(u.atan()) - lit(1.0) / (lit(1.0) + u * u).sqrt()
        }),
        ("cos(atan(u)/2)^2-(1+1/sqrt(1+u^2))/2", |u, _| {
            cos(u.atan() * lit(0.5)).powi(2)
                - (lit(1.0) + lit(1.0) / (lit(1.0) + u * u).sqrt()) * lit(0.5)
        }),
        ("sin(4atan u) closed", |u, _| {
            let d = lit(1.0) + u * u;
            sin(lit(4.0) * u.atan()) - lit(4.0) * u * (lit(1.0) - u * u) / (d * d)
        }),
        ("atan2(u-u,v^2)", |u, v| (u - u).atan2(v * v)),
        ("atan2(u-u,sqrt(v^2))", |u, v| (u - u).atan2((v * v).sqrt())),
        ("atan2(u-u,v)", |u, v| (u - u).atan2(v)),
        ("atan2(0,v^2/sqrt(v^2))", |_, v| {
            lit(0.0).atan2(v * v / (v * v).sqrt())
        }),
        ("atan2(0,v^2/(sqrt(v^2)*|v|))", |_, v| {
            lit(0.0).atan2(v * v / ((v * v).sqrt() * v.abs()))
        }),
        ("sin(pi/6)-1/2+(u-u)", |u, _| {
            sin(S::pi() / lit(6.0)) - lit(0.5) + (u - u)
        }),
        ("sin(pi)*u", |u, _| sin(S::pi()) * u),
        ("cos(pi/2)*u", |u, _| cos(S::pi() / lit(2.0)) * u),
        ("(tau-2pi)*u", |u, _| (S::tau() - lit(2.0) * S::pi()) * u),
        ("sin12-1/2+(u-u)", |u, _| {
            S::sin_of_a_twelfth() - lit(0.5) + (u - u)
        }),
        ("u^3-u*u*u", |u, _| u.powi(3) - u * u * u),
        ("u^-2-1/(u*u)", |u, _| u.powi(-2) - lit(1.0) / (u * u)),
        ("u^0-1", |u, _| u.powi(0) - lit(1.0)),
        ("sqrt(u)*sqrt(v)-sqrt(u*v)", |u, v| {
            u.sqrt() * v.sqrt() - (u * v).sqrt()
        }),
        ("sqrt(u^2*v)-u*sqrt(v)", |u, v| {
            (u * u * v).sqrt() - u * v.sqrt()
        }),
        ("sqrt(u/v)-sqrt(u)/sqrt(v)", |u, v| {
            (u / v).sqrt() - u.sqrt() / v.sqrt()
        }),
        ("u/sqrt(u^2)-1", |u, _| u / (u * u).sqrt() - lit(1.0)),
        ("u/|u|-1", |u, _| u / u.abs() - lit(1.0)),
        ("sqrt(u^2)/|u|-1", |u, _| (u * u).sqrt() / u.abs() - lit(1.0)),
        ("sqrt(u)^4-u^2", |u, _| u.sqrt().powi(4) - u * u),
        ("|u*v|-|u|*|v|", |u, v| (u * v).abs() - u.abs() * v.abs()),
        ("||u||-|u|", |u, _| u.abs().abs() - u.abs()),
        ("copysign(u,v)^2-u^2", |u, v| u.copysign(v).powi(2) - u * u),
        ("|copysign(u,v)|-|u|", |u, v| u.copysign(v).abs() - u.abs()),
        ("floor(u)-floor(u+v-v)", |u, v| u.floor() - (u + v - v).floor()),
        ("tan-sin/cos", |u, _| u.tan() - sin(u) / cos(u)),
        ("asin(sin u)-u", |u, _| sin(u).asin() - u),
        ("sin(asin u)-u", |u, _| sin(u.asin()) - u),
        ("cos(acos u)-u", |u, _| cos(u.acos()) - u),
        ("atan(tan u)-u", |u, _| u.tan().atan() - u),
        ("lesssin-(u-sin u)", |u, _| u.less_sin() - (u - sin(u))),
        ("atan2(sin,cos)-u", |u, _| sin(u).atan2(cos(u)) - u),
        ("atan2(u,v)-atan(u/v)", |u, v| u.atan2(v) - (u / v).atan()),
        ("atan2(2u,1-u^2)-2atan u", |u, _| {
            (lit(2.0) * u).atan2(lit(1.0) - u * u) - lit(2.0) * u.atan()
        }),
        ("hull(u,v)-hull(u,v)", |u, v| {
            use geom_core::SpanLocate;
            u.enclosure_hull(v) - u.enclosure_hull(v)
        }),
        ("sel(u,v+1,v+1)-v-1 nested", |u, v| {
            u.select_le_zero(v + lit(1.0), (v + lit(1.0)).max(v)) - v - lit(1.0)
        }),
        ("max(u,u)-u", |u, _| u.max(u) - u),
        ("max(u,v)-max(v,u)", |u, v| u.max(v) - v.max(u)),
        ("max(u+1,u)-u-1", |u, _| (u + lit(1.0)).max(u) - u - lit(1.0)),
        ("min(u-1,u)-u+1", |u, _| (u - lit(1.0)).min(u) - u + lit(1.0)),
        ("sel(v^2+1,u,v)-v", |u, v| {
            (v * v + lit(1.0)).select_le_zero(u, v) - v
        }),
        ("sel(-v^2-1,u,v)-u", |u, v| {
            (-(v * v) - lit(1.0)).select_le_zero(u, v) - u
        }),
        ("sel(v^2,u,u+0)-u", |u, v| {
            (v * v).select_le_zero(u, u + lit(0.0)) - u
        }),
        ("sel(-v^2,u,v)-u", |u, v| (-(v * v)).select_le_zero(u, v) - u),
        ("sel(-|v|,u,v)-u", |u, v| (-v.abs()).select_le_zero(u, v) - u),
        ("sel(-sqrt(v),u,v)-u", |u, v| (-v.sqrt()).select_le_zero(u, v) - u),
        ("sel(v-v,u,v)-u", |u, v| (v - v).select_le_zero(u, v) - u),
    ]
}

fn row(box_: usize, t: &Template, u: &Sub, v: &Sub) -> String {
    let band = Band::linear(Tol::witness()).expect("a linear band");
    let (out, counts) = with_session_retry(
        budget(),
        SymRules::shipped(),
        SymRetry::kept_atom(),
        || {
            let p = params(box_);
            let m = (t.1)((u.1)(&p), (v.1)(&p));
            geom_core::k_stats::decide("review_probe", Margin::of(m), band)
        },
    );
    let how = match out {
        Ok(Sign::Zero) if counts.symbolic > 0 => "PROVEN".to_owned(),
        Ok(s) => format!("numeric {s:?}"),
        Err(e) if e.margin.is_invalid() => "invalid".to_owned(),
        Err(_) => "undecided".to_owned(),
    };
    format!("box{box_} | {} | u={} v={} | {how}", t.0, u.0, v.0)
}

#[test]
#[ignore = "reviewer differential: prints rows to diff across two trees"]
fn review_the_battery() {
    let (pool, templates) = (pool(), templates());
    let mut proven = 0;
    let mut rows = 0;
    for box_ in 0..3 {
        for t in &templates {
            for u in &pool {
                for v in &pool {
                    let l = row(box_, t, u, v);
                    proven += usize::from(l.ends_with("PROVEN"));
                    rows += 1;
                    println!("ROW {l}");
                }
            }
        }
    }
    println!("TOTAL rows {rows} proven {proven}");
    assert!(proven > 0);
}

fn decide_in_session(f: impl FnOnce() -> S) -> String {
    let band = Band::linear(Tol::witness()).expect("a linear band");
    let (out, counts) = with_session_retry(
        budget(),
        SymRules::shipped(),
        SymRetry::kept_atom(),
        || geom_core::k_stats::decide("review_probe", Margin::of(f()), band),
    );
    match out {
        Ok(Sign::Zero) if counts.symbolic > 0 => "PROVEN".to_owned(),
        Ok(s) => format!("numeric {s:?}"),
        Err(e) if e.margin.is_invalid() => "invalid".to_owned(),
        Err(_) => "undecided".to_owned(),
    }
}

/// **A select whose decision is exactly zero at the point takes
/// `when_le`.** `select(x − x, x − x + (y − y), 1)` is zero wherever the
/// decision is `≤ 0`, which is everywhere; the tier folds the constant
/// decision and proves it. A reading that takes `when_gt` at the tie
/// reads 1 and refutes a `Zero` the tier proves.
#[test]
fn review_a_select_at_a_tie_takes_when_le() {
    let how = decide_in_session(|| {
        let (x, y) = (over("x", -1.0, 1.0), over("y", 2.0, 3.0));
        (x - x).select_le_zero((x - x) + (y - y), lit(1.0)) + (y * x - x * y)
    });
    assert_eq!(how, "PROVEN");
}

/// **A bracket that MOVES clears the readings taken under the old one.**
/// `max(x, −x) − x` is `−2x` at the point of `[-1, 1e-12]` (refuted, undecided) and
/// an identity over `[0.25, 1]`. The node is one node under both
/// brackets, so a reading kept across the move refutes the identity.
#[test]
fn review_a_moved_bracket_clears_the_readings() {
    let band = Band::linear(Tol::witness()).expect("a linear band");
    let m = |x: S| x.max(-x) - x;
    let ((first, second), counts) = with_session_retry(
        budget(),
        SymRules::shipped(),
        SymRetry::kept_atom(),
        || {
            // Straddles over the box (`[-2e-12, 2]`), `-2x > 0` at any
            // point the rule picks: undecided, and refuted on the head.
            let first = geom_core::k_stats::decide(
                "review_probe",
                Margin::of(m(over("x", -1.0, 1e-12))),
                band,
            );
            let second =
                geom_core::k_stats::decide("review_probe", Margin::of(m(over("x", 0.25, 1.0))), band);
            (first.ok(), second.ok())
        },
    );
    println!("first {first:?} second {second:?} {counts:?}");
    assert_eq!(second, Some(Sign::Zero), "{counts:?}");
    assert!(counts.symbolic >= 1, "{counts:?}");
}
